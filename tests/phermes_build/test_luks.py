from phermes_build import luks as luks_mod


def test_format_calls_cryptsetup(monkeypatch):
    calls = []
    monkeypatch.setattr(luks_mod, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")
    luks_mod.format_luks("/dev/sdb3", "secret")
    assert any("cryptsetup" in c[0] and "luksFormat" in c for c in calls)
    assert any("/dev/sdb3" in c for c in calls)


def test_format_passes_passphrase_via_stdin(monkeypatch):
    received = {}

    def fake_run(cmd, *, input=None, check=True):
        received["input"] = input
        return ""

    monkeypatch.setattr(luks_mod, "run_cmd", fake_run)
    luks_mod.format_luks("/dev/sdb3", "my-passphrase")
    assert received["input"] == "my-passphrase"


def test_open_luks_calls_luksopen(monkeypatch):
    calls = []
    monkeypatch.setattr(luks_mod, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")
    luks_mod.open_luks("/dev/sdb3", "phermes_data", "secret")
    assert any("luksOpen" in c for c in calls)
    assert any("phermes_data" in c for c in calls)


def test_close_luks_calls_luksclose(monkeypatch):
    calls = []
    monkeypatch.setattr(luks_mod, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")
    luks_mod.close_luks("phermes_data")
    assert any("luksClose" in c for c in calls)
    assert any("phermes_data" in c for c in calls)


def test_mapper_path():
    assert luks_mod.mapper_path("phermes_data") == "/dev/mapper/phermes_data"


def test_add_passphrase(monkeypatch):
    received = {}

    def fake_run(cmd, *, input=None, check=True):
        received.setdefault("cmds", []).append(cmd)
        received["input"] = input
        return ""

    monkeypatch.setattr(luks_mod, "run_cmd", fake_run)
    luks_mod.add_passphrase("/dev/sdb3", "old-pass", "new-pass")
    assert any("luksAddKey" in c for c in received["cmds"])
    assert "old-pass" in received["input"]
    assert "new-pass" in received["input"]


def test_remove_passphrase(monkeypatch):
    calls = []
    monkeypatch.setattr(luks_mod, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")
    luks_mod.remove_passphrase("/dev/sdb3", "old-pass")
    assert any("luksRemoveKey" in c for c in calls)
