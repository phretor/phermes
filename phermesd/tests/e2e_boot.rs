//! Gated end-to-end: boot the existing Debian node under phermesd on a /dev/kvm host.
//! Run with: `cargo test --test e2e_boot -- --ignored --nocapture`
//! Requires: /dev/kvm, qemu-system-x86_64, OVMF, a prebuilt qcow2 at `PHERMESD_E2E_DISK`,
//! an existing bridge in `PHERMESD_E2E_BRIDGE` (default vmbr0), and the qemu bridge helper
//! allowing that bridge (/etc/qemu/bridge.conf + setuid qemu-bridge-helper). Root likely needed.

use phermesd::config::load_file;
use phermesd::launcher::QemuLauncher;
use phermesd::proto::VmState;
use phermesd::supervisor::Supervisor;
use std::time::Duration;

#[tokio::test]
#[ignore = "needs /dev/kvm, qemu, OVMF, a real disk and bridge"]
async fn boots_node_activate_stop_readopt() {
    let disk = std::env::var("PHERMESD_E2E_DISK").expect("set PHERMESD_E2E_DISK to a qcow2 path");
    let bridge = std::env::var("PHERMESD_E2E_BRIDGE").unwrap_or_else(|_| "vmbr0".to_string());
    let dir = tempfile::tempdir().unwrap();
    let def_path = dir.path().join("linux.toml");
    std::fs::write(
        &def_path,
        format!(
            "flavor = \"linux\"\n[resources]\nmemory_mib = 2048\nvcpus = 2\n\
             [firmware]\novmf_code = \"/usr/share/OVMF/OVMF_CODE.fd\"\n\
             ovmf_vars_template = \"/usr/share/OVMF/OVMF_VARS.fd\"\n\
             [[disk]]\npath = \"{disk}\"\nformat = \"qcow2\"\n\
             [[net]]\nbridge = \"{bridge}\"\n[console]\nserial = true\nvnc = true\n"
        ),
    )
    .unwrap();
    let vm = load_file(&def_path).unwrap();

    let make = || {
        Supervisor::new(
            vec![vm.clone()],
            dir.path().join("run"),
            dir.path().join("run/state.json"),
            Duration::from_secs(30),
            Box::new(QemuLauncher::default()),
        )
    };

    let mut sup = make();
    let info = sup.activate("linux").await.unwrap();
    assert_eq!(info.state, VmState::Running);
    let pid = info.pid.unwrap();
    assert!(phermesd::state::pid_alive(pid));

    let mut sup2 = make();
    sup2.readopt().await.unwrap();
    assert_eq!(sup2.status(None).unwrap().state, VmState::Running);
    assert!(phermesd::state::pid_alive(pid));

    let stopped = sup2.stop(None).await.unwrap();
    assert_eq!(stopped.state, VmState::Stopped);
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(!phermesd::state::pid_alive(pid));
}
