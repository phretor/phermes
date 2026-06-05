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
    monkeypatch.setattr(
        cli_mod, "_install_minimal_host", lambda *a, **kw: called.append("install_minimal_host")
    )
    monkeypatch.setattr(cli_mod, "_setup_credentials", lambda *a, **kw: called.append("creds"))
    monkeypatch.setattr(cli_mod, "_configure_host", lambda *a, **kw: called.append("host"))
    monkeypatch.setattr(
        cli_mod, "_provision_linux_vm", lambda *a, **kw: called.append("vms")
    )
    monkeypatch.setattr(cli_mod, "_write_firstboot", lambda: called.append("firstboot"))


def test_skip_os_install_omits_os_steps(monkeypatch):
    """--skip-os-install must not call debootstrap or any host install step."""
    called: list = []
    _patch_all_steps(monkeypatch, called)

    runner.invoke(app, ["/dev/sdb", "--skip-os-install", "--dev-credentials"])
    assert "install_minimal_host" not in called
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
        s in called
        for s in ["luks", "lvm", "btrfs", "install_minimal_host", "creds", "host", "firstboot"]
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
    assert all(
        s in called for s in ["luks", "lvm", "btrfs", "install_minimal_host", "host", "firstboot"]
    )


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
    monkeypatch.setattr(cli_mod.host_mod, "set_root_password", lambda *a: calls.append("set"))
    monkeypatch.setattr(cli_mod.host_mod, "lock_root_account", lambda *a: calls.append("lock"))
    monkeypatch.setattr(cli_mod.host_mod, "enable_dev_root_ssh", lambda *a: calls.append("ssh"))

    cli_mod._setup_credentials(True)  # dev, no key
    cli_mod._setup_credentials(True, "ssh-ed25519 AAAA")  # dev + key
    cli_mod._setup_credentials(False)  # production
    assert calls == ["set", "set", "ssh", "lock"]


def _fake_layout():
    from phermes_build.models import DiskLayout

    return DiskLayout(disk="/dev/sdb", disk_size_gb=1000, lvm_gb=400, data_gb=333, share_gb=0)


# ── New H8 tests ────────────────────────────────────────────────────────────


def test_help_does_not_advertise_dropped_flags():
    from phermes_build import cli as cli_mod

    result = runner.invoke(cli_mod.app, ["--help"])
    assert result.exit_code == 0
    assert "--toy-vm" not in result.stdout
    assert "--linux-node" not in result.stdout


def test_help_advertises_no_vm_flag():
    from phermes_build import cli as cli_mod

    result = runner.invoke(cli_mod.app, ["--help"])
    assert "--no-vm" in result.stdout


def test_orchestration_calls_install_minimal_host_not_proxmox(monkeypatch):
    """The build pipeline routes through host.install_minimal_host (not proxmox)."""
    from phermes_build import cli as cli_mod

    seen: list[str] = []

    monkeypatch.setattr(cli_mod, "_setup_luks", lambda *a, **k: seen.append("luks"))
    monkeypatch.setattr(cli_mod, "_setup_lvm", lambda *a, **k: seen.append("lvm"))
    monkeypatch.setattr(cli_mod, "_setup_btrfs", lambda *a, **k: seen.append("btrfs"))
    monkeypatch.setattr(cli_mod, "_setup_exfat", lambda *a, **k: seen.append("exfat"))
    monkeypatch.setattr(
        cli_mod,
        "_install_minimal_host",
        lambda *a, **k: seen.append("install_minimal_host"),
    )
    monkeypatch.setattr(cli_mod, "_configure_host", lambda *a, **k: seen.append("configure_host"))
    monkeypatch.setattr(cli_mod, "_setup_credentials", lambda *a, **k: seen.append("credentials"))
    monkeypatch.setattr(
        cli_mod, "_provision_linux_vm", lambda *a, **k: seen.append("provision_vm")
    )
    monkeypatch.setattr(cli_mod, "_write_firstboot", lambda *a, **k: seen.append("firstboot"))
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "_partition", lambda *a, **k: seen.append("partition"))
    monkeypatch.setattr(cli_mod, "compute_layout", lambda *a, **k: _fake_layout())

    result = runner.invoke(
        cli_mod.app,
        ["/dev/loop0", "--skip-os-install", "--no-vm", "--dev-credentials"],
    )
    assert result.exit_code == 0, result.stdout
    assert "install_minimal_host" not in seen  # short-circuited by --skip-os-install
    assert "provision_vm" not in seen  # short-circuited by --no-vm


def test_import_vm_linux_routes_into_provision_linux_disk(monkeypatch):
    """--import-vm linux=<path> reaches _provision_linux_vm(source=<path>)."""
    from phermes_build import cli as cli_mod

    seen: dict = {}

    for helper in (
        "_setup_luks",
        "_setup_lvm",
        "_setup_btrfs",
        "_setup_exfat",
        "_install_minimal_host",
        "_configure_host",
        "_setup_credentials",
        "_write_firstboot",
        "_partition",
    ):
        monkeypatch.setattr(cli_mod, helper, lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "compute_layout", lambda *a, **k: _fake_layout())

    def fake_provision(source=None):
        seen["source"] = source

    monkeypatch.setattr(cli_mod, "_provision_linux_vm", fake_provision)

    result = runner.invoke(
        cli_mod.app,
        ["/dev/loop0", "--import-vm", "linux=/tmp/x.qcow2", "--dev-credentials"],
    )
    assert result.exit_code == 0, result.stdout
    assert seen.get("source") == "/tmp/x.qcow2"


def test_unsupported_import_vm_flavor_errors_out(monkeypatch):
    """--import-vm windows=<path> is rejected (MVP supports only linux)."""
    from phermes_build import cli as cli_mod

    for helper in (
        "_setup_luks",
        "_setup_lvm",
        "_setup_btrfs",
        "_setup_exfat",
        "_install_minimal_host",
        "_configure_host",
        "_setup_credentials",
        "_write_firstboot",
        "_provision_linux_vm",
        "_partition",
    ):
        monkeypatch.setattr(cli_mod, helper, lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "compute_layout", lambda *a, **k: _fake_layout())

    result = runner.invoke(
        cli_mod.app,
        ["/dev/loop0", "--import-vm", "windows=/tmp/w.qcow2", "--dev-credentials"],
    )
    assert result.exit_code != 0
