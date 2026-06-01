from phermes_build import lvm as lvm_mod


def _capture(monkeypatch):
    calls = []
    monkeypatch.setattr(lvm_mod, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")
    return calls


def test_create_pv(monkeypatch):
    calls = _capture(monkeypatch)
    lvm_mod.create_pv("/dev/mapper/phermes_luks")
    assert any("pvcreate" in c[0] for c in calls)


def test_create_vg(monkeypatch):
    calls = _capture(monkeypatch)
    lvm_mod.create_vg("/dev/mapper/phermes_luks", "pve")
    assert any("vgcreate" in c[0] for c in calls)
    assert any("pve" in c for c in calls)


def test_create_root_lv(monkeypatch):
    calls = _capture(monkeypatch)
    lvm_mod.create_root_lv("pve", size_gb=30)
    assert any("lvcreate" in c[0] for c in calls)
    assert any("30G" in str(c) for c in calls)


def test_create_thin_pool(monkeypatch):
    calls = _capture(monkeypatch)
    lvm_mod.create_thin_pool("pve", pool_name="data", size_gb=370)
    assert any("lvcreate" in c[0] for c in calls)
    assert any("--thin" in c for c in calls)
    assert any("370G" in str(c) for c in calls)


def test_compute_lvm_sizes():
    sizes = lvm_mod.compute_lvm_sizes(total_lvm_gb=400)
    assert sizes["root_gb"] == 30
    assert sizes["pool_gb"] == 370


def test_create_thin_volume(monkeypatch):
    calls = _capture(monkeypatch)
    lvm_mod.create_thin_volume("pve", pool_name="data", vol_name="vm-100-disk-0", size_gb=120)
    assert any("lvcreate" in c[0] for c in calls)
    assert any("--thin" in c or "-T" in str(c) for c in calls)


def test_create_btrfs_lv(monkeypatch):
    calls = _capture(monkeypatch)
    path = lvm_mod.create_btrfs_lv("pve", data_gb=330)
    assert any("lvcreate" in c[0] for c in calls)
    assert any("btrfs-data" in str(c) for c in calls)
    assert path == "/dev/pve/btrfs-data"


def test_setup_lvm_calls_all_steps(monkeypatch):
    calls = _capture(monkeypatch)
    result = lvm_mod.setup_lvm("/dev/mapper/phermes_luks", total_lvm_gb=400)
    cmd_names = [c[0] for c in calls]
    assert "pvcreate" in cmd_names
    assert "vgcreate" in cmd_names
    assert "lvcreate" in cmd_names
    assert result["vg"] == "pve"
    assert "root_lv" in result
    assert "thin_pool" in result
