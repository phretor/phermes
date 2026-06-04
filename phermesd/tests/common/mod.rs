//! Shared mock harness for storage tests (used by `storage_lifecycle` and `supervisor_lifecycle`).

use async_trait::async_trait;
use phermesd::btrfs::{BtrfsError, BtrfsOps};
use phermesd::lvm::{Lv, LvmError, LvmOps};
use phermesd::qga::{QgaConnector, QgaControl, QgaError};
use phermesd::storage::StorageConfig;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

// ── helpers ──────────────────────────────────────────────────────────────────

/// Acquire a `Mutex` lock; recover from poison (previous test panicked).
pub fn locked<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[must_use]
pub fn lv(name: &str, tags: &[&str], origin: &str, dp: Option<f64>) -> Lv {
    Lv {
        lv_name: name.into(),
        tags: tags.iter().map(std::string::ToString::to_string).collect(),
        pool_lv: "data".into(),
        origin: origin.into(),
        data_percent: dp,
    }
}

#[must_use]
pub fn cfg() -> StorageConfig {
    StorageConfig { retention: 2, ..StorageConfig::default() }
}

// ── MockLvm ───────────────────────────────────────────────────────────────────

#[derive(Clone, Default)]
pub struct MockLvm {
    pub lvs: Arc<Mutex<Vec<Lv>>>,
    pub calls: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl LvmOps for MockLvm {
    async fn create_thin(
        &self,
        _vg: &str,
        _pool: &str,
        name: &str,
        _gb: u32,
    ) -> Result<(), LvmError> {
        locked(&self.calls).push(format!("create {name}"));
        locked(&self.lvs).push(lv(name, &[], "", None));
        Ok(())
    }

    async fn add_tag(&self, device: &str, tag: &str) -> Result<(), LvmError> {
        locked(&self.calls).push(format!("tag {device} {tag}"));
        let name = device.rsplit('/').next().unwrap_or(device).to_string();
        if let Some(l) = locked(&self.lvs).iter_mut().find(|l| l.lv_name == name) {
            l.tags.push(tag.to_string());
        }
        Ok(())
    }

    async fn snapshot(&self, _vg: &str, origin: &str, snap: &str) -> Result<(), LvmError> {
        locked(&self.calls).push(format!("snap {snap}"));
        locked(&self.lvs).push(lv(snap, &[], origin, None));
        Ok(())
    }

    async fn merge(&self, _vg: &str, snap: &str) -> Result<(), LvmError> {
        locked(&self.calls).push(format!("merge {snap}"));
        Ok(())
    }

    async fn remove(&self, device: &str) -> Result<(), LvmError> {
        locked(&self.calls).push(format!("remove {device}"));
        let name = device.rsplit('/').next().unwrap_or(device).to_string();
        locked(&self.lvs).retain(|l| l.lv_name != name);
        Ok(())
    }

    async fn list(&self, _vg: &str) -> Result<Vec<Lv>, LvmError> {
        Ok(locked(&self.lvs).clone())
    }
}

// ── MockBtrfs ─────────────────────────────────────────────────────────────────

#[derive(Clone, Default)]
pub struct MockBtrfs {
    pub calls: Arc<Mutex<Vec<String>>>,
    pub fail: Arc<AtomicBool>,
}

#[async_trait]
impl BtrfsOps for MockBtrfs {
    async fn snapshot(&self, _src: &Path, dst: &Path) -> Result<(), BtrfsError> {
        locked(&self.calls).push(format!("snapshot {}", dst.display()));
        if self.fail.load(Ordering::SeqCst) {
            return Err(BtrfsError::Failed {
                cmd: "btrfs subvolume snapshot".into(),
                code: 1,
                stderr: "injected".into(),
            });
        }
        Ok(())
    }

    async fn restore(&self, _ro: &Path, dst: &Path) -> Result<(), BtrfsError> {
        locked(&self.calls).push(format!("restore {}", dst.display()));
        Ok(())
    }

    async fn delete(&self, subvol: &Path) -> Result<(), BtrfsError> {
        locked(&self.calls).push(format!("delete {}", subvol.display()));
        Ok(())
    }
}

// ── MockConnector / CountingQga ───────────────────────────────────────────────

pub struct CountingQga {
    pub frozen: Arc<AtomicI64>,
}

#[async_trait]
impl QgaControl for CountingQga {
    async fn ping(&self) -> Result<(), QgaError> {
        Ok(())
    }

    async fn freeze(&self) -> Result<i64, QgaError> {
        self.frozen.fetch_add(1, Ordering::SeqCst);
        Ok(2)
    }

    async fn thaw(&self) -> Result<i64, QgaError> {
        self.frozen.fetch_sub(1, Ordering::SeqCst);
        Ok(2)
    }
}

#[derive(Clone, Default)]
pub struct MockConnector {
    pub frozen: Arc<AtomicI64>,
    pub fail: Arc<AtomicBool>,
}

#[async_trait]
impl QgaConnector for MockConnector {
    async fn connect(&self, _path: &Path) -> Result<Box<dyn QgaControl>, QgaError> {
        if self.fail.load(Ordering::SeqCst) {
            return Err(QgaError::Protocol("agent down".into()));
        }
        Ok(Box::new(CountingQga { frozen: self.frozen.clone() }))
    }
}
