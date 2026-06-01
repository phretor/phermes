import pytest

from phermes_build.models import (
    AcquisitionMode,
    BuildConfig,
    DiskLayout,
    VMConfig,
    VMFlavor,
)


def test_vm_flavor_values():
    assert VMFlavor.MACOS == "macos"
    assert VMFlavor.WINDOWS == "windows"
    assert VMFlavor.LINUX == "linux"


def test_build_config_defaults():
    cfg = BuildConfig(disk="/dev/sdb")
    assert cfg.share_size_gb == 250
    assert cfg.share_encrypted is False
    assert cfg.vms == []
    assert cfg.temp_luks_passphrase == "phermes-change-me"


def test_build_config_rejects_non_dev_path():
    with pytest.raises(Exception, match="must be a /dev/ path"):
        BuildConfig(disk="sdb")


def test_disk_layout_fields():
    layout = DiskLayout(
        disk="/dev/sdb",
        disk_size_gb=1000,
        lvm_gb=400,
        data_gb=333,
        share_gb=250,
    )
    assert layout.efi_mb == 512
    assert layout.boot_mb == 1024
    assert layout.swap_gb == 16


def test_vm_config_import_mode_requires_path():
    with pytest.raises(Exception, match="image_path required"):
        VMConfig(flavor=VMFlavor.MACOS, mode=AcquisitionMode.IMPORT)


def test_vm_config_import_mode_with_path():
    cfg = VMConfig(
        flavor=VMFlavor.MACOS,
        mode=AcquisitionMode.IMPORT,
        image_path="/mnt/share/macos.qcow2",
    )
    assert cfg.image_path == "/mnt/share/macos.qcow2"
