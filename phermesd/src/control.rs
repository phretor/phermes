//! UDS control server: read a JSON request line, dispatch to the supervisor, write a response.

use crate::config::load_dir;
use crate::proto::{encode_line, CheckpointInfo, Request, Response};
use crate::storage::{Checkpoint, SnapKind, Storage, StorageError};
use crate::supervisor::{Supervisor, SupervisorError};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::Mutex;

fn default_disk_gb(vmid: u32) -> u32 {
    match vmid {
        100 => 120,
        101 => 100,
        _ => 40,
    }
}

fn to_checkpoint_info(cp: &Checkpoint) -> CheckpointInfo {
    CheckpointInfo {
        vmid: cp.vmid,
        utc: cp.utc.clone(),
        kind: cp.kind.as_str().to_string(),
    }
}

fn storage_error_response(e: &StorageError) -> Response {
    let kind = match e {
        StorageError::PoolFull { .. } => "pool_full",
        StorageError::SourceTooLarge { .. } => "source_too_large",
        StorageError::NotManaged(_) => "not_managed",
        StorageError::VmActive(_) => "vm_active",
        StorageError::NotFound(_) => "not_found",
        _ => "storage",
    };
    Response::err(kind, &e.to_string())
}

async fn is_active(sup: &Mutex<Supervisor>, vmid: u32) -> bool {
    sup.lock().await.active_vmid() == Some(vmid)
}

async fn active_qga_sock(sup: &Mutex<Supervisor>, vmid: u32) -> Option<std::path::PathBuf> {
    let s = sup.lock().await;
    if s.active_vmid() == Some(vmid) { s.active_qga_sock() } else { None }
}

fn error_response(err: &SupervisorError) -> Response {
    let kind = match err {
        SupervisorError::UnknownId(_) => "unknown_id",
        SupervisorError::NoActive => "no_active",
        SupervisorError::Argv(_) => "argv",
        SupervisorError::Launch { .. } => "launch",
        SupervisorError::Qmp(_) => "qmp",
        SupervisorError::State(_) => "state",
    };
    Response::err(kind, &err.to_string())
}

/// Apply a parsed request against the supervisor and produce a response.
pub async fn dispatch(
    sup: &Mutex<Supervisor>,
    storage: &Mutex<Storage>,
    vms_dir: &Path,
    req: Request,
) -> Response {
    match req {
        Request::List | Request::Status { .. } | Request::Activate { .. } | Request::Stop { .. } | Request::Reload => {
            let mut s = sup.lock().await;
            let result: Result<serde_json::Value, SupervisorError> = match req {
                Request::List => Ok(serde_json::json!(s.list())),
                Request::Status { id } => s.status(id.as_deref()).map(|i| serde_json::json!(i)),
                Request::Activate { id } => s.activate(&id).await.map(|i| serde_json::json!(i)),
                Request::Stop { id } => s.stop(id.as_deref()).await.map(|i| serde_json::json!(i)),
                Request::Reload => match load_dir(vms_dir) {
                    Ok(vms) => Ok(serde_json::json!(s.reload(vms))),
                    Err(e) => return Response::err("config", &e.to_string()),
                },
                _ => unreachable!(),
            };
            match result {
                Ok(value) => Response::ok(value),
                Err(e) => error_response(&e),
            }
        }
        Request::Provision { vmid, from, size, force } => {
            let size_gb = size.unwrap_or_else(|| default_disk_gb(vmid));
            let src = from.as_ref().map(std::path::PathBuf::from);
            let st = storage.lock().await;
            match st.provision(vmid, size_gb, src.as_deref(), force).await {
                Ok(()) => Response::ok(serde_json::json!({"vmid": vmid, "size": size_gb})),
                Err(e) => storage_error_response(&e),
            }
        }
        Request::Delete { vmid } => {
            if is_active(sup, vmid).await {
                return Response::err("vm_active", "stop the VM before deleting its disk");
            }
            let st = storage.lock().await;
            let lvs = match st.list_lvs().await {
                Ok(l) => l,
                Err(e) => return storage_error_response(&e),
            };
            match st.delete(vmid, &lvs).await {
                Ok(()) => Response::ok(serde_json::json!({"vmid": vmid})),
                Err(e) => storage_error_response(&e),
            }
        }
        Request::Snapshot { vmid } => {
            let qga_sock = active_qga_sock(sup, vmid).await;
            let st = storage.lock().await;
            match st.checkpoint(vmid, SnapKind::Manual, qga_sock).await {
                Ok(cp) => Response::ok(serde_json::json!(to_checkpoint_info(&cp))),
                Err(e) => storage_error_response(&e),
            }
        }
        Request::Rollback { vmid, checkpoint } => {
            if is_active(sup, vmid).await {
                return Response::err("vm_active", "stop the VM before rolling back");
            }
            let st = storage.lock().await;
            match st.rollback(vmid, &checkpoint).await {
                Ok(()) => Response::ok(serde_json::json!({"vmid": vmid, "checkpoint": checkpoint})),
                Err(e) => storage_error_response(&e),
            }
        }
        Request::Snapshots { vmid } => {
            let Some(id) = vmid else {
                return Response::err("bad_request", "snapshots requires a vmid");
            };
            let st = storage.lock().await;
            match st.checkpoints(id).await {
                Ok(cps) => Response::ok(serde_json::json!(cps.iter().map(to_checkpoint_info).collect::<Vec<_>>())),
                Err(e) => storage_error_response(&e),
            }
        }
    }
}

async fn handle_conn(
    stream: UnixStream,
    sup: Arc<Mutex<Supervisor>>,
    storage: Arc<Mutex<Storage>>,
    vms_dir: Arc<PathBuf>,
) {
    let (read, mut write) = stream.into_split();
    let mut lines = BufReader::new(read).lines();
    while let Ok(Some(line)) = lines.next_line().await {
        let resp = match serde_json::from_str::<Request>(&line) {
            Ok(req) => {
                tracing::debug!(?req, "dispatch");
                dispatch(&sup, &storage, &vms_dir, req).await
            }
            Err(e) => Response::err("bad_request", &e.to_string()),
        };
        let Ok(encoded) = encode_line(&resp) else {
            break;
        };
        if write.write_all(encoded.as_bytes()).await.is_err() {
            break;
        }
    }
}

/// Bind the control socket and serve until the listener errors.
///
/// # Errors
/// Returns an io error if the socket cannot be bound or accept fails.
pub async fn serve(
    socket_path: &Path,
    vms_dir: PathBuf,
    sup: Arc<Mutex<Supervisor>>,
    storage: Arc<Mutex<Storage>>,
) -> std::io::Result<()> {
    if socket_path.exists() {
        std::fs::remove_file(socket_path)?;
    }
    if let Some(parent) = socket_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let listener = UnixListener::bind(socket_path)?;
    let vms_dir = Arc::new(vms_dir);
    loop {
        let (stream, _) = match listener.accept().await {
            Ok(pair) => pair,
            Err(e) => {
                tracing::warn!(error = %e, "accept failed; continuing");
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                continue;
            }
        };
        let sup = sup.clone();
        let storage = storage.clone();
        let vms_dir = vms_dir.clone();
        tokio::spawn(handle_conn(stream, sup, storage, vms_dir));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::btrfs::RealBtrfs;
    use crate::config::{Console, Disk, DiskInterface, Firmware, Flavor, Net, NetModel, Resources, Vm, VmDef};
    use crate::lvm::RealLvm;
    use crate::proto::VmState;
    use crate::qemu::RuntimePaths;
    use crate::qga::RealQgaConnector;
    use crate::qmp::{QmpControl, QmpError};
    use crate::storage::StorageConfig;
    use crate::supervisor::{Launcher, Spawned};
    use async_trait::async_trait;
    use std::time::Duration;

    struct OkQmp;
    #[async_trait]
    impl QmpControl for OkQmp {
        async fn is_running(&self) -> Result<bool, QmpError> {
            Ok(true)
        }
        async fn powerdown(&self) -> Result<(), QmpError> {
            Ok(())
        }
        async fn wait_shutdown(&mut self) -> Result<(), QmpError> {
            Ok(())
        }
    }

    struct OkLauncher;
    #[async_trait]
    impl Launcher for OkLauncher {
        async fn launch(&self, _v: &Vm, _a: &[String], _r: &RuntimePaths) -> Result<Spawned, SupervisorError> {
            Ok(Spawned { pid: 42, qmp: Box::new(OkQmp) })
        }
        async fn reconnect(&self, _r: &RuntimePaths) -> Result<Box<dyn QmpControl>, SupervisorError> {
            Ok(Box::new(OkQmp))
        }
        fn force_kill(&self, _pid: i32) {}
        fn cleanup(&self, _r: &RuntimePaths) {}
        fn is_alive(&self, _pid: i32) -> bool {
            true
        }
    }

    fn vm(id: &str) -> Vm {
        Vm {
            id: id.to_string(),
            def: VmDef {
                flavor: Flavor::Linux,
                resources: Resources { memory_mib: 512, vcpus: 1, cpu: "host".into() },
                firmware: Firmware { ovmf_code: "/a".into(), ovmf_vars_template: "/b".into() },
                disk: vec![Disk { path: "/c".into(), format: "raw".into(), interface: DiskInterface::VirtioScsi }],
                net: vec![Net { bridge: "vmbr0".into(), model: NetModel::VirtioNet, mac: None }],
                console: Console::default(),
            },
        }
    }

    fn test_storage() -> Mutex<Storage> {
        Mutex::new(Storage::new(
            StorageConfig::default(),
            Box::new(RealLvm),
            Box::new(RealBtrfs),
            Box::new(RealQgaConnector),
        ))
    }

    #[tokio::test]
    async fn dispatch_activate_then_status() {
        let dir = tempfile::tempdir().unwrap();
        let sup = Mutex::new(Supervisor::new(
            vec![vm("linux")],
            dir.path().join("run"),
            dir.path().join("state.json"),
            Duration::from_millis(50),
            Box::new(OkLauncher),
        ));
        let vms_dir = dir.path().to_path_buf();

        let r = dispatch(&sup, &test_storage(), &vms_dir, Request::Activate { id: "linux".into() }).await;
        assert!(r.ok);

        let r = dispatch(&sup, &test_storage(), &vms_dir, Request::Status { id: None }).await;
        assert!(r.ok);
        let info: crate::proto::VmInfo = serde_json::from_value(r.data.unwrap()).unwrap();
        assert_eq!(info.state, VmState::Running);
    }

    #[tokio::test]
    async fn dispatch_unknown_id_is_error_envelope() {
        let dir = tempfile::tempdir().unwrap();
        let sup = Mutex::new(Supervisor::new(
            vec![vm("linux")],
            dir.path().join("run"),
            dir.path().join("state.json"),
            Duration::from_millis(50),
            Box::new(OkLauncher),
        ));
        let r = dispatch(&sup, &test_storage(), dir.path(), Request::Activate { id: "ghost".into() }).await;
        assert!(!r.ok);
        assert_eq!(r.error.unwrap().kind, "unknown_id");
    }

    #[tokio::test]
    async fn snapshots_without_vmid_is_bad_request() {
        let dir = tempfile::tempdir().unwrap();
        let sup = Mutex::new(Supervisor::new(
            vec![vm("linux")],
            dir.path().join("run"),
            dir.path().join("state.json"),
            Duration::from_millis(50),
            Box::new(OkLauncher),
        ));
        let r = dispatch(
            &sup,
            &test_storage(),
            dir.path(),
            Request::Snapshots { vmid: None },
        )
        .await;
        assert!(!r.ok);
        assert_eq!(r.error.unwrap().kind, "bad_request");
    }
}
