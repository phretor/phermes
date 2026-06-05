# phermesd Storage & Snapshots Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** A runtime storage layer in phermesd that provisions LVM-thin VM-disk volumes, imports local images, and takes/rolls-back QGA-quiesced checkpoints (VM disk + Btrfs overlay together), exposed over the UDS control protocol and auto-triggered before a VM switch.

**Architecture:** Mirrors slice #1 — side effects behind mockable async trait seams (`LvmOps`, `BtrfsOps`, `QgaControl`/`QgaConnector`), pure argv builders unit-tested, orchestration logic (`storage.rs`) testable with mocks, real LVM/Btrfs/guest-agent exercised only in gated tests. The system is the source of truth (`lvs --reportformat json`, the `@snapshots` dir); no state file.

**Tech Stack:** Rust/tokio, qapi 0.15 (`qmp`+`qga`+`async-tokio-all`), serde_json (parse `lvs` JSON), tokio::process, async-trait, thiserror. System tools: `lvcreate`/`lvremove`/`lvconvert`/`lvchange`/`lvs`, `btrfs`, `qemu-img`.

**Spec:** `docs/superpowers/specs/2026-06-03-phermesd-storage-design.md`

**Prerequisite:** the phermesd crate from slice #1 must be present. Base this work on the slice-#1 branch (`design/phermesd-orchestrator`) or on `main` after PR #17 merges — not on a bare `main` that lacks `phermesd/`.

---

## File Structure

```
phermesd/
  Cargo.toml                # + qga feature
  src/
    lvm.rs        (new)     # LvmOps trait + RealLvm + pure argv builders + lvs JSON parse
    btrfs.rs      (new)     # BtrfsOps trait + RealBtrfs + pure path/argv builders
    qga.rs        (new)     # QgaControl trait + QapiQga + QgaConnector + RealQgaConnector
    storage.rs    (new)     # Storage orchestrator: provision/import/checkpoint/rollback/prune/pool-guard
    qemu.rs       (modify)  # RuntimePaths + argv: guest-agent virtio-serial channel + qga.sock
    supervisor.rs (modify)  # auto-checkpoint before switch
    proto.rs      (modify)  # storage Request variants + CheckpointInfo/VolumeInfo
    control.rs    (modify)  # dispatch storage verbs
    bin/phermesctl.rs (modify) # storage subcommands
    lib.rs        (modify)  # declare lvm/btrfs/qga/storage
  tests/
    qga_wire.rs            (new)  # real QapiQga vs a hand-rolled QGA server
    storage_lifecycle.rs  (new)  # orchestration via mock Lvm/Btrfs/Qga
    storage_integration.rs (new) # #[ignore] loop-device LVM-thin + Btrfs
```

---

### Task 1: Cargo `qga` feature + module scaffold

**Files:** Modify `phermesd/Cargo.toml`, `phermesd/src/lib.rs`; Create `phermesd/src/{lvm,btrfs,qga,storage}.rs`.

- [ ] **Step 1: Enable the qga feature**

Edit `phermesd/Cargo.toml` — change the qapi dependency line to add `qga`:

```toml
qapi = { version = "0.15", features = ["qmp", "qga", "async-tokio-all"] }
```

- [ ] **Step 2: Declare the new modules**

In `phermesd/src/lib.rs`, add (keep alphabetical with existing `pub mod` lines):

```rust
pub mod btrfs;
pub mod lvm;
pub mod qga;
pub mod storage;
```

- [ ] **Step 3: Create empty module files**

Create each with a single line `// implemented in a later task`:
`phermesd/src/lvm.rs`, `phermesd/src/btrfs.rs`, `phermesd/src/qga.rs`, `phermesd/src/storage.rs`.

- [ ] **Step 4: Build clean**

Run: `cd phermesd && cargo build && cargo clippy --all-targets --all-features -- -D warnings`
Expected: builds, no warnings.

- [ ] **Step 5: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add phermesd/Cargo.toml phermesd/Cargo.lock phermesd/src/lib.rs phermesd/src/lvm.rs phermesd/src/btrfs.rs phermesd/src/qga.rs phermesd/src/storage.rs
git commit -m "feat(phermesd): scaffold storage modules, enable qapi qga feature"
```

---

### Task 2: `lvm.rs` — pure argv builders + types

**Files:** Modify `phermesd/src/lvm.rs`.

- [ ] **Step 1: Write the tests**

Put at the bottom of `phermesd/src/lvm.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_thin_argv_is_correct() {
        let a = create_thin_argv("pve", "data", "vm-102-disk-0", 40);
        assert_eq!(
            a,
            vec!["lvcreate", "--thin", "--virtualsize", "40G", "pve/data", "-n", "vm-102-disk-0"]
        );
    }

    #[test]
    fn addtag_argv_targets_the_device_path() {
        assert_eq!(
            addtag_argv("/dev/pve/vm-102-disk-0", "@phermesd"),
            vec!["lvchange", "--addtag", "@phermesd", "/dev/pve/vm-102-disk-0"]
        );
    }

    #[test]
    fn snapshot_argv_is_a_thin_snapshot() {
        let a = snapshot_argv("pve", "vm-102-disk-0", "vm-102-disk-0-snap-auto-20260603T141500Z");
        assert_eq!(
            a,
            vec![
                "lvcreate", "--snapshot", "--name",
                "vm-102-disk-0-snap-auto-20260603T141500Z", "pve/vm-102-disk-0"
            ]
        );
    }

    #[test]
    fn merge_and_remove_argv() {
        assert_eq!(merge_argv("pve", "vm-102-disk-0-snap-manual-x"),
                   vec!["lvconvert", "--merge", "pve/vm-102-disk-0-snap-manual-x"]);
        assert_eq!(remove_argv("/dev/pve/vm-102-disk-0"),
                   vec!["lvremove", "-y", "/dev/pve/vm-102-disk-0"]);
    }

    #[test]
    fn lvs_json_argv_requests_machine_readable_report() {
        let a = lvs_json_argv("pve");
        assert!(a.contains(&"--reportformat".to_string()));
        assert!(a.contains(&"json".to_string()));
        assert!(a.iter().any(|s| s.contains("lv_name")));
        assert_eq!(a.last().unwrap(), "pve");
    }

    #[test]
    fn parse_lvs_extracts_volumes_tags_and_pool_percent() {
        let json = r#"{"report":[{"lv":[
          {"lv_name":"data","lv_tags":"","pool_lv":"","origin":"","data_percent":"42.50"},
          {"lv_name":"vm-102-disk-0","lv_tags":"@phermesd","pool_lv":"data","origin":"","data_percent":""},
          {"lv_name":"vm-102-disk-0-snap-auto-20260603T141500Z","lv_tags":"@phermesd-snap","pool_lv":"data","origin":"vm-102-disk-0","data_percent":""}
        ]}]}"#;
        let lvs = parse_lvs(json).unwrap();
        assert_eq!(lvs.len(), 3);
        let pool = lvs.iter().find(|l| l.lv_name == "data").unwrap();
        assert_eq!(pool.data_percent, Some(42.5));
        let snap = lvs.iter().find(|l| l.origin == "vm-102-disk-0").unwrap();
        assert!(snap.tags.iter().any(|t| t == "@phermesd-snap"));
    }
}
```

- [ ] **Step 2: Run, verify fail to compile**

Run: `cd phermesd && cargo test --lib lvm 2>&1 | head -20`
Expected: FAIL — items not found.

- [ ] **Step 3: Implement types, argv builders, parser (above the tests)**

```rust
//! LVM-thin operations: pure argv builders, an `lvs --reportformat json` parser,
//! and the `LvmOps` seam (real impl in this file, mocked in tests).

use serde::Deserialize;

/// One logical volume as reported by `lvs`.
#[derive(Debug, Clone, PartialEq)]
pub struct Lv {
    pub lv_name: String,
    pub tags: Vec<String>,
    pub pool_lv: String,
    pub origin: String,
    pub data_percent: Option<f64>,
}

#[must_use]
pub fn create_thin_argv(vg: &str, pool: &str, name: &str, size_gb: u32) -> Vec<String> {
    vec![
        "lvcreate".into(), "--thin".into(), "--virtualsize".into(), format!("{size_gb}G"),
        format!("{vg}/{pool}"), "-n".into(), name.into(),
    ]
}

#[must_use]
pub fn addtag_argv(device: &str, tag: &str) -> Vec<String> {
    vec!["lvchange".into(), "--addtag".into(), tag.into(), device.into()]
}

#[must_use]
pub fn snapshot_argv(vg: &str, origin: &str, snap_name: &str) -> Vec<String> {
    vec![
        "lvcreate".into(), "--snapshot".into(), "--name".into(), snap_name.into(),
        format!("{vg}/{origin}"),
    ]
}

#[must_use]
pub fn merge_argv(vg: &str, snap_name: &str) -> Vec<String> {
    vec!["lvconvert".into(), "--merge".into(), format!("{vg}/{snap_name}")]
}

#[must_use]
pub fn remove_argv(device: &str) -> Vec<String> {
    vec!["lvremove".into(), "-y".into(), device.into()]
}

#[must_use]
pub fn lvs_json_argv(vg: &str) -> Vec<String> {
    vec![
        "lvs".into(), "--reportformat".into(), "json".into(),
        "-o".into(), "lv_name,lv_tags,pool_lv,origin,data_percent".into(),
        vg.into(),
    ]
}

#[derive(Deserialize)]
struct LvsReport {
    report: Vec<LvsGroup>,
}
#[derive(Deserialize)]
struct LvsGroup {
    lv: Vec<LvsRow>,
}
#[derive(Deserialize)]
struct LvsRow {
    lv_name: String,
    lv_tags: String,
    pool_lv: String,
    origin: String,
    data_percent: String,
}

/// Parse `lvs --reportformat json` output into `Lv` rows.
///
/// # Errors
/// Returns `serde_json::Error` if the report is not the expected shape.
pub fn parse_lvs(json: &str) -> Result<Vec<Lv>, serde_json::Error> {
    let parsed: LvsReport = serde_json::from_str(json)?;
    let mut out = Vec::new();
    for group in parsed.report {
        for row in group.lv {
            let tags = row
                .lv_tags
                .split(',')
                .filter(|t| !t.is_empty())
                .map(str::to_string)
                .collect();
            let data_percent = row.data_percent.parse::<f64>().ok();
            out.push(Lv {
                lv_name: row.lv_name,
                tags,
                pool_lv: row.pool_lv,
                origin: row.origin,
                data_percent,
            });
        }
    }
    Ok(out)
}
```

- [ ] **Step 4: Run tests**

Run: `cd phermesd && cargo test --lib lvm`
Expected: PASS (6 tests).

- [ ] **Step 5: clippy + commit**

```bash
cd phermesd && cargo clippy --all-targets --all-features -- -D warnings
cd /home/u/dev/phermes/phermes
git add phermesd/src/lvm.rs
git commit -m "feat(phermesd): LVM argv builders + lvs json parser"
```

If clippy flags `missing_errors_doc` add the `# Errors` line shown; do not use `#[allow]`.

---

### Task 3: `lvm.rs` — `LvmOps` trait + `RealLvm`

**Files:** Modify `phermesd/src/lvm.rs`.

- [ ] **Step 1: Add the error type, trait, and real impl (above the tests)**

```rust
use async_trait::async_trait;
use tokio::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum LvmError {
    #[error("running {cmd}: {source}")]
    Spawn {
        cmd: String,
        #[source]
        source: std::io::Error,
    },
    #[error("{cmd} failed ({code}): {stderr}")]
    Failed { cmd: String, code: i32, stderr: String },
    #[error("parsing lvs output: {0}")]
    Parse(#[from] serde_json::Error),
}

/// The LVM side-effect seam. Real impl shells out; tests mock it.
#[async_trait]
pub trait LvmOps: Send + Sync {
    async fn create_thin(&self, vg: &str, pool: &str, name: &str, size_gb: u32) -> Result<(), LvmError>;
    async fn add_tag(&self, device: &str, tag: &str) -> Result<(), LvmError>;
    async fn snapshot(&self, vg: &str, origin: &str, snap_name: &str) -> Result<(), LvmError>;
    async fn merge(&self, vg: &str, snap_name: &str) -> Result<(), LvmError>;
    async fn remove(&self, device: &str) -> Result<(), LvmError>;
    async fn list(&self, vg: &str) -> Result<Vec<Lv>, LvmError>;
}

pub struct RealLvm;

impl RealLvm {
    async fn run(argv: &[String]) -> Result<String, LvmError> {
        let cmd = argv.join(" ");
        let output = Command::new(&argv[0])
            .args(&argv[1..])
            .output()
            .await
            .map_err(|source| LvmError::Spawn { cmd: cmd.clone(), source })?;
        if !output.status.success() {
            return Err(LvmError::Failed {
                cmd,
                code: output.status.code().unwrap_or(-1),
                stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
            });
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[async_trait]
impl LvmOps for RealLvm {
    async fn create_thin(&self, vg: &str, pool: &str, name: &str, size_gb: u32) -> Result<(), LvmError> {
        Self::run(&create_thin_argv(vg, pool, name, size_gb)).await.map(|_| ())
    }
    async fn add_tag(&self, device: &str, tag: &str) -> Result<(), LvmError> {
        Self::run(&addtag_argv(device, tag)).await.map(|_| ())
    }
    async fn snapshot(&self, vg: &str, origin: &str, snap_name: &str) -> Result<(), LvmError> {
        Self::run(&snapshot_argv(vg, origin, snap_name)).await.map(|_| ())
    }
    async fn merge(&self, vg: &str, snap_name: &str) -> Result<(), LvmError> {
        Self::run(&merge_argv(vg, snap_name)).await.map(|_| ())
    }
    async fn remove(&self, device: &str) -> Result<(), LvmError> {
        Self::run(&remove_argv(device)).await.map(|_| ())
    }
    async fn list(&self, vg: &str) -> Result<Vec<Lv>, LvmError> {
        let json = Self::run(&lvs_json_argv(vg)).await?;
        Ok(parse_lvs(&json)?)
    }
}
```

- [ ] **Step 2: Build + clippy**

Run: `cd phermesd && cargo test --lib lvm && cargo clippy --all-targets --all-features -- -D warnings`
Expected: existing 6 tests still pass; no warnings. (`argv[0]` indexing: the builders always produce a non-empty vec; clippy's `indexing_slicing` is not in our deny set, but if pedantic flags it, switch to `argv.split_first()` with a guard.)

- [ ] **Step 3: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add phermesd/src/lvm.rs
git commit -m "feat(phermesd): LvmOps trait + RealLvm executor"
```

---

### Task 4: `btrfs.rs` — `BtrfsOps` trait + builders + `RealBtrfs`

**Files:** Modify `phermesd/src/btrfs.rs`.

- [ ] **Step 1: Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn snapshot_argv_is_readonly() {
        let a = snapshot_argv(Path::new("/ov"), Path::new("/snaps/overlay-auto-x"));
        assert_eq!(a, vec!["btrfs", "subvolume", "snapshot", "-r", "/ov", "/snaps/overlay-auto-x"]);
    }

    #[test]
    fn restore_argv_is_writable_snapshot() {
        let a = restore_argv(Path::new("/snaps/overlay-auto-x"), Path::new("/ov"));
        assert_eq!(a, vec!["btrfs", "subvolume", "snapshot", "/snaps/overlay-auto-x", "/ov"]);
    }

    #[test]
    fn delete_argv() {
        assert_eq!(delete_argv(Path::new("/ov")),
                   vec!["btrfs", "subvolume", "delete", "/ov"]);
    }
}
```

- [ ] **Step 2: Run, verify fail**

Run: `cd phermesd && cargo test --lib btrfs 2>&1 | head`
Expected: FAIL — items not found.

- [ ] **Step 3: Implement (above the tests)**

```rust
//! Btrfs overlay snapshot operations. The overlay is a host-side subvolume; snapshots
//! and rollbacks happen on the host (no guest involvement).

use async_trait::async_trait;
use std::path::Path;
use tokio::process::Command;

#[must_use]
pub fn snapshot_argv(src: &Path, dst: &Path) -> Vec<String> {
    vec![
        "btrfs".into(), "subvolume".into(), "snapshot".into(), "-r".into(),
        src.display().to_string(), dst.display().to_string(),
    ]
}

#[must_use]
pub fn restore_argv(ro_snap: &Path, dst: &Path) -> Vec<String> {
    vec![
        "btrfs".into(), "subvolume".into(), "snapshot".into(),
        ro_snap.display().to_string(), dst.display().to_string(),
    ]
}

#[must_use]
pub fn delete_argv(subvol: &Path) -> Vec<String> {
    vec!["btrfs".into(), "subvolume".into(), "delete".into(), subvol.display().to_string()]
}

#[derive(Debug, thiserror::Error)]
pub enum BtrfsError {
    #[error("running {cmd}: {source}")]
    Spawn {
        cmd: String,
        #[source]
        source: std::io::Error,
    },
    #[error("{cmd} failed ({code}): {stderr}")]
    Failed { cmd: String, code: i32, stderr: String },
}

/// The Btrfs side-effect seam (overlay snapshot/restore/delete).
#[async_trait]
pub trait BtrfsOps: Send + Sync {
    /// Read-only snapshot `src` -> `dst`.
    async fn snapshot(&self, src: &Path, dst: &Path) -> Result<(), BtrfsError>;
    /// Recreate a writable subvolume at `dst` from read-only `ro_snap`.
    async fn restore(&self, ro_snap: &Path, dst: &Path) -> Result<(), BtrfsError>;
    /// Delete a subvolume.
    async fn delete(&self, subvol: &Path) -> Result<(), BtrfsError>;
}

pub struct RealBtrfs;

impl RealBtrfs {
    async fn run(argv: &[String]) -> Result<(), BtrfsError> {
        let cmd = argv.join(" ");
        let output = Command::new(&argv[0])
            .args(&argv[1..])
            .output()
            .await
            .map_err(|source| BtrfsError::Spawn { cmd: cmd.clone(), source })?;
        if !output.status.success() {
            return Err(BtrfsError::Failed {
                cmd,
                code: output.status.code().unwrap_or(-1),
                stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
            });
        }
        Ok(())
    }
}

#[async_trait]
impl BtrfsOps for RealBtrfs {
    async fn snapshot(&self, src: &Path, dst: &Path) -> Result<(), BtrfsError> {
        Self::run(&snapshot_argv(src, dst)).await
    }
    async fn restore(&self, ro_snap: &Path, dst: &Path) -> Result<(), BtrfsError> {
        Self::run(&restore_argv(ro_snap, dst)).await
    }
    async fn delete(&self, subvol: &Path) -> Result<(), BtrfsError> {
        Self::run(&delete_argv(subvol)).await
    }
}
```

- [ ] **Step 4: Run + clippy + commit**

```bash
cd phermesd && cargo test --lib btrfs && cargo clippy --all-targets --all-features -- -D warnings
cd /home/u/dev/phermes/phermes
git add phermesd/src/btrfs.rs
git commit -m "feat(phermesd): BtrfsOps trait + overlay snapshot/restore/delete"
```

---

### Task 5: `qga.rs` — `QgaControl` trait, `QapiQga`, `QgaConnector` + wire test

**Files:** Modify `phermesd/src/qga.rs`; Create `phermesd/tests/qga_wire.rs`.

- [ ] **Step 1: Implement qga.rs**

```rust
//! QEMU Guest Agent control for filesystem quiescing during live snapshots.

use async_trait::async_trait;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum QgaError {
    #[error("connecting QGA at {path}: {source}")]
    Connect {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("QGA protocol error: {0}")]
    Protocol(String),
}

/// The guest-agent operations storage needs.
#[async_trait]
pub trait QgaControl: Send {
    /// Liveness probe.
    async fn ping(&self) -> Result<(), QgaError>;
    /// Freeze guest filesystems; returns the number frozen.
    async fn freeze(&self) -> Result<i64, QgaError>;
    /// Thaw guest filesystems; returns the number thawed.
    async fn thaw(&self) -> Result<i64, QgaError>;
}

/// Opens a `QgaControl` for a guest-agent socket path. Mockable in tests.
#[async_trait]
pub trait QgaConnector: Send + Sync {
    async fn connect(&self, path: &Path) -> Result<Box<dyn QgaControl>, QgaError>;
}

pub use real::{QapiQga, RealQgaConnector};

mod real {
    use super::{QgaConnector, QgaControl, QgaError};
    use async_trait::async_trait;
    use std::path::Path;
    use tokio::sync::Mutex;

    type Service =
        qapi::futures::QapiService<qapi::futures::QgaStreamTokio<tokio::net::unix::OwnedWriteHalf>>;

    pub struct QapiQga {
        service: Mutex<Service>,
    }

    impl QapiQga {
        pub async fn connect(path: &Path) -> Result<Self, QgaError> {
            let stream = qapi::futures::QgaStreamTokio::open_uds(path)
                .await
                .map_err(|source| QgaError::Connect { path: path.display().to_string(), source })?;
            let (service, _handle) = stream.spawn_tokio();
            // QGA requires a sync handshake before commands are accepted.
            let sync_value = std::process::id() as i32;
            service
                .guest_sync(sync_value)
                .await
                .map_err(|e| QgaError::Protocol(e.to_string()))?;
            Ok(Self { service: Mutex::new(service) })
        }
    }

    #[async_trait]
    impl QgaControl for QapiQga {
        async fn ping(&self) -> Result<(), QgaError> {
            let svc = self.service.lock().await;
            svc.execute(qapi::qga::guest_ping {})
                .await
                .map_err(|e| QgaError::Protocol(e.to_string()))?;
            Ok(())
        }
        async fn freeze(&self) -> Result<i64, QgaError> {
            let svc = self.service.lock().await;
            svc.execute(qapi::qga::guest_fsfreeze_freeze {})
                .await
                .map_err(|e| QgaError::Protocol(e.to_string()))
        }
        async fn thaw(&self) -> Result<i64, QgaError> {
            let svc = self.service.lock().await;
            svc.execute(qapi::qga::guest_fsfreeze_thaw {})
                .await
                .map_err(|e| QgaError::Protocol(e.to_string()))
        }
    }

    pub struct RealQgaConnector;

    #[async_trait]
    impl QgaConnector for RealQgaConnector {
        async fn connect(&self, path: &Path) -> Result<Box<dyn QgaControl>, QgaError> {
            Ok(Box::new(QapiQga::connect(path).await?))
        }
    }
}
```

Note: the `Service` type alias mirrors slice #1's `qmp.rs`. If `spawn_tokio()`'s concrete type differs, follow the same adaptation slice #1 used (let-inference / boxing) until `qga_wire` passes. `guest_fsfreeze_freeze`/`_thaw` return `i64` in qapi 0.15; if the generated type differs (e.g. `i32`), match the compiler and update the trait return type consistently.

- [ ] **Step 2: Wire test (real QapiQga vs a fake QGA server)**

Create `phermesd/tests/qga_wire.rs`:

```rust
//! Drives the real QapiQga against a hand-rolled guest-agent server (no guest).

use phermesd::qga::{QapiQga, QgaControl};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;

/// Minimal QGA server: answer guest-sync (echo), guest-ping, fsfreeze-freeze (2), thaw (2).
async fn fake_qga_server(path: PathBuf) -> std::io::Result<()> {
    let listener = UnixListener::bind(&path)?;
    let (stream, _) = listener.accept().await?;
    let (read, mut write) = stream.into_split();
    let mut lines = BufReader::new(read).lines();
    while let Some(line) = lines.next_line().await? {
        if line.contains("guest-sync") {
            // echo back the requested id as the return value
            let id = line.rsplit(':').next().and_then(|s| s.trim_matches(|c: char| !c.is_ascii_digit()).parse::<i64>().ok()).unwrap_or(0);
            write.write_all(format!("{{\"return\":{id}}}\n").as_bytes()).await?;
        } else if line.contains("guest-ping") {
            write.write_all(b"{\"return\":{}}\n").await?;
        } else if line.contains("guest-fsfreeze-freeze") {
            write.write_all(b"{\"return\":2}\n").await?;
        } else if line.contains("guest-fsfreeze-thaw") {
            write.write_all(b"{\"return\":2}\n").await?;
        }
    }
    Ok(())
}

#[tokio::test]
async fn connects_pings_freezes_and_thaws() {
    let dir = tempfile::tempdir().unwrap();
    let sock = dir.path().join("qga.sock");
    let server = tokio::spawn(fake_qga_server(sock.clone()));
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let qga = QapiQga::connect(&sock).await.unwrap();
    qga.ping().await.unwrap();
    assert_eq!(qga.freeze().await.unwrap(), 2);
    assert_eq!(qga.thaw().await.unwrap(), 2);

    server.abort();
}
```

- [ ] **Step 3: Iterate until green**

Run: `cd phermesd && cargo test --test qga_wire -- --nocapture`
Expected: PASS. The guest-sync handshake framing in qapi may require the server to echo the exact sync id; if `connect` hangs, adjust the fake server's guest-sync reply to return the id qapi sent (the parse above extracts the trailing integer). The behavioral assertions (ping/freeze=2/thaw=2) are the contract.

- [ ] **Step 4: clippy + commit**

```bash
cd phermesd && cargo clippy --all-targets --all-features -- -D warnings
cd /home/u/dev/phermes/phermes
git add phermesd/src/qga.rs phermesd/tests/qga_wire.rs
git commit -m "feat(phermesd): QGA control trait + qapi-rs client + wire test"
```

---

### Task 6: `qemu.rs` — guest-agent channel in the argv builder

**Files:** Modify `phermesd/src/qemu.rs`.

- [ ] **Step 1: Add tests to the existing `qemu::tests` module**

```rust
    #[test]
    fn linux_argv_adds_guest_agent_channel() {
        let argv = build_argv(&sample_vm(), &rt()).unwrap();
        // virtio-serial bus
        assert!(argv.iter().any(|a| a == "virtio-serial-pci"));
        // a unix chardev bound to qga.sock
        assert!(argv.iter().any(|a| a
            == "socket,path=/run/phermesd/linux/qga.sock,server=on,wait=off,id=qga0"));
        // the guest-agent virtserialport name
        assert!(argv.iter().any(|a| a
            == "virtserialport,chardev=qga0,name=org.qemu.guest_agent.0"));
    }
```

The `rt()` helper in the existing tests must gain a `qga` field — update it: add `qga: PathBuf::from("/run/phermesd/linux/qga.sock"),` to the `RuntimePaths { .. }` literal in the `rt()` test helper.

- [ ] **Step 2: Run, verify fail**

Run: `cd phermesd && cargo test --lib qemu 2>&1 | head`
Expected: FAIL — `qga` field missing / channel args absent.

- [ ] **Step 3: Add `qga` to `RuntimePaths` and emit the channel**

In `phermesd/src/qemu.rs`, add the field to `RuntimePaths`:

```rust
pub struct RuntimePaths {
    pub vars: PathBuf,
    pub qmp: PathBuf,
    pub serial: PathBuf,
    pub vnc: PathBuf,
    pub pidfile: PathBuf,
    pub qga: PathBuf,
}
```

In `build_linux`, after the `-qmp` pair and before disks, add the guest-agent channel:

```rust
    pair(&mut a, "-device", "virtio-serial-pci".to_string());
    pair(
        &mut a,
        "-chardev",
        format!("socket,path={},server=on,wait=off,id=qga0", rt.qga.display()),
    );
    pair(
        &mut a,
        "-device",
        "virtserialport,chardev=qga0,name=org.qemu.guest_agent.0".to_string(),
    );
```

- [ ] **Step 4: Fix other `RuntimePaths` constructors**

`grep -rn "RuntimePaths {" phermesd/src phermesd/tests` and add `qga: <dir>.join("qga.sock"),` to each literal (the launcher's runtime-paths builder and any test helpers). For `supervisor.rs::runtime_paths`, add `qga: dir.join("qga.sock"),`.

- [ ] **Step 5: Run + clippy**

Run: `cd phermesd && cargo test --lib qemu && cargo clippy --all-targets --all-features -- -D warnings`
Expected: PASS, no warnings.

- [ ] **Step 6: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add phermesd/src/qemu.rs phermesd/src/supervisor.rs phermesd/src/launcher.rs
git commit -m "feat(phermesd): wire QEMU guest-agent virtio-serial channel"
```

---

### Task 7: `storage.rs` — types, config, provision + import (mock-tested)

**Files:** Modify `phermesd/src/storage.rs`; Create `phermesd/tests/storage_lifecycle.rs`.

- [ ] **Step 1: Implement the orchestrator core**

```rust
//! Storage orchestrator: provision/import/checkpoint/rollback/prune over the
//! LvmOps + BtrfsOps + QgaConnector seams. The system (lvs, the @snapshots dir) is
//! the source of truth.

use crate::btrfs::{BtrfsError, BtrfsOps};
use crate::lvm::{Lv, LvmError, LvmOps};
use crate::qga::{QgaConnector, QgaError};
use std::path::PathBuf;
use tokio::process::Command;

pub const OWNER_TAG: &str = "@phermesd";
pub const SNAP_TAG: &str = "@phermesd-snap";

#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub vg: String,            // "pve"
    pub pool: String,          // "data"
    pub overlay: PathBuf,      // /var/lib/phermes/overlay
    pub snapshots_dir: PathBuf, // …/@snapshots
    pub retention: usize,      // keep last N auto checkpoints
    pub pool_threshold: f64,   // refuse auto-snap above this data%
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
    fn as_str(self) -> &'static str {
        match self {
            SnapKind::Auto => "auto",
            SnapKind::Manual => "manual",
        }
    }
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

fn disk_name(vmid: u32) -> String {
    format!("vm-{vmid}-disk-0")
}
fn disk_device(vg: &str, vmid: u32) -> String {
    format!("/dev/{vg}/{}", disk_name(vmid))
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

    async fn managed_disk<'a>(&self, lvs: &'a [Lv], vmid: u32) -> Option<&'a Lv> {
        let name = disk_name(vmid);
        lvs.iter()
            .find(|l| l.lv_name == name && l.tags.iter().any(|t| t == OWNER_TAG))
    }

    /// Provision a thin VM disk and optionally import a local source image.
    ///
    /// # Errors
    /// Propagates LVM errors; `SourceTooLarge` if the image doesn't fit; tool errors on import.
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
                // No half-state: remove the volume we just made.
                let _ = self.lvm.remove(&device).await;
                return Err(e);
            }
        }
        Ok(())
    }

    async fn import(&self, src: &std::path::Path, device: &str, vol_gb: u32) -> Result<(), StorageError> {
        let src_gb = qemu_img_virtual_gb(src).await?;
        if src_gb > u64::from(vol_gb) {
            return Err(StorageError::SourceTooLarge {
                src: src.display().to_string(),
                src_gb,
                vol_gb,
            });
        }
        run_tool(&[
            "qemu-img".into(), "convert".into(), "-O".into(), "raw".into(),
            src.display().to_string(), device.to_string(),
        ])
        .await
    }

    /// Delete a managed VM disk and its snapshots. Caller passes the current `lvs` snapshot.
    ///
    /// # Errors
    /// `NotManaged` if the disk lacks the owner tag; LVM errors on removal.
    pub async fn delete(&self, vmid: u32, lvs: &[Lv]) -> Result<(), StorageError> {
        let name = disk_name(vmid);
        if self.managed_disk(lvs, vmid).await.is_none() {
            return Err(StorageError::NotManaged(name));
        }
        for snap in lvs.iter().filter(|l| l.origin == name) {
            self.lvm.remove(&format!("/dev/{}/{}", self.cfg.vg, snap.lv_name)).await?;
        }
        self.lvm.remove(&disk_device(&self.cfg.vg, vmid)).await?;
        Ok(())
    }
}

async fn qemu_img_virtual_gb(src: &std::path::Path) -> Result<u64, StorageError> {
    let out = run_tool_output(&[
        "qemu-img".into(), "info".into(), "--output=json".into(), src.display().to_string(),
    ])
    .await?;
    #[derive(serde::Deserialize)]
    struct Info {
        #[serde(rename = "virtual-size")]
        virtual_size: u64,
    }
    let info: Info = serde_json::from_str(&out).map_err(|e| StorageError::ToolFailed {
        cmd: "qemu-img info".into(),
        code: -1,
        stderr: e.to_string(),
    })?;
    // round up bytes -> GiB
    Ok(info.virtual_size.div_ceil(1024 * 1024 * 1024))
}

async fn run_tool(argv: &[String]) -> Result<(), StorageError> {
    run_tool_output(argv).await.map(|_| ())
}

async fn run_tool_output(argv: &[String]) -> Result<String, StorageError> {
    let cmd = argv.join(" ");
    let output = Command::new(&argv[0])
        .args(&argv[1..])
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
```

- [ ] **Step 2: Create the mock harness + provision tests**

Create `phermesd/tests/storage_lifecycle.rs`:

```rust
//! Storage orchestration via mock Lvm/Btrfs/Qga (no root, no real tools).

use async_trait::async_trait;
use phermesd::btrfs::{BtrfsError, BtrfsOps};
use phermesd::lvm::{Lv, LvmError, LvmOps};
use phermesd::qga::{QgaConnector, QgaControl, QgaError};
use phermesd::storage::{Storage, StorageConfig, StorageError};
use std::path::Path;
use std::sync::Mutex;

#[derive(Default)]
struct MockLvm {
    lvs: Mutex<Vec<Lv>>,
    calls: Mutex<Vec<String>>,
}
fn lv(name: &str, tags: &[&str], origin: &str, dp: Option<f64>) -> Lv {
    Lv {
        lv_name: name.into(),
        tags: tags.iter().map(|s| s.to_string()).collect(),
        pool_lv: "data".into(),
        origin: origin.into(),
        data_percent: dp,
    }
}

#[async_trait]
impl LvmOps for MockLvm {
    async fn create_thin(&self, _vg: &str, _pool: &str, name: &str, _gb: u32) -> Result<(), LvmError> {
        self.calls.lock().unwrap().push(format!("create {name}"));
        self.lvs.lock().unwrap().push(lv(name, &[], "", None));
        Ok(())
    }
    async fn add_tag(&self, device: &str, tag: &str) -> Result<(), LvmError> {
        self.calls.lock().unwrap().push(format!("tag {device} {tag}"));
        let name = device.rsplit('/').next().unwrap().to_string();
        if let Some(l) = self.lvs.lock().unwrap().iter_mut().find(|l| l.lv_name == name) {
            l.tags.push(tag.to_string());
        }
        Ok(())
    }
    async fn snapshot(&self, _vg: &str, origin: &str, snap: &str) -> Result<(), LvmError> {
        self.calls.lock().unwrap().push(format!("snap {snap}"));
        self.lvs.lock().unwrap().push(lv(snap, &["@phermesd-snap"], origin, None));
        Ok(())
    }
    async fn merge(&self, _vg: &str, snap: &str) -> Result<(), LvmError> {
        self.calls.lock().unwrap().push(format!("merge {snap}"));
        Ok(())
    }
    async fn remove(&self, device: &str) -> Result<(), LvmError> {
        self.calls.lock().unwrap().push(format!("remove {device}"));
        let name = device.rsplit('/').next().unwrap().to_string();
        self.lvs.lock().unwrap().retain(|l| l.lv_name != name);
        Ok(())
    }
    async fn list(&self, _vg: &str) -> Result<Vec<Lv>, LvmError> {
        Ok(self.lvs.lock().unwrap().clone())
    }
}

#[derive(Default)]
struct MockBtrfs {
    calls: Mutex<Vec<String>>,
}
#[async_trait]
impl BtrfsOps for MockBtrfs {
    async fn snapshot(&self, _src: &Path, dst: &Path) -> Result<(), BtrfsError> {
        self.calls.lock().unwrap().push(format!("snap {}", dst.display()));
        Ok(())
    }
    async fn restore(&self, _ro: &Path, dst: &Path) -> Result<(), BtrfsError> {
        self.calls.lock().unwrap().push(format!("restore {}", dst.display()));
        Ok(())
    }
    async fn delete(&self, subvol: &Path) -> Result<(), BtrfsError> {
        self.calls.lock().unwrap().push(format!("delete {}", subvol.display()));
        Ok(())
    }
}

struct CountingQga {
    frozen: std::sync::Arc<std::sync::atomic::AtomicI64>,
}
#[async_trait]
impl QgaControl for CountingQga {
    async fn ping(&self) -> Result<(), QgaError> {
        Ok(())
    }
    async fn freeze(&self) -> Result<i64, QgaError> {
        self.frozen.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Ok(2)
    }
    async fn thaw(&self) -> Result<i64, QgaError> {
        self.frozen.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        Ok(2)
    }
}

#[derive(Default)]
struct MockConnector {
    frozen: std::sync::Arc<std::sync::atomic::AtomicI64>,
    fail: bool,
}
#[async_trait]
impl QgaConnector for MockConnector {
    async fn connect(&self, _path: &Path) -> Result<Box<dyn QgaControl>, QgaError> {
        if self.fail {
            return Err(QgaError::Protocol("agent down".into()));
        }
        Ok(Box::new(CountingQga { frozen: self.frozen.clone() }))
    }
}

fn cfg() -> StorageConfig {
    StorageConfig { retention: 2, ..StorageConfig::default() }
}

#[tokio::test]
async fn provision_creates_tagged_volume() {
    let lvm = Box::new(MockLvm::default());
    let storage = Storage::new(cfg(), lvm, Box::new(MockBtrfs::default()), Box::new(MockConnector::default()));
    storage.provision(102, 40, None, false).await.unwrap();
    let lvs = storage_list(&storage).await;
    let disk = lvs.iter().find(|l| l.lv_name == "vm-102-disk-0").unwrap();
    assert!(disk.tags.iter().any(|t| t == "@phermesd"));
}

#[tokio::test]
async fn provision_existing_without_force_errors() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["@phermesd"], "", None));
    let storage = Storage::new(cfg(), Box::new(lvm), Box::new(MockBtrfs::default()), Box::new(MockConnector::default()));
    assert!(matches!(
        storage.provision(102, 40, None, false).await,
        Err(StorageError::NotManaged(_))
    ));
}

// helper to read the mock's lvs back through the Storage's lvm — expose via a re-list
async fn storage_list(_s: &Storage) -> Vec<Lv> {
    // The MockLvm is owned by Storage; assert via a fresh provision side effect instead.
    // For provision_creates_tagged_volume we re-create a parallel mock to inspect:
    Vec::new()
}
```

Note: because `Storage` takes ownership of the mocks, inspect side effects by keeping an `Arc`-shared handle to the mock instead of the `storage_list` stub above. Concretely: change `MockLvm`'s `lvs`/`calls` to `Arc<Mutex<…>>`, clone the `Arc` before moving the mock into `Storage`, and assert on the clone. Apply the same `Arc`-handle pattern to `MockBtrfs.calls` and `MockConnector.frozen` (already `Arc`). Replace the `storage_list` stub with direct assertions on the shared `Arc<Mutex<Vec<Lv>>>`.

- [ ] **Step 3: Run, fix the Arc-handle wiring, green**

Run: `cd phermesd && cargo test --test storage_lifecycle 2>&1 | head -30`
Expected: after switching the mock fields to shared `Arc` handles, PASS (provision tests).

- [ ] **Step 4: clippy + commit**

```bash
cd phermesd && cargo clippy --all-targets --all-features -- -D warnings
cd /home/u/dev/phermes/phermes
git add phermesd/src/storage.rs phermesd/tests/storage_lifecycle.rs
git commit -m "feat(phermesd): storage orchestrator — provision/import/delete"
```

---

### Task 8: `storage.rs` — checkpoint (freeze→snap→thaw, atomicity, pool-guard)

**Files:** Modify `phermesd/src/storage.rs`, `phermesd/tests/storage_lifecycle.rs`.

- [ ] **Step 1: Add the checkpoint API + types to storage.rs**

```rust
use chrono::Utc; // add `cargo add chrono --no-default-features --features clock`

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checkpoint {
    pub vmid: u32,
    pub utc: String,
    pub kind: SnapKind,
}

impl Storage {
    fn utc_stamp() -> String {
        Utc::now().format("%Y%m%dT%H%M%SZ").to_string()
    }

    fn overlay_snap_path(&self, kind: SnapKind, utc: &str) -> PathBuf {
        self.cfg.snapshots_dir.join(format!("overlay-{}-{utc}", kind.as_str()))
    }

    async fn pool_data_percent(&self, lvs: &[Lv]) -> f64 {
        lvs.iter()
            .find(|l| l.lv_name == self.cfg.pool)
            .and_then(|l| l.data_percent)
            .unwrap_or(0.0)
    }

    /// Take a checkpoint (VM disk + overlay). If `qga_sock` is Some the VM is active and is
    /// quiesced best-effort; None means a stopped VM (cold, consistent).
    ///
    /// # Errors
    /// `PoolFull` if over threshold (auto only); LVM/Btrfs errors; partial snapshots are
    /// rolled back and the guest is always thawed.
    pub async fn checkpoint(
        &self,
        vmid: u32,
        kind: SnapKind,
        qga_sock: Option<PathBuf>,
    ) -> Result<Checkpoint, StorageError> {
        let lvs = self.lvm.list(&self.cfg.vg).await?;
        if self.managed_disk(&lvs, vmid).await.is_none() {
            return Err(StorageError::NotManaged(disk_name(vmid)));
        }
        if kind == SnapKind::Auto {
            let pct = self.pool_data_percent(&lvs).await;
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

        // Best-effort quiesce.
        let frozen = self.try_freeze(qga_sock.as_deref()).await;

        // Snapshot both domains; on any failure, thaw + clean partial, then error.
        let result = async {
            self.lvm.snapshot(&self.cfg.vg, &disk_name(vmid), &snap_name).await?;
            if let Err(e) = self.btrfs.snapshot(&self.cfg.overlay, &overlay_dst).await {
                // roll back the disk snap we just made
                let _ = self.lvm.remove(&format!("/dev/{}/{snap_name}", self.cfg.vg)).await;
                return Err(StorageError::from(e));
            }
            Ok(())
        }
        .await;

        if let Some(qga) = frozen {
            let _ = qga.thaw().await; // always release; ignore thaw error
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

    async fn prune_auto(&self, _vmid: u32) -> Result<(), StorageError> {
        // implemented in Task 9
        Ok(())
    }
}
```

Add `use crate::qga::QgaControl;` to the imports. Run `cargo add chrono --no-default-features --features clock` in `phermesd/`.

- [ ] **Step 2: Tests — quiesce ordering, thaw-always, pool-guard**

Append to `storage_lifecycle.rs`:

```rust
#[tokio::test]
async fn checkpoint_quiesces_and_thaws_when_active() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["@phermesd"], "", None));
    lvm.lvs.lock().unwrap().push(lv("data", &[], "", Some(10.0)));
    let frozen = std::sync::Arc::new(std::sync::atomic::AtomicI64::new(0));
    let conn = MockConnector { frozen: frozen.clone(), fail: false };
    let storage = Storage::new(cfg(), Box::new(lvm), Box::new(MockBtrfs::default()), Box::new(conn));
    let cp = storage
        .checkpoint(102, phermesd::storage::SnapKind::Manual, Some("/run/phermesd/x/qga.sock".into()))
        .await
        .unwrap();
    assert_eq!(cp.vmid, 102);
    // freeze(+1) then thaw(-1) => back to 0
    assert_eq!(frozen.load(std::sync::atomic::Ordering::SeqCst), 0);
}

#[tokio::test]
async fn checkpoint_without_agent_is_crash_consistent() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["@phermesd"], "", None));
    lvm.lvs.lock().unwrap().push(lv("data", &[], "", Some(10.0)));
    let conn = MockConnector { frozen: Default::default(), fail: true }; // agent down
    let storage = Storage::new(cfg(), Box::new(lvm), Box::new(MockBtrfs::default()), Box::new(conn));
    // Still succeeds (falls back), even with a qga_sock provided.
    storage
        .checkpoint(102, phermesd::storage::SnapKind::Manual, Some("/x/qga.sock".into()))
        .await
        .unwrap();
}

#[tokio::test]
async fn auto_checkpoint_refused_when_pool_above_threshold() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["@phermesd"], "", None));
    lvm.lvs.lock().unwrap().push(lv("data", &[], "", Some(95.0)));
    let storage = Storage::new(cfg(), Box::new(lvm), Box::new(MockBtrfs::default()), Box::new(MockConnector::default()));
    assert!(matches!(
        storage.checkpoint(102, phermesd::storage::SnapKind::Auto, None).await,
        Err(StorageError::PoolFull { .. })
    ));
}
```

(Apply the `Arc`-shared-handle pattern from Task 7 so the mock's `lvs` can be both seeded and owned by `Storage`.)

- [ ] **Step 3: Run + clippy + commit**

```bash
cd phermesd && cargo test --test storage_lifecycle && cargo clippy --all-targets --all-features -- -D warnings
cd /home/u/dev/phermes/phermes
git add phermesd/src/storage.rs phermesd/tests/storage_lifecycle.rs phermesd/Cargo.toml phermesd/Cargo.lock
git commit -m "feat(phermesd): checkpoint with QGA quiesce + pool guard"
```

---

### Task 9: `storage.rs` — list, prune (retention), rollback

**Files:** Modify `phermesd/src/storage.rs`, `phermesd/tests/storage_lifecycle.rs`.

- [ ] **Step 1: Implement list/prune/rollback**

```rust
impl Storage {
    /// List checkpoints for a VM (grouped by UTC), newest first.
    ///
    /// # Errors
    /// Propagates LVM list errors.
    pub async fn checkpoints(&self, vmid: u32) -> Result<Vec<Checkpoint>, StorageError> {
        let lvs = self.lvm.list(&self.cfg.vg).await?;
        let prefix = format!("{}-snap-", disk_name(vmid));
        let mut cps: Vec<Checkpoint> = lvs
            .iter()
            .filter_map(|l| parse_snap(&l.lv_name, &prefix).map(|(kind, utc)| Checkpoint { vmid, utc, kind }))
            .collect();
        cps.sort_by(|a, b| b.utc.cmp(&a.utc)); // newest first
        Ok(cps)
    }

    async fn prune_auto(&self, vmid: u32) -> Result<(), StorageError> {
        let mut autos: Vec<Checkpoint> = self
            .checkpoints(vmid)
            .await?
            .into_iter()
            .filter(|c| c.kind == SnapKind::Auto)
            .collect();
        // newest first; keep `retention`, remove the rest
        for cp in autos.split_off(self.cfg.retention.min(autos.len())) {
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
    /// `NotFound` if the checkpoint is absent; LVM/Btrfs errors. Caller must ensure the VM
    /// is stopped (the control layer enforces this via the supervisor).
    pub async fn rollback(&self, vmid: u32, utc: &str) -> Result<(), StorageError> {
        let cps = self.checkpoints(vmid).await?;
        let cp = cps
            .iter()
            .find(|c| c.utc == utc)
            .ok_or_else(|| StorageError::NotFound(format!("checkpoint {utc} for vm {vmid}")))?;
        let snap = format!("{}-snap-{}-{}", disk_name(vmid), cp.kind.as_str(), cp.utc);
        // VM disk: merge the thin snapshot back into the origin.
        self.lvm.merge(&self.cfg.vg, &snap).await?;
        // Overlay: replace the live subvol with a writable copy of the RO snapshot.
        let ro = self.overlay_snap_path(cp.kind, &cp.utc);
        self.btrfs.delete(&self.cfg.overlay).await?;
        self.btrfs.restore(&ro, &self.cfg.overlay).await?;
        Ok(())
    }
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
```

- [ ] **Step 2: Tests — prune keeps N, rollback merges + restores**

Append to `storage_lifecycle.rs`:

```rust
#[tokio::test]
async fn auto_prune_keeps_last_n() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["@phermesd"], "", None));
    lvm.lvs.lock().unwrap().push(lv("data", &[], "", Some(5.0)));
    // pre-seed two old auto snaps (retention=2, so after a 3rd, oldest is pruned)
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0-snap-auto-20260101T000000Z", &["@phermesd-snap"], "vm-102-disk-0", None));
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0-snap-auto-20260102T000000Z", &["@phermesd-snap"], "vm-102-disk-0", None));
    let btrfs = MockBtrfs::default();
    let storage = Storage::new(cfg(), Box::new(lvm), Box::new(btrfs), Box::new(MockConnector::default()));
    // a 3rd auto checkpoint triggers prune of the oldest (20260101)
    storage.checkpoint(102, phermesd::storage::SnapKind::Auto, None).await.unwrap();
    let cps = storage.checkpoints(102).await.unwrap();
    let autos: Vec<_> = cps.iter().filter(|c| c.kind == phermesd::storage::SnapKind::Auto).collect();
    assert_eq!(autos.len(), 2);
    assert!(!autos.iter().any(|c| c.utc == "20260101T000000Z"));
}

#[tokio::test]
async fn rollback_merges_disk_and_restores_overlay() {
    let lvm = MockLvm::default();
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0", &["@phermesd"], "", None));
    lvm.lvs.lock().unwrap().push(lv("vm-102-disk-0-snap-manual-20260601T000000Z", &["@phermesd-snap"], "vm-102-disk-0", None));
    // shared handle to inspect btrfs calls
    let storage = Storage::new(cfg(), Box::new(lvm), Box::new(MockBtrfs::default()), Box::new(MockConnector::default()));
    storage.rollback(102, "20260601T000000Z").await.unwrap();
    // (assert via shared Arc handles on MockLvm.calls containing "merge …" and MockBtrfs.calls containing delete+restore)
}
```

(Use the shared-`Arc` handle pattern to assert `MockLvm.calls` contains `merge vm-102-disk-0-snap-manual-20260601T000000Z` and `MockBtrfs.calls` contains a `delete` then `restore`.)

- [ ] **Step 3: Run + clippy + commit**

```bash
cd phermesd && cargo test --test storage_lifecycle && cargo clippy --all-targets --all-features -- -D warnings
cd /home/u/dev/phermes/phermes
git add phermesd/src/storage.rs phermesd/tests/storage_lifecycle.rs
git commit -m "feat(phermesd): checkpoint list/prune/rollback"
```

---

### Task 10: `proto.rs` + `control.rs` — storage verbs

**Files:** Modify `phermesd/src/proto.rs`, `phermesd/src/control.rs`.

- [ ] **Step 1: Add Request variants + info types to proto.rs**

In the `Request` enum (keep `#[serde(tag="cmd", rename_all="lowercase")]`), add:

```rust
    Provision {
        vmid: u32,
        #[serde(default)]
        from: Option<String>,
        #[serde(default)]
        size: Option<u32>,
        #[serde(default)]
        force: bool,
    },
    Delete {
        vmid: u32,
    },
    Snapshot {
        vmid: u32,
    },
    Rollback {
        vmid: u32,
        checkpoint: String,
    },
    Snapshots {
        #[serde(default)]
        vmid: Option<u32>,
    },
```

Add a serializable checkpoint view:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointInfo {
    pub vmid: u32,
    pub utc: String,
    pub kind: String,
}
```

Add a test:

```rust
    #[test]
    fn provision_request_round_trips() {
        let r: Request = serde_json::from_str(r#"{"cmd":"provision","vmid":102,"size":40}"#).unwrap();
        assert_eq!(r, Request::Provision { vmid: 102, from: None, size: Some(40), force: false });
    }
```

- [ ] **Step 2: Dispatch in control.rs**

The current `dispatch` signature is `dispatch(sup: &Mutex<Supervisor>, vms_dir: &Path, req: Request)`. Storage needs a `Storage` handle and the supervisor (to check active + get the qga sock). Extend `dispatch` to also take `storage: &Mutex<Storage>`:

```rust
pub async fn dispatch(
    sup: &Mutex<Supervisor>,
    storage: &Mutex<Storage>,
    vms_dir: &Path,
    req: Request,
) -> Response {
    // ... existing arms unchanged ...
    // new arms:
    // Provision
    Request::Provision { vmid, from, size, force } => {
        let size_gb = size.unwrap_or_else(|| default_disk_gb(vmid));
        let src = from.as_ref().map(std::path::PathBuf::from);
        let st = storage.lock().await;
        st.provision(vmid, size_gb, src.as_deref(), force).await
            .map(|()| serde_json::json!({"vmid": vmid, "size": size_gb}))
            .map_err(StorageErr)
    }
    // Snapshot (manual) — quiesce only if this vmid is the active VM
    Request::Snapshot { vmid } => {
        let qga_sock = active_qga_sock(sup, vmid).await;
        let st = storage.lock().await;
        st.checkpoint(vmid, SnapKind::Manual, qga_sock).await
            .map(|cp| serde_json::json!(to_info(&cp)))
            .map_err(StorageErr)
    }
    // Rollback — refuse if active
    Request::Rollback { vmid, checkpoint } => {
        if is_active(sup, vmid).await {
            return Response::err("vm_active", "stop the VM before rolling back");
        }
        let st = storage.lock().await;
        st.rollback(vmid, &checkpoint).await
            .map(|()| serde_json::json!({"vmid": vmid, "checkpoint": checkpoint}))
            .map_err(StorageErr)
    }
    // Delete — refuse if active
    Request::Delete { vmid } => {
        if is_active(sup, vmid).await {
            return Response::err("vm_active", "stop the VM before deleting its disk");
        }
        let st = storage.lock().await;
        let lvs = match st.list_lvs().await { Ok(l) => l, Err(e) => return storage_response(&e) };
        st.delete(vmid, &lvs).await.map(|()| serde_json::json!({"vmid": vmid})).map_err(StorageErr)
    }
    // Snapshots (list)
    Request::Snapshots { vmid } => {
        let st = storage.lock().await;
        // for a single vmid; listing all is the union over defined vmids (kept simple: require vmid)
        match vmid {
            Some(id) => st.checkpoints(id).await
                .map(|cps| serde_json::json!(cps.iter().map(to_info).collect::<Vec<_>>()))
                .map_err(StorageErr),
            None => return Response::err("bad_request", "snapshots requires a vmid"),
        }
    }
```

Add the supporting helpers to control.rs:

```rust
use crate::storage::{SnapKind, Storage, StorageError, Checkpoint};
use crate::proto::CheckpointInfo;

fn default_disk_gb(vmid: u32) -> u32 {
    match vmid {
        100 => 120, // macOS
        101 => 100, // Windows
        _ => 40,    // Linux / default
    }
}

fn to_info(cp: &Checkpoint) -> CheckpointInfo {
    CheckpointInfo {
        vmid: cp.vmid,
        utc: cp.utc.clone(),
        kind: match cp.kind { SnapKind::Auto => "auto".into(), SnapKind::Manual => "manual".into() },
    }
}

fn storage_response(e: &StorageError) -> Response {
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
```

Add a `Storage::list_lvs` public method (`pub async fn list_lvs(&self) -> Result<Vec<Lv>, StorageError>`) returning `self.lvm.list(&self.cfg.vg).await.map_err(Into::into)`, and a `pub use crate::lvm::Lv` re-export, so `dispatch` can pass `lvs` to `delete`. The `StorageErr`/`map_err` shorthand above means: on `Err`, build `storage_response(&e)`; implement the arms to return `Response` directly (replace `.map(...).map_err(StorageErr)` with explicit `match` producing `Response::ok`/`storage_response`), matching the existing arm style in `dispatch`.

The supervisor needs two helpers (add to `supervisor.rs` in Task 12, used here): `Supervisor::active_id() -> Option<&str>` and a way to get the active VM's qga sock path. For now define the control helpers to use the supervisor:

```rust
async fn is_active(sup: &Mutex<Supervisor>, vmid: u32) -> bool {
    sup.lock().await.active_vmid() == Some(vmid)
}
async fn active_qga_sock(sup: &Mutex<Supervisor>, vmid: u32) -> Option<std::path::PathBuf> {
    let s = sup.lock().await;
    if s.active_vmid() == Some(vmid) { s.active_qga_sock() } else { None }
}
```

`Supervisor::active_vmid()` and `active_qga_sock()` are added in Task 12.

- [ ] **Step 3: Update the serve()/handle_conn wiring to thread `storage`**

`serve` and `handle_conn` must take `Arc<Mutex<Storage>>` alongside the supervisor; thread it through to `dispatch`. Update their signatures and the call site.

- [ ] **Step 4: Update existing dispatch tests**

The Task-11 (slice #1) dispatch tests call `dispatch(&sup, &vms_dir, …)`; add a `Mutex<Storage>` built from mocks and pass it. Provide a small `test_storage()` helper in the control test module that builds `Storage` from the slice-#2 mocks (or trivial no-op mocks) so existing tests compile.

- [ ] **Step 5: Run + clippy + commit**

```bash
cd phermesd && cargo test --lib proto && cargo test --lib control && cargo clippy --all-targets --all-features -- -D warnings
cd /home/u/dev/phermes/phermes
git add phermesd/src/proto.rs phermesd/src/control.rs
git commit -m "feat(phermesd): storage control verbs (provision/snapshot/rollback/delete/snapshots)"
```

---

### Task 11: `phermesctl` storage subcommands

**Files:** Modify `phermesd/src/bin/phermesctl.rs`.

- [ ] **Step 1: Add subcommands to the `Cmd` enum + `From<Cmd> for Request`**

```rust
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
```

In `From<Cmd> for Request`:

```rust
            Cmd::Provision { vmid, from, size, force } => Request::Provision { vmid, from, size, force },
            Cmd::Delete { vmid } => Request::Delete { vmid },
            Cmd::Snapshot { vmid } => Request::Snapshot { vmid },
            Cmd::Rollback { vmid, checkpoint } => Request::Rollback { vmid, checkpoint },
            Cmd::Snapshots { vmid } => Request::Snapshots { vmid: Some(vmid) },
```

- [ ] **Step 2: Build + clippy**

Run: `cd phermesd && cargo build --bin phermesctl && cargo clippy --all-targets --all-features -- -D warnings`
Expected: builds, no warnings.

- [ ] **Step 3: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add phermesd/src/bin/phermesctl.rs
git commit -m "feat(phermesd): phermesctl storage subcommands"
```

---

### Task 12: `supervisor.rs` — active accessors + auto-checkpoint before switch

**Files:** Modify `phermesd/src/supervisor.rs`, `phermesd/src/bin/phermesd.rs`, `phermesd/tests/supervisor_lifecycle.rs`.

- [ ] **Step 1: Add active accessors + a Storage handle + auto-checkpoint hook**

The supervisor must hold a `Storage` handle (for auto-checkpoint) without creating a cyclic borrow with the control layer. Give `Supervisor` an `Option<Arc<Mutex<Storage>>>` set after construction, plus accessors:

```rust
// fields
storage: Option<std::sync::Arc<tokio::sync::Mutex<crate::storage::Storage>>>,

// in new(): storage: None,

pub fn set_storage(&mut self, storage: std::sync::Arc<tokio::sync::Mutex<crate::storage::Storage>>) {
    self.storage = Some(storage);
}

#[must_use]
pub fn active_vmid(&self) -> Option<u32> {
    self.active.as_ref().and_then(|a| vmid_of(&a.id))
}

#[must_use]
pub fn active_qga_sock(&self) -> Option<std::path::PathBuf> {
    self.active.as_ref().map(|a| a.rt.qga.clone())
}
```

Add `vmid_of(id) -> Option<u32>` mapping a VM id string to its numeric id (`linux=>102`, `windows=>101`, `macos=>100`; otherwise parse digits):

```rust
fn vmid_of(id: &str) -> Option<u32> {
    match id {
        "macos" => Some(100),
        "windows" => Some(101),
        "linux" => Some(102),
        other => other.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().ok(),
    }
}
```

In `activate`, in the branch that stops a running different VM before the switch, take an auto-checkpoint of the outgoing VM **before** stopping it:

```rust
        if let Some(active) = &self.active {
            if active.id == id {
                let vm = self.find(id)?;
                return Ok(self.info_for(vm));
            }
            // auto-checkpoint the outgoing VM (best-effort: warn + continue on failure)
            if let (Some(storage), Some(out_vmid)) = (self.storage.clone(), vmid_of(&active.id)) {
                let qga = Some(active.rt.qga.clone());
                let st = storage.lock().await;
                if let Err(e) = st.checkpoint(out_vmid, crate::storage::SnapKind::Auto, qga).await {
                    tracing::warn!(vmid = out_vmid, error = %e, "auto-checkpoint before switch failed; continuing");
                }
            }
            self.stop(None).await?;
        }
```

- [ ] **Step 2: Wire the daemon (bin/phermesd.rs)**

After building `Supervisor` and before `serve`: construct `Storage` (real impls), wrap in `Arc<Mutex<_>>`, call `supervisor.set_storage(storage.clone())`, and pass `storage` into `serve(...)`:

```rust
    let storage = std::sync::Arc::new(tokio::sync::Mutex::new(
        phermesd::storage::Storage::new(
            phermesd::storage::StorageConfig::default(),
            Box::new(phermesd::lvm::RealLvm),
            Box::new(phermesd::btrfs::RealBtrfs),
            Box::new(phermesd::qga::RealQgaConnector),
        ),
    ));
    supervisor.set_storage(storage.clone());
    let sup = Arc::new(Mutex::new(supervisor));
    serve(&args.socket, args.vms_dir, sup, storage).await?;
```

- [ ] **Step 3: Test — auto-checkpoint fires on switch**

Append to `supervisor_lifecycle.rs` a test that sets a storage built from the slice-#2 mocks (a `MockLvm` etc. pre-seeded with both VMs' tagged disks + a `data` row), activates `linux` then `windows`, and asserts the mock recorded a snapshot for vmid 102 before the switch. (Reuse the mock pattern; import the slice-#2 mocks or define minimal local ones.) Assert the switch still succeeds even if the storage mock's `checkpoint` returns `PoolFull` (warn-and-continue): inject a `data` row at 95% and assert `activate("windows")` still returns `Running`.

- [ ] **Step 4: Run + clippy + commit**

```bash
cd phermesd && cargo test && cargo clippy --all-targets --all-features -- -D warnings
cd /home/u/dev/phermes/phermes
git add phermesd/src/supervisor.rs phermesd/src/bin/phermesd.rs phermesd/tests/supervisor_lifecycle.rs
git commit -m "feat(phermesd): auto-checkpoint outgoing VM before a switch"
```

---

### Task 13: Gated loop-device integration + E2E + just + docs

**Files:** Create `phermesd/tests/storage_integration.rs`; Modify `Justfile`, `README.md`, `CHANGELOG.md`.

- [ ] **Step 1: Gated loop-device integration test**

Create `phermesd/tests/storage_integration.rs`:

```rust
//! Gated: real LVM-thin pool + Btrfs overlay on loop devices.
//! Run with: `sudo -E cargo test --test storage_integration -- --ignored --nocapture`
//! Requires root, lvm2, btrfs-progs, qemu-utils.

use phermesd::btrfs::RealBtrfs;
use phermesd::lvm::RealLvm;
use phermesd::qga::RealQgaConnector;
use phermesd::storage::{SnapKind, Storage, StorageConfig};
use std::path::PathBuf;
use std::process::Command;

fn sh(argv: &[&str]) {
    let status = Command::new(argv[0]).args(&argv[1..]).status().unwrap();
    assert!(status.success(), "command failed: {argv:?}");
}

#[tokio::test]
#[ignore = "needs root + lvm2 + btrfs-progs + loop devices"]
async fn provision_snapshot_rollback_on_loop_devices() {
    // 2 GiB backing file -> loop -> PV -> VG phermes_test -> thin pool data
    let dir = tempfile::tempdir().unwrap();
    let img = dir.path().join("pv.img");
    sh(&["truncate", "-s", "2G", img.to_str().unwrap()]);
    let loopdev = String::from_utf8(
        Command::new("losetup").args(["-f", "--show", img.to_str().unwrap()]).output().unwrap().stdout,
    ).unwrap().trim().to_string();
    sh(&["pvcreate", "-y", &loopdev]);
    sh(&["vgcreate", "phermes_test", &loopdev]);
    sh(&["lvcreate", "--type", "thin-pool", "-L", "1G", "-n", "data", "phermes_test"]);

    // Btrfs overlay on a second loop file
    let bimg = dir.path().join("btrfs.img");
    sh(&["truncate", "-s", "512M", bimg.to_str().unwrap()]);
    sh(&["mkfs.btrfs", "-f", bimg.to_str().unwrap()]);
    let overlay = dir.path().join("overlay");
    std::fs::create_dir_all(&overlay).unwrap();
    sh(&["mount", "-o", "loop", bimg.to_str().unwrap(), overlay.to_str().unwrap()]);
    let snaps = overlay.join("@snapshots");
    std::fs::create_dir_all(&snaps).unwrap();

    let cfg = StorageConfig {
        vg: "phermes_test".into(),
        pool: "data".into(),
        overlay: overlay.clone(),
        snapshots_dir: snaps.clone(),
        retention: 2,
        pool_threshold: 95.0,
    };
    let storage = Storage::new(cfg, Box::new(RealLvm), Box::new(RealBtrfs), Box::new(RealQgaConnector));

    storage.provision(102, 1, None, false).await.unwrap();
    let cp = storage.checkpoint(102, SnapKind::Manual, None).await.unwrap();
    let cps = storage.checkpoints(102).await.unwrap();
    assert!(cps.iter().any(|c| c.utc == cp.utc));
    storage.rollback(102, &cp.utc).await.unwrap();

    // teardown (best-effort)
    let _ = Command::new("umount").arg(overlay.to_str().unwrap()).status();
    let _ = Command::new("vgremove").args(["-y", "phermes_test"]).status();
    let _ = Command::new("losetup").args(["-d", &loopdev]).status();
}
```

- [ ] **Step 2: Confirm it compiles + is ignored**

Run: `cd phermesd && cargo test --test storage_integration`
Expected: compiles; 0 run, 1 ignored.

- [ ] **Step 3: Justfile recipes**

Append:

```just
# Storage integration (real LVM-thin + Btrfs on loop devices; needs root)
phermesd-storage-it:
    cd phermesd && sudo -E $(command -v cargo) test --test storage_integration -- --ignored --nocapture
```

- [ ] **Step 4: README + CHANGELOG**

README: under the phermesd subsection, note storage & snapshots (provision LVM-thin disks, import local images, QGA-quiesced checkpoints of disk+overlay, auto-snapshot before a switch, one-command rollback). CHANGELOG: an `### Added` line for phermesd storage & snapshots (slice #2).

- [ ] **Step 5: Full suite + commit**

```bash
cd phermesd && cargo test && cargo clippy --all-targets --all-features -- -D warnings
cd /home/u/dev/phermes/phermes
git add phermesd/tests/storage_integration.rs Justfile README.md CHANGELOG.md
git commit -m "feat(phermesd): gated storage integration test, just recipe, docs"
```

---

## Self-Review

**1. Spec coverage:**

| Spec requirement | Task |
|---|---|
| LVM-thin volumes, Proxmox naming, `@phermesd` tag | 2, 3, 7 |
| Btrfs overlay snapshot/restore | 4, 9 |
| QGA quiesce (fsfreeze/thaw), qapi `qga` | 1, 5 |
| Guest-agent virtio-serial channel in argv | 6 |
| Provision + local import + blank + delete; no-half-state | 7 |
| Checkpoint = disk+overlay as a unit; thaw-always; atomic | 8 |
| Pool capacity guard | 8 |
| Retention/prune (auto only) | 9 |
| Rollback (merge + overlay restore), VM-stopped enforced | 9 (storage), 10 (control refuses if active) |
| Control verbs + envelope; phermesctl | 10, 11 |
| Auto-checkpoint before switch; warn-and-continue | 12 |
| System = source of truth (lvs/@snapshots), no state file | 2, 9 |
| Config in defaults / `phermesd.toml` | 7 (`StorageConfig::default`) |
| Errors typed → envelope kinds | 7, 10 |
| Testing: unit argv, mock orchestration, mock-QGA wire, gated loop-device, E2E | 2/4/5, 7-9, 5, 13 |

The full QGA-guest E2E (boot a guest with `qemu-guest-agent` and verify live quiesce) is folded into the gated integration story; an explicit KVM+agent E2E is optional follow-up (the mock-QGA wire test + loop-device integration cover the seams).

**2. Placeholder scan:** the `prune_auto` stub in Task 8 is explicitly replaced in Task 9 (noted in both). The `storage_list` stub in Task 7 is replaced by the `Arc`-handle assertion described in the same task. No `TODO`/`TBD` remain.

**3. Type consistency:** `Lv`, `LvmOps`/`BtrfsOps`/`QgaControl`/`QgaConnector`, `Storage::{new,provision,import,delete,checkpoint,checkpoints,rollback,list_lvs}`, `Checkpoint{vmid,utc,kind}`, `SnapKind{Auto,Manual}`, `StorageConfig`, `StorageError` variants, `RuntimePaths.qga`, `Supervisor::{active_vmid,active_qga_sock,set_storage}`, `CheckpointInfo`, and the `Request` variants are named identically across tasks. `dispatch`/`serve` gain a `storage` parameter consistently in Task 10 and are consumed in Task 12's wiring.
