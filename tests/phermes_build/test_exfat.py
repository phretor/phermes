import pytest
from phermes_build import exfat as exfat_mod


def test_format_exfat(monkeypatch):
    calls = []
    monkeypatch.setattr(exfat_mod, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")
    exfat_mod.format_exfat("/dev/sdb4")
    assert any("mkfs.exfat" in c[0] for c in calls)
    assert any("PHERMES_SHARE" in str(c) for c in calls)


def test_is_exfat_available_true(monkeypatch):
    monkeypatch.setattr(exfat_mod, "run_cmd", lambda cmd, **kw: "/usr/bin/mkfs.exfat")
    assert exfat_mod.is_exfat_available() is True


def test_is_exfat_available_false(monkeypatch):
    from phermes_build.runner import CommandError

    def raise_error(cmd, **kw):
        raise CommandError(cmd, 1, "not found")

    monkeypatch.setattr(exfat_mod, "run_cmd", raise_error)
    assert exfat_mod.is_exfat_available() is False
