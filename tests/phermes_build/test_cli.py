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

    def fake_provision(source=None, seed_iso_path=None):
        seen["source"] = source

    monkeypatch.setattr(cli_mod, "_provision_linux_vm", fake_provision)

    result = runner.invoke(
        cli_mod.app,
        ["/dev/loop0", "--import-vm", "linux=/tmp/x.qcow2", "--dev-credentials"],
    )
    assert result.exit_code == 0, result.stdout
    assert seen.get("source") == "/tmp/x.qcow2"


def test_unsupported_import_vm_flavor_errors_out(monkeypatch):
    """--import-vm macos=<path> is rejected (slice #5a supports linux + windows)."""
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
        ["/dev/loop0", "--import-vm", "macos=/tmp/m.qcow2", "--dev-credentials"],
    )
    assert result.exit_code != 0


# ── C7 tests: _write_cloud_init_seed + build() threading ────────────────────


def test_no_seed_when_dev_credentials_absent(monkeypatch):
    """Production-style build: no --dev-credentials -> no cloud-init seed."""
    from phermes_build import cli as cli_mod

    seen: dict = {}
    def fake_no_seed(key):
        seen["called_with"] = key

    monkeypatch.setattr(cli_mod, "_write_cloud_init_seed", fake_no_seed)
    for helper in ("_setup_luks", "_setup_lvm", "_setup_btrfs", "_setup_exfat",
                   "_install_minimal_host", "_configure_host", "_setup_credentials",
                   "_write_firstboot", "_provision_linux_vm", "_partition"):
        monkeypatch.setattr(cli_mod, helper, lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "compute_layout", lambda *a, **k: _fake_layout())

    result = runner.invoke(cli_mod.app, [
        "/dev/loop0", "--import-vm", "linux=/tmp/x.qcow2", "--luks-passphrase", "test-pass",
    ])
    assert result.exit_code == 0, result.stdout
    # Helper called with None (no key => no seed)
    assert seen.get("called_with") is None


def test_seed_written_when_dev_credentials_and_key_set(monkeypatch):
    """Dev build: --dev-credentials + --dev-ssh-pubkey -> seed generated."""
    from phermes_build import cli as cli_mod

    seen: dict = {}

    def fake_seed(key):
        seen["called_with"] = key
        return "/var/lib/phermes/seed/linux.iso"

    monkeypatch.setattr(cli_mod, "_write_cloud_init_seed", fake_seed)

    def fake_provision(source=None, seed_iso_path=None):
        seen.update({"source": source, "seed": seed_iso_path})

    monkeypatch.setattr(cli_mod, "_provision_linux_vm", fake_provision)
    for helper in ("_setup_luks", "_setup_lvm", "_setup_btrfs", "_setup_exfat",
                   "_install_minimal_host", "_configure_host", "_setup_credentials",
                   "_write_firstboot", "_partition"):
        monkeypatch.setattr(cli_mod, helper, lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "compute_layout", lambda *a, **k: _fake_layout())

    result = runner.invoke(cli_mod.app, [
        "/dev/loop0",
        "--dev-credentials",
        "--dev-ssh-pubkey", "ssh-ed25519 AAAA...op@host",
        "--import-vm", "linux=/tmp/x.qcow2",
    ])
    assert result.exit_code == 0, result.stdout
    assert seen.get("called_with") == "ssh-ed25519 AAAA...op@host"
    assert seen.get("seed") == "/var/lib/phermes/seed/linux.iso"


def test_no_seed_when_no_vm(monkeypatch):
    """--no-vm: even with --dev-credentials, no seed is written (no VM to seed)."""
    from phermes_build import cli as cli_mod

    seen: dict = {}

    def fake_seed(key):
        seen["seed_called"] = True
        return None

    monkeypatch.setattr(cli_mod, "_write_cloud_init_seed", fake_seed)
    monkeypatch.setattr(cli_mod, "_provision_linux_vm",
                        lambda *a, **k: seen.setdefault("provision_called", True))
    for helper in ("_setup_luks", "_setup_lvm", "_setup_btrfs", "_setup_exfat",
                   "_install_minimal_host", "_configure_host", "_setup_credentials",
                   "_write_firstboot", "_partition"):
        monkeypatch.setattr(cli_mod, helper, lambda *a, **k: None)
    monkeypatch.setattr(cli_mod, "validate_disk_path", lambda d: None)
    monkeypatch.setattr(cli_mod, "compute_layout", lambda *a, **k: _fake_layout())

    result = runner.invoke(cli_mod.app, [
        "/dev/loop0",
        "--dev-credentials",
        "--dev-ssh-pubkey", "ssh-ed25519 AAAA...op@host",
        "--no-vm",
    ])
    assert result.exit_code == 0, result.stdout
    # Provisioning was skipped, so the seed helper was never invoked.
    assert "seed_called" not in seen
    assert "provision_called" not in seen


def test_write_cloud_init_seed_returns_none_when_no_key():
    from phermes_build import cli as cli_mod

    assert cli_mod._write_cloud_init_seed(None) is None


def test_write_cloud_init_seed_calls_cloud_init_when_key_present(monkeypatch, tmp_path):
    """_write_cloud_init_seed shells the work to cloud_init.write_seed_iso and
    returns LINUX_SEED_PATH (the guest path, NOT the chroot path)."""
    from phermes_build import cli as cli_mod
    from phermes_build import cloud_init as ci_mod_local
    from phermes_build import vm as vm_mod_local

    captured: dict = {}

    def fake_write_seed_iso(out_path: str, cfg) -> None:
        captured["out"] = out_path
        captured["keys"] = cfg.ssh_authorized_keys

    monkeypatch.setattr(ci_mod_local, "write_seed_iso", fake_write_seed_iso)
    monkeypatch.setattr(cli_mod, "PVE_ROOT_MOUNT", str(tmp_path))

    out = cli_mod._write_cloud_init_seed("ssh-ed25519 AAAA...op@host")
    assert out == vm_mod_local.LINUX_SEED_PATH
    assert captured["out"] == str(tmp_path / "var/lib/phermes/seed/linux.iso")
    assert captured["keys"] == ["ssh-ed25519 AAAA...op@host"]


# ── W5: _vm_source generic flavor extraction ──────────────────────────────


def test_vm_source_returns_linux_path():
    from phermes_build import cli as cli_mod
    assert cli_mod._vm_source(["linux=/tmp/x.qcow2"], "linux") == "/tmp/x.qcow2"


def test_vm_source_returns_windows_path():
    from phermes_build import cli as cli_mod
    assert cli_mod._vm_source(["windows=/tmp/win.qcow2"], "windows") == "/tmp/win.qcow2"


def test_vm_source_picks_correct_flavor_when_both_given():
    from phermes_build import cli as cli_mod
    args = ["linux=/a.qcow2", "windows=/b.qcow2"]
    assert cli_mod._vm_source(args, "linux") == "/a.qcow2"
    assert cli_mod._vm_source(args, "windows") == "/b.qcow2"


def test_vm_source_returns_none_when_flavor_absent():
    from phermes_build import cli as cli_mod
    assert cli_mod._vm_source(["linux=/a.qcow2"], "windows") is None


def test_vm_source_rejects_unsupported_flavor():
    import typer

    from phermes_build import cli as cli_mod

    with pytest.raises(typer.BadParameter):
        cli_mod._vm_source(["macos=/m.qcow2"], "linux")
