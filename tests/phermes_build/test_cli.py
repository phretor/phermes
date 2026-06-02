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


def test_skip_os_install_flag_in_help():
    result = runner.invoke(app, ["--help"])
    assert "skip-os-install" in result.output


def _patch_all_steps(monkeypatch, called: list) -> None:
    """Patch every step function so no real system calls are made."""
    import phermes_build.cli as cli_mod

    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "compute_layout", lambda *a, **kw: _fake_layout())
    monkeypatch.setattr(
        cli_mod.partitioner, "create_partition_table", lambda *a, **kw: called.append("partition")
    )
    monkeypatch.setattr(cli_mod, "_setup_luks", lambda *a, **kw: called.append("luks"))
    monkeypatch.setattr(cli_mod, "_setup_lvm", lambda *a, **kw: called.append("lvm"))
    monkeypatch.setattr(cli_mod, "_setup_btrfs", lambda *a, **kw: called.append("btrfs"))
    monkeypatch.setattr(cli_mod, "_setup_exfat", lambda *a, **kw: called.append("exfat"))
    monkeypatch.setattr(cli_mod, "_install_proxmox", lambda *a, **kw: called.append("proxmox"))
    monkeypatch.setattr(cli_mod, "_setup_credentials", lambda *a, **kw: called.append("creds"))
    monkeypatch.setattr(cli_mod, "_configure_host", lambda *a, **kw: called.append("host"))
    monkeypatch.setattr(cli_mod, "_provision_vms", lambda *a, **kw: called.append("vms"))
    monkeypatch.setattr(cli_mod, "_write_firstboot", lambda: called.append("firstboot"))


def test_skip_os_install_omits_os_steps(monkeypatch):
    """--skip-os-install must not call debootstrap or any Proxmox install step."""
    called: list = []
    _patch_all_steps(monkeypatch, called)

    runner.invoke(app, ["/dev/sdb", "--skip-os-install", "--dev-credentials"])
    assert "proxmox" not in called
    assert "host" not in called
    assert "firstboot" not in called
    assert "luks" in called
    assert "lvm" in called
    assert "btrfs" in called


def test_full_install_runs_all_steps(monkeypatch):
    """Without --skip-os-install all steps run, including credentials."""
    called: list = []
    _patch_all_steps(monkeypatch, called)

    runner.invoke(app, ["/dev/sdb", "--dev-credentials"])
    assert all(
        s in called for s in ["luks", "lvm", "btrfs", "proxmox", "creds", "host", "firstboot"]
    )


def test_verbose_runs_all_steps_and_sets_flag(monkeypatch):
    """--verbose runs the same steps (no spinner) and enables streaming."""
    import phermes_build.cli as cli_mod

    seen: list = []
    monkeypatch.setattr(cli_mod, "set_verbose", lambda v: seen.append(("verbose", v)))
    called: list = []
    _patch_all_steps(monkeypatch, called)

    result = runner.invoke(app, ["/dev/sdb", "--verbose", "--dev-credentials"])
    assert result.exit_code == 0
    assert ("verbose", True) in seen
    assert all(s in called for s in ["luks", "lvm", "btrfs", "proxmox", "host", "firstboot"])


def test_production_build_requires_passphrase(monkeypatch):
    """A non-dev build with no passphrase must fail closed, never ship a known key."""
    called: list = []
    _patch_all_steps(monkeypatch, called)

    result = runner.invoke(app, ["/dev/sdb"])
    assert result.exit_code != 0
    assert called == []  # bailed before running any step


def test_resolve_luks_passphrase():
    from phermes_build.cli import TEMP_PASSPHRASE, _resolve_luks_passphrase

    assert _resolve_luks_passphrase(True, None) == TEMP_PASSPHRASE
    assert _resolve_luks_passphrase(False, "my-secret") == "my-secret"
    with pytest.raises(SystemExit):
        _resolve_luks_passphrase(False, None)


def test_setup_credentials_dev_sets_password_prod_locks(monkeypatch):
    import phermes_build.cli as cli_mod

    calls: list = []
    monkeypatch.setattr(cli_mod.proxmox, "set_root_password", lambda *a: calls.append("set"))
    monkeypatch.setattr(cli_mod.proxmox, "lock_root_account", lambda *a: calls.append("lock"))

    cli_mod._setup_credentials(True)
    cli_mod._setup_credentials(False)
    assert calls == ["set", "lock"]


def _fake_layout():
    from phermes_build.models import DiskLayout
    return DiskLayout(disk="/dev/sdb", disk_size_gb=1000, lvm_gb=400, data_gb=333, share_gb=0)
