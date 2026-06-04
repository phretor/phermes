//! phermesctl: a thin UDS client for phermesd.

use clap::{Parser, Subcommand};
use phermesd::proto::{encode_line, Request, Response};
use std::io::Write;
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

#[derive(Debug, Parser)]
#[command(name = "phermesctl", about = "Control the PHermes VM orchestrator")]
struct Args {
    #[arg(long, default_value = "/run/phermesd/control.sock")]
    socket: PathBuf,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// List all defined VMs and their states.
    List,
    /// Show detail for a VM (or the active one).
    Status { id: Option<String> },
    /// Make a VM active (stops the current one first).
    Activate { id: String },
    /// Gracefully stop the active VM (or the named one).
    Stop { id: Option<String> },
    /// Re-scan the definitions directory.
    Reload,
    /// Provision a VM disk (optionally importing a local image).
    Provision {
        vmid: u32,
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        size: Option<u32>,
        #[arg(long)]
        force: bool,
    },
    /// Delete a VM disk and its snapshots.
    Delete { vmid: u32 },
    /// Take a manual checkpoint (disk + overlay).
    Snapshot { vmid: u32 },
    /// Roll a stopped VM back to a checkpoint.
    Rollback { vmid: u32, checkpoint: String },
    /// List checkpoints for a VM.
    Snapshots { vmid: u32 },
}

impl From<Cmd> for Request {
    fn from(c: Cmd) -> Self {
        match c {
            Cmd::List => Request::List,
            Cmd::Status { id } => Request::Status { id },
            Cmd::Activate { id } => Request::Activate { id },
            Cmd::Stop { id } => Request::Stop { id },
            Cmd::Reload => Request::Reload,
            Cmd::Provision { vmid, from, size, force } => Request::Provision { vmid, from, size, force },
            Cmd::Delete { vmid } => Request::Delete { vmid },
            Cmd::Snapshot { vmid } => Request::Snapshot { vmid },
            Cmd::Rollback { vmid, checkpoint } => Request::Rollback { vmid, checkpoint },
            Cmd::Snapshots { vmid } => Request::Snapshots { vmid: Some(vmid) },
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let req: Request = args.cmd.into();

    let stream = UnixStream::connect(&args.socket).await?;
    let (read, mut write) = stream.into_split();
    write.write_all(encode_line(&req)?.as_bytes()).await?;

    let mut lines = BufReader::new(read).lines();
    let line = lines
        .next_line()
        .await?
        .ok_or_else(|| anyhow::anyhow!("phermesd closed the connection without replying"))?;
    let resp: Response = serde_json::from_str(&line)?;

    let mut out = std::io::stdout();
    let pretty = serde_json::to_string_pretty(&resp)?;
    writeln!(out, "{pretty}")?;
    if resp.ok {
        Ok(())
    } else {
        Err(anyhow::anyhow!("command failed"))
    }
}
