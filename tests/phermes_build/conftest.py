import pytest


def pytest_collection_modifyitems(config, items):
    """Skip integration tests unless -m integration is explicitly passed."""
    if config.option.markexpr and "integration" in config.option.markexpr:
        return
    skip_integration = pytest.mark.skip(reason="integration tests require root; run with -m integration")
    for item in items:
        if item.get_closest_marker("integration"):
            item.add_marker(skip_integration)


@pytest.fixture()
def mock_runner(monkeypatch):
    """Capture all run_cmd calls without executing them.

    Returns a list that accumulates (module_path, cmd) tuples.
    Call mock_runner.patch(module) to activate for that module.
    """
    captured: list[list[str]] = []

    def fake_run_cmd(cmd, *, input=None, check=True):  # noqa: A002
        captured.append(list(cmd))
        return ""

    return captured, fake_run_cmd


@pytest.fixture()
def tmp_disk(tmp_path):
    """512 MB sparse file usable as a loop device in integration tests."""
    img = tmp_path / "disk.img"
    img.write_bytes(b"")
    import subprocess
    subprocess.run(["truncate", "-s", "512M", str(img)], check=True)
    return img
