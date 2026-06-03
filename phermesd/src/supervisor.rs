//! VM lifecycle state machine. Generic over a Launcher so it is tested without QEMU.

use crate::config::Vm;
use crate::proto::{VmInfo, VmState};
use crate::qemu::{build_argv, QemuError, RuntimePaths};
use crate::qmp::{QmpControl, QmpError};
use crate::state::{pid_alive, State, VmRuntime};
use async_trait::async_trait;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum SupervisorError {
    #[error("unknown VM id: {0}")]
    UnknownId(String),
    #[error("no active VM")]
    NoActive,
    #[error("building argv: {0}")]
    Argv(#[from] QemuError),
    #[error("launching {id}: {source}")]
    Launch {
        id: String,
        #[source]
        source: std::io::Error,
    },
    #[error("QMP: {0}")]
    Qmp(#[from] QmpError),
    #[error("state: {0}")]
    State(#[from] crate::state::StateError),
}

/// Result of a successful spawn: the pid plus a live QMP control channel.
pub struct Spawned {
    pub pid: i32,
    pub qmp: Box<dyn QmpControl>,
}

/// Abstracts the side-effecting parts of running a VM so the state machine is pure logic.
#[async_trait]
pub trait Launcher: Send + Sync {
    /// Prepare runtime dir, copy NVRAM, spawn QEMU in its own session, connect QMP.
    ///
    /// # Errors
    /// Returns a `SupervisorError` if the runtime dir, spawn, or QMP connect fails.
    async fn launch(&self, vm: &Vm, argv: &[String], rt: &RuntimePaths) -> Result<Spawned, SupervisorError>;
    /// Reconnect QMP to an already-running VM (re-adopt).
    ///
    /// # Errors
    /// Returns a `SupervisorError` if the QMP socket cannot be reconnected.
    async fn reconnect(&self, rt: &RuntimePaths) -> Result<Box<dyn QmpControl>, SupervisorError>;
    /// SIGKILL a pid (used when graceful stop times out).
    fn force_kill(&self, pid: i32);
    /// Remove a VM's runtime directory.
    fn cleanup(&self, rt: &RuntimePaths);
    /// Whether a pid is still alive (mockable for tests).
    fn is_alive(&self, pid: i32) -> bool {
        pid_alive(pid)
    }
}

struct Active {
    id: String,
    pid: i32,
    state: VmState,
    qmp: Box<dyn QmpControl>,
    rt: RuntimePaths,
}

pub struct Supervisor {
    vms: Vec<Vm>,
    run_root: PathBuf,
    state_path: PathBuf,
    stop_timeout: Duration,
    launcher: Box<dyn Launcher>,
    active: Option<Active>,
}

impl Supervisor {
    #[must_use]
    pub fn new(
        vms: Vec<Vm>,
        run_root: PathBuf,
        state_path: PathBuf,
        stop_timeout: Duration,
        launcher: Box<dyn Launcher>,
    ) -> Self {
        Self { vms, run_root, state_path, stop_timeout, launcher, active: None }
    }

    fn runtime_paths(&self, id: &str) -> RuntimePaths {
        let dir = self.run_root.join(id);
        RuntimePaths {
            vars: dir.join("OVMF_VARS.fd"),
            qmp: dir.join("qmp.sock"),
            serial: dir.join("serial.sock"),
            vnc: dir.join("vnc.sock"),
            pidfile: dir.join("vm.pid"),
        }
    }

    fn find(&self, id: &str) -> Result<&Vm, SupervisorError> {
        self.vms
            .iter()
            .find(|v| v.id == id)
            .ok_or_else(|| SupervisorError::UnknownId(id.to_string()))
    }

    fn info_for(&self, vm: &Vm) -> VmInfo {
        let active = self.active.as_ref().filter(|a| a.id == vm.id);
        let (state, pid, qmp, serial, vnc) = match active {
            Some(a) => {
                let rt = &a.rt;
                (
                    a.state,
                    Some(a.pid),
                    Some(rt.qmp.clone()),
                    vm.def.console.serial.then(|| rt.serial.clone()),
                    vm.def.console.vnc.then(|| rt.vnc.clone()),
                )
            }
            None => (VmState::Defined, None, None, None, None),
        };
        VmInfo { id: vm.id.clone(), flavor: vm.def.flavor, state, pid, qmp, serial, vnc }
    }

    #[must_use]
    pub fn list(&self) -> Vec<VmInfo> {
        self.vms.iter().map(|v| self.info_for(v)).collect()
    }

    /// # Errors
    /// `UnknownId` if `id` names no defined VM, `NoActive` if `id` is None and nothing is active.
    pub fn status(&self, id: Option<&str>) -> Result<VmInfo, SupervisorError> {
        let id = match id {
            Some(i) => i.to_string(),
            None => self.active.as_ref().ok_or(SupervisorError::NoActive)?.id.clone(),
        };
        let vm = self.find(&id)?;
        Ok(self.info_for(vm))
    }

    fn persist(&self) -> Result<(), SupervisorError> {
        let state = State {
            active: self.active.as_ref().map(|a| VmRuntime {
                id: a.id.clone(),
                flavor: self.find(&a.id).map_or(crate::config::Flavor::Linux, |v| v.def.flavor),
                pid: a.pid,
                qmp: a.rt.qmp.clone(),
                serial: Some(a.rt.serial.clone()),
                vnc: Some(a.rt.vnc.clone()),
                started_at: now_secs(),
            }),
        };
        state.save(&self.state_path)?;
        Ok(())
    }

    /// Make `id` the active VM. If another VM is running, stop it first (implicit switch).
    ///
    /// # Errors
    /// `UnknownId`, or propagates argv/launch/QMP/state errors.
    pub async fn activate(&mut self, id: &str) -> Result<VmInfo, SupervisorError> {
        let _ = self.find(id)?;
        if let Some(active) = &self.active {
            if active.id == id {
                let vm = self.find(id)?;
                return Ok(self.info_for(vm));
            }
            self.stop(None).await?;
        }
        let vm = self.find(id)?.clone();
        let rt = self.runtime_paths(id);
        let argv = build_argv(&vm, &rt)?;
        let spawned = self.launcher.launch(&vm, &argv, &rt).await?;
        let running = spawned.qmp.is_running().await?;
        let state = if running { VmState::Running } else { VmState::Starting };
        self.active = Some(Active { id: id.to_string(), pid: spawned.pid, state, qmp: spawned.qmp, rt });
        self.persist()?;
        let vm = self.find(id)?;
        Ok(self.info_for(vm))
    }

    /// Gracefully stop the active VM (or `id` if it is the active one).
    ///
    /// # Errors
    /// `NoActive` if nothing is active, `UnknownId` if `id` is not the active VM.
    pub async fn stop(&mut self, id: Option<&str>) -> Result<VmInfo, SupervisorError> {
        let mut active = self.active.take().ok_or(SupervisorError::NoActive)?;
        if let Some(want) = id {
            if want != active.id {
                let info_id = active.id.clone();
                self.active = Some(active);
                return Err(SupervisorError::UnknownId(format!(
                    "{want} is not the active VM ({info_id})"
                )));
            }
        }
        active.state = VmState::Stopping;
        let _ = active.qmp.powerdown().await;
        let graceful = tokio::time::timeout(self.stop_timeout, active.qmp.wait_shutdown()).await;
        if graceful.is_err() {
            self.launcher.force_kill(active.pid);
        }
        self.launcher.cleanup(&active.rt);
        let vm = self.find(&active.id)?.clone();
        self.persist()?;
        Ok(VmInfo {
            id: vm.id,
            flavor: vm.def.flavor,
            state: VmState::Stopped,
            pid: None,
            qmp: None,
            serial: None,
            vnc: None,
        })
    }

    /// Reload definitions (called by the `reload` command).
    pub fn reload(&mut self, vms: Vec<Vm>) -> Vec<VmInfo> {
        self.vms = vms;
        self.list()
    }

    /// On startup, re-adopt a VM recorded as active if its process is still alive.
    ///
    /// # Errors
    /// Propagates state load/save errors.
    pub async fn readopt(&mut self) -> Result<(), SupervisorError> {
        let state = State::load(&self.state_path)?;
        let Some(rec) = state.active else {
            return Ok(());
        };
        let rt = self.runtime_paths(&rec.id);
        if self.launcher.is_alive(rec.pid) {
            match self.launcher.reconnect(&rt).await {
                Ok(qmp) => {
                    self.active = Some(Active {
                        id: rec.id,
                        pid: rec.pid,
                        state: VmState::Running,
                        qmp,
                        rt,
                    });
                    return Ok(());
                }
                Err(_) => {
                    self.launcher.cleanup(&rt);
                }
            }
        } else {
            self.launcher.cleanup(&rt);
        }
        State::default().save(&self.state_path)?;
        Ok(())
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}
