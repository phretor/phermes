import os

from phermes_build.runner import run_cmd

SUBVOLUMES = ["@overlay", "@phermes", "@snapshots"]


def format_btrfs(device: str, label: str = "PHERMES_DATA") -> None:
    run_cmd(["mkfs.btrfs", "-L", label, "-f", device])


def mount_btrfs(device: str, mount_point: str) -> None:
    os.makedirs(mount_point, exist_ok=True)
    run_cmd(["mount", "-t", "btrfs", "-o", "compress=zstd", device, mount_point])


def create_subvolumes(mount_point: str) -> None:
    for name in SUBVOLUMES:
        run_cmd(["btrfs", "subvolume", "create", os.path.join(mount_point, name)])
    for subdir in ["hermes", "documents"]:
        os.makedirs(os.path.join(mount_point, "@overlay", subdir), exist_ok=True)


def snapshot_overlay(mount_point: str, timestamp: str) -> str:
    src = os.path.join(mount_point, "@overlay")
    dst = os.path.join(mount_point, "@snapshots", f"overlay-{timestamp}")
    run_cmd(["btrfs", "subvolume", "snapshot", "-r", src, dst])
    return dst


def unmount(mount_point: str) -> None:
    run_cmd(["umount", mount_point])
