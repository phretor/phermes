# phermesd Console Proxy Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `phermesctl console <id>` puts the operator's terminal in raw mode, opens the active VM's `/run/phermesd/<id>/serial.sock`, proxies bytes both ways, detaches on Ctrl-], and restores the terminal cleanly on every exit path — without touching the daemon or the UDS protocol.

**Architecture:** Pure phermesctl change. Convert the binary from a single file to a small directory (`bin/phermesctl/{main,console}.rs`) so the proxy module is unit-testable in isolation. New `console.rs` exposes a pure `find_escape` function, a RAII `RawTtyGuard` over `nix::sys::termios`, a generic `run_pump` over `AsyncRead`/`AsyncWrite` (tested with `tokio::io::duplex`), and a thin `run_console` that does the UDS Status round-trip → `UnixStream::connect` → raw-mode → race the pump against SIGINT/SIGTERM.

**Tech Stack:** Rust (no new crates); `nix` gains the `term` and `pty` feature flags (already in the crate); `tokio` (already has `signal`, `net`, `io-util`); `thiserror`, `anyhow`, `clap` (all already in use).

**Spec:** `docs/superpowers/specs/2026-06-10-phermesd-console-proxy-design.md`

**Prerequisite:** stack the implementation branch on slice #4a (`feat/phermesd-cloud-init`, PR #23) so the `phermesctl` baseline already has the post-#2 storage verbs and the slice #6 changes are present. Bare-`main` works too once #4a/#6/#2 are merged, but stacking is the established pattern.

---

## File Structure

```
phermesd/
  Cargo.toml                          (modify)  # nix features += term, pty; [[bin]] path
  src/bin/phermesctl.rs                (delete after move) — moves to phermesctl/main.rs
  src/bin/phermesctl/main.rs           (new, moved content) # existing CLI + new Console arm
  src/bin/phermesctl/console.rs        (new)     # find_escape, ConsoleError, RawTtyGuard,
                                                #   run_pump, run_console
README.md                              (modify)
CHANGELOG.md                           (modify)
```

The proxy lives entirely in `console.rs`. `main.rs` is the CLI dispatch — it grows by one new `Cmd::Console { id }` variant plus a small `if let Cmd::Console { id } = &args.cmd { return console::run_console(...).await; }` short-circuit before the existing Request/Response flow.

---

### Task 1: Move `phermesctl.rs` → `phermesctl/main.rs` (no behavior change)

**Files:**
- Delete: `phermesd/src/bin/phermesctl.rs`
- Create: `phermesd/src/bin/phermesctl/main.rs` (same content, moved)
- Modify: `phermesd/Cargo.toml` — `[[bin]] name = "phermesctl"` block, update `path`

- [ ] **Step 1: Move the file with `git mv`**

```bash
cd /home/u/dev/phermes/phermes/phermesd
mkdir -p src/bin/phermesctl
git mv src/bin/phermesctl.rs src/bin/phermesctl/main.rs
```

- [ ] **Step 2: Update `[[bin]]` path in Cargo.toml**

Open `phermesd/Cargo.toml`. Find the `[[bin]]` block for phermesctl:

```toml
[[bin]]
name = "phermesctl"
path = "src/bin/phermesctl.rs"
```

Change `path` to:

```toml
[[bin]]
name = "phermesctl"
path = "src/bin/phermesctl/main.rs"
```

(Leave the `[[bin]]` block for `phermesd` itself unchanged.)

- [ ] **Step 3: Verify the binary still builds + the existing suite passes**

```bash
cd /home/u/dev/phermes/phermes/phermesd && cargo build --bin phermesctl && cargo test && cargo clippy --all-targets --all-features -- -D warnings
```

Expected: builds, all existing tests green, clippy clean (this is a pure file rename — nothing else changed).

- [ ] **Step 4: Smoke check that the CLI surface is identical**

```bash
cd /home/u/dev/phermes/phermes/phermesd && ./target/debug/phermesctl --help 2>&1 | head -10
```

Expected: the same help text as before (list, status, activate, stop, reload, plus #2 and #4a verbs depending on base).

- [ ] **Step 5: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add phermesd/Cargo.toml phermesd/src/bin/phermesctl
git commit -m "refactor(phermesd): move phermesctl to bin/phermesctl/{main}.rs directory"
```

---

### Task 2: Enable `nix` `term` and `pty` feature flags

**Files:**
- Modify: `phermesd/Cargo.toml`

- [ ] **Step 1: Edit Cargo.toml**

Find the `nix` dependency line:

```toml
nix = { version = "0.31.3", features = ["signal", "process"] }
```

Change to:

```toml
nix = { version = "0.31.3", features = ["signal", "process", "term", "pty"] }
```

- [ ] **Step 2: Build to confirm the features compile cleanly**

```bash
cd /home/u/dev/phermes/phermes/phermesd && cargo build && cargo test && cargo clippy --all-targets --all-features -- -D warnings
```

Expected: all green; no new warnings.

- [ ] **Step 3: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add phermesd/Cargo.toml phermesd/Cargo.lock
git commit -m "build(phermesd): enable nix term + pty features for console proxy"
```

---

### Task 3: `console.rs` — `find_escape` + `ConsoleError`

**Files:**
- Create: `phermesd/src/bin/phermesctl/console.rs`
- Modify: `phermesd/src/bin/phermesctl/main.rs` (one `mod console;` line + use)

- [ ] **Step 1: Create console.rs with `ConsoleError` and `find_escape` + their tests**

Create `phermesd/src/bin/phermesctl/console.rs`:

```rust
//! Console proxy for `phermesctl console <id>`.
//!
//! Opens the per-VM serial socket directly (after discovering its path via the
//! existing UDS Status round-trip), puts the operator's terminal in raw mode,
//! and proxies bytes both ways until Ctrl-] or a signal.

use std::path::PathBuf;

/// Detach key: ASCII 0x1d (Ctrl-]).
pub const ESCAPE_BYTE: u8 = 0x1d;

#[derive(Debug, thiserror::Error)]
pub enum ConsoleError {
    #[error("VM '{0}' is not active (no serial socket); activate it first")]
    NotActive(String),
    #[error("connecting to {path}: {source}")]
    Connect {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("UDS control protocol: {0}")]
    Control(String),
    #[error("terminal io: {0}")]
    Tty(String),
}

/// Return the index of the first ESCAPE_BYTE in `bytes`, or `None`.
#[must_use]
pub fn find_escape(bytes: &[u8]) -> Option<usize> {
    bytes.iter().position(|&b| b == ESCAPE_BYTE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_escape_returns_none_for_empty_and_clean_chunks() {
        assert_eq!(find_escape(b""), None);
        assert_eq!(find_escape(b"hello"), None);
        assert_eq!(find_escape(b"\x00\x01\x02"), None);
    }

    #[test]
    fn find_escape_returns_index_at_start_middle_end() {
        assert_eq!(find_escape(b"\x1d"), Some(0));
        assert_eq!(find_escape(b"hi\x1dthere"), Some(2));
        assert_eq!(find_escape(b"end\x1d"), Some(3));
    }

    #[test]
    fn find_escape_returns_first_occurrence_only() {
        assert_eq!(find_escape(b"\x1dabc\x1d"), Some(0));
        assert_eq!(find_escape(b"ab\x1dcd\x1def"), Some(2));
    }
}
```

- [ ] **Step 2: Register the module from main.rs**

Open `phermesd/src/bin/phermesctl/main.rs`. At the very top, BEFORE the `use` lines, add:

```rust
mod console;
```

Do not add any `use console::…` yet — Task 6 wires the dispatch.

- [ ] **Step 3: Run the new tests + the whole suite**

```bash
cd /home/u/dev/phermes/phermes/phermesd && cargo test --bin phermesctl
cd /home/u/dev/phermes/phermes/phermesd && cargo test
```

Expected: 3 new `console::tests::*` tests pass; whole crate suite stays green. The `#[cfg(test)] mod tests` block inside `console.rs` is compiled with the binary's test runner; `cargo test --bin phermesctl` runs them.

- [ ] **Step 4: Clippy**

```bash
cd /home/u/dev/phermes/phermes/phermesd && cargo clippy --all-targets --all-features -- -D warnings
```

Expected: clean.

- [ ] **Step 5: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add phermesd/src/bin/phermesctl
git commit -m "feat(phermesd): console.rs — find_escape + ConsoleError"
```

---

### Task 4: `RawTtyGuard` (RAII raw-mode termios + pty-pair test)

**Files:**
- Modify: `phermesd/src/bin/phermesctl/console.rs`

- [ ] **Step 1: Add `RawTtyGuard` to console.rs (above the `#[cfg(test)] mod tests`)**

Append to `phermesd/src/bin/phermesctl/console.rs` (after `find_escape`, before `#[cfg(test)] mod tests`):

```rust
use nix::sys::termios::{
    self, ControlFlags, InputFlags, LocalFlags, OutputFlags, SetArg, Termios,
};
use std::os::fd::{AsRawFd, BorrowedFd, RawFd};

/// RAII guard that puts a tty into raw mode and restores the original termios
/// on drop. Drop fires on every exit path, including panic unwind — the
/// operator's terminal is never left in raw mode.
pub struct RawTtyGuard {
    fd: RawFd,
    original: Termios,
}

impl RawTtyGuard {
    /// Switch `fd` to raw mode. `fd` MUST be a tty (caller checks).
    ///
    /// # Errors
    /// Returns `ConsoleError::Tty` if `tcgetattr` or `tcsetattr` fails.
    pub fn enter(fd: BorrowedFd<'_>) -> Result<Self, ConsoleError> {
        let raw = fd.as_raw_fd();
        let original = termios::tcgetattr(fd)
            .map_err(|e| ConsoleError::Tty(format!("tcgetattr: {e}")))?;
        let mut raw_mode = original.clone();
        // Input flags: don't translate CR->NL, don't pause on Ctrl-S/Q.
        raw_mode.input_flags.remove(
            InputFlags::IGNBRK
                | InputFlags::BRKINT
                | InputFlags::PARMRK
                | InputFlags::ISTRIP
                | InputFlags::INLCR
                | InputFlags::IGNCR
                | InputFlags::ICRNL
                | InputFlags::IXON,
        );
        // Output: leave OPOST on so guest "\n" becomes "\r\n" on the operator's tty.
        // (The guest's output is text the operator reads, not bytes piped elsewhere.)
        let _ = OutputFlags::OPOST;
        // Local flags: no echo, no canonical mode, no signal generation
        // (Ctrl-C must reach the guest as a byte).
        raw_mode.local_flags.remove(
            LocalFlags::ECHO
                | LocalFlags::ECHONL
                | LocalFlags::ICANON
                | LocalFlags::ISIG
                | LocalFlags::IEXTEN,
        );
        // 8-bit characters; no parity bit.
        raw_mode.control_flags.remove(ControlFlags::CSIZE | ControlFlags::PARENB);
        raw_mode.control_flags.insert(ControlFlags::CS8);
        termios::tcsetattr(fd, SetArg::TCSANOW, &raw_mode)
            .map_err(|e| ConsoleError::Tty(format!("tcsetattr (raw): {e}")))?;
        Ok(Self { fd: raw, original })
    }
}

impl Drop for RawTtyGuard {
    fn drop(&mut self) {
        // Safety: the fd was valid when we entered; we never close it ourselves.
        // If the operator closed it (e.g., process exit), tcsetattr will fail
        // silently — there is no useful action here.
        let fd = unsafe { BorrowedFd::borrow_raw(self.fd) };
        let _ = termios::tcsetattr(fd, SetArg::TCSANOW, &self.original);
    }
}
```

- [ ] **Step 2: Append a pty-pair test to the `#[cfg(test)] mod tests` block**

In `phermesd/src/bin/phermesctl/console.rs`, inside the existing `mod tests` block (under the `find_escape` tests), append:

```rust
    #[test]
    fn raw_tty_guard_clears_canonical_flags_and_restores_on_drop() {
        use nix::pty::openpty;
        use std::os::fd::AsFd;

        let pty = openpty(None, None).expect("openpty");
        // We poke at the slave; the master is held to keep the pair open.
        let slave_fd = pty.slave.as_fd();

        let original = termios::tcgetattr(slave_fd).expect("tcgetattr (pre)");
        assert!(original.local_flags.contains(LocalFlags::ICANON));
        assert!(original.local_flags.contains(LocalFlags::ECHO));
        assert!(original.local_flags.contains(LocalFlags::ISIG));

        {
            let _guard = RawTtyGuard::enter(slave_fd).expect("enter raw");
            let raw = termios::tcgetattr(slave_fd).expect("tcgetattr (raw)");
            assert!(!raw.local_flags.contains(LocalFlags::ICANON));
            assert!(!raw.local_flags.contains(LocalFlags::ECHO));
            assert!(!raw.local_flags.contains(LocalFlags::ISIG));
            assert!(!raw.local_flags.contains(LocalFlags::IEXTEN));
            assert!(!raw.input_flags.contains(InputFlags::ICRNL));
            assert!(!raw.input_flags.contains(InputFlags::IXON));
        }
        // Guard dropped; flags restored.
        let after = termios::tcgetattr(slave_fd).expect("tcgetattr (post)");
        assert_eq!(after.local_flags, original.local_flags);
        assert_eq!(after.input_flags, original.input_flags);

        drop(pty);
    }
```

- [ ] **Step 3: Run the tests**

```bash
cd /home/u/dev/phermes/phermes/phermesd && cargo test --bin phermesctl
```

Expected: 4 console tests pass (3 from Task 3 + 1 new).

- [ ] **Step 4: Clippy**

```bash
cd /home/u/dev/phermes/phermes/phermesd && cargo clippy --all-targets --all-features -- -D warnings
```

Expected: clean. The `let _ = OutputFlags::OPOST;` line is a deliberate no-op comment-as-code to document that we intentionally leave OPOST on; if clippy flags `clippy::no_effect`, replace it with a plain `// OPOST left on intentionally — see Section "Why ISIG is cleared" in the spec.` comment and delete the let.

- [ ] **Step 5: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add phermesd/src/bin/phermesctl/console.rs
git commit -m "feat(phermesd): RawTtyGuard for raw-mode termios with RAII restore"
```

---

### Task 5: `run_pump` — generic bidirectional async byte pump

**Files:**
- Modify: `phermesd/src/bin/phermesctl/console.rs`

- [ ] **Step 1: Append `PumpExit` + `run_pump` to console.rs**

Add at the bottom of the production code (above `#[cfg(test)] mod tests`):

```rust
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Why the pump returned.
#[derive(Debug, PartialEq, Eq)]
pub enum PumpExit {
    /// Operator hit Ctrl-].
    Detached,
    /// The remote (QEMU's serial socket) closed.
    Disconnected,
}

/// Bidirectional byte pump. Reads from `stdin_r` and writes to `sock_w` (with
/// Ctrl-] detection); reads from `sock_r` and writes to `stdout_w` (flushed
/// after every chunk for interactive responsiveness). Returns on the first of:
///   * `stdin_r` yields a chunk containing 0x1d (PumpExit::Detached)
///   * `sock_r` reports EOF (PumpExit::Disconnected)
///   * Either side errors (returns ConsoleError::Tty).
///
/// Signals are handled by the caller — keep this function pure async I/O so it
/// is testable with `tokio::io::duplex`.
///
/// # Errors
/// Returns `ConsoleError::Tty` on any I/O failure during the pump.
pub async fn run_pump<SR, SW, IR, OW>(
    mut sock_r: SR,
    mut sock_w: SW,
    mut stdin_r: IR,
    mut stdout_w: OW,
) -> Result<PumpExit, ConsoleError>
where
    SR: AsyncRead + Unpin,
    SW: AsyncWrite + Unpin,
    IR: AsyncRead + Unpin,
    OW: AsyncWrite + Unpin,
{
    let mut stdin_buf = [0u8; 4096];
    let mut sock_buf = [0u8; 4096];
    loop {
        tokio::select! {
            // stdin -> sock_w, with Ctrl-] detection
            r = stdin_r.read(&mut stdin_buf) => {
                let n = r.map_err(|e| ConsoleError::Tty(format!("stdin read: {e}")))?;
                if n == 0 {
                    // stdin closed (e.g., operator redirected from /dev/null)
                    return Ok(PumpExit::Detached);
                }
                let chunk = &stdin_buf[..n];
                if let Some(idx) = find_escape(chunk) {
                    if idx > 0 {
                        sock_w.write_all(&chunk[..idx]).await
                            .map_err(|e| ConsoleError::Tty(format!("sock write: {e}")))?;
                    }
                    return Ok(PumpExit::Detached);
                }
                sock_w.write_all(chunk).await
                    .map_err(|e| ConsoleError::Tty(format!("sock write: {e}")))?;
            }
            // sock_r -> stdout_w, with flush after every chunk
            r = sock_r.read(&mut sock_buf) => {
                let n = r.map_err(|e| ConsoleError::Tty(format!("sock read: {e}")))?;
                if n == 0 {
                    return Ok(PumpExit::Disconnected);
                }
                stdout_w.write_all(&sock_buf[..n]).await
                    .map_err(|e| ConsoleError::Tty(format!("stdout write: {e}")))?;
                stdout_w.flush().await
                    .map_err(|e| ConsoleError::Tty(format!("stdout flush: {e}")))?;
            }
        }
    }
}
```

- [ ] **Step 2: Append four integration tests to the `mod tests` block**

```rust
    #[tokio::test]
    async fn pump_forwards_bytes_in_both_directions() {
        use tokio::io::AsyncWriteExt as _;
        let (sock_client, mut sock_server) = tokio::io::duplex(64);
        let (sock_r, sock_w) = tokio::io::split(sock_client);
        let (stdin_w_tx, stdin_r) = tokio::io::duplex(64);
        let (_unused_r, mut stdin_w) = tokio::io::split(stdin_w_tx);
        let stdout = Vec::<u8>::new();

        let pump = tokio::spawn(async move {
            run_pump(sock_r, sock_w, stdin_r, stdout).await
        });

        // operator types "world" on stdin -> sock_server should read it
        stdin_w.write_all(b"world").await.unwrap();
        let mut buf = [0u8; 5];
        tokio::io::AsyncReadExt::read_exact(&mut sock_server, &mut buf).await.unwrap();
        assert_eq!(&buf, b"world");

        // guest emits "hello" on sock_server -> need to capture stdout from the
        // pump task; close stdin and sock_server cleanly to end the pump.
        sock_server.write_all(b"hello").await.unwrap();
        drop(sock_server);            // sock EOF -> Disconnected
        drop(stdin_w);

        let result = pump.await.unwrap().unwrap();
        // The pump returned via EOF; we cannot inspect `stdout` after the move,
        // so use a channel-style test below instead.
        assert_eq!(result, PumpExit::Disconnected);
    }

    #[tokio::test]
    async fn pump_writes_stdout_bytes_and_flushes() {
        use std::sync::Arc;
        use tokio::io::AsyncWriteExt as _;
        use tokio::sync::Mutex;

        // Wrap stdout in an Arc<Mutex<Vec<u8>>> we can read from outside the pump.
        let stdout: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
        // Use a thin AsyncWrite shim around the Arc<Mutex<Vec<u8>>>.
        struct ArcWriter(Arc<Mutex<Vec<u8>>>);
        impl tokio::io::AsyncWrite for ArcWriter {
            fn poll_write(
                self: std::pin::Pin<&mut Self>,
                _cx: &mut std::task::Context<'_>,
                buf: &[u8],
            ) -> std::task::Poll<std::io::Result<usize>> {
                let mut guard = self.0.try_lock().expect("uncontended in test");
                guard.extend_from_slice(buf);
                std::task::Poll::Ready(Ok(buf.len()))
            }
            fn poll_flush(
                self: std::pin::Pin<&mut Self>,
                _cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<std::io::Result<()>> {
                std::task::Poll::Ready(Ok(()))
            }
            fn poll_shutdown(
                self: std::pin::Pin<&mut Self>,
                _cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<std::io::Result<()>> {
                std::task::Poll::Ready(Ok(()))
            }
        }

        let (sock_client, mut sock_server) = tokio::io::duplex(64);
        let (sock_r, sock_w) = tokio::io::split(sock_client);
        let (_stdin_w_unused, stdin_r) = tokio::io::duplex(64);
        let writer = ArcWriter(stdout.clone());

        let pump = tokio::spawn(async move {
            run_pump(sock_r, sock_w, stdin_r, writer).await
        });

        sock_server.write_all(b"hello").await.unwrap();
        // Give the pump a tick to copy the bytes.
        tokio::task::yield_now().await;
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        {
            let guard = stdout.lock().await;
            assert_eq!(guard.as_slice(), b"hello");
        }
        drop(sock_server);
        let result = pump.await.unwrap().unwrap();
        assert_eq!(result, PumpExit::Disconnected);
    }

    #[tokio::test]
    async fn pump_returns_detached_on_ctrl_bracket() {
        use tokio::io::AsyncWriteExt as _;

        let (sock_client, mut sock_server) = tokio::io::duplex(64);
        let (sock_r, sock_w) = tokio::io::split(sock_client);
        let (stdin_w_side, stdin_r) = tokio::io::duplex(64);
        let (_unused_r, mut stdin_w) = tokio::io::split(stdin_w_side);
        let stdout = Vec::<u8>::new();

        let pump = tokio::spawn(async move {
            run_pump(sock_r, sock_w, stdin_r, stdout).await
        });

        // Send bytes followed by Ctrl-]. The bytes before the escape should
        // reach sock_server; the pump should return PumpExit::Detached.
        stdin_w.write_all(b"abc\x1ddef").await.unwrap();

        let mut buf = [0u8; 3];
        tokio::io::AsyncReadExt::read_exact(&mut sock_server, &mut buf).await.unwrap();
        assert_eq!(&buf, b"abc");

        let result = pump.await.unwrap().unwrap();
        assert_eq!(result, PumpExit::Detached);
    }

    #[tokio::test]
    async fn pump_returns_disconnected_on_socket_eof() {
        let (sock_client, sock_server) = tokio::io::duplex(64);
        let (sock_r, sock_w) = tokio::io::split(sock_client);
        let (_unused_w, stdin_r) = tokio::io::duplex(64);
        let stdout = Vec::<u8>::new();

        let pump = tokio::spawn(async move {
            run_pump(sock_r, sock_w, stdin_r, stdout).await
        });

        drop(sock_server);  // EOF on the socket
        let result = pump.await.unwrap().unwrap();
        assert_eq!(result, PumpExit::Disconnected);
    }
```

- [ ] **Step 3: Run the tests**

```bash
cd /home/u/dev/phermes/phermes/phermesd && cargo test --bin phermesctl -- console::tests --nocapture
```

Expected: 8 console tests pass (3 + 1 RawTtyGuard + 4 pump).

- [ ] **Step 4: Clippy**

```bash
cd /home/u/dev/phermes/phermes/phermesd && cargo clippy --all-targets --all-features -- -D warnings
```

Expected: clean. Pedantic may complain about the `ArcWriter` boilerplate in the test; if it does, leave it — the test is the place that needs that exact shim.

- [ ] **Step 5: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add phermesd/src/bin/phermesctl/console.rs
git commit -m "feat(phermesd): run_pump — generic bidirectional pump with Ctrl-] detect"
```

---

### Task 6: `run_console` + `Cmd::Console` dispatch

**Files:**
- Modify: `phermesd/src/bin/phermesctl/console.rs`
- Modify: `phermesd/src/bin/phermesctl/main.rs`

- [ ] **Step 1: Append `run_console` to console.rs**

Add at the bottom of `console.rs` (above `#[cfg(test)] mod tests`):

```rust
use phermesd::proto::{encode_line, Request, Response, VmInfo};
use std::path::Path;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UnixStream;

/// Top-level entry for `phermesctl console <id>`.
///
/// 1. Sends Request::Status over the existing UDS to discover the serial.sock path.
/// 2. Connects to that Unix socket directly.
/// 3. Enters raw mode on stdin.
/// 4. Races run_pump against SIGINT and SIGTERM.
/// 5. Restores the terminal (RAII), prints a one-word status to stderr, returns.
///
/// # Errors
/// Returns `ConsoleError` for protocol, connect, and tty failures. Successful
/// exits (Detached, Disconnected, signal) return `Ok(())`.
pub async fn run_console(id: &str, control_sock: &Path) -> Result<(), ConsoleError> {
    // 1. Status round-trip
    let stream = UnixStream::connect(control_sock).await.map_err(|e| {
        ConsoleError::Connect { path: control_sock.to_path_buf(), source: e }
    })?;
    let (read, mut write) = stream.into_split();
    let req = Request::Status { id: Some(id.to_string()) };
    let line = encode_line(&req).map_err(|e| ConsoleError::Control(e.to_string()))?;
    tokio::io::AsyncWriteExt::write_all(&mut write, line.as_bytes())
        .await
        .map_err(|e| ConsoleError::Control(format!("write: {e}")))?;
    let mut lines = BufReader::new(read).lines();
    let reply = lines.next_line().await.map_err(|e| ConsoleError::Control(format!("read: {e}")))?
        .ok_or_else(|| ConsoleError::Control("daemon closed connection".into()))?;
    let resp: Response = serde_json::from_str(&reply).map_err(|e| ConsoleError::Control(format!("decode: {e}")))?;
    if !resp.ok {
        let kind = resp.error.as_ref().map_or("unknown", |e| e.kind.as_str());
        let msg = resp.error.as_ref().map_or("", |e| e.message.as_str());
        return Err(ConsoleError::Control(format!("{kind}: {msg}")));
    }
    let data = resp.data.ok_or_else(|| ConsoleError::Control("Status response missing data".into()))?;
    let info: VmInfo = serde_json::from_value(data).map_err(|e| ConsoleError::Control(format!("VmInfo: {e}")))?;
    let serial = info.serial.ok_or_else(|| ConsoleError::NotActive(id.to_string()))?;

    // 2. Open the serial socket
    let serial_stream = UnixStream::connect(&serial).await.map_err(|e| {
        ConsoleError::Connect { path: serial.clone(), source: e }
    })?;
    let (sock_r, sock_w) = serial_stream.into_split();

    // 3. Raw mode on stdin
    use std::os::fd::AsFd;
    let stdin = std::io::stdin();
    let _guard = RawTtyGuard::enter(stdin.as_fd())?;

    // 4. Race pump against signals
    let stdin_async = tokio::io::stdin();
    let stdout_async = tokio::io::stdout();
    let pump = run_pump(sock_r, sock_w, stdin_async, stdout_async);

    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .map_err(|e| ConsoleError::Tty(format!("install sigterm: {e}")))?;

    let exit_msg = tokio::select! {
        result = pump => {
            match result? {
                PumpExit::Detached => "Detached.",
                PumpExit::Disconnected => "Disconnected.",
            }
        }
        _ = tokio::signal::ctrl_c() => "Interrupted.",
        _ = sigterm.recv() => "Terminated.",
    };

    // 5. RawTtyGuard drops here -> terminal restored.
    // Print status to stderr so piping stdout is unaffected.
    eprintln!("{exit_msg}");
    Ok(())
}
```

- [ ] **Step 2: Wire `Cmd::Console { id }` into main.rs**

Open `phermesd/src/bin/phermesctl/main.rs`.

(a) In the `Cmd` enum, ADD a new variant at the end (alphabetical position is fine):

```rust
    /// Attach to the active VM's serial console (raw, Ctrl-] to detach).
    Console { id: String },
```

(b) The `impl From<Cmd> for Request` block currently maps every variant to a Request. Console doesn't have a corresponding Request (it uses the existing Status); leaving it out of the `From` impl would make the match non-exhaustive and fail to compile. Add a `Cmd::Console { .. } => unreachable!("Console is handled before From<Cmd>")` arm — but `unreachable!()` is denied by the project clippy config. Use this pattern instead: handle Console BEFORE the From conversion in `main()` so it never reaches the conversion. Implementation:

In `main()`, find the line:
```rust
let req: Request = args.cmd.into();
```

Replace the block around it. The existing `main()` ends with sending `req` and reading the response. Restructure to:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Console is handled out-of-band: it uses the existing Status round-trip
    // internally and then opens the serial.sock directly.
    if let Cmd::Console { id } = &args.cmd {
        return console::run_console(id, &args.socket).await.map_err(anyhow::Error::from);
    }

    let req: Request = args.cmd.into();
    // ... existing send/recv/print/exit-code logic unchanged ...
}
```

Because Console is handled by the short-circuit, the `From<Cmd> for Request` impl never sees it. Add this arm to the `From` impl to keep exhaustiveness:

```rust
            Cmd::Console { .. } => {
                // Handled by the short-circuit in main(); never reached here.
                Request::List
            }
```

(`Request::List` is a benign placeholder that will never run because of the short-circuit. This avoids `unreachable!()` and clippy.)

- [ ] **Step 3: Build, run the suite, smoke `--help`**

```bash
cd /home/u/dev/phermes/phermes/phermesd && cargo build --bin phermesctl && cargo test && cargo clippy --all-targets --all-features -- -D warnings
./target/debug/phermesctl --help 2>&1 | head -20
./target/debug/phermesctl console --help 2>&1 | head -5
```

Expected: builds, all tests pass, clippy clean; the `--help` output lists `console` as a subcommand; `console --help` shows the id positional argument.

- [ ] **Step 4: Live smoke (no daemon needed — just confirm the error path)**

```bash
./target/debug/phermesctl --socket /tmp/no-such.sock console linux 2>&1 | head -3
```

Expected: an error like "connecting to /tmp/no-such.sock: No such file or directory" on stderr; exit code 1.

- [ ] **Step 5: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add phermesd/src/bin/phermesctl
git commit -m "feat(phermesd): phermesctl console <id> subcommand"
```

---

### Task 7: README + CHANGELOG

**Files:**
- Modify: `README.md`
- Modify: `CHANGELOG.md`

- [ ] **Step 1: README**

Open `README.md`. Find the `### phermesd (in development)` subsection. At the end of its paragraph (after the existing slice #4a sentence), append:

```
Slice #4b (implemented): `phermesctl console <id>` attaches to the guest's serial line — raw terminal, Ctrl-] to detach. Ctrl-C reaches the guest (not phermesctl). VNC is doc-only: from a workstation, run `ssh -L 5900:/run/phermesd/linux/vnc.sock <appliance>` and point any VNC client at `localhost:5900`. Design: [`docs/superpowers/specs/2026-06-10-phermesd-console-proxy-design.md`](docs/superpowers/specs/2026-06-10-phermesd-console-proxy-design.md).
```

- [ ] **Step 2: CHANGELOG**

Open `CHANGELOG.md`. Under `## [Unreleased]` → `### Added`, prepend (above the slice-#4a entry):

```markdown
- `phermesctl console <id>` (slice #4b): attaches to the guest's serial console
  over `/run/phermesd/<id>/serial.sock` with raw-mode TTY and Ctrl-] detach.
  Ctrl-C reaches the guest; the operator's terminal is restored on every exit
  path (incl. panic). VNC is documented as `ssh -L 5900:.../vnc.sock <host>`
  with no new daemon code. No new dependencies; `nix` gains the `term`+`pty`
  feature flags.
```

- [ ] **Step 3: Verify everything still builds + tests pass + lints clean**

```bash
cd /home/u/dev/phermes/phermes/phermesd && cargo test && cargo clippy --all-targets --all-features -- -D warnings
```

Expected: all green.

- [ ] **Step 4: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add README.md CHANGELOG.md
git commit -m "docs: phermesd console proxy (slice #4b)"
```

---

## Out of Scope (carries to later slices)

- Web / noVNC bridge → Phase 2 (PHermes web UI).
- macOS + Windows guests → slice #5.
- Audit logging of byte streams → security hardening pass.
- Reconnect-on-disconnect.
- Scrollback replay.
- Window-size / SIGWINCH propagation.
- Read-only attach mode.
- Multi-attach broadcast (would need daemon-side fanout).

---

## Self-Review

**1. Spec coverage** — every spec section maps to a task:

| Spec section / requirement | Task(s) |
|---|---|
| File restructure: `bin/phermesctl.rs` → `bin/phermesctl/{main,console}.rs`; Cargo.toml `[[bin]] path` | 1 |
| `nix` `term`+`pty` feature flags | 2 |
| `find_escape` pure function + table tests | 3 |
| `ConsoleError` typed enum (NotActive, Connect, Control, Tty) | 3 |
| `RawTtyGuard` RAII with `tcgetattr`/`tcsetattr`; clears ICANON/ECHO/ISIG/IEXTEN/ICRNL/IXON | 4 |
| `RawTtyGuard` pty-pair test asserting restoration | 4 |
| `run_pump` generic over AsyncRead/AsyncWrite; two select arms (stdin↔socket); Ctrl-] semantics; flush after every socket→stdout chunk | 5 |
| `run_pump` integration tests via `tokio::io::duplex` (4 cases) | 5 |
| `run_console` Status round-trip → connect → raw mode → race against SIGINT/SIGTERM → restore | 6 |
| `Cmd::Console { id }` variant + main.rs short-circuit | 6 |
| README serial verb + VNC SSH-forward; CHANGELOG | 7 |
| Production-no-seed invariant | (not applicable — #4b doesn't change credentials) |
| Success criterion (operator-verified end-to-end smoke) | operator runs post-merge |

The "Open questions" from the spec are answered in the plan: `tokio::signal::unix` (already in features); `nix` `term`+`pty` (Task 2); the canonical Termios mask (Task 4 lists every flag).

**2. Placeholder scan** — none. The `Cmd::Console { .. } => Request::List` placeholder in Task 6 is concrete with an explicit rationale ("benign placeholder, never reached because of the short-circuit in main()") and avoids `unreachable!()` per the project's clippy config. The `let _ = OutputFlags::OPOST;` no-op in Task 4 is a documented intentional construct, with a fallback if clippy flags it.

**3. Type consistency** — `ConsoleError` (NotActive/Connect{path,source}/Control(String)/Tty(String)), `find_escape(&[u8]) -> Option<usize>`, `RawTtyGuard::enter(BorrowedFd) -> Result<Self, ConsoleError>`, `PumpExit::{Detached,Disconnected}`, `run_pump<SR,SW,IR,OW>(...) -> Result<PumpExit, ConsoleError>`, `run_console(id: &str, control_sock: &Path) -> Result<(), ConsoleError>`, `Cmd::Console { id: String }`. Names + signatures stay identical across tasks and tests.
