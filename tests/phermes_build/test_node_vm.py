from phermes_build import node_vm as node_mod


def test_helper_script_uses_cloudinit_and_vmbr0():
    s = node_mod.node_helper_script()
    assert "qm create" in s
    assert "--cpu host" in s  # nested virt extensions for the guest
    assert "bridge=vmbr0" in s  # NIC on the internal bridge
    assert "--ide2" in s and "cloudinit" in s  # cloud-init drive
    assert "--ciuser" in s and "--sshkeys" in s  # dev user + authorized key
    assert node_mod.NODE_IP in s
    assert str(node_mod.NODE_VMID) in s


def test_helper_script_installs_uv_via_vendor_data():
    s = node_mod.node_helper_script()
    assert "astral.sh/uv/install.sh" in s  # uv installed so Hermes can run
    assert "vendor=local:snippets/phermes-node-vendor.yaml" in s
    assert "package_update" in s


def test_install_node_vm_downloads_and_writes_helper(monkeypatch, tmp_path):
    calls = []
    monkeypatch.setattr(node_mod, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")

    node_mod.install_node_vm(str(tmp_path), url="http://example/node.qcow2")

    assert any("wget" in c[0] for c in calls)
    assert any("http://example/node.qcow2" in c for c in calls)
    # follows redirects (Debian 'latest' link)
    assert any("-L" in c for c in calls if c[0] == "wget")

    helper = tmp_path / "usr/local/bin/phermes-node"
    assert helper.exists()
    assert oct(helper.stat().st_mode)[-3:] == "755"
    assert (tmp_path / "var/lib/phermes").is_dir()


def test_debian_cloud_url_is_glibc_genericcloud():
    assert "debian" in node_mod.DEBIAN_CLOUD_URL
    assert "genericcloud" in node_mod.DEBIAN_CLOUD_URL
    assert node_mod.DEBIAN_CLOUD_URL.endswith(".qcow2")
