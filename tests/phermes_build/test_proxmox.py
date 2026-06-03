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


def test_install_grub_uefi_removable(monkeypatch):
    calls = _capture(monkeypatch)
    prox_mod.install_grub("/mnt/pve-root")
    grub_calls = [c for c in calls if "grub-install" in c]
    assert grub_calls
    grub = grub_calls[0]
    assert "--target=x86_64-efi" in grub
    assert "--removable" in grub
    assert "--efi-directory=/boot/efi" in grub


def test_install_grub_runs_update_grub(monkeypatch):
    calls = _capture(monkeypatch)
    prox_mod.install_grub("/mnt/pve-root")
    assert any("update-grub" in c for c in calls)


def test_format_boot_partitions(monkeypatch):
    calls = _capture(monkeypatch)
    prox_mod.format_boot_partitions("/dev/loop0p1", "/dev/loop0p2")
    assert any("mkfs.vfat" in c[0] for c in calls)
    assert any("mkfs.ext4" in c[0] for c in calls)
    assert any("/dev/loop0p1" in c for c in calls)
    assert any("/dev/loop0p2" in c for c in calls)


def test_mount_boot_order(monkeypatch):
    calls = _capture(monkeypatch)
    with patch("os.makedirs"):
        prox_mod.mount_boot("/mnt/pve-root", "/dev/loop0p1", "/dev/loop0p2")
    mounts = [c for c in calls if c[0] == "mount"]
    # /boot must be mounted before /boot/efi
    assert mounts[0][-1].endswith("/boot")
    assert mounts[1][-1].endswith("/boot/efi")


def test_fstab_content():
    content = prox_mod.fstab_content()
    assert "/dev/pve/root" in content
    assert "LABEL=boot" in content
    assert "/boot/efi" in content


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
    assert "GRUB_DISABLE_OS_PROBER=true" in content
    assert "console=ttyS0,115200" in content
    assert 'GRUB_TERMINAL="console serial"' in content


def test_install_grub_uses_no_nvram(monkeypatch):
    calls = _capture(monkeypatch)
    prox_mod.install_grub("/mnt/pve-root")
    assert any("--no-nvram" in c for c in calls)


def test_setup_policy_rcd_creates_file(tmp_path):
    prox_mod._setup_policy_rcd(str(tmp_path))
    policy = tmp_path / "usr" / "sbin" / "policy-rc.d"
    assert policy.exists()
    assert "exit 101" in policy.read_text()
    assert oct(policy.stat().st_mode)[-3:] == "755"


def test_teardown_policy_rcd_removes_file(tmp_path):
    prox_mod._setup_policy_rcd(str(tmp_path))
    prox_mod._teardown_policy_rcd(str(tmp_path))
    assert not (tmp_path / "usr" / "sbin" / "policy-rc.d").exists()


def test_teardown_policy_rcd_idempotent(tmp_path):
    prox_mod._teardown_policy_rcd(str(tmp_path))  # should not raise


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


def test_set_root_password(monkeypatch):
    received = {}

    def fake_run(cmd, *, input=None, check=True):  # noqa: A002
        received["cmd"] = cmd
        received["input"] = input
        return ""

    monkeypatch.setattr(prox_mod, "run_cmd", fake_run)
    prox_mod.set_root_password("/mnt/pve-root", "secret123")
    assert "chpasswd" in received["cmd"]
    assert "chroot" in received["cmd"]
    assert received["input"] == "root:secret123\n"


def test_lock_root_account(monkeypatch):
    calls = _capture(monkeypatch)
    prox_mod.lock_root_account("/mnt/pve-root")
    assert any("passwd" in c for c in calls)
    assert any("--lock" in c for c in calls)
    assert any("root" in c for c in calls)


def test_enable_dev_root_ssh(tmp_path):
    prox_mod.enable_dev_root_ssh(str(tmp_path), "ssh-ed25519 AAAAtest phermes-dev")
    auth = tmp_path / "root" / ".ssh" / "authorized_keys"
    assert auth.read_text().strip() == "ssh-ed25519 AAAAtest phermes-dev"
    assert oct(auth.stat().st_mode)[-3:] == "600"
    assert oct((tmp_path / "root" / ".ssh").stat().st_mode)[-3:] == "700"
    dropin = tmp_path / "etc" / "ssh" / "sshd_config.d" / "phermes-dev.conf"
    assert "PermitRootLogin yes" in dropin.read_text()


def test_etc_hosts_content_resolves_hostname():
    content = prox_mod.etc_hosts_content("phermes")
    assert "127.0.0.1 localhost" in content
    assert "phermes" in content


def test_write_host_identity(tmp_path):
    import os

    os.makedirs(tmp_path / "etc")
    prox_mod.write_host_identity(str(tmp_path), "phermes")
    assert (tmp_path / "etc" / "hostname").read_text().strip() == "phermes"
    assert "phermes" in (tmp_path / "etc" / "hosts").read_text()


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
