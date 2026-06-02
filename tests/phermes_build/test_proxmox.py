from unittest.mock import patch

from phermes_build import proxmox as prox_mod


def _capture(monkeypatch):
    calls = []
    monkeypatch.setattr(prox_mod, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")
    return calls


def test_format_root_lv(monkeypatch):
    calls = _capture(monkeypatch)
    prox_mod.format_root_lv("/dev/pve/root")
    assert any("mkfs.ext4" in c[0] for c in calls)


def test_debootstrap_called(monkeypatch):
    calls = _capture(monkeypatch)
    prox_mod.run_debootstrap("/mnt/pve-root")
    assert any("debootstrap" in c[0] for c in calls)
    assert any("bookworm" in c for c in calls)


def test_proxmox_apt_sources_content():
    content = prox_mod.proxmox_apt_sources()
    assert "pve-no-subscription" in content
    assert "download.proxmox.com" in content
    assert "bookworm" in content


def test_install_grub_uses_force(monkeypatch):
    calls = _capture(monkeypatch)
    prox_mod.install_grub("/mnt/pve-root", "/dev/sdb")
    assert any("grub-install" in str(c) for c in calls)
    assert any("--force" in c for c in calls)


def test_crypttab_content():
    content = prox_mod.crypttab_entry(
        luks_device="/dev/sdb3",
        luks_name="phermes_luks",
    )
    assert "phermes_luks" in content
    assert "/dev/sdb3" in content
    assert "luks" in content


def test_grub_defaults_content():
    content = prox_mod.grub_defaults_content()
    assert "GRUB_ENABLE_CRYPTODISK=y" in content


def test_chroot_apt_install(monkeypatch):
    calls = _capture(monkeypatch)
    prox_mod.chroot_apt_install("/mnt/pve-root", "proxmox-ve", "postfix")
    # env prefix wraps the command — search across the full command list
    assert any("chroot" in str(c) for c in calls)
    assert any("apt-get" in str(c) for c in calls)
    assert any("proxmox-ve" in str(c) for c in calls)


def test_chroot_apt_install_sets_debian_frontend(monkeypatch):
    calls = _capture(monkeypatch)
    prox_mod.chroot_apt_install("/mnt/pve-root", "proxmox-ve")
    assert any("DEBIAN_FRONTEND=noninteractive" in str(c) for c in calls)


def test_bind_chroot_mounts_all_dirs(monkeypatch, tmp_path):
    calls = _capture(monkeypatch)
    with patch("os.makedirs"):
        prox_mod._bind_chroot(str(tmp_path))
    mount_calls = [c for c in calls if c[0] == "mount"]
    mounted = [c[-1] for c in mount_calls]
    assert any("proc" in m for m in mounted)
    assert any("sys" in m for m in mounted)
    assert any("/dev" in m for m in mounted)


def test_unbind_chroot_unmounts_all_dirs(monkeypatch, tmp_path):
    calls = _capture(monkeypatch)
    prox_mod._unbind_chroot(str(tmp_path))
    umount_calls = [c for c in calls if "umount" in c[0]]
    assert len(umount_calls) == len(prox_mod._CHROOT_BIND_MOUNTS)
