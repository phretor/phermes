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
