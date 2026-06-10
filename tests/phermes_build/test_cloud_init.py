"""Tests for cloud_init.py — pure YAML renderers."""

import pytest

from phermes_build import cloud_init as ci_mod


def _cfg(**overrides):
    base = {
        "ssh_authorized_keys": ["ssh-ed25519 AAAA...op@host"],
    }
    base.update(overrides)
    return ci_mod.SeedConfig(**base)


def test_seedconfig_defaults():
    cfg = _cfg()
    assert cfg.hostname == "phermes-linux"
    assert cfg.username == "dev"
    assert cfg.install_uv is True
    assert cfg.ssh_authorized_keys == ["ssh-ed25519 AAAA...op@host"]


def test_meta_data_yaml_has_instance_id_and_local_hostname():
    out = ci_mod.meta_data_yaml(_cfg())
    assert "instance-id: phermes-phermes-linux" in out
    assert "local-hostname: phermes-linux" in out


def test_meta_data_yaml_uses_custom_hostname():
    out = ci_mod.meta_data_yaml(_cfg(hostname="dev-box"))
    assert "instance-id: phermes-dev-box" in out
    assert "local-hostname: dev-box" in out


def test_user_data_yaml_is_cloud_config_with_key_only_login():
    out = ci_mod.user_data_yaml(_cfg())
    assert out.startswith("#cloud-config\n")
    # Key-only login enforced
    assert "ssh_pwauth: false" in out
    assert "disable_root: true" in out
    assert "lock_passwd: true" in out
    # user + key
    assert "name: dev" in out
    assert "ssh-ed25519 AAAA...op@host" in out
    # sudo + shell
    assert "NOPASSWD:ALL" in out
    assert "shell: /bin/bash" in out


def test_user_data_yaml_renders_multiple_keys():
    cfg = ci_mod.SeedConfig(
        ssh_authorized_keys=[
            "ssh-ed25519 AAAA...first",
            "ssh-ed25519 BBBB...second",
        ],
    )
    out = ci_mod.user_data_yaml(cfg)
    assert "ssh-ed25519 AAAA...first" in out
    assert "ssh-ed25519 BBBB...second" in out


def test_vendor_data_yaml_installs_uv_by_default():
    out = ci_mod.vendor_data_yaml(_cfg())
    assert out.startswith("#cloud-config\n")
    assert "package_update: true" in out
    assert "curl" in out
    assert "https://astral.sh/uv/install.sh" in out
    assert "UV_INSTALL_DIR=/usr/local/bin" in out


def test_vendor_data_yaml_is_empty_when_install_uv_false():
    out = ci_mod.vendor_data_yaml(_cfg(install_uv=False))
    assert out.startswith("#cloud-config\n")
    assert "uv" not in out
    assert "runcmd" not in out


def _capture(monkeypatch):
    calls = []
    monkeypatch.setattr(ci_mod, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")
    return calls


def test_write_seed_iso_invokes_genisoimage_with_cidata_volid(monkeypatch, tmp_path):
    calls = _capture(monkeypatch)
    out = str(tmp_path / "seed.iso")
    ci_mod.write_seed_iso(out, _cfg())
    assert len(calls) == 1
    argv = calls[0]
    assert argv[0] == "genisoimage"
    # -output <out>
    out_idx = argv.index("-output")
    assert argv[out_idx + 1] == out
    # -volid CIDATA (the label NoCloud's datasource scans for)
    vol_idx = argv.index("-volid")
    assert argv[vol_idx + 1] == "CIDATA"
    # -joliet + -rock present
    assert "-joliet" in argv
    assert "-rock" in argv
    # The three seed files appear by name at the tail
    last_three = argv[-3:]
    names = [p.split("/")[-1] for p in last_three]
    assert set(names) == {"meta-data", "user-data", "vendor-data"}


def test_write_seed_iso_writes_three_files_into_tempdir(monkeypatch, tmp_path):
    """Capture the temp-dir paths that genisoimage is told to pack, and assert
    each one exists with the expected first line when genisoimage runs."""
    from pathlib import Path
    captured: dict = {}

    def fake_run_cmd(cmd, **kw):
        # The three seed files are the last three positional args.
        meta = Path(cmd[-3])
        user = Path(cmd[-2])
        vendor = Path(cmd[-1])
        captured["meta_text"] = meta.read_text()
        captured["user_text"] = user.read_text()
        captured["vendor_text"] = vendor.read_text()
        return ""

    monkeypatch.setattr(ci_mod, "run_cmd", fake_run_cmd)
    ci_mod.write_seed_iso(str(tmp_path / "seed.iso"), _cfg())
    assert captured["meta_text"].startswith("instance-id:")
    assert captured["user_text"].startswith("#cloud-config\n")
    assert "ssh-ed25519 AAAA...op@host" in captured["user_text"]
    assert captured["vendor_text"].startswith("#cloud-config\n")


def test_write_seed_iso_creates_output_parent_directory(monkeypatch, tmp_path):
    monkeypatch.setattr(ci_mod, "run_cmd", lambda cmd, **kw: "")
    nested = tmp_path / "deeply" / "nested" / "seed.iso"
    ci_mod.write_seed_iso(str(nested), _cfg())
    assert nested.parent.exists()


def test_write_seed_iso_rejects_empty_key_list(monkeypatch, tmp_path):
    calls = _capture(monkeypatch)
    cfg = ci_mod.SeedConfig(ssh_authorized_keys=[])
    with pytest.raises(ValueError, match="ssh_authorized_keys"):
        ci_mod.write_seed_iso(str(tmp_path / "seed.iso"), cfg)
    # No genisoimage call happened
    assert calls == []
