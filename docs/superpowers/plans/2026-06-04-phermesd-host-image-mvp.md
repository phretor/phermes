# phermesd Host Image MVP Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace `phermes-build`'s Proxmox install with a minimal Debian host running `phermesd`, so `just smoke-create && just smoke-full && just smoke-qemu` produces a bootable phermesd-based SSD that answers `phermesctl list` over SSH.

**Architecture:** A new `host.py` replaces `proxmox.py` as the OS installer; `vm.py` is rewritten to write `/etc/phermes/vms/linux.toml` and provision an LVM-thin volume using slice-#2 conventions; a new `systemd_units.py` ships the `phermesd.service` unit; the `Dockerfile` gains a Rust builder stage that produces `phermesd`/`phermesctl` binaries copied into `/app/bin/` and from there into the chroot.

**Tech Stack:** Python 3.13 (phermes-build), Debian bookworm (target host), systemd (target init), nftables (target firewall+NAT), Docker multi-stage (binary delivery), Rust 1.95 (phermesd build stage).

**Spec:** `docs/superpowers/specs/2026-06-04-phermesd-host-image-mvp-design.md`

**Prerequisite:** base the implementation branch on `main` *after* slice #2 (PR #19) and `chore/phermesd-recipes-in-container` are merged — slice #2's `phermesd` binary set and the container-based `phermesd-*` recipes are dependencies. If those PRs aren't merged when work starts, stack the branch on `feat/phermesd-storage` (which carries both sets of commits) instead of bare `main`.

---

## File Structure

```
src/phermes_build/
  host.py            (new)     # Replaces proxmox.py — install_minimal_host + helpers
  systemd_units.py   (new)     # phermesd.service text + chroot installer
  vm.py              (rewrite) # Linux-only: write_linux_def + provision_linux_disk
  cli.py             (modify)  # Orchestration swap + --no-vm flag
  host_config.py     (modify)  # Drop RBAC; nftables_ruleset() gains NAT chain
  proxmox.py         (delete)
  node_vm.py         (delete)
Dockerfile           (modify)  # Multi-stage Rust builder + drop dead apt packages
tests/phermes_build/
  test_host.py           (new)
  test_systemd_units.py  (new)
  test_vm.py             (rewrite)
  test_cli.py            (modify)
  test_host_config.py    (modify)
  test_proxmox.py        (delete)
  test_node_vm.py        (delete)
Justfile             (modify)  # smoke-full now drives the phermesd-based build
README.md            (modify)
CHANGELOG.md         (modify)
```

Test convention (existing): `tests/phermes_build/test_<module>.py`; standard mock is
`monkeypatch.setattr(<mod>, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")`.

---

### Task 1: Drop dead apt packages from Dockerfile

**Files:**
- Modify: `Dockerfile`

Earlier inspection showed `fdisk`, `gdisk`, `parted`, and `wget` are never invoked by `phermes-build` (the partitioner uses `sfdisk` which lives in `util-linux`; `wget` was Proxmox-keyring only). Drop them.

- [ ] **Step 1: Edit `Dockerfile`'s apt-get line**

Open `Dockerfile`. The current install line is:
```dockerfile
RUN apt-get update && apt-get install -y --no-install-recommends \
    btrfs-progs \
    cryptsetup \
    debootstrap \
    dosfstools \
    exfatprogs \
    fdisk \
    gdisk \
    parted \
    lvm2 \
    udev \
    util-linux \
    wget \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
```

Replace it with:
```dockerfile
RUN apt-get update && apt-get install -y --no-install-recommends \
    btrfs-progs \
    cryptsetup \
    debootstrap \
    dosfstools \
    exfatprogs \
    lvm2 \
    udev \
    util-linux \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
```

- [ ] **Step 2: Rebuild image and verify the existing smoke recipe still works**

Run:
```bash
just docker-build 2>&1 | tail -3
```
Expected: image builds; no errors about missing packages downstream. (Full smoke would need root + a loop device; this step just confirms the image builds.)

- [ ] **Step 3: Commit**

```bash
git add Dockerfile
git commit -m "chore: drop unused fdisk/gdisk/parted/wget from phermes-build image"
```

---

### Task 2: Multi-stage Dockerfile — Rust builder + binaries copied into /app/bin/

**Files:**
- Modify: `Dockerfile`

- [ ] **Step 1: Replace the Dockerfile with the multi-stage form**

Open `Dockerfile`. Replace the entire file with:

```dockerfile
# ── Stage 1: build phermesd + phermesctl binaries from the working tree ──────
FROM rust:1.95-slim-bookworm AS phermesd-builder
WORKDIR /work
COPY phermesd/ ./
RUN cargo build --release --bin phermesd --bin phermesctl

# ── Stage 2: phermes-build runtime image (toolchain + Python + phermesd bins) ─
FROM debian:bookworm-slim

# Linux toolchain required by phermes-build (host-side disk operations)
RUN apt-get update && apt-get install -y --no-install-recommends \
    btrfs-progs \
    cryptsetup \
    debootstrap \
    dosfstools \
    exfatprogs \
    lvm2 \
    udev \
    util-linux \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=ghcr.io/astral-sh/uv:0.11.17 /uv /usr/local/bin/uv

# phermesd binaries from the builder stage; phermes-build copies them into the chroot
COPY --from=phermesd-builder /work/target/release/phermesd /app/bin/phermesd
COPY --from=phermesd-builder /work/target/release/phermesctl /app/bin/phermesctl

WORKDIR /app
COPY .python-version pyproject.toml uv.lock ./
# Install deps only — cached layer unaffected by src/ changes
RUN uv sync --frozen --no-dev --no-install-project

COPY src/ ./src/
# Install the local package now that source files are present
RUN uv sync --frozen --no-dev

ENTRYPOINT ["uv", "run", "--no-sync", "phermes-build"]
```

- [ ] **Step 2: Build and confirm binaries land at /app/bin/**

Run:
```bash
just docker-build 2>&1 | tail -3
docker run --rm --entrypoint /bin/ls phermes-build /app/bin
```
Expected: image builds; `ls` prints `phermesctl` and `phermesd` (two files).

- [ ] **Step 3: Confirm they execute under the image**

Run:
```bash
docker run --rm --entrypoint /app/bin/phermesctl phermes-build --help 2>&1 | head -5
```
Expected: clap-generated usage text (subcommands list/status/activate/stop/reload/provision/snapshot/rollback/snapshots/delete).

- [ ] **Step 4: Commit**

```bash
git add Dockerfile
git commit -m "feat(builder): multi-stage Dockerfile shipping phermesd binaries in /app/bin"
```

---

### Task 3: `systemd_units.py` — phermesd.service text + chroot installer

**Files:**
- Create: `src/phermes_build/systemd_units.py`
- Create: `tests/phermes_build/test_systemd_units.py`

- [ ] **Step 1: Write failing tests**

Create `tests/phermes_build/test_systemd_units.py`:

```python
import os

from phermes_build import systemd_units as sd_mod


def test_phermesd_service_unit_text_has_required_fields():
    unit = sd_mod.phermesd_service_unit()
    # Identity + docs
    assert "Description=PHermes VM Orchestrator" in unit
    # Dependencies
    assert "After=network-online.target" in unit
    assert "Wants=network-online.target" in unit
    assert "ConditionPathExists=/dev/kvm" in unit
    # ExecStart points at the binary we ship and uses production paths
    assert "/usr/local/sbin/phermesd" in unit
    assert "--vms-dir /etc/phermes/vms" in unit
    assert "--socket /run/phermesd/control.sock" in unit
    # Restart policy + Install
    assert "Restart=on-failure" in unit
    assert "WantedBy=multi-user.target" in unit


def test_install_phermesd_unit_writes_file_and_creates_wants_symlink(tmp_path):
    mount_point = str(tmp_path / "chroot")
    os.makedirs(mount_point)

    sd_mod.install_phermesd_unit(mount_point)

    unit_path = tmp_path / "chroot" / "etc/systemd/system/phermesd.service"
    assert unit_path.exists()
    assert "ExecStart=" in unit_path.read_text()

    wants_link = (
        tmp_path / "chroot" / "etc/systemd/system/multi-user.target.wants" / "phermesd.service"
    )
    assert wants_link.is_symlink()
    assert os.readlink(str(wants_link)) == "/etc/systemd/system/phermesd.service"


def test_install_phermesd_unit_is_idempotent(tmp_path):
    mount_point = str(tmp_path / "chroot")
    os.makedirs(mount_point)
    sd_mod.install_phermesd_unit(mount_point)
    # second call must not raise (symlink already exists)
    sd_mod.install_phermesd_unit(mount_point)
```

- [ ] **Step 2: Run, verify fail to import**

Run: `uv run pytest tests/phermes_build/test_systemd_units.py -v 2>&1 | head -10`
Expected: collection error or `ImportError` — `phermes_build.systemd_units` does not exist.

- [ ] **Step 3: Implement**

Create `src/phermes_build/systemd_units.py`:

```python
"""Systemd unit text + chroot installer for phermesd.

Kept separate from host.py so the unit text is testable in isolation and so a
later hardening pass (immutable host / OpenRC) can swap this module without
touching the OS installer.
"""

import os


def phermesd_service_unit() -> str:
    """Return the text of /etc/systemd/system/phermesd.service.

    `After=lvm2-activation.service` is intentionally omitted: on a fresh Debian
    bookworm root the canonical unit name is `lvm2-monitor.service`; the
    dependency on `local-fs.target` already established by systemd's default
    ordering is sufficient for our use (phermesd opens `/etc/phermes/vms/` and
    `/run/phermesd/`, not LVM devices directly — the LVM activation happens at
    initramfs unlock time, long before this unit starts).
    """
    return (
        "[Unit]\n"
        "Description=PHermes VM Orchestrator\n"
        "Documentation=https://github.com/phretor/phermes\n"
        "After=network-online.target\n"
        "Wants=network-online.target\n"
        "ConditionPathExists=/dev/kvm\n"
        "\n"
        "[Service]\n"
        "Type=simple\n"
        "ExecStart=/usr/local/sbin/phermesd --vms-dir /etc/phermes/vms --run-dir /run/phermesd --socket /run/phermesd/control.sock\n"
        "Restart=on-failure\n"
        "RestartSec=5s\n"
        "\n"
        "[Install]\n"
        "WantedBy=multi-user.target\n"
    )


def install_phermesd_unit(mount_point: str) -> None:
    """Write the unit into the chroot and create the multi-user.target.wants symlink.

    Mirrors the trick used by the old install_pve_firstboot_init: there is no
    running systemd in the chroot, so `systemctl enable` is unavailable.
    """
    unit_path = os.path.join(mount_point, "etc/systemd/system/phermesd.service")
    os.makedirs(os.path.dirname(unit_path), exist_ok=True)
    with open(unit_path, "w") as f:
        f.write(phermesd_service_unit())

    wants = os.path.join(mount_point, "etc/systemd/system/multi-user.target.wants")
    os.makedirs(wants, exist_ok=True)
    link = os.path.join(wants, "phermesd.service")
    if not os.path.islink(link):
        os.symlink("/etc/systemd/system/phermesd.service", link)
```

- [ ] **Step 4: Run tests, verify pass**

Run: `uv run pytest tests/phermes_build/test_systemd_units.py -v`
Expected: PASS (3 tests).

- [ ] **Step 5: Lint + typecheck**

Run: `uv run ruff check src/phermes_build/systemd_units.py tests/phermes_build/test_systemd_units.py && uv run ty check src/phermes_build/systemd_units.py`
Expected: clean.

- [ ] **Step 6: Commit**

```bash
git add src/phermes_build/systemd_units.py tests/phermes_build/test_systemd_units.py
git commit -m "feat(builder): systemd_units module — phermesd.service text + installer"
```

---

### Task 4: Rewrite `vm.py` for phermesd (Linux-only TOML def + LVM-thin provisioning)

**Files:**
- Modify (full rewrite): `src/phermes_build/vm.py`
- Modify (full rewrite): `tests/phermes_build/test_vm.py`

- [ ] **Step 1: Replace tests/phermes_build/test_vm.py with the new content**

```python
"""Tests for the rewritten vm.py — Linux-only, phermesd TOML def + LVM-thin LV."""

import os

from phermes_build import vm as vm_mod


def _capture(monkeypatch):
    calls = []
    monkeypatch.setattr(vm_mod, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")
    return calls


def test_constants_mirror_phermesd_storage_conventions():
    # These MUST equal the values hardcoded in phermesd/src/storage.rs.
    # If you change one side, change the other (a CI grep is a future hardening pass).
    assert vm_mod.STORAGE_VG == "pve"
    assert vm_mod.STORAGE_POOL == "data"
    assert vm_mod.OWNER_TAG == "phermesd"
    assert vm_mod.LINUX_VMID == 102


def test_write_linux_def_emits_expected_toml(tmp_path):
    chroot = str(tmp_path / "chroot")
    os.makedirs(chroot)
    vm_mod.write_linux_def(chroot)

    toml_path = tmp_path / "chroot" / "etc/phermes/vms/linux.toml"
    assert toml_path.exists()
    content = toml_path.read_text()
    assert 'flavor = "linux"' in content
    assert "memory_mib = 4096" in content
    assert "vcpus = 4" in content
    assert 'cpu = "host"' in content
    assert 'ovmf_code = "/usr/share/OVMF/OVMF_CODE.fd"' in content
    assert 'ovmf_vars_template = "/usr/share/OVMF/OVMF_VARS.fd"' in content
    assert 'path = "/dev/pve/vm-102-disk-0"' in content
    assert 'format = "raw"' in content
    assert 'interface = "virtio-scsi"' in content
    assert 'bridge = "vmbr0"' in content
    assert 'model = "virtio-net"' in content
    assert "serial = true" in content
    assert "vnc = true" in content


def test_write_linux_def_honors_override_resources(tmp_path):
    chroot = str(tmp_path / "chroot")
    os.makedirs(chroot)
    vm_mod.write_linux_def(chroot, memory_mib=8192, vcpus=8)
    content = (tmp_path / "chroot" / "etc/phermes/vms/linux.toml").read_text()
    assert "memory_mib = 8192" in content
    assert "vcpus = 8" in content


def test_provision_linux_disk_creates_thin_lv_and_tags_it(monkeypatch):
    calls = _capture(monkeypatch)
    vm_mod.provision_linux_disk()
    # 1st call: lvcreate --thin --virtualsize <default>G pve/data -n vm-102-disk-0
    assert calls[0][0] == "lvcreate"
    assert "--thin" in calls[0]
    assert "--virtualsize" in calls[0]
    assert "40G" in calls[0]
    assert "pve/data" in calls[0]
    assert "vm-102-disk-0" in calls[0]
    # 2nd call: lvchange --addtag phermesd /dev/pve/vm-102-disk-0
    assert calls[1] == ["lvchange", "--addtag", "phermesd", "/dev/pve/vm-102-disk-0"]
    # No qemu-img invocation when source is None
    assert not any(c[0] == "qemu-img" for c in calls)


def test_provision_linux_disk_with_source_runs_qemu_img_convert(monkeypatch):
    calls = _capture(monkeypatch)
    vm_mod.provision_linux_disk(source="/tmp/source.qcow2")
    qemu_calls = [c for c in calls if c[0] == "qemu-img"]
    assert len(qemu_calls) == 1
    assert qemu_calls[0] == [
        "qemu-img",
        "convert",
        "-O",
        "raw",
        "/tmp/source.qcow2",
        "/dev/pve/vm-102-disk-0",
    ]


def test_provision_linux_disk_custom_size(monkeypatch):
    calls = _capture(monkeypatch)
    vm_mod.provision_linux_disk(size_gb=100)
    assert "100G" in calls[0]
```

- [ ] **Step 2: Run, verify fail (existing vm.py exports differ)**

Run: `uv run pytest tests/phermes_build/test_vm.py -v 2>&1 | head -30`
Expected: FAIL — module attributes (`STORAGE_VG`, `LINUX_VMID`, `write_linux_def`, `provision_linux_disk`) do not exist; the old `_VM_IDS`/`provision_vm`/etc. are still there.

- [ ] **Step 3: Replace src/phermes_build/vm.py wholesale**

```python
"""Linux VM provisioning at install time (phermesd-based).

Linux-only for the MVP — Windows + macOS return in #5. Conventions (VG/pool/tag
names, vmid) MUST stay aligned with phermesd's storage module
(`phermesd/src/storage.rs`); a CI grep is a future hardening pass.
"""

import os

from phermes_build.runner import run_cmd

# Mirror phermesd's storage conventions. See phermesd/src/storage.rs and
# docs/superpowers/specs/2026-06-03-phermesd-storage-design.md.
STORAGE_VG = "pve"
STORAGE_POOL = "data"
OWNER_TAG = "phermesd"
LINUX_VMID = 102

DEFAULT_DISK_GB = 40
DEFAULT_MEMORY_MIB = 4096
DEFAULT_VCPUS = 4


def _linux_def_text(*, memory_mib: int, vcpus: int) -> str:
    """Render /etc/phermes/vms/linux.toml content.

    Production conventions:
      * raw block device at /dev/<vg>/vm-<vmid>-disk-0
      * vmbr0 bridge (set up by host.write_network_interfaces)
      * virtio-scsi + virtio-net (perf + driver availability in Linux guests)
      * serial + vnc unix sockets exposed (slice #4's console proxy reads them)
    """
    return (
        f'flavor = "linux"\n'
        f"[resources]\n"
        f"memory_mib = {memory_mib}\n"
        f"vcpus = {vcpus}\n"
        f'cpu = "host"\n'
        f"[firmware]\n"
        f'ovmf_code = "/usr/share/OVMF/OVMF_CODE.fd"\n'
        f'ovmf_vars_template = "/usr/share/OVMF/OVMF_VARS.fd"\n'
        f"[[disk]]\n"
        f'path = "/dev/{STORAGE_VG}/vm-{LINUX_VMID}-disk-0"\n'
        f'format = "raw"\n'
        f'interface = "virtio-scsi"\n'
        f"[[net]]\n"
        f'bridge = "vmbr0"\n'
        f'model = "virtio-net"\n'
        f"[console]\n"
        f"serial = true\n"
        f"vnc = true\n"
    )


def write_linux_def(
    chroot_mount: str,
    *,
    memory_mib: int = DEFAULT_MEMORY_MIB,
    vcpus: int = DEFAULT_VCPUS,
) -> None:
    """Write /etc/phermes/vms/linux.toml inside the chroot."""
    vms_dir = os.path.join(chroot_mount, "etc/phermes/vms")
    os.makedirs(vms_dir, exist_ok=True)
    def_path = os.path.join(vms_dir, "linux.toml")
    with open(def_path, "w") as f:
        f.write(_linux_def_text(memory_mib=memory_mib, vcpus=vcpus))


def provision_linux_disk(
    size_gb: int = DEFAULT_DISK_GB,
    source: str | None = None,
) -> None:
    """Create the thin LV, tag it 'phermesd', optionally populate from a local image.

    Runs against the host's live VG (the one phermes-build just created), NOT
    against a chroot. Caller ensures the VG `pve` and thin pool `data` exist.
    """
    disk_name = f"vm-{LINUX_VMID}-disk-0"
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

- [ ] **Step 4: Run tests, verify pass**

Run: `uv run pytest tests/phermes_build/test_vm.py -v`
Expected: PASS (6 tests).

- [ ] **Step 5: Lint + typecheck**

Run: `uv run ruff check src/phermes_build/vm.py tests/phermes_build/test_vm.py && uv run ty check src/phermes_build/vm.py`
Expected: clean.

- [ ] **Step 6: Commit**

```bash
git add src/phermes_build/vm.py tests/phermes_build/test_vm.py
git commit -m "feat(builder): rewrite vm.py for phermesd Linux-only TOML def + LVM-thin"
```

---

### Task 5: `host_config.py` — drop RBAC, add NAT chain to nftables_ruleset

**Files:**
- Modify: `src/phermes_build/host_config.py`
- Modify: `tests/phermes_build/test_host_config.py`

- [ ] **Step 1: Read current nftables_ruleset and any RBAC test references**

Open `src/phermes_build/host_config.py` and `tests/phermes_build/test_host_config.py`. Note: the current `nftables_ruleset()` returns a filter-only ruleset; you'll append a `nat/postrouting` chain. The constants `PHERMES_ROLE` + `PHERMES_USER_REALM` and the function `configure_proxmox_rbac()` go away.

- [ ] **Step 2: Update tests/phermes_build/test_host_config.py**

Remove every test that touches `configure_proxmox_rbac`, `PHERMES_ROLE`, or `PHERMES_USER_REALM`. Add:

```python
def test_nftables_ruleset_includes_nat_chain_for_vmbr0():
    rules = host_config.nftables_ruleset()
    # NAT table + postrouting chain present
    assert "table ip nat" in rules
    assert "chain postrouting" in rules
    assert "type nat hook postrouting" in rules
    # MASQUERADE the vmbr0 network out eth0
    assert "10.10.10.0/24" in rules
    assert 'oifname "eth0"' in rules
    assert "masquerade" in rules
```

Confirm `host_config` is imported at the top of `test_host_config.py` as `from phermes_build import host_config` (the file already does this; if not, adjust).

- [ ] **Step 3: Run, verify the new NAT test fails (ruleset has no NAT yet)**

Run: `uv run pytest tests/phermes_build/test_host_config.py::test_nftables_ruleset_includes_nat_chain_for_vmbr0 -v`
Expected: FAIL — `"table ip nat" in rules` is False.

- [ ] **Step 4: Edit src/phermes_build/host_config.py**

Delete the lines:
```python
PHERMES_ROLE = "PHermesUser"
PHERMES_USER_REALM = "pve"
```

Delete the entire `configure_proxmox_rbac` function (everything from its `def` line through the end of its body — verify by grepping `grep -n configure_proxmox_rbac src/phermes_build/host_config.py` after the edit; expected: no results).

Find the existing `nftables_ruleset()` function. It returns a multi-line string of nft rules. Append a NAT block to that returned string. Concretely, change the return statement so the final ruleset ends with:

```nftables
table ip nat {
    chain postrouting {
        type nat hook postrouting priority 100;
        ip saddr 10.10.10.0/24 oifname "eth0" masquerade
    }
}
```

In the function body, this is a trailing append to the existing string. Whatever the current return statement is, add the block above as the last table. Example (read the existing function and adapt; the literal content of the prepend may differ):

```python
def nftables_ruleset() -> str:
    return (
        # … existing filter rules — leave as-is …
        "\n"
        "table ip nat {\n"
        "    chain postrouting {\n"
        "        type nat hook postrouting priority 100;\n"
        '        ip saddr 10.10.10.0/24 oifname "eth0" masquerade\n'
        "    }\n"
        "}\n"
    )
```

- [ ] **Step 5: Run tests**

Run: `uv run pytest tests/phermes_build/test_host_config.py -v`
Expected: PASS (all surviving tests + the new NAT test). Any remaining test that imports `PHERMES_ROLE` / `configure_proxmox_rbac` must have been removed in Step 2.

- [ ] **Step 6: Lint + typecheck**

Run: `uv run ruff check src/phermes_build/host_config.py tests/phermes_build/test_host_config.py && uv run ty check src/phermes_build/host_config.py`
Expected: clean.

- [ ] **Step 7: Commit**

```bash
git add src/phermes_build/host_config.py tests/phermes_build/test_host_config.py
git commit -m "refactor(builder): drop Proxmox RBAC; nftables_ruleset includes vmbr0 NAT"
```

---

### Task 6: `host.py` — debian_apt_sources + write_network_interfaces (eth0 DHCP) + install_phermesd_binaries

**Files:**
- Create: `src/phermes_build/host.py`
- Create: `tests/phermes_build/test_host.py`

This task creates `host.py` with the *new* pieces and **imports the unchanged-from-Proxmox helpers from `proxmox.py`** (the actual move happens in Task 9 when `proxmox.py` is deleted). This keeps the diff per task small and TDD-friendly.

- [ ] **Step 1: Write failing tests**

Create `tests/phermes_build/test_host.py`:

```python
import os
import stat

from phermes_build import host as host_mod


def test_debian_apt_sources_has_bookworm_main_updates_security_no_proxmox():
    sources = host_mod.debian_apt_sources()
    assert "deb http://deb.debian.org/debian bookworm main" in sources
    assert "deb http://deb.debian.org/debian bookworm-updates main" in sources
    assert "deb http://security.debian.org/debian-security bookworm-security main" in sources
    # Proxmox repo MUST NOT appear
    assert "proxmox" not in sources.lower()
    assert "pve-no-subscription" not in sources


def test_write_network_interfaces_eth0_is_dhcp_no_masquerade(tmp_path):
    chroot = str(tmp_path / "chroot")
    os.makedirs(chroot)
    host_mod.write_network_interfaces(chroot)
    content = (tmp_path / "chroot" / "etc/network/interfaces").read_text()
    # eth0 is DHCP (no static-IP-for-pmxcfs hack)
    assert "iface eth0 inet dhcp" in content
    # vmbr0 keeps its static address (NAT moved to nftables — verify no iptables here)
    assert "address 10.10.10.1/24" in content
    assert "bridge-ports none" in content
    assert "sysctl -w net.ipv4.ip_forward=1" in content
    # No iptables anywhere
    assert "iptables" not in content
    assert "MASQUERADE" not in content


def test_install_phermesd_binaries_copies_and_chmods(tmp_path, monkeypatch):
    # Fake the /app/bin source location.
    fake_bin = tmp_path / "app_bin"
    fake_bin.mkdir()
    (fake_bin / "phermesd").write_bytes(b"\x7fELF-phermesd")
    (fake_bin / "phermesctl").write_bytes(b"\x7fELF-phermesctl")
    monkeypatch.setattr(host_mod, "PHERMESD_BIN_SRC", str(fake_bin))

    chroot = str(tmp_path / "chroot")
    os.makedirs(chroot)
    host_mod.install_phermesd_binaries(chroot)

    for name in ("phermesd", "phermesctl"):
        dst = tmp_path / "chroot" / "usr/local/sbin" / name
        assert dst.exists()
        assert dst.read_bytes() == (fake_bin / name).read_bytes()
        # 0o755
        mode = stat.S_IMODE(dst.stat().st_mode)
        assert mode == 0o755


def test_install_phermesd_binaries_fails_fast_when_source_missing(tmp_path, monkeypatch):
    # Source dir does not contain phermesd
    fake_bin = tmp_path / "empty"
    fake_bin.mkdir()
    monkeypatch.setattr(host_mod, "PHERMESD_BIN_SRC", str(fake_bin))
    chroot = str(tmp_path / "chroot")
    os.makedirs(chroot)
    try:
        host_mod.install_phermesd_binaries(chroot)
    except FileNotFoundError as e:
        # Error message must mention the missing source and the fix.
        msg = str(e)
        assert "phermesd" in msg
        assert "rebuild" in msg.lower() or "image" in msg.lower()
    else:
        raise AssertionError("expected FileNotFoundError")
```

- [ ] **Step 2: Run, verify fail to import**

Run: `uv run pytest tests/phermes_build/test_host.py -v 2>&1 | head -10`
Expected: ImportError — `phermes_build.host` does not exist.

- [ ] **Step 3: Implement src/phermes_build/host.py**

```python
"""Minimal Debian host installer (replaces proxmox.py).

The unchanged-from-Proxmox plumbing (debootstrap, chroot bind-mounts, policy-rcd,
GRUB install, fstab/crypttab/grub-defaults emitters, identity/root-password
helpers) is imported from proxmox.py for the duration of this slice; Task 9
deletes proxmox.py and moves those definitions in here.
"""

import os
import shutil

# Re-exported for now from proxmox.py — moved into this module in Task 9.
from phermes_build.proxmox import (  # noqa: F401  (used by cli.py)
    _bind_chroot,
    _setup_policy_rcd,
    _teardown_policy_rcd,
    _unbind_chroot,
    chroot_apt_install,
    crypttab_entry,
    enable_dev_root_ssh,
    format_boot_partitions,
    format_root_lv,
    fstab_content,
    grub_defaults_content,
    install_grub,
    lock_root_account,
    mount_boot,
    run_debootstrap,
    set_root_password,
    unmount_boot,
    write_host_identity,
)

# Source for phermesd/phermesctl binaries inside the phermes-build image.
# Populated by the Rust builder stage in Dockerfile.
PHERMESD_BIN_SRC = "/app/bin"

DEBIAN_RELEASE = "bookworm"


def debian_apt_sources() -> str:
    """sources.list for the installed host — Debian main + updates + security."""
    return (
        f"deb http://deb.debian.org/debian {DEBIAN_RELEASE} main\n"
        f"deb http://deb.debian.org/debian {DEBIAN_RELEASE}-updates main\n"
        f"deb http://security.debian.org/debian-security "
        f"{DEBIAN_RELEASE}-security main\n"
    )


def network_interfaces_content(nic: str = "eth0") -> str:
    """eth0 DHCP + vmbr0 bridge (NAT moved to nftables — see host_config)."""
    return (
        "auto lo\n"
        "iface lo inet loopback\n"
        "\n"
        f"auto {nic}\n"
        f"iface {nic} inet dhcp\n"
        "\n"
        "auto vmbr0\n"
        "iface vmbr0 inet static\n"
        "    address 10.10.10.1/24\n"
        "    bridge-ports none\n"
        "    bridge-stp off\n"
        "    bridge-fd 0\n"
        "    post-up sysctl -w net.ipv4.ip_forward=1\n"
    )


def write_network_interfaces(mount_point: str, nic: str = "eth0") -> None:
    """Write /etc/network/interfaces in the chroot."""
    path = os.path.join(mount_point, "etc/network/interfaces")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write(network_interfaces_content(nic))


def install_phermesd_binaries(mount_point: str) -> None:
    """Copy phermesd + phermesctl from PHERMESD_BIN_SRC into chroot's /usr/local/sbin/.

    Fails fast if the source is missing — a stale phermes-build image is a clear
    'rebuild the image' situation, not something to discover mid-install.
    """
    target = os.path.join(mount_point, "usr/local/sbin")
    os.makedirs(target, exist_ok=True)
    for binary in ("phermesd", "phermesctl"):
        src = os.path.join(PHERMESD_BIN_SRC, binary)
        if not os.path.isfile(src):
            raise FileNotFoundError(
                f"phermesd binary missing at {src} — rebuild the phermes-build image "
                f"(just docker-build) to refresh /app/bin/."
            )
        dst = os.path.join(target, binary)
        shutil.copy2(src, dst)
        os.chmod(dst, 0o755)
```

- [ ] **Step 4: Run tests, verify pass**

Run: `uv run pytest tests/phermes_build/test_host.py -v`
Expected: PASS (4 tests).

- [ ] **Step 5: Lint + typecheck + full unit suite still green**

Run:
```bash
uv run ruff check src/phermes_build/host.py tests/phermes_build/test_host.py
uv run ty check src/phermes_build/host.py
uv run pytest -q
```
Expected: clean / all-green.

- [ ] **Step 6: Commit**

```bash
git add src/phermes_build/host.py tests/phermes_build/test_host.py
git commit -m "feat(builder): host.py — debian sources, eth0 DHCP, install_phermesd_binaries"
```

---

### Task 7: `host.py::install_minimal_host` — the new OS installer

**Files:**
- Modify: `src/phermes_build/host.py`
- Modify: `tests/phermes_build/test_host.py`

- [ ] **Step 1: Add failing tests for install_minimal_host**

Append to `tests/phermes_build/test_host.py`:

```python
def test_install_minimal_host_calls_helpers_in_correct_order(monkeypatch, tmp_path):
    """install_minimal_host orchestrates a fixed sequence of OS-build helpers.

    We mock every helper to record the call order and assert the spec'd sequence.
    """
    calls: list[str] = []

    def rec(name):
        def _impl(*args, **kw):
            calls.append(name)
            return ""
        return _impl

    # Mock every reusable helper (currently re-exported from proxmox via host).
    for helper in [
        "format_boot_partitions",
        "run_debootstrap",
        "mount_boot",
        "write_host_identity",
        "write_network_interfaces",
        "_bind_chroot",
        "_setup_policy_rcd",
        "chroot_apt_install",
        "install_phermesd_binaries",
        "install_grub",
        "_teardown_policy_rcd",
        "_unbind_chroot",
        "unmount_boot",
    ]:
        monkeypatch.setattr(host_mod, helper, rec(helper))

    # systemd_units.install_phermesd_unit is the other newcomer.
    from phermes_build import systemd_units as sd_mod
    monkeypatch.setattr(sd_mod, "install_phermesd_unit", rec("install_phermesd_unit"))

    # Anything that writes files (crypttab/fstab/grub defaults/apt sources) we mock the
    # filesystem layer by using a tmp_path mount.
    monkeypatch.setattr(host_mod, "run_cmd", rec("chroot_apt_update"))

    mount = str(tmp_path / "chroot")
    os.makedirs(mount)
    host_mod.install_minimal_host(
        mount_point=mount,
        disk="/dev/loop0",
        luks_device="/dev/loop0p3",
        efi_device="/dev/loop0p1",
        boot_device="/dev/loop0p2",
    )

    # The spec'd order: format boot -> debootstrap -> mount boot -> identity/network
    #   -> bind chroot -> policy-rcd -> apt update -> apt install -> phermesd binaries
    #   -> phermesd unit -> grub -> initramfs -> teardown
    expected_prefix = [
        "format_boot_partitions",
        "run_debootstrap",
        "mount_boot",
        "write_host_identity",
        "write_network_interfaces",
        "_bind_chroot",
        "_setup_policy_rcd",
    ]
    assert calls[: len(expected_prefix)] == expected_prefix

    # install_phermesd_binaries + install_phermesd_unit happen AFTER apt install
    apt_install_idx = calls.index("chroot_apt_install")
    bin_idx = calls.index("install_phermesd_binaries")
    unit_idx = calls.index("install_phermesd_unit")
    grub_idx = calls.index("install_grub")
    assert apt_install_idx < bin_idx < unit_idx < grub_idx

    # Teardown comes last and in reverse-of-setup order
    assert calls[-3:] == ["_teardown_policy_rcd", "_unbind_chroot", "unmount_boot"]


def test_install_minimal_host_writes_apt_sources_crypttab_fstab_grub(
    monkeypatch, tmp_path
):
    """All four config files land in the chroot with expected content snippets."""
    # No-op all the run_cmd-heavy helpers
    for helper in [
        "format_boot_partitions",
        "run_debootstrap",
        "mount_boot",
        "_bind_chroot",
        "_setup_policy_rcd",
        "chroot_apt_install",
        "install_phermesd_binaries",
        "install_grub",
        "_teardown_policy_rcd",
        "_unbind_chroot",
        "unmount_boot",
    ]:
        monkeypatch.setattr(host_mod, helper, lambda *a, **k: "")
    monkeypatch.setattr(host_mod, "run_cmd", lambda *a, **k: "")
    from phermes_build import systemd_units as sd_mod
    monkeypatch.setattr(sd_mod, "install_phermesd_unit", lambda *a, **k: None)

    mount = str(tmp_path / "chroot")
    os.makedirs(mount)
    host_mod.install_minimal_host(
        mount_point=mount,
        disk="/dev/loop0",
        luks_device="/dev/loop0p3",
        efi_device="/dev/loop0p1",
        boot_device="/dev/loop0p2",
    )

    sources = (tmp_path / "chroot" / "etc/apt/sources.list").read_text()
    assert "bookworm main" in sources
    assert "proxmox" not in sources.lower()

    crypttab = (tmp_path / "chroot" / "etc/crypttab").read_text()
    assert "phermes_luks" in crypttab
    assert "/dev/loop0p3" in crypttab

    fstab = (tmp_path / "chroot" / "etc/fstab").read_text()
    assert "/dev/pve/root" in fstab
    assert "LABEL=boot" in fstab

    grub_defaults = (tmp_path / "chroot" / "etc/default/grub").read_text()
    assert "GRUB_ENABLE_CRYPTODISK=y" in grub_defaults
```

- [ ] **Step 2: Run, verify fail**

Run: `uv run pytest tests/phermes_build/test_host.py -v 2>&1 | head -10`
Expected: FAIL — `install_minimal_host` not found.

- [ ] **Step 3: Add `install_minimal_host` to src/phermes_build/host.py**

Append (note: `phermes_apt_packages()` is a small new helper to keep the apt list testable later):

```python
def phermes_apt_packages() -> list[str]:
    """Packages installed inside the chroot on the minimal phermesd host."""
    return [
        # Hypervisor + UEFI firmware + qemu-img (slice-#2 storage import)
        "qemu-system-x86",
        "qemu-utils",
        "ovmf",
        # phermesd storage layer shells to these
        "lvm2",
        "btrfs-progs",
        # Boot / unlock
        "cryptsetup-initramfs",
        "dropbear-initramfs",
        "grub-efi-amd64",
        # Management surface (phermesctl over SSH; eth0 DHCP)
        "openssh-server",
        "isc-dhcp-client",
        # Firewall + NAT (nftables only — iptables intentionally absent)
        "nftables",
        # Vmbr0 share + mDNS
        "samba",
        "avahi-daemon",
    ]


def install_minimal_host(
    mount_point: str,
    disk: str,
    luks_device: str,
    efi_device: str,
    boot_device: str,
) -> None:
    """Full minimal-Debian + phermesd installation sequence into a mounted chroot.

    The root LV is expected to be already mounted at `mount_point`. Formats the
    EFI/boot partitions, debootstraps Debian, mounts /boot and /boot/efi, installs
    the runtime apt set + the phermesd binaries + the systemd unit, installs a
    removable-UEFI GRUB, regenerates initramfs.
    """
    # Local import to avoid an import cycle if systemd_units ever needs host.
    from phermes_build import systemd_units
    from phermes_build.runner import run_cmd

    format_boot_partitions(efi_device, boot_device)
    run_debootstrap(mount_point)
    mount_boot(mount_point, efi_device, boot_device)
    write_host_identity(mount_point)
    write_network_interfaces(mount_point)

    sources_path = os.path.join(mount_point, "etc/apt/sources.list")
    with open(sources_path, "w") as f:
        f.write(debian_apt_sources())

    crypttab_path = os.path.join(mount_point, "etc/crypttab")
    with open(crypttab_path, "w") as f:
        f.write(crypttab_entry(luks_device, "phermes_luks"))

    fstab_path = os.path.join(mount_point, "etc/fstab")
    with open(fstab_path, "w") as f:
        f.write(fstab_content())

    grub_path = os.path.join(mount_point, "etc/default/grub")
    with open(grub_path, "w") as f:
        f.write(grub_defaults_content())

    _bind_chroot(mount_point)
    _setup_policy_rcd(mount_point)
    try:
        run_cmd(["chroot", mount_point, "apt-get", "update"])
        chroot_apt_install(mount_point, *phermes_apt_packages())
        install_phermesd_binaries(mount_point)
        systemd_units.install_phermesd_unit(mount_point)
        install_grub(mount_point)
        run_cmd(["chroot", mount_point, "update-initramfs", "-u", "-k", "all"])
    finally:
        _teardown_policy_rcd(mount_point)
        _unbind_chroot(mount_point)
        unmount_boot(mount_point)
```

- [ ] **Step 4: Run tests, verify pass**

Run: `uv run pytest tests/phermes_build/test_host.py -v`
Expected: PASS (6 tests now).

- [ ] **Step 5: Full unit suite + lint + typecheck**

Run:
```bash
uv run pytest -q
uv run ruff check src/phermes_build/host.py tests/phermes_build/test_host.py
uv run ty check src/phermes_build/host.py
```
Expected: all-green / clean.

- [ ] **Step 6: Commit**

```bash
git add src/phermes_build/host.py tests/phermes_build/test_host.py
git commit -m "feat(builder): host.install_minimal_host — the new OS installer"
```

---

### Task 8: `cli.py` — orchestration swap, --no-vm flag, drop --toy-vm/--linux-node

**Files:**
- Modify: `src/phermes_build/cli.py`
- Modify: `tests/phermes_build/test_cli.py`

- [ ] **Step 1: Inspect current cli.py**

Run `grep -nE "import |^def |add_typer|@app|--toy-vm|--linux-node|--import-vm|--no-vm|provision_vms|_install_proxmox|_install_node_vm|node_vm" src/phermes_build/cli.py | head -40` to map the change surface. The mods to make:

1. Replace `from phermes_build import … proxmox …` with `from phermes_build import … host as host_mod, systemd_units, vm as vm_mod`.
2. Drop `import node_vm`.
3. Replace every `proxmox.<func>(...)` call with the equivalent `host_mod.<func>(...)`.
4. Replace the existing `_install_proxmox(layout)` step with `_install_minimal_host(layout)` calling `host_mod.install_minimal_host(...)`.
5. Drop the `_install_node_vm` helper and the optional "Installing Linux node VM" step.
6. Replace `_provision_vms(cfg)` with a new helper that, when not `--no-vm`, calls `vm_mod.write_linux_def(PVE_ROOT_MOUNT)` and `vm_mod.provision_linux_disk(source=<import_vm["linux"] if present>)`.
7. Add the `--no-vm` flag; drop `--toy-vm` and `--linux-node`. Keep `--import-vm linux=<path>`.

- [ ] **Step 2: Add failing tests**

Append to `tests/phermes_build/test_cli.py`:

```python
from unittest.mock import patch

from typer.testing import CliRunner

from phermes_build import cli as cli_mod

runner = CliRunner()


def test_no_toy_vm_or_linux_node_flag():
    result = runner.invoke(cli_mod.app, ["--help"])
    assert result.exit_code == 0
    assert "--toy-vm" not in result.stdout
    assert "--linux-node" not in result.stdout


def test_no_vm_flag_is_advertised():
    result = runner.invoke(cli_mod.app, ["--help"])
    assert "--no-vm" in result.stdout


def test_orchestration_calls_install_minimal_host_not_proxmox(monkeypatch):
    """Smoke: the build pipeline routes through host.install_minimal_host."""
    seen: list[str] = []

    # No-op every step the build invokes; record which OS-installer was picked.
    monkeypatch.setattr(cli_mod, "_setup_luks", lambda *a, **k: seen.append("luks"))
    monkeypatch.setattr(cli_mod, "_setup_lvm", lambda *a, **k: seen.append("lvm"))
    monkeypatch.setattr(cli_mod, "_setup_btrfs", lambda *a, **k: seen.append("btrfs"))
    monkeypatch.setattr(cli_mod, "_setup_exfat", lambda *a, **k: seen.append("exfat"))
    monkeypatch.setattr(
        cli_mod,
        "_install_minimal_host",
        lambda *a, **k: seen.append("install_minimal_host"),
    )
    monkeypatch.setattr(cli_mod, "_configure_host", lambda *a, **k: seen.append("configure_host"))
    monkeypatch.setattr(cli_mod, "_setup_credentials", lambda *a, **k: seen.append("credentials"))
    monkeypatch.setattr(cli_mod, "_provision_linux_vm", lambda *a, **k: seen.append("provision_vm"))
    monkeypatch.setattr(cli_mod, "_write_firstboot", lambda *a, **k: seen.append("firstboot"))

    # Make validate + planning succeed.
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(
        cli_mod, "plan_disk_layout", lambda d, share_size_gb=None: object()
    )
    monkeypatch.setattr(cli_mod, "_partition", lambda *a, **k: seen.append("partition"))

    result = runner.invoke(
        cli_mod.app,
        ["/dev/loop0", "--skip-os-install", "--no-vm", "--dev-credentials"],
    )
    # --skip-os-install short-circuits before _install_minimal_host; just check no crash.
    assert result.exit_code == 0
    assert "install_minimal_host" not in seen  # because of --skip-os-install


def test_import_vm_linux_routes_into_provision_linux_disk(monkeypatch):
    """--import-vm linux=<path> reaches provision_linux_disk(source=<path>)."""
    seen: dict = {}

    monkeypatch.setattr(cli_mod, "_setup_luks", lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "_setup_lvm", lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "_setup_btrfs", lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "_setup_exfat", lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "_install_minimal_host", lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "_configure_host", lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "_setup_credentials", lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "_write_firstboot", lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "plan_disk_layout", lambda d, share_size_gb=None: object())
    monkeypatch.setattr(cli_mod, "_partition", lambda *a, **k: None)

    def fake_provision(source=None):
        seen["source"] = source

    monkeypatch.setattr(cli_mod, "_provision_linux_vm", fake_provision)

    result = runner.invoke(
        cli_mod.app,
        ["/dev/loop0", "--import-vm", "linux=/tmp/x.qcow2", "--dev-credentials"],
    )
    assert result.exit_code == 0, result.stdout
    assert seen.get("source") == "/tmp/x.qcow2"
```

(Adjust the existing test_cli.py imports/decorators to whatever is already there. The exact list of `monkeypatch.setattr` targets depends on how the existing build flow is decomposed — the spec lists the orchestration steps; align with them.)

- [ ] **Step 3: Run, verify fail**

Run: `uv run pytest tests/phermes_build/test_cli.py -v 2>&1 | head -30`
Expected: FAIL — flag and helper names not yet aligned with the new spec.

- [ ] **Step 4: Edit src/phermes_build/cli.py**

Make these changes (paste the exact edits as appropriate to the current file; the function `build()` is the Typer command):

(a) Imports at top — replace
```python
from phermes_build import (
    ...
    proxmox,
    ...
    node_vm,
    ...
)
```
with
```python
from phermes_build import (
    ...
    host as host_mod,
    systemd_units,
    ...
)
from phermes_build import vm as vm_mod
```
(Drop `proxmox` and `node_vm`.)

(b) The `build()` Typer command's flag list — drop `--toy-vm` and `--linux-node`; add `--no-vm`. Keep `--import-vm` (it's a `list[str]` of `flavor=path` entries; the MVP recognizes only `linux=<path>`):

```python
@app.command()
def build(
    disk: str = typer.Argument(..., help="Target block device"),
    skip_os_install: bool = typer.Option(False, "--skip-os-install"),
    share_size: str = typer.Option("250G", "--share-size"),
    share_encrypted: bool = typer.Option(False, "--share-encrypted"),
    no_vm: bool = typer.Option(False, "--no-vm", help="Skip Linux VM provisioning."),
    import_vm: list[str] = typer.Option(
        [], "--import-vm", help="flavor=path (only linux= is supported in the MVP)"
    ),
    verbose: bool = typer.Option(False, "--verbose"),
    dev_credentials: bool = typer.Option(False, "--dev-credentials"),
    dev_ssh_pubkey: str | None = typer.Option(None, "--dev-ssh-pubkey"),
    luks_passphrase: str | None = typer.Option(None, "--luks-passphrase"),
) -> None:
    # … existing body …
```

(c) Helper renames inside `build()`:
- Replace `("Installing Proxmox VE", lambda: _install_proxmox(layout))` with
  `("Installing minimal Debian host + phermesd", lambda: _install_minimal_host(layout))`.
- Delete the conditional `os_steps.append(("Installing Linux node VM (dev)", lambda: _install_node_vm()))`.
- Replace `("Provisioning VMs", lambda: _provision_vms(cfg))` with
  `("Provisioning Linux VM", lambda: _provision_linux_vm(source=_linux_source(import_vm)))`
  guarded by `if not no_vm`.

(d) Helper functions — replace at module level:

```python
def _install_minimal_host(layout) -> None:
    """Install minimal Debian + phermesd into the mounted chroot."""
    from phermes_build.runner import run_cmd

    run_cmd(["mount", "/dev/pve/root", PVE_ROOT_MOUNT])
    host_mod.install_minimal_host(
        mount_point=PVE_ROOT_MOUNT,
        disk=layout.disk,
        luks_device=layout.partition("luks"),
        efi_device=layout.partition("efi"),
        boot_device=layout.partition("boot"),
    )


def _provision_linux_vm(source: str | None) -> None:
    vm_mod.write_linux_def(PVE_ROOT_MOUNT)
    vm_mod.provision_linux_disk(source=source)


def _linux_source(import_vm_args: list[str]) -> str | None:
    """Parse --import-vm linux=<path> (MVP only supports linux=)."""
    for entry in import_vm_args:
        flavor, _, path = entry.partition("=")
        if flavor == "linux" and path:
            return path
        if flavor and flavor != "linux":
            raise typer.BadParameter(
                f"--import-vm flavor '{flavor}' is not supported in the MVP "
                f"(only 'linux=<path>')."
            )
    return None
```

(e) Replace every previous `proxmox.<...>` reference in `_setup_credentials`/`_setup_lvm`/etc. with `host_mod.<...>` (same function names — they're re-exported by host.py per Task 6).

- [ ] **Step 5: Run tests, verify pass**

Run: `uv run pytest tests/phermes_build/test_cli.py -v`
Expected: PASS.

- [ ] **Step 6: Full unit suite + lint + typecheck**

Run:
```bash
uv run pytest -q
uv run ruff check src/phermes_build/cli.py tests/phermes_build/test_cli.py
uv run ty check src/phermes_build/cli.py
```
Expected: all-green / clean. (Tests for the now-stale `proxmox` and `node_vm` modules may still pass for now — they're deleted in Task 9.)

- [ ] **Step 7: Commit**

```bash
git add src/phermes_build/cli.py tests/phermes_build/test_cli.py
git commit -m "feat(builder): cli orchestration swap, --no-vm flag, drop --toy-vm/--linux-node"
```

---

### Task 9: Delete `proxmox.py` + `node_vm.py`; move re-exported helpers into `host.py`

**Files:**
- Delete: `src/phermes_build/proxmox.py`
- Delete: `src/phermes_build/node_vm.py`
- Delete: `tests/phermes_build/test_proxmox.py`
- Delete: `tests/phermes_build/test_node_vm.py`
- Modify: `src/phermes_build/host.py`

- [ ] **Step 1: Move the re-exported helpers' bodies into host.py**

At the top of `src/phermes_build/host.py`, the `from phermes_build.proxmox import (...)` block is the placeholder. Replace it with the actual function definitions. Concretely, copy these from the soon-to-be-deleted `src/phermes_build/proxmox.py` into `src/phermes_build/host.py` (paste their bodies verbatim, in alphabetical order, immediately after the existing `DEBIAN_RELEASE = ...` constant):

`_bind_chroot`, `_setup_policy_rcd`, `_teardown_policy_rcd`, `_unbind_chroot`,
`chroot_apt_install`, `crypttab_entry`, `enable_dev_root_ssh`,
`format_boot_partitions`, `format_root_lv`, `fstab_content`,
`grub_defaults_content`, `install_grub`, `lock_root_account`, `mount_boot`,
`run_debootstrap`, `set_root_password`, `unmount_boot`, `write_host_identity`.

Also copy the constants those bodies use:
`_CHROOT_BIND_MOUNTS = ["proc", "sys", "dev", "dev/pts"]`
`TEMP_ROOT_PASSWORD = "phermes-change-me"`
`DEFAULT_HOSTNAME = "phermes"`
(plus any others that the migrated functions reference).

Add `import contextlib` and `from phermes_build.runner import run_cmd` to the top of host.py if not already present.

Delete the `from phermes_build.proxmox import (...)` block from host.py.

- [ ] **Step 2: Verify host.py imports are minimal**

Run `head -20 src/phermes_build/host.py` and confirm it imports `contextlib`, `os`, `shutil`, and `from phermes_build.runner import run_cmd` — nothing from `phermes_build.proxmox` should remain.

- [ ] **Step 3: Delete proxmox.py and node_vm.py and their tests**

```bash
git rm src/phermes_build/proxmox.py
git rm src/phermes_build/node_vm.py
git rm tests/phermes_build/test_proxmox.py
git rm tests/phermes_build/test_node_vm.py
```

- [ ] **Step 4: grep for stragglers**

Run:
```bash
grep -rn "phermes_build.proxmox\|phermes_build.node_vm\|from phermes_build import proxmox\|from phermes_build import node_vm" src/ tests/ 2>&1
```
Expected: empty output. Anything that appears must be edited to import from `host` (helpers) or removed.

- [ ] **Step 5: Run the full unit suite + lint + typecheck**

```bash
uv run pytest -q
uv run ruff check src/ tests/
uv run ty check src/
```
Expected: all-green / clean.

- [ ] **Step 6: Commit**

```bash
git add -A src/phermes_build tests/phermes_build
git commit -m "refactor(builder): delete proxmox.py + node_vm.py; helpers live in host.py"
```

---

### Task 10: Justfile `smoke-full` + README + CHANGELOG

**Files:**
- Modify: `Justfile`
- Modify: `README.md`
- Modify: `CHANGELOG.md`

- [ ] **Step 1: Justfile — drop the now-bogus `smoke-full-node` recipe**

In `Justfile`, the recipe `smoke-full-node native="0": (_smoke-build native "--linux-node")` references a deleted flag. Delete that recipe entirely.

The `smoke-full` recipe already calls `_smoke-build` with no extra flags, which is exactly what the MVP wants (`phermes-build` itself is now phermesd-based, so the same invocation does the new thing). No other recipe change is strictly required for this slice.

- [ ] **Step 2: Verify just parses + lists**

Run: `just --list 2>&1 | grep -E "^\s*(smoke-full|smoke-full-node)"`
Expected: `smoke-full` appears; `smoke-full-node` does NOT.

- [ ] **Step 3: README — describe the new boot chain**

Find the existing description of the boot chain in `README.md` (a line like `└─ PHermes — LUKS2-encrypted SSD → GRUB → Proxmox VE host` or similar). Update to:

```
└─ PHermes — LUKS2-encrypted SSD → GRUB → minimal Debian → phermesd → Linux guest
```

In the "Status" or "phermesd" subsection, add a one-paragraph note that the appliance now boots a phermesd-based host (no Proxmox), management is `phermesctl` over SSH, and that the `--import-vm linux=<path>` flag preserves the install-time provisioning UX. Pointer to the spec: `docs/superpowers/specs/2026-06-04-phermesd-host-image-mvp-design.md`.

- [ ] **Step 4: CHANGELOG — add the slice-#6 MVP entry**

Under the `## [Unreleased]` → `### Added` (or create a `### Changed`) section, add:

```markdown
- `phermes-build` host image migration (slice #6 MVP): the assembled appliance now boots a
  **minimal Debian host running phermesd** instead of Proxmox VE. The boot chain is
  `EFI → GRUB → Debian → systemd → phermesd → KVM Linux guest`. Management is `phermesctl`
  over SSH; `--import-vm linux=<path>` preserves install-time VM provisioning; a new
  `--no-vm` flag installs the host alone. `proxmox.py` and `node_vm.py` deleted; the
  proven LUKS/LVM-thin/Btrfs/exFAT/GRUB-removable/Dropbear/Samba/Avahi plumbing reused.
```

- [ ] **Step 5: Run the full unit suite one more time**

```bash
uv run pytest -q
```
Expected: all-green.

- [ ] **Step 6: Commit**

```bash
git add Justfile README.md CHANGELOG.md
git commit -m "docs: phermesd host image MVP — docs + drop smoke-full-node"
```

---

## Out of Scope (carries to later slices)

- Console proxy / web UI → slice #4.
- Cloud-init NoCloud seed generation → slice #4.
- macOS guest (OpenCore + applesmc + SMBIOS) → slice #5.
- Windows guest → slice #5 extension.
- Signed/pinned phermesd release artifacts → hardening pass.
- Shared Python↔Rust constants config or CI grep → hardening pass.
- Stripping systemd for an immutable/OpenRC host → later vision.

---

## Self-Review

**1. Spec coverage** — every spec section maps to a task:

| Spec section / requirement | Task(s) |
|---|---|
| Architecture & file structure: `host.py` + `systemd_units.py` new, `vm.py` rewritten, `proxmox.py`/`node_vm.py` deleted, `host_config.py` modified | 3, 4, 5, 6, 7, 9 |
| Multi-stage `Dockerfile` shipping phermesd binaries into `/app/bin/` | 1, 2 |
| Dropping dead apt packages (fdisk/gdisk/parted/wget) | 1 |
| `install_minimal_host` apt set + chroot binary copy + systemd unit install | 6, 7 |
| `phermesd.service` unit text + chroot installer | 3 |
| `vm.py` constants mirror phermesd storage; `write_linux_def`; `provision_linux_disk` | 4 |
| `host_config.py`: drop RBAC, add NAT chain | 5 |
| Networking: eth0 DHCP, vmbr0 keeps but no iptables, NAT in nftables | 5, 6 |
| `cli.py` orchestration swap; `--no-vm`; drop `--toy-vm`/`--linux-node`; keep `--import-vm linux=` | 8 |
| Errors (defensive binaries-present check) | 6 |
| Testing: unit for host/vm/systemd_units/host_config + smoke-full | 3, 4, 5, 6, 7, 8, 10 |
| Success criterion: smoke-full produces appliance answering `phermesctl list` | 10 (recipe) — operator verifies on real loop device |
| Out-of-scope items | listed above |

**2. Placeholder scan** — none. Two `Open questions` from the spec (Debian sources line; systemd unit dep name) are answered concretely in the plan: Task 6 uses `bookworm main` + `-updates main` + `bookworm-security main` (the existing builder used the same set); Task 3 documents the choice to omit `After=lvm2-activation.service` because Debian's canonical unit is `lvm2-monitor.service` and `local-fs.target` ordering is already established by systemd defaults. The third open question (bundled qcow2 for smoke) is resolved in favor of `--no-vm` for the MVP smoke (Task 10 doesn't ship a bundled image).

**3. Type consistency** — `STORAGE_VG = "pve"`, `STORAGE_POOL = "data"`, `OWNER_TAG = "phermesd"`, `LINUX_VMID = 102`, `DEFAULT_DISK_GB = 40`, `DEFAULT_MEMORY_MIB = 4096`, `DEFAULT_VCPUS = 4`, `PHERMESD_BIN_SRC = "/app/bin"`, function signatures (`write_linux_def(chroot_mount, *, memory_mib, vcpus)`, `provision_linux_disk(size_gb, source)`, `install_phermesd_binaries(mount_point)`, `install_phermesd_unit(mount_point)`, `install_minimal_host(mount_point, disk, luks_device, efi_device, boot_device)`) are identical across tasks and tests.

---

## Execution Handoff

Plan complete and saved to `docs/superpowers/plans/2026-06-04-phermesd-host-image-mvp.md`.
