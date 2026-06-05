"""Minimal Debian host installer (replaces proxmox.py).

The plumbing (debootstrap, chroot bind-mounts, policy-rcd, GRUB install,
fstab/crypttab/grub-defaults emitters, identity/root-password helpers) lives
here directly; proxmox.py is deleted.
"""

import contextlib
import os
import shutil

from phermes_build.runner import run_cmd

# Source for phermesd/phermesctl binaries inside the phermes-build image.
# Populated by the Rust builder stage in Dockerfile.
PHERMESD_BIN_SRC = "/app/bin"

DEBIAN_RELEASE = "bookworm"

# Temporary root password set at build time so the console is loginable for
# setup and recovery. The first-boot wizard replaces it.
TEMP_ROOT_PASSWORD = "phermes-change-me"

# Bind-mounts required for chroot apt-get and postinstall scripts.
_CHROOT_BIND_MOUNTS = ["proc", "sys", "dev", "dev/pts"]

DEFAULT_HOSTNAME = "phermes"

# Provisional static network matching QEMU's user-mode (slirp) defaults.
SMOKE_ADDRESS = "10.0.2.15"
SMOKE_CIDR = "10.0.2.15/24"
SMOKE_GATEWAY = "10.0.2.2"
SMOKE_DNS = "10.0.2.3"

# Internal bridge for guest VMs, NATed out the uplink so guests reach the internet.
VMBR0_CIDR = "10.10.10.1/24"
VMBR0_NET = "10.10.10.0/24"

# ---------------------------------------------------------------------------
# Chroot helpers
# ---------------------------------------------------------------------------


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

# ---------------------------------------------------------------------------
# Partition / boot helpers
# ---------------------------------------------------------------------------


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


def crypttab_entry(luks_device: str, luks_name: str) -> str:
    return f"{luks_name}\t{luks_device}\tnone\tluks,discard\n"


def fstab_content() -> str:
    """fstab for the installed system. Boot partitions referenced by label."""
    return (
        "/dev/pve/root  /          ext4  errors=remount-ro  0  1\n"
        "LABEL=boot     /boot      ext4  defaults           0  2\n"
        "LABEL=PHERMESEFI /boot/efi vfat defaults           0  2\n"
    )


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


# ---------------------------------------------------------------------------
# GRUB helpers
# ---------------------------------------------------------------------------


def grub_defaults_content() -> str:
    return (
        'GRUB_DEFAULT=0\n'
        'GRUB_TIMEOUT=5\n'
        'GRUB_DISTRIBUTOR="PHermes"\n'
        # Verbose boot (no "quiet") so the serial console shows the full boot —
        # appropriate for a headless appliance where serial is the recovery path.
        'GRUB_CMDLINE_LINUX_DEFAULT=""\n'
        # Dual console: serial (ttyS0) for logging/recovery, tty0 last so the
        # local display/keyboard (and the VNC console) own /dev/console — that
        # is where the interactive LUKS prompt appears. Boot logs still mirror
        # to serial; headless unlock uses Dropbear over SSH.
        # net.ifnames=0 forces the primary NIC to eth0 so the network config is
        # predictable across hardware (and QEMU).
        'GRUB_CMDLINE_LINUX="console=ttyS0,115200 console=tty0 net.ifnames=0"\n'
        'GRUB_ENABLE_CRYPTODISK=y\n'
        # Prevent os-prober from scanning host partitions inside a container
        'GRUB_DISABLE_OS_PROBER=true\n'
        # Show the GRUB menu on both the display and the serial line
        'GRUB_TERMINAL="console serial"\n'
        'GRUB_SERIAL_COMMAND="serial --unit=0 --speed=115200"\n'
    )


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


# ---------------------------------------------------------------------------
# Identity / password helpers
# ---------------------------------------------------------------------------


def etc_hosts_content(hostname: str, address: str = SMOKE_ADDRESS) -> str:
    return (
        "127.0.0.1 localhost\n"
        f"{address} {hostname}.local {hostname}\n"
        "::1 localhost ip6-localhost ip6-loopback\n"
    )


def write_host_identity(mount_point: str, hostname: str = DEFAULT_HOSTNAME) -> None:
    """Write /etc/hostname and /etc/hosts so the node name resolves — required
    for pmxcfs (and thus qm/pveproxy) to start. The wizard can change it later."""
    with open(os.path.join(mount_point, "etc/hostname"), "w") as f:
        f.write(hostname + "\n")
    with open(os.path.join(mount_point, "etc/hosts"), "w") as f:
        f.write(etc_hosts_content(hostname))


def set_root_password(mount_point: str, password: str) -> None:
    """Set the chroot's root password via chpasswd (dev only — wizard replaces it)."""
    run_cmd(["chroot", mount_point, "chpasswd"], input=f"root:{password}\n")


def lock_root_account(mount_point: str) -> None:
    """Lock root so there is no console login. The production default — admin
    happens through the PHermes UI / restricted Proxmox RBAC, never a shipped
    password."""
    run_cmd(["chroot", mount_point, "passwd", "--lock", "root"])


def enable_dev_root_ssh(mount_point: str, pubkey: str) -> None:
    """DEV ONLY: allow root SSH (key + password) and install an authorized key.

    Never called for production builds — there root is locked and the default
    sshd config (prohibit-password) applies, so root SSH is impossible."""
    ssh_dir = os.path.join(mount_point, "root/.ssh")
    os.makedirs(ssh_dir, exist_ok=True)
    os.chmod(ssh_dir, 0o700)
    auth = os.path.join(ssh_dir, "authorized_keys")
    with open(auth, "w") as f:
        f.write(pubkey.rstrip("\n") + "\n")
    os.chmod(auth, 0o600)

    dropin_dir = os.path.join(mount_point, "etc/ssh/sshd_config.d")
    os.makedirs(dropin_dir, exist_ok=True)
    with open(os.path.join(dropin_dir, "phermes-dev.conf"), "w") as f:
        f.write("PermitRootLogin yes\n")


# ---------------------------------------------------------------------------
# Host-specific content generators
# ---------------------------------------------------------------------------


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
