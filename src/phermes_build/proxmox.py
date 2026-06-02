import os

from phermes_build.runner import run_cmd

DEBIAN_RELEASE = "bookworm"
PROXMOX_KEYRING_URL = (
    "https://enterprise.proxmox.com/debian/proxmox-release-bookworm.gpg"
)
PROXMOX_KEYRING_PATH = "/etc/apt/trusted.gpg.d/proxmox-release-bookworm.gpg"

# Bind-mounts required for chroot apt-get and postinstall scripts.
_CHROOT_BIND_MOUNTS = ["proc", "sys", "dev", "dev/pts"]


def format_root_lv(device: str) -> None:
    run_cmd(["mkfs.ext4", "-F", "-L", "pve-root", device])


def run_debootstrap(mount_point: str) -> None:
    run_cmd(
        [
            "debootstrap",
            "--arch", "amd64",
            DEBIAN_RELEASE,
            mount_point,
            "http://deb.debian.org/debian",
        ]
    )


def proxmox_apt_sources() -> str:
    return (
        f"deb http://deb.debian.org/debian {DEBIAN_RELEASE} main contrib\n"
        f"deb http://deb.debian.org/debian {DEBIAN_RELEASE}-updates main contrib\n"
        f"deb http://security.debian.org/debian-security "
        f"{DEBIAN_RELEASE}-security main contrib\n"
        f"deb http://download.proxmox.com/debian/pve {DEBIAN_RELEASE} pve-no-subscription\n"
    )


def crypttab_entry(luks_device: str, luks_name: str) -> str:
    return f"{luks_name}\t{luks_device}\tnone\tluks,discard\n"


def grub_defaults_content() -> str:
    return (
        'GRUB_DEFAULT=0\n'
        'GRUB_TIMEOUT=5\n'
        'GRUB_DISTRIBUTOR="PHermes"\n'
        'GRUB_CMDLINE_LINUX_DEFAULT="quiet"\n'
        'GRUB_CMDLINE_LINUX=""\n'
        'GRUB_ENABLE_CRYPTODISK=y\n'
    )


def _bind_chroot(mount_point: str) -> None:
    for sub in _CHROOT_BIND_MOUNTS:
        target = os.path.join(mount_point, sub)
        os.makedirs(target, exist_ok=True)
        run_cmd(["mount", "--bind", f"/{sub}", target])


def _unbind_chroot(mount_point: str) -> None:
    for sub in reversed(_CHROOT_BIND_MOUNTS):
        run_cmd(["umount", "-l", os.path.join(mount_point, sub)], check=False)


def install_grub(mount_point: str, disk: str) -> None:
    # --force required for loop devices and virtual disks
    run_cmd(["chroot", mount_point, "grub-install", "--force", disk])
    run_cmd(["chroot", mount_point, "update-grub"])


def chroot_apt_install(mount_point: str, *packages: str) -> None:
    # DEBIAN_FRONTEND=noninteractive suppresses postfix and other interactive prompts
    run_cmd(
        [
            "env", "DEBIAN_FRONTEND=noninteractive",
            "chroot", mount_point,
            "apt-get", "install", "-y", "--no-install-recommends",
            *packages,
        ]
    )


def fetch_proxmox_keyring(mount_point: str) -> None:
    dest = os.path.join(mount_point, PROXMOX_KEYRING_PATH.lstrip("/"))
    run_cmd(["wget", "-qO", dest, PROXMOX_KEYRING_URL])


def install_proxmox(mount_point: str, disk: str, luks_device: str) -> None:
    """Full Proxmox VE installation sequence into a mounted chroot."""
    run_debootstrap(mount_point)
    fetch_proxmox_keyring(mount_point)

    sources_path = os.path.join(mount_point, "etc/apt/sources.list")
    with open(sources_path, "w") as f:
        f.write(proxmox_apt_sources())

    run_cmd(["chroot", mount_point, "apt-get", "update"])

    # /proc /sys /dev /dev/pts must be mounted for apt postinstall scripts,
    # grub-install, and update-initramfs to work inside the chroot.
    _bind_chroot(mount_point)
    try:
        chroot_apt_install(
            mount_point,
            "proxmox-ve", "postfix", "open-iscsi",
            "cryptsetup-initramfs", "dropbear-initramfs",
            "grub-pc",
        )

        crypttab_path = os.path.join(mount_point, "etc/crypttab")
        with open(crypttab_path, "w") as f:
            f.write(crypttab_entry(luks_device, "phermes_luks"))

        grub_path = os.path.join(mount_point, "etc/default/grub")
        with open(grub_path, "w") as f:
            f.write(grub_defaults_content())

        install_grub(mount_point, disk)
        run_cmd(["chroot", mount_point, "update-initramfs", "-u", "-k", "all"])
    finally:
        _unbind_chroot(mount_point)
