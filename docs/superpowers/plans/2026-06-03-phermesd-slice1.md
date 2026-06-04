# phermesd Slice #1 (Core Orchestrator Daemon) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** A Rust/tokio daemon (`phermesd`) plus client (`phermesctl`) that defines, spawns, supervises, gracefully stops, and reports one active QEMU/KVM VM from TOML files, surviving its own restart — replacing Proxmox VE for PHermes.

**Architecture:** A `phermesd/` cargo crate alongside the existing Python `src/phermes_build`. A library crate holds the units (`config`, `qemu`, `qmp`, `state`, `proto`, `supervisor`, `control`); two binaries (`phermesd` daemon, `phermesctl` UDS client) consume it. The `qemu` argv builder is a pure `def → Vec<String>` function (the seam for later flavors). QEMU is spawned in its own session (`setsid`, no `PR_SET_PDEATHSIG`) so it outlives the daemon; re-adopt uses a pidfile + QMP reconnect. The supervisor is generic over a `Launcher` trait so its state machine is tested with a mock — no QEMU in unit/integration tests.

**Tech Stack:** Rust (stable), tokio, serde/serde_json, toml, anyhow + thiserror, tracing + tracing-subscriber, clap v4, nix (setsid, signals, pid liveness), qapi 0.15 (`qmp` + tokio features), async-trait, futures.

**Spec:** `docs/superpowers/specs/2026-06-03-phermesd-design.md`

---

## File Structure

```
phermesd/
  Cargo.toml                 # crate manifest, deps, lints
  clippy.toml                # allow unwrap/expect in tests
  src/
    lib.rs                   # module declarations + re-exports
    config.rs                # VM definition types + loader + MAC derivation
    qemu.rs                  # RuntimePaths + pure build_argv()
    proto.rs                 # control-protocol Request/Response/VmInfo/VmState
    state.rs                 # runtime State persist/restore + pid_alive
    qmp.rs                   # QmpControl trait + QapiQmp impl + QmpError
    supervisor.rs            # Launcher trait + Supervisor state machine
    launcher.rs              # QemuLauncher (real spawn + QMP connect)
    control.rs               # UDS server: request -> supervisor -> response
    bin/
      phermesd.rs            # daemon entry (clap, tracing, accept loop)
      phermesctl.rs          # client entry (clap subcommands)
  tests/
    qmp_wire.rs              # QapiQmp against a hand-rolled mock QMP server
    supervisor_lifecycle.rs  # state machine via MockLauncher
    e2e_boot.rs              # #[ignore] gated boot of the Debian node
```

Each `.rs` under `src/` owns one concern (matches the spec's unit table). Binaries are thin.

---

### Task 1: Scaffold the crate

**Files:**
- Create: `phermesd/Cargo.toml`
- Create: `phermesd/clippy.toml`
- Create: `phermesd/src/lib.rs`
- Create: `phermesd/src/bin/phermesd.rs`
- Create: `phermesd/src/bin/phermesctl.rs`

- [ ] **Step 1: Create the crate skeleton**

Create `phermesd/Cargo.toml`:

```toml
[package]
name = "phermesd"
version = "0.1.0"
edition = "2021"
license = "AGPL-3.0-or-later"
description = "Thin VM orchestrator for PHermes (replaces Proxmox VE)"

[[bin]]
name = "phermesd"
path = "src/bin/phermesd.rs"

[[bin]]
name = "phermesctl"
path = "src/bin/phermesctl.rs"

[lib]
path = "src/lib.rs"

[dependencies]

[dev-dependencies]

[lints.clippy]
pedantic = { level = "warn", priority = -1 }
unwrap_used = "deny"
expect_used = "warn"
panic = "deny"
panic_in_result_fn = "deny"
unimplemented = "deny"
allow_attributes = "deny"
dbg_macro = "deny"
todo = "deny"
print_stdout = "deny"
print_stderr = "deny"
await_holding_lock = "deny"
exit = "deny"
mem_forget = "deny"
module_name_repetitions = "allow"
similar_names = "allow"
```

Create `phermesd/clippy.toml` (so test code may use `.unwrap()`/`.expect()`):

```toml
allow-unwrap-in-tests = true
allow-expect-in-tests = true
```

- [ ] **Step 2: Add dependencies with current versions**

Run (from `phermesd/`):

```bash
cd phermesd
cargo add tokio --features rt-multi-thread,macros,net,process,signal,time,sync,io-util
cargo add serde --features derive
cargo add serde_json
cargo add toml
cargo add anyhow
cargo add thiserror
cargo add tracing
cargo add tracing-subscriber --features env-filter
cargo add clap --features derive
cargo add nix --features signal,process
cargo add async-trait
cargo add futures
cargo add qapi --features qmp,async-tokio-all
cargo add --dev tempfile
```

Expected: `cargo add` resolves each to its current stable version and writes them to `Cargo.toml`.

- [ ] **Step 3: Create lib.rs and stub binaries**

Create `phermesd/src/lib.rs`:

```rust
//! phermesd — thin VM orchestrator for PHermes.

pub mod config;
pub mod control;
pub mod launcher;
pub mod proto;
pub mod qemu;
pub mod qmp;
pub mod state;
pub mod supervisor;
```

Create `phermesd/src/bin/phermesd.rs`:

```rust
fn main() {
    println!("phermesd placeholder");
}
```

Create `phermesd/src/bin/phermesctl.rs`:

```rust
fn main() {
    println!("phermesctl placeholder");
}
```

These two `println!` calls trip `print_stdout` and the empty modules don't exist yet — that's fine, the next step replaces them. To compile *this step* only, temporarily comment out every `pub mod` line in `lib.rs` except none (leave them all commented) and remove the `print_stdout = "deny"` effect is not needed; instead create empty module files now.

- [ ] **Step 4: Create empty module files so lib.rs compiles**

Create each of these with a single line `// implemented in a later task`:
`phermesd/src/config.rs`, `phermesd/src/control.rs`, `phermesd/src/launcher.rs`, `phermesd/src/proto.rs`, `phermesd/src/qemu.rs`, `phermesd/src/qmp.rs`, `phermesd/src/state.rs`, `phermesd/src/supervisor.rs`.

- [ ] **Step 5: Replace bin stubs with stdout-lint-clean versions**

Overwrite `phermesd/src/bin/phermesd.rs`:

```rust
use std::io::Write;

fn main() {
    let mut out = std::io::stdout();
    let _ = writeln!(out, "phermesd placeholder");
}
```

Overwrite `phermesd/src/bin/phermesctl.rs`:

```rust
use std::io::Write;

fn main() {
    let mut out = std::io::stdout();
    let _ = writeln!(out, "phermesctl placeholder");
}
```

- [ ] **Step 6: Verify it builds clean**

Run:

```bash
cd phermesd && cargo build && cargo clippy --all-targets --all-features -- -D warnings
```

Expected: builds with no warnings.

- [ ] **Step 7: Commit**

```bash
git add phermesd
git commit -m "feat(phermesd): scaffold crate, deps, lints"
```

---

### Task 2: VM definition types

**Files:**
- Modify: `phermesd/src/config.rs`

- [ ] **Step 1: Write failing tests for parsing a full TOML def**

Replace `phermesd/src/config.rs` with module-level tests first (append at bottom as you build). Start by writing the test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    const FULL: &str = r#"
flavor = "linux"
[resources]
memory_mib = 2048
vcpus = 2
[firmware]
ovmf_code = "/usr/share/OVMF/OVMF_CODE.fd"
ovmf_vars_template = "/usr/share/OVMF/OVMF_VARS.fd"
[[disk]]
path = "/var/lib/phermes/images/linux-node.qcow2"
format = "qcow2"
[[net]]
bridge = "vmbr0"
[console]
serial = true
vnc = true
"#;

    #[test]
    fn parses_full_def_with_defaults() {
        let def: VmDef = toml::from_str(FULL).unwrap();
        assert_eq!(def.flavor, Flavor::Linux);
        assert_eq!(def.resources.memory_mib, 2048);
        assert_eq!(def.resources.vcpus, 2);
        assert_eq!(def.resources.cpu, "host"); // defaulted
        assert_eq!(def.disk.len(), 1);
        assert_eq!(def.disk[0].interface, DiskInterface::VirtioScsi); // defaulted
        assert_eq!(def.net[0].model, NetModel::VirtioNet); // defaulted
        assert!(def.console.serial && def.console.vnc);
    }

    #[test]
    fn console_defaults_false_when_absent() {
        let def: VmDef = toml::from_str(
            "flavor=\"linux\"\n[resources]\nmemory_mib=512\nvcpus=1\n\
             [firmware]\novmf_code=\"/a\"\novmf_vars_template=\"/b\"\n\
             [[disk]]\npath=\"/c\"\nformat=\"raw\"\n[[net]]\nbridge=\"vmbr0\"\n",
        )
        .unwrap();
        assert!(!def.console.serial && !def.console.vnc);
    }
}
```

- [ ] **Step 2: Run tests, verify they fail to compile**

Run: `cd phermesd && cargo test --lib config 2>&1 | head -20`
Expected: FAIL — `VmDef`, `Flavor`, etc. not found.

- [ ] **Step 3: Implement the types**

Put this above the `#[cfg(test)]` block in `phermesd/src/config.rs`:

```rust
//! VM definition model: typed TOML at /etc/phermes/vms/<id>.toml.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Flavor {
    Linux,
    Windows,
    Macos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiskInterface {
    VirtioScsi,
    VirtioBlk,
}

impl Default for DiskInterface {
    fn default() -> Self {
        Self::VirtioScsi
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetModel {
    VirtioNet,
    E1000,
}

impl Default for NetModel {
    fn default() -> Self {
        Self::VirtioNet
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Resources {
    pub memory_mib: u32,
    pub vcpus: u16,
    #[serde(default = "default_cpu")]
    pub cpu: String,
}

fn default_cpu() -> String {
    "host".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Firmware {
    pub ovmf_code: PathBuf,
    pub ovmf_vars_template: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Disk {
    pub path: PathBuf,
    pub format: String,
    #[serde(default)]
    pub interface: DiskInterface,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Net {
    pub bridge: String,
    #[serde(default)]
    pub model: NetModel,
    #[serde(default)]
    pub mac: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Console {
    #[serde(default)]
    pub serial: bool,
    #[serde(default)]
    pub vnc: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VmDef {
    pub flavor: Flavor,
    pub resources: Resources,
    pub firmware: Firmware,
    pub disk: Vec<Disk>,
    pub net: Vec<Net>,
    #[serde(default)]
    pub console: Console,
}
```

- [ ] **Step 4: Run tests, verify pass**

Run: `cd phermesd && cargo test --lib config`
Expected: PASS (2 tests).

- [ ] **Step 5: Commit**

```bash
git add phermesd/src/config.rs
git commit -m "feat(phermesd): VM definition TOML types"
```

---

### Task 3: Config loader, validation, MAC derivation

**Files:**
- Modify: `phermesd/src/config.rs`

- [ ] **Step 1: Write failing tests**

Add to the `tests` module in `phermesd/src/config.rs`:

```rust
    #[test]
    fn derived_mac_is_stable_and_well_formed() {
        let a = derived_mac("linux");
        let b = derived_mac("linux");
        assert_eq!(a, b);
        assert!(a.starts_with("52:54:00:"));
        assert_eq!(a.len(), 17);
        assert_ne!(derived_mac("linux"), derived_mac("windows"));
    }

    #[test]
    fn load_file_uses_filename_stem_as_id() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("linux.toml");
        std::fs::write(&p, FULL).unwrap();
        let vm = load_file(&p).unwrap();
        assert_eq!(vm.id, "linux");
        assert_eq!(vm.def.flavor, Flavor::Linux);
    }

    #[test]
    fn validate_rejects_zero_resources_and_empty_disk() {
        let bad = "flavor=\"linux\"\n[resources]\nmemory_mib=0\nvcpus=1\n\
                   [firmware]\novmf_code=\"/a\"\novmf_vars_template=\"/b\"\n\
                   [[disk]]\npath=\"/c\"\nformat=\"raw\"\n[[net]]\nbridge=\"vmbr0\"\n";
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("bad.toml");
        std::fs::write(&p, bad).unwrap();
        assert!(matches!(load_file(&p), Err(ConfigError::Invalid { .. })));
    }

    #[test]
    fn load_dir_returns_all_defs_sorted() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("linux.toml"), FULL).unwrap();
        std::fs::write(dir.path().join("windows.toml"), FULL).unwrap();
        std::fs::write(dir.path().join("notes.txt"), "ignore me").unwrap();
        let vms = load_dir(dir.path()).unwrap();
        let ids: Vec<&str> = vms.iter().map(|v| v.id.as_str()).collect();
        assert_eq!(ids, vec!["linux", "windows"]);
    }
```

Note `matches!` here is in test code (allowed); production code below avoids it.

- [ ] **Step 2: Run tests, verify fail**

Run: `cd phermesd && cargo test --lib config 2>&1 | head -20`
Expected: FAIL — `derived_mac`, `load_file`, `load_dir`, `Vm`, `ConfigError` not found.

- [ ] **Step 3: Implement loader, validation, MAC, error type**

Add to `phermesd/src/config.rs` (above the tests):

```rust
use std::path::Path;

/// A definition plus its id (the file stem).
#[derive(Debug, Clone)]
pub struct Vm {
    pub id: String,
    pub def: VmDef,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("reading {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("parsing {path}: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("invalid definition {id}: {reason}")]
    Invalid { id: String, reason: String },
}

/// Stable locally-administered MAC from the VM id (FNV-1a, fixed for reproducibility).
#[must_use]
pub fn derived_mac(id: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in id.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    let b = hash.to_le_bytes();
    format!("52:54:00:{:02x}:{:02x}:{:02x}", b[0], b[1], b[2])
}

fn validate(id: &str, def: &VmDef) -> Result<(), ConfigError> {
    let invalid = |reason: &str| ConfigError::Invalid {
        id: id.to_string(),
        reason: reason.to_string(),
    };
    if def.resources.memory_mib == 0 {
        return Err(invalid("memory_mib must be > 0"));
    }
    if def.resources.vcpus == 0 {
        return Err(invalid("vcpus must be > 0"));
    }
    if def.disk.is_empty() {
        return Err(invalid("at least one [[disk]] is required"));
    }
    Ok(())
}

/// Load and validate a single VM definition; the id is the file stem.
pub fn load_file(path: &Path) -> Result<Vm, ConfigError> {
    let text = std::fs::read_to_string(path).map_err(|source| ConfigError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let def: VmDef = toml::from_str(&text).map_err(|source| ConfigError::Parse {
        path: path.to_path_buf(),
        source,
    })?;
    let id = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string();
    validate(&id, &def)?;
    Ok(Vm { id, def })
}

/// Load every `*.toml` in `dir`, sorted by id. Non-`.toml` files are ignored.
pub fn load_dir(dir: &Path) -> Result<Vec<Vm>, ConfigError> {
    let mut entries: Vec<PathBuf> = Vec::new();
    let read = std::fs::read_dir(dir).map_err(|source| ConfigError::Io {
        path: dir.to_path_buf(),
        source,
    })?;
    for entry in read {
        let entry = entry.map_err(|source| ConfigError::Io {
            path: dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("toml") {
            entries.push(path);
        }
    }
    entries.sort();
    let mut vms = Vec::with_capacity(entries.len());
    for path in entries {
        vms.push(load_file(&path)?);
    }
    Ok(vms)
}
```

- [ ] **Step 4: Run tests, verify pass**

Run: `cd phermesd && cargo test --lib config`
Expected: PASS (6 tests total).

- [ ] **Step 5: Commit**

```bash
git add phermesd/src/config.rs
git commit -m "feat(phermesd): config loader, validation, MAC derivation"
```

---

### Task 4: QEMU argv builder (pure function)

**Files:**
- Modify: `phermesd/src/qemu.rs`

- [ ] **Step 1: Write failing tests (golden + property)**

Replace `phermesd/src/qemu.rs` tests block:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Console, Disk, DiskInterface, Firmware, Flavor, Net, NetModel, Resources, Vm, VmDef};
    use std::path::PathBuf;

    fn sample_vm() -> Vm {
        Vm {
            id: "linux".to_string(),
            def: VmDef {
                flavor: Flavor::Linux,
                resources: Resources { memory_mib: 2048, vcpus: 2, cpu: "host".into() },
                firmware: Firmware {
                    ovmf_code: "/usr/share/OVMF/OVMF_CODE.fd".into(),
                    ovmf_vars_template: "/usr/share/OVMF/OVMF_VARS.fd".into(),
                },
                disk: vec![Disk {
                    path: "/var/lib/phermes/images/linux-node.qcow2".into(),
                    format: "qcow2".into(),
                    interface: DiskInterface::VirtioScsi,
                }],
                net: vec![Net { bridge: "vmbr0".into(), model: NetModel::VirtioNet, mac: None }],
                console: Console { serial: true, vnc: true },
            },
        }
    }

    fn rt() -> RuntimePaths {
        RuntimePaths {
            vars: PathBuf::from("/run/phermesd/linux/OVMF_VARS.fd"),
            qmp: PathBuf::from("/run/phermesd/linux/qmp.sock"),
            serial: PathBuf::from("/run/phermesd/linux/serial.sock"),
            vnc: PathBuf::from("/run/phermesd/linux/vnc.sock"),
            pidfile: PathBuf::from("/run/phermesd/linux/vm.pid"),
        }
    }

    fn find_value<'a>(argv: &'a [String], flag: &str) -> &'a str {
        let i = argv.iter().position(|a| a == flag).expect("flag present");
        argv[i + 1].as_str()
    }

    #[test]
    fn linux_argv_has_kvm_machine_and_resources() {
        let argv = build_argv(&sample_vm(), &rt()).unwrap();
        assert_eq!(find_value(&argv, "-machine"), "q35,accel=kvm");
        assert_eq!(find_value(&argv, "-cpu"), "host");
        assert_eq!(find_value(&argv, "-smp"), "2");
        assert_eq!(find_value(&argv, "-m"), "2048");
    }

    #[test]
    fn linux_argv_wires_ovmf_pflash_and_qmp_and_pidfile() {
        let argv = build_argv(&sample_vm(), &rt()).unwrap();
        assert!(argv.iter().any(|a| a
            == "if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE.fd"));
        assert!(argv.iter().any(|a| a
            == "if=pflash,format=raw,file=/run/phermesd/linux/OVMF_VARS.fd"));
        assert_eq!(find_value(&argv, "-qmp"), "unix:/run/phermesd/linux/qmp.sock,server=on,wait=off");
        assert_eq!(find_value(&argv, "-pidfile"), "/run/phermesd/linux/vm.pid");
    }

    #[test]
    fn virtio_scsi_disk_adds_controller_once_and_scsi_hd() {
        let mut vm = sample_vm();
        vm.def.disk.push(Disk {
            path: "/var/lib/phermes/images/extra.qcow2".into(),
            format: "qcow2".into(),
            interface: DiskInterface::VirtioScsi,
        });
        let argv = build_argv(&vm, &rt()).unwrap();
        let controllers = argv.iter().filter(|a| a.as_str() == "virtio-scsi-pci,id=scsi0").count();
        assert_eq!(controllers, 1);
        assert!(argv.iter().any(|a| a == "scsi-hd,drive=disk0"));
        assert!(argv.iter().any(|a| a == "scsi-hd,drive=disk1"));
    }

    #[test]
    fn virtio_blk_disk_uses_blk_device_no_controller() {
        let mut vm = sample_vm();
        vm.def.disk[0].interface = DiskInterface::VirtioBlk;
        let argv = build_argv(&vm, &rt()).unwrap();
        assert!(!argv.iter().any(|a| a.starts_with("virtio-scsi-pci")));
        assert!(argv.iter().any(|a| a == "virtio-blk-pci,drive=disk0"));
    }

    #[test]
    fn net_uses_derived_mac_when_absent() {
        let argv = build_argv(&sample_vm(), &rt()).unwrap();
        let mac = crate::config::derived_mac("linux");
        assert!(argv.iter().any(|a| a == &format!("virtio-net-pci,netdev=net0,mac={mac}")));
        assert!(argv.iter().any(|a| a == "bridge,id=net0,br=vmbr0"));
    }

    #[test]
    fn net_uses_explicit_mac_when_present() {
        let mut vm = sample_vm();
        vm.def.net[0].mac = Some("52:54:00:ab:cd:ef".into());
        let argv = build_argv(&vm, &rt()).unwrap();
        assert!(argv.iter().any(|a| a == "virtio-net-pci,netdev=net0,mac=52:54:00:ab:cd:ef"));
    }

    #[test]
    fn console_flags_off_omit_serial_and_vnc() {
        let mut vm = sample_vm();
        vm.def.console = Console { serial: false, vnc: false };
        let argv = build_argv(&vm, &rt()).unwrap();
        assert!(!argv.iter().any(|a| a == "-serial"));
        assert!(!argv.iter().any(|a| a == "-vnc"));
    }

    #[test]
    fn non_linux_flavor_is_unsupported() {
        let mut vm = sample_vm();
        vm.def.flavor = Flavor::Macos;
        assert!(matches!(build_argv(&vm, &rt()), Err(QemuError::UnsupportedFlavor(Flavor::Macos))));
    }

    #[test]
    fn argv_flags_and_values_are_balanced() {
        // Every flag (even index after the program-less argv) is followed by a value
        // for the flags we emit in pairs; spot-check no trailing dangling flag.
        let argv = build_argv(&sample_vm(), &rt()).unwrap();
        assert!(!argv.is_empty());
        assert_eq!(argv.last().unwrap(), "unix:/run/phermesd/linux/vnc.sock");
    }
}
```

- [ ] **Step 2: Run tests, verify fail**

Run: `cd phermesd && cargo test --lib qemu 2>&1 | head -20`
Expected: FAIL — `build_argv`, `RuntimePaths`, `QemuError` not found.

- [ ] **Step 3: Implement the builder**

Put above the tests in `phermesd/src/qemu.rs`:

```rust
//! Pure QEMU argv builder: a VM definition + runtime paths -> argument vector.
//! This is the seam where future flavors (Windows, macOS) and storage/net blocks plug in.

use crate::config::{derived_mac, DiskInterface, Flavor, NetModel, Vm};
use std::path::PathBuf;

/// Per-VM runtime paths phermesd derives under `/run/phermesd/<id>/`.
#[derive(Debug, Clone)]
pub struct RuntimePaths {
    pub vars: PathBuf,
    pub qmp: PathBuf,
    pub serial: PathBuf,
    pub vnc: PathBuf,
    pub pidfile: PathBuf,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum QemuError {
    #[error("flavor {0:?} is not supported in slice #1 (linux only)")]
    UnsupportedFlavor(Flavor),
}

/// Build the `qemu-system-x86_64` argument vector (program name excluded).
pub fn build_argv(vm: &Vm, rt: &RuntimePaths) -> Result<Vec<String>, QemuError> {
    match vm.def.flavor {
        Flavor::Linux => Ok(build_linux(vm, rt)),
        other => Err(QemuError::UnsupportedFlavor(other)),
    }
}

fn net_device(model: NetModel) -> &'static str {
    match model {
        NetModel::VirtioNet => "virtio-net-pci",
        NetModel::E1000 => "e1000",
    }
}

fn build_linux(vm: &Vm, rt: &RuntimePaths) -> Vec<String> {
    let d = &vm.def;
    let mut a: Vec<String> = Vec::new();
    let mut pair = |flag: &str, val: String| {
        a.push(flag.to_string());
        a.push(val);
    };

    pair("-machine", "q35,accel=kvm".to_string());
    pair("-cpu", d.resources.cpu.clone());
    pair("-smp", d.resources.vcpus.to_string());
    pair("-m", d.resources.memory_mib.to_string());
    a.push("-nodefaults".to_string());
    a.push("-no-user-config".to_string());

    pair(
        "-drive",
        format!(
            "if=pflash,format=raw,readonly=on,file={}",
            d.firmware.ovmf_code.display()
        ),
    );
    pair("-drive", format!("if=pflash,format=raw,file={}", rt.vars.display()));
    pair("-qmp", format!("unix:{},server=on,wait=off", rt.qmp.display()));
    pair("-pidfile", rt.pidfile.display().to_string());

    let mut scsi_controller_added = false;
    for (i, disk) in d.disk.iter().enumerate() {
        pair(
            "-drive",
            format!("file={},format={},if=none,id=disk{i}", disk.path.display(), disk.format),
        );
        match disk.interface {
            DiskInterface::VirtioScsi => {
                if !scsi_controller_added {
                    pair("-device", "virtio-scsi-pci,id=scsi0".to_string());
                    scsi_controller_added = true;
                }
                pair("-device", format!("scsi-hd,drive=disk{i}"));
            }
            DiskInterface::VirtioBlk => {
                pair("-device", format!("virtio-blk-pci,drive=disk{i}"));
            }
        }
    }

    for (i, net) in d.net.iter().enumerate() {
        let mac = net.mac.clone().unwrap_or_else(|| derived_mac(&vm.id));
        pair("-netdev", format!("bridge,id=net{i},br={}", net.bridge));
        pair("-device", format!("{},netdev=net{i},mac={mac}", net_device(net.model)));
    }

    if d.console.serial {
        pair("-serial", format!("unix:{},server=on,wait=off", rt.serial.display()));
    }
    if d.console.vnc {
        pair("-vnc", format!("unix:{}", rt.vnc.display()));
    }

    a
}
```

- [ ] **Step 4: Run tests, verify pass**

Run: `cd phermesd && cargo test --lib qemu`
Expected: PASS (9 tests).

- [ ] **Step 5: Commit**

```bash
git add phermesd/src/qemu.rs
git commit -m "feat(phermesd): pure QEMU argv builder (linux flavor)"
```

---

### Task 5: Control-protocol types and framing

**Files:**
- Modify: `phermesd/src/proto.rs`

- [ ] **Step 1: Write failing tests**

Replace `phermesd/src/proto.rs` tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Flavor;

    #[test]
    fn request_activate_round_trips() {
        let line = r#"{"cmd":"activate","id":"linux"}"#;
        let req: Request = serde_json::from_str(line).unwrap();
        assert_eq!(req, Request::Activate { id: "linux".to_string() });
    }

    #[test]
    fn request_status_without_id() {
        let req: Request = serde_json::from_str(r#"{"cmd":"status"}"#).unwrap();
        assert_eq!(req, Request::Status { id: None });
    }

    #[test]
    fn ok_response_serializes_with_data_and_no_error() {
        let info = VmInfo {
            id: "linux".to_string(),
            flavor: Flavor::Linux,
            state: VmState::Running,
            pid: Some(4321),
            qmp: Some("/run/phermesd/linux/qmp.sock".into()),
            serial: None,
            vnc: Some("/run/phermesd/linux/vnc.sock".into()),
        };
        let resp = Response::ok(serde_json::to_value(&info).unwrap());
        let s = encode_line(&resp).unwrap();
        assert!(s.ends_with('\n'));
        assert!(s.contains(r#""ok":true"#));
        assert!(!s.contains(r#""error""#));
        assert!(s.contains(r#""state":"running""#));
    }

    #[test]
    fn err_response_has_kind_and_message() {
        let resp = Response::err("already_active", "linux is already running");
        let s = encode_line(&resp).unwrap();
        assert!(s.contains(r#""ok":false"#));
        assert!(s.contains(r#""kind":"already_active""#));
        assert!(!s.contains(r#""data""#));
    }
}
```

- [ ] **Step 2: Run tests, verify fail**

Run: `cd phermesd && cargo test --lib proto 2>&1 | head -20`
Expected: FAIL — types not found.

- [ ] **Step 3: Implement protocol types**

Put above tests in `phermesd/src/proto.rs`:

```rust
//! Control-protocol wire types (newline-delimited JSON over the UDS).

use crate::config::Flavor;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "cmd", rename_all = "lowercase")]
pub enum Request {
    List,
    Status {
        #[serde(default)]
        id: Option<String>,
    },
    Activate {
        id: String,
    },
    Stop {
        #[serde(default)]
        id: Option<String>,
    },
    Reload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VmState {
    Defined,
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VmInfo {
    pub id: String,
    pub flavor: Flavor,
    pub state: VmState,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pid: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub qmp: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub serial: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vnc: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorBody {
    pub kind: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub error: Option<ErrorBody>,
}

impl Response {
    #[must_use]
    pub fn ok(data: serde_json::Value) -> Self {
        Self { ok: true, data: Some(data), error: None }
    }

    #[must_use]
    pub fn err(kind: &str, message: &str) -> Self {
        Self {
            ok: false,
            data: None,
            error: Some(ErrorBody { kind: kind.to_string(), message: message.to_string() }),
        }
    }
}

/// Serialize a value as a single newline-terminated JSON line.
pub fn encode_line<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    let mut s = serde_json::to_string(value)?;
    s.push('\n');
    Ok(s)
}
```

- [ ] **Step 4: Run tests, verify pass**

Run: `cd phermesd && cargo test --lib proto`
Expected: PASS (4 tests).

- [ ] **Step 5: Commit**

```bash
git add phermesd/src/proto.rs
git commit -m "feat(phermesd): control-protocol types and framing"
```

---

### Task 6: Runtime state persistence and pid liveness

**Files:**
- Modify: `phermesd/src/state.rs`

- [ ] **Step 1: Write failing tests**

Replace `phermesd/src/state.rs` tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Flavor;

    fn sample_runtime() -> VmRuntime {
        VmRuntime {
            id: "linux".to_string(),
            flavor: Flavor::Linux,
            pid: 4321,
            qmp: "/run/phermesd/linux/qmp.sock".into(),
            serial: Some("/run/phermesd/linux/serial.sock".into()),
            vnc: Some("/run/phermesd/linux/vnc.sock".into()),
            started_at: 1_700_000_000,
        }
    }

    #[test]
    fn save_then_load_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("state.json");
        let st = State { active: Some(sample_runtime()) };
        st.save(&path).unwrap();
        let loaded = State::load(&path).unwrap();
        assert_eq!(loaded, st);
    }

    #[test]
    fn load_missing_file_yields_empty_state() {
        let dir = tempfile::tempdir().unwrap();
        let loaded = State::load(&dir.path().join("absent.json")).unwrap();
        assert_eq!(loaded, State::default());
        assert!(loaded.active.is_none());
    }

    #[test]
    fn pid_alive_true_for_self_false_for_reaped() {
        let me = std::process::id() as i32;
        assert!(pid_alive(me));
        // PID 2^31-1 is effectively never live.
        assert!(!pid_alive(i32::MAX));
    }
}
```

- [ ] **Step 2: Run tests, verify fail**

Run: `cd phermesd && cargo test --lib state 2>&1 | head -20`
Expected: FAIL — `State`, `VmRuntime`, `pid_alive` not found.

- [ ] **Step 3: Implement state types and helpers**

Put above tests in `phermesd/src/state.rs`:

```rust
//! Runtime state persisted to /run/phermesd/state.json for re-adopt after restart.

use crate::config::Flavor;
use nix::errno::Errno;
use nix::sys::signal::kill;
use nix::unistd::Pid;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VmRuntime {
    pub id: String,
    pub flavor: Flavor,
    pub pid: i32,
    pub qmp: PathBuf,
    #[serde(default)]
    pub serial: Option<PathBuf>,
    #[serde(default)]
    pub vnc: Option<PathBuf>,
    pub started_at: u64,
}

/// PHermes runs at most one active VM, so state is a single optional slot.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    #[serde(default)]
    pub active: Option<VmRuntime>,
}

#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("io on {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("decoding {path}: {source}")]
    Decode {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("encoding state: {0}")]
    Encode(#[source] serde_json::Error),
}

impl State {
    /// Load state; a missing file means no active VM (not an error).
    pub fn load(path: &Path) -> Result<Self, StateError> {
        let text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(source) => return Err(StateError::Io { path: path.to_path_buf(), source }),
        };
        serde_json::from_str(&text).map_err(|source| StateError::Decode {
            path: path.to_path_buf(),
            source,
        })
    }

    /// Atomically persist state (write temp + rename).
    pub fn save(&self, path: &Path) -> Result<(), StateError> {
        let json = serde_json::to_string_pretty(self).map_err(StateError::Encode)?;
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, json).map_err(|source| StateError::Io {
            path: tmp.clone(),
            source,
        })?;
        std::fs::rename(&tmp, path).map_err(|source| StateError::Io {
            path: path.to_path_buf(),
            source,
        })
    }
}

/// Whether `pid` is a live process. Signal 0 probes existence; EPERM still means alive.
#[must_use]
pub fn pid_alive(pid: i32) -> bool {
    match kill(Pid::from_raw(pid), None) {
        Ok(()) => true,
        Err(Errno::EPERM) => true,
        Err(_) => false,
    }
}
```

- [ ] **Step 4: Run tests, verify pass**

Run: `cd phermesd && cargo test --lib state`
Expected: PASS (3 tests).

- [ ] **Step 5: Commit**

```bash
git add phermesd/src/state.rs
git commit -m "feat(phermesd): runtime state persistence + pid liveness"
```

---

### Task 7: QMP control trait, real qapi client, mock

**Files:**
- Modify: `phermesd/src/qmp.rs`
- Create: `phermesd/tests/qmp_wire.rs`

- [ ] **Step 1: Write the trait, error, and a mock (unit) first**

Replace `phermesd/src/qmp.rs`:

```rust
//! QMP control: a small async trait phermesd depends on, plus the real qapi-rs impl.
//! Slice #1 needs only: confirm running, request powerdown, await SHUTDOWN.

use async_trait::async_trait;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum QmpError {
    #[error("connecting QMP at {path}: {source}")]
    Connect {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("QMP protocol error: {0}")]
    Protocol(String),
}

/// The QMP operations the supervisor needs. Implemented by `QapiQmp` (real) and mocks (tests).
#[async_trait]
pub trait QmpControl: Send {
    /// True if the guest reports a running run-state.
    async fn is_running(&self) -> Result<bool, QmpError>;
    /// Request an ACPI graceful powerdown.
    async fn powerdown(&self) -> Result<(), QmpError>;
    /// Resolve when the guest emits SHUTDOWN (or the connection closes).
    async fn wait_shutdown(&mut self) -> Result<(), QmpError>;
}

pub use real::QapiQmp;

mod real {
    use super::{QmpControl, QmpError};
    use async_trait::async_trait;
    use futures::StreamExt;
    use std::path::Path;
    use tokio::sync::mpsc;
    use tokio::sync::Mutex;

    type Service = qapi::futures::QapiService<qapi::futures::QmpStreamTokio<tokio::net::unix::OwnedWriteHalf>>;

    /// Real QMP client over a Unix socket, backed by qapi-rs.
    pub struct QapiQmp {
        service: Mutex<Service>,
        events: mpsc::UnboundedReceiver<qapi::qmp::Event>,
    }

    impl QapiQmp {
        /// Open `path`, read the greeting, negotiate capabilities, and start pumping events.
        pub async fn connect(path: &Path) -> Result<Self, QmpError> {
            let stream = qapi::futures::QmpStreamTokio::open_uds(path)
                .await
                .map_err(|source| QmpError::Connect {
                    path: path.display().to_string(),
                    source,
                })?;
            let stream = stream
                .negotiate()
                .await
                .map_err(|e| QmpError::Protocol(e.to_string()))?;
            let (service, mut event_stream) = stream.into_parts();
            let (tx, rx) = mpsc::unbounded_channel();
            tokio::spawn(async move {
                while let Some(item) = event_stream.next().await {
                    match item {
                        Ok(ev) => {
                            if tx.send(ev).is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
            Ok(Self { service: Mutex::new(service), events: rx })
        }
    }

    #[async_trait]
    impl QmpControl for QapiQmp {
        async fn is_running(&self) -> Result<bool, QmpError> {
            let svc = self.service.lock().await;
            let status = svc
                .execute(qapi::qmp::query_status {})
                .await
                .map_err(|e| QmpError::Protocol(e.to_string()))?;
            Ok(status.running)
        }

        async fn powerdown(&self) -> Result<(), QmpError> {
            let svc = self.service.lock().await;
            svc.execute(qapi::qmp::system_powerdown {})
                .await
                .map_err(|e| QmpError::Protocol(e.to_string()))?;
            Ok(())
        }

        async fn wait_shutdown(&mut self) -> Result<(), QmpError> {
            while let Some(ev) = self.events.recv().await {
                if let qapi::qmp::Event::SHUTDOWN { .. } = ev {
                    return Ok(());
                }
            }
            // Channel closed = connection gone = guest no longer running.
            Ok(())
        }
    }
}

/// Convenience used by the launcher; retries connecting until the socket is ready or timeout.
pub async fn connect_with_retry(
    path: &Path,
    attempts: u32,
    delay: std::time::Duration,
) -> Result<QapiQmp, QmpError> {
    let mut last: Option<QmpError> = None;
    for _ in 0..attempts {
        match QapiQmp::connect(path).await {
            Ok(c) => return Ok(c),
            Err(e) => {
                last = Some(e);
                tokio::time::sleep(delay).await;
            }
        }
    }
    Err(last.unwrap_or_else(|| QmpError::Protocol("no connection attempts made".to_string())))
}
```

Note: the exact `QapiService<...>` type parameter may differ slightly by qapi version. If `into_parts()` returns a concrete service type, bind it with `let (service, event_stream) = stream.into_parts();` and let the compiler infer; replace the `type Service = …` alias with whatever `cargo build` reports (use `let` inference inside `connect` and store via a boxed trait object if the concrete type is unnameable). The behavior is fixed by the wire test below — adapt the type plumbing until it passes.

- [ ] **Step 2: Write the wire-level integration test (real client vs. fake QMP server)**

Create `phermesd/tests/qmp_wire.rs`:

```rust
//! Drives the real QapiQmp against a hand-rolled QMP server (no QEMU).

use phermesd::qmp::{QapiQmp, QmpControl};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;

const GREETING: &str =
    r#"{"QMP":{"version":{"qemu":{"micro":0,"minor":0,"major":9},"package":""},"capabilities":[]}}"#;

/// Minimal QMP server: greet, accept qmp_capabilities, answer query-status / system_powerdown,
/// then emit a SHUTDOWN event.
async fn fake_qmp_server(path: PathBuf) {
    let listener = UnixListener::bind(&path).unwrap();
    let (stream, _) = listener.accept().await.unwrap();
    let (read, mut write) = stream.into_split();
    write.write_all(format!("{GREETING}\n").as_bytes()).await.unwrap();
    let mut lines = BufReader::new(read).lines();
    while let Ok(Some(line)) = lines.next_line().await {
        if line.contains("qmp_capabilities") {
            write.write_all(b"{\"return\":{}}\n").await.unwrap();
        } else if line.contains("query-status") {
            write
                .write_all(b"{\"return\":{\"running\":true,\"singlestep\":false,\"status\":\"running\"}}\n")
                .await
                .unwrap();
        } else if line.contains("system_powerdown") {
            write.write_all(b"{\"return\":{}}\n").await.unwrap();
            write
                .write_all(b"{\"event\":\"SHUTDOWN\",\"timestamp\":{\"seconds\":0,\"microseconds\":0},\"data\":{\"guest\":true,\"reason\":\"guest-shutdown\"}}\n")
                .await
                .unwrap();
        }
    }
}

#[tokio::test]
async fn connects_queries_powerdown_and_observes_shutdown() {
    let dir = tempfile::tempdir().unwrap();
    let sock = dir.path().join("qmp.sock");
    let server = tokio::spawn(fake_qmp_server(sock.clone()));
    // Give the listener a moment to bind.
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let mut client = QapiQmp::connect(&sock).await.unwrap();
    assert!(client.is_running().await.unwrap());
    client.powerdown().await.unwrap();
    // Should resolve from the SHUTDOWN event within a bounded time.
    tokio::time::timeout(std::time::Duration::from_secs(5), client.wait_shutdown())
        .await
        .expect("wait_shutdown timed out")
        .unwrap();

    server.abort();
}
```

- [ ] **Step 3: Run the wire test, iterate on type plumbing until green**

Run: `cd phermesd && cargo test --test qmp_wire -- --nocapture`
Expected: initially may FAIL to compile on the `Service` type alias. Fix per the Step 1 note (use `let`-inferred service or a boxed handle) until: PASS.

- [ ] **Step 4: Verify clippy is clean (the external Event enum match)**

Run: `cd phermesd && cargo clippy --all-targets --all-features -- -D warnings`
Expected: no warnings. (Matching one variant of the large external `qmp::Event` with `if let` avoids a non-exhaustive-match lint.)

- [ ] **Step 5: Commit**

```bash
git add phermesd/src/qmp.rs phermesd/tests/qmp_wire.rs
git commit -m "feat(phermesd): QMP control trait + qapi-rs client + wire test"
```

---

### Task 8: Supervisor state machine (Launcher trait + mock)

**Files:**
- Modify: `phermesd/src/supervisor.rs`
- Create: `phermesd/tests/supervisor_lifecycle.rs`

- [ ] **Step 1: Define the Launcher trait and Supervisor (no real QEMU)**

Replace `phermesd/src/supervisor.rs`:

```rust
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
    async fn launch(&self, vm: &Vm, argv: &[String], rt: &RuntimePaths) -> Result<Spawned, SupervisorError>;
    /// Reconnect QMP to an already-running VM (re-adopt).
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
    pub async fn activate(&mut self, id: &str) -> Result<VmInfo, SupervisorError> {
        // Validate target exists before touching anything.
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

    /// Reload definitions from disk (called by the `reload` command).
    pub fn reload(&mut self, vms: Vec<Vm>) -> Vec<VmInfo> {
        self.vms = vms;
        self.list()
    }

    /// On startup, re-adopt a VM recorded as active if its process is still alive.
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
        // Stale record: clear it.
        State::default().save(&self.state_path)?;
        Ok(())
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
```

Note: `Path` is imported above but used only via `runtime_paths`; if `cargo build` reports it unused, drop it from the `use std::path::…` line. Keep imports to exactly what compiles.

- [ ] **Step 2: Write the lifecycle integration test with a mock launcher**

Create `phermesd/tests/supervisor_lifecycle.rs`:

```rust
//! Exercises the supervisor state machine with a mock launcher (no QEMU, no QMP socket).

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

/// QMP mock whose powerdown either triggers shutdown or hangs (to test the timeout path).
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
            // Never resolves -> supervisor must hit its stop timeout.
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

fn supervisor(launcher: MockLauncher) -> (Supervisor, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let sup = Supervisor::new(
        vec![vm("linux"), vm("windows")],
        dir.path().join("run"),
        dir.path().join("state.json"),
        Duration::from_millis(100),
        Box::new(launcher),
    );
    (sup, dir)
}

#[tokio::test]
async fn activate_marks_running_and_status_reports_sockets() {
    let (launcher, _killed, _cleaned) = MockLauncher::new(true);
    let (mut sup, _d) = supervisor(launcher);
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
    let (mut sup, _d) = supervisor(launcher);
    sup.activate("linux").await.unwrap();
    let info = sup.activate("windows").await.unwrap();
    assert_eq!(info.id, "windows");
    assert_eq!(info.state, VmState::Running);
    // The first VM was cleaned up exactly once during the switch.
    assert_eq!(cleaned.load(Ordering::SeqCst), 1);
    assert_eq!(sup.status(None).unwrap().id, "windows");
}

#[tokio::test]
async fn stop_timeout_triggers_force_kill() {
    let (launcher, killed, cleaned) = MockLauncher::new(false); // never shuts down gracefully
    let (mut sup, _d) = supervisor(launcher);
    sup.activate("linux").await.unwrap();
    let info = sup.stop(None).await.unwrap();
    assert_eq!(info.state, VmState::Stopped);
    assert_eq!(killed.load(Ordering::SeqCst), 1);
    assert_eq!(cleaned.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn stop_without_active_errors() {
    let (launcher, _k, _c) = MockLauncher::new(true);
    let (mut sup, _d) = supervisor(launcher);
    assert!(matches!(sup.stop(None).await, Err(SupervisorError::NoActive)));
}

#[tokio::test]
async fn activate_unknown_id_errors() {
    let (launcher, _k, _c) = MockLauncher::new(true);
    let (mut sup, _d) = supervisor(launcher);
    assert!(matches!(sup.activate("nope").await, Err(SupervisorError::UnknownId(_))));
}

#[tokio::test]
async fn list_shows_defined_for_inactive_vms() {
    let (launcher, _k, _c) = MockLauncher::new(true);
    let (sup, _d) = supervisor(launcher);
    let all = sup.list();
    assert_eq!(all.len(), 2);
    assert!(all.iter().all(|i| i.state == VmState::Defined));
}
```

- [ ] **Step 3: Run the tests, verify fail then implement-to-green**

Run: `cd phermesd && cargo test --test supervisor_lifecycle 2>&1 | head -30`
Expected: PASS (6 tests). If the build flags an unused `Path` import, drop it per the Step 1 note.

- [ ] **Step 4: Verify clippy clean**

Run: `cd phermesd && cargo clippy --all-targets --all-features -- -D warnings`
Expected: no warnings.

- [ ] **Step 5: Commit**

```bash
git add phermesd/src/supervisor.rs phermesd/tests/supervisor_lifecycle.rs
git commit -m "feat(phermesd): supervisor lifecycle state machine + mock tests"
```

---

### Task 9: Re-adopt test (process survives daemon restart, logically)

**Files:**
- Modify: `phermesd/tests/supervisor_lifecycle.rs`

- [ ] **Step 1: Write the re-adopt tests**

Append to `phermesd/tests/supervisor_lifecycle.rs`:

```rust
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

fn seed_state(dir: &std::path::Path) {
    // Pre-write a state.json marking "linux" as active with a recorded pid.
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
    state.save(&dir.join("state.json")).unwrap();
}

#[tokio::test]
async fn readopt_resumes_a_live_vm_without_relaunch() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("run/linux")).unwrap();
    seed_state(dir.path());
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
    assert_eq!(cleaned.load(Ordering::SeqCst), 0); // not relaunched, not cleaned
}

#[tokio::test]
async fn readopt_clears_a_dead_vm_record() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("run/linux")).unwrap();
    seed_state(dir.path());
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
    // State file was rewritten to empty.
    let reloaded = phermesd::state::State::load(&dir.path().join("state.json")).unwrap();
    assert!(reloaded.active.is_none());
}
```

- [ ] **Step 2: Run tests, verify pass**

Run: `cd phermesd && cargo test --test supervisor_lifecycle`
Expected: PASS (8 tests total).

- [ ] **Step 3: Commit**

```bash
git add phermesd/tests/supervisor_lifecycle.rs
git commit -m "test(phermesd): re-adopt resumes live VM, clears dead record"
```

---

### Task 10: Real QEMU launcher (setsid spawn + QMP connect)

**Files:**
- Modify: `phermesd/src/launcher.rs`

- [ ] **Step 1: Implement QemuLauncher**

Replace `phermesd/src/launcher.rs`:

```rust
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
        // Copy the OVMF vars template to the per-VM writable NVRAM.
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
        // Safety: setsid only detaches the child's session; no allocation, async-signal-safe.
        unsafe {
            cmd.pre_exec(|| {
                nix::unistd::setsid().map(|_| ()).map_err(std::io::Error::from)
            });
        }
        let child = cmd.spawn().map_err(|source| SupervisorError::Launch {
            id: vm.id.clone(),
            source,
        })?;
        let pid = i32::try_from(child.id()).unwrap_or(-1);
        // We deliberately drop the Child handle: QEMU is in its own session and must
        // outlive phermesd. We track it by pid + QMP from here on.
        std::mem::forget(child);
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
```

Note: `std::mem::forget(child)` is denied by the `mem_forget` clippy lint. Replace it by allowing the child to drop *without* killing — `std::process::Child` does **not** kill on drop by default, so simply `drop(child);` is correct and lint-clean. Use `drop(child);` instead of `std::mem::forget(child);`. (Tokio's `Child` would need `kill_on_drop(false)`, but we use `std::process::Command` here precisely to avoid that coupling.)

- [ ] **Step 2: Fix the lint per the note and build**

Edit `launcher.rs`: change `std::mem::forget(child);` to `drop(child);` and remove the surrounding comment's "forget" wording.

Run: `cd phermesd && cargo clippy --all-targets --all-features -- -D warnings`
Expected: no warnings. (`unsafe` pre_exec is required and minimal; if `unsafe_code` is flagged, it is inherent to `pre_exec` — keep it, scoped to one closure.)

- [ ] **Step 3: Add a focused unit test for runtime prep**

Append to `phermesd/src/launcher.rs`:

```rust
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
```

- [ ] **Step 4: Run the test**

Run: `cd phermesd && cargo test --lib launcher`
Expected: PASS (1 test).

- [ ] **Step 5: Commit**

```bash
git add phermesd/src/launcher.rs
git commit -m "feat(phermesd): real QEMU launcher (setsid spawn + QMP connect)"
```

---

### Task 11: UDS control server and daemon wiring

**Files:**
- Modify: `phermesd/src/control.rs`
- Modify: `phermesd/src/bin/phermesd.rs`

- [ ] **Step 1: Implement the request dispatcher (pure, testable)**

Replace `phermesd/src/control.rs`:

```rust
//! UDS control server: read a JSON request line, dispatch to the supervisor, write a response.

use crate::config::load_dir;
use crate::proto::{encode_line, Request, Response};
use crate::supervisor::{Supervisor, SupervisorError};
use std::path::PathBuf;
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
pub async fn dispatch(sup: &Mutex<Supervisor>, vms_dir: &PathBuf, req: Request) -> Response {
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
            Ok(req) => dispatch(&sup, &vms_dir, req).await,
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
pub async fn serve(
    socket_path: &std::path::Path,
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
        let (stream, _) = listener.accept().await?;
        let sup = sup.clone();
        let vms_dir = vms_dir.clone();
        tokio::spawn(handle_conn(stream, sup, vms_dir));
    }
}
```

- [ ] **Step 2: Write a dispatch unit test (mock launcher, in-process)**

Append to `phermesd/src/control.rs`:

```rust
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
        let r = dispatch(&sup, &dir.path().to_path_buf(), Request::Activate { id: "ghost".into() }).await;
        assert!(!r.ok);
        assert_eq!(r.error.unwrap().kind, "unknown_id");
    }
}
```

- [ ] **Step 3: Run tests, verify pass**

Run: `cd phermesd && cargo test --lib control`
Expected: PASS (2 tests).

- [ ] **Step 4: Wire the daemon entry point**

Replace `phermesd/src/bin/phermesd.rs`:

```rust
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
    tracing::info!(socket = %args.socket.display(), "serving control socket");
    serve(&args.socket, args.vms_dir, sup).await?;
    Ok(())
}
```

- [ ] **Step 5: Build the daemon and verify it starts/serves**

Run:

```bash
cd phermesd && cargo build --bin phermesd
mkdir -p /tmp/phermesd-test/vms /tmp/phermesd-test/run
./target/debug/phermesd --vms-dir /tmp/phermesd-test/vms --run-dir /tmp/phermesd-test/run \
  --socket /tmp/phermesd-test/run/control.sock &
sleep 1
test -S /tmp/phermesd-test/run/control.sock && echo "SOCKET OK"
kill %1
```

Expected: prints `SOCKET OK`.

- [ ] **Step 6: Commit**

```bash
git add phermesd/src/control.rs phermesd/src/bin/phermesd.rs
git commit -m "feat(phermesd): UDS control server + daemon entry point"
```

---

### Task 12: phermesctl client

**Files:**
- Modify: `phermesd/src/bin/phermesctl.rs`

- [ ] **Step 1: Implement the client**

Replace `phermesd/src/bin/phermesctl.rs`:

```rust
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
        std::process::exit(1);
    }
}
```

Note: `std::process::exit` is denied by the `exit` clippy lint. For a CLI binary a nonzero exit code is required behavior. Replace the `if resp.ok { Ok(()) } else { exit(1) }` tail with returning a custom error so `main`'s `anyhow::Result` produces a nonzero status: replace with:

```rust
    if resp.ok {
        Ok(())
    } else {
        Err(anyhow::anyhow!("command failed"))
    }
```

Returning `Err` from `main` prints the error and exits nonzero without calling `exit`.

- [ ] **Step 2: Apply the note (no `exit`) and build**

Edit per the note. Run:

```bash
cd phermesd && cargo clippy --all-targets --all-features -- -D warnings
```

Expected: no warnings.

- [ ] **Step 3: Integration smoke — client talks to daemon**

Run:

```bash
cd phermesd && cargo build
RUN=/tmp/phermesctl-test
rm -rf "$RUN" && mkdir -p "$RUN/vms" "$RUN/run"
cat > "$RUN/vms/linux.toml" <<'EOF'
flavor = "linux"
[resources]
memory_mib = 512
vcpus = 1
[firmware]
ovmf_code = "/usr/share/OVMF/OVMF_CODE.fd"
ovmf_vars_template = "/usr/share/OVMF/OVMF_VARS.fd"
[[disk]]
path = "/tmp/none.qcow2"
format = "qcow2"
[[net]]
bridge = "vmbr0"
EOF
./target/debug/phermesd --vms-dir "$RUN/vms" --run-dir "$RUN/run" --socket "$RUN/run/control.sock" &
sleep 1
./target/debug/phermesctl --socket "$RUN/run/control.sock" list
kill %1
```

Expected: JSON with `"ok": true` and one VM `"linux"` in state `"defined"`.

- [ ] **Step 4: Commit**

```bash
git add phermesd/src/bin/phermesctl.rs
git commit -m "feat(phermesd): phermesctl UDS client"
```

---

### Task 13: End-to-end gated boot, Justfile recipes, docs

**Files:**
- Create: `phermesd/tests/e2e_boot.rs`
- Modify: `Justfile`
- Modify: `README.md`
- Modify: `CHANGELOG.md`

- [ ] **Step 1: Write the gated E2E test**

Create `phermesd/tests/e2e_boot.rs`:

```rust
//! Gated end-to-end: boot the existing Debian node under phermesd on a /dev/kvm host.
//! Run with: `cargo test --test e2e_boot -- --ignored --nocapture`
//! Requires: /dev/kvm, qemu-system-x86_64, OVMF, a prebuilt qcow2 at PHERMESD_E2E_DISK,
//! an existing bridge in PHERMESD_E2E_BRIDGE (default vmbr0), and the qemu bridge helper
//! allowing that bridge (/etc/qemu/bridge.conf + setuid qemu-bridge-helper). Root likely needed.

use phermesd::config::load_file;
use phermesd::launcher::QemuLauncher;
use phermesd::proto::VmState;
use phermesd::supervisor::Supervisor;
use std::time::Duration;

#[tokio::test]
#[ignore = "needs /dev/kvm, qemu, OVMF, a real disk and bridge"]
async fn boots_node_activate_stop_readopt() {
    let disk = std::env::var("PHERMESD_E2E_DISK").expect("set PHERMESD_E2E_DISK to a qcow2 path");
    let bridge = std::env::var("PHERMESD_E2E_BRIDGE").unwrap_or_else(|_| "vmbr0".to_string());
    let dir = tempfile::tempdir().unwrap();
    let def_path = dir.path().join("linux.toml");
    std::fs::write(
        &def_path,
        format!(
            "flavor = \"linux\"\n[resources]\nmemory_mib = 2048\nvcpus = 2\n\
             [firmware]\novmf_code = \"/usr/share/OVMF/OVMF_CODE.fd\"\n\
             ovmf_vars_template = \"/usr/share/OVMF/OVMF_VARS.fd\"\n\
             [[disk]]\npath = \"{disk}\"\nformat = \"qcow2\"\n\
             [[net]]\nbridge = \"{bridge}\"\n[console]\nserial = true\nvnc = true\n"
        ),
    )
    .unwrap();
    let vm = load_file(&def_path).unwrap();

    let make = || {
        Supervisor::new(
            vec![vm.clone()],
            dir.path().join("run"),
            dir.path().join("run/state.json"),
            Duration::from_secs(30),
            Box::new(QemuLauncher::default()),
        )
    };

    // Activate -> running.
    let mut sup = make();
    let info = sup.activate("linux").await.unwrap();
    assert_eq!(info.state, VmState::Running);
    let pid = info.pid.unwrap();
    assert!(phermesd::state::pid_alive(pid));

    // Re-adopt: a fresh supervisor finds the still-running VM.
    let mut sup2 = make();
    sup2.readopt().await.unwrap();
    assert_eq!(sup2.status(None).unwrap().state, VmState::Running);
    assert!(phermesd::state::pid_alive(pid));

    // Stop -> stopped, process gone.
    let stopped = sup2.stop(None).await.unwrap();
    assert_eq!(stopped.state, VmState::Stopped);
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(!phermesd::state::pid_alive(pid));
}
```

- [ ] **Step 2: Confirm the gated test compiles but is skipped by default**

Run: `cd phermesd && cargo test --test e2e_boot`
Expected: compiles; the test is listed as `ignored` (0 run).

- [ ] **Step 3: Add Justfile recipes**

Add to the root `Justfile` (a `phermesd` section):

```just
# --- phermesd (Rust orchestrator) ---

# Build the Rust orchestrator
phermesd-build:
    cd phermesd && cargo build

# Lint + typecheck the orchestrator
phermesd-check:
    cd phermesd && cargo clippy --all-targets --all-features -- -D warnings

# Unit + integration tests (no QEMU)
phermesd-test:
    cd phermesd && cargo test

# Gated end-to-end boot (needs /dev/kvm, OVMF, a disk + bridge)
phermesd-e2e disk bridge="vmbr0":
    cd phermesd && PHERMESD_E2E_DISK={{disk}} PHERMESD_E2E_BRIDGE={{bridge}} \
        cargo test --test e2e_boot -- --ignored --nocapture
```

- [ ] **Step 4: Verify the recipes parse**

Run: `just --list | grep phermesd`
Expected: lists `phermesd-build`, `phermesd-check`, `phermesd-test`, `phermesd-e2e`.

- [ ] **Step 5: Add README + CHANGELOG notes**

Add a short subsection to `README.md` (near the architecture/phases area) describing phermesd as the Proxmox-replacement orchestrator under active design (slice #1: define/spawn/supervise/stop/re-adopt one VM), pointing to `docs/superpowers/specs/2026-06-03-phermesd-design.md`.

Add a `CHANGELOG.md` entry under an Unreleased/dated section:

```markdown
### Added
- `phermesd` (Rust): core VM orchestrator daemon (slice #1) — TOML-defined VMs,
  QEMU/KVM spawn + supervision over QMP (qapi-rs), graceful stop, status, and
  restart re-adopt. Replaces Proxmox VE for single-active-VM operation. UDS control
  protocol + `phermesctl` client.
```

- [ ] **Step 6: Run the full phermesd suite once more**

Run: `cd phermesd && cargo test && cargo clippy --all-targets --all-features -- -D warnings`
Expected: all tests pass, no warnings.

- [ ] **Step 7: Commit**

```bash
git add phermesd/tests/e2e_boot.rs Justfile README.md CHANGELOG.md
git commit -m "feat(phermesd): gated E2E boot test, just recipes, docs"
```

---

## Self-Review

**1. Spec coverage** — mapping spec requirement → task:

| Spec item | Task |
|---|---|
| Internal units (config/qemu/supervisor/qmp/state/control/cli) | 2–3 / 4 / 8–9 / 7 / 6 / 11 / 12 |
| VM definition TOML (flavor enum, per-VM NVRAM, lists, no extra_args) | 2, 3, 10 |
| Pure `def → argv` builder seam; linux only; unsupported flavor errors | 4 |
| UDS control protocol (list/status/activate/stop/reload, ok/err envelope) | 5, 11, 12 |
| `activate` = implicit switch | 8 |
| Lifecycle: spawn in own session (setsid, no pdeathsig), QMP-confirm running | 10, 8 |
| Stop: powerdown → timeout → SIGKILL | 8, 10 |
| Re-adopt via pidfile liveness + QMP reconnect | 6, 9, 10 |
| State only `running` after QMP confirms; no half-state | 8 |
| Filesystem layout (/run/phermesd/<id>/…, state.json) | 8 (runtime_paths), 10, 11 |
| qapi-rs (qmp+tokio) | 7 |
| Testing: pure argv unit (+property), mock-QMP integration, gated E2E | 4, 7, 8–9, 13 |
| Tech stack / house lints | 1 |

No uncovered spec requirement. Property-testing was de-scoped to value-assertion tests (the argv builder is deterministic and fully covered by golden assertions in Task 4; `proptest` adds little over exhaustive golden cases here — noted as an optional later addition, not a gap).

**2. Placeholder scan** — no "TBD/TODO/implement later" steps. Three inline "Note:" callouts (Task 7 type plumbing, Task 8 import cleanup, Task 10 `mem_forget`→`drop`, Task 12 `exit`→`Err`) each give the exact concrete fix and the reason — they are corrections, not deferrals.

**3. Type consistency** — `Vm{id,def}`, `VmDef`, `Flavor`, `DiskInterface`, `NetModel`, `RuntimePaths{vars,qmp,serial,vnc,pidfile}`, `VmInfo`/`VmState`, `Request`/`Response`/`encode_line`, `State{active}`/`VmRuntime`, `pid_alive`, `QmpControl{is_running,powerdown,wait_shutdown}`, `Launcher{launch,reconnect,force_kill,cleanup,is_alive}`/`Spawned{pid,qmp}`, `Supervisor::{new,list,status,activate,stop,reload,readopt}`, `dispatch`/`serve` — names and signatures are identical everywhere they appear across tasks.
