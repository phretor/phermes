//! phermesd daemon: load defs, re-adopt, serve the control socket.

use clap::Parser;
use phermesd::config::load_dir;
use phermesd::control::serve;
use phermesd::launcher::QemuLauncher;
use phermesd::supervisor::Supervisor;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[derive(Debug, Parser)]
#[command(name = "phermesd", about = "PHermes VM orchestrator daemon")]
struct Args {
    #[arg(long, default_value = "/etc/phermes/vms")]
    vms_dir: PathBuf,
    #[arg(long, default_value = "/run/phermesd")]
    run_dir: PathBuf,
    #[arg(long, default_value = "/run/phermesd/control.sock")]
    socket: PathBuf,
    #[arg(long, default_value_t = 30)]
    stop_timeout_secs: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let vms = load_dir(&args.vms_dir)?;
    tracing::info!(count = vms.len(), "loaded VM definitions");

    let mut supervisor = Supervisor::new(
        vms,
        args.run_dir.clone(),
        args.run_dir.join("state.json"),
        Duration::from_secs(args.stop_timeout_secs),
        Box::new(QemuLauncher::default()),
    );
    supervisor.readopt().await?;

    let sup = Arc::new(Mutex::new(supervisor));
    let storage = Arc::new(Mutex::new(
        phermesd::storage::Storage::new(
            phermesd::storage::StorageConfig::default(),
            Box::new(phermesd::lvm::RealLvm),
            Box::new(phermesd::btrfs::RealBtrfs),
            Box::new(phermesd::qga::RealQgaConnector),
        ),
    ));
    tracing::info!(socket = %args.socket.display(), "serving control socket");
    serve(&args.socket, args.vms_dir, sup, storage).await?;
    Ok(())
}
