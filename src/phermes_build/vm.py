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
