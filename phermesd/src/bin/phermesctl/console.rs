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
}
