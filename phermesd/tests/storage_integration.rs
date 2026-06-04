//! Gated: real LVM-thin pool + Btrfs overlay on loop devices.
//! Run with: `sudo -E cargo test --test storage_integration -- --ignored --nocapture`
//! Requires root, lvm2, btrfs-progs, qemu-utils.

use phermesd::btrfs::RealBtrfs;
use phermesd::lvm::RealLvm;
use phermesd::qga::RealQgaConnector;
use phermesd::storage::{SnapKind, Storage, StorageConfig};
use std::process::Command;

fn sh(argv: &[&str]) -> std::io::Result<()> {
    let Some((head, tail)) = argv.split_first() else {
        return Err(std::io::Error::other("empty command"));
    };
    let status = Command::new(head).args(tail).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!("command failed: {argv:?}")))
    }
}

fn losetup_attach(path: &str) -> std::io::Result<String> {
    let out = Command::new("losetup").args(["-f", "--show", path]).output()?;
    if !out.status.success() {
        return Err(std::io::Error::other("losetup failed"));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

#[tokio::test]
#[ignore = "needs root + lvm2 + btrfs-progs + loop devices"]
async fn provision_snapshot_rollback_on_loop_devices() {
    let dir = tempfile::tempdir().unwrap();
    let img = dir.path().join("pv.img");
    sh(&["truncate", "-s", "2G", img.to_str().unwrap()]).unwrap();
    let loopdev = losetup_attach(img.to_str().unwrap()).unwrap();
    sh(&["pvcreate", "-y", &loopdev]).unwrap();
    sh(&["vgcreate", "phermes_test", &loopdev]).unwrap();
    // --config disables udev sync so this runs in udev-less environments (CI/containers),
    // matching RealLvm's executor.
    sh(&[
        "lvcreate", "--config", "activation { udev_sync = 0 udev_rules = 0 }",
        "--type", "thin-pool", "-L", "1G", "-n", "data", "phermes_test",
    ])
    .unwrap();

    let bimg = dir.path().join("btrfs.img");
    sh(&["truncate", "-s", "512M", bimg.to_str().unwrap()]).unwrap();
    sh(&["mkfs.btrfs", "-f", bimg.to_str().unwrap()]).unwrap();
    let mnt = dir.path().join("overlay");
    std::fs::create_dir_all(&mnt).unwrap();
    sh(&["mount", "-o", "loop", bimg.to_str().unwrap(), mnt.to_str().unwrap()]).unwrap();
    // Model production: an @overlay subvolume (rollback deletes+recreates it) plus an
    // @snapshots dir to hold the read-only overlay snapshots — both under the btrfs root.
    let overlay = mnt.join("@overlay");
    sh(&["btrfs", "subvolume", "create", overlay.to_str().unwrap()]).unwrap();
    let snaps = mnt.join("@snapshots");
    std::fs::create_dir_all(&snaps).unwrap();

    let cfg = StorageConfig {
        vg: "phermes_test".into(),
        pool: "data".into(),
        overlay: overlay.clone(),
        snapshots_dir: snaps.clone(),
        retention: 2,
        pool_threshold: 95.0,
    };
    let storage = Storage::new(cfg, Box::new(RealLvm), Box::new(RealBtrfs), Box::new(RealQgaConnector));

    storage.provision(102, 1, None, false).await.unwrap();
    let cp = storage.checkpoint(102, SnapKind::Manual, None).await.unwrap();
    let cps = storage.checkpoints(102).await.unwrap();
    assert!(cps.iter().any(|c| c.utc == cp.utc));
    storage.rollback(102, &cp.utc).await.unwrap();

    let _ = Command::new("umount").arg(mnt.to_str().unwrap()).status();
    let _ = Command::new("vgremove").args(["-y", "phermes_test"]).status();
    let _ = Command::new("losetup").args(["-d", &loopdev]).status();
}
