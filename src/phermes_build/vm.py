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
WINDOWS_VMID = 101

DEFAULT_DISK_GB = 40
DEFAULT_MEMORY_MIB = 4096
DEFAULT_VCPUS = 4

WINDOWS_DEFAULT_DISK_GB = 100
WINDOWS_DEFAULT_MEMORY_MIB = 8192
WINDOWS_DEFAULT_VCPUS = 4

SEED_DIR = "/var/lib/phermes/seed"
LINUX_SEED_PATH = f"{SEED_DIR}/linux.iso"


def _linux_def_text(*, memory_mib: int, vcpus: int, seed_iso_path: str | None) -> str:
    """Render /etc/phermes/vms/linux.toml content.

    Production conventions:
      * raw block device at /dev/<vg>/vm-<vmid>-disk-0
      * vmbr0 bridge (set up by host.write_network_interfaces)
      * virtio-scsi + virtio-net (perf + driver availability in Linux guests)
      * serial + vnc unix sockets exposed (slice #4's console proxy reads them)

    When ``seed_iso_path`` is set, a second [[disk]] block is appended for the
    cloud-init NoCloud CDROM. When None (production), no CDROM is emitted.
    """
    text = (
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
    )
    if seed_iso_path is not None:
        text += (
            f"[[disk]]\n"
            f'path = "{seed_iso_path}"\n'
            f'format = "raw"\n'
            f'interface = "cdrom"\n'
        )
    text += (
        "[[net]]\n"
        'bridge = "vmbr0"\n'
        'model = "virtio-net"\n'
        "[console]\n"
        "serial = true\n"
        "vnc = true\n"
    )
    return text


def write_linux_def(
    chroot_mount: str,
    *,
    memory_mib: int = DEFAULT_MEMORY_MIB,
    vcpus: int = DEFAULT_VCPUS,
    seed_iso_path: str | None = None,
) -> None:
    """Write /etc/phermes/vms/linux.toml inside the chroot.

    If ``seed_iso_path`` is set, the def references it as a CDROM [[disk]].
    """
    vms_dir = os.path.join(chroot_mount, "etc/phermes/vms")
    os.makedirs(vms_dir, exist_ok=True)
    def_path = os.path.join(vms_dir, "linux.toml")
    with open(def_path, "w") as f:
        f.write(_linux_def_text(
            memory_mib=memory_mib,
            vcpus=vcpus,
            seed_iso_path=seed_iso_path,
        ))


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
