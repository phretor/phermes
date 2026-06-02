from phermes_build import toy_vm as toy_mod


def test_helper_script_uses_cpu_host_and_serial():
    script = toy_mod.toy_helper_script()
    assert "qm create" in script
    assert "--cpu host" in script  # nested virt extensions exposed to the guest
    assert "--serial0 socket" in script
    assert "qm terminal" in script
    assert str(toy_mod.TOY_VMID) in script


def test_install_toy_vm_downloads_and_writes_helper(monkeypatch, tmp_path):
    calls = []
    monkeypatch.setattr(toy_mod, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")

    toy_mod.install_toy_vm(str(tmp_path), url="http://example/toy.qcow2")

    # Image fetched via wget to the in-image path
    assert any("wget" in c[0] for c in calls)
    assert any("http://example/toy.qcow2" in c for c in calls)

    # Helper installed and executable
    helper = tmp_path / "usr/local/bin/phermes-toy-vm"
    assert helper.exists()
    assert oct(helper.stat().st_mode)[-3:] == "755"

    # Image destination directory created
    assert (tmp_path / "var/lib/phermes").is_dir()


def test_alpine_url_is_pinned():
    assert "alpine" in toy_mod.ALPINE_QCOW2_URL
    assert toy_mod.ALPINE_QCOW2_URL.endswith(".qcow2")
