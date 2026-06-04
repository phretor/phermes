//! Exercises the supervisor state machine with a mock launcher (no QEMU, no QMP socket).

mod common;
use common::{cfg, lv, MockBtrfs, MockConnector, MockLvm};

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

struct FailingQmp;
#[async_trait]
impl QmpControl for FailingQmp {
    async fn is_running(&self) -> Result<bool, QmpError> {
        Err(QmpError::Protocol("handshake failed".into()))
    }
    async fn powerdown(&self) -> Result<(), QmpError> {
        Ok(())
    }
    async fn wait_shutdown(&mut self) -> Result<(), QmpError> {
        Ok(())
    }
}

struct FailingLauncher {
    killed: Arc<AtomicI32>,
    cleaned: Arc<AtomicI32>,
}

#[async_trait]
impl Launcher for FailingLauncher {
    async fn launch(&self, _vm: &Vm, _argv: &[String], _rt: &RuntimePaths) -> Result<Spawned, SupervisorError> {
        Ok(Spawned { pid: 2222, qmp: Box::new(FailingQmp) })
    }
    async fn reconnect(&self, _rt: &RuntimePaths) -> Result<Box<dyn QmpControl>, SupervisorError> {
        Ok(Box::new(FailingQmp))
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

#[tokio::test]
async fn activate_with_failed_qmp_confirmation_kills_and_cleans() {
    let dir = tempfile::tempdir().unwrap();
    let killed = Arc::new(AtomicI32::new(0));
    let cleaned = Arc::new(AtomicI32::new(0));
    let mut sup = Supervisor::new(
        vec![vm("linux")],
        dir.path().join("run"),
        dir.path().join("state.json"),
        Duration::from_millis(100),
        Box::new(FailingLauncher { killed: killed.clone(), cleaned: cleaned.clone() }),
    );
    let res = sup.activate("linux").await;
    assert!(matches!(res, Err(SupervisorError::Qmp(_))));
    assert_eq!(killed.load(Ordering::SeqCst), 1);
    assert_eq!(cleaned.load(Ordering::SeqCst), 1);
    assert!(matches!(sup.status(None), Err(SupervisorError::NoActive)));
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

/// A launcher whose `is_alive` is configurable, to simulate live vs. dead re-adopt.
struct AdoptLauncher {
    alive: bool,
    cleaned: Arc<AtomicI32>,
}

#[async_trait]
impl Launcher for AdoptLauncher {
    async fn launch(&self, _vm: &Vm, _argv: &[String], _rt: &RuntimePaths) -> Result<Spawned, SupervisorError> {
        Ok(Spawned {
            pid: 1234,
            qmp: Box::new(MockQmp { shutdown_on_powerdown: true, powered_down: Arc::new(AtomicBool::new(false)) }),
        })
    }
    async fn reconnect(&self, _rt: &RuntimePaths) -> Result<Box<dyn QmpControl>, SupervisorError> {
        Ok(Box::new(MockQmp { shutdown_on_powerdown: true, powered_down: Arc::new(AtomicBool::new(false)) }))
    }
    fn force_kill(&self, _pid: i32) {}
    fn cleanup(&self, _rt: &RuntimePaths) {
        self.cleaned.fetch_add(1, Ordering::SeqCst);
    }
    fn is_alive(&self, _pid: i32) -> bool {
        self.alive
    }
}

fn seed_state(dir: &std::path::Path) -> std::io::Result<()> {
    let rt = phermesd::state::VmRuntime {
        id: "linux".to_string(),
        flavor: Flavor::Linux,
        pid: 1234,
        qmp: dir.join("run/linux/qmp.sock"),
        serial: Some(dir.join("run/linux/serial.sock")),
        vnc: Some(dir.join("run/linux/vnc.sock")),
        started_at: 1,
    };
    let state = phermesd::state::State { active: Some(rt) };
    state.save(&dir.join("state.json")).map_err(|e| std::io::Error::other(e.to_string()))
}

#[tokio::test]
async fn readopt_resumes_a_live_vm_without_relaunch() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("run/linux")).unwrap();
    seed_state(dir.path()).unwrap();
    let cleaned = Arc::new(AtomicI32::new(0));
    let mut sup = Supervisor::new(
        vec![vm("linux"), vm("windows")],
        dir.path().join("run"),
        dir.path().join("state.json"),
        Duration::from_millis(100),
        Box::new(AdoptLauncher { alive: true, cleaned: cleaned.clone() }),
    );
    sup.readopt().await.unwrap();
    let st = sup.status(None).unwrap();
    assert_eq!(st.id, "linux");
    assert_eq!(st.state, VmState::Running);
    assert_eq!(cleaned.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn readopt_clears_a_dead_vm_record() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("run/linux")).unwrap();
    seed_state(dir.path()).unwrap();
    let cleaned = Arc::new(AtomicI32::new(0));
    let mut sup = Supervisor::new(
        vec![vm("linux")],
        dir.path().join("run"),
        dir.path().join("state.json"),
        Duration::from_millis(100),
        Box::new(AdoptLauncher { alive: false, cleaned: cleaned.clone() }),
    );
    sup.readopt().await.unwrap();
    assert!(matches!(sup.status(None), Err(SupervisorError::NoActive)));
    assert_eq!(cleaned.load(Ordering::SeqCst), 1);
    let reloaded = phermesd::state::State::load(&dir.path().join("state.json")).unwrap();
    assert!(reloaded.active.is_none());
}

struct LivenessLauncher {
    alive: Arc<AtomicBool>,
}

#[async_trait]
impl Launcher for LivenessLauncher {
    async fn launch(&self, _vm: &Vm, _argv: &[String], _rt: &RuntimePaths) -> Result<Spawned, SupervisorError> {
        Ok(Spawned {
            pid: 5000,
            qmp: Box::new(MockQmp { shutdown_on_powerdown: true, powered_down: Arc::new(AtomicBool::new(false)) }),
        })
    }
    async fn reconnect(&self, _rt: &RuntimePaths) -> Result<Box<dyn QmpControl>, SupervisorError> {
        Ok(Box::new(MockQmp { shutdown_on_powerdown: true, powered_down: Arc::new(AtomicBool::new(false)) }))
    }
    fn force_kill(&self, _pid: i32) {}
    fn cleanup(&self, _rt: &RuntimePaths) {}
    fn is_alive(&self, _pid: i32) -> bool {
        self.alive.load(Ordering::SeqCst)
    }
}

#[tokio::test]
async fn status_reports_failed_when_active_pid_dies() {
    let dir = tempfile::tempdir().unwrap();
    let alive = Arc::new(AtomicBool::new(true));
    let mut sup = Supervisor::new(
        vec![vm("linux")],
        dir.path().join("run"),
        dir.path().join("state.json"),
        Duration::from_millis(100),
        Box::new(LivenessLauncher { alive: alive.clone() }),
    );
    let info = sup.activate("linux").await.unwrap();
    assert_eq!(info.state, VmState::Running);
    alive.store(false, Ordering::SeqCst);
    assert_eq!(sup.status(None).unwrap().state, VmState::Failed);
}

#[tokio::test]
async fn switch_auto_checkpoints_outgoing_vm() {
    let dir = tempfile::tempdir().unwrap();
    let lvm = MockLvm::default();
    // outgoing linux=vmid 102 must be a managed disk + a pool row for the guard
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["@phermesd"], "", None));
    lvm.lvs.lock().unwrap().push(lv("data", &[], "", Some(10.0)));
    let lvm_calls = lvm.calls.clone();
    let storage = std::sync::Arc::new(tokio::sync::Mutex::new(phermesd::storage::Storage::new(
        cfg(),
        Box::new(lvm),
        Box::new(MockBtrfs::default()),
        Box::new(MockConnector::default()),
    )));

    let (launcher, _killed, _cleaned) = MockLauncher::new(true);
    let mut sup = Supervisor::new(
        vec![vm("linux"), vm("windows")],
        dir.path().join("run"),
        dir.path().join("state.json"),
        Duration::from_millis(100),
        Box::new(launcher),
    );
    sup.set_storage(storage.clone());

    sup.activate("linux").await.unwrap();
    let info = sup.activate("windows").await.unwrap();
    assert_eq!(info.state, VmState::Running);

    // an auto checkpoint LV snapshot for vm-102 was taken before the switch
    let calls = lvm_calls.lock().unwrap();
    assert!(
        calls.iter().any(|c| c.starts_with("snap vm-102-disk-0-snap-auto-")),
        "expected an auto snapshot of the outgoing VM, calls = {calls:?}"
    );
}

#[tokio::test]
async fn switch_continues_when_auto_checkpoint_fails() {
    let dir = tempfile::tempdir().unwrap();
    let lvm = MockLvm::default();
    // pool at 95% -> auto checkpoint refused (PoolFull); switch must still succeed
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["@phermesd"], "", None));
    lvm.lvs.lock().unwrap().push(lv("data", &[], "", Some(95.0)));
    let storage = std::sync::Arc::new(tokio::sync::Mutex::new(phermesd::storage::Storage::new(
        cfg(),
        Box::new(lvm),
        Box::new(MockBtrfs::default()),
        Box::new(MockConnector::default()),
    )));

    let (launcher, _killed, _cleaned) = MockLauncher::new(true);
    let mut sup = Supervisor::new(
        vec![vm("linux"), vm("windows")],
        dir.path().join("run"),
        dir.path().join("state.json"),
        Duration::from_millis(100),
        Box::new(launcher),
    );
    sup.set_storage(storage.clone());

    sup.activate("linux").await.unwrap();
    let info = sup.activate("windows").await.unwrap();
    assert_eq!(info.state, VmState::Running); // warn-and-continue
}
