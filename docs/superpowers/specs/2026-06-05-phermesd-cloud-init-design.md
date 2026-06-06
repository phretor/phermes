# phermesd — Cloud-init NoCloud Seed (Design)

**Date:** 2026-06-05
**Status:** Draft — pending implementation plan
**Sub-project:** #4a of the `phermesd` replacement for Proxmox VE

---

## Context & motivation

[Slice #6 MVP](2026-06-04-phermesd-host-image-mvp-design.md) ships a bootable phermesd-based
appliance, but a freshly-provisioned Linux guest comes up with no user, no SSH key, and no
runtime — useless without manual install. The deleted `node_vm.py` used cloud-init NoCloud
to bootstrap a Debian cloud image into a usable Hermes node; this sub-project restores that
capability on top of the new orchestrator.

#4 was decomposed into two independent slices during brainstorming:
- **#4a — cloud-init NoCloud seed generation** (this spec). Installs an instance ID, the
  operator's SSH key, a `dev` user, and the `uv` runtime so Hermes can run.
- **#4b — console proxy** (later). Bridges phermesd's per-VM VNC/serial UDS sockets to a
  network-reachable interface.

Linux-only for both #4a and #4b. macOS + Windows return in #5 (neither uses NoCloud).

## Decisions (resolved during brainstorming)

| Decision | Choice |
|---|---|
| Slice scope | **#4a = cloud-init only; #4b = console proxy** (split into two slices, two PRs) |
| Seed generation | **At install time by `phermes-build`** (mirrors deleted node_vm.py). Re-configuration requires a re-build for the MVP; runtime regeneration is deferred. |
| Guest delivery | **Extend slice #1's `DiskInterface` enum with a `Cdrom` variant.** Linux.toml gets a second `[[disk]]` block referencing `seed.iso` with `interface = "cdrom"`. cloud-init's NoCloud datasource auto-detects by `CIDATA` filesystem label, not by bus. |
| Content | **`dev` user (key-only, locked password), operator's SSH key, DHCP, `uv` via vendor-data.** Only emitted when `--dev-credentials` is set; production builds ship NO seed (preserves the [shipped-credentials invariant](../../threat-model.md)). |

## Scope boundary

**In:**
- New `src/phermes_build/cloud_init.py` Python module: `SeedConfig` dataclass + three pure
  YAML renderers (`meta_data_yaml`, `user_data_yaml`, `vendor_data_yaml`) + `write_seed_iso`
  (shells `genisoimage`).
- `phermesd/src/config.rs`: `DiskInterface` enum gains `Cdrom` variant.
- `phermesd/src/qemu.rs`: `build_linux` disk loop gains a `Cdrom` arm emitting
  `-drive media=cdrom,readonly=on,...` + `-device ide-cd,...`.
- `Dockerfile` (phermes-build): add `genisoimage` to the apt set.
- `src/phermes_build/vm.py`: `write_linux_def` gains a `seed_iso_path` keyword; constants
  `SEED_DIR = "/var/lib/phermes/seed"`, `LINUX_SEED_PATH = f"{SEED_DIR}/linux.iso"`.
- `src/phermes_build/cli.py`: a new `_write_cloud_init_seed` helper, threaded into
  `_provision_linux_vm` when `--dev-credentials` is set and `--no-vm` is not.

**Out (deferred):**
- Console proxy / web UI / noVNC / SSH-bridged serial (slice #4b).
- macOS + Windows guests (slice #5; neither uses NoCloud).
- Runtime seed regeneration (operator edits a config → `phermesctl seed regenerate`).
- Static IP / per-NIC network customization in user-data (DHCP only).
- Per-VM seed config (only the Linux VM has a seed; the `cloud_init` module is reusable
  but `cli.py` only invokes it for Linux).
- CDROM eject / swappable media.

**Prerequisite:** the implementation branch must stack on slice #6's tip
(`feat/phermesd-host-image-mvp`, PR #20), not bare `main`. #4a touches:
- `vm.py` — the rewritten Linux-only form from slice #6
- `cli.py` — the new `_provision_linux_vm` orchestration step from slice #6
- `phermesd/src/config.rs` + `phermesd/src/qemu.rs` — slice #1's def model, present on every
  branch since #17 merged
- `Dockerfile` — slice #6's multi-stage form

## Architecture & file structure

### New (Python)

| File | Responsibility |
|---|---|
| `src/phermes_build/cloud_init.py` | `SeedConfig` dataclass; `meta_data_yaml()` / `user_data_yaml()` / `vendor_data_yaml()` pure renderers; `write_seed_iso(out_path, cfg)` shells `genisoimage` |
| `tests/phermes_build/test_cloud_init.py` | YAML render tests; `write_seed_iso` test mocking `run_cmd`; empty-key-list rejection |

### Modified (Python)

| File | Change |
|---|---|
| `Dockerfile` | Add `genisoimage` to the runtime apt set |
| `src/phermes_build/vm.py` | Add `seed_iso_path: str | None = None` keyword to `write_linux_def`; when set, append a second `[[disk]]` block; new constants `SEED_DIR` / `LINUX_SEED_PATH` |
| `src/phermes_build/cli.py` | New helper `_write_cloud_init_seed(dev_ssh_pubkey) -> str | None`; thread the seed path through `_provision_linux_vm` |
| `tests/phermes_build/test_vm.py` | New tests: seed-iso block present when `seed_iso_path` given, absent otherwise |
| `tests/phermes_build/test_cli.py` | New tests: seed written when `--dev-credentials` + `--dev-ssh-pubkey`; NOT written without `--dev-credentials`; NOT written with `--no-vm` |

### Modified (Rust)

| File | Change |
|---|---|
| `phermesd/src/config.rs` | `DiskInterface` enum gains `Cdrom` variant (kebab-case serde → TOML `"cdrom"`). Default unchanged. |
| `phermesd/src/qemu.rs` | `build_linux`'s disk loop gains a `Cdrom` arm |
| `phermesd/src/qemu.rs` tests | One golden test: `cdrom` disk emits `media=cdrom,readonly=on` + `ide-cd` device |
| `phermesd/src/config.rs` tests | Round-trip TOML `interface = "cdrom"` ↔ `DiskInterface::Cdrom` |

### Modified (docs)

| File | Change |
|---|---|
| `README.md`, `CHANGELOG.md` | Note that dev builds ship a cloud-init seed; production ships none |

### On-appliance filesystem layout

```
/var/lib/phermes/seed/linux.iso     ← written by phermes-build (dev builds only)
/etc/phermes/vms/linux.toml         ← references seed.iso as a CDROM [[disk]] (dev) or omits it (prod)
```

`/var/lib/phermes/seed/` mirrors slice #2's `/var/lib/phermes/images/` convention; both are
under `/var/lib/phermes/`.

## `cloud_init.py` — seed generation

`SeedConfig` is the input contract:

```python
@dataclass(frozen=True)
class SeedConfig:
    hostname: str = "phermes-linux"
    username: str = "dev"
    ssh_authorized_keys: list[str] = field(default_factory=list)
    install_uv: bool = True
```

No password field — key-only login enforced by `lock_passwd: true` + `ssh_pwauth: false`.

Three pure render functions, each returning a string:

```python
def meta_data_yaml(cfg: SeedConfig) -> str:
    """/CIDATA/meta-data — instance-id + hostname."""
    return (
        f"instance-id: phermes-{cfg.hostname}\n"
        f"local-hostname: {cfg.hostname}\n"
    )

def user_data_yaml(cfg: SeedConfig) -> str:
    """/CIDATA/user-data — user + SSH key + ssh_pwauth off + disable_root."""
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

Network config is intentionally absent — DHCP from `vmbr0` is cloud-init's Debian-cloud
default, and not pinning a static IP keeps the spec future-proof to bridge-subnet changes.

`write_seed_iso`:

```python
def write_seed_iso(out_path: str, cfg: SeedConfig) -> None:
    """Render YAML files and pack them into a NoCloud ISO labeled CIDATA.

    Fails with ValueError if cfg.ssh_authorized_keys is empty (a key-only login
    with zero keys would lock the operator out of the dev VM).
    """
    if not cfg.ssh_authorized_keys:
        raise ValueError("SeedConfig.ssh_authorized_keys must contain at least one key")
    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    with tempfile.TemporaryDirectory() as tmp:
        (Path(tmp) / "meta-data").write_text(meta_data_yaml(cfg))
        (Path(tmp) / "user-data").write_text(user_data_yaml(cfg))
        (Path(tmp) / "vendor-data").write_text(vendor_data_yaml(cfg))
        run_cmd([
            "genisoimage", "-output", out_path,
            "-volid", "CIDATA",
            "-joliet", "-rock",
            f"{tmp}/meta-data", f"{tmp}/user-data", f"{tmp}/vendor-data",
        ])
```

## Rust changes

### `phermesd/src/config.rs`

```rust
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiskInterface {
    #[default]
    VirtioScsi,
    VirtioBlk,
    Cdrom,                 // NEW — emits an IDE CDROM (read-only)
}
```

### `phermesd/src/qemu.rs`

`build_linux`'s existing disk loop gets one new arm:

```rust
for (i, disk) in d.disk.iter().enumerate() {
    match disk.interface {
        DiskInterface::Cdrom => {
            pair(&mut a, "-drive", format!(
                "media=cdrom,readonly=on,format={},file={},if=none,id=disk{i}",
                disk.format, disk.path.display(),
            ));
            pair(&mut a, "-device", format!("ide-cd,drive=disk{i}"));
        }
        DiskInterface::VirtioScsi => { /* existing virtio-scsi-pci + scsi-hd arm */ }
        DiskInterface::VirtioBlk  => { /* existing virtio-blk-pci arm */ }
    }
}
```

The `scsi_controller_added` flag continues to gate the `virtio-scsi-pci` device on
`VirtioScsi`; `Cdrom` never touches it.

**Why `ide-cd`:** canonical NoCloud delivery on q35. Works with Debian cloud-init out of the
box, no driver-order quirks. The seed is read-only, small (kilobytes), cold-attached — IDE
is exactly the right fit.

### Slice #2 interaction — none required

`Storage::provision`/`delete`/`checkpoint` operate only on `@phermesd`-tagged thin LVs. The
seed.iso is an external file, never tagged, never managed by storage. Adding a CDROM
`[[disk]]` to a def triggers no storage code path.

## `vm.py` change

`write_linux_def` gains a `seed_iso_path` keyword. When `None` (production), the rendered
TOML is identical to today's. When set (dev), a second `[[disk]]` block is appended:

```toml
[[disk]]
path = "/var/lib/phermes/seed/linux.iso"
format = "raw"
interface = "cdrom"
```

New module-level constants:

```python
SEED_DIR = "/var/lib/phermes/seed"
LINUX_SEED_PATH = f"{SEED_DIR}/linux.iso"
```

## `cli.py` orchestration delta

One new helper:

```python
def _write_cloud_init_seed(dev_ssh_pubkey: str | None) -> str | None:
    """Generate seed.iso in the chroot when a key is available. Returns the
    on-guest path (NOT the chroot path), or None if no seed should ship.
    """
    if not dev_ssh_pubkey:
        return None
    cfg = cloud_init.SeedConfig(ssh_authorized_keys=[dev_ssh_pubkey])
    chroot_path = os.path.join(PVE_ROOT_MOUNT, vm_mod.LINUX_SEED_PATH.lstrip("/"))
    cloud_init.write_seed_iso(chroot_path, cfg)
    return vm_mod.LINUX_SEED_PATH
```

`_provision_linux_vm` gains the seed path:

```python
def _provision_linux_vm(source: str | None, seed_iso_path: str | None) -> None:
    vm_mod.write_linux_def(PVE_ROOT_MOUNT, seed_iso_path=seed_iso_path)
    vm_mod.provision_linux_disk(source=source)
```

And in `build()`:

```python
if not no_vm:
    seed = _write_cloud_init_seed(dev_ssh_pubkey if dev_credentials else None)
    os_steps.append(
        ("Provisioning Linux VM", lambda: _provision_linux_vm(
            source=_linux_source(import_vm), seed_iso_path=seed,
        ))
    )
```

**Production invariant preserved.** When `--dev-credentials` is absent OR
`--dev-ssh-pubkey` is empty, `_write_cloud_init_seed` returns `None`, `write_linux_def`
emits no CDROM block, no seed.iso is written.

## Errors

- `cloud_init.write_seed_iso` raises `ValueError` on empty key list (defensive).
- `genisoimage` failure → `subprocess.CalledProcessError` from `run_cmd`, propagated by
  cli.py's existing error path.
- Missing `genisoimage` binary → `FileNotFoundError`, matching slice #6's
  `install_phermesd_binaries` "rebuild the phermes-build image" pattern.

## Testing

| Layer | What | Notes |
|---|---|---|
| Unit (Python — cloud_init) | `meta_data_yaml`/`user_data_yaml`/`vendor_data_yaml` pure renders; `write_seed_iso` mocks `run_cmd` and asserts `genisoimage -volid CIDATA` argv + the three temp files are written; `SeedConfig` defaults; empty-key-list rejected | `monkeypatch.setattr(cloud_init_mod, "run_cmd", fake)` |
| Unit (Python — vm.py) | `write_linux_def(seed_iso_path=…)` adds the CDROM block; `write_linux_def(seed_iso_path=None)` omits it | Snapshot the rendered TOML |
| Unit (Python — cli.py) | `_write_cloud_init_seed` returns `None` when no key; returns `LINUX_SEED_PATH` when key present; with `--dev-credentials` but `--no-vm` the seed is NOT written | Mock `cloud_init.write_seed_iso` and assert call presence/absence |
| Unit (Rust — qemu.rs) | `linux_argv_with_cdrom_disk_emits_media_cdrom_and_ide_cd` (golden) | New test in the existing `qemu::tests` module |
| Unit (Rust — config.rs) | Round-trip TOML `interface = "cdrom"` ↔ `DiskInterface::Cdrom` | Trivial serde test |
| Integration (smoke) | `just smoke-create && just smoke-full && just smoke-qemu` with `--dev-credentials --dev-ssh-pubkey <key> --import-vm linux=<debian-cloud.qcow2>` | The slice's success criterion |

## Success criterion

After the smoke harness boots the appliance and the operator SSHes in:

1. `phermesctl activate linux` brings the guest up.
2. `cat /var/lib/phermes/seed/linux.iso | head -c 32 | od -c | head -2` shows the ISO9660
   magic bytes (sanity check that the seed exists).
3. Within ~30–60 seconds of activate, the guest's cloud-init runs and finishes (visible on
   the serial unix socket via `socat /run/phermesd/linux/serial.sock -`).
4. `ssh dev@<guest-ip>` succeeds with the operator's SSH key.
5. `uv --version` works inside the guest.

The guest IP discovery is operator-side: the appliance does not expose VM IPs to the
host-side `phermesctl status` in #4a (that's a separate feature; for now the operator can
read it from the cloud-init log or `arp -n` on the vmbr0 host).

## Open questions (for the plan)

- **Bundled Debian cloud image vs operator-supplied.** Today's smoke flow uses
  `--import-vm linux=<path>`; the operator must provide a qcow2. Bundling a small Debian
  cloud image would make the smoke recipe self-contained, but bloats the phermes-build
  image. Lean toward operator-supplied for the MVP; revisit if the smoke flow becomes
  painful.
- **`UV_INSTALL_DIR=/usr/local/bin`** in vendor-data assumes the installer respects the env
  var. Confirm against the current `https://astral.sh/uv/install.sh` script at plan time;
  fall back to the default `~/.local/bin` + a symlink if the env var isn't honored.
- **`genisoimage` vs `xorrisofs`** — `genisoimage` is the long-standing tool; some Debian
  releases recommend `xorrisofs` as the modern equivalent. Confirm `genisoimage` is still
  the right call on bookworm-slim at plan time.
