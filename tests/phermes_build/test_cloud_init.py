"""Tests for cloud_init.py — pure YAML renderers."""

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
