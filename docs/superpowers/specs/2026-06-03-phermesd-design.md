# phermesd — Core Orchestrator Daemon (Design)

**Date:** 2026-06-03
**Status:** Draft — pending implementation plan
**Sub-project:** #1 of the `phermesd` replacement for Proxmox VE

---

## Context & motivation

PHermes currently runs guest VMs on a **Proxmox VE** host. Building the appliance proved
the thesis end-to-end (LUKS2-encrypted SSD → Proxmox host → nested Linux node → Hermes
Agent), but it also showed how little of Proxmox we actually use: `qm`, storage, cloud-init,
and the console are thin shims over QEMU/KVM/LVM/Linux bridges, while Proxmox's
clustering/management machinery (`pmxcfs`, the web UI, corosync, the Perl stack) is overhead
we firewall off and work *around* (every recent build fix — node-name resolution, node-dir
init, `storage.cfg`, the `phermes-pve-init` shim — was that machinery fighting a single-node
appliance).

`phermesd` replaces Proxmox with a thin Rust VM orchestrator on a minimal, hardened host.
Motivation: a small, **auditable** trusted surface aligned with the
[STRIDE threat model](../../threat-model.md) ("host minimal and trusted"); **no AGPL
Proxmox** dependency (cleaner for selling appliances); the original "immutable Alpine-style
host" vision; and full control of VM behavior (relevant to a future fixed-install UKI/TPM
mode).

## Scope: this is sub-project #1 of several

`phermesd` decomposes into independent sub-projects, each its own spec → plan → build:

1. **Core orchestrator daemon** — *this spec*. VM definition model + spawn/supervise QEMU.
2. **Storage & snapshots** — LVM-thin/qcow2 volumes, image import, snapshot-before-change.
3. **Networking** — bridge + NAT.
4. **Console + cloud-init** — VNC/serial proxy for the PHermes UI; NoCloud seed generation.
5. **macOS path** — OpenCore image + `-cpu`/`applesmc`/SMBIOS recipe, ported from OSX-PROXMOX.
6. **Host image** — the minimal/immutable base `phermesd` ships on, and `phermes-build`
   changes to assemble it instead of Proxmox.

Order: #1 is the foundation; #2–#4 extend it; #5 specializes #1+#2; #6 packages it. This
spec covers **#1 only**.

### Boundary (what #1 does and does not do)

`phermesd` (slice #1) **consumes** what a definition references — a *prebuilt* disk image
path and an *existing* bridge name — and:

- **does** define, spawn, supervise, gracefully stop, and report status of one active VM,
  surviving its own restart (re-adopt).
- **does not** provision storage, create bridges, proxy the console, or generate cloud-init.
  It *reports* the VM's VNC/serial socket paths so the future console proxy (#4) can use them.

Success criterion: boot the existing Debian node VM under `phermesd` on a `/dev/kvm` host —
`activate → running → status → stop → stopped`, and **re-adopt** (restart `phermesd`; the
VM keeps running).

## Decisions (resolved during brainstorming)

| Decision | Choice | Rationale |
|---|---|---|
| Control model | **Hybrid**: file-defined VMs + explicit activation, no reconcile loop | Matches PHermes's "one active VM, switch on command"; file-based defs are agent-/wizard-editable; a desired-state controller is overkill |
| Supervision | **`phermesd` supervises QEMU directly** | Init-agnostic — **no systemd/distro-service coupling** (host base may be OpenRC/Alpine); precise QMP lifecycle control; tiny trusted surface |
| Transport | **Unix-domain socket**, line-delimited JSON | Smallest surface — no network listener; an HTTP adapter layers on later |
| QMP/QGA | **`qapi-rs`** (`qmp`+`tokio` features; `qga` later) | Schema-typed protocol = correctness; QGA needed by #2 (snapshot quiescing); avoids a hand-roll→replace churn. Pin to the shipped QEMU's schema |

## Architecture

```
  phermesctl ─┐                         /etc/phermes/vms/*.toml   (definitions, source of truth)
  (UI later) ─┴──UDS JSON──▶ phermesd ──reads──┘
                              │
                              ├─ spawns ─▶ qemu-system-x86_64 (own session; survives phermesd)
                              │                 │
                              └─ QMP (qapi-rs) ─┘  /run/phermesd/<id>/qmp.sock
                              │
                              └─ /run/phermesd/state.json + <id>/vm.pid   (re-adopt after restart)
```

Internal units (one responsibility each, independently testable):

| Unit | Responsibility | Depends on |
|---|---|---|
| `config` | Load + validate VM defs (TOML → typed structs) | toml/serde |
| `qemu` | Build `qemu-system-x86_64` argv from a def — **pure `def → Vec<OsString>` function** | — |
| `supervisor` | Spawn/monitor the QEMU process; the lifecycle state machine | tokio::process, qemu, qmp, state |
| `qmp` | Drive QMP (status, `system_powerdown`, events) | qapi-rs, tokio |
| `state` | Persist/restore runtime state for re-adopt | serde_json |
| `control` | UDS server: parse JSON request → dispatch → reply | tokio::net, serde_json |
| `cli` | `phermesctl` (thin UDS client) | clap |

The `qemu` unit being a **pure function** is the deliberate seam where the macOS recipe (#5)
and storage/net (#2/#3) inject their pieces later without touching the supervisor.

## VM definition model

A static TOML file at `/etc/phermes/vms/<id>.toml` — the source of truth for *what a VM is*
(the id is the filename). Runtime paths are derived by `phermesd` under `/run/phermesd/<id>/`,
keeping the def read-only and host-independent.

```toml
# /etc/phermes/vms/linux.toml
flavor = "linux"            # linux | windows | macos  (slice #1 implements linux)

[resources]
memory_mib = 2048
vcpus = 2
cpu = "host"               # -cpu

[firmware]                 # UEFI
ovmf_code = "/usr/share/OVMF/OVMF_CODE.fd"
ovmf_vars_template = "/usr/share/OVMF/OVMF_VARS.fd"   # copied per-VM on first activate

[[disk]]
path = "/var/lib/phermes/images/linux-node.qcow2"     # prebuilt — slice #1 consumes it
format = "qcow2"
interface = "virtio-scsi"

[[net]]
bridge = "vmbr0"           # existing bridge — slice #1 consumes it
model = "virtio-net"
mac = "52:54:00:ab:cd:ef"  # optional; phermesd derives a stable one from the id if omitted

[console]
serial = true              # expose a serial socket
vnc = true                 # expose a VNC socket
```

- **`flavor` enum** drives the `qemu` command-builder's dispatch. Slice #1 implements
  `linux`; `windows`/`macos` are known-future arms. macOS's OpenCore/`applesmc`/SMBIOS will
  be a structured `[macos]` block in #5 — **not** a raw arg escape hatch.
- **Per-VM NVRAM**: `ovmf_vars_template` is copied to `/run/phermesd/<id>/OVMF_VARS.fd` on
  first activate; each VM gets its own writable UEFI vars and the def stays immutable.
- **Lists** for `disk`/`net` so multi-disk/multi-NIC is natural later; slice #1 uses one each.
- **No `extra_args`** — every field is typed so defs stay auditable and the builder stays a
  total function.

## Control protocol (UDS)

One JSON request → one JSON response per line on `/run/phermesd/control.sock`. Permission is
gated by socket ownership/mode. The command set is **transport-agnostic** (the later HTTP
adapter maps routes onto these).

| Command | Args | Behaviour | Returns |
|---|---|---|---|
| `list` | — | All defined VMs + which is active | `[{id, flavor, state}]` |
| `status` | `id?` | Detail for a VM (or the active one) | `{id, state, pid?, qmp, serial?, vnc?, since?}` |
| `activate` | `id` | Make `id` active. **Implicit switch**: graceful-stop any running VM first, then start `id` | `{id, state, vnc?, serial?}` |
| `stop` | `id?` | Graceful `system_powerdown` (timeout → SIGKILL) | `{id, state}` |
| `reload` | — | Re-scan the defs dir | `[{id, ...}]` |

`state ∈ { defined, starting, running, stopping, stopped, failed }`.

Envelope:
```json
→ {"cmd":"activate","id":"linux"}
← {"ok":true,"data":{"id":"linux","state":"running","vnc":"/run/phermesd/linux/vnc.sock","serial":"/run/phermesd/linux/serial.sock"}}
← {"ok":false,"error":{"kind":"already_active","message":"linux is already running"}}
```

- **`activate` is the switch** — no separate verb; activating B while A runs stops A then
  starts B (the one-active model as a single intent).
- **Request/response only** for slice #1; the UI polls `status`. A push/event stream is
  deferred to the HTTP adapter (SSE/WS) — not baked into the UDS layer.
- **`reload` is explicit**, not a file-watcher — consistent with "no reconcile loop".

## Lifecycle & supervision

State machine per VM: `defined → starting → running → stopping → stopped`, `failed` on error.

**Activate `id`:**
1. If another VM is `running`, run its **Stop** sequence first (the switch).
2. Create `/run/phermesd/<id>/`; copy `ovmf_vars_template` → per-VM `OVMF_VARS.fd`.
3. `qemu` unit builds argv — `-qmp unix:…/qmp.sock,server,nowait`, `-serial unix:…/serial.sock,…`,
   `-vnc unix:…/vnc.sock`, `-pidfile …/vm.pid`, `-machine q35,accel=kvm`, OVMF pflash
   (code + per-VM vars), `-cpu`, memory, disks, nets.
4. Spawn `qemu-system-x86_64` **in its own session** (`setsid` via `nix`, **no**
   `PR_SET_PDEATHSIG`) so the VM **survives a `phermesd` restart**.
5. Connect QMP, run `qmp_capabilities`, `query-status` → `running`. Record pid + socket
   paths in `state.json`.

**Monitor (running):** hold the QMP connection and react to the `SHUTDOWN` event / connection
drop; `kill(pid, 0)` liveness as a backstop. Unexpected exit → `failed` (sockets cleaned,
logs kept); graceful exit → `stopped`.

**Stop `id`:** QMP `system_powerdown` → await `SHUTDOWN`/exit up to `stop_timeout`
(default 30s) → on timeout, `SIGKILL` the pid → clean runtime dir + state.

**Re-adopt on `phermesd` startup:** read `state.json`; for each VM marked `running`, check
`vm.pid` liveness **and** reconnect QMP. Alive → resume monitoring (no restart). Dead/stale
→ mark `stopped`, clean up.

**Key invariant:** a VM's lifetime is tied to *QEMU + its pidfile*, **not** to the `phermesd`
process. `phermesd` is an observer/controller of a process that outlives it — exactly what
`qm` gave us, and what a plain `tokio` child (dies with parent) would not. This is what makes
`phermesd` restart-/upgrade-safe.

## Filesystem & privileges

```
/etc/phermes/vms/<id>.toml        VM definitions (read-only at runtime)
/etc/phermes/phermesd.toml        daemon config (paths, default stop_timeout) — minimal
/var/lib/phermes/images/          prebuilt disk images (consumed; provisioning is #2)
/usr/share/OVMF/                  firmware (edk2/ovmf on the host)
/run/phermesd/                    runtime (tmpfs; re-derived on reboot):
  control.sock                    UDS control socket
  state.json                      running VMs: id, pid, sockets, flavor, started_at
  <id>/{qmp.sock, serial.sock, vnc.sock, vm.pid, OVMF_VARS.fd}
/usr/bin/{phermesd, phermesctl}
```

**Privilege model (slice #1):** `phermesd` runs privileged (needs `/dev/kvm`, the bridge tap,
disk images). Dropping *QEMU* to a confined uid + capabilities (libvirt-style) is **deferred**
to the host-image/hardening sub-project — noted, not done here.

## Error handling

- Typed domain errors (`thiserror`): `Config`, `Spawn`, `Qmp`, `State`; `anyhow` at the top.
  Failures surface over the UDS as `{"ok":false,"error":{kind,message}}` — fail fast, actionable.
- **No orphans, no half-state:** a failed `activate` (QEMU spawns but QMP unreachable within
  timeout) kills the process, cleans the runtime dir, and leaves state consistent. A VM is
  `running` in `state.json` *only* once QMP confirms it.
- Re-adopt tolerates stale/dead entries (clean, don't crash).

## Testing (TDD; mock the boundaries)

- **Unit:** the `qemu` command-builder is a pure `def → argv` function → assert argv per
  flavor/def with **no QEMU** (highest-value tests). Plus config parse/validate, `state`
  round-trip, control-protocol framing. Property tests (`proptest`) on the argv builder.
- **Integration:** a **mock QMP server** (a UDS speaking the capabilities handshake + canned
  responses/events) drives the supervisor without real QEMU — covers stop-timeout→SIGKILL
  and re-adopt (seed `state.json` + a fake alive/dead pid).
- **End-to-end (gated, like the smoke harness):** boot the existing Debian node under
  `phermesd` on a `/dev/kvm` host — `activate→running→status→stop→stopped`, plus re-adopt
  (restart `phermesd`; VM still running). The slice-#1 success criterion.

## Tech stack

Rust (latest stable). Crates: **tokio** (rt, process, net/UDS, signal, time) · **serde** +
**serde_json** · **toml** · **anyhow** + **thiserror** · **tracing** + **tracing-subscriber**
· **clap** v4 · **nix** (PID liveness, `setsid`, signals) · **qapi** (`qmp`+`tokio`). House
Rust lints from the global standards (clippy pedantic, `unwrap_used`/`panic` denied, etc.).

Deliberately **out** for slice #1: no HTTP framework (UDS only; `axum` when the adapter lands),
no libvirt/virt-wrapper crates, no QEMU-abstraction crate.

## Out of scope (this slice)

Storage/image provisioning, bridge/NAT setup, console proxying, cloud-init/NoCloud generation,
the macOS recipe, the host image and `phermes-build` migration, QEMU privilege-dropping, an
event/push channel, and the HTTP adapter — each is a later sub-project or hardening pass.

## Open questions (for the plan or later slices)

- Exact `qapi-rs` version ↔ QEMU version pin (decide when the host-image QEMU is chosen in #6).
- Whether `phermesd.toml` is needed in slice #1 or defaults suffice (lean toward defaults).
- MAC derivation scheme from the VM id (stable, locally-administered range `52:54:00:…`).
