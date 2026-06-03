//! UDS control server: read a JSON request line, dispatch to the supervisor, write a response.

use crate::config::load_dir;
use crate::proto::{encode_line, Request, Response};
use crate::supervisor::{Supervisor, SupervisorError};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::Mutex;

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
pub async fn dispatch(sup: &Mutex<Supervisor>, vms_dir: &Path, req: Request) -> Response {
    let mut sup = sup.lock().await;
    let result: Result<serde_json::Value, SupervisorError> = match req {
        Request::List => Ok(serde_json::json!(sup.list())),
        Request::Status { id } => sup.status(id.as_deref()).map(|i| serde_json::json!(i)),
        Request::Activate { id } => sup.activate(&id).await.map(|i| serde_json::json!(i)),
        Request::Stop { id } => sup.stop(id.as_deref()).await.map(|i| serde_json::json!(i)),
        Request::Reload => match load_dir(vms_dir) {
            Ok(vms) => Ok(serde_json::json!(sup.reload(vms))),
            Err(e) => return Response::err("config", &e.to_string()),
        },
    };
    match result {
        Ok(value) => Response::ok(value),
        Err(e) => error_response(&e),
    }
}

async fn handle_conn(stream: UnixStream, sup: Arc<Mutex<Supervisor>>, vms_dir: Arc<PathBuf>) {
    let (read, mut write) = stream.into_split();
    let mut lines = BufReader::new(read).lines();
    while let Ok(Some(line)) = lines.next_line().await {
        let resp = match serde_json::from_str::<Request>(&line) {
            Ok(req) => {
                tracing::debug!(?req, "dispatch");
                dispatch(&sup, &vms_dir, req).await
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
        let vms_dir = vms_dir.clone();
        tokio::spawn(handle_conn(stream, sup, vms_dir));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Console, Disk, DiskInterface, Firmware, Flavor, Net, NetModel, Resources, Vm, VmDef};
    use crate::proto::VmState;
    use crate::qemu::RuntimePaths;
    use crate::qmp::{QmpControl, QmpError};
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

        let r = dispatch(&sup, &vms_dir, Request::Activate { id: "linux".into() }).await;
        assert!(r.ok);

        let r = dispatch(&sup, &vms_dir, Request::Status { id: None }).await;
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
        let r = dispatch(&sup, dir.path(), Request::Activate { id: "ghost".into() }).await;
        assert!(!r.ok);
        assert_eq!(r.error.unwrap().kind, "unknown_id");
    }
}
