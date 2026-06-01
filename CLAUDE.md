# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repo is

PHermes is a bootable SSD appliance that runs [Hermes Agent](https://github.com/NousResearch/hermes-agent) (MIT) inside a QEMU/KVM VM (macOS default, Windows and Linux supported) on a Proxmox VE host. It ships as `phermes-build`, a Python CLI that assembles the full stack on a target SSD from official upstream sources — no OS binaries are bundled.

**Boot chain:** `EFI → GRUB → Proxmox VE → QEMU/KVM VM → Hermes Agent`

Full design: [`docs/superpowers/specs/2026-05-31-phermes-design.md`](docs/superpowers/specs/2026-05-31-phermes-design.md)

## Implementation phases

- **Phase 1 (current):** `phermes-build` CLI — plan at [`docs/superpowers/plans/2026-05-31-phase1-phermes-build.md`](docs/superpowers/plans/2026-05-31-phase1-phermes-build.md)
- **Phase 2:** PHermes web UI (FastAPI + HTMX, first-boot wizard)
- **Phase 3:** `phermes` CLI (vm switch, update, rollback)
- **Phase 4:** VM definitions + Hermes-in-VM integration

## Development commands

```bash
# Install deps
uv sync

# Run tests (unit only — no root required)
uv run pytest -q

# Run a single test
uv run pytest tests/phermes_build/test_disk.py::test_compute_layout_1tb -v

# Run integration tests (requires root + loop device support)
sudo uv run pytest -m integration -v

# Lint and format
uv run ruff check src/ tests/
uv run ruff format src/ tests/

# Type check
uv run ty check src/

# Install CLI locally
uv pip install -e .
phermes-build --help
```

## Architecture

### `phermes-build` (Phase 1)

Source lives in `src/phermes_build/`. Each module owns exactly one concern:

| Module | Responsibility |
|---|---|
| `runner.py` | `run_cmd()` — thin `subprocess.run` wrapper; all system calls go through this so tests can monkeypatch it |
| `models.py` | Pydantic v2: `BuildConfig`, `DiskLayout`, `VMFlavor`, `VMConfig` |
| `disk.py` | Block device detection, 500 GB minimum validation, proportional layout math |
| `partitioner.py` | GPT table via `sfdisk`; `partition_path()` handles NVMe `p`-separator |
| `luks.py` | LUKS2 lifecycle — format/open/close/rekey; passphrase always via stdin (`--key-file -`) |
| `lvm.py` | LVM PV → VG (`pve`) → thin pool (`data`, ~370 GB) → root LV (`pve-root`, 30 GB) |
| `btrfs.py` | Format + mount + three subvolumes: `@overlay`, `@phermes`, `@snapshots` |
| `exfat.py` | Optional `PHERMES_SHARE` exFAT partition (outside LUKS by default) |
| `proxmox.py` | `debootstrap bookworm` → Proxmox community repo → `apt install proxmox-ve` in chroot → GRUB with `GRUB_ENABLE_CRYPTODISK=y` |
| `host_config.py` | nftables rules, Samba (`smb.conf` bound to `vmbr0`), Dropbear initramfs (port 2222), Avahi service, Proxmox RBAC (`PHermesUser` role) |
| `vm.py` | Proxmox VM config generation, `qm importdisk`, acquisition scheduling via flag files |
| `firstboot.py` | First-boot flag at `@phermes/firstboot.flag`; MOTD written to `/etc/issue` in chroot |
| `cli.py` | Typer entry point; orchestrates all modules with Rich progress display |

### Key invariants

- **All system calls go through `run_cmd()`** — never call `subprocess` directly from modules. This makes every module testable without root.
- **Tests monkeypatch at module level**: `monkeypatch.setattr(disk_mod, "run_cmd", fake)` — not `phermes_build.runner.run_cmd`. Each module imports `run_cmd` directly so patches must target the module's own reference.
- **Integration tests** (`@pytest.mark.integration`) require root and loop devices; they are never run in normal CI.
- **LUKS passphrase flow**: `phermes-build` sets a known temp passphrase (`phermes-change-me`) so Proxmox can boot; the first-boot wizard (Phase 2) replaces it.
- **One VM active at a time**: VM IDs are fixed — macOS=100, Windows=101, Linux=102.
- **Disk sizing**: `LVM_GB = 400` is a constant (Proxmox OS + VM thin pool). `PHERMES_DATA` (Btrfs) and `PHERMES_SHARE` (exFAT) split the remainder proportionally.

### Storage layout (what phermes-build produces)

```
[EFI 512MB][/boot 1GB][LUKS2 container → swap+LVM+Btrfs][PHERMES_SHARE exFAT]
                                          └── LVM: pve-root(30G) + pve-data thin pool
                                          └── Btrfs: @overlay / @phermes / @snapshots
```

### PHermes web UI (Phase 2 — not yet implemented)

FastAPI + HTMX, port 443. Five views: Dashboard, Console (noVNC proxy), Hermes (`:9119` proxy), Switch, Settings. Proxmox UI (port 8006) is firewalled to localhost.

### Networking (on the installed system)

Samba is bound to `vmbr0` (VM bridge) only — never exposed to the LAN. Port 8006 is localhost-only. The `phermes.local` mDNS name is advertised via Avahi.
