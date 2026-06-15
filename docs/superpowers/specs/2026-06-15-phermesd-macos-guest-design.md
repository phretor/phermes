# phermesd — macOS guest support (slice #5b)

**Status:** Design (approved)
**Date:** 2026-06-15
**Stack:** Stacked on slice #5a (PR #25, branch `feat/phermesd-windows-guest`).

## Goal

Make `phermesctl activate macos` boot a macOS guest under phermesd on a PHermes appliance, using the same BYOI shape as slice #5a Windows, plus a runtime-supplied Apple OSK so production builds ship no Apple credentials.

## Locked decisions

1. **Install model:** Full BYOI. Operator supplies one qcow2 containing OpenCore EFI + macOS. No Apple binaries, no OpenCore distribution, no Apple-endpoint downloads in our toolchain.
2. **OSK source:** Runtime secret file at `/etc/phermes/secrets/apple.osk` (mode 0400, root-owned). phermesd reads it at every macOS VM launch. `--dev-credentials --dev-apple-osk <osk>` pre-seeds the file in dev builds; production builds leave the path absent.
3. **Device profile:** Hackintosh-community defaults — Penryn CPU + Intel-vendor masking + `invtsc/pcid/ssse3/sse4.2/popcnt/avx/aes/xsave/xsaveopt`, OVMF firmware, AHCI disk, vmxnet3 net, nec-usb-xhci + usb-tablet. Defaults: 12 GiB RAM, 4 vCPU (2 cores × 2 threads), 120 GB LVM-thin LV. VMID=100.

## Architecture

### `phermesd/src/qemu.rs`

Add `fn build_macos(vm: &Vm, rt: &Runtime, osk: &str) -> Vec<String>`. Pure argv builder; OSK arrives as a parameter so unit tests pass a literal string.

Dispatch table becomes fully explicit — no wildcard arm:

```rust
match vm.def.flavor {
    Flavor::Linux | Flavor::Windows => Ok(build_pc_uefi(vm, rt)),
    Flavor::Macos => build_macos_with_secrets(vm, rt),
}
```

`build_macos_with_secrets(vm, rt) -> Result<Vec<String>, QemuError>` is the seam helper: reads the OSK file, then delegates to the pure `build_macos(vm, rt, &osk)`. Any future `Flavor` variant forces a new explicit arm.

Generated argv differs from `build_pc_uefi` at:

| Concern | `build_pc_uefi` | `build_macos` |
|---|---|---|
| `-cpu` | `host` | `Penryn,kvm=on,vendor=GenuineIntel,+invtsc,+pcid,+ssse3,+sse4.2,+popcnt,+avx,+aes,+xsave,+xsaveopt,check` |
| `-smp` | `<vcpus>` | `cpus=4,cores=2,threads=2,sockets=1` |
| `-machine` | `q35,accel=kvm` | `q35,accel=kvm,kernel-irqchip=split` |
| applesmc | (absent) | `-device isa-applesmc,osk=<64 hex chars>` |
| Disk | virtio-scsi | `-device ich9-ahci,id=ahci0` + `-device ide-hd,bus=ahci0.0,...` |
| Net | virtio-net | `-device vmxnet3,...` |
| USB | (none) | `-device nec-usb-xhci,id=xhci` + `-device usb-tablet,bus=xhci.0` |

Everything else (OVMF, QMP, serial+vnc sockets, `-pidfile`) follows the slice #1 plumbing unchanged.

### New `phermesd/src/secrets.rs`

```rust
pub enum SecretsError {
    NotFound { path: PathBuf },
    BadPermissions { path: PathBuf, mode: u32 },
    BadFormat { reason: &'static str },
    Io(std::io::Error),
}

pub fn read_apple_osk(path: &Path) -> Result<String, SecretsError>;
```

Validation:
- Exists; if not → `NotFound`.
- `stat.mode & 0o077 == 0` (no group/other access); else → `BadPermissions`. Owner-UID check is skipped when the secrets root path does not start with `/etc/` so tests can pass a tmp dir.
- Content trimmed of trailing whitespace.
- Length exactly 64, all chars `[0-9a-fA-F]`; else → `BadFormat`.
- Returns the trimmed string. Never logs the value.

### `phermes-build` (Python)

`src/phermes_build/vm.py`:
- Constants: `MACOS_VMID = 100`, `MACOS_DEFAULT_DISK_GB = 120`, `MACOS_DEFAULT_MEMORY_MIB = 12288`, `MACOS_DEFAULT_VCPUS = 4`.
- `_macos_def_text(*, memory_mib, vcpus) -> str` — emits TOML mirroring `_windows_def_text` but with `flavor = "macos"`, the macOS device-profile fields, **no OSK**, no CDROM.
- `write_macos_def(chroot, *, memory_mib=…, vcpus=…)` — same shape as `write_windows_def`.
- `provision_macos_disk(size_gb=MACOS_DEFAULT_DISK_GB, source=None)` — same lvcreate/lvchange/qemu-img-convert flow as `provision_windows_disk`, with `vm-100-disk-0`.

`src/phermes_build/cli.py`:
- `_SUPPORTED_IMPORT_FLAVORS = ("linux", "windows", "macos")`. `_vm_source` unchanged otherwise.
- `_provision_macos_vm(source: str | None) -> None` — writes the macOS def and provisions the disk (no seed path argument).
- New CLI flag: `--dev-apple-osk <osk>` (env var: `PHERMES_DEV_APPLE_OSK`). Only valid with `--dev-credentials`; rejected otherwise via `typer.BadParameter`.
- `_write_apple_osk_secret(chroot, osk)` — writes `<chroot>/etc/phermes/secrets/apple.osk` mode 0400, owner root. The parent `secrets/` directory is mode 0700.
- `build()` wires macOS into `os_steps` only when `macos_source is not None`. Always create the secrets directory (regardless of OSK presence) so the operator can drop the file in post-install.

### Data flow at launch

```
phermesctl activate macos
  -> phermesd Launcher::launch(vm)
     -> match vm.def.flavor { Flavor::Macos => ... }
        -> secrets::read_apple_osk("/etc/phermes/secrets/apple.osk")
           -> NotFound -> LauncherError::MissingAppleOsk { path }
                          -> phermesctl prints: "Apple OSK not configured.
                             Write the 64-char hex string to
                             /etc/phermes/secrets/apple.osk (mode 0400) and retry."
           -> Ok(osk) -> build_macos(vm, rt, &osk)
                     -> setsid + QEMU spawn (slice #1 plumbing)
```

The OSK string ends up in the QEMU argv (`/proc/<pid>/cmdline`). Documented limitation — anyone with root on the host already has the LUKS passphrase and full disk access, so this is not a credible additional exposure.

## Error catalog

| Site | Variant | Trigger | Operator action |
|---|---|---|---|
| `secrets.rs` | `NotFound` | file missing | write it, mode 0400 |
| `secrets.rs` | `BadPermissions` | mode > 0o400 or wrong owner | `chmod 400 && chown root:root` |
| `secrets.rs` | `BadFormat` | not 64 hex after trim | re-check the string |
| `cli.py` | `BadParameter` | `--dev-apple-osk` without `--dev-credentials` | add `--dev-credentials` or drop OSK flag |
| `cli.py` | `BadParameter` | malformed `--dev-apple-osk` | re-check |
| Launcher | `MissingAppleOsk { path }` | wraps `NotFound` for activate path | same as `NotFound` |

## Testing

**Rust unit (`qemu.rs`):**
- `macos_argv_includes_penryn_cpu_with_full_flag_set` (byte-check `-cpu Penryn,…`)
- `macos_argv_includes_isa_applesmc_with_osk` (literal OSK passed in argv)
- `macos_argv_uses_ahci_not_virtio_scsi`
- `macos_argv_uses_vmxnet3`
- `macos_argv_byte_identical_for_equivalent_def_modulo_osk` (determinism)

**Rust unit (`secrets.rs`):**
- `read_apple_osk_returns_ok_for_valid_file`
- `read_apple_osk_strips_trailing_whitespace`
- `read_apple_osk_rejects_short_string`
- `read_apple_osk_rejects_non_hex`
- `read_apple_osk_rejects_world_readable_mode`
- `read_apple_osk_returns_not_found_when_missing`

**Python unit (`vm.py`):**
- `test_macos_constants`
- `test_write_macos_def_emits_expected_toml` (Penryn cpu, ahci disk, vmxnet3 net, no OSK)
- `test_write_macos_def_honors_override_resources`
- `test_provision_macos_disk_creates_thin_lv_and_tags_it` (120 GB default)
- `test_provision_macos_disk_with_source_runs_qemu_img_convert`
- `test_provision_macos_disk_custom_size`

**Python unit (`cli.py`):**
- `test_vm_source_accepts_macos_flavor`
- `test_import_vm_macos_routes_into_provision_macos_vm`
- `test_dev_apple_osk_writes_secret_file_with_0400_mode`
- `test_dev_apple_osk_without_dev_credentials_errors_out`
- `test_production_build_does_not_create_apple_osk_file`
- `test_secrets_dir_created_even_without_osk` (mode 0700)
- Update `test_unsupported_import_vm_flavor_errors_out` — flip from `macos=` to a still-unsupported sentinel (e.g. `freebsd=`).

## Success criterion

`phermesctl activate macos` on the assembled appliance — with operator's qcow2 imported via `--import-vm macos=<path>` and `/etc/phermes/secrets/apple.osk` populated — boots macOS to the login screen and accepts VNC via the slice #4b SSH-tunnel pattern. Absence of the OSK file produces `MissingAppleOsk` with the documented operator-action message, not a QEMU crash or kernel panic loop.

## Out of scope (explicit deferrals)

- Audio (no `hda-output`).
- iServices / per-host MLB/ROM serial generation — operator's OpenCore handles SMBIOS.
- AMD-host CPU patches (Intel hosts only for MVP).
- UI for OSK upload (Phase 2 web UI concern).
- macOS unattended install (no cloud-init equivalent).
- `--download-vm macos` (no Apple-endpoint integration).
- Audio passthrough, GPU passthrough, USB passthrough.
- macOS version detection / per-version CPU profile selection.
