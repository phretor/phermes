"""Integration tests — require root and loop device support.

Run with: sudo uv run pytest -m integration -v
"""
import subprocess

import pytest

from phermes_build.disk import disk_size_gb
from phermes_build.luks import close_luks, format_luks, open_luks
from phermes_build.models import DiskLayout
from phermes_build.partitioner import create_partition_table


@pytest.fixture(scope="module")
def loop_device(tmp_path_factory):
    """Create a 2 GB sparse image and attach it as a loop device."""
    img = tmp_path_factory.mktemp("disk") / "disk.img"
    subprocess.run(["truncate", "-s", "2G", str(img)], check=True)
    result = subprocess.run(
        ["losetup", "--find", "--show", str(img)],
        capture_output=True, text=True, check=True,
    )
    dev = result.stdout.strip()
    yield dev
    subprocess.run(["losetup", "-d", dev], check=True)


@pytest.mark.integration
def test_disk_size_detected(loop_device):
    size = disk_size_gb(loop_device)
    assert size == 2  # 2 GB


@pytest.mark.integration
def test_partition_table_created(loop_device):
    layout = DiskLayout(
        disk=loop_device,
        disk_size_gb=2,
        lvm_gb=1,
        data_gb=0,
        share_gb=0,
        swap_gb=0,
        efi_mb=100,
        boot_mb=100,
    )
    create_partition_table(layout)
    result = subprocess.run(
        ["sfdisk", "-l", loop_device], capture_output=True, text=True
    )
    assert "GPT" in result.stdout


@pytest.mark.integration
def test_luks_create_open_close(tmp_path):
    img = tmp_path / "luks.img"
    subprocess.run(["truncate", "-s", "256M", str(img)], check=True)
    result = subprocess.run(
        ["losetup", "--find", "--show", str(img)],
        capture_output=True, text=True, check=True,
    )
    luks_dev = result.stdout.strip()
    try:
        format_luks(luks_dev, "test-pass")
        mp = open_luks(luks_dev, "test_phermes_integ", "test-pass")
        assert mp == "/dev/mapper/test_phermes_integ"
        close_luks("test_phermes_integ")
    finally:
        subprocess.run(["losetup", "-d", luks_dev])
