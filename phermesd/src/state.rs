//! Runtime state persisted to /run/phermesd/state.json for re-adopt after restart.

use crate::config::Flavor;
use nix::errno::Errno;
use nix::sys::signal::kill;
use nix::unistd::Pid;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VmRuntime {
    pub id: String,
    pub flavor: Flavor,
    pub pid: i32,
    pub qmp: PathBuf,
    #[serde(default)]
    pub serial: Option<PathBuf>,
    #[serde(default)]
    pub vnc: Option<PathBuf>,
    pub started_at: u64,
}

/// `PHermes` runs at most one active VM, so state is a single optional slot.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    #[serde(default)]
    pub active: Option<VmRuntime>,
}

#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("io on {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("decoding {path}: {source}")]
    Decode {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("encoding state: {0}")]
    Encode(#[source] serde_json::Error),
}

impl State {
    /// Load state; a missing file means no active VM (not an error).
    ///
    /// # Errors
    /// Returns `StateError::Io` on a read failure other than not-found, or
    /// `StateError::Decode` if the file is not valid state JSON.
    pub fn load(path: &Path) -> Result<Self, StateError> {
        let text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(source) => return Err(StateError::Io { path: path.to_path_buf(), source }),
        };
        serde_json::from_str(&text).map_err(|source| StateError::Decode {
            path: path.to_path_buf(),
            source,
        })
    }

    /// Atomically persist state (write temp + rename).
    ///
    /// # Errors
    /// Returns `StateError::Encode` if serialization fails, or `StateError::Io`
    /// if the temp write or rename fails.
    pub fn save(&self, path: &Path) -> Result<(), StateError> {
        let json = serde_json::to_string_pretty(self).map_err(StateError::Encode)?;
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, json).map_err(|source| StateError::Io {
            path: tmp.clone(),
            source,
        })?;
        std::fs::rename(&tmp, path).map_err(|source| StateError::Io {
            path: path.to_path_buf(),
            source,
        })
    }
}

/// Whether `pid` is a live process. Signal 0 probes existence; EPERM still means alive.
#[must_use]
pub fn pid_alive(pid: i32) -> bool {
    match kill(Pid::from_raw(pid), None) {
        Ok(()) | Err(Errno::EPERM) => true,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Flavor;

    fn sample_runtime() -> VmRuntime {
        VmRuntime {
            id: "linux".to_string(),
            flavor: Flavor::Linux,
            pid: 4321,
            qmp: "/run/phermesd/linux/qmp.sock".into(),
            serial: Some("/run/phermesd/linux/serial.sock".into()),
            vnc: Some("/run/phermesd/linux/vnc.sock".into()),
            started_at: 1_700_000_000,
        }
    }

    #[test]
    fn save_then_load_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("state.json");
        let st = State { active: Some(sample_runtime()) };
        st.save(&path).unwrap();
        let loaded = State::load(&path).unwrap();
        assert_eq!(loaded, st);
    }

    #[test]
    fn load_missing_file_yields_empty_state() {
        let dir = tempfile::tempdir().unwrap();
        let loaded = State::load(&dir.path().join("absent.json")).unwrap();
        assert_eq!(loaded, State::default());
        assert!(loaded.active.is_none());
    }

    #[test]
    fn pid_alive_true_for_self_false_for_reaped() {
        let me = i32::try_from(std::process::id()).unwrap();
        assert!(pid_alive(me));
        // PID 2^31-1 is effectively never live.
        assert!(!pid_alive(i32::MAX));
    }
}
