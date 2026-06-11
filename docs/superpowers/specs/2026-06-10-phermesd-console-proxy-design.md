# phermesd — Console Proxy (Design)

**Date:** 2026-06-10
**Status:** Draft — pending implementation plan
**Sub-project:** #4b of the `phermesd` replacement for Proxmox VE

---

## Context & motivation

Slice #1 wires the guest's serial console and VNC display to per-VM Unix sockets
(`/run/phermesd/<id>/serial.sock`, `vnc.sock`); the slice-#6 appliance puts the operator
inside an SSH session on the host. There is, today, no ergonomic way to attach to a guest's
console — the operator has to know about `socat - UNIX-CONNECT:<path>` or hand-roll an
`ssh -L` to the VNC socket.

#4b adds a `phermesctl console <id>` subcommand that opens the serial socket directly,
puts the operator's terminal into raw mode, proxies bytes both ways with a Ctrl-]
detach key, and restores the terminal cleanly on every exit path. VNC is doc-only —
modern OpenSSH forwards Unix sockets directly (`ssh -L 5900:<vnc.sock> <host>`), so the
operator's VNC workflow doesn't need any phermesd-side code.

Linux guest only for the MVP (matches #4a scope). macOS + Windows ride along when they
land in #5 — the serial proxy is flavor-agnostic, so it Just Works once those flavors
exist in the def model.

## Decisions (resolved during brainstorming)

| Decision | Choice |
|---|---|
| Transport | **Serial via `phermesctl console`, VNC via documented `ssh -L`** — no new LAN-facing surface, no static assets, no TLS. The full noVNC/web bridge returns when Phase 2 builds the proper web UI. |
| Byte path | **`phermesctl` opens `serial.sock` directly** after discovering the path via the existing `Request::Status` UDS call. UDS protocol stays pure request/response JSON. Daemon never sees the attach. |
| Escape sequence | **Ctrl-]** (familiar from `telnet`, no `qm terminal` baggage). |
| Multi-attach | **Single-attach** — already enforced by QEMU's unix-server socket semantics; no phermesd-side enforcement needed. |
| Layer | **In-process Rust proxy** in phermesctl using existing `nix` + `tokio` deps — no new crates, no `socat` shell-out. |

## Scope boundary

**In:**
- New `phermesctl` subcommand `console <id>` and its supporting module
  (`bin/phermesctl/console.rs`).
- Convert `phermesctl` from a single-file binary to a small directory layout
  (`bin/phermesctl/main.rs` + `bin/phermesctl/console.rs`) so the proxy module is testable
  in isolation. One-time mechanical move.
- RAII raw-mode TTY guard using `nix::sys::termios`.
- Bidirectional byte pump (`stdin → serial.sock`, `serial.sock → stdout`) with Ctrl-]
  detection, flush-on-write for interactive responsiveness, and SIGINT/SIGTERM handling.
- Typed `ConsoleError` rendered on stderr; non-zero exit codes for failure modes.
- Doc additions in `README.md` and `CHANGELOG.md` covering both the serial verb and the
  VNC-over-SSH-forward workflow.

**Out (deferred):**
- Web / noVNC bridge → Phase 2 (PHermes web UI).
- Audit logging of byte streams → security hardening pass.
- Reconnect-on-disconnect — operator re-runs the command.
- Scrollback — would need a host-side recorder, separate slice.
- Window-size negotiation / SIGWINCH propagation — serial is character-mode.
- Read-only attach mode — useful for shadowing; defer.
- Multi-attach broadcast — QEMU enforces single-attach, and changing that would require
  the daemon to fan out bytes (out of scope).

**Prerequisite:** the implementation branch should stack on slice #4a
(`feat/phermesd-cloud-init`, PR #23) so all open phermesd PRs land in order. #4b touches
only `phermesctl`'s file layout + adds new files; it does not depend on #4a's behavior, so
in principle a bare-`main` base also works once PR #17 is merged (it is). Stacking is
recommended for consistency with the established pattern.

## Architecture & file structure

| File | Change |
|---|---|
| `phermesd/src/bin/phermesctl.rs` | **Renamed** to `phermesd/src/bin/phermesctl/main.rs`. Content otherwise unchanged except a new `Cmd::Console { id: String }` enum variant and the dispatch arm that handles it (see below). |
| `phermesd/src/bin/phermesctl/console.rs` | **New.** `ConsoleError`, `RawTtyGuard`, `find_escape`, `run_pump`, `run_console`. |
| `phermesd/Cargo.toml` | `[[bin]] name = "phermesctl"` already has `path = "src/bin/phermesctl.rs"`; update to `"src/bin/phermesctl/main.rs"`. |
| `README.md` | New subsection under the existing `### phermesd` block: serial console + VNC over SSH-forward. |
| `CHANGELOG.md` | `### Added` entry for slice #4b. |

**Slice #1/#2 reach:** none. The UDS control protocol is unchanged. The supervisor is
unchanged. The storage layer is unchanged. Only `phermesctl` grows.

**Dependencies:** zero new crates. `nix` is already present (slice #1 used it for
`setsid`/signals); we add the `term` feature if not already enabled for `nix::sys::termios`,
and confirm `pty` is on for the test (cheap to enable). `tokio::io::duplex` is in the
existing feature set; `tokio::signal::unix` likewise.

## The `console` subcommand's lifecycle

```
phermesctl console <id>
  │
  ▼
1. Send Request::Status { id: Some(id) } over the existing UDS.
     ─ Response ok=false → ConsoleError::Control(msg) → stderr + exit 1
     ─ Response ok=true  → parse VmInfo from data
  │
  ▼
2. Extract vm_info.serial.
     ─ None    → ConsoleError::NotActive(id) → stderr + exit 1
     ─ Some(p) → continue
  │
  ▼
3. tokio::net::UnixStream::connect(p).
     ─ ECONNREFUSED → ConsoleError::Connect{ path, source } → stderr + exit 1
     ─ Ok(stream)  → split into (read_half, write_half)
  │
  ▼
4. RawTtyGuard::enter(stdin_fd).
     - tcgetattr → save original Termios.
     - Clear ICANON, ECHO, ISIG, IEXTEN; clear ICRNL, IXON.
     - tcsetattr(stdin, TCSANOW, raw).
     - Drop impl restores original on EVERY exit (incl. panic unwind).
  │
  ▼
5. tokio::select! over four futures:
     a. stdin → write_half — read chunk; scan for 0x1d (Ctrl-]); on hit, send
        bytes up to (not incl.) escape, then break the select.
     b. read_half → stdout — read chunk, write+flush stdout. EOF → "Disconnected."
        on stderr, break the select.
     c. signal::unix::SIGINT → break the select.
     d. signal::unix::SIGTERM → break the select.
  │
  ▼
6. RawTtyGuard drops → tcsetattr restores original termios.
   Print "Detached." on stderr. Exit 0.
```

### Why `ISIG` is cleared

By default the terminal driver translates Ctrl-C → SIGINT in the *operator's* process
(phermesctl). With `ISIG` off, Ctrl-C is a plain `0x03` byte that flows to the guest —
which is what the operator wants. SIGINT/SIGTERM are still observed at the *Rust* level
via `tokio::signal::unix` (those handlers are independent of the terminal driver), so the
operator can still kill phermesctl from another tty if needed.

### `find_escape` semantics

```rust
fn find_escape(bytes: &[u8]) -> Option<usize> { bytes.iter().position(|&b| b == 0x1d) }
```

Pure function. The stdin pump:
1. Reads a chunk into `buf`.
2. If `find_escape(&buf)` is `Some(i)`, writes `&buf[..i]` to the socket and breaks
   (any bytes after the escape are discarded — typing rapidly across the detach is rare;
   if it matters the operator can re-attach and retype).
3. Otherwise writes the whole chunk and continues.

### `flush` discipline

Raw mode disables the terminal driver's line buffering, but Rust's `tokio::io::stdout()`
has an internal buffer. We `flush().await` after each socket-→-stdout write so the
operator sees output immediately.

### What does NOT happen

- No reconnect on disconnect — operator re-runs the command. (Saves a state machine.)
- No scrollback replay — QEMU's serial socket is stream-only; pre-attach output is gone.
- No window-size propagation — serial is character-mode; no SIGWINCH.
- No audit/logging — defer to a security pass.

## Errors

```rust
#[derive(Debug, thiserror::Error)]
pub enum ConsoleError {
    #[error("VM '{0}' is not active (no serial socket); activate it first")]
    NotActive(String),
    #[error("connecting to {path}: {source}")]
    Connect { path: String, #[source] source: std::io::Error },
    #[error("UDS control protocol: {0}")]
    Control(String),
    #[error("terminal io: {0}")]
    Tty(String),
}
```

`main()` matches and prints to stderr; `exit 1` for all failure modes. The clean exit
paths (Ctrl-] / Ctrl-C / SIGTERM / guest closed the socket) print a one-word status
("Detached." or "Disconnected.") on stderr and exit 0, so piping stdout works.

## Testing

Three layers; each tests a distinct seam.

### Unit (escape detection)

`find_escape` is a free function. Parameterized table:

| Input | Expected |
|---|---|
| `b""` | `None` |
| `b"hello"` | `None` |
| `b"\x1d"` | `Some(0)` |
| `b"hi\x1dthere"` | `Some(2)` |
| `b"end\x1d"` | `Some(3)` |

(Property test if `proptest` is convenient, otherwise a table.)

### Unit (RawTtyGuard against a pty pair)

Using `nix::pty::openpty`:
1. Open `(master, slave)`.
2. `let guard = RawTtyGuard::enter(slave.as_raw_fd())?;`
3. `tcgetattr(slave)` — assert `ICANON`/`ECHO`/`ISIG`/`IEXTEN`/`ICRNL`/`IXON` are cleared.
4. `drop(guard);`
5. `tcgetattr(slave)` — assert original flags restored.

No real stdin involved; works on any Linux test runner.

### Integration (the pump)

`run_pump` is parameterized over `R: AsyncRead + Unpin` and `W: AsyncWrite + Unpin` for the
socket side, and another pair for stdin/stdout. The real `run_console` is a thin wrapper
wiring the actual types. The test uses `tokio::io::duplex` for both pairs:

```rust
let (sock_client, sock_server) = tokio::io::duplex(64);   // fake serial.sock
let (stdin_w, stdin_r)         = tokio::io::duplex(64);   // fake stdin
let stdout = Vec::<u8>::new();                            // fake stdout
// spawn run_pump(sock_client, &mut stdout, stdin_r)
// drive sock_server and stdin_w from the test
```

Assertions (table-driven):

1. **Bidirectional bytes flow.** Write "hello" on the server side → `stdout` contains
   "hello". Write "world" on stdin_w → `sock_server` reads "world".
2. **Ctrl-] breaks the loop.** Write "abc\x1ddef" on stdin_w → `sock_server` reads
   "abc"; pump task completes.
3. **EOF on the server breaks the loop.** Drop `sock_server` → pump task completes,
   "Disconnected" status surfaced.
4. **flush after every chunk.** Write 1-byte chunks; assert `stdout` sees them one at a
   time, not buffered (compare bytes-written after each tick).

### Out of scope for automated tests

The full `phermesctl console linux` flow against a real UDS + real QEMU. Operator-verified
in the smoke harness (success criterion below).

## Success criterion (operator-verified)

1. Build + boot a slice-#6 appliance with `--dev-credentials --dev-ssh-pubkey <key>
   --import-vm linux=<debian-cloud.qcow2>` (slice #4a provisions cloud-init).
2. SSH in as root.
3. `phermesctl activate linux`.
4. `phermesctl console linux` — operator sees the cloud-init / Debian login prompt;
   logs in as `dev` (cloud-init configured); reaches a shell; Ctrl-C at a shell prompt
   sends SIGINT to the guest's foreground process (not to phermesctl).
5. Ctrl-] — terminal restored; "Detached." on stderr; exit 0.
6. From a workstation:
   `ssh -L 5900:/run/phermesd/linux/vnc.sock <appliance>`
   then `vncviewer localhost:5900` — shows the guest's framebuffer.

## Open questions (for the plan)

- **`tokio::signal::unix` vs. `ctrlc` crate** — slice #1 doesn't use either today. Lean
  `tokio::signal::unix` since it's already in the existing tokio feature set; confirm at
  plan time that the relevant feature flag is on.
- **`nix` feature flags** — confirm `term` (for `sys::termios`) and `pty` (for the unit
  test) are on; add to `phermesd/Cargo.toml` if not. Both are tiny.
- **Exact `Termios` flag mask** — Section "Why ISIG is cleared" lists the input/local
  flags to clear. The plan should snapshot a canonical "raw mode" mask (cf. `cfmakeraw`
  semantics) and stick to it; small differences (`OPOST` on/off) matter for how the
  guest's `\n` looks on the operator's tty.
