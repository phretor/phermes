//! QEMU Guest Agent control for filesystem quiescing during live snapshots.

use async_trait::async_trait;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum QgaError {
    #[error("connecting QGA at {path}: {source}")]
    Connect {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("QGA protocol error: {0}")]
    Protocol(String),
}

/// The guest-agent operations storage needs.
#[async_trait]
pub trait QgaControl: Send {
    /// Liveness probe.
    ///
    /// # Errors
    /// Returns `QgaError::Protocol` if the agent does not respond.
    async fn ping(&self) -> Result<(), QgaError>;
    /// Freeze guest filesystems; returns the number frozen.
    ///
    /// # Errors
    /// Returns `QgaError::Protocol` on agent failure.
    async fn freeze(&self) -> Result<i64, QgaError>;
    /// Thaw guest filesystems; returns the number thawed.
    ///
    /// # Errors
    /// Returns `QgaError::Protocol` on agent failure.
    async fn thaw(&self) -> Result<i64, QgaError>;
}

/// Opens a `QgaControl` for a guest-agent socket path. Mockable in tests.
#[async_trait]
pub trait QgaConnector: Send + Sync {
    /// Connect to the guest agent at `path`.
    ///
    /// # Errors
    /// Returns `QgaError` if the socket cannot be opened or the sync handshake fails.
    async fn connect(&self, path: &Path) -> Result<Box<dyn QgaControl>, QgaError>;
}

pub use real::{QapiQga, RealQgaConnector};

mod real {
    use super::{QgaConnector, QgaControl, QgaError};
    use async_trait::async_trait;
    use std::path::Path;
    use tokio::io::{ReadHalf, WriteHalf};
    use tokio::net::UnixStream;

    type Service = qapi::futures::QapiService<
        qapi::futures::QgaStreamTokio<WriteHalf<UnixStream>>,
    >;

    /// Real qapi-rs QGA client over a Unix socket.
    pub struct QapiQga {
        service: Service,
        _pump: tokio::task::JoinHandle<()>,
    }

    impl QapiQga {
        /// Connect to a QGA Unix socket, then perform the sync handshake.
        ///
        /// # Errors
        /// Returns [`QgaError::Connect`] if the socket cannot be opened, or
        /// [`QgaError::Protocol`] if the sync handshake fails.
        pub async fn connect(path: &Path) -> Result<Self, QgaError> {
            let stream =
                qapi::futures::QgaStreamTokio::<ReadHalf<UnixStream>>::open_uds(path)
                    .await
                    .map_err(|source| QgaError::Connect {
                        path: path.display().to_string(),
                        source,
                    })?;
            let (service, pump) = stream.spawn_tokio();
            let sync_value = std::process::id().cast_signed();
            service
                .guest_sync(sync_value)
                .await
                .map_err(|e| QgaError::Protocol(e.to_string()))?;
            Ok(Self { service, _pump: pump })
        }
    }

    #[async_trait]
    impl QgaControl for QapiQga {
        async fn ping(&self) -> Result<(), QgaError> {
            self.service
                .execute(qapi::qga::guest_ping {})
                .await
                .map_err(|e| QgaError::Protocol(e.to_string()))?;
            Ok(())
        }

        async fn freeze(&self) -> Result<i64, QgaError> {
            self.service
                .execute(qapi::qga::guest_fsfreeze_freeze {})
                .await
                .map_err(|e| QgaError::Protocol(e.to_string()))
        }

        async fn thaw(&self) -> Result<i64, QgaError> {
            self.service
                .execute(qapi::qga::guest_fsfreeze_thaw {})
                .await
                .map_err(|e| QgaError::Protocol(e.to_string()))
        }
    }

    /// Production connector — opens a real Unix socket to the guest agent.
    pub struct RealQgaConnector;

    #[async_trait]
    impl QgaConnector for RealQgaConnector {
        async fn connect(&self, path: &Path) -> Result<Box<dyn QgaControl>, QgaError> {
            Ok(Box::new(QapiQga::connect(path).await?))
        }
    }
}
