from phermes_build import btrfs as btrfs_mod


def _capture(monkeypatch):
    calls = []
    monkeypatch.setattr(btrfs_mod, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")
    return calls


def test_format_btrfs(monkeypatch):
    calls = _capture(monkeypatch)
    btrfs_mod.format_btrfs("/dev/mapper/phermes_data_part")
    assert any("mkfs.btrfs" in c[0] for c in calls)
    assert any("PHERMES_DATA" in str(c) for c in calls)


def test_create_subvolumes_creates_all_three(monkeypatch):
    calls = _capture(monkeypatch)
    # Also patch os.makedirs to avoid actual filesystem calls
    monkeypatch.setattr(btrfs_mod.os, "makedirs", lambda *a, **kw: None)
    btrfs_mod.create_subvolumes("/mnt/data")
    subvol_calls = [c for c in calls if "subvolume" in c and "create" in c]
    names = [c[-1] for c in subvol_calls]
    assert any("@overlay" in n for n in names)
    assert any("@phermes" in n for n in names)
    assert any("@snapshots" in n for n in names)


def test_snapshot_overlay(monkeypatch):
    calls = _capture(monkeypatch)
    btrfs_mod.snapshot_overlay("/mnt/data", "2026-05-31T12-00-00")
    snap_calls = [c for c in calls if "snapshot" in c]
    assert snap_calls
    assert any("2026-05-31T12-00-00" in str(c) for c in snap_calls)


def test_mount_btrfs(monkeypatch):
    calls = _capture(monkeypatch)
    monkeypatch.setattr(btrfs_mod.os, "makedirs", lambda *a, **kw: None)
    btrfs_mod.mount_btrfs("/dev/sdb4", "/mnt/data")
    assert any("mount" in c[0] for c in calls)
    assert any("btrfs" in str(c) for c in calls)


def test_unmount(monkeypatch):
    calls = _capture(monkeypatch)
    btrfs_mod.unmount("/mnt/data")
    assert any("umount" in c[0] for c in calls)
    assert any("/mnt/data" in c for c in calls)
