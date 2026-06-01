# PHermes

**Personal Hermes** — a fully encrypted, self-hosted, portable AI orchestration appliance.
Plug it into any KVM-capable machine. Boot. Open a browser. Your [Hermes Agent](https://github.com/NousResearch/hermes-agent) environment is ready.

---

### After boot

```
 ____  _   _
|  _ \| | | | ___ _ __ _ __ ___   ___  ___
| |_) | |_| |/ _ \ '__| '_ ` _ \ / _ \/ __|
|  __/|  _  |  __/ |  | | | | | |  __/\__ \
|_|   |_| |_|\___|_|  |_| |_| |_|\___||___/

 ─────────────────────────────────────────────────────
 [ENCRYPTED]  [KVM-ISOLATED]  [SELF-HOSTED]  [PORTABLE]

   VM       macOS 15 Sequoia     ● running
   Hermes   gateway v0.4.1       ● ready
   Storage  LUKS2 / Btrfs / LVM  ● mounted
   Network  192.168.1.42         phermes.local

 → https://phermes.local
 ─────────────────────────────────────────────────────
```

### During build

```
 ╔══════════════════════════════════════════════════════════╗
 ║  phermes-build  ·  Secure AI appliance constructor      ║
 ╠══════════════════════════════════════════════════════════╣
 ║                                                          ║
 ║  ▸ Partitioning SSD ................................ done ║
 ║  ▸ Creating LUKS2 container ........ [████████░░] 80%   ║
 ║  ▸ Installing Proxmox VE .............. pending          ║
 ║  ▸ Configuring PHermes ................. pending          ║
 ║  ▸ Importing VM image .................. pending          ║
 ║                                                          ║
 ╚══════════════════════════════════════════════════════════╝
```

---

## What it is

PHermes packages Hermes Agent inside a QEMU/KVM virtual machine (macOS by default,
Windows and Linux also supported) running on a Proxmox VE host. The entire stack lives
on a single SSD. No cloud. No installation on the host. Unplug and take it anywhere.

**Boot chain:** `EFI → GRUB → Proxmox VE → QEMU/KVM VM → Hermes`

The Proxmox management interface is invisible to end users. PHermes replaces it with a
purpose-built web UI that exposes only what matters: VM console, Hermes dashboard, storage,
and system controls.

## Security

PHermes is designed for users who treat their AI environment as infrastructure worth protecting.

- **Full-disk encryption.** LUKS2 wraps everything except `/boot` and the EFI partition.
  Data at rest is unreadable without the passphrase. Theft of the drive means nothing.
- **VM isolation.** Hermes runs inside a VM, never on the host. The Proxmox host surface
  is minimal, hardened, and not exposed to the network.
- **Network lockdown.** Proxmox management (port 8006) is firewalled to localhost —
  reachable only via SSH tunnel. Samba is bound to the internal VM bridge, not the LAN.
- **No binaries distributed.** `phermes-build` fetches Proxmox from official community
  repos at build time. The tool itself is signed and checksummed. Nothing is bundled or
  pre-compiled by PHermes.
- **No mandatory cloud.** LLM API calls are the only optional outbound traffic. Everything
  else runs entirely offline. Local model backends (llama.cpp, vLLM, Ollama) are supported.
- **Snapshot-before-change.** Every VM switch and every update triggers an automatic
  LVM-thin snapshot of the VM disk and a Btrfs snapshot of user data before touching
  anything. One command rolls everything back.

## Personalisation

PHermes is not a fixed appliance. It adapts to how you work.

- **Guest OS.** Start with macOS (default), switch to Windows or Linux at any time.
  Your data follows you across switches via a shared Samba mount.
- **LLM backend.** Anthropic, OpenAI, AWS Bedrock, or any local model served via
  llama.cpp, vLLM, LiteLLM, or Ollama. Configured at first boot, changeable anytime.
- **Storage split.** Choose how much of the SSD goes to the persistent data partition
  vs. the exFAT drag-and-drop share. Default: 330 GB / 250 GB on a 1 TB drive.
- **VM specs.** RAM, disk size, and display configuration are set at build time and
  adjustable later via the PHermes UI.
- **Bring your own images.** Pre-stage a VM disk before running `phermes-build`,
  download at first boot, or use the `phermes-build --download-vm` pipeline.
  PHermes never distributes OS images.
- **Knowledge base grows with you.** Hermes profiles, tasks, memory, and documents
  live on the encrypted data partition and persist across guest OS switches, updates,
  and even full re-flashes of the system partition.

## Status

Early development. Building in public — starting from the design spec.

Design spec: [`docs/superpowers/specs/2026-05-31-phermes-design.md`](docs/superpowers/specs/2026-05-31-phermes-design.md)

## Hardware requirements

- Any x86-64 machine with KVM support (Intel VT-x or AMD-V)
- ≥32 GB RAM recommended (16 GB minimum)
- ≥500 GB SSD (1 TB recommended; 2 TB+ for large knowledge bases)

## Distribution

PHermes ships as `phermes-build` — a signed Python CLI that assembles a bootable SSD
from official upstream sources. No Proxmox, macOS, or Windows binaries are bundled.

```bash
# Minimal — first-boot wizard handles VM image acquisition
phermes-build /dev/sdX

# With a pre-staged macOS image
phermes-build /dev/sdX --import-vm macos=/path/to/macos.qcow2

# Download VM images at build time
phermes-build /dev/sdX --download-vm macos --download-vm windows

# Custom share partition
phermes-build /dev/sdX --share-size 500G --share-encrypted
```

> **macOS note:** macOS VMs may only be run on Apple hardware under the macOS EULA.
> PHermes does not distribute macOS images; users supply their own installer.

## License

PHermes own code: Apache-2.0.
Proxmox VE (fetched from official repos at build time): AGPL-3.0.
Hermes Agent: MIT.
