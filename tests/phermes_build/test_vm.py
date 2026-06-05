"""Tests for the rewritten vm.py — Linux-only, phermesd TOML def + LVM-thin LV."""

import os

from phermes_build import vm as vm_mod


def _capture(monkeypatch):
    calls = []
    monkeypatch.setattr(vm_mod, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")
    return calls


def test_constants_mirror_phermesd_storage_conventions():
    # These MUST equal the values hardcoded in phermesd/src/storage.rs.
    # If you change one side, change the other (a CI grep is a future hardening pass).
    assert vm_mod.STORAGE_VG == "pve"
    assert vm_mod.STORAGE_POOL == "data"
    assert vm_mod.OWNER_TAG == "phermesd"
    assert vm_mod.LINUX_VMID == 102


def test_write_linux_def_emits_expected_toml(tmp_path):
    chroot = str(tmp_path / "chroot")
    os.makedirs(chroot)
    vm_mod.write_linux_def(chroot)

    toml_path = tmp_path / "chroot" / "etc/phermes/vms/linux.toml"
    assert toml_path.exists()
    content = toml_path.read_text()
    assert 'flavor = "linux"' in content
    assert "memory_mib = 4096" in content
    assert "vcpus = 4" in content
    assert 'cpu = "host"' in content
    assert 'ovmf_code = "/usr/share/OVMF/OVMF_CODE.fd"' in content
    assert 'ovmf_vars_template = "/usr/share/OVMF/OVMF_VARS.fd"' in content
    assert 'path = "/dev/pve/vm-102-disk-0"' in content
    assert 'format = "raw"' in content
    assert 'interface = "virtio-scsi"' in content
    assert 'bridge = "vmbr0"' in content
    assert 'model = "virtio-net"' in content
    assert "serial = true" in content
    assert "vnc = true" in content


def test_write_linux_def_honors_override_resources(tmp_path):
    chroot = str(tmp_path / "chroot")
    os.makedirs(chroot)
    vm_mod.write_linux_def(chroot, memory_mib=8192, vcpus=8)
    content = (tmp_path / "chroot" / "etc/phermes/vms/linux.toml").read_text()
    assert "memory_mib = 8192" in content
    assert "vcpus = 8" in content


def test_provision_linux_disk_creates_thin_lv_and_tags_it(monkeypatch):
    calls = _capture(monkeypatch)
    vm_mod.provision_linux_disk()
    # 1st call: lvcreate --thin --virtualsize <default>G pve/data -n vm-102-disk-0
    assert calls[0][0] == "lvcreate"
    assert "--thin" in calls[0]
    assert "--virtualsize" in calls[0]
    assert "40G" in calls[0]
    assert "pve/data" in calls[0]
    assert "vm-102-disk-0" in calls[0]
    # 2nd call: lvchange --addtag phermesd /dev/pve/vm-102-disk-0
    assert calls[1] == ["lvchange", "--addtag", "phermesd", "/dev/pve/vm-102-disk-0"]
    # No qemu-img invocation when source is None
    assert not any(c[0] == "qemu-img" for c in calls)


def test_provision_linux_disk_with_source_runs_qemu_img_convert(monkeypatch):
    calls = _capture(monkeypatch)
    vm_mod.provision_linux_disk(source="/tmp/source.qcow2")
    qemu_calls = [c for c in calls if c[0] == "qemu-img"]
    assert len(qemu_calls) == 1
    assert qemu_calls[0] == [
        "qemu-img",
        "convert",
        "-O",
        "raw",
        "/tmp/source.qcow2",
        "/dev/pve/vm-102-disk-0",
    ]


def test_provision_linux_disk_custom_size(monkeypatch):
    calls = _capture(monkeypatch)
    vm_mod.provision_linux_disk(size_gb=100)
    assert "100G" in calls[0]
