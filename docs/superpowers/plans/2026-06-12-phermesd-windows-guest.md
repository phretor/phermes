# phermesd Windows Guest Support Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `phermesctl activate windows` boots a Windows guest under phermesd, after `phermes-build --import-vm windows=<path>` populates the LVM-thin disk and writes `/etc/phermes/vms/windows.toml`. Both Linux and Windows VMs can be provisioned in one build.

**Architecture:** Mirror slice #4a's Linux pattern. The Rust change is one-line: rename `build_linux` → `build_pc_uefi` (it's already a generic q35+UEFI+virtio PC builder) and route `Flavor::Linux | Flavor::Windows` to it. The Python side adds `WINDOWS_VMID = 101` + companion constants, a `write_windows_def` + `provision_windows_disk` pair (byte-for-byte mirror of the Linux pair), generalizes `_linux_source` → `_vm_source(args, flavor)` to accept both `linux=` and `windows=`, and adds a `_provision_windows_vm` helper that `build()` appends as a second VM-provisioning step.

**Tech Stack:** No new dependencies. Rust (slice #1 def model + argv builder); Python 3.13 + uv + pytest (phermes-build). All existing.

**Spec:** `docs/superpowers/specs/2026-06-12-phermesd-windows-guest-design.md`

**Prerequisite:** stack the implementation branch on slice #4b (`feat/phermesd-console-proxy`, PR #24) so all open phermesd PRs land in order. Bare `main` works once PRs #19/#20/#23/#24 are merged.

---

## File Structure

```
phermesd/src/
  qemu.rs               (modify)  # rename build_linux -> build_pc_uefi; Windows dispatch

src/phermes_build/
  vm.py                 (modify)  # Windows constants + write_windows_def + provision_windows_disk
  cli.py                (modify)  # _vm_source generic; _provision_windows_vm; build() wiring
tests/phermes_build/
  test_vm.py            (modify)  # 6 new Windows tests
  test_cli.py           (modify)  # 4 new cli tests
README.md               (modify)
CHANGELOG.md            (modify)
```

The Rust change is the smallest — one rename + one match arm change. All other code is in Python and follows slice #4a's Linux pattern exactly, just substituting `windows` for `linux` and `101` for `102`.

---

### Task 1: `phermesd/src/qemu.rs` — rename + Windows dispatch

**Files:**
- Modify: `phermesd/src/qemu.rs`

- [ ] **Step 1: Inspect the current qemu.rs state**

Run:
```bash
cd /home/u/dev/phermes/phermes && grep -nE "fn build_linux|fn build_argv|UnsupportedFlavor|non_linux_flavor_is_unsupported|sample_vm\(\) ->|Flavor::Linux|Flavor::Windows|Flavor::Macos" phermesd/src/qemu.rs | head -25
```

Note the line numbers of `fn build_linux`, the dispatch `match vm.def.flavor` in `build_argv`, and the test `non_linux_flavor_is_unsupported`. You'll need these for the surgical edits below.

- [ ] **Step 2: Write the failing Windows test (in the existing `#[cfg(test)] mod tests` block)**

Append to the test module in `phermesd/src/qemu.rs`:

```rust
    #[test]
    fn windows_argv_is_byte_identical_to_linux_argv_for_equivalent_def() {
        // Build a Linux def and a Windows def with otherwise identical fields,
        // then assert build_argv produces byte-for-byte identical argv. The
        // Rust dispatch in build_argv routes both flavors through the same
        // build_pc_uefi function, so they SHOULD be identical.
        let lx = sample_vm();
        let mut win = sample_vm();
        win.def.flavor = Flavor::Windows;
        let argv_lx = build_argv(&lx, &rt()).unwrap();
        let argv_win = build_argv(&win, &rt()).unwrap();
        assert_eq!(argv_lx, argv_win);
    }
```

- [ ] **Step 3: Update the existing `non_linux_flavor_is_unsupported` test to assert `Macos` specifically**

Find the existing test in `phermesd/src/qemu.rs`:
```rust
    #[test]
    fn non_linux_flavor_is_unsupported() {
        let mut vm = sample_vm();
        vm.def.flavor = Flavor::Windows;  // OR Flavor::Macos
        let result = build_argv(&vm, &rt());
        assert!(matches!(result, Err(QemuError::UnsupportedFlavor(_))));
    }
```

The exact form may differ — replace it with this Macos-specific version:
```rust
    #[test]
    fn macos_flavor_is_unsupported() {
        let mut vm = sample_vm();
        vm.def.flavor = Flavor::Macos;
        let result = build_argv(&vm, &rt());
        assert!(matches!(result, Err(QemuError::UnsupportedFlavor(Flavor::Macos))));
    }
```

Note the rename `non_linux_flavor_is_unsupported` → `macos_flavor_is_unsupported`: Windows is no longer "non-Linux unsupported"; only macOS is.

- [ ] **Step 4: Run tests, verify the new one fails (Windows still UnsupportedFlavor) and the renamed one passes**

Run: `cd /home/u/dev/phermes/phermes/phermesd && cargo test --lib qemu 2>&1 | tail -15`
Expected: FAIL — `windows_argv_is_byte_identical_…` expects `Ok` but gets `Err(UnsupportedFlavor(Windows))`. `macos_flavor_is_unsupported` passes (Macos has always been Unsupported).

- [ ] **Step 5: Rename `fn build_linux` → `fn build_pc_uefi` and route Windows to it**

In `phermesd/src/qemu.rs`:

(a) Find the function declaration:
```rust
fn build_linux(vm: &Vm, rt: &RuntimePaths) -> Vec<String> {
```
Replace with:
```rust
fn build_pc_uefi(vm: &Vm, rt: &RuntimePaths) -> Vec<String> {
```

(b) Update the doc comment immediately above it to reflect the new name. If the comment says something like "Build the Linux argv vector," change it to:
```rust
/// Build the argv vector for a generic q35+UEFI+virtio PC. Used for both
/// Linux and Windows guests; their QEMU command lines are identical given
/// the same firmware/disk/net/console settings (only the in-guest OS
/// behavior differs). macOS uses a separate builder (slice #5b).
```

(c) Find every internal call to `build_linux` inside `qemu.rs` and rename to `build_pc_uefi`. There may be no internal calls; the function is just dispatched from `build_argv`. Grep to be sure:
```bash
cd /home/u/dev/phermes/phermes && grep -n "build_linux" phermesd/src/qemu.rs
```
Expected after rename: only test names like `linux_argv_…` remain; no `build_linux` function refs. Any remaining refs to `build_linux` are bugs — change them to `build_pc_uefi`.

(d) Find the dispatch in `build_argv`. Looking for:
```rust
match vm.def.flavor {
    Flavor::Linux => Ok(build_linux(vm, rt)),
    other => Err(QemuError::UnsupportedFlavor(other)),
}
```

Replace with:
```rust
match vm.def.flavor {
    Flavor::Linux | Flavor::Windows => Ok(build_pc_uefi(vm, rt)),
    other => Err(QemuError::UnsupportedFlavor(other)),
}
```

The `other` arm still catches `Flavor::Macos`, which is correct.

- [ ] **Step 6: Run tests, verify all pass**

Run: `cd /home/u/dev/phermes/phermes/phermesd && cargo test --lib qemu`
Expected: PASS — `windows_argv_is_byte_identical_…` passes (Windows dispatches to the same builder); `macos_flavor_is_unsupported` passes; all existing `linux_argv_*` golden tests pass (the rename is behavior-preserving).

- [ ] **Step 7: clippy + full Rust suite**

Run:
```bash
cd /home/u/dev/phermes/phermes/phermesd && cargo test && cargo clippy --all-targets --all-features -- -D warnings
```
Expected: all green; clippy clean. The rename + dispatch arm are syntactically clean; no new lints should fire.

- [ ] **Step 8: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add phermesd/src/qemu.rs
git commit -m "feat(phermesd): support Flavor::Windows via build_pc_uefi (renamed from build_linux)"
```

---

### Task 2: `vm.py` — Windows constants

**Files:**
- Modify: `src/phermes_build/vm.py`
- Modify: `tests/phermes_build/test_vm.py`

- [ ] **Step 1: Append the failing constants test to `tests/phermes_build/test_vm.py`**

```python
def test_windows_constants():
    assert vm_mod.WINDOWS_VMID == 101
    assert vm_mod.WINDOWS_DEFAULT_DISK_GB == 100
    assert vm_mod.WINDOWS_DEFAULT_MEMORY_MIB == 8192
    assert vm_mod.WINDOWS_DEFAULT_VCPUS == 4
```

- [ ] **Step 2: Run, verify fail**

Run: `uv run pytest tests/phermes_build/test_vm.py::test_windows_constants -v 2>&1 | head -10`
Expected: FAIL — `WINDOWS_VMID` / `WINDOWS_DEFAULT_DISK_GB` / `WINDOWS_DEFAULT_MEMORY_MIB` / `WINDOWS_DEFAULT_VCPUS` not defined.

- [ ] **Step 3: Add the constants to `src/phermes_build/vm.py`**

Find the existing constants block (where `LINUX_VMID`, `DEFAULT_DISK_GB`, etc. are defined). Add the Windows analogs in the same block (alphabetical / grouped by flavor):

```python
WINDOWS_VMID = 101
WINDOWS_DEFAULT_DISK_GB = 100
WINDOWS_DEFAULT_MEMORY_MIB = 8192
WINDOWS_DEFAULT_VCPUS = 4
```

(Place them after the Linux defaults; group by flavor is fine. If a `SEED_DIR` constant exists from slice #4a, you can OPTIONALLY add `WINDOWS_SEED_PATH = f"{SEED_DIR}/windows.iso"` as a placeholder for the future unattend.xml slice — but it's not used in #5a and YAGNI says skip it. Leave it out unless lints complain.)

- [ ] **Step 4: Run tests, verify pass**

Run: `uv run pytest tests/phermes_build/test_vm.py::test_windows_constants -v`
Expected: PASS.

- [ ] **Step 5: lint + typecheck + whole suite**

Run:
```bash
uv run pytest -q
uv run ruff check src/phermes_build/vm.py tests/phermes_build/test_vm.py
uv run ty check src/phermes_build/vm.py
```
Expected: clean; whole suite green.

- [ ] **Step 6: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add src/phermes_build/vm.py tests/phermes_build/test_vm.py
git commit -m "feat(builder): vm.py Windows constants (VMID=101, defaults 8GiB/4vcpu/100GB)"
```

---

### Task 3: `vm.py` — `write_windows_def`

**Files:**
- Modify: `src/phermes_build/vm.py`
- Modify: `tests/phermes_build/test_vm.py`

- [ ] **Step 1: Append failing tests to `tests/phermes_build/test_vm.py`**

```python
def test_write_windows_def_emits_expected_toml(tmp_path):
    import os as _os
    chroot = str(tmp_path / "chroot")
    _os.makedirs(chroot)
    vm_mod.write_windows_def(chroot)

    toml_path = tmp_path / "chroot" / "etc/phermes/vms/windows.toml"
    assert toml_path.exists()
    content = toml_path.read_text()
    assert 'flavor = "windows"' in content
    assert "memory_mib = 8192" in content
    assert "vcpus = 4" in content
    assert 'cpu = "host"' in content
    assert 'ovmf_code = "/usr/share/OVMF/OVMF_CODE.fd"' in content
    assert 'ovmf_vars_template = "/usr/share/OVMF/OVMF_VARS.fd"' in content
    assert 'path = "/dev/pve/vm-101-disk-0"' in content
    assert 'format = "raw"' in content
    assert 'interface = "virtio-scsi"' in content
    assert 'bridge = "vmbr0"' in content
    assert 'model = "virtio-net"' in content
    assert "serial = true" in content
    assert "vnc = true" in content
    # Slice #5a: no cloud-init seed for Windows — windows.toml has exactly ONE [[disk]] block.
    assert content.count("[[disk]]") == 1


def test_write_windows_def_honors_override_resources(tmp_path):
    import os as _os
    chroot = str(tmp_path / "chroot")
    _os.makedirs(chroot)
    vm_mod.write_windows_def(chroot, memory_mib=16384, vcpus=8)
    content = (tmp_path / "chroot" / "etc/phermes/vms/windows.toml").read_text()
    assert "memory_mib = 16384" in content
    assert "vcpus = 8" in content
```

- [ ] **Step 2: Run, verify fail**

Run: `uv run pytest tests/phermes_build/test_vm.py -k windows -v 2>&1 | head -20`
Expected: FAIL — `write_windows_def` doesn't exist yet.

- [ ] **Step 3: Add `_windows_def_text` + `write_windows_def` to `src/phermes_build/vm.py`**

Below the existing `_linux_def_text` / `write_linux_def` pair, add their Windows mirrors:

```python
def _windows_def_text(*, memory_mib: int, vcpus: int) -> str:
    """Render /etc/phermes/vms/windows.toml content.

    Slice #5a is BYOI: operator supplies a pre-installed Windows qcow2 (virtio
    drivers already loaded). The def therefore has no cloud-init seed CDROM
    block — that's unattend.xml territory and a later slice.

    Operators whose images lack virtio drivers can hand-edit `interface = "sata"`
    or `model = "e1000"` post-build.
    """
    return (
        f'flavor = "windows"\n'
        f"[resources]\n"
        f"memory_mib = {memory_mib}\n"
        f"vcpus = {vcpus}\n"
        f'cpu = "host"\n'
        f"[firmware]\n"
        f'ovmf_code = "/usr/share/OVMF/OVMF_CODE.fd"\n'
        f'ovmf_vars_template = "/usr/share/OVMF/OVMF_VARS.fd"\n'
        f"[[disk]]\n"
        f'path = "/dev/{STORAGE_VG}/vm-{WINDOWS_VMID}-disk-0"\n'
        f'format = "raw"\n'
        f'interface = "virtio-scsi"\n'
        f"[[net]]\n"
        f'bridge = "vmbr0"\n'
        f'model = "virtio-net"\n'
        f"[console]\n"
        f"serial = true\n"
        f"vnc = true\n"
    )


def write_windows_def(
    chroot_mount: str,
    *,
    memory_mib: int = WINDOWS_DEFAULT_MEMORY_MIB,
    vcpus: int = WINDOWS_DEFAULT_VCPUS,
) -> None:
    """Write /etc/phermes/vms/windows.toml inside the chroot."""
    vms_dir = os.path.join(chroot_mount, "etc/phermes/vms")
    os.makedirs(vms_dir, exist_ok=True)
    def_path = os.path.join(vms_dir, "windows.toml")
    with open(def_path, "w") as f:
        f.write(_windows_def_text(memory_mib=memory_mib, vcpus=vcpus))
```

- [ ] **Step 4: Run tests, verify pass**

Run: `uv run pytest tests/phermes_build/test_vm.py -k windows -v`
Expected: PASS — both new Windows tests + the constants test from Task 2.

- [ ] **Step 5: lint + typecheck + whole suite**

Run:
```bash
uv run pytest -q
uv run ruff check src/phermes_build/vm.py tests/phermes_build/test_vm.py
uv run ty check src/phermes_build/vm.py
```
Expected: clean; whole suite green.

- [ ] **Step 6: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add src/phermes_build/vm.py tests/phermes_build/test_vm.py
git commit -m "feat(builder): vm.write_windows_def (mirror of write_linux_def, no seed)"
```

---

### Task 4: `vm.py` — `provision_windows_disk`

**Files:**
- Modify: `src/phermes_build/vm.py`
- Modify: `tests/phermes_build/test_vm.py`

- [ ] **Step 1: Append failing tests to `tests/phermes_build/test_vm.py`**

```python
def test_provision_windows_disk_creates_thin_lv_and_tags_it(monkeypatch):
    calls = _capture(monkeypatch)
    vm_mod.provision_windows_disk()
    # 1st call: lvcreate --thin --virtualsize 100G pve/data -n vm-101-disk-0
    assert calls[0][0] == "lvcreate"
    assert "--thin" in calls[0]
    assert "--virtualsize" in calls[0]
    assert "100G" in calls[0]
    assert "pve/data" in calls[0]
    assert "vm-101-disk-0" in calls[0]
    # 2nd call: lvchange --addtag phermesd /dev/pve/vm-101-disk-0
    assert calls[1] == ["lvchange", "--addtag", "phermesd", "/dev/pve/vm-101-disk-0"]
    # No qemu-img invocation when source is None
    assert not any(c[0] == "qemu-img" for c in calls)


def test_provision_windows_disk_with_source_runs_qemu_img_convert(monkeypatch):
    calls = _capture(monkeypatch)
    vm_mod.provision_windows_disk(source="/tmp/win.qcow2")
    qemu_calls = [c for c in calls if c[0] == "qemu-img"]
    assert len(qemu_calls) == 1
    assert qemu_calls[0] == [
        "qemu-img",
        "convert",
        "-O",
        "raw",
        "/tmp/win.qcow2",
        "/dev/pve/vm-101-disk-0",
    ]


def test_provision_windows_disk_custom_size(monkeypatch):
    calls = _capture(monkeypatch)
    vm_mod.provision_windows_disk(size_gb=200)
    assert "200G" in calls[0]
```

(`_capture` is the existing helper in `test_vm.py` from slice #4a's Linux tests; reuse it as-is.)

- [ ] **Step 2: Run, verify fail**

Run: `uv run pytest tests/phermes_build/test_vm.py -k provision_windows -v 2>&1 | head -15`
Expected: FAIL — `provision_windows_disk` doesn't exist.

- [ ] **Step 3: Add `provision_windows_disk` to `src/phermes_build/vm.py`**

Below `provision_linux_disk`, add:

```python
def provision_windows_disk(
    size_gb: int = WINDOWS_DEFAULT_DISK_GB,
    source: str | None = None,
) -> None:
    """Create the thin LV for the Windows VM, tag it 'phermesd', optionally
    populate from a local image.

    Runs against the host's live VG (the one phermes-build just created), NOT
    against a chroot. Caller ensures the VG `pve` and thin pool `data` exist.
    """
    disk_name = f"vm-{WINDOWS_VMID}-disk-0"
    device = f"/dev/{STORAGE_VG}/{disk_name}"
    run_cmd(
        [
            "lvcreate",
            "--thin",
            "--virtualsize",
            f"{size_gb}G",
            f"{STORAGE_VG}/{STORAGE_POOL}",
            "-n",
            disk_name,
        ]
    )
    run_cmd(["lvchange", "--addtag", OWNER_TAG, device])
    if source is not None:
        run_cmd(["qemu-img", "convert", "-O", "raw", source, device])
```

- [ ] **Step 4: Run tests**

Run: `uv run pytest tests/phermes_build/test_vm.py -v`
Expected: PASS — all existing tests + the 3 new Windows provision tests + the 2 def tests + the constants test (6 new total).

- [ ] **Step 5: lint + typecheck + whole suite**

Run:
```bash
uv run pytest -q
uv run ruff check src/phermes_build/vm.py tests/phermes_build/test_vm.py
uv run ty check src/phermes_build/vm.py
```
Expected: clean; whole suite green.

- [ ] **Step 6: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add src/phermes_build/vm.py tests/phermes_build/test_vm.py
git commit -m "feat(builder): vm.provision_windows_disk (lvcreate+tag+qemu-img convert)"
```

---

### Task 5: `cli.py` — generalize `_linux_source` → `_vm_source`

**Files:**
- Modify: `src/phermes_build/cli.py`
- Modify: `tests/phermes_build/test_cli.py`

- [ ] **Step 1: Inspect the current `_linux_source` shape**

Run:
```bash
cd /home/u/dev/phermes/phermes && grep -nA8 "def _linux_source" src/phermes_build/cli.py
```

Note the exact signature. From slice #6 it should be `def _linux_source(import_vm_args: list[str]) -> str | None:` and parses `flavor=path` entries, raising `typer.BadParameter` for non-`linux` flavors.

- [ ] **Step 2: Append failing tests to `tests/phermes_build/test_cli.py`**

```python
def test_vm_source_accepts_linux_and_windows(monkeypatch):
    """The generalized helper returns the path for the requested flavor."""
    from phermes_build import cli as cli_mod
    args = ["linux=/tmp/lx.qcow2", "windows=/tmp/win.qcow2"]
    assert cli_mod._vm_source(args, "linux") == "/tmp/lx.qcow2"
    assert cli_mod._vm_source(args, "windows") == "/tmp/win.qcow2"


def test_vm_source_returns_none_when_flavor_absent(monkeypatch):
    from phermes_build import cli as cli_mod
    args = ["linux=/tmp/lx.qcow2"]
    assert cli_mod._vm_source(args, "windows") is None


def test_vm_source_rejects_unknown_flavor(monkeypatch):
    """An --import-vm freebsd=... entry raises typer.BadParameter."""
    import typer
    from phermes_build import cli as cli_mod
    args = ["freebsd=/tmp/fbsd.qcow2"]
    try:
        cli_mod._vm_source(args, "linux")
    except typer.BadParameter as e:
        assert "freebsd" in str(e)
    else:
        raise AssertionError("expected typer.BadParameter")
```

- [ ] **Step 3: Run, verify fail**

Run: `uv run pytest tests/phermes_build/test_cli.py -k vm_source -v 2>&1 | head -15`
Expected: FAIL — `_vm_source` doesn't exist (only `_linux_source` does).

- [ ] **Step 4: Replace `_linux_source` with `_vm_source` in `src/phermes_build/cli.py`**

Find the existing function. Delete it. Add:

```python
def _vm_source(import_vm_args: list[str], flavor: str) -> str | None:
    """Parse --import-vm <flavor>=<path> entries; return the path for the
    requested flavor, or None if absent.

    Supported flavors (#5a): linux, windows. macOS returns in #5b.

    Raises typer.BadParameter if an entry uses an unsupported flavor name.
    """
    supported = {"linux", "windows"}
    found: str | None = None
    for entry in import_vm_args:
        entry_flavor, _, path = entry.partition("=")
        if entry_flavor not in supported:
            raise typer.BadParameter(
                f"--import-vm flavor '{entry_flavor}' is not supported "
                f"(supported: {sorted(supported)})"
            )
        if not path:
            raise typer.BadParameter(
                f"--import-vm {entry_flavor}=<path> requires a non-empty path"
            )
        if entry_flavor == flavor:
            found = path
    return found
```

If `import typer` isn't already at the top of cli.py, add it. (It almost certainly is — `typer` is the CLI framework.)

- [ ] **Step 5: Update the single existing call site to use the new helper**

Find where `_linux_source(import_vm)` is called in `cli.py` (slice #4a/#6 calls it inside `build()`). Replace with `_vm_source(import_vm, "linux")`:

Before:
```python
source = _linux_source(import_vm)
```

After:
```python
source = _vm_source(import_vm, "linux")
```

(The eventual Windows call site is Task 6.)

- [ ] **Step 6: Run tests, verify pass**

Run: `uv run pytest tests/phermes_build/test_cli.py -v 2>&1 | tail -25`
Expected: PASS — the 3 new `_vm_source` tests + every existing test that previously called `_linux_source` (because the rename is mechanical and the new helper handles `linux=` identically to the old one).

- [ ] **Step 7: lint + typecheck + whole suite**

Run:
```bash
uv run pytest -q
uv run ruff check src/phermes_build/cli.py tests/phermes_build/test_cli.py
uv run ty check src/phermes_build/cli.py
```
Expected: clean; whole suite green.

- [ ] **Step 8: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add src/phermes_build/cli.py tests/phermes_build/test_cli.py
git commit -m "refactor(builder): _linux_source -> _vm_source (accepts linux + windows)"
```

---

### Task 6: `cli.py` — `_provision_windows_vm` + `build()` wiring

**Files:**
- Modify: `src/phermes_build/cli.py`
- Modify: `tests/phermes_build/test_cli.py`

- [ ] **Step 1: Append failing tests to `tests/phermes_build/test_cli.py`**

```python
def test_import_vm_windows_routes_into_provision_windows_disk(monkeypatch):
    """--import-vm windows=<path> reaches _provision_windows_vm(source=<path>)."""
    from phermes_build import cli as cli_mod
    seen: dict = {}
    monkeypatch.setattr(
        cli_mod, "_provision_windows_vm",
        lambda source: seen.update({"source": source}),
    )
    for helper in ("_setup_luks", "_setup_lvm", "_setup_btrfs", "_setup_exfat",
                   "_install_minimal_host", "_configure_host", "_setup_credentials",
                   "_write_firstboot", "_provision_linux_vm", "_partition",
                   "_write_cloud_init_seed"):
        monkeypatch.setattr(cli_mod, helper, lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "plan_disk_layout", lambda *a, **k: object())

    result = runner.invoke(cli_mod.app, [
        "/dev/loop0",
        "--dev-credentials",
        "--dev-ssh-pubkey", "ssh-ed25519 AAAA...op@host",
        "--import-vm", "windows=/tmp/win.qcow2",
    ])
    assert result.exit_code == 0, result.stdout
    assert seen.get("source") == "/tmp/win.qcow2"


def test_import_vm_both_flavors_provisions_both_vms(monkeypatch):
    """Both --import-vm linux= and --import-vm windows= in one build."""
    from phermes_build import cli as cli_mod
    seen: dict = {}

    def fake_linux(source, seed_iso_path):
        seen["linux"] = source

    def fake_windows(source):
        seen["windows"] = source

    monkeypatch.setattr(cli_mod, "_provision_linux_vm", fake_linux)
    monkeypatch.setattr(cli_mod, "_provision_windows_vm", fake_windows)
    for helper in ("_setup_luks", "_setup_lvm", "_setup_btrfs", "_setup_exfat",
                   "_install_minimal_host", "_configure_host", "_setup_credentials",
                   "_write_firstboot", "_partition", "_write_cloud_init_seed"):
        monkeypatch.setattr(cli_mod, helper, lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "plan_disk_layout", lambda *a, **k: object())

    result = runner.invoke(cli_mod.app, [
        "/dev/loop0",
        "--dev-credentials",
        "--dev-ssh-pubkey", "ssh-ed25519 AAAA...op@host",
        "--import-vm", "linux=/tmp/lx.qcow2",
        "--import-vm", "windows=/tmp/win.qcow2",
    ])
    assert result.exit_code == 0, result.stdout
    assert seen.get("linux") == "/tmp/lx.qcow2"
    assert seen.get("windows") == "/tmp/win.qcow2"


def test_no_vm_skips_both_provisioning_helpers(monkeypatch):
    """--no-vm skips Linux AND Windows provisioning even with both --import-vm flags."""
    from phermes_build import cli as cli_mod
    seen: dict = {}
    monkeypatch.setattr(
        cli_mod, "_provision_linux_vm",
        lambda *a, **k: seen.setdefault("linux_called", True),
    )
    monkeypatch.setattr(
        cli_mod, "_provision_windows_vm",
        lambda *a, **k: seen.setdefault("windows_called", True),
    )
    for helper in ("_setup_luks", "_setup_lvm", "_setup_btrfs", "_setup_exfat",
                   "_install_minimal_host", "_configure_host", "_setup_credentials",
                   "_write_firstboot", "_partition", "_write_cloud_init_seed"):
        monkeypatch.setattr(cli_mod, helper, lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "plan_disk_layout", lambda *a, **k: object())

    result = runner.invoke(cli_mod.app, [
        "/dev/loop0",
        "--dev-credentials",
        "--dev-ssh-pubkey", "ssh-ed25519 AAAA...op@host",
        "--no-vm",
        "--import-vm", "linux=/tmp/lx.qcow2",
        "--import-vm", "windows=/tmp/win.qcow2",
    ])
    assert result.exit_code == 0, result.stdout
    assert "linux_called" not in seen
    assert "windows_called" not in seen
```

- [ ] **Step 2: Run, verify fail**

Run: `uv run pytest tests/phermes_build/test_cli.py -k 'import_vm_windows or import_vm_both or no_vm_skips' -v 2>&1 | tail -20`
Expected: FAIL — `_provision_windows_vm` doesn't exist.

- [ ] **Step 3: Add `_provision_windows_vm` + wire the new step into `build()`**

(a) Add the new helper to `cli.py` (place next to `_provision_linux_vm`):

```python
def _provision_windows_vm(source: str | None) -> None:
    """Write /etc/phermes/vms/windows.toml in the chroot, then create + populate
    the LVM-thin volume for the Windows VM (slice #5a BYOI)."""
    vm_mod.write_windows_def(PVE_ROOT_MOUNT)
    vm_mod.provision_windows_disk(source=source)
```

(b) In `build()`, find the existing Linux provisioning block (the lambda-appended `os_steps` entry guarded by `if not no_vm:` from slice #4a's wiring). After the Linux block, add a parallel Windows block. The final shape should be:

```python
        if not no_vm:
            seed = _write_cloud_init_seed(dev_ssh_pubkey if dev_credentials else None)
            linux_source = _vm_source(import_vm, "linux")
            if linux_source is not None:
                os_steps.append(
                    (
                        "Provisioning Linux VM",
                        lambda: _provision_linux_vm(
                            source=linux_source,
                            seed_iso_path=seed,
                        ),
                    )
                )
            windows_source = _vm_source(import_vm, "windows")
            if windows_source is not None:
                os_steps.append(
                    (
                        "Provisioning Windows VM",
                        lambda: _provision_windows_vm(source=windows_source),
                    )
                )
```

Key change: the Linux step is now CONDITIONAL on `linux_source is not None` (today's code may always append it; if so, this conditional is correct — a user with `--no-vm` not set but no `--import-vm` would still get an empty VM def written, which may not be desired). Re-read the current code before deciding; if the Linux step has historically been unconditional, KEEP that behavior — make Windows the conditional one only. The spec's "Provisioning Windows VM iff `_vm_source(…, "windows")` is Some" is the authoritative behavior for Windows.

Pragmatic guidance: if Linux's existing behavior is "always append", leave it. The plan's spec language about "Linux iff Some" is for symmetry; behavior continuity is more important than symmetry.

(c) Make sure `linux_source` is captured outside the lambda (slice #4a established this — variables captured by lambda must be bound at definition time). Same applies to `windows_source`.

- [ ] **Step 4: Run cli tests, verify pass**

Run: `uv run pytest tests/phermes_build/test_cli.py -v 2>&1 | tail -30`
Expected: PASS — existing tests + the 3 new Windows tests.

- [ ] **Step 5: lint + typecheck + whole suite**

Run:
```bash
uv run pytest -q
uv run ruff check src/phermes_build/cli.py tests/phermes_build/test_cli.py
uv run ty check src/phermes_build/cli.py
```
Expected: clean; whole suite green.

- [ ] **Step 6: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add src/phermes_build/cli.py tests/phermes_build/test_cli.py
git commit -m "feat(builder): cli --import-vm windows= provisions Windows VM (mirrors Linux)"
```

---

### Task 7: README + CHANGELOG

**Files:**
- Modify: `README.md`
- Modify: `CHANGELOG.md`

- [ ] **Step 1: README**

Open `README.md`. Find the `### phermesd (in development)` subsection (it should already contain slice #4a/#4b language from prior PRs if they've merged; if not, append at the end of whichever paragraph exists). Append:

```
Slice #5a (implemented): Windows guests are now supported (BYOI). Pass `--import-vm windows=<path>` (and optionally also `--import-vm linux=<path>` in the same build) to a pre-installed Windows qcow2; `phermesctl activate windows` boots it. Defaults: 8 GiB RAM, 4 vCPUs, 100 GB disk. Operators whose images lack virtio drivers can hand-edit `interface = "sata"` or `model = "e1000"` in `/etc/phermes/vms/windows.toml` post-build. Design: [`docs/superpowers/specs/2026-06-12-phermesd-windows-guest-design.md`](docs/superpowers/specs/2026-06-12-phermesd-windows-guest-design.md).
```

- [ ] **Step 2: CHANGELOG**

Open `CHANGELOG.md`. Under `## [Unreleased]` → `### Added`, prepend:

```markdown
- `phermesd` Windows guest support (slice #5a): `Flavor::Windows` now dispatches to
  the same QEMU argv builder as Linux (renamed `build_linux` -> `build_pc_uefi` for
  honest naming; `Flavor::Macos` continues to return `UnsupportedFlavor` until #5b).
  `phermes-build --import-vm windows=<path>` populates an LVM-thin disk
  (`vm-101-disk-0`, 100 GiB default) from a pre-installed Windows qcow2 (BYOI) and
  writes `/etc/phermes/vms/windows.toml`. Both `--import-vm linux=` and
  `--import-vm windows=` can be passed in one build; `--no-vm` skips both. No
  cloud-init / unattend.xml in #5a (deferred).
```

- [ ] **Step 3: Verify everything still builds + tests pass + lints clean**

Run:
```bash
cd /home/u/dev/phermes/phermes/phermesd && cargo test && cargo clippy --all-targets --all-features -- -D warnings
cd /home/u/dev/phermes/phermes && uv run pytest -q && uv run ruff check src/ tests/ && uv run ty check src/
```
Expected: all green; clippy + ruff + ty clean.

- [ ] **Step 4: Commit**

```bash
cd /home/u/dev/phermes/phermes
git add README.md CHANGELOG.md
git commit -m "docs: phermesd Windows guest support (slice #5a)"
```

---

## Out of Scope (carries to later slices)

- macOS guest support → #5b (the `build_pc_uefi` rename pays off there).
- Automated Windows installer with attached virtio-win.iso → later slice.
- `unattend.xml` generation → later slice (Windows analog of #4a's cloud-init).
- TPM 2.0 / Secure Boot OVMF vars variant for Windows 11 → later slice (needs `swtpm`).
- QGA-quiesced snapshots for Windows — the virtio-serial QGA channel is already wired by
  slice #1, but quiescing requires `qemu-guest-agent` installed in the Windows guest
  (operator's responsibility for #5a).
- Per-flavor `--no-vm` (e.g., `--no-linux-vm`, `--no-windows-vm`) — YAGNI until someone
  asks; today's `--no-vm` is a single boolean that skips both.

---

## Self-Review

**1. Spec coverage** — every spec section maps to a task:

| Spec section / requirement | Task(s) |
|---|---|
| Rust rename `build_linux` → `build_pc_uefi`; dispatch `Linux \| Windows` | 1 |
| Macos arm still returns `UnsupportedFlavor`; test renamed | 1 |
| New `windows_argv_is_byte_identical_to_linux_argv_…` test | 1 |
| `WINDOWS_VMID = 101`, `WINDOWS_DEFAULT_DISK_GB = 100`, `WINDOWS_DEFAULT_MEMORY_MIB = 8192`, `WINDOWS_DEFAULT_VCPUS = 4` | 2 |
| `_windows_def_text` private renderer + `write_windows_def` | 3 |
| `provision_windows_disk` (lvcreate+lvchange+qemu-img) | 4 |
| `_linux_source` → `_vm_source(args, flavor)` generic helper | 5 |
| `typer.BadParameter` for unknown flavor | 5 |
| `_provision_windows_vm` + `build()` step appended when `--import-vm windows=` set | 6 |
| Both `linux=` and `windows=` in one build | 6 |
| `--no-vm` skips both | 6 |
| README + CHANGELOG | 7 |
| Out-of-scope items (#5b, virtio-win install, unattend.xml, TPM, QGA quiesce, per-flavor --no-vm) | Out of Scope section |
| Success criterion (operator-verified end-to-end smoke) | operator runs post-merge |

The "Open questions" from the spec are answered in the plan: callers of `build_linux` are
checked by grep in Task 1 Step 5; the virtio-net default is called out in Task 7's README
text; per-flavor `--no-vm` is explicitly Out of Scope.

**2. Placeholder scan** — none. Task 6 Step 3 documents the Linux-conditional-or-unconditional
ambiguity with pragmatic guidance ("re-read the current code; preserve behavior continuity").
The `WINDOWS_SEED_PATH` constant in Task 2 is explicitly omitted as YAGNI for #5a; not a
placeholder.

**3. Type consistency** — `WINDOWS_VMID = 101`, `WINDOWS_DEFAULT_*` constants,
`_windows_def_text(*, memory_mib, vcpus)`, `write_windows_def(chroot_mount, *, memory_mib, vcpus)`,
`provision_windows_disk(size_gb, source)`, `_vm_source(import_vm_args, flavor) -> str | None`,
`_provision_windows_vm(source)`, `build_pc_uefi`, `Flavor::Linux | Flavor::Windows` —
names and signatures match across tasks and tests.
