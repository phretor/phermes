from phermes_build.models import DiskLayout
from phermes_build.runner import run_cmd


def partition_path(disk: str, num: int) -> str:
    # The kernel inserts a 'p' separator when the device name ends in a digit
    # (nvme0n1p1, mmcblk0p1, loop0p1) to disambiguate from the partition number.
    sep = "p" if disk[-1].isdigit() else ""
    return f"{disk}{sep}{num}"


def _build_sfdisk_script(layout: DiskLayout) -> str:
    luks_gb = layout.swap_gb + layout.lvm_gb + layout.data_gb
    if layout.share_encrypted and layout.share_gb > 0:
        luks_gb += layout.share_gb

    lines = [
        "label: gpt",
        "unit: sectors",
        "",
        f",{layout.efi_mb}M,U,*",
        f",{layout.boot_mb}M,L",
        f",{luks_gb}G,L",
    ]

    if layout.share_gb > 0 and not layout.share_encrypted:
        lines.append(f",{layout.share_gb}G,L")

    return "\n".join(lines) + "\n"


def create_partition_table(layout: DiskLayout) -> None:
    script = _build_sfdisk_script(layout)
    run_cmd(["sfdisk", "--force", layout.disk], input=script)
    # Force kernel to re-read partition table. Works in containers where udevd is absent.
    run_cmd(["blockdev", "--rereadpt", layout.disk], check=False)
    # partprobe waits for kernel acknowledgment; udevadm settle is best-effort (needs udevd)
    run_cmd(["partprobe", layout.disk], check=False)
    run_cmd(["udevadm", "settle"], check=False)
