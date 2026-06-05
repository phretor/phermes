# phermesd — Host Image MVP (Design)

**Date:** 2026-06-04
**Status:** Draft — pending implementation plan
**Sub-project:** #6 (MVP) of the `phermesd` replacement for Proxmox VE

---

## Context & motivation

[Slice #1](2026-06-03-phermesd-design.md) built the orchestrator daemon and
[slice #2](2026-06-03-phermesd-storage-design.md) added storage & snapshots, but
`phermes-build` (the Python builder) still assembles a **Proxmox VE** host. This sub-project
replaces the Proxmox install with a minimal Debian host that runs `phermesd` directly, giving
PHermes its first end-to-end appliance without any Proxmox dependency.

It is scoped as an **MVP**: deliver a bootable phermesd-based SSD as fast as possible, then
factor out networking (#3), console + cloud-init (#4), and the macOS path (#5) as later
slices on the running appliance. Linux-only guests for the MVP; the proven low-level builder
plumbing (LUKS, LVM-thin, Btrfs, exFAT, removable-UEFI GRUB, partitioner, Dropbear-in-
initramfs, Samba, Avahi) is reused unchanged.

## Decisions (resolved during brainstorming)

| Decision | Choice |
|---|---|
| Init story | **systemd unit on Debian** (`phermesd.service`). phermesd remains init-agnostic (no `sd_notify`/`sd_listen`); systemd is just the launcher. |
| Binary delivery | **Multi-stage Docker build.** A `rust:1.95-slim-bookworm` builder stage in `Dockerfile` produces release `phermesd` + `phermesctl`; the final stage `COPY`s them into `/app/bin/` so install-time copy into the chroot is trivial. |
| VM at install time | **phermes-build provisions the Linux VM disk during install** (mirrors today's `--import-vm` UX): create the thin LV in the just-built VG, tag it `phermesd`, optionally `qemu-img convert` a source image into it, write `/etc/phermes/vms/linux.toml`. |
| Management surface | **`phermesctl` over SSH only.** OpenSSH on port 22 for management; Dropbear in initramfs for headless LUKS unlock; no web UI in MVP (returns in #4). |

## Scope boundary

**In:**
- `phermes-build` runs end-to-end and produces a bootable SSD whose boot chain is
  `EFI → GRUB → minimal Debian → systemd → phermesd → QEMU/KVM Linux guest`.
- `Dockerfile` rewritten as a multi-stage build that ships `phermesd`/`phermesctl` binaries.
- A new `host.py` replaces `proxmox.py`; the genuinely Debian (non-Proxmox) plumbing moves
  over unchanged.
- A new `vm.py` writes `/etc/phermes/vms/linux.toml` and provisions an LVM-thin VM disk in
  the freshly-built VG using the same conventions as slice #2.
- A new `systemd_units.py` emits the `phermesd.service` unit text.
- Networking: `eth0` → DHCP (the pmxcfs static-IP constraint is gone); `vmbr0` keeps
  `10.10.10.1/24`; NAT moves from `iptables MASQUERADE` to nftables.
- `host_config.py::configure_proxmox_rbac` and the `PHermesUser` role are deleted; the rest
  of `host_config.py` is preserved.
- CLI flags `--import-vm linux=<path>` (preserved) and `--no-vm` (new) work end-to-end.
- The existing `just smoke-full` recipe is *replaced* (same name) to drive the new build.

**Out (deferred):**
- Console proxy / web UI (#4).
- Cloud-init NoCloud seed generation; `node_vm.py` deleted for now (returns in #4).
- macOS (#5) and Windows guests.
- Stripping systemd for an immutable/OpenRC host (later vision).
- Signed/pinned phermesd release artifacts (hardening).
- A shared Python ↔ Rust constants config or a CI grep to enforce equality
  (documented mitigation only for now).

## Architecture & file structure

### Replaced (the Proxmox-shaped surface)

| Old | Becomes |
|---|---|
| `src/phermes_build/proxmox.py` (392 L) | **deleted** |
| `proxmox.py::install_proxmox` | `host.py::install_minimal_host` |
| `proxmox.py::proxmox_apt_sources` | `host.py::debian_apt_sources` (no pve repo) |
| `proxmox.py::pve_init_script` / `pve_init_service` / `install_pve_firstboot_init` | **deleted** (no pmxcfs) |
| `proxmox.py::fetch_proxmox_keyring` | **deleted** |
| `proxmox.py::SMOKE_ADDRESS` static-IP hack | **deleted** (eth0 → DHCP) |
| `src/phermes_build/vm.py` (73 L) | rewritten: writes `/etc/phermes/vms/<id>.toml` + provisions the LVM-thin volume |
| `host_config.py::configure_proxmox_rbac` + `PHERMES_ROLE` | **deleted** |
| `src/phermes_build/node_vm.py` (83 L) | **deleted** for MVP (cloud-init returns in #4) |

### Reused unchanged (Debian plumbing, not Proxmox plumbing)

The following move from `proxmox.py` into `host.py` byte-for-byte (or with trivial renames):
`_bind_chroot` / `_unbind_chroot` / `_setup_policy_rcd` / `_teardown_policy_rcd`,
`format_boot_partitions` / `mount_boot` / `unmount_boot` / `install_grub` /
`chroot_apt_install`, `write_host_identity` / `write_network_interfaces` (modified — see
[Networking](#networking)), `set_root_password` / `lock_root_account` / `enable_dev_root_ssh`,
`format_root_lv`, `crypttab_entry`, `fstab_content`, `grub_defaults_content`,
`run_debootstrap`.

All of `disk.py`, `partitioner.py`, `luks.py`, `lvm.py`, `btrfs.py`, `exfat.py`,
`firstboot.py`, `runner.py`, `models.py`, and `host_config.py` (minus the RBAC bits) remain
untouched.

### New files

- `src/phermes_build/host.py` — successor to `proxmox.py`.
- `src/phermes_build/systemd_units.py` — small module that emits the `phermesd.service`
  unit text and installs it (separate so unit text is unit-testable in isolation).

### Modified files

- `Dockerfile` — multi-stage build (Rust builder + Python final stage).
- `src/phermes_build/cli.py` — orchestration call sites swapped.
- `README.md`, `CHANGELOG.md` — describe the new boot chain.

### New boot chain on the installed appliance

```
EFI → GRUB → minimal Debian → systemd → phermesd.service → QEMU/KVM guest
```
(Was: `… → Proxmox VE → … → QEMU/KVM guest`.)

## `install_minimal_host`

Same skeleton as `install_proxmox` (format boot → debootstrap → bind chroot → policy-rcd
→ apt → grub → initramfs → teardown), with the Proxmox layer swapped for a phermesd layer.

**apt set on the host** (replaces `proxmox-ve postfix open-iscsi iptables`):
```
qemu-system-x86 qemu-utils ovmf
lvm2 btrfs-progs
cryptsetup-initramfs dropbear-initramfs grub-efi-amd64
openssh-server isc-dhcp-client
nftables samba avahi-daemon
```

The `Dockerfile`'s base apt set also gets a cleanup pass: `fdisk`, `gdisk`, `parted`, and
`wget` are unused by the Python builder and are removed (sfdisk lives in `util-linux`; no
Proxmox keyring to fetch).

### Phermesd binaries into the chroot

```python
def install_phermesd_binaries(mount_point: str) -> None:
    """Copy phermesd + phermesctl from /app/bin/ into the chroot's /usr/local/sbin/."""
    target = os.path.join(mount_point, "usr/local/sbin")
    os.makedirs(target, exist_ok=True)
    for binary in ("phermesd", "phermesctl"):
        src = f"/app/bin/{binary}"
        dst = os.path.join(target, binary)
        shutil.copy2(src, dst)
        os.chmod(dst, 0o755)
```

`/app/bin/` is populated by the multi-stage `Dockerfile`:

```dockerfile
FROM rust:1.95-slim-bookworm AS phermesd-builder
WORKDIR /work
COPY phermesd/ ./
RUN cargo build --release --bin phermesd --bin phermesctl

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    btrfs-progs cryptsetup debootstrap dosfstools exfatprogs \
    lvm2 udev util-linux ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=ghcr.io/astral-sh/uv:0.11.17 /uv /usr/local/bin/uv
COPY --from=phermesd-builder /work/target/release/phermesd /app/bin/phermesd
COPY --from=phermesd-builder /work/target/release/phermesctl /app/bin/phermesctl
WORKDIR /app
COPY .python-version pyproject.toml uv.lock ./
RUN uv sync --frozen --no-dev --no-install-project
COPY src/ ./src/
RUN uv sync --frozen --no-dev
ENTRYPOINT ["uv", "run", "--no-sync", "phermes-build"]
```

A defensive check inside `install_phermesd_binaries` verifies `/app/bin/phermesd` exists
before opening the chroot, so a stale image fails fast with a clear "rebuild the
phermes-build image" message rather than mid-install.

### `phermesd.service` systemd unit

```ini
[Unit]
Description=PHermes VM Orchestrator
Documentation=https://github.com/phretor/phermes
After=network-online.target lvm2-activation.service
Wants=network-online.target
ConditionPathExists=/dev/kvm

[Service]
Type=simple
ExecStart=/usr/local/sbin/phermesd --vms-dir /etc/phermes/vms --run-dir /run/phermesd --socket /run/phermesd/control.sock
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
```

Enabled in the chroot via the same wants-symlink trick used by the old
`install_pve_firstboot_init` (no live systemd to call `systemctl enable`):

```python
def install_phermesd_unit(mount_point: str) -> None:
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

## VM provisioning at install time

`vm.py` is rewritten as a small Linux-only module (Windows + macOS return in #5):

```python
# Mirror phermesd's storage conventions (phermesd/src/storage.rs). A future
# hardening pass can promote these into a shared config + a CI check.
STORAGE_VG = "pve"
STORAGE_POOL = "data"
OWNER_TAG = "phermesd"
LINUX_VMID = 102
DEFAULT_DISK_GB = 40
DEFAULT_MEMORY_MIB = 4096
DEFAULT_VCPUS = 4

def write_linux_def(chroot_mount: str, *, memory_mib: int = DEFAULT_MEMORY_MIB,
                    vcpus: int = DEFAULT_VCPUS) -> None:
    """Write /etc/phermes/vms/linux.toml in the chroot."""
    # … emits the TOML below …

def provision_linux_disk(size_gb: int = DEFAULT_DISK_GB, source: str | None = None) -> None:
    """Create the thin LV, tag it 'phermesd', optionally populate from source."""
    device = f"/dev/{STORAGE_VG}/vm-{LINUX_VMID}-disk-0"
    run_cmd(["lvcreate", "--thin", "--virtualsize", f"{size_gb}G",
             f"{STORAGE_VG}/{STORAGE_POOL}", "-n", f"vm-{LINUX_VMID}-disk-0"])
    run_cmd(["lvchange", "--addtag", OWNER_TAG, device])
    if source is not None:
        run_cmd(["qemu-img", "convert", "-O", "raw", source, device])
```

The chroot's `/etc/phermes/vms/linux.toml`:

```toml
flavor = "linux"
[resources]
memory_mib = 4096
vcpus = 4
cpu = "host"
[firmware]
ovmf_code = "/usr/share/OVMF/OVMF_CODE.fd"
ovmf_vars_template = "/usr/share/OVMF/OVMF_VARS.fd"
[[disk]]
path = "/dev/pve/vm-102-disk-0"
format = "raw"
interface = "virtio-scsi"
[[net]]
bridge = "vmbr0"
model = "virtio-net"
[console]
serial = true
vnc = true
```

**Convention drift is the real risk.** Python's constants must stay aligned with Rust's
(`phermesd/src/storage.rs`). Two mitigations baked into the plan:
1. A comment in `vm.py` pointing at `phermesd/src/storage.rs` and the slice-#2 spec.
2. Hardcode the *exact same* names; a CI grep can later assert equality (follow-up, not MVP).

## Networking

Three changes from today's `/etc/network/interfaces`:

| Today (`proxmox.py::network_interfaces_content`) | MVP |
|---|---|
| `eth0 inet static, address 10.0.2.15/24, gateway 10.0.2.2` | `eth0 inet dhcp` (pmxcfs constraint gone) |
| `vmbr0 inet static, address 10.10.10.1/24, post-up iptables -t nat … MASQUERADE` | `vmbr0` same address, `post-up sysctl ip_forward=1` only |
| `iptables -t nat … MASQUERADE` in `/etc/network/interfaces` | NAT lives in `host_config.nftables_ruleset()`'s new `nat/postrouting` chain |

So **iptables is dropped entirely** — nftables is the single firewall + NAT surface.
`host_config.nftables_ruleset()` gains:

```nftables
table ip nat {
    chain postrouting {
        type nat hook postrouting priority 100;
        ip saddr 10.10.10.0/24 oifname "eth0" masquerade
    }
}
```

**Unchanged:** Avahi `phermes.local` mDNS (`avahi_service_config()`), Samba bound to vmbr0
only (`samba_config()`), Dropbear in initramfs on port 2222 for headless LUKS unlock
(`dropbear_initramfs_config()`), OpenSSH on port 22 for `phermesctl`-over-SSH management.

## Orchestration changes (`cli.py`)

| Today | MVP |
|---|---|
| validate_disk | unchanged |
| _setup_luks → _setup_lvm → _setup_btrfs → _setup_exfat | unchanged |
| `_install_proxmox(layout)` | `_install_minimal_host(layout)` |
| _configure_host (nftables/samba/dropbear/avahi/RBAC/motd/network) | same minus RBAC; nftables now includes NAT |
| _setup_credentials (root password, dev SSH) | unchanged |
| `vm.provision_vm(cfg)` writes Proxmox config + `qm importdisk` | `vm.write_linux_def() + vm.provision_linux_disk(source=…)` |
| Optional `node_vm.install_node_vm()` | **dropped** (cloud-init returns in #4) |
| `write_firstboot_flag` | unchanged |

Inside `_install_minimal_host`, the new two steps run right after the chroot `apt install`
succeeds and before `install_grub` (so the chroot is still bind-mounted and policy-rcd is
still in place):
1. `host.install_phermesd_binaries(mount_point)`.
2. `systemd_units.install_phermesd_unit(mount_point)`.

### CLI flags

| Flag | Status |
|---|---|
| `--import-vm linux=<path>` | preserved; calls `provision_linux_disk(source=<path>)` |
| `--no-vm` | **new**; skip VM provisioning entirely (host-only install) |
| `--toy-vm` | **dropped** (Proxmox-specific) |
| `--linux-node` | **dropped** for MVP (returns in #4) |
| `--share-size`, `--share-encrypted`, `--skip-os-install`, `--verbose`, `--dev-credentials`, `--dev-ssh-pubkey`, `--luks-passphrase` | preserved unchanged |

## Errors

No new error categories. `phermes-build` continues to use `runner.run_cmd`; failures
propagate as `CalledProcessError` with the offending command captured; `cli.py` renders
them as today. The one defensive check (binaries present in `/app/bin/`) fails fast with an
actionable "rebuild the phermes-build image" message.

## Testing

| Layer | What | Notes |
|---|---|---|
| **Unit** (no root, no Docker) | New tests for `host.py` (`install_phermesd_binaries`, `install_phermesd_unit`, `debian_apt_sources`, the modified `write_network_interfaces`); `vm.py` (`write_linux_def` emits the expected TOML, `provision_linux_disk` runs the expected lvcreate+lvchange+qemu-img sequence); `systemd_units.py` (unit text shape). | Use `monkeypatch.setattr(<mod>, "run_cmd", fake)` per CLAUDE.md. |
| **Unchanged unit tests** | `disk`, `partitioner`, `luks`, `lvm`, `btrfs`, `exfat`, `runner`, `firstboot`, `host_config` (the surviving bits) | Must stay green. |
| **Integration** (loop device + root, gated like today) | `just smoke-full` is **replaced** — same recipe name, now runs the phermesd-based install. CI's `integration-tests` job runs it against a loop device. | Per "Replace, don't deprecate". |
| **End-to-end smoke** | `just smoke-create && just smoke-full && just smoke-qemu` boots the appliance under OVMF. | See success criterion. |

## Success criterion

`just smoke-create && just smoke-full && just smoke-qemu` boots the assembled SSD under
OVMF; the smoke harness then asserts, after SSH'ing in:

1. SSH on port 22 reachable as root with the dev key.
2. `systemctl is-active phermesd` returns `active`.
3. `phermesctl list` returns valid JSON with the `linux` def.
4. `/dev/pve/vm-102-disk-0` exists and is tagged `phermesd`.

## Open questions (for the plan)

- The exact Debian apt sources line (`deb http://deb.debian.org/debian bookworm main` vs.
  also pulling `bookworm-updates`/`bookworm-security`) — confirm at plan time; current
  builder includes all three for Proxmox, and we likely keep that.
- Whether the systemd unit's `After=lvm2-activation.service` is the correct Debian unit name
  (vs. `lvm2-monitor.service` or a generic `local-fs.target`) — verify against a freshly
  installed bookworm at plan time and adjust.
- Whether the smoke harness should include a tiny bundled Linux qcow2 so the MVP demos a
  `phermesctl activate linux` flow end-to-end, or whether `--no-vm` is fine and the
  reviewer brings their own image. Lean toward `--no-vm` for the MVP smoke; deciding at
  plan time.
