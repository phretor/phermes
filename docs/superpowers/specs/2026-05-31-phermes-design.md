# PHermes Design Spec

**Date:** 2026-05-31
**Status:** Draft — pending implementation plan

---

## Overview

PHermes (Personal Hermes / Phretor's Hermes) is a bootable SSD appliance that runs
[Hermes Agent](https://github.com/NousResearch/hermes-agent) (NousResearch, MIT) inside
a full GUI VM on any KVM-capable bare-metal host. The user plugs in a PHermes SSD, boots,
opens a browser, and has a fully functional Hermes environment accessible from anywhere on
their local network.

**Boot chain:** EFI → GRUB → Proxmox VE host → QEMU/KVM VM (macOS default) → Hermes

PHermes ships as `phermes-build`, a signed CLI tool that assembles a bootable SSD from
official upstream sources. No Proxmox, macOS, or Windows binaries are distributed by
PHermes. License: PHermes own code is independent of Proxmox's AGPL-3.0.

---

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│  Physical hardware  (KVM-capable, ≥32 GB RAM recommended)    │
├──────────────────────────────────────────────────────────────┤
│  UEFI → GRUB                                                 │
├──────────────────────────────────────────────────────────────┤
│  Proxmox VE host  (Debian-based)                             │
│  ┌───────────────────────┐  ┌──────────────────────────────┐ │
│  │  PHermes web UI       │  │  phermes CLI                 │ │
│  │  FastAPI + HTMX       │  │  first-boot · vm · update    │ │
│  │  port 443             │  │                              │ │
│  └───────────────────────┘  └──────────────────────────────┘ │
│  Proxmox web UI: localhost:8006 only (SSH tunnel for recovery)│
├──────────────────────────────────────────────────────────────┤
│  QEMU/KVM VMs (one active at a time)                         │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  macOS VM (OpenCore) │ Windows VM │ Linux VM            │ │
│  │         └── Hermes (uv · Python · REST · web :9119)     │ │
│  └─────────────────────────────────────────────────────────┘ │
├──────────────────────────────────────────────────────────────┤
│  SSD storage (see Storage Layout)                            │
└──────────────────────────────────────────────────────────────┘
```

**Key decisions:**
- Proxmox VE handles VM lifecycle, LVM-thin storage, and noVNC console — no custom hypervisor code
- `phermes` CLI wraps Proxmox APIs for PHermes-specific operations
- Hermes runs inside the active VM only; the host stays minimal and trusted
- Proxmox UI is firewalled from LAN — users interact exclusively with the PHermes web UI
- Only one VM runs at a time; VM switching is a first-class operation with automatic snapshotting

---

## Storage Layout

```
SSD (≥500 GB minimum, 1 TB recommended)
├── EFI               512 MB   UEFI boot (plaintext — firmware requirement)
├── boot                1 GB   /boot ext4 (plaintext — GRUB reads pre-unlock)
├── LUKS2 container    ~rest   single passphrase, unlocks everything below
│   ├── swap           16 GB   encrypted swap
│   ├── LVM PV        ~400 GB  Proxmox OS + LVM-thin VM image pool
│   │   ├── pve-root   30 GB   Proxmox system root (ext4)
│   │   └── pve-data  ~370 GB  LVM-thin pool
│   │       ├── macos ~120 GB  macOS VM disk
│   │       ├── win   ~100 GB  Windows VM disk
│   │       └── linux  ~40 GB  Linux VM disk (optional)
│   └── PHERMES_DATA  ~330 GB  Btrfs — user overlay, Hermes data, snapshots
│       ├── @overlay            SMB-mounted into active VM
│       │   ├── hermes/         Hermes profiles, tasks, knowledge base
│       │   └── documents/
│       ├── @phermes            PHermes config and state (config.yaml, etc.)
│       └── @snapshots/         Btrfs subvolume snapshots (pre-switch, pre-update)
└── PHERMES_SHARE     ~250 GB  exFAT — drag-and-drop from any OS (default: plaintext)
                               configurable: --share-size N, --share-encrypted
```

**Sizing:** `phermes-build` enforces a 500 GB minimum and scales LVM / Btrfs / SHARE
proportionally to actual SSD size. A 2 TB SSD gives ~900 GB to user partitions.

**Snapshots:**
- VM disks: LVM-thin snapshot (instantaneous) before every switch and update
- User data: `btrfs subvolume snapshot -r @overlay @snapshots/overlay-TIMESTAMP`
  (instantaneous, copy-on-write, space-efficient) before every switch and update
- No ZFS — eliminates ECC RAM requirement; suitable for arbitrary consumer hardware

---

## VM Definitions

All VMs: q35 machine type, UEFI boot, LVM-thin disk, single active at a time.

| | macOS | Windows | Linux |
|---|---|---|---|
| Boot | OpenCore EFI (OSX-PROXMOX config) | Standard UEFI | Standard UEFI |
| CPU | host-passthrough + macOS compat flags | host-passthrough | host-passthrough |
| RAM | ≥8 GB (16 recommended) | ≥8 GB | ≥4 GB |
| Display | vmware-svga | virtio-vga | virtio-vga |
| Network | vmxnet3 | VirtIO | VirtIO |
| Disk | ~120 GB LVM-thin | ~100 GB LVM-thin | ~40 GB LVM-thin |

**macOS note:** macOS VMs may only be run on Apple hardware under the macOS EULA.
PHermes does not distribute macOS images; users supply their own installer.

---

## Data Overlay

`PHERMES_DATA/@overlay` is shared into the active VM via Samba (SMB) from the host.
Samba runs on the Proxmox host, bound to the VM bridge interface only (not LAN-exposed).
Avahi advertises `phermes.local` so guests discover the share without hardcoded IPs.

| Guest | Mount point | Client |
|---|---|---|
| macOS | `/Volumes/PHermesData` | native SMB (fstab) |
| Windows | `Z:\` | native SMB (mapped drive) |
| Linux | `/mnt/phermes-data` | cifs-utils / fstab |

Inside each VM, Hermes config directories (`~/.config/hermes/`, `~/.local/share/hermes/`)
are symlinked into the SMB mount. Hermes data — profiles, tasks, knowledge base — persists
across VM flavor switches transparently.

`PHERMES_SHARE` (exFAT) is also auto-mounted in the active VM:
`/Volumes/PHermesShare` (macOS), `Y:\` (Windows), `/mnt/phermes-share` (Linux).

---

## Hermes Integration

Hermes Agent (MIT, NousResearch) is installed inside each VM via `uv`:

```bash
uv pip install hermes-agent
```

On VM boot, a guest-native service starts `hermes gateway` bound to `0.0.0.0:9119`:
systemd unit (Linux), launchd plist (macOS), Windows Service (Windows). The PHermes web UI reverse-proxies this port,
exposing the Hermes dashboard at `https://phermes.local/hermes`.

LLM API keys set during first-boot wizard are written to `@phermes/config.yaml` and
injected into the VM's Hermes environment on each boot.

---

## VM Switching

Switching VM flavor is a first-class operation managed by `phermes switch <flavor>`:

1. Gracefully shut down active VM (guest shutdown, not force-kill)
2. `btrfs subvolume snapshot -r @overlay @snapshots/overlay-TIMESTAMP`
3. LVM-thin snapshot of active VM disk (instantaneous)
4. Start target VM
5. Target VM boots, auto-mounts `PHERMES_DATA` via SMB, starts Hermes service
6. PHermes web UI console redirects to the new VM's noVNC session

Rollback to pre-switch state: `phermes rollback` restores both LVM-thin and Btrfs snapshots.

---

## First-Boot Wizard

On first boot, the physical display shows (via Proxmox text console):

```
PHermes is ready.
Connect from any browser on this network: https://phermes.local
                                      or: https://192.168.x.x
```

The PHermes web UI serves a one-time wizard (first-boot flag cleared on completion):

1. **Identity** — username, password (Proxmox admin + Samba auth)
2. **Network** — DHCP (default) or static IP, hostname (default: `phermes`)
3. **Encryption** — change LUKS2 passphrase from the build-time default; `phermes-build`
   sets a temporary passphrase so Proxmox can boot on first run; wizard replaces it and
   prompts the user to test unlock via Dropbear before proceeding
4. **Storage split** — PHERMES_SHARE size (default 250 GB; 0 to disable; encrypted flag)
5. **LLM providers** — API keys for Anthropic, OpenAI, or local endpoint; ≥1 required
6. **VM flavor** — which guest to set up first (macOS default)
7. **VM image acquisition**:
   - **Download now** — PHermes fetches and assembles image (progress in UI)
   - **Already staged** — user confirms image on PHERMES_SHARE; PHermes imports it
   - **Skip** — defer; user returns to this step later via Settings
8. **Review + confirm** — summary before applying

Total time excluding VM download: under 5 minutes.

---

## Networking

```
Physical NIC / Wi-Fi
        │
    vmbr0 (Proxmox bridge, DHCP or static)
    ├── Proxmox host  ← phermes.local
    │   ports: 443 (PHermes UI), 2222 (Dropbear initramfs SSH)
    ├── macOS VM      ← internet via bridge; Samba from host
    ├── Windows VM
    └── Linux VM
```

**Host firewall (nftables):**

| Port | Access | Purpose |
|---|---|---|
| 443 | LAN | PHermes web UI |
| 2222 | LAN | Dropbear SSH (LUKS unlock, headless machines) |
| 8006 | localhost only | Proxmox web UI (SSH tunnel for recovery) |
| 445 | VM bridge only | Samba (not exposed to LAN) |

VMs: outbound internet allowed, inbound blocked except from host.

**LUKS unlock on headless machines:** user SSHs to `phermes.local:2222` during initramfs
to enter passphrase. Physical keyboard + display is the alternative for non-headless use.

---

## PHermes Web UI

FastAPI + HTMX, port 443. Minimal JS surface, easy to audit.

| View | Purpose |
|---|---|
| Dashboard | VM status, Hermes health, storage usage, update badge |
| Console | Embedded noVNC (proxied from Proxmox API) |
| Hermes | Reverse-proxied Hermes dashboard (`:9119`) |
| Switch | Change active VM flavor; shows snapshot + estimated boot time |
| Settings | LLM API keys, network config, passphrase change, update trigger |

Proxmox's own UI is not advertised; accessible only via SSH tunnel to `localhost:8006`.

---

## Distribution Model

PHermes ships **only** `phermes-build` — a signed Python CLI (uv-managed). No Proxmox,
macOS, or Windows binaries are bundled or distributed.

```bash
# Minimal — wizard handles VM image acquisition at first boot
phermes-build /dev/sdX

# Pre-staged macOS image (mode B)
phermes-build /dev/sdX --import-vm macos=/path/to/macos.qcow2

# Download VM images at build time (mode C)
phermes-build /dev/sdX --download-vm macos --download-vm windows

# Custom share partition
phermes-build /dev/sdX --share-size 500G --share-encrypted
```

**What `phermes-build` does:**
1. Partitions the SSD (EFI, boot, LUKS2, optional PHERMES_SHARE)
2. Installs Debian + Proxmox VE from official community repos
3. Applies PHermes config (nftables, Samba, Proxmox RBAC, PHermes web UI, Dropbear initramfs)
4. Imports or schedules VM image acquisition
5. Writes first-boot flag — wizard runs once on first browser visit

GitHub releases ship a signed binary and checksummed install script. Users verify before
running.

---

## Updates

```bash
phermes update           # full update sequence
phermes update --host    # Proxmox + Debian packages only
phermes update --ui      # PHermes web UI + CLI only
phermes update --hermes  # Hermes inside the active VM only
```

**`phermes update` sequence:**
1. Btrfs snapshot of `@overlay` + LVM-thin snapshot of active VM disk
2. `apt update && apt full-upgrade` on Proxmox host (community repos, no subscription)
3. Pull latest PHermes release from GitHub; replace CLI + web UI
4. SSH into active VM; `uv pip install --upgrade hermes-agent`
5. Report changes; offer `phermes rollback` if anything looks wrong

PHermes dashboard shows a non-intrusive update badge on new releases.

---

## Open Questions

- macOS OpenCore EFI config: track OSX-PROXMOX upstream or maintain a PHermes fork?
- Hermes LLM key injection mechanism: env vars in VM service unit vs encrypted keystore?
- Wi-Fi support on host: `wpa_supplicant` configured at first boot, or Ethernet-only initially?
- GPU passthrough: out of scope for v1 but worth noting as a future extension point
