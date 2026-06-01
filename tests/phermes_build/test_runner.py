import subprocess

import pytest

from phermes_build.runner import CommandError, run_cmd


def test_run_cmd_returns_stdout(monkeypatch):
    def fake_run(cmd, **kwargs):
        return subprocess.CompletedProcess(cmd, 0, stdout="hello\n", stderr="")

    monkeypatch.setattr(subprocess, "run", fake_run)
    assert run_cmd(["echo", "hello"]) == "hello"


def test_run_cmd_raises_on_nonzero(monkeypatch):
    def fake_run(cmd, **kwargs):
        return subprocess.CompletedProcess(cmd, 1, stdout="", stderr="something failed")

    monkeypatch.setattr(subprocess, "run", fake_run)
    with pytest.raises(CommandError) as exc:
        run_cmd(["false"])
    assert exc.value.returncode == 1
    assert "something failed" in str(exc.value)


def test_run_cmd_check_false_does_not_raise(monkeypatch):
    def fake_run(cmd, **kwargs):
        return subprocess.CompletedProcess(cmd, 1, stdout="out", stderr="err")

    monkeypatch.setattr(subprocess, "run", fake_run)
    result = run_cmd(["false"], check=False)
    assert result == "out"


def test_run_cmd_passes_input(monkeypatch):
    received = {}

    def fake_run(cmd, **kwargs):
        received["input"] = kwargs.get("input")
        return subprocess.CompletedProcess(cmd, 0, stdout="", stderr="")

    monkeypatch.setattr(subprocess, "run", fake_run)
    run_cmd(["cat"], input="data")
    assert received["input"] == "data"
