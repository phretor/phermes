# phermesd macOS guest support (slice #5b) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `phermesctl activate macos` boot a BYOI macOS guest under phermesd, with the Apple OSK read at launch from a root-owned secret file.

**Architecture:** Rust adds `secrets.rs` (typed Apple OSK file reader) and a `build_macos` argv builder in `qemu.rs` with a `build_macos_with_secrets` seam. `phermes-build` extends `_vm_source` to accept `macos=<path>`, adds `vm.write_macos_def` + `vm.provision_macos_disk`, and a `--dev-apple-osk` flag that drops the OSK into `/etc/phermes/secrets/apple.osk` (mode 0400) in dev builds.

**Tech Stack:** Rust (tokio, nix, qapi-rs); Python 3.13 + uv + ruff + ty + pytest.

**Spec:** `docs/superpowers/specs/2026-06-15-phermesd-macos-guest-design.md`.

**Branch:** `feat/phermesd-macos-guest`, stacked on `feat/phermesd-windows-guest` (PR #25). Push as `feat/phermesd-macos-guest:feat/phermesd-macos-guest` with explicit local:remote form. PR base = `feat/phermesd-windows-guest`.

---

## File structure

| File | Action | Responsibility |
|---|---|---|
| `phermesd/src/secrets.rs` | Create | `SecretsError` + `read_apple_osk(path)` — file existence, mode 0400, 64-char hex validation |
| `phermesd/src/lib.rs` (or main.rs) | Modify | `mod secrets;` registration |
| `phermesd/src/qemu.rs` | Modify | New `build_macos` (pure) + `build_macos_with_secrets` (seam) + `Flavor::Macos` dispatch + new tests; flip `macos_flavor_is_unsupported` test |
| `phermesd/src/error.rs` (or whichever holds QemuError) | Modify | `QemuError::MissingAppleOsk { path }` variant |
| `src/phermes_build/vm.py` | Modify | `MACOS_*` constants, `_macos_def_text`, `write_macos_def`, `provision_macos_disk` |
| `src/phermes_build/cli.py` | Modify | Extend `_SUPPORTED_IMPORT_FLAVORS`; `--dev-apple-osk` flag; `_write_apple_osk_secret`; `_provision_macos_vm`; build() wiring |
| `tests/phermes_build/test_vm.py` | Modify | macOS constants + write_def + provision_disk tests |
| `tests/phermes_build/test_cli.py` | Modify | macOS flavor tests + OSK flag tests + provision routing; flip `test_unsupported_import_vm_flavor_errors_out` to use `freebsd=` |
| `CHANGELOG.md` | Modify | Slice #5b entry |
| `README.md` | Modify | Add macOS BYOI example + slice #5b paragraph |

---

## Task M1: phermesd `secrets.rs` (Apple OSK reader)

**Files:**
- Create: `phermesd/src/secrets.rs`
- Modify: `phermesd/src/lib.rs` (or `phermesd/src/main.rs` — whichever declares the existing modules)

- [ ] **Step 1: Locate module declaration site**

```bash
grep -n "^mod " phermesd/src/lib.rs phermesd/src/main.rs 2>/dev/null
```

Identify the file that contains the other `mod xxx;` lines. That's where `mod secrets;` goes.

- [ ] **Step 2: Write the failing tests** — create `phermesd/src/secrets.rs` with this skeleton (no implementation yet, just the public surface + tests at the bottom):

```rust
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum SecretsError {
    NotFound { path: PathBuf },
    BadPermissions { path: PathBuf, mode: u32 },
    BadFormat { reason: &'static str },
    Io(std::io::Error),
}

impl std::fmt::Display for SecretsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound { path } => write!(f, "apple OSK file not found: {}", path.display()),
            Self::BadPermissions { path, mode } => write!(
                f,
                "apple OSK file {} has insecure mode {:o}; expected 0o400 with no group/other access",
                path.display(),
                mode
            ),
            Self::BadFormat { reason } => write!(f, "apple OSK has bad format: {reason}"),
            Self::Io(e) => write!(f, "apple OSK read I/O error: {e}"),
        }
    }
}

impl std::error::Error for SecretsError {}

pub const APPLE_OSK_PATH: &str = "/etc/phermes/secrets/apple.osk";

pub fn read_apple_osk(path: &Path) -> Result<String, SecretsError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    fn write_osk(dir: &Path, mode: u32, body: &str) -> PathBuf {
        let p = dir.join("apple.osk");
        fs::write(&p, body).unwrap();
        let mut perms = fs::metadata(&p).unwrap().permissions();
        perms.set_mode(mode);
        fs::set_permissions(&p, perms).unwrap();
        p
    }

    #[test]
    fn returns_ok_for_valid_file() {
        let tmp = tempfile::tempdir().unwrap();
        let osk = "0123456789abcdef".repeat(4); // 64 hex chars
        let p = write_osk(tmp.path(), 0o400, &osk);
        assert_eq!(read_apple_osk(&p).unwrap(), osk);
    }

    #[test]
    fn strips_trailing_whitespace() {
        let tmp = tempfile::tempdir().unwrap();
        let osk = "0123456789abcdef".repeat(4);
        let p = write_osk(tmp.path(), 0o400, &format!("{osk}\n"));
        assert_eq!(read_apple_osk(&p).unwrap(), osk);
    }

    #[test]
    fn rejects_short_string() {
        let tmp = tempfile::tempdir().unwrap();
        let p = write_osk(tmp.path(), 0o400, "deadbeef");
        assert!(matches!(read_apple_osk(&p), Err(SecretsError::BadFormat { .. })));
    }

    #[test]
    fn rejects_non_hex() {
        let tmp = tempfile::tempdir().unwrap();
        let p = write_osk(tmp.path(), 0o400, &"z".repeat(64));
        assert!(matches!(read_apple_osk(&p), Err(SecretsError::BadFormat { .. })));
    }

    #[test]
    fn rejects_world_readable_mode() {
        let tmp = tempfile::tempdir().unwrap();
        let osk = "0123456789abcdef".repeat(4);
        let p = write_osk(tmp.path(), 0o444, &osk);
        assert!(matches!(read_apple_osk(&p), Err(SecretsError::BadPermissions { .. })));
    }

    #[test]
    fn returns_not_found_when_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let p = tmp.path().join("nope.osk");
        assert!(matches!(read_apple_osk(&p), Err(SecretsError::NotFound { .. })));
    }
}
```

- [ ] **Step 3: Wire `mod secrets;`** in the module declaration file from Step 1.

- [ ] **Step 4: Check `tempfile` is in dev-deps**

```bash
grep -n "tempfile" phermesd/Cargo.toml
```

If absent, add to `[dev-dependencies]`:

```bash
cargo add --package phermesd --dev tempfile
```

(Pin to whatever the latest stable minor is at lookup time.)

- [ ] **Step 5: Verify tests fail**

Run: `cargo test -p phermesd secrets`
Expected: 6 failures with `not implemented`.

- [ ] **Step 6: Implement `read_apple_osk`**

Replace the `unimplemented!()` body with:

```rust
pub fn read_apple_osk(path: &Path) -> Result<String, SecretsError> {
    let meta = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(SecretsError::NotFound { path: path.to_path_buf() });
        }
        Err(e) => return Err(SecretsError::Io(e)),
    };
    use std::os::unix::fs::PermissionsExt;
    let mode = meta.permissions().mode() & 0o777;
    if mode & 0o077 != 0 {
        return Err(SecretsError::BadPermissions { path: path.to_path_buf(), mode });
    }
    let raw = std::fs::read_to_string(path).map_err(SecretsError::Io)?;
    let trimmed = raw.trim_end();
    if trimmed.len() != 64 {
        return Err(SecretsError::BadFormat { reason: "expected exactly 64 chars after trim" });
    }
    if !trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(SecretsError::BadFormat { reason: "expected 64 hex chars [0-9a-fA-F]" });
    }
    Ok(trimmed.to_string())
}
```

- [ ] **Step 7: Verify tests pass + clippy + whole suite**

```bash
cargo test -p phermesd secrets
cargo clippy -p phermesd --all-targets --all-features -- -D warnings
cargo test -p phermesd
```

Expected: 6 new tests pass, no clippy warnings, full Rust suite green.

- [ ] **Step 8: Commit**

```bash
git add phermesd/src/secrets.rs phermesd/src/lib.rs phermesd/Cargo.toml phermesd/Cargo.lock
git commit -m "feat(phermesd): secrets.rs Apple OSK reader (mode 0400 + 64 hex validation)"
```

(Adjust path for `lib.rs` vs `main.rs` and only include `Cargo.toml`/`Cargo.lock` if Step 4 modified them.)

---

## Task M2: phermesd `build_macos` argv builder + dispatch

**Files:**
- Modify: `phermesd/src/qemu.rs`
- Modify: whichever module declares `QemuError` (locate with `grep -n "enum QemuError" phermesd/src/*.rs`)

- [ ] **Step 1: Locate `QemuError` and existing `macos_flavor_is_unsupported` test**

```bash
grep -n "enum QemuError\|macos_flavor_is_unsupported\|UnsupportedFlavor" phermesd/src/qemu.rs phermesd/src/error.rs 2>/dev/null
```

- [ ] **Step 2: Add `QemuError::MissingAppleOsk` variant**

In the file that declares `QemuError`, add:

```rust
MissingAppleOsk { path: std::path::PathBuf, source: crate::secrets::SecretsError },
```

And in its `Display` impl (or `#[derive(Error)]` annotation if thiserror):

```rust
Self::MissingAppleOsk { path, source } => write!(
    f,
    "Apple OSK not available at {} — write the 64-char hex string (mode 0400, root-owned) and retry. ({source})",
    path.display()
),
```

If `QemuError` uses `thiserror`, add `#[error("...")]` instead and mark `source` with `#[source]`.

- [ ] **Step 3: Flip the existing `macos_flavor_is_unsupported` test**

Find the test (slice #5a's W1 renamed it from `non_linux_flavor_is_unsupported`). It currently asserts `Flavor::Macos` returns `UnsupportedFlavor`. Rename and rewrite:

```rust
#[test]
fn macos_flavor_returns_missing_osk_when_secrets_path_absent() {
    let vm = vm_with_flavor(Flavor::Macos);
    let rt = test_runtime_with_secrets_root("/nonexistent/phermes/secrets");
    let err = build_argv(&vm, &rt).unwrap_err();
    assert!(matches!(err, QemuError::MissingAppleOsk { .. }));
}
```

(The `vm_with_flavor` and `test_runtime_with_secrets_root` helpers may need extending — see Step 4.)

- [ ] **Step 4: Write new `build_macos` unit tests**

These tests call the **pure** `build_macos(vm, rt, osk)` directly with a literal OSK string — they don't touch the filesystem.

```rust
const TEST_OSK: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

#[test]
fn macos_argv_includes_penryn_cpu_with_full_flag_set() {
    let vm = vm_with_flavor(Flavor::Macos);
    let rt = test_runtime();
    let argv = build_macos(&vm, &rt, TEST_OSK);
    let cpu_idx = argv.iter().position(|a| a == "-cpu").expect("-cpu present");
    let cpu_val = &argv[cpu_idx + 1];
    assert!(cpu_val.starts_with("Penryn,"), "got: {cpu_val}");
    for flag in [
        "kvm=on", "vendor=GenuineIntel", "+invtsc", "+pcid", "+ssse3",
        "+sse4.2", "+popcnt", "+avx", "+aes", "+xsave", "+xsaveopt", "check",
    ] {
        assert!(cpu_val.contains(flag), "missing {flag} in {cpu_val}");
    }
}

#[test]
fn macos_argv_includes_isa_applesmc_with_osk() {
    let vm = vm_with_flavor(Flavor::Macos);
    let argv = build_macos(&vm, &test_runtime(), TEST_OSK);
    let joined = argv.join(" ");
    assert!(joined.contains(&format!("isa-applesmc,osk={TEST_OSK}")), "{joined}");
}

#[test]
fn macos_argv_uses_ahci_not_virtio_scsi() {
    let vm = vm_with_flavor(Flavor::Macos);
    let argv = build_macos(&vm, &test_runtime(), TEST_OSK);
    let joined = argv.join(" ");
    assert!(joined.contains("ich9-ahci"), "no ahci controller: {joined}");
    assert!(joined.contains("ide-hd"), "no ide-hd disk: {joined}");
    assert!(!joined.contains("virtio-scsi"), "virtio-scsi present: {joined}");
}

#[test]
fn macos_argv_uses_vmxnet3() {
    let vm = vm_with_flavor(Flavor::Macos);
    let argv = build_macos(&vm, &test_runtime(), TEST_OSK);
    let joined = argv.join(" ");
    assert!(joined.contains("vmxnet3"), "no vmxnet3: {joined}");
    assert!(!joined.contains("virtio-net"), "virtio-net present: {joined}");
}

#[test]
fn macos_argv_byte_identical_for_equivalent_def_modulo_osk() {
    let vm = vm_with_flavor(Flavor::Macos);
    let a = build_macos(&vm, &test_runtime(), TEST_OSK);
    let b = build_macos(&vm, &test_runtime(), TEST_OSK);
    assert_eq!(a, b);
}
```

`vm_with_flavor` and `test_runtime` likely already exist (slice #5a's W1 added one). Reuse them. If `test_runtime_with_secrets_root` doesn't exist yet, add a thin variant of `test_runtime` that takes a path.

- [ ] **Step 5: Verify tests fail**

```bash
cargo test -p phermesd qemu::tests::macos
```

Expected: 5 new tests fail (functions don't exist) + the flipped test fails.

- [ ] **Step 6: Implement `build_macos` (pure)**

Place after `build_pc_uefi`:

```rust
fn build_macos(vm: &Vm, rt: &Runtime, osk: &str) -> Vec<String> {
    let mut argv = vec![
        "qemu-system-x86_64".to_string(),
        "-enable-kvm".to_string(),
        "-machine".to_string(),
        "q35,accel=kvm,kernel-irqchip=split".to_string(),
        "-cpu".to_string(),
        "Penryn,kvm=on,vendor=GenuineIntel,+invtsc,+pcid,+ssse3,+sse4.2,+popcnt,\
         +avx,+aes,+xsave,+xsaveopt,check".to_string(),
        "-smp".to_string(),
        format!("cpus={},cores=2,threads=2,sockets=1", vm.def.resources.vcpus),
        "-m".to_string(),
        format!("{}", vm.def.resources.memory_mib),
        "-drive".to_string(),
        format!(
            "if=pflash,format=raw,readonly=on,file={}",
            vm.def.firmware.ovmf_code.display()
        ),
        "-drive".to_string(),
        format!(
            "if=pflash,format=raw,file={}",
            rt.ovmf_vars_path(&vm.id).display()
        ),
        "-device".to_string(),
        format!("isa-applesmc,osk={osk}"),
        "-device".to_string(),
        "ich9-ahci,id=ahci0".to_string(),
    ];
    for (i, disk) in vm.def.disks.iter().enumerate() {
        argv.push("-drive".to_string());
        argv.push(format!(
            "id=disk{i},if=none,format={},file={}",
            disk.format,
            disk.path.display()
        ));
        argv.push("-device".to_string());
        argv.push(format!("ide-hd,bus=ahci0.{i},drive=disk{i}"));
    }
    for (i, net) in vm.def.nets.iter().enumerate() {
        argv.push("-netdev".to_string());
        argv.push(format!("user,id=net{i}"));
        argv.push("-device".to_string());
        argv.push(format!("vmxnet3,netdev=net{i}"));
        let _ = net; // bridge wiring is launcher's job; argv stays user-net for unit-test determinism
    }
    argv.push("-device".to_string());
    argv.push("nec-usb-xhci,id=xhci".to_string());
    argv.push("-device".to_string());
    argv.push("usb-tablet,bus=xhci.0".to_string());
    if vm.def.console.serial {
        argv.push("-chardev".to_string());
        argv.push(format!("socket,id=serial0,path={},server=on,wait=off",
            rt.serial_sock_path(&vm.id).display()));
        argv.push("-serial".to_string());
        argv.push("chardev:serial0".to_string());
    }
    if vm.def.console.vnc {
        argv.push("-vnc".to_string());
        argv.push(format!("unix:{}", rt.vnc_sock_path(&vm.id).display()));
    }
    argv.push("-qmp".to_string());
    argv.push(format!("unix:{},server=on,wait=off",
        rt.qmp_sock_path(&vm.id).display()));
    argv.push("-pidfile".to_string());
    argv.push(rt.pidfile_path(&vm.id).display().to_string());
    argv
}
```

**Note:** Match `build_pc_uefi`'s style exactly — same helper accessors (`rt.serial_sock_path(...)`, etc.). If those accessor names differ, mirror what `build_pc_uefi` uses. Don't invent new ones.

- [ ] **Step 7: Implement `build_macos_with_secrets` (seam)**

```rust
fn build_macos_with_secrets(vm: &Vm, rt: &Runtime) -> Result<Vec<String>, QemuError> {
    let path = rt.apple_osk_path();
    let osk = crate::secrets::read_apple_osk(&path).map_err(|source| {
        QemuError::MissingAppleOsk { path: path.clone(), source }
    })?;
    Ok(build_macos(vm, rt, &osk))
}
```

If `Runtime` has no `apple_osk_path()` accessor, add one returning `PathBuf::from(crate::secrets::APPLE_OSK_PATH)` (or whatever path the test runtime is set up to override). The test runtime variant `test_runtime_with_secrets_root` should return a path under a tmp dir.

- [ ] **Step 8: Update dispatch in `build_argv`**

Replace the existing match:

```rust
match vm.def.flavor {
    Flavor::Linux | Flavor::Windows => Ok(build_pc_uefi(vm, rt)),
    Flavor::Macos => build_macos_with_secrets(vm, rt),
}
```

The previous `other @ Flavor::Macos => Err(QemuError::UnsupportedFlavor(other))` arm is removed.

- [ ] **Step 9: Verify pass + clippy + whole suite**

```bash
cargo test -p phermesd
cargo clippy -p phermesd --all-targets --all-features -- -D warnings
```

Expected: all green; new tests pass; the flipped `macos_flavor_returns_missing_osk_when_secrets_path_absent` passes; no clippy warnings.

- [ ] **Step 10: Commit**

```bash
git add phermesd/src/qemu.rs phermesd/src/error.rs  # adjust if QemuError lives elsewhere
git commit -m "feat(phermesd): build_macos argv + Flavor::Macos dispatch via secrets-backed OSK"
```

---

## Task M3: phermes-build `vm.py` macOS constants + write_def + provision_disk

**Files:**
- Modify: `src/phermes_build/vm.py`
- Modify: `tests/phermes_build/test_vm.py`

- [ ] **Step 1: Append failing tests** to `tests/phermes_build/test_vm.py`:

```python


def test_macos_constants():
    assert vm_mod.MACOS_VMID == 100
    assert vm_mod.MACOS_DEFAULT_DISK_GB == 120
    assert vm_mod.MACOS_DEFAULT_MEMORY_MIB == 12288
    assert vm_mod.MACOS_DEFAULT_VCPUS == 4


def test_write_macos_def_emits_expected_toml(tmp_path):
    import os as _os
    chroot = str(tmp_path / "chroot")
    _os.makedirs(chroot)
    vm_mod.write_macos_def(chroot)

    toml_path = tmp_path / "chroot" / "etc/phermes/vms/macos.toml"
    assert toml_path.exists()
    content = toml_path.read_text()
    assert 'flavor = "macos"' in content
    assert "memory_mib = 12288" in content
    assert "vcpus = 4" in content
    assert 'cpu = "Penryn"' in content
    assert 'ovmf_code = "/usr/share/OVMF/OVMF_CODE.fd"' in content
    assert 'path = "/dev/pve/vm-100-disk-0"' in content
    assert 'format = "raw"' in content
    assert 'interface = "ahci"' in content
    assert 'model = "vmxnet3"' in content
    assert "serial = true" in content
    assert "vnc = true" in content
    # No OSK in toml — phermesd reads it from /etc/phermes/secrets/apple.osk at launch.
    assert "osk" not in content.lower()
    assert content.count("[[disk]]") == 1


def test_write_macos_def_honors_override_resources(tmp_path):
    import os as _os
    chroot = str(tmp_path / "chroot")
    _os.makedirs(chroot)
    vm_mod.write_macos_def(chroot, memory_mib=16384, vcpus=8)
    content = (tmp_path / "chroot" / "etc/phermes/vms/macos.toml").read_text()
    assert "memory_mib = 16384" in content
    assert "vcpus = 8" in content


def test_provision_macos_disk_creates_thin_lv_and_tags_it(monkeypatch):
    calls = _capture(monkeypatch)
    vm_mod.provision_macos_disk()
    assert calls[0][0] == "lvcreate"
    assert "--thin" in calls[0]
    assert "--virtualsize" in calls[0]
    assert "120G" in calls[0]
    assert "pve/data" in calls[0]
    assert "vm-100-disk-0" in calls[0]
    assert calls[1] == ["lvchange", "--addtag", "phermesd", "/dev/pve/vm-100-disk-0"]
    assert not any(c[0] == "qemu-img" for c in calls)


def test_provision_macos_disk_with_source_runs_qemu_img_convert(monkeypatch):
    calls = _capture(monkeypatch)
    vm_mod.provision_macos_disk(source="/tmp/macos.qcow2")
    qemu_calls = [c for c in calls if c[0] == "qemu-img"]
    assert len(qemu_calls) == 1
    assert qemu_calls[0] == [
        "qemu-img", "convert", "-O", "raw",
        "/tmp/macos.qcow2", "/dev/pve/vm-100-disk-0",
    ]


def test_provision_macos_disk_custom_size(monkeypatch):
    calls = _capture(monkeypatch)
    vm_mod.provision_macos_disk(size_gb=200)
    assert "200G" in calls[0]
```

- [ ] **Step 2: Verify fail**

```bash
uv run pytest tests/phermes_build/test_vm.py -k macos -v 2>&1 | tail -15
```

Expected: 6 failures (constants/funcs don't exist).

- [ ] **Step 3: Implement in `src/phermes_build/vm.py`**

Add constants after the Windows ones:

```python
MACOS_VMID = 100
MACOS_DEFAULT_DISK_GB = 120
MACOS_DEFAULT_MEMORY_MIB = 12288
MACOS_DEFAULT_VCPUS = 4
```

Add `_macos_def_text` + `write_macos_def` after `write_windows_def`:

```python
def _macos_def_text(*, memory_mib: int, vcpus: int) -> str:
    """Render /etc/phermes/vms/macos.toml content.

    Slice #5b is BYOI: operator supplies an OpenCore-EFI + macOS qcow2 plus the
    Apple OSK as a runtime secret at /etc/phermes/secrets/apple.osk. The def
    omits the OSK — phermesd reads it at launch.
    """
    return (
        f'flavor = "macos"\n'
        f"[resources]\n"
        f"memory_mib = {memory_mib}\n"
        f"vcpus = {vcpus}\n"
        f'cpu = "Penryn"\n'
        f"[firmware]\n"
        f'ovmf_code = "/usr/share/OVMF/OVMF_CODE.fd"\n'
        f'ovmf_vars_template = "/usr/share/OVMF/OVMF_VARS.fd"\n'
        f"[[disk]]\n"
        f'path = "/dev/{STORAGE_VG}/vm-{MACOS_VMID}-disk-0"\n'
        f'format = "raw"\n'
        f'interface = "ahci"\n'
        f"[[net]]\n"
        f'bridge = "vmbr0"\n'
        f'model = "vmxnet3"\n'
        f"[console]\n"
        f"serial = true\n"
        f"vnc = true\n"
    )


def write_macos_def(
    chroot_mount: str,
    *,
    memory_mib: int = MACOS_DEFAULT_MEMORY_MIB,
    vcpus: int = MACOS_DEFAULT_VCPUS,
) -> None:
    """Write /etc/phermes/vms/macos.toml inside the chroot."""
    vms_dir = os.path.join(chroot_mount, "etc/phermes/vms")
    os.makedirs(vms_dir, exist_ok=True)
    def_path = os.path.join(vms_dir, "macos.toml")
    with open(def_path, "w") as f:
        f.write(_macos_def_text(memory_mib=memory_mib, vcpus=vcpus))
```

Add `provision_macos_disk` after `provision_windows_disk`:

```python
def provision_macos_disk(
    size_gb: int = MACOS_DEFAULT_DISK_GB,
    source: str | None = None,
) -> None:
    """Create the macOS thin LV, tag it 'phermesd', optionally populate from a local image.

    BYOI: ``source`` is an operator-supplied qcow2 with OpenCore EFI baked in.
    qemu-img handles qcow2/vmdk/raw inputs transparently.
    """
    disk_name = f"vm-{MACOS_VMID}-disk-0"
    device = f"/dev/{STORAGE_VG}/{disk_name}"
    run_cmd([
        "lvcreate", "--thin", "--virtualsize", f"{size_gb}G",
        f"{STORAGE_VG}/{STORAGE_POOL}", "-n", disk_name,
    ])
    run_cmd(["lvchange", "--addtag", OWNER_TAG, device])
    if source is not None:
        run_cmd(["qemu-img", "convert", "-O", "raw", source, device])
```

- [ ] **Step 4: Verify pass + lint + typecheck + whole suite**

```bash
uv run pytest -q
uv run ruff check src/phermes_build/vm.py tests/phermes_build/test_vm.py
uv run ty check src/phermes_build/vm.py
```

Expected: suite count 131 passed / 3 skipped (was 125/3, +6).

- [ ] **Step 5: Commit**

```bash
git add src/phermes_build/vm.py tests/phermes_build/test_vm.py
git commit -m "feat(builder): vm.py macOS constants + write_macos_def + provision_macos_disk"
```

---

## Task M4: phermes-build `cli.py` accept macos= flavor

**Files:**
- Modify: `src/phermes_build/cli.py`
- Modify: `tests/phermes_build/test_cli.py`

- [ ] **Step 1: Write failing tests** — append to `tests/phermes_build/test_cli.py`:

```python


# ── M4: _vm_source accepts macos= ───────────────────────────────────────────


def test_vm_source_accepts_macos_flavor():
    from phermes_build import cli as cli_mod
    assert cli_mod._vm_source(["macos=/tmp/m.qcow2"], "macos") == "/tmp/m.qcow2"


def test_vm_source_returns_none_for_other_flavor_when_only_macos_given():
    from phermes_build import cli as cli_mod
    assert cli_mod._vm_source(["macos=/tmp/m.qcow2"], "linux") is None


def test_vm_source_picks_macos_when_all_three_given():
    from phermes_build import cli as cli_mod
    args = ["linux=/a", "windows=/b", "macos=/c"]
    assert cli_mod._vm_source(args, "macos") == "/c"
```

Also flip the existing `test_unsupported_import_vm_flavor_errors_out` — `macos=` is now accepted:

```bash
grep -n "macos=/tmp/m.qcow2\|slice #5a supports" tests/phermes_build/test_cli.py
```

Replace the body's `macos=/tmp/m.qcow2` with `freebsd=/tmp/fb.qcow2` and update the docstring to reflect that `freebsd=` is the new sentinel-rejected flavor.

- [ ] **Step 2: Verify fail**

```bash
uv run pytest tests/phermes_build/test_cli.py -k "vm_source or unsupported_import" -v 2>&1 | tail -10
```

Expected: 3 new tests fail (assert mismatch — macos currently raises); the flipped test passes only if cli.py is also updated; OR fails because `freebsd=` still raises (which is what we want).

- [ ] **Step 3: Update `_SUPPORTED_IMPORT_FLAVORS`** in `src/phermes_build/cli.py`:

```python
_SUPPORTED_IMPORT_FLAVORS = ("linux", "windows", "macos")
```

- [ ] **Step 4: Verify pass + lint + typecheck + whole suite**

```bash
uv run pytest -q
uv run ruff check src/phermes_build/cli.py tests/phermes_build/test_cli.py
uv run ty check src/phermes_build/cli.py
```

Expected: suite count 134 passed / 3 skipped.

- [ ] **Step 5: Commit**

```bash
git add src/phermes_build/cli.py tests/phermes_build/test_cli.py
git commit -m "feat(builder): _vm_source accepts macos=<path>; flip sentinel-reject test to freebsd="
```

---

## Task M5: phermes-build `cli.py` `--dev-apple-osk` flag + secret writing

**Files:**
- Modify: `src/phermes_build/cli.py`
- Modify: `tests/phermes_build/test_cli.py`

- [ ] **Step 1: Write failing tests** — append:

```python


# ── M5: --dev-apple-osk flag + secret file ────────────────────────────────


_VALID_OSK = "0123456789abcdef" * 4  # 64 hex chars


def test_dev_apple_osk_writes_secret_file_with_0400_mode(tmp_path, monkeypatch):
    from phermes_build import cli as cli_mod

    chroot = str(tmp_path / "chroot")
    cli_mod._write_apple_osk_secret(chroot, _VALID_OSK)

    osk_path = tmp_path / "chroot" / "etc/phermes/secrets/apple.osk"
    assert osk_path.exists()
    assert osk_path.read_text() == _VALID_OSK
    import os as _os
    assert (_os.stat(osk_path).st_mode & 0o777) == 0o400
    # Parent dir mode 0700
    secrets_dir = tmp_path / "chroot" / "etc/phermes/secrets"
    assert (_os.stat(secrets_dir).st_mode & 0o777) == 0o700


def test_write_apple_osk_secret_rejects_bad_format(tmp_path):
    import typer
    from phermes_build import cli as cli_mod
    with pytest.raises(typer.BadParameter):
        cli_mod._write_apple_osk_secret(str(tmp_path / "chroot"), "tooshort")
    with pytest.raises(typer.BadParameter):
        cli_mod._write_apple_osk_secret(str(tmp_path / "chroot"), "z" * 64)


def test_dev_apple_osk_without_dev_credentials_errors_out(monkeypatch):
    """--dev-apple-osk without --dev-credentials must fail closed."""
    from phermes_build import cli as cli_mod

    for helper in (
        "_setup_luks", "_setup_lvm", "_setup_btrfs", "_setup_exfat",
        "_install_minimal_host", "_configure_host", "_setup_credentials",
        "_write_firstboot", "_partition", "_provision_linux_vm",
    ):
        monkeypatch.setattr(cli_mod, helper, lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "compute_layout", lambda *a, **k: _fake_layout())

    result = runner.invoke(
        cli_mod.app,
        [
            "/dev/loop0",
            "--dev-apple-osk", _VALID_OSK,
            "--luks-passphrase", "x" * 12,
        ],
    )
    assert result.exit_code != 0


def test_secrets_dir_created_even_without_osk(tmp_path):
    from phermes_build import cli as cli_mod
    chroot = str(tmp_path / "chroot")
    cli_mod._ensure_secrets_dir(chroot)
    import os as _os
    secrets_dir = tmp_path / "chroot" / "etc/phermes/secrets"
    assert secrets_dir.exists()
    assert (_os.stat(secrets_dir).st_mode & 0o777) == 0o700
```

- [ ] **Step 2: Verify fail**

```bash
uv run pytest tests/phermes_build/test_cli.py -k "apple_osk or secrets_dir" -v 2>&1 | tail -15
```

Expected: 4 failures.

- [ ] **Step 3: Add the CLI flag**

In `src/phermes_build/cli.py` `build()` signature, after `dev_ssh_pubkey`:

```python
    dev_apple_osk: Annotated[
        str | None,
        typer.Option(
            envvar="PHERMES_DEV_APPLE_OSK",
            help="DEV: Apple OSK (64 hex chars) for macOS guest (with --dev-credentials)",
        ),
    ] = None,
```

- [ ] **Step 4: Add helpers** (place near `_write_cloud_init_seed`):

```python
def _ensure_secrets_dir(chroot_mount: str) -> str:
    """Create <chroot>/etc/phermes/secrets/ mode 0700 and return its path."""
    secrets_dir = os.path.join(chroot_mount, "etc/phermes/secrets")
    os.makedirs(secrets_dir, exist_ok=True)
    os.chmod(secrets_dir, 0o700)
    return secrets_dir


def _write_apple_osk_secret(chroot_mount: str, osk: str) -> None:
    """Validate OSK and write to <chroot>/etc/phermes/secrets/apple.osk mode 0400."""
    osk = osk.strip()
    if len(osk) != 64:
        raise typer.BadParameter(
            "--dev-apple-osk must be exactly 64 characters after trim"
        )
    if not all(c in "0123456789abcdefABCDEF" for c in osk):
        raise typer.BadParameter(
            "--dev-apple-osk must be 64 hex characters [0-9a-fA-F]"
        )
    secrets_dir = _ensure_secrets_dir(chroot_mount)
    osk_path = os.path.join(secrets_dir, "apple.osk")
    with open(osk_path, "w") as f:
        f.write(osk)
    os.chmod(osk_path, 0o400)
```

- [ ] **Step 5: Wire validation in `build()`**

Right after the existing `_resolve_luks_passphrase` call (or after the dev_credentials check), add:

```python
    if dev_apple_osk is not None and not dev_credentials:
        raise typer.BadParameter(
            "--dev-apple-osk requires --dev-credentials"
        )
```

And before the steps execute (in the os_steps construction block), unconditionally append a "Creating secrets directory" step so the empty `secrets/` exists for production builds too:

```python
    os_steps.append(
        ("Creating secrets directory", lambda: _ensure_secrets_dir(PVE_ROOT_MOUNT))
    )
    if dev_credentials and dev_apple_osk:
        os_steps.append(
            (
                "Writing Apple OSK secret",
                lambda: _write_apple_osk_secret(PVE_ROOT_MOUNT, dev_apple_osk),
            )
        )
```

Place these after `_setup_credentials` and before VM provisioning steps.

- [ ] **Step 6: Verify pass + lint + typecheck + whole suite**

```bash
uv run pytest -q
uv run ruff check src/phermes_build/cli.py tests/phermes_build/test_cli.py
uv run ty check src/phermes_build/cli.py
```

Expected: suite 138 passed / 3 skipped.

- [ ] **Step 7: Commit**

```bash
git add src/phermes_build/cli.py tests/phermes_build/test_cli.py
git commit -m "feat(builder): --dev-apple-osk + /etc/phermes/secrets/apple.osk (mode 0400)"
```

---

## Task M6: phermes-build `_provision_macos_vm` + `build()` wiring

**Files:**
- Modify: `src/phermes_build/cli.py`
- Modify: `tests/phermes_build/test_cli.py`

- [ ] **Step 1: Write failing tests** — append:

```python


# ── M6: macOS VM provisioning wired into build() ─────────────────────────


def test_import_vm_macos_routes_into_provision_macos_vm(monkeypatch):
    """--import-vm macos=<path> reaches _provision_macos_vm(source=<path>)."""
    from phermes_build import cli as cli_mod

    seen: dict = {}
    for helper in (
        "_setup_luks", "_setup_lvm", "_setup_btrfs", "_setup_exfat",
        "_install_minimal_host", "_configure_host", "_setup_credentials",
        "_write_firstboot", "_partition", "_provision_linux_vm",
        "_provision_windows_vm",
    ):
        monkeypatch.setattr(cli_mod, helper, lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "compute_layout", lambda *a, **k: _fake_layout())

    def fake_provision_macos(source=None):
        seen["source"] = source
    monkeypatch.setattr(cli_mod, "_provision_macos_vm", fake_provision_macos)

    result = runner.invoke(
        cli_mod.app,
        ["/dev/loop0", "--import-vm", "macos=/tmp/m.qcow2", "--dev-credentials"],
    )
    assert result.exit_code == 0, result.stdout
    assert seen.get("source") == "/tmp/m.qcow2"


def test_no_vm_skips_macos_provision_even_with_import(monkeypatch):
    from phermes_build import cli as cli_mod

    seen: list[str] = []
    for helper in (
        "_setup_luks", "_setup_lvm", "_setup_btrfs", "_setup_exfat",
        "_install_minimal_host", "_configure_host", "_setup_credentials",
        "_write_firstboot", "_partition",
        "_provision_linux_vm", "_provision_windows_vm",
    ):
        monkeypatch.setattr(cli_mod, helper, lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "compute_layout", lambda *a, **k: _fake_layout())
    monkeypatch.setattr(
        cli_mod, "_provision_macos_vm", lambda *a, **k: seen.append("macos")
    )

    result = runner.invoke(
        cli_mod.app,
        ["/dev/loop0", "--no-vm",
         "--import-vm", "macos=/tmp/m.qcow2", "--dev-credentials"],
    )
    assert result.exit_code == 0, result.stdout
    assert "macos" not in seen


def test_provision_macos_vm_calls_write_def_and_provision_disk(monkeypatch):
    from phermes_build import cli as cli_mod
    from phermes_build import vm as vm_mod

    calls: list[str] = []
    monkeypatch.setattr(
        vm_mod, "write_macos_def",
        lambda mount, **kw: calls.append(f"write:{mount}"),
    )
    monkeypatch.setattr(
        vm_mod, "provision_macos_disk",
        lambda source=None: calls.append(f"disk:{source}"),
    )

    cli_mod._provision_macos_vm(source="/tmp/m.qcow2")

    assert any(c.startswith("write:") for c in calls)
    assert "disk:/tmp/m.qcow2" in calls
```

- [ ] **Step 2: Verify fail**

```bash
uv run pytest tests/phermes_build/test_cli.py -k "macos" -v 2>&1 | tail -10
```

Expected: 3 failures (function doesn't exist).

- [ ] **Step 3: Implement `_provision_macos_vm`**

Add after `_provision_windows_vm`:

```python
def _provision_macos_vm(source: str | None) -> None:
    """BYOI: write macos.toml + create+populate the LVM-thin LV.

    Only invoked when the operator supplied --import-vm macos=<path>. No
    cloud-init seed; the OSK lives at /etc/phermes/secrets/apple.osk and is
    read by phermesd at launch.
    """
    vm_mod.write_macos_def(PVE_ROOT_MOUNT)
    vm_mod.provision_macos_disk(source=source)
```

- [ ] **Step 4: Wire into `build()`**

After the existing `windows_source` extraction and after the existing Windows step append, add:

```python
    macos_source = _vm_source(import_vm or [], "macos")
```

(Place it next to `linux_source` / `windows_source`.)

And in the `if not no_vm:` block, after the Windows step:

```python
        if macos_source is not None:
            os_steps.append(
                (
                    "Provisioning macOS VM",
                    lambda: _provision_macos_vm(source=macos_source),
                )
            )
```

- [ ] **Step 5: Verify pass + lint + typecheck + whole suite**

```bash
uv run pytest -q
uv run ruff check src/phermes_build/cli.py tests/phermes_build/test_cli.py
uv run ty check src/phermes_build/cli.py
```

Expected: suite 141 passed / 3 skipped.

- [ ] **Step 6: Commit**

```bash
git add src/phermes_build/cli.py tests/phermes_build/test_cli.py
git commit -m "feat(builder): provision macOS VM when --import-vm macos=<path> supplied"
```

---

## Task M7: README + CHANGELOG

**Files:**
- Modify: `CHANGELOG.md`
- Modify: `README.md`

- [ ] **Step 1: Add to CHANGELOG.md `[Unreleased] / Added`** (top of the section, above the slice #5a entry):

```markdown
- `phermes-build` macOS guest support (slice #5b, BYOI): `--import-vm
  macos=<path>` provisions a macOS VM at install time. The operator supplies a
  qcow2 with OpenCore EFI + macOS baked in. Defaults: 12 GiB RAM, 4 vCPUs
  (2 cores × 2 threads), VMID=100, AHCI storage, vmxnet3 net, OVMF firmware,
  Penryn CPU with the Hackintosh flag set. The Apple OSK is **not** shipped in
  the install image: production builds expect the operator to drop the 64-char
  hex string into `/etc/phermes/secrets/apple.osk` (mode 0400) post-install;
  `--dev-credentials --dev-apple-osk <osk>` pre-seeds it for dev builds.
  `phermesd` adds `secrets.rs` (typed OSK reader with mode/format validation)
  and `build_macos` argv in `qemu.rs`; the dispatch is now fully explicit
  (no wildcard arm). The whole pipeline ships no Apple binaries or OpenCore
  images.
```

- [ ] **Step 2: Update README.md `--import-vm` example** (the BYOI section near line 183):

Find:

```bash
phermes-build /dev/sdX \
  --import-vm linux=/path/to/linux.qcow2 \
  --import-vm windows=/path/to/windows.qcow2
```

Replace with:

```bash
phermes-build /dev/sdX \
  --import-vm linux=/path/to/linux.qcow2 \
  --import-vm windows=/path/to/windows.qcow2 \
  --import-vm macos=/path/to/macos.qcow2 \
  --dev-credentials --dev-apple-osk <64-hex-char-string>
# Production: drop the OSK into /etc/phermes/secrets/apple.osk (mode 0400)
# on the appliance post-install instead of using --dev-apple-osk.
```

- [ ] **Step 3: Append slice #5b paragraph** to the long phermesd description (after the slice #5a sentence):

```markdown
 Slice #5b (implemented): macOS guest support via BYOI — `--import-vm
macos=<path>` provisions a macOS VM (120 GB LVM-thin LV, VMID=100, 12 GiB
RAM, AHCI + vmxnet3, OVMF + Penryn CPU). The operator supplies an OpenCore
+ macOS qcow2 plus the Apple OSK at `/etc/phermes/secrets/apple.osk` (mode
0400) — production builds ship no Apple credentials. Design:
[`docs/superpowers/specs/2026-06-15-phermesd-macos-guest-design.md`](docs/superpowers/specs/2026-06-15-phermesd-macos-guest-design.md).
```

- [ ] **Step 4: Verify suite + commit**

```bash
uv run pytest -q
git add README.md CHANGELOG.md
git commit -m "docs(slice #5b): README + CHANGELOG note macOS BYOI guest support"
```

---

## Final checks (after M7)

```bash
cd phermesd && cargo test 2>&1 | grep "test result"
cd .. && uv run pytest -q
cd phermesd && cargo clippy --all-targets --all-features -- -D warnings
cd .. && uv run ruff check src/ tests/
uv run ty check src/phermes_build/
```

All green → invoke `superpowers:finishing-a-development-branch` to push `feat/phermesd-macos-guest:feat/phermesd-macos-guest` with explicit local:remote form and open the stacked PR with base = `feat/phermesd-windows-guest`.

---

## Self-review (controller's notes)

- **Spec coverage:** secrets.rs ✓ (M1), build_macos + dispatch ✓ (M2), `MissingAppleOsk` error variant ✓ (M2), vm.py constants + write_def + provision_disk ✓ (M3), `_vm_source` macos ✓ (M4), `--dev-apple-osk` + secret writing ✓ (M5), `_provision_macos_vm` + build() wiring ✓ (M6), README + CHANGELOG ✓ (M7), flip `test_unsupported_import_vm_flavor_errors_out` ✓ (M4).
- **Type consistency:** `MACOS_VMID` used in vm.py and matches spec (100). `MACOS_DEFAULT_*` constants consistent across plan. `QemuError::MissingAppleOsk { path, source }` matches spec. `SecretsError` variants consistent.
- **Placeholder scan:** no TBD/TODO; every code step has the literal code.
- **Test count progression:** Python 125 → 131 (M3) → 134 (M4) → 138 (M5) → 141 (M6). Rust: existing 75 → 81 (M1) → 86 (M2, +5 new −1 renamed).
