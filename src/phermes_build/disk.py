import json

from phermes_build.models import DiskLayout
from phermes_build.runner import run_cmd

MIN_DISK_GB = 500
LVM_GB = 400  # Proxmox OS (30 GB pve-root) + LVM-thin pool for VM images


def list_disks() -> list[str]:
    """List unmounted block devices."""
    out = run_cmd(
        [
            "lsblk",
            "--json",
            "--output",
            "NAME,TYPE,SIZE,MOUNTPOINTS",
        ]
    )
    devices = json.loads(out)["blockdevices"]
    return [
        f"/dev/{d['name']}"
        for d in devices
        if d["type"] == "disk" and not any(m for m in d.get("mountpoints", []) if m)
    ]


def disk_size_gb(disk: str) -> int:
    """Get disk size in GB."""
    out = run_cmd(["blockdev", "--getsize64", disk])
    return int(out) // (1024**3)


def validate_disk(disk: str, required_gb: int = MIN_DISK_GB) -> None:
    """Validate disk is large enough."""
    size = disk_size_gb(disk)
    if size < required_gb:
        raise ValueError(f"{disk} is {size} GB; minimum is {required_gb} GB")


def _compute_layout_from_size(
    disk: str,
    disk_size_gb: int,
    share_size_gb: int = 250,
    share_encrypted: bool = False,
) -> DiskLayout:
    """Compute partition layout from disk size."""
    fixed_overhead_gb = 1 + 16 + 1  # /boot + swap + EFI (rounded up)
    usable = disk_size_gb - fixed_overhead_gb - LVM_GB

    data_gb = (
        usable - share_size_gb
        if share_size_gb > 0 and not share_encrypted
        else usable
    )

    return DiskLayout(
        disk=disk,
        disk_size_gb=disk_size_gb,
        lvm_gb=LVM_GB,
        data_gb=max(data_gb, 50),
        share_gb=share_size_gb,
        share_encrypted=share_encrypted,
    )


def compute_layout(
    disk: str,
    share_size_gb: int = 250,
    share_encrypted: bool = False,
) -> DiskLayout:
    """Validate disk and compute partition layout."""
    validate_disk(disk)
    size = disk_size_gb(disk)
    return _compute_layout_from_size(disk, size, share_size_gb, share_encrypted)
