# PHermes

**Personal Hermes** — a bootable SSD appliance that runs [Hermes Agent](https://github.com/NousResearch/hermes-agent) inside a full GUI VM on any KVM-capable hardware.

Plug it in. Boot. Open a browser. Your AI orchestration environment is ready.

## What it is

PHermes packages Hermes Agent inside a QEMU/KVM virtual machine (macOS by default, Windows and Linux also supported) running on a Proxmox VE host. The entire stack lives on a single SSD. No cloud. No installation on the host. Unplug and take it anywhere.

**Boot chain:** `EFI → GRUB → Proxmox VE → QEMU/KVM VM → Hermes`

## Status

Early development. Building in public — starting from the design spec.

Design spec: [`docs/superpowers/specs/2026-05-31-phermes-design.md`](docs/superpowers/specs/2026-05-31-phermes-design.md)

## Hardware requirements

- Any x86-64 machine with KVM support (Intel VT-x or AMD-V)
- ≥32 GB RAM recommended
- ≥500 GB SSD (1 TB recommended, 2 TB+ for large knowledge bases)

## Distribution

PHermes ships as `phermes-build` — a CLI tool that assembles a bootable SSD from official upstream sources. No Proxmox, macOS, or Windows binaries are bundled.

```bash
# Minimal — first-boot wizard handles VM image acquisition
phermes-build /dev/sdX

# With pre-staged macOS image
phermes-build /dev/sdX --import-vm macos=/path/to/macos.qcow2
```

> **macOS note:** macOS VMs may only be run on Apple hardware under the macOS EULA.
> PHermes does not distribute macOS images; users supply their own installer.

## License

PHermes own code: Apache-2.0.
Proxmox VE (fetched from official repos at build time): AGPL-3.0.
Hermes Agent: MIT.
