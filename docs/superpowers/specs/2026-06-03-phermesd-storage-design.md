# phermesd — Storage & Snapshots (Design)

**Date:** 2026-06-03
**Status:** Draft — pending implementation plan
**Sub-project:** #2 of the `phermesd` replacement for Proxmox VE

---

## Context & motivation

[Slice #1](2026-06-03-phermesd-design.md) built the core orchestrator: it defines VMs from
TOML and spawns/supervises one QEMU/KVM guest, but it **consumes prebuilt disk paths** — it
does not create, populate, snapshot, or roll back storage. Sub-project #2 adds the runtime
storage layer.

The intent is set by the existing infrastructure and the [STRIDE threat
model](../../threat-model.md): VM disks live on **LVM-thin** (instant copy-on-write
snapshots), the persistent Hermes data lives on **Btrfs** (`@overlay`), and the appliance
offers **snapshot-before-change with one-command rollback** (the DoS/"bad update bricks the
system" mitigation, TB7). The Proxmox build already produced exactly this shape
(`local-lvm` thin pool, `vm-<id>-disk-N` volumes, Btrfs subvolumes); #2 keeps those
conventions and drives them from phermesd instead of `qm`/`pmxcfs`.

## Decisions (resolved during brainstorming)

| Decision | Choice |
|---|---|
| VM-disk backend | **LVM-thin raw volumes**, reusing Proxmox conventions (VG `pve`, pool `data`, `vm-<vmid>-disk-N`). Persistent data stays on Btrfs `@overlay`. Two snapshot domains. |
| Snapshot policy | **Auto-snapshot before a VM switch** + **manual** `snapshot`/`rollback`; **both domains** (disk + overlay); retention on auto snapshots. |
| Provisioning | Create the thin LV + optionally import a **local source image** (or leave blank); network downloads stay upstream. |
| Consistency | **Live snapshots via QGA** `fsfreeze`/`thaw`, falling back to crash-consistent if the guest agent is absent. Pulls the qapi-rs `qga` feature into the codebase. |
| Integration | **Approach 1** — phermesd-native storage modules behind mockable trait seams; the **system is the source of truth** (`lvs`, `btrfs subvolume list`); no separate state file. |

## Scope boundary

**In:** provision/import/delete LVM-thin VM-disk volumes; live (QGA-quiesced) and cold
checkpoints of *disk + overlay together*; retention/prune; rollback of both domains; a
thin-pool capacity guard; new control commands + `phermesctl` verbs; auto-checkpoint wired
into the switch path.

**Out:** creating the VG/pool/Btrfs container (build-time / host-image #6); image
**acquisition** (downloads/URLs/ISO fetch — upstream); **def authoring**
(`/etc/phermes/vms/<id>.toml` is written by #6 or a higher-level command — #2 only ensures
the *volume* a def references exists); the macOS recipe (#5); the web UI (Phase 2).

**Assumed to exist:** VG `pve` with thin pool `data`, and the Btrfs overlay volume mounted at
`/var/lib/phermes/overlay`. #2 operates strictly within them.

## Architecture & units

phermesd gains a runtime storage layer structured like slice #1: side effects behind
mockable trait seams, orchestration logic pure and unit-testable, real LVM/Btrfs exercised
only in gated loop-device tests.

| Unit | Responsibility | Seam |
|---|---|---|
| `lvm.rs` | thin-volume create/remove/snapshot/merge/list/tag | `LvmOps` trait + real impl shelling to `lvcreate`/`lvremove`/`lvconvert`/`lvs`/`lvchange`; pure argv builders unit-tested |
| `btrfs.rs` | overlay subvolume snapshot/list/delete/restore | `BtrfsOps` trait + real impl (`btrfs subvolume …`) |
| `qga.rs` | guest quiesce: `guest-ping`, `guest-fsfreeze-freeze`/`-thaw` | `QgaControl` trait + qapi-rs **`qga`** impl over a per-VM `qga.sock` (parallel to slice #1's `QmpControl`) |
| `storage.rs` | policy orchestrator: provision, import, checkpoint, rollback, prune, pool-guard | composes `LvmOps`+`BtrfsOps`+`QgaControl`; mockable in tests |

**Surgical touches to slice #1:**

- `qemu.rs` — extend the argv builder and `RuntimePaths` to add a guest-agent virtio-serial
  channel (`-device virtio-serial-pci` + a `virtserialport,chardev=…,name=org.qemu.guest_agent.0`
  on a unix `chardev`) bound to a new `qga.sock`. This is what lets QGA reach the guest.
- `supervisor.rs` — connect and hold the active VM's `QgaControl` alongside its `QmpControl`;
  call `storage` for the auto-checkpoint before a switch.
- `proto.rs`/`control.rs`/`cli.rs` — the new verbs (below).

## Storage model, ownership & naming

- **VM-disk volumes:** thin LVs `vm-<vmid>-disk-<n>` in `pve/data`, fixed VM IDs from slice #1
  (macOS=100, Windows=101, Linux=102). The phermesd def's `[[disk]]` points at
  `path=/dev/pve/vm-<vmid>-disk-0, format=raw, interface=virtio-scsi` — **no def-model change**.
- **Ownership:** phermesd only snapshots/rolls-back/deletes volumes it owns, marked by an
  **LVM tag `@phermesd`** set at provision time (not inferred from naming). External paths
  (e.g., a slice-#1 qcow2 file) are left untouched. "List managed" = `lvs` filtered by tag.
- **Overlay domain:** host-side Btrfs `@overlay` (mounted `/var/lib/phermes/overlay`, shared
  into the guest via Samba). Snapshotted host-side into `@snapshots/overlay-<…>` — no guest
  involvement, always host-FS-consistent.
- **Snapshot identity (name carries the metadata — no DB):**
  - VM disk: `vm-<vmid>-disk-<n>-snap-<kind>-<utc>`, `kind ∈ {auto, manual}`, `utc` sortable
    (e.g. `…-snap-auto-20260603T141500Z`), tagged `@phermesd-snap`.
  - Overlay: `@snapshots/overlay-<kind>-<utc>`.
  - A **checkpoint** = the disk snapshot and overlay snapshot sharing the same `<utc>`.
- **State = the system:** listing/grouping/pruning derive from `lvs` (by tag) +
  `btrfs subvolume list`. Nothing to drift.

## Provisioning, import & delete

`provision <vmid> [--from <source>] [--size <GB>]`:

1. Verify `pve/data` exists; pick size from the flavor default (macOS 120 / Windows 100 /
   Linux 40 GB) or `--size`.
2. `lvcreate --thin --virtualsize <size>G pve/data -n vm-<vmid>-disk-0`, then
   `lvchange --addtag @phermesd /dev/pve/vm-<vmid>-disk-0`.
3. If `--from <source>` (local qcow2/raw/img): validate the source virtual size ≤ volume size
   (`qemu-img info`), then populate with `qemu-img convert -O raw <source>
   /dev/pve/vm-<vmid>-disk-0`. On failure, remove the half-populated volume — no half-state.
4. No `--from` → leave blank (install-from-ISO / OpenCore later).

**Idempotency/safety:** `provision` refuses to overwrite an existing managed volume unless
`--force` (remove + recreate). It only ever touches `@phermesd`-tagged volumes.

`delete <vmid>`: refuse if the VM is active; remove the volume's snapshots, then `lvremove`
the origin. Tagged volumes only.

## Snapshot model

A snapshot is a **checkpoint**: VM disk + overlay captured together, tied by a shared UTC.

`snapshot <vmid>` (manual) and the internal auto-path both run:

1. **Quiesce (best-effort):** if the VM is *active* and its QGA channel answers `guest-ping`,
   call `guest-fsfreeze-freeze`. If QGA is absent/unresponsive (agent not installed,
   mid-install), skip → crash-consistent, warn, tag the checkpoint as such. A *stopped* VM
   needs no quiesce — its disk is already cold.
2. **Snapshot VM disk:** `lvcreate --snapshot -n vm-<vmid>-disk-0-snap-<kind>-<utc>
   pve/vm-<vmid>-disk-0` (instant CoW), tag `@phermesd-snap`.
3. **Snapshot overlay (host-side):** `btrfs subvolume snapshot -r …/overlay
   …/@snapshots/overlay-<kind>-<utc>`.
4. **Thaw:** `guest-fsfreeze-thaw` — in a finally-style guard so the guest is **never left
   frozen**, on every path.

**Atomicity:** freeze → both snapshots → thaw. If a snapshot fails after freeze, thaw runs,
any partial snapshot is removed, and the error propagates — a checkpoint is all-or-nothing.
The freeze window is just the two instant CoW ops.

**Retention:** keep the last **N auto** checkpoints per VM (default 5, configurable);
**manual checkpoints are never auto-pruned**. After each auto-snapshot, prune older auto
checkpoints (removing both the LV snap and the overlay snap of that UTC).

**Thin-pool safety guard:** thin snapshots consume pool space as the origin diverges; an
exhausted pool freezes *all* thin volumes. Before snapshotting, check `lvs` data%; if the
pool is above a threshold (default 90%), refuse the auto-snapshot with a clear warning rather
than risk exhaustion.

**QGA channel:** `qga.rs` connects the active VM's `qga.sock` at activate (parallel to
`QmpControl`); the supervisor holds it. Only the active guest is ever quiesced.

## Rollback

`rollback <vmid> <checkpoint-utc>` — **the VM must be stopped** (LVM merge cannot run on a
live origin; reverting a running guest's disk is unsafe). Refuse if active, with `--force` to
stop-then-roll.

- **VM disk:** `lvconvert --merge pve/vm-<vmid>-disk-0-snap-<kind>-<utc>` reverts the origin
  to the snapshot. *Caveat:* LVM merge also discards snapshots of that origin taken **after**
  the chosen one.
- **Overlay:** Btrfs has no in-place merge — delete the live `@overlay` and recreate it as a
  writable snapshot of the chosen read-only `@snapshots/overlay-<kind>-<utc>`, with the share
  idle (VM stopped); phermesd remounts the new `@overlay`.
- Both revert to the same UTC → a consistent pair.

## Control protocol & supervisor integration

New `Request` variants (proto.rs) + `control.rs` dispatch + `phermesctl` subcommands, all
using the existing `{ok, data | error{kind,message}}` envelope:

| Command | Args | Returns |
|---|---|---|
| `provision` | `vmid, from?, size?, force?` | volume info |
| `delete` | `vmid` | ok |
| `snapshot` | `vmid` | checkpoint info |
| `rollback` | `vmid, checkpoint` | ok |
| `snapshots` | `vmid?` | checkpoints grouped by UTC (kind, consistency, created-at) |

**Supervisor integration:** `activate` connects and holds the active VM's `QgaControl` (needs
the guest-agent channel above). On a switch from a running VM, it takes an **auto** checkpoint
before stopping the outgoing VM. A failed safety snapshot (e.g., pool-guard refusal) **warns
loudly and continues** the switch — a user who asked to switch is not trapped; manual
`snapshot`/`rollback` instead hard-fail on error.

## Filesystem & config

- Reuses `/var/lib/phermes/images/` (source images) and `/var/lib/phermes/overlay`
  (Btrfs `@overlay` mount).
- Storage settings in `phermesd.toml` with defaults: pool `pve/data`, overlay path,
  retention `N = 5`, pool threshold `90%`.
- No new state files — the system is the source of truth.

## Error handling

Typed `StorageError` (thiserror): `Lvm`, `Btrfs`, `Qga`, `PoolFull`, `SourceTooLarge`,
`NotManaged`, `VmActive`, `NotFound` → mapped to UDS envelope kinds. Fail fast with context;
the thaw-guard always releases a freeze; partial provision/snapshot/rollback is cleaned so no
half-state is left.

## Testing

- **Unit:** pure argv builders for `lvm`/`btrfs`/`qga` commands (highest value); orchestration
  logic with mock `LvmOps`/`BtrfsOps`/`QgaControl` — assert freeze→snap→thaw ordering,
  thaw-on-failure, retention/prune math, pool-guard refusal, atomic rollback-on-partial.
- **Integration (gated, loop device + root):** a real LVM-thin pool + Btrfs overlay on loop
  devices → provision, import a tiny image, snapshot, list, rollback, prune; a hand-rolled
  **mock QGA server** (like slice #1's mock-QMP wire test) for the freeze/thaw path without a
  real guest agent.
- **E2E (gated, KVM + guest agent):** boot a guest with `qemu-guest-agent`, live-snapshot it
  (verify quiesce via `guest-ping`), switch VMs (auto-checkpoint), and roll back. The
  slice-#2 success criterion.

## Success criterion

Provision a thin VM-disk from a local image; auto-checkpoint it (disk + overlay,
QGA-quiesced) on a switch; list checkpoints; and roll back to a prior checkpoint with the VM
stopped — verified on loop devices (LVM/Btrfs) and, gated, against a real guest agent.

## Open questions (for the plan)

- Exact `qapi-rs` `qga` event/command surface for `guest-fsfreeze-*` (confirm at plan time
  against the pinned version).
- Btrfs overlay rollback while Samba may hold the mount — confirm the unmount/remount sequence
  and whether the share must be stopped (likely yes; the VM is already stopped).
- Whether `phermesd.toml` gains a `[storage]` table now or defaults suffice for the first cut.
