import pytest
from typer.testing import CliRunner

from phermes_build.cli import app
from phermes_build.models import BuildConfig

runner = CliRunner()


def test_help_exits_cleanly():
    result = runner.invoke(app, ["--help"])
    assert result.exit_code == 0
    assert "Usage" in result.output


def test_missing_disk_arg_shows_error():
    result = runner.invoke(app, [])
    assert result.exit_code != 0


def test_build_config_share_default():
    cfg = BuildConfig(disk="/dev/sdb")
    assert cfg.share_size_gb == 250


def test_validate_disk_path_rejects_non_dev(tmp_path):
    from phermes_build.cli import validate_disk_path

    with pytest.raises(SystemExit):
        validate_disk_path("not-a-dev-path")


def test_validate_disk_path_rejects_nonexistent():
    from phermes_build.cli import validate_disk_path

    with pytest.raises(SystemExit):
        validate_disk_path("/dev/nonexistent_phermes_test_disk")
