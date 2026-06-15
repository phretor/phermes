//! Typed reader for the Apple OSK runtime secret (/etc/phermes/secrets/apple.osk).
//!
//! Slice #5b: macOS guests require the 64-char Apple OSK at QEMU launch. The
//! file is operator-supplied and never embedded in the install image for
//! production builds. This module enforces mode 0o400 and format validation
//! at read time so misconfigurations surface with actionable errors instead
//! of QEMU panics.

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// Canonical on-disk location for the Apple OSK secret.
pub const APPLE_OSK_PATH: &str = "/etc/phermes/secrets/apple.osk";

#[derive(Debug, thiserror::Error)]
pub enum SecretsError {
    #[error("apple OSK file not found: {path}")]
    NotFound { path: PathBuf },
    #[error("apple OSK file {path} has insecure mode {mode:o}; expected 0o400 (no group/other access)")]
    BadPermissions { path: PathBuf, mode: u32 },
    #[error("apple OSK has bad format: {reason}")]
    BadFormat { reason: &'static str },
    #[error("apple OSK read I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Read, validate, and return the Apple OSK from `path`.
///
/// # Errors
///
/// * `NotFound` if the file does not exist
/// * `BadPermissions` if mode has any group/other bits set
/// * `BadFormat` if content isn't exactly 64 hex chars after trimming trailing whitespace
/// * `Io` on any other read error
pub fn read_apple_osk(path: &Path) -> Result<String, SecretsError> {
    let meta = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(SecretsError::NotFound { path: path.to_path_buf() });
        }
        Err(e) => return Err(SecretsError::Io(e)),
    };
    let mode = meta.permissions().mode() & 0o777;
    if mode & 0o077 != 0 {
        return Err(SecretsError::BadPermissions { path: path.to_path_buf(), mode });
    }
    let raw = std::fs::read_to_string(path)?;
    let trimmed = raw.trim_end();
    if trimmed.len() != 64 {
        return Err(SecretsError::BadFormat { reason: "expected exactly 64 chars after trim" });
    }
    if !trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(SecretsError::BadFormat { reason: "expected 64 hex chars [0-9a-fA-F]" });
    }
    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    fn write_osk(dir: &Path, mode: u32, body: &str) -> PathBuf {
        let p = dir.join("apple.osk");
        fs::write(&p, body).unwrap();
        let mut perms = fs::metadata(&p).unwrap().permissions();
        perms.set_mode(mode);
        fs::set_permissions(&p, perms).unwrap();
        p
    }

    fn valid_osk() -> String {
        "0123456789abcdef".repeat(4)
    }

    #[test]
    fn returns_ok_for_valid_file() {
        let tmp = tempfile::tempdir().unwrap();
        let osk = valid_osk();
        let p = write_osk(tmp.path(), 0o400, &osk);
        assert_eq!(read_apple_osk(&p).unwrap(), osk);
    }

    #[test]
    fn strips_trailing_whitespace() {
        let tmp = tempfile::tempdir().unwrap();
        let osk = valid_osk();
        let p = write_osk(tmp.path(), 0o400, &format!("{osk}\n"));
        assert_eq!(read_apple_osk(&p).unwrap(), osk);
    }

    #[test]
    fn rejects_short_string() {
        let tmp = tempfile::tempdir().unwrap();
        let p = write_osk(tmp.path(), 0o400, "deadbeef");
        assert!(matches!(read_apple_osk(&p), Err(SecretsError::BadFormat { .. })));
    }

    #[test]
    fn rejects_non_hex() {
        let tmp = tempfile::tempdir().unwrap();
        let p = write_osk(tmp.path(), 0o400, &"z".repeat(64));
        assert!(matches!(read_apple_osk(&p), Err(SecretsError::BadFormat { .. })));
    }

    #[test]
    fn rejects_world_readable_mode() {
        let tmp = tempfile::tempdir().unwrap();
        let osk = valid_osk();
        let p = write_osk(tmp.path(), 0o444, &osk);
        assert!(matches!(read_apple_osk(&p), Err(SecretsError::BadPermissions { .. })));
    }

    #[test]
    fn returns_not_found_when_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let p = tmp.path().join("nope.osk");
        assert!(matches!(read_apple_osk(&p), Err(SecretsError::NotFound { .. })));
    }
}
