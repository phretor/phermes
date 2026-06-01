import json

import pytest

from phermes_build import vm as vm_mod
from phermes_build.models import AcquisitionMode, VMConfig, VMFlavor


def _capture(monkeypatch):
    calls = []
    monkeypatch.setattr(vm_mod, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")
    return calls


def test_import_vm_calls_qm_importdisk(monkeypatch):
    calls = _capture(monkeypatch)
    cfg = VMConfig(
        flavor=VMFlavor.MACOS,
        mode=AcquisitionMode.IMPORT,
        image_path="/mnt/share/macos.qcow2",
    )
    vm_mod.import_vm(cfg, vm_id=100, storage="local-lvm")
    assert any("qm" in c[0] or "importdisk" in str(c) for c in calls)


def test_schedule_writes_flag(tmp_path):
    cfg = VMConfig(flavor=VMFlavor.WINDOWS, mode=AcquisitionMode.DOWNLOAD)
    vm_mod.schedule_vm_acquisition(cfg, flag_dir=str(tmp_path))
    flag = tmp_path / "acquire_windows.json"
    assert flag.exists()
    data = json.loads(flag.read_text())
    assert data["flavor"] == "windows"
    assert data["mode"] == "download"


def test_vm_id_for_flavor():
    assert vm_mod.vm_id_for_flavor(VMFlavor.MACOS) == 100
    assert vm_mod.vm_id_for_flavor(VMFlavor.WINDOWS) == 101
    assert vm_mod.vm_id_for_flavor(VMFlavor.LINUX) == 102


def test_proxmox_vm_config_macos():
    conf = vm_mod.proxmox_vm_config(VMFlavor.MACOS, vm_id=100, disk_gb=120)
    assert "q35" in conf
    assert "100" in conf


def test_proxmox_vm_config_windows():
    conf = vm_mod.proxmox_vm_config(VMFlavor.WINDOWS, vm_id=101, disk_gb=100)
    assert "q35" in conf


def test_proxmox_vm_config_macos_specific():
    conf = vm_mod.proxmox_vm_config(VMFlavor.MACOS, vm_id=100, disk_gb=120)
    assert "vmxnet3" in conf
    assert "Penryn" in conf


def test_proxmox_vm_config_non_macos_virtio():
    conf = vm_mod.proxmox_vm_config(VMFlavor.WINDOWS, vm_id=101, disk_gb=100)
    assert "virtio" in conf


def test_import_vm_raises_for_non_import_mode(monkeypatch):
    _capture(monkeypatch)
    cfg = VMConfig(flavor=VMFlavor.MACOS, mode=AcquisitionMode.SKIP)
    with pytest.raises(ValueError, match="mode=IMPORT"):
        vm_mod.import_vm(cfg, vm_id=100)
