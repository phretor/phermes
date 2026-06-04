//! Storage orchestration via mock Lvm/Btrfs/Qga (no root, no real tools).

use async_trait::async_trait;
use phermesd::btrfs::{BtrfsError, BtrfsOps};
use phermesd::lvm::{Lv, LvmError, LvmOps};
use phermesd::qga::{QgaConnector, QgaControl, QgaError};
use phermesd::storage::{Storage, StorageConfig, StorageError};
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

// ── helpers ──────────────────────────────────────────────────────────────────

/// Acquire a `Mutex` lock; recover from poison (previous test panicked).
fn locked<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
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
        locked(&self.lvs).push(lv(snap, &["@phermesd-snap"], origin, None));
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
}

#[async_trait]
impl BtrfsOps for MockBtrfs {
    async fn snapshot(&self, _src: &Path, dst: &Path) -> Result<(), BtrfsError> {
        locked(&self.calls).push(format!("snapshot {}", dst.display()));
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

// ── tests ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn provision_creates_tagged_volume() {
    let lvm = MockLvm::default();
    let lvs_handle = lvm.lvs.clone();
    let storage = Storage::new(
        cfg(),
        Box::new(lvm),
        Box::new(MockBtrfs::default()),
        Box::new(MockConnector::default()),
    );
    storage.provision(102, 40, None, false).await.unwrap();
    let lvs = lvs_handle.lock().unwrap();
    let disk = lvs.iter().find(|l| l.lv_name == "vm-102-disk-0").unwrap();
    assert!(disk.tags.iter().any(|t| t == "@phermesd"));
}

#[tokio::test]
async fn provision_existing_without_force_errors() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["@phermesd"], "", None));
    let storage = Storage::new(
        cfg(),
        Box::new(lvm),
        Box::new(MockBtrfs::default()),
        Box::new(MockConnector::default()),
    );
    assert!(matches!(
        storage.provision(102, 40, None, false).await,
        Err(StorageError::NotManaged(_))
    ));
}

#[tokio::test]
async fn delete_removes_disk_and_its_snapshots() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["@phermesd"], "", None));
    lvm.lvs
        .lock()
        .unwrap()
        .push(lv("vm-102-disk-0-snap-manual-x", &["@phermesd-snap"], "vm-102-disk-0", None));
    let calls = lvm.calls.clone();
    let storage = Storage::new(
        cfg(),
        Box::new(lvm.clone()),
        Box::new(MockBtrfs::default()),
        Box::new(MockConnector::default()),
    );
    let lvs = storage.list_lvs().await.unwrap();
    storage.delete(102, &lvs).await.unwrap();
    let recorded = calls.lock().unwrap().clone();
    assert!(
        recorded
            .iter()
            .any(|c| c.contains("remove") && c.contains("vm-102-disk-0-snap-manual-x")),
        "snapshot not removed: {recorded:?}"
    );
    assert!(
        recorded.iter().any(|c| c == "remove /dev/pve/vm-102-disk-0"),
        "disk not removed: {recorded:?}"
    );
}
