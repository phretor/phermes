"""Minimal Debian host installer (replaces proxmox.py).

The unchanged-from-Proxmox plumbing (debootstrap, chroot bind-mounts, policy-rcd,
GRUB install, fstab/crypttab/grub-defaults emitters, identity/root-password
helpers) is imported from proxmox.py for the duration of this slice; Task H9
deletes proxmox.py and moves those definitions in here.
"""

import os
import shutil

# Re-exported for now from proxmox.py — moved into this module in Task H9.
from phermes_build.proxmox import (  # noqa: F401  (used by cli.py)
    TEMP_ROOT_PASSWORD,
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
from phermes_build.runner import run_cmd

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

    format_boot_partitions(efi_device, boot_device)
    run_debootstrap(mount_point)
    mount_boot(mount_point, efi_device, boot_device)
    write_host_identity(mount_point)
    write_network_interfaces(mount_point)

    sources_path = os.path.join(mount_point, "etc/apt/sources.list")
    os.makedirs(os.path.dirname(sources_path), exist_ok=True)
    with open(sources_path, "w") as f:
        f.write(debian_apt_sources())

    crypttab_path = os.path.join(mount_point, "etc/crypttab")
    os.makedirs(os.path.dirname(crypttab_path), exist_ok=True)
    with open(crypttab_path, "w") as f:
        f.write(crypttab_entry(luks_device, "phermes_luks"))

    fstab_path = os.path.join(mount_point, "etc/fstab")
    os.makedirs(os.path.dirname(fstab_path), exist_ok=True)
    with open(fstab_path, "w") as f:
        f.write(fstab_content())

    grub_path = os.path.join(mount_point, "etc/default/grub")
    os.makedirs(os.path.dirname(grub_path), exist_ok=True)
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
