import contextlib
import os

from phermes_build.runner import run_cmd

DEBIAN_RELEASE = "bookworm"
PROXMOX_KEYRING_URL = (
    "https://enterprise.proxmox.com/debian/proxmox-release-bookworm.gpg"
)
PROXMOX_KEYRING_PATH = "/etc/apt/trusted.gpg.d/proxmox-release-bookworm.gpg"

# Temporary root password set at build time so the console is loginable for
# setup and recovery. The first-boot wizard replaces it. Same convention as the
# temporary LUKS passphrase.
TEMP_ROOT_PASSWORD = "phermes-change-me"

# Bind-mounts required for chroot apt-get and postinstall scripts.
_CHROOT_BIND_MOUNTS = ["proc", "sys", "dev", "dev/pts"]


def set_root_password(mount_point: str, password: str) -> None:
    """Set the chroot's root password via chpasswd (dev only — wizard replaces it)."""
    run_cmd(["chroot", mount_point, "chpasswd"], input=f"root:{password}\n")


def lock_root_account(mount_point: str) -> None:
    """Lock root so there is no console login. The production default — admin
    happens through the PHermes UI / restricted Proxmox RBAC, never a shipped
    password."""
    run_cmd(["chroot", mount_point, "passwd", "--lock", "root"])


def format_root_lv(device: str) -> None:
    run_cmd(["mkfs.ext4", "-F", "-L", "pve-root", device])


def format_boot_partitions(efi_device: str, boot_device: str) -> None:
    """Format the EFI system partition (FAT32) and the plaintext /boot (ext4)."""
    run_cmd(["mkfs.vfat", "-F", "32", "-n", "PHERMESEFI", efi_device])
    run_cmd(["mkfs.ext4", "-F", "-L", "boot", boot_device])


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


def fstab_content() -> str:
    """fstab for the installed system. Boot partitions referenced by label."""
    return (
        "/dev/pve/root  /          ext4  errors=remount-ro  0  1\n"
        "LABEL=boot     /boot      ext4  defaults           0  2\n"
        "LABEL=PHERMESEFI /boot/efi vfat defaults           0  2\n"
    )


def grub_defaults_content() -> str:
    return (
        'GRUB_DEFAULT=0\n'
        'GRUB_TIMEOUT=5\n'
        'GRUB_DISTRIBUTOR="PHermes"\n'
        # Verbose boot (no "quiet") so the serial console shows the full boot —
        # appropriate for a headless appliance where serial is the recovery path.
        'GRUB_CMDLINE_LINUX_DEFAULT=""\n'
        # Dual console: tty0 for the VGA/VNC display, ttyS0 for the serial line.
        # The last console= owns /dev/console, so the initramfs LUKS prompt is
        # interactive over serial — needed for headless boot and recovery.
        'GRUB_CMDLINE_LINUX="console=tty0 console=ttyS0,115200"\n'
        'GRUB_ENABLE_CRYPTODISK=y\n'
        # Prevent os-prober from scanning host partitions inside a container
        'GRUB_DISABLE_OS_PROBER=true\n'
        # Show the GRUB menu on both the display and the serial line
        'GRUB_TERMINAL="console serial"\n'
        'GRUB_SERIAL_COMMAND="serial --unit=0 --speed=115200"\n'
    )


def _bind_chroot(mount_point: str) -> None:
    for sub in _CHROOT_BIND_MOUNTS:
        target = os.path.join(mount_point, sub)
        os.makedirs(target, exist_ok=True)
        run_cmd(["mount", "--bind", f"/{sub}", target])


def _unbind_chroot(mount_point: str) -> None:
    for sub in reversed(_CHROOT_BIND_MOUNTS):
        run_cmd(["umount", "-l", os.path.join(mount_point, sub)], check=False)


def _setup_policy_rcd(mount_point: str) -> None:
    """Block service starts during apt postinstall — no systemd in chroot."""
    path = os.path.join(mount_point, "usr/sbin/policy-rc.d")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write("#!/bin/sh\nexit 101\n")
    os.chmod(path, 0o755)


def _teardown_policy_rcd(mount_point: str) -> None:
    path = os.path.join(mount_point, "usr/sbin/policy-rc.d")
    with contextlib.suppress(FileNotFoundError):
        os.unlink(path)


def mount_boot(mount_point: str, efi_device: str, boot_device: str) -> None:
    """Mount /boot then /boot/efi inside the new root."""
    boot_mp = os.path.join(mount_point, "boot")
    os.makedirs(boot_mp, exist_ok=True)
    run_cmd(["mount", boot_device, boot_mp])
    efi_mp = os.path.join(boot_mp, "efi")
    os.makedirs(efi_mp, exist_ok=True)
    run_cmd(["mount", efi_device, efi_mp])


def unmount_boot(mount_point: str) -> None:
    run_cmd(["umount", "-l", os.path.join(mount_point, "boot/efi")], check=False)
    run_cmd(["umount", "-l", os.path.join(mount_point, "boot")], check=False)


def install_grub(mount_point: str) -> None:
    """Install GRUB for UEFI in removable mode (boots on any machine, no NVRAM).

    --removable writes to EFI/BOOT/BOOTX64.EFI so the SSD boots on any UEFI
    firmware without a pre-registered boot entry — exactly what a portable
    appliance needs. --no-nvram skips the efibootmgr call (no efivars in a
    container). --target is explicit because the container has no efivars for
    grub-install to auto-detect from.
    """
    run_cmd(
        [
            "chroot", mount_point, "grub-install",
            "--target=x86_64-efi",
            "--efi-directory=/boot/efi",
            "--boot-directory=/boot",
            "--removable",
            "--no-nvram",
        ]
    )
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


def install_proxmox(
    mount_point: str,
    disk: str,
    luks_device: str,
    efi_device: str,
    boot_device: str,
) -> None:
    """Full Proxmox VE installation sequence into a mounted chroot.

    The root LV is expected to be already mounted at `mount_point`. This formats
    the EFI/boot partitions, debootstraps Debian, mounts /boot and /boot/efi,
    installs Proxmox VE and a UEFI GRUB, and writes crypttab/fstab.
    """
    format_boot_partitions(efi_device, boot_device)
    run_debootstrap(mount_point)
    mount_boot(mount_point, efi_device, boot_device)

    fetch_proxmox_keyring(mount_point)

    sources_path = os.path.join(mount_point, "etc/apt/sources.list")
    with open(sources_path, "w") as f:
        f.write(proxmox_apt_sources())

    crypttab_path = os.path.join(mount_point, "etc/crypttab")
    with open(crypttab_path, "w") as f:
        f.write(crypttab_entry(luks_device, "phermes_luks"))

    fstab_path = os.path.join(mount_point, "etc/fstab")
    with open(fstab_path, "w") as f:
        f.write(fstab_content())

    grub_path = os.path.join(mount_point, "etc/default/grub")
    with open(grub_path, "w") as f:
        f.write(grub_defaults_content())

    # /proc /sys /dev /dev/pts must be mounted for apt postinstall scripts,
    # grub-install, and update-initramfs to work inside the chroot.
    _bind_chroot(mount_point)
    # policy-rc.d prevents service starts — no running systemd in chroot
    _setup_policy_rcd(mount_point)
    try:
        run_cmd(["chroot", mount_point, "apt-get", "update"])
        chroot_apt_install(
            mount_point,
            "proxmox-ve", "postfix", "open-iscsi",
            "cryptsetup-initramfs", "dropbear-initramfs",
            "grub-efi-amd64",
        )

        install_grub(mount_point)
        run_cmd(["chroot", mount_point, "update-initramfs", "-u", "-k", "all"])
    finally:
        _teardown_policy_rcd(mount_point)
        _unbind_chroot(mount_point)
        unmount_boot(mount_point)
