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


def test_constants_for_seed_paths_exposed():
    assert vm_mod.SEED_DIR == "/var/lib/phermes/seed"
    assert vm_mod.LINUX_SEED_PATH == "/var/lib/phermes/seed/linux.iso"


def test_write_linux_def_omits_seed_cdrom_when_seed_iso_path_is_none(tmp_path):
    chroot = str(tmp_path / "chroot")
    import os as _os
    _os.makedirs(chroot)
    vm_mod.write_linux_def(chroot)  # default: no seed
    content = (tmp_path / "chroot" / "etc/phermes/vms/linux.toml").read_text()
    # The OS disk is present
    assert 'path = "/dev/pve/vm-102-disk-0"' in content
    # No CDROM
    assert "cdrom" not in content
    assert "seed" not in content


def test_write_linux_def_emits_seed_cdrom_when_seed_iso_path_given(tmp_path):
    chroot = str(tmp_path / "chroot")
    import os as _os
    _os.makedirs(chroot)
    vm_mod.write_linux_def(chroot, seed_iso_path="/var/lib/phermes/seed/linux.iso")
    content = (tmp_path / "chroot" / "etc/phermes/vms/linux.toml").read_text()
    # Both disks present
    assert 'path = "/dev/pve/vm-102-disk-0"' in content
    assert 'path = "/var/lib/phermes/seed/linux.iso"' in content
    assert 'format = "raw"' in content
    assert 'interface = "cdrom"' in content
    # OS disk still virtio-scsi
    assert 'interface = "virtio-scsi"' in content


def test_windows_constants():
    assert vm_mod.WINDOWS_VMID == 101
    assert vm_mod.WINDOWS_DEFAULT_DISK_GB == 100
    assert vm_mod.WINDOWS_DEFAULT_MEMORY_MIB == 8192
    assert vm_mod.WINDOWS_DEFAULT_VCPUS == 4


def test_write_windows_def_emits_expected_toml(tmp_path):
    import os as _os
    chroot = str(tmp_path / "chroot")
    _os.makedirs(chroot)
    vm_mod.write_windows_def(chroot)

    toml_path = tmp_path / "chroot" / "etc/phermes/vms/windows.toml"
    assert toml_path.exists()
    content = toml_path.read_text()
    assert 'flavor = "windows"' in content
    assert "memory_mib = 8192" in content
    assert "vcpus = 4" in content
    assert 'cpu = "host"' in content
    assert 'ovmf_code = "/usr/share/OVMF/OVMF_CODE.fd"' in content
    assert 'ovmf_vars_template = "/usr/share/OVMF/OVMF_VARS.fd"' in content
    assert 'path = "/dev/pve/vm-101-disk-0"' in content
    assert 'format = "raw"' in content
    assert 'interface = "virtio-scsi"' in content
    assert 'bridge = "vmbr0"' in content
    assert 'model = "virtio-net"' in content
    assert "serial = true" in content
    assert "vnc = true" in content
    # Slice #5a: no cloud-init seed for Windows — windows.toml has exactly ONE [[disk]] block.
    assert content.count("[[disk]]") == 1


def test_write_windows_def_honors_override_resources(tmp_path):
    import os as _os
    chroot = str(tmp_path / "chroot")
    _os.makedirs(chroot)
    vm_mod.write_windows_def(chroot, memory_mib=16384, vcpus=8)
    content = (tmp_path / "chroot" / "etc/phermes/vms/windows.toml").read_text()
    assert "memory_mib = 16384" in content
    assert "vcpus = 8" in content


def test_provision_windows_disk_creates_thin_lv_and_tags_it(monkeypatch):
    calls = _capture(monkeypatch)
    vm_mod.provision_windows_disk()
    assert calls[0][0] == "lvcreate"
    assert "--thin" in calls[0]
    assert "--virtualsize" in calls[0]
    assert "100G" in calls[0]
    assert "pve/data" in calls[0]
    assert "vm-101-disk-0" in calls[0]
    assert calls[1] == ["lvchange", "--addtag", "phermesd", "/dev/pve/vm-101-disk-0"]
    assert not any(c[0] == "qemu-img" for c in calls)


def test_provision_windows_disk_with_source_runs_qemu_img_convert(monkeypatch):
    calls = _capture(monkeypatch)
    vm_mod.provision_windows_disk(source="/tmp/windows.qcow2")
    qemu_calls = [c for c in calls if c[0] == "qemu-img"]
    assert len(qemu_calls) == 1
    assert qemu_calls[0] == [
        "qemu-img",
        "convert",
        "-O",
        "raw",
        "/tmp/windows.qcow2",
        "/dev/pve/vm-101-disk-0",
    ]


def test_provision_windows_disk_custom_size(monkeypatch):
    calls = _capture(monkeypatch)
    vm_mod.provision_windows_disk(size_gb=200)
    assert "200G" in calls[0]
