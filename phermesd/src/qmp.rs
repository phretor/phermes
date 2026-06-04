//! QMP control: a small async trait phermesd depends on, plus the real qapi-rs impl.

use async_trait::async_trait;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum QmpError {
    #[error("connecting QMP at {path}: {source}")]
    Connect {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("QMP protocol error: {0}")]
    Protocol(String),
}

#[async_trait]
pub trait QmpControl: Send {
    /// True if the guest reports a running run-state.
    ///
    /// # Errors
    /// Returns [`QmpError::Protocol`] if the `query-status` command fails.
    async fn is_running(&self) -> Result<bool, QmpError>;
    /// Request an ACPI graceful powerdown.
    ///
    /// # Errors
    /// Returns [`QmpError::Protocol`] if the `system_powerdown` command fails.
    async fn powerdown(&self) -> Result<(), QmpError>;
    /// Resolve when the guest emits SHUTDOWN (or the connection closes).
    ///
    /// # Errors
    /// Returns [`QmpError::Protocol`] only if event delivery itself fails.
    async fn wait_shutdown(&mut self) -> Result<(), QmpError>;
}

pub use real::QapiQmp;

mod real {
    use super::{QmpControl, QmpError};
    use async_trait::async_trait;
    use futures::StreamExt;
    use std::path::Path;
    use tokio::io::{ReadHalf, WriteHalf};
    use tokio::net::UnixStream;
    use tokio::sync::mpsc;

    type QmpWrite = qapi::futures::QmpStreamTokio<WriteHalf<UnixStream>>;
    type QmpRead = qapi::futures::QmpStreamTokio<ReadHalf<UnixStream>>;
    type Service = qapi::futures::QapiService<QmpWrite>;
    type Events = qapi::futures::QapiEvents<QmpRead>;

    /// Real qapi-rs QMP client over a Unix socket.
    pub struct QapiQmp {
        service: Service,
        events: mpsc::UnboundedReceiver<qapi::qmp::Event>,
        _pump: tokio::task::JoinHandle<()>,
    }

    impl QapiQmp {
        /// Connect to a QMP Unix socket, negotiate capabilities, and start the
        /// events pump.
        ///
        /// # Errors
        /// Returns [`QmpError::Connect`] if the socket cannot be opened, or
        /// [`QmpError::Protocol`] if capability negotiation fails.
        pub async fn connect(path: &Path) -> Result<Self, QmpError> {
            let stream = qapi::futures::QmpStreamTokio::open_uds(path)
                .await
                .map_err(|source| QmpError::Connect {
                    path: path.display().to_string(),
                    source,
                })?;
            let stream = stream
                .negotiate()
                .await
                .map_err(|e| QmpError::Protocol(e.to_string()))?;
            let (service, events) = stream.into_parts();
            let (tx, rx) = mpsc::unbounded_channel();
            let pump = tokio::spawn(pump_events(events, tx));
            Ok(Self {
                service,
                events: rx,
                _pump: pump,
            })
        }
    }

    async fn pump_events(mut events: Events, tx: mpsc::UnboundedSender<qapi::qmp::Event>) {
        while let Some(item) = events.next().await {
            match item {
                Ok(ev) => {
                    if tx.send(ev).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    }

    #[async_trait]
    impl QmpControl for QapiQmp {
        async fn is_running(&self) -> Result<bool, QmpError> {
            let status = self
                .service
                .execute(qapi::qmp::query_status {})
                .await
                .map_err(|e| QmpError::Protocol(e.to_string()))?;
            Ok(status.running)
        }

        async fn powerdown(&self) -> Result<(), QmpError> {
            self.service
                .execute(qapi::qmp::system_powerdown {})
                .await
                .map_err(|e| QmpError::Protocol(e.to_string()))?;
            Ok(())
        }

        async fn wait_shutdown(&mut self) -> Result<(), QmpError> {
            while let Some(ev) = self.events.recv().await {
                if let qapi::qmp::Event::SHUTDOWN { .. } = ev {
                    return Ok(());
                }
            }
            Ok(())
        }
    }
}

/// Retries connecting until the socket is ready or attempts run out.
///
/// # Errors
/// Returns the last connection error if all attempts fail.
pub async fn connect_with_retry(
    path: &Path,
    attempts: u32,
    delay: std::time::Duration,
) -> Result<QapiQmp, QmpError> {
    let mut last = QmpError::Protocol("no attempts made".to_owned());
    for _ in 0..attempts {
        match QapiQmp::connect(path).await {
            Ok(client) => return Ok(client),
            Err(e) => {
                last = e;
                tokio::time::sleep(delay).await;
            }
        }
    }
    Err(last)
}
