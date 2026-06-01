from phermes_build.runner import CommandError, run_cmd


def is_exfat_available() -> bool:
    try:
        run_cmd(["which", "mkfs.exfat"])
        return True
    except CommandError:
        return False


def format_exfat(device: str, label: str = "PHERMES_SHARE") -> None:
    if not is_exfat_available():
        raise RuntimeError("mkfs.exfat not found; install exfatprogs")
    run_cmd(["mkfs.exfat", "-n", label, device])
