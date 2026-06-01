from phermes_build import partitioner as part_mod
from phermes_build.models import DiskLayout

LAYOUT_1TB = DiskLayout(
    disk="/dev/sdb",
    disk_size_gb=1000,
    lvm_gb=400,
    data_gb=333,
    share_gb=250,
)

LAYOUT_NO_SHARE = DiskLayout(
    disk="/dev/sdb",
    disk_size_gb=1000,
    lvm_gb=400,
    data_gb=583,
    share_gb=0,
)


def test_sfdisk_script_contains_efi_partition():
    script = part_mod._build_sfdisk_script(LAYOUT_1TB)
    assert "U" in script  # U = EFI partition type in sfdisk


def test_sfdisk_script_four_partitions_with_share():
    script = part_mod._build_sfdisk_script(LAYOUT_1TB)
    lines = [ln for ln in script.splitlines() if ln.startswith(",") or "size=" in ln]
    assert len(lines) == 4


def test_sfdisk_script_three_partitions_without_share():
    script = part_mod._build_sfdisk_script(LAYOUT_NO_SHARE)
    lines = [ln for ln in script.splitlines() if ln.startswith(",") or "size=" in ln]
    assert len(lines) == 3


def test_create_partition_table_calls_sfdisk(monkeypatch):
    calls = []
    monkeypatch.setattr(part_mod, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")
    part_mod.create_partition_table(LAYOUT_1TB)
    assert any(c[0] == "sfdisk" for c in calls)
    assert any("/dev/sdb" in c for c in calls)
    assert any("udevadm" in c[0] for c in calls)


def test_partition_path():
    assert part_mod.partition_path("/dev/sdb", 1) == "/dev/sdb1"
    assert part_mod.partition_path("/dev/nvme0n1", 1) == "/dev/nvme0n1p1"
