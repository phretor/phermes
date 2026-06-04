//! Storage orchestration via mock Lvm/Btrfs/Qga (no root, no real tools).

mod common;
use common::*;

use phermesd::storage::{Storage, StorageError};
use std::sync::atomic::Ordering;

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
    assert!(disk.tags.iter().any(|t| t == "phermesd"));
}

#[tokio::test]
async fn provision_existing_without_force_errors() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["phermesd"], "", None));
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
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["phermesd"], "", None));
    lvm.lvs
        .lock()
        .unwrap()
        .push(lv("vm-102-disk-0-snap-manual-x", &["phermesd-snap"], "vm-102-disk-0", None));
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

#[tokio::test]
async fn checkpoint_quiesces_and_thaws_when_active() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["phermesd"], "", None));
    lvm.lvs.lock().unwrap().push(lv("data", &[], "", Some(10.0)));
    let conn = MockConnector::default();
    let frozen = conn.frozen.clone();
    let storage = Storage::new(cfg(), Box::new(lvm), Box::new(MockBtrfs::default()), Box::new(conn));
    let cp = storage
        .checkpoint(102, phermesd::storage::SnapKind::Manual, Some("/run/phermesd/x/qga.sock".into()))
        .await
        .unwrap();
    assert_eq!(cp.vmid, 102);
    // freeze(+1) then thaw(-1) => back to 0
    assert_eq!(frozen.load(std::sync::atomic::Ordering::SeqCst), 0);
}

#[tokio::test]
async fn checkpoint_without_agent_is_crash_consistent() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["phermesd"], "", None));
    lvm.lvs.lock().unwrap().push(lv("data", &[], "", Some(10.0)));
    let conn = MockConnector::default();
    conn.fail.store(true, std::sync::atomic::Ordering::SeqCst); // agent down
    let storage = Storage::new(cfg(), Box::new(lvm), Box::new(MockBtrfs::default()), Box::new(conn));
    // Still succeeds (falls back to crash-consistent) even with a qga_sock provided.
    storage
        .checkpoint(102, phermesd::storage::SnapKind::Manual, Some("/x/qga.sock".into()))
        .await
        .unwrap();
}

#[tokio::test]
async fn auto_checkpoint_refused_when_pool_above_threshold() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["phermesd"], "", None));
    lvm.lvs.lock().unwrap().push(lv("data", &[], "", Some(95.0)));
    let storage = Storage::new(cfg(), Box::new(lvm), Box::new(MockBtrfs::default()), Box::new(MockConnector::default()));
    assert!(matches!(
        storage.checkpoint(102, phermesd::storage::SnapKind::Auto, None).await,
        Err(StorageError::PoolFull { .. })
    ));
}

#[tokio::test]
async fn checkpoint_tags_the_lv_snapshot() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["phermesd"], "", None));
    lvm.lvs.lock().unwrap().push(lv("data", &[], "", Some(10.0)));
    let lvs_handle = lvm.lvs.clone();
    let storage = Storage::new(cfg(), Box::new(lvm), Box::new(MockBtrfs::default()), Box::new(MockConnector::default()));
    storage.checkpoint(102, phermesd::storage::SnapKind::Manual, None).await.unwrap();
    let lvs = lvs_handle.lock().unwrap();
    let snap = lvs.iter().find(|l| l.lv_name.starts_with("vm-102-disk-0-snap-manual-")).unwrap();
    assert!(snap.tags.iter().any(|t| t == "phermesd-snap"), "snapshot LV must be tagged phermesd-snap");
}

#[tokio::test]
async fn auto_prune_keeps_last_n() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["phermesd"], "", None));
    lvm.lvs.lock().unwrap().push(lv("data", &[], "", Some(5.0)));
    // two pre-existing auto snaps; cfg() retention=2, so a 3rd prunes the oldest (20260101)
    lvm.lvs.lock().unwrap().push(lv(
        "vm-102-disk-0-snap-auto-20260101T000000Z",
        &["phermesd-snap"],
        "vm-102-disk-0",
        None,
    ));
    lvm.lvs.lock().unwrap().push(lv(
        "vm-102-disk-0-snap-auto-20260102T000000Z",
        &["phermesd-snap"],
        "vm-102-disk-0",
        None,
    ));
    let storage = Storage::new(
        cfg(),
        Box::new(lvm),
        Box::new(MockBtrfs::default()),
        Box::new(MockConnector::default()),
    );
    storage.checkpoint(102, phermesd::storage::SnapKind::Auto, None).await.unwrap();
    let cps = storage.checkpoints(102).await.unwrap();
    let autos: Vec<_> = cps.iter().filter(|c| c.kind == phermesd::storage::SnapKind::Auto).collect();
    assert_eq!(autos.len(), 2);
    assert!(!autos.iter().any(|c| c.utc == "20260101T000000Z"));
}

#[tokio::test]
async fn rollback_merges_disk_and_restores_overlay() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["phermesd"], "", None));
    lvm.lvs.lock().unwrap().push(lv(
        "vm-102-disk-0-snap-manual-20260601T000000Z",
        &["phermesd-snap"],
        "vm-102-disk-0",
        None,
    ));
    let lvm_calls = lvm.calls.clone();
    let btrfs = MockBtrfs::default();
    let btrfs_calls = btrfs.calls.clone();
    let storage = Storage::new(
        cfg(),
        Box::new(lvm),
        Box::new(btrfs),
        Box::new(MockConnector::default()),
    );
    storage.rollback(102, "20260601T000000Z").await.unwrap();
    assert!(lvm_calls
        .lock()
        .unwrap()
        .iter()
        .any(|c| c == "merge vm-102-disk-0-snap-manual-20260601T000000Z"));
    let bc = btrfs_calls.lock().unwrap();
    let del = bc.iter().position(|c| c.starts_with("delete"));
    let res = bc.iter().position(|c| c.starts_with("restore"));
    assert!(del.is_some() && res.is_some() && del < res, "delete must precede restore");
}

#[tokio::test]
async fn rollback_unknown_checkpoint_errors() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["phermesd"], "", None));
    let storage = Storage::new(
        cfg(),
        Box::new(lvm),
        Box::new(MockBtrfs::default()),
        Box::new(MockConnector::default()),
    );
    assert!(matches!(
        storage.rollback(102, "nope").await,
        Err(StorageError::NotFound(_))
    ));
}

#[tokio::test]
async fn checkpoint_rolls_back_lv_snap_and_thaws_when_overlay_fails() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["phermesd"], "", None));
    lvm.lvs.lock().unwrap().push(lv("data", &[], "", Some(10.0)));
    let lvm_calls = lvm.calls.clone();
    let btrfs = MockBtrfs::default();
    btrfs.fail.store(true, Ordering::SeqCst);
    let conn = MockConnector::default();
    let frozen = conn.frozen.clone();
    let storage = Storage::new(cfg(), Box::new(lvm), Box::new(btrfs), Box::new(conn));

    let res = storage
        .checkpoint(102, phermesd::storage::SnapKind::Manual, Some("/x/qga.sock".into()))
        .await;
    assert!(matches!(res, Err(StorageError::Btrfs(_))));
    // the LV snapshot we created was rolled back
    let calls = lvm_calls.lock().unwrap();
    assert!(calls.iter().any(|c| c.starts_with("snap vm-102-disk-0-snap-manual-")));
    assert!(calls.iter().any(|c| c.starts_with("remove /dev/pve/vm-102-disk-0-snap-manual-")));
    // guest thawed on the error path
    assert_eq!(frozen.load(Ordering::SeqCst), 0);
}
