import os
import stat

from phermes_build import host as host_mod


def test_debian_apt_sources_has_bookworm_main_updates_security_no_proxmox():
    sources = host_mod.debian_apt_sources()
    assert "deb http://deb.debian.org/debian bookworm main" in sources
    assert "deb http://deb.debian.org/debian bookworm-updates main" in sources
    assert "deb http://security.debian.org/debian-security bookworm-security main" in sources
    # Proxmox repo MUST NOT appear
    assert "proxmox" not in sources.lower()
    assert "pve-no-subscription" not in sources


def test_write_network_interfaces_eth0_is_dhcp_no_masquerade(tmp_path):
    chroot = str(tmp_path / "chroot")
    os.makedirs(chroot)
    host_mod.write_network_interfaces(chroot)
    content = (tmp_path / "chroot" / "etc/network/interfaces").read_text()
    # eth0 is DHCP (no static-IP-for-pmxcfs hack)
    assert "iface eth0 inet dhcp" in content
    # vmbr0 keeps its static address (NAT moved to nftables — verify no iptables here)
    assert "address 10.10.10.1/24" in content
    assert "bridge-ports none" in content
    assert "sysctl -w net.ipv4.ip_forward=1" in content
    # No iptables anywhere
    assert "iptables" not in content
    assert "MASQUERADE" not in content


def test_install_phermesd_binaries_copies_and_chmods(tmp_path, monkeypatch):
    # Fake the /app/bin source location.
    fake_bin = tmp_path / "app_bin"
    fake_bin.mkdir()
    (fake_bin / "phermesd").write_bytes(b"\x7fELF-phermesd")
    (fake_bin / "phermesctl").write_bytes(b"\x7fELF-phermesctl")
    monkeypatch.setattr(host_mod, "PHERMESD_BIN_SRC", str(fake_bin))

    chroot = str(tmp_path / "chroot")
    os.makedirs(chroot)
    host_mod.install_phermesd_binaries(chroot)

    for name in ("phermesd", "phermesctl"):
        dst = tmp_path / "chroot" / "usr/local/sbin" / name
        assert dst.exists()
        assert dst.read_bytes() == (fake_bin / name).read_bytes()
        # 0o755
        mode = stat.S_IMODE(dst.stat().st_mode)
        assert mode == 0o755


def test_install_phermesd_binaries_fails_fast_when_source_missing(tmp_path, monkeypatch):
    # Source dir does not contain phermesd
    fake_bin = tmp_path / "empty"
    fake_bin.mkdir()
    monkeypatch.setattr(host_mod, "PHERMESD_BIN_SRC", str(fake_bin))
    chroot = str(tmp_path / "chroot")
    os.makedirs(chroot)
    try:
        host_mod.install_phermesd_binaries(chroot)
    except FileNotFoundError as e:
        # Error message must mention the missing source and the fix.
        msg = str(e)
        assert "phermesd" in msg
        assert "rebuild" in msg.lower() or "image" in msg.lower()
    else:
        raise AssertionError("expected FileNotFoundError")


def test_install_minimal_host_calls_helpers_in_correct_order(monkeypatch, tmp_path):
    """install_minimal_host orchestrates a fixed sequence of OS-build helpers.

    We mock every helper to record the call order and assert the spec'd sequence.
    """
    calls: list[str] = []

    def rec(name):
        def _impl(*args, **kw):
            calls.append(name)
            return ""
        return _impl

    # Mock every reusable helper (currently re-exported from proxmox via host).
    for helper in [
        "format_boot_partitions",
        "run_debootstrap",
        "mount_boot",
        "write_host_identity",
        "write_network_interfaces",
        "_bind_chroot",
        "_setup_policy_rcd",
        "chroot_apt_install",
        "install_phermesd_binaries",
        "install_grub",
        "_teardown_policy_rcd",
        "_unbind_chroot",
        "unmount_boot",
    ]:
        monkeypatch.setattr(host_mod, helper, rec(helper))

    # systemd_units.install_phermesd_unit is the other newcomer.
    from phermes_build import systemd_units as sd_mod
    monkeypatch.setattr(sd_mod, "install_phermesd_unit", rec("install_phermesd_unit"))

    # Capture run_cmd at module level (used for chroot apt-get update + update-initramfs).
    monkeypatch.setattr(host_mod, "run_cmd", rec("run_cmd"))

    mount = str(tmp_path / "chroot")
    os.makedirs(mount)
    host_mod.install_minimal_host(
        mount_point=mount,
        disk="/dev/loop0",
        luks_device="/dev/loop0p3",
        efi_device="/dev/loop0p1",
        boot_device="/dev/loop0p2",
    )

    # The spec'd order: format boot -> debootstrap -> mount boot -> identity/network
    #   -> bind chroot -> policy-rcd -> apt update -> apt install -> phermesd binaries
    #   -> phermesd unit -> grub -> initramfs -> teardown
    expected_prefix = [
        "format_boot_partitions",
        "run_debootstrap",
        "mount_boot",
        "write_host_identity",
        "write_network_interfaces",
        "_bind_chroot",
        "_setup_policy_rcd",
    ]
    assert calls[: len(expected_prefix)] == expected_prefix

    # install_phermesd_binaries + install_phermesd_unit happen AFTER apt install
    apt_install_idx = calls.index("chroot_apt_install")
    bin_idx = calls.index("install_phermesd_binaries")
    unit_idx = calls.index("install_phermesd_unit")
    grub_idx = calls.index("install_grub")
    assert apt_install_idx < bin_idx < unit_idx < grub_idx

    # Teardown comes last and in reverse-of-setup order
    assert calls[-3:] == ["_teardown_policy_rcd", "_unbind_chroot", "unmount_boot"]


def test_install_minimal_host_writes_apt_sources_crypttab_fstab_grub(
    monkeypatch, tmp_path
):
    """All four config files land in the chroot with expected content snippets."""
    # No-op every helper
    for helper in [
        "format_boot_partitions",
        "run_debootstrap",
        "mount_boot",
        "write_host_identity",
        "write_network_interfaces",
        "_bind_chroot",
        "_setup_policy_rcd",
        "chroot_apt_install",
        "install_phermesd_binaries",
        "install_grub",
        "_teardown_policy_rcd",
        "_unbind_chroot",
        "unmount_boot",
    ]:
        monkeypatch.setattr(host_mod, helper, lambda *a, **k: "")
    monkeypatch.setattr(host_mod, "run_cmd", lambda *a, **k: "")
    from phermes_build import systemd_units as sd_mod
    monkeypatch.setattr(sd_mod, "install_phermesd_unit", lambda *a, **k: None)

    mount = str(tmp_path / "chroot")
    os.makedirs(mount)
    host_mod.install_minimal_host(
        mount_point=mount,
        disk="/dev/loop0",
        luks_device="/dev/loop0p3",
        efi_device="/dev/loop0p1",
        boot_device="/dev/loop0p2",
    )

    sources = (tmp_path / "chroot" / "etc/apt/sources.list").read_text()
    assert "bookworm main" in sources
    assert "proxmox" not in sources.lower()

    crypttab = (tmp_path / "chroot" / "etc/crypttab").read_text()
    assert "phermes_luks" in crypttab
    assert "/dev/loop0p3" in crypttab

    fstab = (tmp_path / "chroot" / "etc/fstab").read_text()
    assert "/dev/pve/root" in fstab
    assert "LABEL=boot" in fstab

    grub_defaults = (tmp_path / "chroot" / "etc/default/grub").read_text()
    assert "GRUB_ENABLE_CRYPTODISK=y" in grub_defaults


def test_phermes_apt_packages_includes_qemu_lvm_nftables_no_iptables():
    pkgs = host_mod.phermes_apt_packages()
    # Hypervisor + storage
    assert "qemu-system-x86" in pkgs
    assert "qemu-utils" in pkgs
    assert "ovmf" in pkgs
    assert "lvm2" in pkgs
    assert "btrfs-progs" in pkgs
    # Boot / unlock
    assert "cryptsetup-initramfs" in pkgs
    assert "dropbear-initramfs" in pkgs
    assert "grub-efi-amd64" in pkgs
    # Management + DHCP
    assert "openssh-server" in pkgs
    assert "isc-dhcp-client" in pkgs
    # Firewall + NAT (nftables only; iptables MUST NOT appear)
    assert "nftables" in pkgs
    assert "iptables" not in pkgs
    # Share + mDNS
    assert "samba" in pkgs
    assert "avahi-daemon" in pkgs
