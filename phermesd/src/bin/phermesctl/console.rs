//! Console proxy for `phermesctl console <id>`.
//!
//! Opens the per-VM serial socket directly (after discovering its path via the
//! existing UDS Status round-trip), puts the operator's terminal in raw mode,
//! and proxies bytes both ways until Ctrl-] or a signal.

use std::path::PathBuf;

/// Detach key: ASCII 0x1d (Ctrl-]).
pub const ESCAPE_BYTE: u8 = 0x1d;

#[derive(Debug, thiserror::Error)]
pub enum ConsoleError {
    #[error("VM '{0}' is not active (no serial socket); activate it first")]
    NotActive(String),
    #[error("connecting to {path}: {source}")]
    Connect {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("UDS control protocol: {0}")]
    Control(String),
    #[error("terminal io: {0}")]
    Tty(String),
}

/// Return the index of the first `ESCAPE_BYTE` in `bytes`, or `None`.
#[must_use]
pub fn find_escape(bytes: &[u8]) -> Option<usize> {
    bytes.iter().position(|&b| b == ESCAPE_BYTE)
}

use nix::sys::termios::{self, ControlFlags, InputFlags, LocalFlags, SetArg, Termios};
use std::os::fd::{AsRawFd, BorrowedFd, RawFd};

/// RAII guard that puts a tty into raw mode and restores the original termios
/// on drop. Drop fires on every exit path, including panic unwind — the
/// operator's terminal is never left in raw mode.
pub struct RawTtyGuard {
    fd: RawFd,
    original: Termios,
}

impl RawTtyGuard {
    /// Switch `fd` to raw mode. `fd` MUST be a tty (caller checks).
    ///
    /// # Errors
    /// Returns `ConsoleError::Tty` if `tcgetattr` or `tcsetattr` fails.
    pub fn enter(fd: BorrowedFd<'_>) -> Result<Self, ConsoleError> {
        let raw = fd.as_raw_fd();
        let original = termios::tcgetattr(fd)
            .map_err(|e| ConsoleError::Tty(format!("tcgetattr: {e}")))?;
        let mut raw_mode = original.clone();
        // Input flags: don't translate CR->NL, don't pause on Ctrl-S/Q.
        raw_mode.input_flags.remove(
            InputFlags::IGNBRK
                | InputFlags::BRKINT
                | InputFlags::PARMRK
                | InputFlags::ISTRIP
                | InputFlags::INLCR
                | InputFlags::IGNCR
                | InputFlags::ICRNL
                | InputFlags::IXON,
        );
        // Local flags: no echo, no canonical mode, no signal generation
        // (Ctrl-C must reach the guest as a byte, not raise SIGINT in phermesctl).
        raw_mode.local_flags.remove(
            LocalFlags::ECHO
                | LocalFlags::ECHONL
                | LocalFlags::ICANON
                | LocalFlags::ISIG
                | LocalFlags::IEXTEN,
        );
        // 8-bit characters; no parity bit.
        raw_mode.control_flags.remove(ControlFlags::CSIZE | ControlFlags::PARENB);
        raw_mode.control_flags.insert(ControlFlags::CS8);
        // OPOST left ON intentionally: the operator reads the guest's text on
        // their terminal; OPOST handles guest "\n" -> "\r\n" translation. Raw
        // for input, cooked for output.
        termios::tcsetattr(fd, SetArg::TCSANOW, &raw_mode)
            .map_err(|e| ConsoleError::Tty(format!("tcsetattr (raw): {e}")))?;
        Ok(Self { fd: raw, original })
    }
}

impl Drop for RawTtyGuard {
    fn drop(&mut self) {
        // Safety: the fd was valid when we entered; we never close it ourselves.
        // If the operator closed it (e.g., process exit), tcsetattr fails
        // silently — there is no useful action here.
        let fd = unsafe { BorrowedFd::borrow_raw(self.fd) };
        let _ = termios::tcsetattr(fd, SetArg::TCSANOW, &self.original);
    }
}

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Why the pump returned.
#[derive(Debug, PartialEq, Eq)]
pub enum PumpExit {
    /// Operator hit Ctrl-].
    Detached,
    /// The remote (QEMU's serial socket) closed.
    Disconnected,
}

/// Bidirectional byte pump. Reads from `stdin_r` and writes to `sock_w` (with
/// Ctrl-] detection); reads from `sock_r` and writes to `stdout_w` (flushed
/// after every chunk for interactive responsiveness). Returns on the first of:
///   * `stdin_r` yields a chunk containing 0x1d (`PumpExit::Detached`)
///   * `sock_r` reports EOF (`PumpExit::Disconnected`)
///   * Either side errors (returns `ConsoleError::Tty`).
///
/// Signals are handled by the caller — keep this function pure async I/O so it
/// is testable with `tokio::io::duplex`.
///
/// # Errors
/// Returns `ConsoleError::Tty` on any I/O failure during the pump.
pub async fn run_pump<SR, SW, IR, OW>(
    mut sock_r: SR,
    mut sock_w: SW,
    mut stdin_r: IR,
    mut stdout_w: OW,
) -> Result<PumpExit, ConsoleError>
where
    SR: AsyncRead + Unpin,
    SW: AsyncWrite + Unpin,
    IR: AsyncRead + Unpin,
    OW: AsyncWrite + Unpin,
{
    let mut stdin_buf = [0u8; 4096];
    let mut sock_buf = [0u8; 4096];
    loop {
        tokio::select! {
            // stdin -> sock_w, with Ctrl-] detection
            r = stdin_r.read(&mut stdin_buf) => {
                let n = r.map_err(|e| ConsoleError::Tty(format!("stdin read: {e}")))?;
                if n == 0 {
                    // stdin closed (e.g., operator redirected from /dev/null)
                    return Ok(PumpExit::Detached);
                }
                let chunk = &stdin_buf[..n];
                if let Some(idx) = find_escape(chunk) {
                    if idx > 0 {
                        sock_w.write_all(&chunk[..idx]).await
                            .map_err(|e| ConsoleError::Tty(format!("sock write: {e}")))?;
                    }
                    return Ok(PumpExit::Detached);
                }
                sock_w.write_all(chunk).await
                    .map_err(|e| ConsoleError::Tty(format!("sock write: {e}")))?;
            }
            // sock_r -> stdout_w, with flush after every chunk
            r = sock_r.read(&mut sock_buf) => {
                let n = r.map_err(|e| ConsoleError::Tty(format!("sock read: {e}")))?;
                if n == 0 {
                    return Ok(PumpExit::Disconnected);
                }
                stdout_w.write_all(&sock_buf[..n]).await
                    .map_err(|e| ConsoleError::Tty(format!("stdout write: {e}")))?;
                stdout_w.flush().await
                    .map_err(|e| ConsoleError::Tty(format!("stdout flush: {e}")))?;
            }
        }
    }
}

use phermesd::proto::{encode_line, Request, Response, VmInfo};
use std::io::Write as _;
use std::os::fd::AsFd;
use std::path::Path;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UnixStream;

/// Top-level entry for `phermesctl console <id>`.
///
/// 1. Sends `Request::Status` over the existing UDS to discover the serial.sock path.
/// 2. Connects to that Unix socket directly.
/// 3. Enters raw mode on stdin.
/// 4. Races `run_pump` against SIGINT and SIGTERM.
/// 5. Restores the terminal (RAII), prints a one-word status to stderr, returns.
///
/// # Errors
/// Returns `ConsoleError` for protocol, connect, and tty failures. Successful
/// exits (Detached, Disconnected, signal) return `Ok(())`.
pub async fn run_console(id: &str, control_sock: &Path) -> Result<(), ConsoleError> {
    // 1. Status round-trip on the control socket
    let stream = UnixStream::connect(control_sock).await.map_err(|e| ConsoleError::Connect {
        path: control_sock.to_path_buf(),
        source: e,
    })?;
    let (read, mut write) = stream.into_split();
    let req = Request::Status { id: Some(id.to_string()) };
    let line = encode_line(&req).map_err(|e| ConsoleError::Control(e.to_string()))?;
    tokio::io::AsyncWriteExt::write_all(&mut write, line.as_bytes())
        .await
        .map_err(|e| ConsoleError::Control(format!("write: {e}")))?;
    let mut lines = BufReader::new(read).lines();
    let reply = lines
        .next_line()
        .await
        .map_err(|e| ConsoleError::Control(format!("read: {e}")))?
        .ok_or_else(|| ConsoleError::Control("daemon closed connection".into()))?;
    let resp: Response = serde_json::from_str(&reply)
        .map_err(|e| ConsoleError::Control(format!("decode: {e}")))?;
    if !resp.ok {
        let kind = resp.error.as_ref().map_or("unknown", |e| e.kind.as_str());
        let msg = resp.error.as_ref().map_or("", |e| e.message.as_str());
        return Err(ConsoleError::Control(format!("{kind}: {msg}")));
    }
    let data = resp
        .data
        .ok_or_else(|| ConsoleError::Control("Status response missing data".into()))?;
    let info: VmInfo = serde_json::from_value(data)
        .map_err(|e| ConsoleError::Control(format!("VmInfo: {e}")))?;
    let serial = info.serial.ok_or_else(|| ConsoleError::NotActive(id.to_string()))?;

    // 2. Open the serial socket
    let serial_stream = UnixStream::connect(&serial).await.map_err(|e| ConsoleError::Connect {
        path: serial.clone(),
        source: e,
    })?;
    let (sock_r, sock_w) = serial_stream.into_split();

    // 3. Raw mode on stdin
    let stdin = std::io::stdin();
    let _guard = RawTtyGuard::enter(stdin.as_fd())?;

    // 4. Race pump against signals
    let stdin_async = tokio::io::stdin();
    let stdout_async = tokio::io::stdout();
    let pump = run_pump(sock_r, sock_w, stdin_async, stdout_async);

    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .map_err(|e| ConsoleError::Tty(format!("install sigterm: {e}")))?;

    let exit_msg = tokio::select! {
        result = pump => {
            match result? {
                PumpExit::Detached => "Detached.",
                PumpExit::Disconnected => "Disconnected.",
            }
        }
        _ = tokio::signal::ctrl_c() => "Interrupted.",
        _ = sigterm.recv() => "Terminated.",
    };

    // 5. RawTtyGuard drops here -> terminal restored.
    // Write status to stderr so piping stdout is unaffected.
    let _ = writeln!(std::io::stderr(), "{exit_msg}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_escape_returns_none_for_empty_and_clean_chunks() {
        assert_eq!(find_escape(b""), None);
        assert_eq!(find_escape(b"hello"), None);
        assert_eq!(find_escape(b"\x00\x01\x02"), None);
    }

    #[test]
    fn find_escape_returns_index_at_start_middle_end() {
        assert_eq!(find_escape(b"\x1d"), Some(0));
        assert_eq!(find_escape(b"hi\x1dthere"), Some(2));
        assert_eq!(find_escape(b"end\x1d"), Some(3));
    }

    #[test]
    fn find_escape_returns_first_occurrence_only() {
        assert_eq!(find_escape(b"\x1dabc\x1d"), Some(0));
        assert_eq!(find_escape(b"ab\x1dcd\x1def"), Some(2));
    }

    #[test]
    fn raw_tty_guard_clears_canonical_flags_and_restores_on_drop() {
        use nix::pty::openpty;
        use std::os::fd::AsFd;

        let pty = openpty(None, None).expect("openpty");
        let slave_fd = pty.slave.as_fd();

        let original = termios::tcgetattr(slave_fd).expect("tcgetattr (pre)");
        assert!(original.local_flags.contains(LocalFlags::ICANON));
        assert!(original.local_flags.contains(LocalFlags::ECHO));
        assert!(original.local_flags.contains(LocalFlags::ISIG));

        {
            let _guard = RawTtyGuard::enter(slave_fd).expect("enter raw");
            let raw = termios::tcgetattr(slave_fd).expect("tcgetattr (raw)");
            assert!(!raw.local_flags.contains(LocalFlags::ICANON));
            assert!(!raw.local_flags.contains(LocalFlags::ECHO));
            assert!(!raw.local_flags.contains(LocalFlags::ISIG));
            assert!(!raw.local_flags.contains(LocalFlags::IEXTEN));
            assert!(!raw.input_flags.contains(InputFlags::ICRNL));
            assert!(!raw.input_flags.contains(InputFlags::IXON));
        }
        // Guard dropped; flags restored.
        let after = termios::tcgetattr(slave_fd).expect("tcgetattr (post)");
        assert_eq!(after.local_flags, original.local_flags);
        assert_eq!(after.input_flags, original.input_flags);

        drop(pty);
    }

    #[tokio::test]
    async fn pump_forwards_bytes_in_both_directions() {
        use tokio::io::AsyncWriteExt as _;
        let (sock_client, mut sock_server) = tokio::io::duplex(64);
        let (sock_r, sock_w) = tokio::io::split(sock_client);
        let (stdin_w_tx, stdin_r) = tokio::io::duplex(64);
        let (_unused_r, mut stdin_w) = tokio::io::split(stdin_w_tx);
        let stdout = Vec::<u8>::new();

        let pump = tokio::spawn(async move {
            run_pump(sock_r, sock_w, stdin_r, stdout).await
        });

        // operator types "world" on stdin -> sock_server should read it
        stdin_w.write_all(b"world").await.unwrap();
        let mut buf = [0u8; 5];
        tokio::io::AsyncReadExt::read_exact(&mut sock_server, &mut buf).await.unwrap();
        assert_eq!(&buf, b"world");

        // close the server side and stdin -> pump returns Disconnected
        drop(sock_server);
        drop(stdin_w);

        let result = pump.await.unwrap().unwrap();
        assert_eq!(result, PumpExit::Disconnected);
    }

    /// Shared-buffer writer for testing stdout side of the pump.
    struct ArcWriter(std::sync::Arc<tokio::sync::Mutex<Vec<u8>>>);
    impl tokio::io::AsyncWrite for ArcWriter {
        fn poll_write(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
            buf: &[u8],
        ) -> std::task::Poll<std::io::Result<usize>> {
            let mut guard = self.0.try_lock().expect("uncontended in test");
            guard.extend_from_slice(buf);
            std::task::Poll::Ready(Ok(buf.len()))
        }
        fn poll_flush(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            std::task::Poll::Ready(Ok(()))
        }
        fn poll_shutdown(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            std::task::Poll::Ready(Ok(()))
        }
    }

    #[tokio::test]
    async fn pump_writes_stdout_bytes_and_flushes() {
        use std::sync::Arc;
        use tokio::io::AsyncWriteExt as _;
        use tokio::sync::Mutex;

        // Wrap stdout in an Arc<Mutex<Vec<u8>>> we can read from outside the pump.
        let stdout: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
        let (sock_client, mut sock_server) = tokio::io::duplex(64);
        let (sock_r, sock_w) = tokio::io::split(sock_client);
        let (_stdin_w_unused, stdin_r) = tokio::io::duplex(64);
        let writer = ArcWriter(stdout.clone());

        let pump = tokio::spawn(async move {
            run_pump(sock_r, sock_w, stdin_r, writer).await
        });

        sock_server.write_all(b"hello").await.unwrap();
        // Give the pump a tick to copy the bytes.
        tokio::task::yield_now().await;
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        {
            let guard = stdout.lock().await;
            assert_eq!(guard.as_slice(), b"hello");
        }
        drop(sock_server);
        let result = pump.await.unwrap().unwrap();
        assert_eq!(result, PumpExit::Disconnected);
    }

    #[tokio::test]
    async fn pump_returns_detached_on_ctrl_bracket() {
        use tokio::io::AsyncWriteExt as _;

        let (sock_client, mut sock_server) = tokio::io::duplex(64);
        let (sock_r, sock_w) = tokio::io::split(sock_client);
        let (stdin_w_side, stdin_r) = tokio::io::duplex(64);
        let (_unused_r, mut stdin_w) = tokio::io::split(stdin_w_side);
        let stdout = Vec::<u8>::new();

        let pump = tokio::spawn(async move {
            run_pump(sock_r, sock_w, stdin_r, stdout).await
        });

        // Send bytes followed by Ctrl-]. The bytes before the escape should
        // reach sock_server; the pump should return PumpExit::Detached.
        stdin_w.write_all(b"abc\x1ddef").await.unwrap();

        let mut buf = [0u8; 3];
        tokio::io::AsyncReadExt::read_exact(&mut sock_server, &mut buf).await.unwrap();
        assert_eq!(&buf, b"abc");

        let result = pump.await.unwrap().unwrap();
        assert_eq!(result, PumpExit::Detached);
    }

    #[tokio::test]
    async fn pump_returns_disconnected_on_socket_eof() {
        let (sock_client, sock_server) = tokio::io::duplex(64);
        let (sock_r, sock_w) = tokio::io::split(sock_client);
        let (_unused_w, stdin_r) = tokio::io::duplex(64);
        let stdout = Vec::<u8>::new();

        let pump = tokio::spawn(async move {
            run_pump(sock_r, sock_w, stdin_r, stdout).await
        });

        drop(sock_server); // EOF on the socket
        let result = pump.await.unwrap().unwrap();
        assert_eq!(result, PumpExit::Disconnected);
    }
}
