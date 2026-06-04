//! Btrfs overlay snapshot operations. The overlay is a host-side subvolume; snapshots
//! and rollbacks happen on the host (no guest involvement).

use async_trait::async_trait;
use std::path::Path;
use tokio::process::Command;

#[must_use]
pub fn snapshot_argv(src: &Path, dst: &Path) -> Vec<String> {
    vec![
        "btrfs".into(), "subvolume".into(), "snapshot".into(), "-r".into(),
        src.display().to_string(), dst.display().to_string(),
    ]
}

#[must_use]
pub fn restore_argv(ro_snap: &Path, dst: &Path) -> Vec<String> {
    vec![
        "btrfs".into(), "subvolume".into(), "snapshot".into(),
        ro_snap.display().to_string(), dst.display().to_string(),
    ]
}

#[must_use]
pub fn delete_argv(subvol: &Path) -> Vec<String> {
    vec!["btrfs".into(), "subvolume".into(), "delete".into(), subvol.display().to_string()]
}

#[derive(Debug, thiserror::Error)]
pub enum BtrfsError {
    #[error("running {cmd}: {source}")]
    Spawn {
        cmd: String,
        #[source]
        source: std::io::Error,
    },
    #[error("{cmd} failed ({code}): {stderr}")]
    Failed { cmd: String, code: i32, stderr: String },
}

/// The Btrfs side-effect seam (overlay snapshot/restore/delete).
#[async_trait]
pub trait BtrfsOps: Send + Sync {
    /// Read-only snapshot `src` -> `dst`.
    ///
    /// # Errors
    /// Returns `BtrfsError` if the `btrfs` command fails to spawn or exits non-zero.
    async fn snapshot(&self, src: &Path, dst: &Path) -> Result<(), BtrfsError>;
    /// Recreate a writable subvolume at `dst` from read-only `ro_snap`.
    ///
    /// # Errors
    /// Returns `BtrfsError` if the `btrfs` command fails.
    async fn restore(&self, ro_snap: &Path, dst: &Path) -> Result<(), BtrfsError>;
    /// Delete a subvolume.
    ///
    /// # Errors
    /// Returns `BtrfsError` if the `btrfs` command fails.
    async fn delete(&self, subvol: &Path) -> Result<(), BtrfsError>;
}

pub struct RealBtrfs;

impl RealBtrfs {
    async fn run(argv: &[String]) -> Result<(), BtrfsError> {
        let cmd = argv.join(" ");
        let Some((head, tail)) = argv.split_first() else {
            unreachable!("argv builders always produce non-empty slices")
        };
        let output = Command::new(head)
            .args(tail)
            .output()
            .await
            .map_err(|source| BtrfsError::Spawn { cmd: cmd.clone(), source })?;
        if !output.status.success() {
            return Err(BtrfsError::Failed {
                cmd,
                code: output.status.code().unwrap_or(-1),
                stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
            });
        }
        Ok(())
    }
}

#[async_trait]
impl BtrfsOps for RealBtrfs {
    async fn snapshot(&self, src: &Path, dst: &Path) -> Result<(), BtrfsError> {
        Self::run(&snapshot_argv(src, dst)).await
    }
    async fn restore(&self, ro_snap: &Path, dst: &Path) -> Result<(), BtrfsError> {
        Self::run(&restore_argv(ro_snap, dst)).await
    }
    async fn delete(&self, subvol: &Path) -> Result<(), BtrfsError> {
        Self::run(&delete_argv(subvol)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn snapshot_argv_is_readonly() {
        let a = snapshot_argv(Path::new("/ov"), Path::new("/snaps/overlay-auto-x"));
        assert_eq!(a, vec!["btrfs", "subvolume", "snapshot", "-r", "/ov", "/snaps/overlay-auto-x"]);
    }

    #[test]
    fn restore_argv_is_writable_snapshot() {
        let a = restore_argv(Path::new("/snaps/overlay-auto-x"), Path::new("/ov"));
        assert_eq!(a, vec!["btrfs", "subvolume", "snapshot", "/snaps/overlay-auto-x", "/ov"]);
    }

    #[test]
    fn delete_argv_removes_subvolume() {
        assert_eq!(delete_argv(Path::new("/ov")),
                   vec!["btrfs", "subvolume", "delete", "/ov"]);
    }
}
