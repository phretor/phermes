# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added
- `phermesd` (Rust): core VM orchestrator daemon (slice #1) — TOML-defined VMs,
  QEMU/KVM spawn + supervision over QMP (qapi-rs), graceful stop, status, and
  restart re-adopt. Replaces Proxmox VE for single-active-VM operation. UDS control
  protocol + `phermesctl` client.
- `phermesd` storage & snapshots (slice #2): LVM-thin VM-disk provisioning + local image
  import, QGA-quiesced checkpoints of the VM disk and Btrfs overlay as a unit, auto-snapshot
  before a VM switch, retention pruning, thin-pool capacity guard, and one-command rollback,
  via `phermesctl provision/snapshot/rollback/snapshots`.
