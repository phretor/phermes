# phermesd — Windows Guest Support (Design)

**Date:** 2026-06-12
**Status:** Draft — pending implementation plan
**Sub-project:** #5a of the `phermesd` replacement for Proxmox VE

---

## Context & motivation

Slice #1's `Flavor` enum declared all three guest variants (Linux, Windows, macOS); only
`Flavor::Linux` was wired through `build_argv`. The other two return
`QemuError::UnsupportedFlavor`. Slice #5 was decomposed during brainstorming into:

- **#5a — Windows** (this spec): wires `Flavor::Windows` end-to-end.
- **#5b — macOS**: OpenCore + applesmc + SMBIOS + Penryn CPU flags. Separate slice.

Inspecting `build_linux` revealed it is **already a generic q35 + UEFI + virtio PC
builder** — every device it emits (virtio-scsi/virtio-blk/cdrom disks, virtio-net/e1000
NICs, virtio-serial-pci for the QEMU Guest Agent, OVMF pflash, serial/vnc unix sockets) is
flavor-agnostic. The old `proxmox_vm_config` confirms this: its Windows branch fell
through to the same defaults Linux used; only macOS had special args. So enabling Windows
on the Rust side is a **one-line dispatch change plus a function rename**: `build_linux` →
`build_pc_uefi` (honest naming), with both `Linux` and `Windows` routed to it. macOS gets
its own arm in #5b.

The bulk of the work is on the Python side: mirror slice #4a's Linux provisioning pattern
for Windows — constants for `WINDOWS_VMID = 101`, a `write_windows_def`, a
`provision_windows_disk`, and a `--import-vm windows=<path>` flag that coexists with
`--import-vm linux=<path>` (both VMs can be installed in one build).

## Decisions (resolved during brainstorming)

| Decision | Choice |
|---|---|
| Install model | **BYOI** — operator brings a pre-installed Windows qcow2 (virtio drivers already loaded). Mirrors slice #4a's Linux assumption (operator supplies a Debian cloud image). |
| Rust dispatch | **Rename `build_linux` → `build_pc_uefi`; route `Flavor::Linux \| Flavor::Windows => build_pc_uefi`.** The rename pays off in #5b (which adds a separate `build_macos`). |
| Def model changes | **None.** The existing `Flavor::Windows` variant + `DiskInterface::{VirtioScsi, VirtioBlk, Cdrom}` + `NetModel::{VirtioNet, E1000}` cover every Windows install option the operator might need. Operator hand-edits `interface = "sata"` or `model = "e1000"` if their image lacks virtio drivers. |
| Windows defaults | **8 GiB RAM, 4 vCPUs, 100 GB disk** — matches the old `_VM_DISK_GB[WINDOWS] = 100` and aligns with Windows 11 baselines. (Linux defaults stay 4/4/40.) |
| Cloud-init / unattend.xml | **No automated guest config in #5a** — operator's qcow2 is expected to be already configured. unattend.xml generation is a later slice (Windows analog of #4a). |
| Multi-flavor builds | `phermes-build … --import-vm linux=lx.qcow2 --import-vm windows=win.qcow2` provisions **both** VMs (each with its own def + thin LV). `--no-vm` still skips both. |

## Scope boundary

**In:**
- Rust rename `build_linux` → `build_pc_uefi` in `phermesd/src/qemu.rs`; dispatch arm
  `Flavor::Linux | Flavor::Windows => build_pc_uefi`.
- One new Rust test: `windows_argv_is_byte_identical_to_linux_argv_for_equivalent_def`.
- Python `vm.py`: new constants (`WINDOWS_VMID = 101`, `WINDOWS_DEFAULT_DISK_GB = 100`,
  `WINDOWS_DEFAULT_MEMORY_MIB = 8192`, `WINDOWS_DEFAULT_VCPUS = 4`); new functions
  `write_windows_def(chroot_mount, *, memory_mib, vcpus)` and
  `provision_windows_disk(size_gb, source)`. Mirrors the Linux pattern byte-for-byte.
- Python `cli.py`: extend the `_linux_source` helper into a `_vm_source(import_vm_args, flavor)`
  that supports both `linux=<path>` and `windows=<path>`; add a `_provision_windows_vm`
  helper; `build()` appends a "Provisioning Windows VM" step when `--import-vm windows=…`
  is present and `--no-vm` is not.
- Tests for the Python additions (≈ 6 in `test_vm.py`, ≈ 4 in `test_cli.py`).
- README + CHANGELOG documenting Windows BYOI and the per-flavor `--import-vm` invocation.

**Out (deferred):**
- macOS guest support → #5b (the `build_pc_uefi` rename pays off there).
- Automated Windows installer with attached virtio-win.iso (a "first-install dance" that
  attaches a Windows install ISO + virtio-win.iso, runs setup, sideloads storage drivers).
- `unattend.xml` generation → later slice (Windows analog of #4a's cloud-init).
- TPM 2.0 / Secure Boot OVMF vars variant for Windows 11 → later slice (needs `swtpm`).
- QGA-quiesced snapshots for Windows — the virtio-serial QGA channel is already wired by
  slice #1, but quiescing requires `qemu-guest-agent` installed in the Windows guest
  (operator's responsibility for #5a).

**Prerequisite:** implementation branch should stack on slice #4b (`feat/phermesd-console-proxy`,
PR #24) so all open phermesd PRs land in order. Bare `main` works once
PRs #19/#20/#23/#24 are merged.

## Architecture & file structure

### Rust (`phermesd/`)

| File | Change |
|---|---|
| `src/qemu.rs` | Rename `build_linux` → `build_pc_uefi`. Update its doc comment to reflect that it's a generic q35+UEFI+virtio PC builder, not Linux-specific. Change `build_argv`'s match: `Flavor::Linux \| Flavor::Windows => Ok(build_pc_uefi(vm, rt))`. `Flavor::Macos` still returns `Err(QemuError::UnsupportedFlavor(Flavor::Macos))`. |
| `src/qemu.rs::tests` | Existing `linux_argv_*` golden tests stay (verify the renamed function still emits the same argv). Add ONE new test: `windows_argv_is_byte_identical_to_linux_argv_for_equivalent_def` — builds two `Vm`s with identical fields except `flavor`, asserts `build_argv(vm_lx, &rt) == build_argv(vm_win, &rt)`. The existing `non_linux_flavor_is_unsupported` test is updated to use `Flavor::Macos` specifically (since Windows is now supported). |

### Python (`src/phermes_build/`)

| File | Change |
|---|---|
| `vm.py` | Add module-level constants: `WINDOWS_VMID = 101`, `WINDOWS_DEFAULT_DISK_GB = 100`, `WINDOWS_DEFAULT_MEMORY_MIB = 8192`, `WINDOWS_DEFAULT_VCPUS = 4`. Add `WINDOWS_SEED_PATH = f"{SEED_DIR}/windows.iso"` placeholder (unused in #5a — no seed shipped — but reserved for a future unattend slice). Add `_windows_def_text(*, memory_mib, vcpus)` (private renderer), `write_windows_def(chroot_mount, *, memory_mib=…, vcpus=…)`, `provision_windows_disk(size_gb=WINDOWS_DEFAULT_DISK_GB, source=None)`. The internals mirror `write_linux_def`/`provision_linux_disk` byte-for-byte; only flavor string, VMID, and defaults differ. |
| `cli.py` | Replace `_linux_source(import_vm_args)` with a generic `_vm_source(import_vm_args, flavor)`; both `linux=<path>` and `windows=<path>` are accepted; unrecognised flavors raise `typer.BadParameter`. Add `_provision_windows_vm(source)` mirroring `_provision_linux_vm`. In `build()`, when `not no_vm`: append a "Provisioning Linux VM" step iff `_vm_source(import_vm, "linux")` is `Some`; append a "Provisioning Windows VM" step iff `_vm_source(import_vm, "windows")` is `Some`. Order: Linux first, then Windows (alphabetical by flavor name). |
| `tests/phermes_build/test_vm.py` | New tests: `test_windows_constants` (4 module-level checks), `test_write_windows_def_emits_expected_toml`, `test_write_windows_def_honors_override_resources`, `test_provision_windows_disk_creates_thin_lv_and_tags_it`, `test_provision_windows_disk_with_source_runs_qemu_img_convert`, `test_provision_windows_disk_custom_size`. ≈ 6 new tests. |
| `tests/phermes_build/test_cli.py` | New tests: `test_import_vm_windows_routes_into_provision_windows_disk` (mirrors the Linux one); `test_import_vm_both_flavors_provisions_both_vms` (one invocation produces both os_steps); `test_no_vm_skips_both_provisioning_helpers` (--no-vm with both flags); `test_vm_source_rejects_unknown_flavor` (freebsd=… raises BadParameter). ≈ 4 new tests. |

### Docs

| File | Change |
|---|---|
| `README.md` | `### phermesd` subsection gains a `Slice #5a (implemented)` sentence describing Windows BYOI and the `--import-vm windows=<path>` flag. |
| `CHANGELOG.md` | `### Added` entry for slice #5a. |

## Data flow

### Build time — `phermes-build --dev-credentials --import-vm windows=/tmp/win.qcow2`

1. `cli.build()` parses `import_vm: list[str]`; `_vm_source(import_vm, "windows")` returns
   `"/tmp/win.qcow2"`.
2. Conditional step appended:
   ```python
   if not no_vm:
       windows_source = _vm_source(import_vm, "windows")
       if windows_source is not None:
           os_steps.append(
               ("Provisioning Windows VM",
                lambda: _provision_windows_vm(source=windows_source))
           )
   ```
3. `_provision_windows_vm(source)`:
   - `vm.write_windows_def(PVE_ROOT_MOUNT)` → writes `/etc/phermes/vms/windows.toml`.
   - `vm.provision_windows_disk(source=source)` → `lvcreate --thin --virtualsize 100G
     pve/data -n vm-101-disk-0` → `lvchange --addtag phermesd /dev/pve/vm-101-disk-0` →
     `qemu-img convert -O raw /tmp/win.qcow2 /dev/pve/vm-101-disk-0`.

No cloud-init seed for Windows in #5a. The `windows.toml` has no second `[[disk]]` CDROM
block; the operator's qcow2 is expected to be already configured.

### `windows.toml` written into the chroot

```toml
flavor = "windows"
[resources]
memory_mib = 8192
vcpus = 4
cpu = "host"
[firmware]
ovmf_code = "/usr/share/OVMF/OVMF_CODE.fd"
ovmf_vars_template = "/usr/share/OVMF/OVMF_VARS.fd"
[[disk]]
path = "/dev/pve/vm-101-disk-0"
format = "raw"
interface = "virtio-scsi"
[[net]]
bridge = "vmbr0"
model = "virtio-net"
[console]
serial = true
vnc = true
```

Identical shape to `linux.toml`; only `flavor`, the `vm-N-disk-0` VMID (101 vs 102), and
resource defaults (8192/4/100 vs 4096/4/40) differ. Operators with images that lack virtio
drivers hand-edit `interface = "sata"` or `model = "e1000"` post-build.

### Runtime — `phermesctl activate windows`

1. `phermesd::config::load_dir(/etc/phermes/vms/)` discovers `windows.toml`, parses
   `Flavor::Windows`.
2. `phermesctl activate windows` over the existing UDS.
3. `Supervisor::activate("windows")` → `qemu::build_argv(vm, rt)` matches
   `Flavor::Linux | Flavor::Windows => Ok(build_pc_uefi(vm, rt))`.
4. The emitted argv is byte-identical to what a Linux def with the same paths would
   produce. QEMU launches; QMP confirms running.
5. `phermesctl console windows` (slice #4b) attaches to `/run/phermesd/windows/serial.sock`
   (flavor-agnostic). `ssh -L 5900:/run/phermesd/windows/vnc.sock <appliance>` exposes
   VNC.
6. `phermesctl snapshot windows`, `rollback`, `delete` (slice #2) work because storage
   operations key off the `phermesd` tag and VMID, not flavor.
7. `phermesctl activate linux` from a fresh shell gracefully stops the active Windows VM
   first (slice #1's auto-switch path, also flavor-agnostic).

**Storage VMID layout:** `vm-100-disk-0` reserved for macOS (#5b), `vm-101-disk-0` for
Windows (this slice), `vm-102-disk-0` for Linux (existing). All three IDs match slice #1's
fixed mapping. No collisions; all three can coexist on the same appliance.

## Errors

No new error categories.

- Rust: `Flavor::Macos` still returns `QemuError::UnsupportedFlavor(Flavor::Macos)`. The
  existing test stays, narrowed to that specific variant.
- Python: `provision_windows_disk` propagates `subprocess.CalledProcessError` from
  `runner.run_cmd` (lvcreate/lvchange/qemu-img failures) — same pattern as the Linux
  path.
- Python: `_vm_source(import_vm, flavor)` raises `typer.BadParameter` for any
  `--import-vm <unknown>=…` flavor. Slice #6's original `_linux_source` already had this
  shape; the new generic version preserves it.

## Testing

| Layer | What | Notes |
|---|---|---|
| Rust unit (`qemu.rs`) | New: `windows_argv_is_byte_identical_to_linux_argv_for_equivalent_def`. Existing `linux_argv_*` golden tests prove the rename is behavior-preserving (they reference `build_argv`, not `build_linux` directly, so they pass unchanged). `non_linux_flavor_is_unsupported` is renamed to `macos_flavor_is_unsupported` and asserts `Flavor::Macos` specifically. | One new test, one rename. |
| Python unit (`test_vm.py`) | Six new tests mirroring Linux: constants, def TOML shape, override resources, lvcreate+lvchange sequence, qemu-img convert when source set, custom size. Use `monkeypatch.setattr(vm_mod, "run_cmd", fake)`. | Direct parallel of the slice #6 Linux tests. |
| Python unit (`test_cli.py`) | Four new tests: `--import-vm windows=…` routes to `_provision_windows_vm`; both flavors in one invocation produces both os_steps entries; `--no-vm` skips both even with both flags; `_vm_source` rejects unknown flavors with `typer.BadParameter`. | Typer `CliRunner` pattern. |
| Rust integration | None new. Slice #2's storage tests and slice #1's supervisor tests are flavor-agnostic — once `Flavor::Windows` dispatches, they cover Windows too. | Verified by the existing test suite running unchanged. |
| Operator smoke (out of CI) | `just smoke-create && just smoke-full` with `--dev-credentials --dev-ssh-pubkey <key> --import-vm windows=<win.qcow2>`. `just smoke-qemu` boots the appliance. Verify the four success criteria below. | Operator-verified post-merge. |

## Success criterion

After the smoke harness boots the appliance and the operator SSHes in:

1. `phermesctl list` shows `windows` as a defined VM (alongside any linux VM the same
   build provisioned).
2. `phermesctl activate windows` succeeds; QMP reports running within the normal timeout;
   `/run/phermesd/windows/{serial,vnc,qga,qmp}.sock` all exist.
3. `vncviewer localhost:5900` via
   `ssh -L 5900:/run/phermesd/windows/vnc.sock <appliance>` shows the Windows desktop or
   login screen.
4. `phermesctl activate linux` from a fresh session gracefully stops Windows (slice #1's
   auto-switch path, including slice #2's auto-checkpoint if the active VM has the
   `phermesd` tag) and boots Linux.

Live snapshot quiescing for Windows (slice #2's QGA fsfreeze) is **not** part of #5a's
success criterion — it requires `qemu-guest-agent` installed in the Windows guest, which
is the operator's responsibility.

## Open questions (for the plan)

- **`build_linux` callers in tests** — the rename is mechanical. Confirm at plan time that
  the only callers are `build_argv` and the test module (no test elsewhere references
  `build_linux` directly); if any do, rename them.
- **Default `model = "virtio-net"` in `windows.toml`** — operator images without virtio
  drivers will fail to acquire network at first boot. We default to virtio-net (best
  perf, mirrors Linux) and document the hand-edit. Confirm the plan calls this out in the
  README, not just the spec.
- **`--no-vm` semantics with multiple flavors** — `--no-vm` is a single boolean today and
  skips ALL VM provisioning. Per-flavor skip (`--no-linux-vm` / `--no-windows-vm`) is YAGNI
  until someone asks. Confirm at plan time.
