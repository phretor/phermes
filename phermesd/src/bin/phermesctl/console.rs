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
}
