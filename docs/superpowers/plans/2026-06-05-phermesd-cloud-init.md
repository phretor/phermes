# phermesd Cloud-init NoCloud Seed Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** When `phermes-build` runs with `--dev-credentials --dev-ssh-pubkey <key>`, the assembled appliance ships `/var/lib/phermes/seed/linux.iso` (a NoCloud-labelled cloud-init seed), the Linux VM's def references it as a CDROM, and the guest boots configured with the operator's SSH key + `uv` installed.

**Architecture:** Two-language change. Rust side: slice #1's `DiskInterface` enum gains a `Cdrom` variant; `qemu.rs::build_linux` gains an arm emitting `-drive media=cdrom,readonly=on,…` + `-device ide-cd,drive=diskN`. Python side: new `cloud_init.py` module renders three YAML files and shells `genisoimage`; `vm.py::write_linux_def` accepts an optional `seed_iso_path` keyword; `cli.py` calls the seed generator when `--dev-credentials` is set and threads the path through. Production builds (no `--dev-credentials`) emit no seed.

**Tech Stack:** Rust (slice #1 def model + argv builder); Python 3.13 + uv + pytest (phermes-build); `genisoimage` (Debian apt) for ISO9660 assembly; cloud-init NoCloud datasource (operator-supplied Debian cloud image).

**Spec:** `docs/superpowers/specs/2026-06-05-phermesd-cloud-init-design.md`

**Prerequisite:** base the implementation branch on slice #6's tip (`feat/phermesd-host-image-mvp`, PR #20). #4a touches the rewritten-in-#6 `vm.py` and `cli.py`. If #20 is merged when implementation starts, base on `main`; otherwise stack on `feat/phermesd-host-image-mvp` (same pattern as #6 stacking on #19).

---

## File Structure

```
phermesd/src/
  config.rs           (modify)  # DiskInterface::Cdrom variant
  qemu.rs             (modify)  # build_linux Cdrom arm + golden test

src/phermes_build/
  cloud_init.py       (new)     # SeedConfig + 3 YAML renderers + write_seed_iso
  vm.py               (modify)  # SEED_DIR/LINUX_SEED_PATH consts; seed_iso_path keyword
  cli.py              (modify)  # _write_cloud_init_seed helper; thread seed path
Dockerfile            (modify)  # add genisoimage to apt set
tests/phermes_build/
  test_cloud_init.py  (new)
  test_vm.py          (modify)
  test_cli.py         (modify)
README.md             (modify)
CHANGELOG.md          (modify)
```

Test pattern (existing): `tests/phermes_build/test_<module>.py`; mock with `monkeypatch.setattr(<mod>, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")`.

---

### Task 1: `phermesd/src/config.rs` — add `DiskInterface::Cdrom`

**Files:**
- Modify: `phermesd/src/config.rs`

- [ ] **Step 1: Add a failing TOML round-trip test**

Append to the existing `#[cfg(test)] mod tests` block in `phermesd/src/config.rs`:

```rust
    #[test]
    fn disk_interface_cdrom_parses_from_toml() {
        let toml_text = r#"
flavor = "linux"
[resources]
memory_mib = 1024
vcpus = 1
[firmware]
ovmf_code = "/a"
ovmf_vars_template = "/b"
[[disk]]
path = "/var/lib/phermes/seed/linux.iso"
format = "raw"
interface = "cdrom"
[[net]]
bridge = "vmbr0"
"#;
        let def: VmDef = toml::from_str(toml_text).unwrap();
        assert_eq!(def.disk[0].interface, DiskInterface::Cdrom);
    }
```

- [ ] **Step 2: Run, verify fail (variant missing)**

Run: `cd phermesd && cargo test --lib config 2>&1 | head -15`
Expected: FAIL — `DiskInterface::Cdrom` does not exist.

- [ ] **Step 3: Extend the enum**

Edit `phermesd/src/config.rs`. Find the existing `DiskInterface` definition:

```rust
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiskInterface {
    #[default]
    VirtioScsi,
    VirtioBlk,
}
```

Add `Cdrom` as a third variant:

```rust
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiskInterface {
    #[default]
    VirtioScsi,
    VirtioBlk,
    Cdrom,
}
```

- [ ] **Step 4: Run the test + the whole config suite**

Run: `cd phermesd && cargo test --lib config`
Expected: PASS (existing + new test). The default stays `VirtioScsi`, so no existing test breaks.

- [ ] **Step 5: clippy clean**

Run: `cd phermesd && cargo clippy --all-targets --all-features -- -D warnings`
Expected: no warnings.

This task does NOT touch `qemu.rs` yet, so the build may fail there — that's Task 2. To make this commit self-contained, also add the bare match arm in `qemu.rs::build_linux` that prevents a non-exhaustive-match error: the next task replaces it.

In `phermesd/src/qemu.rs`, find the existing `match disk.interface { ... }` block in `build_linux`. Add a TEMPORARY placeholder arm at the END so it compiles:

```rust
        match disk.interface {
            DiskInterface::VirtioScsi => { /* existing virtio-scsi-pci + scsi-hd code */ }
            DiskInterface::VirtioBlk => { /* existing virtio-blk-pci code */ }
            DiskInterface::Cdrom => {
                // Implemented in Task 2.
                unimplemented!("DiskInterface::Cdrom is wired in Task 2")
            }
        }
```

`unimplemented!()` is permitted because the lint scope only denies it via clippy pedantic — confirm with the next step.

- [ ] **Step 6: Re-run clippy after the placeholder**

Run: `cd phermesd && cargo clippy --all-targets --all-features -- -D warnings`
Expected: clean OR clippy flags `unimplemented!` due to the `unimplemented = "deny"` lint. If it does, replace the `unimplemented!()` line with this concrete placeholder that satisfies the lint AND short-circuits Task 1's commit safely (no Cdrom in any test fixture yet, so this branch is unreachable in Task 1's test suite):

```rust
            DiskInterface::Cdrom => {
                // Implemented in Task 2.
                return Err(QemuError::UnsupportedFlavor(d.flavor));
            }
```

This returns the existing `UnsupportedFlavor` error if a `Cdrom` disk is encountered — incorrect long-term but unreachable until Task 2's test fixture exercises it; Task 2 replaces it with the real implementation.

- [ ] **Step 7: Run full Rust suite + commit**

Run:
```bash
cd phermesd && cargo test && cargo clippy --all-targets --all-features -- -D warnings
cd /home/u/dev/phermes/phermes
git add phermesd/src/config.rs phermesd/src/qemu.rs
git commit -m "feat(phermesd): DiskInterface::Cdrom variant (config + placeholder argv arm)"
```
Expected: all green; clippy clean.

---

### Task 2: `phermesd/src/qemu.rs` — emit `media=cdrom,readonly=on` + `ide-cd`

**Files:**
- Modify: `phermesd/src/qemu.rs`

- [ ] **Step 1: Write the failing golden test**

In `phermesd/src/qemu.rs`'s existing `#[cfg(test)] mod tests` block, append:

```rust
    #[test]
    fn linux_argv_with_cdrom_disk_emits_media_cdrom_and_ide_cd() {
        let mut vm = sample_vm();
        // Append a second disk: the cloud-init seed CDROM.
        vm.def.disk.push(Disk {
            path: "/var/lib/phermes/seed/linux.iso".into(),
            format: "raw".into(),
            interface: DiskInterface::Cdrom,
        });
        let argv = build_argv(&vm, &rt()).unwrap();
        // The seed appears as disk1 (the OS disk in sample_vm is disk0).
        assert!(argv.iter().any(|a| a == "media=cdrom,readonly=on,format=raw,file=/var/lib/phermes/seed/linux.iso,if=none,id=disk1"));
        assert!(argv.iter().any(|a| a == "ide-cd,drive=disk1"));
        // The seed CDROM does NOT add a virtio-scsi controller for itself
        // (only the existing virtio-scsi OS disk does).
        let scsi_controllers = argv.iter().filter(|a| a.as_str() == "virtio-scsi-pci,id=scsi0").count();
        assert_eq!(scsi_controllers, 1);
    }
```

- [ ] **Step 2: Run, verify fail**

Run: `cd phermesd && cargo test --lib qemu 2>&1 | head -20`
Expected: FAIL — the `Cdrom` arm returns `UnsupportedFlavor` (the Task 1 placeholder) or hits `unimplemented!()`, so `build_argv` returns Err and `.unwrap()` panics. The test's `argv.iter().any(...)` never runs.

- [ ] **Step 3: Replace the placeholder Cdrom arm**

In `phermesd/src/qemu.rs`, find the placeholder Cdrom arm from Task 1 and replace with the real implementation:

```rust
            DiskInterface::Cdrom => {
                pair(
                    &mut a,
                    "-drive",
                    format!(
                        "media=cdrom,readonly=on,format={},file={},if=none,id=disk{i}",
                        disk.format,
                        disk.path.display(),
                    ),
                );
                pair(&mut a, "-device", format!("ide-cd,drive=disk{i}"));
            }
```

The `scsi_controller_added` flag and other arms are unchanged.

- [ ] **Step 4: Run the test, verify pass**

Run: `cd phermesd && cargo test --lib qemu`
Expected: PASS (the new test + all existing qemu tests). The new test asserts both the `-drive` payload and the `-device ide-cd,...` line.

- [ ] **Step 5: clippy + whole suite**

Run:
```bash
cd phermesd && cargo clippy --all-targets --all-features -- -D warnings
cd phermesd && cargo test
```
Expected: clippy clean; whole suite green.

- [ ] **Step 6: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add phermesd/src/qemu.rs
git commit -m "feat(phermesd): emit ide-cd CDROM arg for DiskInterface::Cdrom"
```

---

### Task 3: `src/phermes_build/cloud_init.py` — `SeedConfig` + three YAML renderers

**Files:**
- Create: `src/phermes_build/cloud_init.py`
- Create: `tests/phermes_build/test_cloud_init.py`

This task creates the module with `SeedConfig` and the three pure-function renderers. `write_seed_iso` is Task 4. Follow TDD.

- [ ] **Step 1: Write failing tests**

Create `tests/phermes_build/test_cloud_init.py`:

```python
"""Tests for cloud_init.py — pure YAML renderers."""

from phermes_build import cloud_init as ci_mod


def _cfg(**overrides):
    base = {
        "ssh_authorized_keys": ["ssh-ed25519 AAAA...op@host"],
    }
    base.update(overrides)
    return ci_mod.SeedConfig(**base)


def test_seedconfig_defaults():
    cfg = _cfg()
    assert cfg.hostname == "phermes-linux"
    assert cfg.username == "dev"
    assert cfg.install_uv is True
    assert cfg.ssh_authorized_keys == ["ssh-ed25519 AAAA...op@host"]


def test_meta_data_yaml_has_instance_id_and_local_hostname():
    out = ci_mod.meta_data_yaml(_cfg())
    assert "instance-id: phermes-phermes-linux" in out
    assert "local-hostname: phermes-linux" in out


def test_meta_data_yaml_uses_custom_hostname():
    out = ci_mod.meta_data_yaml(_cfg(hostname="dev-box"))
    assert "instance-id: phermes-dev-box" in out
    assert "local-hostname: dev-box" in out


def test_user_data_yaml_is_cloud_config_with_key_only_login():
    out = ci_mod.user_data_yaml(_cfg())
    assert out.startswith("#cloud-config\n")
    # Key-only login enforced
    assert "ssh_pwauth: false" in out
    assert "disable_root: true" in out
    assert "lock_passwd: true" in out
    # user + key
    assert "name: dev" in out
    assert "ssh-ed25519 AAAA...op@host" in out
    # sudo + shell
    assert "NOPASSWD:ALL" in out
    assert "shell: /bin/bash" in out


def test_user_data_yaml_renders_multiple_keys():
    cfg = ci_mod.SeedConfig(
        ssh_authorized_keys=[
            "ssh-ed25519 AAAA...first",
            "ssh-ed25519 BBBB...second",
        ],
    )
    out = ci_mod.user_data_yaml(cfg)
    assert "ssh-ed25519 AAAA...first" in out
    assert "ssh-ed25519 BBBB...second" in out


def test_vendor_data_yaml_installs_uv_by_default():
    out = ci_mod.vendor_data_yaml(_cfg())
    assert out.startswith("#cloud-config\n")
    assert "package_update: true" in out
    assert "curl" in out
    assert "https://astral.sh/uv/install.sh" in out
    assert "UV_INSTALL_DIR=/usr/local/bin" in out


def test_vendor_data_yaml_is_empty_when_install_uv_false():
    out = ci_mod.vendor_data_yaml(_cfg(install_uv=False))
    assert out.startswith("#cloud-config\n")
    assert "uv" not in out
    assert "runcmd" not in out
```

- [ ] **Step 2: Run, verify ImportError**

Run: `uv run pytest tests/phermes_build/test_cloud_init.py -v 2>&1 | head -10`
Expected: ImportError — `phermes_build.cloud_init` does not exist.

- [ ] **Step 3: Implement the module**

Create `src/phermes_build/cloud_init.py`:

```python
"""Cloud-init NoCloud seed generation.

Builds a seed.iso (labelled CIDATA) that bootstraps a Linux guest at first boot
with a `dev` user, the operator's SSH key, DHCP, and `uv` (so the Hermes runtime
can run). The seed is written by phermes-build at install time and attached to
the guest as a CDROM (see slice #1's DiskInterface::Cdrom).

Linux-only. Windows + macOS don't use NoCloud — they return in #5.
"""

import os
import tempfile
from dataclasses import dataclass, field
from pathlib import Path

from phermes_build.runner import run_cmd


@dataclass(frozen=True)
class SeedConfig:
    """Input to seed generation. Populated by cli.py from build-time flags."""

    hostname: str = "phermes-linux"
    username: str = "dev"
    ssh_authorized_keys: list[str] = field(default_factory=list)
    install_uv: bool = True


def meta_data_yaml(cfg: SeedConfig) -> str:
    """/CIDATA/meta-data — instance-id + hostname."""
    return (
        f"instance-id: phermes-{cfg.hostname}\n"
        f"local-hostname: {cfg.hostname}\n"
    )


def user_data_yaml(cfg: SeedConfig) -> str:
    """/CIDATA/user-data — user + SSH key + key-only login + disable_root."""
    keys = "\n".join(f"      - {k.strip()}" for k in cfg.ssh_authorized_keys)
    return (
        "#cloud-config\n"
        "ssh_pwauth: false\n"
        "disable_root: true\n"
        f"hostname: {cfg.hostname}\n"
        "users:\n"
        f"  - name: {cfg.username}\n"
        "    sudo: ALL=(ALL) NOPASSWD:ALL\n"
        "    shell: /bin/bash\n"
        "    lock_passwd: true\n"
        "    ssh_authorized_keys:\n"
        f"{keys}\n"
    )


def vendor_data_yaml(cfg: SeedConfig) -> str:
    """/CIDATA/vendor-data — installs uv if cfg.install_uv."""
    if not cfg.install_uv:
        return "#cloud-config\n{}\n"
    return (
        "#cloud-config\n"
        "package_update: true\n"
        "packages:\n"
        "  - curl\n"
        "  - ca-certificates\n"
        "runcmd:\n"
        "  - [ sh, -c, "
        "'curl -LsSf https://astral.sh/uv/install.sh | "
        "env UV_INSTALL_DIR=/usr/local/bin sh' ]\n"
    )
```

(`write_seed_iso` is Task 4. The `os`/`tempfile`/`Path`/`run_cmd` imports anticipate it; if `ruff` flags them as unused, comment "# used by write_seed_iso in Task 4" — but slice #6 saw ruff was OK with imports actually used by later additions. Run lint and fix only if it complains.)

- [ ] **Step 4: Run tests, verify pass**

Run: `uv run pytest tests/phermes_build/test_cloud_init.py -v`
Expected: PASS (7 tests).

- [ ] **Step 5: lint + typecheck + whole suite**

Run:
```bash
uv run pytest -q
uv run ruff check src/phermes_build/cloud_init.py tests/phermes_build/test_cloud_init.py
uv run ty check src/phermes_build/cloud_init.py
```
Expected: clean; whole Python suite green. If ruff flags unused imports (`os`, `tempfile`, `Path`, `run_cmd`), remove them — Task 4 re-adds them when it needs them.

- [ ] **Step 6: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add src/phermes_build/cloud_init.py tests/phermes_build/test_cloud_init.py
git commit -m "feat(builder): cloud_init module — SeedConfig + meta/user/vendor renderers"
```

---

### Task 4: `cloud_init.py::write_seed_iso` + empty-key defensive check

**Files:**
- Modify: `src/phermes_build/cloud_init.py`
- Modify: `tests/phermes_build/test_cloud_init.py`

- [ ] **Step 1: Append failing tests**

Append to `tests/phermes_build/test_cloud_init.py`:

```python
import pytest


def _capture(monkeypatch):
    calls = []
    monkeypatch.setattr(ci_mod, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")
    return calls


def test_write_seed_iso_invokes_genisoimage_with_cidata_volid(monkeypatch, tmp_path):
    calls = _capture(monkeypatch)
    out = str(tmp_path / "seed.iso")
    ci_mod.write_seed_iso(out, _cfg())
    assert len(calls) == 1
    argv = calls[0]
    assert argv[0] == "genisoimage"
    # -output <out>
    out_idx = argv.index("-output")
    assert argv[out_idx + 1] == out
    # -volid CIDATA (the label NoCloud's datasource scans for)
    vol_idx = argv.index("-volid")
    assert argv[vol_idx + 1] == "CIDATA"
    # -joliet + -rock present
    assert "-joliet" in argv
    assert "-rock" in argv
    # The three seed files appear by name at the tail
    tail = " ".join(argv[-3:])
    assert tail.endswith("meta-data user-data vendor-data") \
        or "meta-data" in tail and "user-data" in tail and "vendor-data" in tail


def test_write_seed_iso_writes_three_files_into_tempdir(monkeypatch, tmp_path):
    """Capture the temp-dir paths that genisoimage is told to pack, and assert
    each one exists with the expected first line when genisoimage runs."""
    captured: dict = {}

    def fake_run_cmd(cmd, **kw):
        # The three seed files are the last three positional args.
        captured["meta"] = Path(cmd[-3])
        captured["user"] = Path(cmd[-2])
        captured["vendor"] = Path(cmd[-1])
        for p in captured.values():
            captured[f"{p.name}_text"] = p.read_text()
        return ""

    from pathlib import Path
    monkeypatch.setattr(ci_mod, "run_cmd", fake_run_cmd)
    ci_mod.write_seed_iso(str(tmp_path / "seed.iso"), _cfg())
    assert captured["meta-data_text"].startswith("instance-id:")
    assert captured["user-data_text"].startswith("#cloud-config\n")
    assert "ssh-ed25519 AAAA...op@host" in captured["user-data_text"]
    assert captured["vendor-data_text"].startswith("#cloud-config\n")


def test_write_seed_iso_creates_output_parent_directory(monkeypatch, tmp_path):
    monkeypatch.setattr(ci_mod, "run_cmd", lambda cmd, **kw: "")
    nested = tmp_path / "deeply" / "nested" / "seed.iso"
    ci_mod.write_seed_iso(str(nested), _cfg())
    assert nested.parent.exists()


def test_write_seed_iso_rejects_empty_key_list(monkeypatch, tmp_path):
    calls = _capture(monkeypatch)
    cfg = ci_mod.SeedConfig(ssh_authorized_keys=[])
    with pytest.raises(ValueError, match="ssh_authorized_keys"):
        ci_mod.write_seed_iso(str(tmp_path / "seed.iso"), cfg)
    # No genisoimage call happened
    assert calls == []
```

- [ ] **Step 2: Run, verify fail**

Run: `uv run pytest tests/phermes_build/test_cloud_init.py -v 2>&1 | head -10`
Expected: FAIL — `write_seed_iso` not defined.

- [ ] **Step 3: Add write_seed_iso**

Append to `src/phermes_build/cloud_init.py`:

```python
def write_seed_iso(out_path: str, cfg: SeedConfig) -> None:
    """Render meta-data/user-data/vendor-data and pack them into a CIDATA ISO.

    The resulting ISO9660 image has volume label CIDATA so cloud-init's NoCloud
    datasource auto-detects it on any block device.

    Raises ValueError if cfg.ssh_authorized_keys is empty (a key-only login with
    zero keys would lock the operator out of the dev VM).
    """
    if not cfg.ssh_authorized_keys:
        raise ValueError(
            "SeedConfig.ssh_authorized_keys must contain at least one key"
        )
    parent = os.path.dirname(out_path)
    if parent:
        os.makedirs(parent, exist_ok=True)
    with tempfile.TemporaryDirectory() as tmp:
        tmp_dir = Path(tmp)
        (tmp_dir / "meta-data").write_text(meta_data_yaml(cfg))
        (tmp_dir / "user-data").write_text(user_data_yaml(cfg))
        (tmp_dir / "vendor-data").write_text(vendor_data_yaml(cfg))
        run_cmd(
            [
                "genisoimage",
                "-output",
                out_path,
                "-volid",
                "CIDATA",
                "-joliet",
                "-rock",
                str(tmp_dir / "meta-data"),
                str(tmp_dir / "user-data"),
                str(tmp_dir / "vendor-data"),
            ]
        )
```

- [ ] **Step 4: Run tests, verify pass**

Run: `uv run pytest tests/phermes_build/test_cloud_init.py -v`
Expected: PASS (11 tests total: 7 from Task 3 + 4 new).

- [ ] **Step 5: lint + typecheck + whole suite**

Run:
```bash
uv run pytest -q
uv run ruff check src/phermes_build/cloud_init.py tests/phermes_build/test_cloud_init.py
uv run ty check src/phermes_build/cloud_init.py
```
Expected: clean; whole suite green.

- [ ] **Step 6: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add src/phermes_build/cloud_init.py tests/phermes_build/test_cloud_init.py
git commit -m "feat(builder): cloud_init.write_seed_iso (genisoimage shell-out + defensive check)"
```

---

### Task 5: `Dockerfile` — add `genisoimage` to apt set

**Files:**
- Modify: `Dockerfile`

- [ ] **Step 1: Edit Dockerfile**

Find the runtime-stage apt-get block in `Dockerfile` (the one in the `FROM debian:bookworm-slim` stage). The current set (post-#6):

```dockerfile
RUN apt-get update && apt-get install -y --no-install-recommends \
    btrfs-progs \
    cryptsetup \
    debootstrap \
    dosfstools \
    exfatprogs \
    fdisk \
    lvm2 \
    udev \
    util-linux \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
```

Add `genisoimage` in alphabetical order (between `exfatprogs` and `fdisk`):

```dockerfile
RUN apt-get update && apt-get install -y --no-install-recommends \
    btrfs-progs \
    cryptsetup \
    debootstrap \
    dosfstools \
    exfatprogs \
    fdisk \
    genisoimage \
    lvm2 \
    udev \
    util-linux \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
```

- [ ] **Step 2: Rebuild image and verify the binary lands**

Run:
```bash
cd /home/u/dev/phermes/phermes && just docker-build 2>&1 | tail -3
docker run --rm --entrypoint /usr/bin/which phermes-build genisoimage 2>&1
```
Expected: image builds; `which` prints `/usr/bin/genisoimage`.

- [ ] **Step 3: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add Dockerfile
git commit -m "build(builder): add genisoimage to phermes-build image for NoCloud seed"
```

---

### Task 6: `vm.py` — `seed_iso_path` keyword + constants

**Files:**
- Modify: `src/phermes_build/vm.py`
- Modify: `tests/phermes_build/test_vm.py`

- [ ] **Step 1: Append failing tests**

Append to `tests/phermes_build/test_vm.py`:

```python
def test_constants_for_seed_paths_exposed():
    assert vm_mod.SEED_DIR == "/var/lib/phermes/seed"
    assert vm_mod.LINUX_SEED_PATH == "/var/lib/phermes/seed/linux.iso"


def test_write_linux_def_omits_seed_cdrom_when_seed_iso_path_is_none(tmp_path):
    chroot = str(tmp_path / "chroot")
    import os as _os
    _os.makedirs(chroot)
    vm_mod.write_linux_def(chroot)  # default: no seed
    content = (tmp_path / "chroot" / "etc/phermes/vms/linux.toml").read_text()
    # The OS disk is present
    assert 'path = "/dev/pve/vm-102-disk-0"' in content
    # No CDROM
    assert "cdrom" not in content
    assert "seed" not in content


def test_write_linux_def_emits_seed_cdrom_when_seed_iso_path_given(tmp_path):
    chroot = str(tmp_path / "chroot")
    import os as _os
    _os.makedirs(chroot)
    vm_mod.write_linux_def(chroot, seed_iso_path="/var/lib/phermes/seed/linux.iso")
    content = (tmp_path / "chroot" / "etc/phermes/vms/linux.toml").read_text()
    # Both disks present
    assert 'path = "/dev/pve/vm-102-disk-0"' in content
    assert 'path = "/var/lib/phermes/seed/linux.iso"' in content
    assert 'format = "raw"' in content
    assert 'interface = "cdrom"' in content
    # OS disk still virtio-scsi
    assert 'interface = "virtio-scsi"' in content
```

- [ ] **Step 2: Run, verify fail**

Run: `uv run pytest tests/phermes_build/test_vm.py -v 2>&1 | head -20`
Expected: FAIL — `SEED_DIR`/`LINUX_SEED_PATH` not defined; `write_linux_def` doesn't accept `seed_iso_path`.

- [ ] **Step 3: Modify src/phermes_build/vm.py**

(a) Add the new constants near the existing `LINUX_VMID` block (alphabetical/grouped — pick a sensible placement):

```python
SEED_DIR = "/var/lib/phermes/seed"
LINUX_SEED_PATH = f"{SEED_DIR}/linux.iso"
```

(b) Find `_linux_def_text(*, memory_mib: int, vcpus: int) -> str:` and add a `seed_iso_path: str | None = None` keyword:

```python
def _linux_def_text(*, memory_mib: int, vcpus: int, seed_iso_path: str | None) -> str:
    """Render /etc/phermes/vms/linux.toml content.

    When ``seed_iso_path`` is set, a second [[disk]] block is appended for the
    cloud-init NoCloud CDROM. When None (production), no CDROM is emitted.
    """
    text = (
        f'flavor = "linux"\n'
        f"[resources]\n"
        f"memory_mib = {memory_mib}\n"
        f"vcpus = {vcpus}\n"
        f'cpu = "host"\n'
        f"[firmware]\n"
        f'ovmf_code = "/usr/share/OVMF/OVMF_CODE.fd"\n'
        f'ovmf_vars_template = "/usr/share/OVMF/OVMF_VARS.fd"\n'
        f"[[disk]]\n"
        f'path = "/dev/{STORAGE_VG}/vm-{LINUX_VMID}-disk-0"\n'
        f'format = "raw"\n'
        f'interface = "virtio-scsi"\n'
    )
    if seed_iso_path is not None:
        text += (
            f"[[disk]]\n"
            f'path = "{seed_iso_path}"\n'
            f'format = "raw"\n'
            f'interface = "cdrom"\n'
        )
    text += (
        f"[[net]]\n"
        f'bridge = "vmbr0"\n'
        f'model = "virtio-net"\n'
        f"[console]\n"
        f"serial = true\n"
        f"vnc = true\n"
    )
    return text
```

(c) Update `write_linux_def`'s signature to accept and forward the new keyword:

```python
def write_linux_def(
    chroot_mount: str,
    *,
    memory_mib: int = DEFAULT_MEMORY_MIB,
    vcpus: int = DEFAULT_VCPUS,
    seed_iso_path: str | None = None,
) -> None:
    """Write /etc/phermes/vms/linux.toml inside the chroot.

    If ``seed_iso_path`` is set, the def references it as a CDROM [[disk]].
    """
    vms_dir = os.path.join(chroot_mount, "etc/phermes/vms")
    os.makedirs(vms_dir, exist_ok=True)
    def_path = os.path.join(vms_dir, "linux.toml")
    with open(def_path, "w") as f:
        f.write(_linux_def_text(
            memory_mib=memory_mib,
            vcpus=vcpus,
            seed_iso_path=seed_iso_path,
        ))
```

- [ ] **Step 4: Run vm tests**

Run: `uv run pytest tests/phermes_build/test_vm.py -v`
Expected: PASS — existing tests + the 3 new ones.

- [ ] **Step 5: lint + typecheck + whole suite**

Run:
```bash
uv run pytest -q
uv run ruff check src/phermes_build/vm.py tests/phermes_build/test_vm.py
uv run ty check src/phermes_build/vm.py
```
Expected: clean; whole suite green.

- [ ] **Step 6: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add src/phermes_build/vm.py tests/phermes_build/test_vm.py
git commit -m "feat(builder): vm.write_linux_def(seed_iso_path) + SEED_DIR/LINUX_SEED_PATH"
```

---

### Task 7: `cli.py` — `_write_cloud_init_seed` helper + threading

**Files:**
- Modify: `src/phermes_build/cli.py`
- Modify: `tests/phermes_build/test_cli.py`

- [ ] **Step 1: Inspect current cli.py**

Run:
```bash
grep -nE "_provision_linux_vm|_linux_source|dev_credentials|dev_ssh_pubkey|--no-vm|_write_cloud_init_seed|cloud_init|seed_iso_path" /home/u/dev/phermes/phermes/src/phermes_build/cli.py
```

Note the current shape:
- `def _provision_linux_vm(source: str | None) -> None:` (from slice #6)
- `def _linux_source(import_vm_args: list[str]) -> str | None:` (from slice #6)
- `build(...)` accepts `dev_credentials: bool`, `dev_ssh_pubkey: str | None`, `no_vm: bool`, `import_vm: list[str]`.

- [ ] **Step 2: Add failing tests**

Append to `tests/phermes_build/test_cli.py`. Re-use the existing `runner = CliRunner()` and the imports the file already has; if not present, add `from typer.testing import CliRunner` and `from phermes_build import cli as cli_mod`.

```python
def test_no_seed_when_dev_credentials_absent(monkeypatch):
    """Production-style build: no --dev-credentials -> no cloud-init seed."""
    seen: dict = {}
    monkeypatch.setattr(cli_mod, "_write_cloud_init_seed", lambda key: seen.setdefault("called_with", key))
    # No-op the rest of the build so it short-circuits.
    for helper in ("_setup_luks", "_setup_lvm", "_setup_btrfs", "_setup_exfat",
                   "_install_minimal_host", "_configure_host", "_setup_credentials",
                   "_write_firstboot", "_provision_linux_vm", "_partition"):
        monkeypatch.setattr(cli_mod, helper, lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "plan_disk_layout", lambda *a, **k: object())

    result = runner.invoke(cli_mod.app, ["/dev/loop0", "--import-vm", "linux=/tmp/x.qcow2"])
    assert result.exit_code == 0, result.stdout
    # Helper called with None (no key => no seed)
    assert seen.get("called_with") is None


def test_seed_written_when_dev_credentials_and_key_set(monkeypatch):
    """Dev build: --dev-credentials + --dev-ssh-pubkey -> seed generated."""
    seen: dict = {}
    monkeypatch.setattr(cli_mod, "_write_cloud_init_seed", lambda key: seen.setdefault("called_with", key) or "/var/lib/phermes/seed/linux.iso")
    monkeypatch.setattr(cli_mod, "_provision_linux_vm", lambda source, seed_iso_path: seen.update({"source": source, "seed": seed_iso_path}))
    for helper in ("_setup_luks", "_setup_lvm", "_setup_btrfs", "_setup_exfat",
                   "_install_minimal_host", "_configure_host", "_setup_credentials",
                   "_write_firstboot", "_partition"):
        monkeypatch.setattr(cli_mod, helper, lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "plan_disk_layout", lambda *a, **k: object())

    result = runner.invoke(cli_mod.app, [
        "/dev/loop0",
        "--dev-credentials",
        "--dev-ssh-pubkey", "ssh-ed25519 AAAA...op@host",
        "--import-vm", "linux=/tmp/x.qcow2",
    ])
    assert result.exit_code == 0, result.stdout
    assert seen.get("called_with") == "ssh-ed25519 AAAA...op@host"
    assert seen.get("seed") == "/var/lib/phermes/seed/linux.iso"


def test_no_seed_when_no_vm(monkeypatch):
    """--no-vm: even with --dev-credentials, no seed is written (no VM to seed)."""
    seen: dict = {}

    def fake_seed(key):
        seen["seed_called"] = True
        return None

    monkeypatch.setattr(cli_mod, "_write_cloud_init_seed", fake_seed)
    monkeypatch.setattr(cli_mod, "_provision_linux_vm", lambda *a, **k: seen.setdefault("provision_called", True))
    for helper in ("_setup_luks", "_setup_lvm", "_setup_btrfs", "_setup_exfat",
                   "_install_minimal_host", "_configure_host", "_setup_credentials",
                   "_write_firstboot", "_partition"):
        monkeypatch.setattr(cli_mod, helper, lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "plan_disk_layout", lambda *a, **k: object())

    result = runner.invoke(cli_mod.app, [
        "/dev/loop0",
        "--dev-credentials",
        "--dev-ssh-pubkey", "ssh-ed25519 AAAA...op@host",
        "--no-vm",
    ])
    assert result.exit_code == 0, result.stdout
    # Provisioning was skipped, so the seed helper was never invoked.
    assert "seed_called" not in seen
    assert "provision_called" not in seen


def test_write_cloud_init_seed_returns_none_when_no_key():
    assert cli_mod._write_cloud_init_seed(None) is None


def test_write_cloud_init_seed_calls_cloud_init_when_key_present(monkeypatch, tmp_path):
    """_write_cloud_init_seed shells the work to cloud_init.write_seed_iso and
    returns LINUX_SEED_PATH (the guest path, NOT the chroot path)."""
    captured: dict = {}

    from phermes_build import cloud_init as ci_mod
    from phermes_build import vm as vm_mod_local

    def fake_write_seed_iso(out_path: str, cfg) -> None:
        captured["out"] = out_path
        captured["keys"] = cfg.ssh_authorized_keys

    monkeypatch.setattr(ci_mod, "write_seed_iso", fake_write_seed_iso)
    # Avoid touching /mnt/pve-root in the test; redirect via the module constant.
    monkeypatch.setattr(cli_mod, "PVE_ROOT_MOUNT", str(tmp_path))

    out = cli_mod._write_cloud_init_seed("ssh-ed25519 AAAA...op@host")
    assert out == vm_mod_local.LINUX_SEED_PATH
    assert captured["out"] == str(tmp_path / "var/lib/phermes/seed/linux.iso")
    assert captured["keys"] == ["ssh-ed25519 AAAA...op@host"]
```

- [ ] **Step 3: Run, verify fail**

Run: `uv run pytest tests/phermes_build/test_cli.py -v 2>&1 | tail -20`
Expected: FAIL — `_write_cloud_init_seed` not found; `_provision_linux_vm` doesn't accept `seed_iso_path`.

- [ ] **Step 4: Edit src/phermes_build/cli.py**

(a) **Imports.** Ensure `cloud_init` is imported. Find the existing `from phermes_build import ... vm as vm_mod ...` block and add `cloud_init` to it (alphabetical order):

```python
from phermes_build import (
    ...
    cloud_init,
    ...
    vm as vm_mod,
)
```

(b) **Add `_write_cloud_init_seed`** at module level (place near `_linux_source`):

```python
def _write_cloud_init_seed(dev_ssh_pubkey: str | None) -> str | None:
    """Generate /var/lib/phermes/seed/linux.iso in the chroot.

    Returns the on-guest path (NOT the chroot path) for use in linux.toml, or
    None when no seed should ship (production builds or no key supplied).
    """
    if not dev_ssh_pubkey:
        return None
    cfg = cloud_init.SeedConfig(ssh_authorized_keys=[dev_ssh_pubkey])
    chroot_path = os.path.join(PVE_ROOT_MOUNT, vm_mod.LINUX_SEED_PATH.lstrip("/"))
    cloud_init.write_seed_iso(chroot_path, cfg)
    return vm_mod.LINUX_SEED_PATH
```

(If `os` isn't already imported at the top, add `import os`.)

(c) **Update `_provision_linux_vm`'s signature**:

```python
def _provision_linux_vm(source: str | None, seed_iso_path: str | None) -> None:
    vm_mod.write_linux_def(PVE_ROOT_MOUNT, seed_iso_path=seed_iso_path)
    vm_mod.provision_linux_disk(source=source)
```

(d) **In `build()`**, replace the current `_provision_linux_vm` step with the seed-aware form. Locate the existing block (looks something like `if not no_vm: os_steps.append(("Provisioning Linux VM", lambda: _provision_linux_vm(source=_linux_source(import_vm))))`) and replace with:

```python
        if not no_vm:
            seed = _write_cloud_init_seed(dev_ssh_pubkey if dev_credentials else None)
            os_steps.append(
                (
                    "Provisioning Linux VM",
                    lambda: _provision_linux_vm(
                        source=_linux_source(import_vm),
                        seed_iso_path=seed,
                    ),
                )
            )
```

- [ ] **Step 5: Run tests, verify pass**

Run: `uv run pytest tests/phermes_build/test_cli.py -v`
Expected: PASS — existing tests + the 5 new ones.

- [ ] **Step 6: lint + typecheck + whole suite**

Run:
```bash
uv run pytest -q
uv run ruff check src/phermes_build/cli.py tests/phermes_build/test_cli.py
uv run ty check src/phermes_build/cli.py
```
Expected: clean; whole suite green.

- [ ] **Step 7: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add src/phermes_build/cli.py tests/phermes_build/test_cli.py
git commit -m "feat(builder): cli writes cloud-init seed when --dev-credentials + key set"
```

---

### Task 8: README + CHANGELOG

**Files:**
- Modify: `README.md`
- Modify: `CHANGELOG.md`

- [ ] **Step 1: Update README**

In the `### phermesd (in development)` subsection of `README.md`, append a sentence about #4a:

```
Slice #4a (implemented): dev builds (`--dev-credentials --dev-ssh-pubkey <key>`) now ship a cloud-init NoCloud seed at `/var/lib/phermes/seed/linux.iso`, attached to the Linux guest as a CDROM. First boot configures a `dev` user with the operator's SSH key, brings up DHCP on `vmbr0`, and installs `uv` so Hermes can run. Production builds (no `--dev-credentials`) ship no seed. Design: [`docs/superpowers/specs/2026-06-05-phermesd-cloud-init-design.md`](docs/superpowers/specs/2026-06-05-phermesd-cloud-init-design.md).
```

- [ ] **Step 2: Update CHANGELOG**

Under `## [Unreleased]` → `### Added`, append:

```markdown
- `phermes-build` cloud-init NoCloud seed (slice #4a): when run with
  `--dev-credentials --dev-ssh-pubkey <key>`, a `seed.iso` (label CIDATA) is
  generated and attached to the Linux guest as a CDROM. The seed contains a
  `dev` user with the operator's SSH key (key-only login, locked password),
  DHCP, and a `uv` installer in vendor-data. Slice #1's `DiskInterface` gains
  a `Cdrom` variant; `qemu.rs` emits `-drive media=cdrom,readonly=on,…` +
  `-device ide-cd,…`. Production builds ship no seed.
```

- [ ] **Step 3: Final full suite**

```bash
cd /home/u/dev/phermes/phermes && uv run pytest -q && cd phermesd && cargo test 2>&1 | grep "test result: ok" | wc -l
```
Expected: Python all-green; Rust ≥ 11 test binaries all green (1 added — `qmp_wire`, `qga_wire`, `supervisor_lifecycle`, `e2e_boot` ignored, `storage_lifecycle`, `storage_integration` ignored, and the lib tests, plus existing/new).

- [ ] **Step 4: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add README.md CHANGELOG.md
git commit -m "docs: phermesd cloud-init NoCloud seed (slice #4a)"
```

---

## Out of Scope (carries to later slices)

- Console proxy / web UI / noVNC / SSH-bridged serial → slice #4b.
- macOS + Windows guests → slice #5 (neither uses NoCloud).
- Runtime seed regeneration (operator-editable config + `phermesctl seed regenerate`).
- Static IP / per-NIC network customization in user-data (DHCP only).
- Per-VM seed config (only Linux gets a seed in MVP).
- CDROM eject / swappable media.
- Bundling a Debian cloud qcow2 in the phermes-build image (operator supplies via `--import-vm linux=<path>`).

---

## Self-Review

**1. Spec coverage** — every spec section maps to a task:

| Spec section / requirement | Task(s) |
|---|---|
| Rust `DiskInterface::Cdrom` variant + serde round-trip | 1 |
| Rust `build_linux` Cdrom arm emitting `-drive media=cdrom,readonly=on,…` + `ide-cd` | 2 |
| Slice #2 storage non-interaction | implicit (verified by qemu test in 2 — `Storage` never reads the CDROM disk) |
| `cloud_init.SeedConfig` dataclass + defaults | 3 |
| `meta_data_yaml` / `user_data_yaml` / `vendor_data_yaml` pure renderers | 3 |
| `write_seed_iso` shells `genisoimage`, writes three temp files | 4 |
| `write_seed_iso` empty-key defensive check | 4 |
| Dockerfile gains `genisoimage` | 5 |
| `vm.py`: `SEED_DIR`, `LINUX_SEED_PATH`, `seed_iso_path` keyword | 6 |
| `vm.py` def text: CDROM block when seed set, omitted otherwise | 6 |
| `cli.py` `_write_cloud_init_seed` helper + `_provision_linux_vm` threading | 7 |
| Production-no-seed invariant (no `--dev-credentials` => no CDROM, no seed.iso) | 7 (assertion test) |
| `--no-vm` short-circuit (no seed even with `--dev-credentials`) | 7 (assertion test) |
| Errors: ValueError on empty keys, CalledProcessError on genisoimage failure, FileNotFoundError on missing binary | 4 (ValueError); the other two propagate through existing `run_cmd` machinery |
| README + CHANGELOG | 8 |
| Out-of-scope items | listed above |

The "Success criterion" (smoke recipe end-to-end with a Debian cloud qcow2) is operator-verified post-merge — no task can run it in CI without root + KVM + a bundled cloud image.

**2. Placeholder scan** — none. Task 1's "TEMPORARY placeholder" arm in `qemu.rs` is concrete (`return Err(QemuError::UnsupportedFlavor(d.flavor))`) with rationale and explicit replacement in Task 2; Task 3 notes "imports anticipate Task 4" with concrete fallback ("remove them if ruff complains; Task 4 re-adds"). No "TBD"/"TODO"/"Add appropriate error handling" anywhere.

**3. Type consistency** — `SeedConfig` (hostname/username/ssh_authorized_keys/install_uv), `meta_data_yaml`/`user_data_yaml`/`vendor_data_yaml`/`write_seed_iso(out_path, cfg)`, `SEED_DIR`/`LINUX_SEED_PATH`, `_write_cloud_init_seed(dev_ssh_pubkey) -> str | None`, `_provision_linux_vm(source, seed_iso_path)`, `write_linux_def(..., seed_iso_path=…)`, `DiskInterface::Cdrom`, `interface = "cdrom"` — names and signatures match across tasks and tests.
