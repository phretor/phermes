//! Exercises the supervisor state machine with a mock launcher (no QEMU, no QMP socket).

use async_trait::async_trait;
use phermesd::config::{Console, Disk, DiskInterface, Firmware, Flavor, Net, NetModel, Resources, Vm, VmDef};
use phermesd::proto::VmState;
use phermesd::qemu::RuntimePaths;
use phermesd::qmp::{QmpControl, QmpError};
use phermesd::supervisor::{Launcher, Spawned, Supervisor, SupervisorError};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;
use std::time::Duration;

fn vm(id: &str) -> Vm {
    Vm {
        id: id.to_string(),
        def: VmDef {
            flavor: Flavor::Linux,
            resources: Resources { memory_mib: 512, vcpus: 1, cpu: "host".into() },
            firmware: Firmware { ovmf_code: "/a".into(), ovmf_vars_template: "/b".into() },
            disk: vec![Disk { path: "/c".into(), format: "qcow2".into(), interface: DiskInterface::VirtioScsi }],
            net: vec![Net { bridge: "vmbr0".into(), model: NetModel::VirtioNet, mac: None }],
            console: Console { serial: true, vnc: true },
        },
    }
}

struct MockQmp {
    shutdown_on_powerdown: bool,
    powered_down: Arc<AtomicBool>,
}

#[async_trait]
impl QmpControl for MockQmp {
    async fn is_running(&self) -> Result<bool, QmpError> {
        Ok(true)
    }
    async fn powerdown(&self) -> Result<(), QmpError> {
        self.powered_down.store(true, Ordering::SeqCst);
        Ok(())
    }
    async fn wait_shutdown(&mut self) -> Result<(), QmpError> {
        if self.shutdown_on_powerdown {
            Ok(())
        } else {
            std::future::pending::<()>().await;
            Ok(())
        }
    }
}

struct MockLauncher {
    next_pid: AtomicI32,
    shutdown_on_powerdown: bool,
    killed: Arc<AtomicI32>,
    cleaned: Arc<AtomicI32>,
}

impl MockLauncher {
    fn new(shutdown_on_powerdown: bool) -> (Self, Arc<AtomicI32>, Arc<AtomicI32>) {
        let killed = Arc::new(AtomicI32::new(0));
        let cleaned = Arc::new(AtomicI32::new(0));
        (
            Self {
                next_pid: AtomicI32::new(1000),
                shutdown_on_powerdown,
                killed: killed.clone(),
                cleaned: cleaned.clone(),
            },
            killed,
            cleaned,
        )
    }
}

#[async_trait]
impl Launcher for MockLauncher {
    async fn launch(&self, _vm: &Vm, _argv: &[String], _rt: &RuntimePaths) -> Result<Spawned, SupervisorError> {
        let pid = self.next_pid.fetch_add(1, Ordering::SeqCst);
        Ok(Spawned {
            pid,
            qmp: Box::new(MockQmp {
                shutdown_on_powerdown: self.shutdown_on_powerdown,
                powered_down: Arc::new(AtomicBool::new(false)),
            }),
        })
    }
    async fn reconnect(&self, _rt: &RuntimePaths) -> Result<Box<dyn QmpControl>, SupervisorError> {
        Ok(Box::new(MockQmp { shutdown_on_powerdown: true, powered_down: Arc::new(AtomicBool::new(false)) }))
    }
    fn force_kill(&self, _pid: i32) {
        self.killed.fetch_add(1, Ordering::SeqCst);
    }
    fn cleanup(&self, _rt: &RuntimePaths) {
        self.cleaned.fetch_add(1, Ordering::SeqCst);
    }
    fn is_alive(&self, _pid: i32) -> bool {
        true
    }
}

fn supervisor(launcher: MockLauncher, dir: &tempfile::TempDir) -> Supervisor {
    Supervisor::new(
        vec![vm("linux"), vm("windows")],
        dir.path().join("run"),
        dir.path().join("state.json"),
        Duration::from_millis(100),
        Box::new(launcher),
    )
}

#[tokio::test]
async fn activate_marks_running_and_status_reports_sockets() {
    let (launcher, _killed, _cleaned) = MockLauncher::new(true);
    let dir = tempfile::tempdir().unwrap();
    let mut sup = supervisor(launcher, &dir);
    let info = sup.activate("linux").await.unwrap();
    assert_eq!(info.state, VmState::Running);
    assert!(info.pid.is_some());
    assert!(info.vnc.is_some() && info.serial.is_some());

    let st = sup.status(None).unwrap();
    assert_eq!(st.id, "linux");
    assert_eq!(st.state, VmState::Running);
}

#[tokio::test]
async fn activating_second_vm_stops_the_first() {
    let (launcher, _killed, cleaned) = MockLauncher::new(true);
    let dir = tempfile::tempdir().unwrap();
    let mut sup = supervisor(launcher, &dir);
    sup.activate("linux").await.unwrap();
    let info = sup.activate("windows").await.unwrap();
    assert_eq!(info.id, "windows");
    assert_eq!(info.state, VmState::Running);
    assert_eq!(cleaned.load(Ordering::SeqCst), 1);
    assert_eq!(sup.status(None).unwrap().id, "windows");
}

#[tokio::test]
async fn stop_timeout_triggers_force_kill() {
    let (launcher, killed, cleaned) = MockLauncher::new(false);
    let dir = tempfile::tempdir().unwrap();
    let mut sup = supervisor(launcher, &dir);
    sup.activate("linux").await.unwrap();
    let info = sup.stop(None).await.unwrap();
    assert_eq!(info.state, VmState::Stopped);
    assert_eq!(killed.load(Ordering::SeqCst), 1);
    assert_eq!(cleaned.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn stop_without_active_errors() {
    let (launcher, _k, _c) = MockLauncher::new(true);
    let dir = tempfile::tempdir().unwrap();
    let mut sup = supervisor(launcher, &dir);
    assert!(matches!(sup.stop(None).await, Err(SupervisorError::NoActive)));
}

#[tokio::test]
async fn activate_unknown_id_errors() {
    let (launcher, _k, _c) = MockLauncher::new(true);
    let dir = tempfile::tempdir().unwrap();
    let mut sup = supervisor(launcher, &dir);
    assert!(matches!(sup.activate("nope").await, Err(SupervisorError::UnknownId(_))));
}

#[tokio::test]
async fn list_shows_defined_for_inactive_vms() {
    let (launcher, _k, _c) = MockLauncher::new(true);
    let dir = tempfile::tempdir().unwrap();
    let sup = supervisor(launcher, &dir);
    let all = sup.list();
    assert_eq!(all.len(), 2);
    assert!(all.iter().all(|i| i.state == VmState::Defined));
}
