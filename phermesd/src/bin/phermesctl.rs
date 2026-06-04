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
}

impl From<Cmd> for Request {
    fn from(c: Cmd) -> Self {
        match c {
            Cmd::List => Request::List,
            Cmd::Status { id } => Request::Status { id },
            Cmd::Activate { id } => Request::Activate { id },
            Cmd::Stop { id } => Request::Stop { id },
            Cmd::Reload => Request::Reload,
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
