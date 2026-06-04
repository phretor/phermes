//! Storage orchestrator: provision/import/checkpoint/rollback/prune over the
//! [`LvmOps`] + [`BtrfsOps`] + [`QgaConnector`] seams. The system (lvs, the @snapshots dir) is
//! the source of truth.

use crate::btrfs::{BtrfsError, BtrfsOps};
use crate::lvm::{Lv, LvmError, LvmOps};
use crate::qga::{QgaConnector, QgaControl, QgaError};
use chrono::Utc;
use std::path::PathBuf;
use tokio::process::Command;

pub const OWNER_TAG: &str = "@phermesd";
pub const SNAP_TAG: &str = "@phermesd-snap";

#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub vg: String,
    pub pool: String,
    pub overlay: PathBuf,
    pub snapshots_dir: PathBuf,
    pub retention: usize,
    pub pool_threshold: f64,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            vg: "pve".into(),
            pool: "data".into(),
            overlay: "/var/lib/phermes/overlay".into(),
            snapshots_dir: "/var/lib/phermes/overlay/@snapshots".into(),
            retention: 5,
            pool_threshold: 90.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapKind {
    Auto,
    Manual,
}

impl SnapKind {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            SnapKind::Auto => "auto",
            SnapKind::Manual => "manual",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checkpoint {
    pub vmid: u32,
    pub utc: String,
    pub kind: SnapKind,
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("lvm: {0}")]
    Lvm(#[from] LvmError),
    #[error("btrfs: {0}")]
    Btrfs(#[from] BtrfsError),
    #[error("qga: {0}")]
    Qga(#[from] QgaError),
    #[error("thin pool {pool} is {percent:.1}% full (threshold {threshold:.0}%)")]
    PoolFull { pool: String, percent: f64, threshold: f64 },
    #[error("source {src} virtual size {src_gb}G exceeds volume {vol_gb}G")]
    SourceTooLarge { src: String, src_gb: u64, vol_gb: u32 },
    #[error("volume {0} is not managed by phermesd")]
    NotManaged(String),
    #[error("VM {0} is active; stop it first")]
    VmActive(u32),
    #[error("{0} not found")]
    NotFound(String),
    #[error("running {cmd}: {source}")]
    Tool {
        cmd: String,
        #[source]
        source: std::io::Error,
    },
    #[error("{cmd} failed ({code}): {stderr}")]
    ToolFailed { cmd: String, code: i32, stderr: String },
}

pub struct Storage {
    cfg: StorageConfig,
    lvm: Box<dyn LvmOps>,
    btrfs: Box<dyn BtrfsOps>,
    qga: Box<dyn QgaConnector>,
}

#[must_use]
pub fn disk_name(vmid: u32) -> String {
    format!("vm-{vmid}-disk-0")
}

#[must_use]
pub fn disk_device(vg: &str, vmid: u32) -> String {
    format!("/dev/{vg}/{}", disk_name(vmid))
}

fn managed_disk(lvs: &[Lv], vmid: u32) -> Option<&Lv> {
    let name = disk_name(vmid);
    lvs.iter().find(|l| l.lv_name == name && l.tags.iter().any(|t| t == OWNER_TAG))
}

/// Parse `<disk>-snap-<kind>-<utc>` (given the `<disk>-snap-` prefix) into (kind, utc).
fn parse_snap(lv_name: &str, prefix: &str) -> Option<(SnapKind, String)> {
    let rest = lv_name.strip_prefix(prefix)?;
    let (kind_s, utc) = rest.split_once('-')?;
    let kind = match kind_s {
        "auto" => SnapKind::Auto,
        "manual" => SnapKind::Manual,
        _ => return None,
    };
    Some((kind, utc.to_string()))
}

impl Storage {
    #[must_use]
    pub fn new(
        cfg: StorageConfig,
        lvm: Box<dyn LvmOps>,
        btrfs: Box<dyn BtrfsOps>,
        qga: Box<dyn QgaConnector>,
    ) -> Self {
        Self { cfg, lvm, btrfs, qga }
    }

    /// List the volume group's logical volumes.
    ///
    /// # Errors
    /// Propagates LVM errors.
    pub async fn list_lvs(&self) -> Result<Vec<Lv>, StorageError> {
        Ok(self.lvm.list(&self.cfg.vg).await?)
    }

    /// Provision a thin VM disk and optionally import a local source image.
    ///
    /// # Errors
    /// Propagates LVM errors; `NotManaged` if it exists without `--force`;
    /// `SourceTooLarge` if the image doesn't fit; tool errors on import.
    pub async fn provision(
        &self,
        vmid: u32,
        size_gb: u32,
        source: Option<&std::path::Path>,
        force: bool,
    ) -> Result<(), StorageError> {
        let lvs = self.lvm.list(&self.cfg.vg).await?;
        let name = disk_name(vmid);
        let exists = lvs.iter().any(|l| l.lv_name == name);
        if exists {
            if !force {
                return Err(StorageError::NotManaged(format!(
                    "{name} already exists (use --force to replace)"
                )));
            }
            self.delete(vmid, &lvs).await?;
        }
        self.lvm.create_thin(&self.cfg.vg, &self.cfg.pool, &name, size_gb).await?;
        let device = disk_device(&self.cfg.vg, vmid);
        self.lvm.add_tag(&device, OWNER_TAG).await?;
        if let Some(src) = source {
            if let Err(e) = self.import(src, &device, size_gb).await {
                let _ = self.lvm.remove(&device).await;
                return Err(e);
            }
        }
        Ok(())
    }

    async fn import(
        &self,
        src: &std::path::Path,
        device: &str,
        vol_gb: u32,
    ) -> Result<(), StorageError> {
        let src_gb = qemu_img_virtual_gb(src).await?;
        if src_gb > u64::from(vol_gb) {
            return Err(StorageError::SourceTooLarge {
                src: src.display().to_string(),
                src_gb,
                vol_gb,
            });
        }
        run_tool(&[
            "qemu-img".into(),
            "convert".into(),
            "-O".into(),
            "raw".into(),
            src.display().to_string(),
            device.to_string(),
        ])
        .await
    }

    /// Delete a managed VM disk and its snapshots. Caller passes the current `lvs`.
    ///
    /// # Errors
    /// `NotManaged` if the disk lacks the owner tag; LVM errors on removal.
    pub async fn delete(&self, vmid: u32, lvs: &[Lv]) -> Result<(), StorageError> {
        let name = disk_name(vmid);
        if managed_disk(lvs, vmid).is_none() {
            return Err(StorageError::NotManaged(name));
        }
        for snap in lvs.iter().filter(|l| l.origin == name) {
            self.lvm
                .remove(&format!("/dev/{}/{}", self.cfg.vg, snap.lv_name))
                .await?;
        }
        self.lvm.remove(&disk_device(&self.cfg.vg, vmid)).await?;
        Ok(())
    }

    fn utc_stamp() -> String {
        Utc::now().format("%Y%m%dT%H%M%SZ").to_string()
    }

    fn overlay_snap_path(&self, kind: SnapKind, utc: &str) -> std::path::PathBuf {
        self.cfg.snapshots_dir.join(format!("overlay-{}-{utc}", kind.as_str()))
    }

    fn pool_data_percent(lvs: &[Lv], pool: &str) -> f64 {
        lvs.iter().find(|l| l.lv_name == pool).and_then(|l| l.data_percent).unwrap_or(0.0)
    }

    /// Take a checkpoint (VM disk + overlay). If `qga_sock` is Some the VM is active and is
    /// quiesced best-effort; None means a stopped VM (cold, consistent).
    ///
    /// # Errors
    /// `NotManaged` if the disk isn't ours; `PoolFull` if over threshold (auto only);
    /// LVM/Btrfs errors. Partial snapshots are rolled back and the guest is always thawed.
    pub async fn checkpoint(
        &self,
        vmid: u32,
        kind: SnapKind,
        qga_sock: Option<std::path::PathBuf>,
    ) -> Result<Checkpoint, StorageError> {
        let lvs = self.lvm.list(&self.cfg.vg).await?;
        if managed_disk(&lvs, vmid).is_none() {
            return Err(StorageError::NotManaged(disk_name(vmid)));
        }
        if kind == SnapKind::Auto {
            let pct = Self::pool_data_percent(&lvs, &self.cfg.pool);
            if pct > self.cfg.pool_threshold {
                return Err(StorageError::PoolFull {
                    pool: self.cfg.pool.clone(),
                    percent: pct,
                    threshold: self.cfg.pool_threshold,
                });
            }
        }

        let utc = Self::utc_stamp();
        let snap_name = format!("{}-snap-{}-{utc}", disk_name(vmid), kind.as_str());
        let overlay_dst = self.overlay_snap_path(kind, &utc);

        let frozen = self.try_freeze(qga_sock.as_deref()).await;

        let result = async {
            self.lvm.snapshot(&self.cfg.vg, &disk_name(vmid), &snap_name).await?;
            if let Err(e) = self.btrfs.snapshot(&self.cfg.overlay, &overlay_dst).await {
                let _ = self.lvm.remove(&format!("/dev/{}/{snap_name}", self.cfg.vg)).await;
                return Err(StorageError::from(e));
            }
            Ok::<(), StorageError>(())
        }
        .await;

        if let Some(qga) = frozen {
            let _ = qga.thaw().await;
        }
        result?;

        if kind == SnapKind::Auto {
            self.prune_auto(vmid).await?;
        }
        Ok(Checkpoint { vmid, utc, kind })
    }

    /// Connect QGA and freeze; returns the live handle if freeze succeeded (so the caller
    /// can thaw), or None if no agent / freeze failed (crash-consistent).
    async fn try_freeze(&self, qga_sock: Option<&std::path::Path>) -> Option<Box<dyn QgaControl>> {
        let sock = qga_sock?;
        let qga = self.qga.connect(sock).await.ok()?;
        if qga.ping().await.is_err() {
            return None;
        }
        match qga.freeze().await {
            Ok(_) => Some(qga),
            Err(_) => None,
        }
    }

    /// List checkpoints for a VM (newest first).
    ///
    /// # Errors
    /// Propagates LVM list errors.
    pub async fn checkpoints(&self, vmid: u32) -> Result<Vec<Checkpoint>, StorageError> {
        let lvs = self.lvm.list(&self.cfg.vg).await?;
        let prefix = format!("{}-snap-", disk_name(vmid));
        let mut cps: Vec<Checkpoint> = lvs
            .iter()
            .filter_map(|l| {
                parse_snap(&l.lv_name, &prefix).map(|(kind, utc)| Checkpoint { vmid, utc, kind })
            })
            .collect();
        cps.sort_by(|a, b| b.utc.cmp(&a.utc));
        Ok(cps)
    }

    async fn prune_auto(&self, vmid: u32) -> Result<(), StorageError> {
        let mut autos: Vec<Checkpoint> = self
            .checkpoints(vmid)
            .await?
            .into_iter()
            .filter(|c| c.kind == SnapKind::Auto)
            .collect();
        let keep = self.cfg.retention.min(autos.len());
        for cp in autos.split_off(keep) {
            self.remove_checkpoint(vmid, &cp).await?;
        }
        Ok(())
    }

    async fn remove_checkpoint(&self, vmid: u32, cp: &Checkpoint) -> Result<(), StorageError> {
        let snap = format!("{}-snap-{}-{}", disk_name(vmid), cp.kind.as_str(), cp.utc);
        self.lvm.remove(&format!("/dev/{}/{snap}", self.cfg.vg)).await?;
        let overlay = self.overlay_snap_path(cp.kind, &cp.utc);
        self.btrfs.delete(&overlay).await?;
        Ok(())
    }

    /// Roll back a stopped VM's disk + overlay to a checkpoint.
    ///
    /// # Errors
    /// `NotFound` if the checkpoint is absent; LVM/Btrfs errors. The control layer ensures
    /// the VM is stopped before calling this.
    pub async fn rollback(&self, vmid: u32, utc: &str) -> Result<(), StorageError> {
        let cps = self.checkpoints(vmid).await?;
        let cp = cps
            .iter()
            .find(|c| c.utc == utc)
            .ok_or_else(|| StorageError::NotFound(format!("checkpoint {utc} for vm {vmid}")))?;
        let snap = format!("{}-snap-{}-{}", disk_name(vmid), cp.kind.as_str(), cp.utc);
        self.lvm.merge(&self.cfg.vg, &snap).await?;
        let ro = self.overlay_snap_path(cp.kind, &cp.utc);
        self.btrfs.delete(&self.cfg.overlay).await?;
        self.btrfs.restore(&ro, &self.cfg.overlay).await?;
        Ok(())
    }
}

#[derive(serde::Deserialize)]
struct QemuImgInfo {
    #[serde(rename = "virtual-size")]
    virtual_size: u64,
}

async fn qemu_img_virtual_gb(src: &std::path::Path) -> Result<u64, StorageError> {
    let out = run_tool_output(&[
        "qemu-img".into(),
        "info".into(),
        "--output=json".into(),
        src.display().to_string(),
    ])
    .await?;
    let info: QemuImgInfo =
        serde_json::from_str(&out).map_err(|e| StorageError::ToolFailed {
            cmd: "qemu-img info".into(),
            code: -1,
            stderr: e.to_string(),
        })?;
    Ok(info.virtual_size.div_ceil(1_024 * 1_024 * 1_024))
}

async fn run_tool(argv: &[String]) -> Result<(), StorageError> {
    run_tool_output(argv).await.map(|_| ())
}

async fn run_tool_output(argv: &[String]) -> Result<String, StorageError> {
    let cmd = argv.join(" ");
    let Some((head, tail)) = argv.split_first() else {
        unreachable!("tool argv is always non-empty")
    };
    let output = Command::new(head)
        .args(tail)
        .output()
        .await
        .map_err(|source| StorageError::Tool { cmd: cmd.clone(), source })?;
    if !output.status.success() {
        return Err(StorageError::ToolFailed {
            cmd,
            code: output.status.code().unwrap_or(-1),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
