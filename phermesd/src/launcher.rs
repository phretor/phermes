//! Real Launcher: spawns qemu-system-x86_64 in its own session (survives phermesd),
//! then connects QMP. The supervisor's only window onto the OS.

use crate::config::Vm;
use crate::qemu::RuntimePaths;
use crate::qmp::{connect_with_retry, QmpControl};
use crate::supervisor::{Launcher, Spawned, SupervisorError};
use async_trait::async_trait;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::time::Duration;

pub struct QemuLauncher {
    pub binary: String,
}

impl Default for QemuLauncher {
    fn default() -> Self {
        Self { binary: "qemu-system-x86_64".to_string() }
    }
}

impl QemuLauncher {
    fn prepare_runtime(vm: &Vm, rt: &RuntimePaths) -> Result<(), SupervisorError> {
        let dir = rt.qmp.parent().unwrap_or_else(|| std::path::Path::new("/run/phermesd"));
        std::fs::create_dir_all(dir).map_err(|source| SupervisorError::Launch {
            id: vm.id.clone(),
            source,
        })?;
        std::fs::copy(&vm.def.firmware.ovmf_vars_template, &rt.vars).map_err(|source| {
            SupervisorError::Launch { id: vm.id.clone(), source }
        })?;
        Ok(())
    }
}

#[async_trait]
impl Launcher for QemuLauncher {
    async fn launch(&self, vm: &Vm, argv: &[String], rt: &RuntimePaths) -> Result<Spawned, SupervisorError> {
        Self::prepare_runtime(vm, rt)?;
        let mut cmd = Command::new(&self.binary);
        cmd.args(argv);
        // New session => QEMU is not killed when phermesd exits. No PR_SET_PDEATHSIG.
        // Safety: setsid only detaches the child's session; async-signal-safe, no allocation.
        unsafe {
            cmd.pre_exec(|| nix::unistd::setsid().map(|_| ()).map_err(std::io::Error::from));
        }
        let child = cmd.spawn().map_err(|source| SupervisorError::Launch {
            id: vm.id.clone(),
            source,
        })?;
        let pid = i32::try_from(child.id()).unwrap_or(-1);
        // QEMU is in its own session and must outlive phermesd. std::process::Child does NOT
        // kill on drop, so dropping the handle here leaves QEMU running; we track it by pid + QMP.
        drop(child);
        let qmp = connect_with_retry(&rt.qmp, 50, Duration::from_millis(100)).await?;
        Ok(Spawned { pid, qmp: Box::new(qmp) })
    }

    async fn reconnect(&self, rt: &RuntimePaths) -> Result<Box<dyn QmpControl>, SupervisorError> {
        let qmp = connect_with_retry(&rt.qmp, 5, Duration::from_millis(100)).await?;
        Ok(Box::new(qmp))
    }

    fn force_kill(&self, pid: i32) {
        let _ = kill(Pid::from_raw(pid), Signal::SIGKILL);
    }

    fn cleanup(&self, rt: &RuntimePaths) {
        if let Some(dir) = rt.qmp.parent() {
            let _ = std::fs::remove_dir_all(dir);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Console, Disk, DiskInterface, Firmware, Flavor, Net, NetModel, Resources, VmDef};

    #[test]
    fn prepare_runtime_creates_dir_and_copies_nvram() {
        let dir = tempfile::tempdir().unwrap();
        let template = dir.path().join("OVMF_VARS.fd");
        std::fs::write(&template, b"vars").unwrap();
        let vm = Vm {
            id: "linux".to_string(),
            def: VmDef {
                flavor: Flavor::Linux,
                resources: Resources { memory_mib: 512, vcpus: 1, cpu: "host".into() },
                firmware: Firmware { ovmf_code: "/x".into(), ovmf_vars_template: template },
                disk: vec![Disk { path: "/c".into(), format: "raw".into(), interface: DiskInterface::VirtioScsi }],
                net: vec![Net { bridge: "vmbr0".into(), model: NetModel::VirtioNet, mac: None }],
                console: Console::default(),
            },
        };
        let rtdir = dir.path().join("run/linux");
        let rt = RuntimePaths {
            vars: rtdir.join("OVMF_VARS.fd"),
            qmp: rtdir.join("qmp.sock"),
            serial: rtdir.join("serial.sock"),
            vnc: rtdir.join("vnc.sock"),
            pidfile: rtdir.join("vm.pid"),
        };
        QemuLauncher::prepare_runtime(&vm, &rt).unwrap();
        assert!(rt.vars.exists());
        assert_eq!(std::fs::read(&rt.vars).unwrap(), b"vars");
    }
}
