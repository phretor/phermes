import json

import pytest

from phermes_build import disk as disk_mod
from phermes_build.models import DiskLayout

LSBLK_OUTPUT = json.dumps(
    {
        "blockdevices": [
            {"name": "sda", "type": "disk", "size": "500G", "mountpoints": ["/"]},
            {"name": "sdb", "type": "disk", "size": "1T", "mountpoints": []},
            {"name": "sdc", "type": "disk", "size": "2T", "mountpoints": [None]},
        ]
    }
)


def test_list_disks_excludes_mounted(monkeypatch):
    monkeypatch.setattr(disk_mod, "run_cmd", lambda *a, **kw: LSBLK_OUTPUT)
    result = disk_mod.list_disks()
    assert "/dev/sda" not in result
    assert "/dev/sdb" in result
    assert "/dev/sdc" in result  # None mountpoint means unmounted


def test_disk_size_gb(monkeypatch):
    monkeypatch.setattr(disk_mod, "run_cmd", lambda *a, **kw: str(1024**3 * 1000))
    assert disk_mod.disk_size_gb("/dev/sdb") == 1000


def test_validate_disk_raises_if_too_small(monkeypatch):
    monkeypatch.setattr(disk_mod, "run_cmd", lambda *a, **kw: str(1024**3 * 100))
    with pytest.raises(ValueError, match="minimum is 500"):
        disk_mod.validate_disk("/dev/sdb")


def test_validate_disk_passes_for_large_enough(monkeypatch):
    monkeypatch.setattr(disk_mod, "run_cmd", lambda *a, **kw: str(1024**3 * 1000))
    disk_mod.validate_disk("/dev/sdb")  # should not raise


def test_compute_layout_1tb():
    layout = disk_mod._compute_layout_from_size("/dev/sdb", 1000, share_size_gb=250)
    assert isinstance(layout, DiskLayout)
    assert layout.disk == "/dev/sdb"
    assert layout.lvm_gb == 400
    assert layout.share_gb == 250
    assert layout.data_gb > 0
    total = layout.lvm_gb + layout.data_gb + layout.share_gb + layout.swap_gb
    assert total <= 1000


def test_compute_layout_no_share():
    layout = disk_mod._compute_layout_from_size("/dev/sdb", 1000, share_size_gb=0)
    assert layout.share_gb == 0


def test_compute_layout_minimum_disk():
    layout = disk_mod._compute_layout_from_size("/dev/sdb", 500, share_size_gb=0)
    assert layout.data_gb >= 50
