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
