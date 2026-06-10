# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added
- `phermes-build` cloud-init NoCloud seed (slice #4a): when run with
  `--dev-credentials --dev-ssh-pubkey <key>`, a `seed.iso` (label CIDATA) is
  generated and attached to the Linux guest as a CDROM. The seed contains a
  `dev` user with the operator's SSH key (key-only login, locked password),
  DHCP, and a `uv` installer in vendor-data. Slice #1's `DiskInterface` gains
  a `Cdrom` variant; `qemu.rs` emits `-drive media=cdrom,readonly=on,…` +
  `-device ide-cd,…`. Production builds ship no seed.
- `phermes-build` host image migration (slice #6 MVP): the assembled appliance now boots a
  **minimal Debian host running phermesd** instead of Proxmox VE. The boot chain is
  `EFI → GRUB → Debian → systemd → phermesd → KVM Linux guest`. Management is `phermesctl`
  over SSH; `--import-vm linux=<path>` preserves install-time VM provisioning; a new
  `--no-vm` flag installs the host alone. `proxmox.py` and `node_vm.py` deleted; the
  proven LUKS/LVM-thin/Btrfs/exFAT/GRUB-removable/Dropbear/Samba/Avahi plumbing reused.
- `phermesd` (Rust): core VM orchestrator daemon (slice #1) — TOML-defined VMs,
  QEMU/KVM spawn + supervision over QMP (qapi-rs), graceful stop, status, and
  restart re-adopt. Replaces Proxmox VE for single-active-VM operation. UDS control
  protocol + `phermesctl` client.
- `phermesd` storage & snapshots (slice #2): LVM-thin VM-disk provisioning + local image
  import, QGA-quiesced checkpoints of the VM disk and Btrfs overlay as a unit, auto-snapshot
  before a VM switch, retention pruning, thin-pool capacity guard, and one-command rollback,
  via `phermesctl provision/snapshot/rollback/snapshots`.
