from phermes_build import host_config as hc


def test_nftables_blocks_8006_from_lan():
    rules = hc.nftables_ruleset()
    assert "8006" in rules
    assert "127.0.0.1" in rules


def test_nftables_allows_443_from_lan():
    rules = hc.nftables_ruleset()
    assert "443" in rules
    assert "accept" in rules


def test_nftables_allows_2222_from_lan():
    rules = hc.nftables_ruleset()
    assert "2222" in rules


def test_nftables_samba_restricted_to_bridge():
    rules = hc.nftables_ruleset()
    assert "445" in rules


def test_samba_config_not_lan_exposed():
    conf = hc.samba_config(share_path="/mnt/data/@overlay", username="alice")
    assert "interfaces" in conf
    assert "vmbr0" in conf
    assert "/mnt/data/@overlay" in conf


def test_samba_config_contains_phermes_share():
    conf = hc.samba_config(share_path="/mnt/data/@overlay", username="alice")
    assert "[PHermesData]" in conf


def test_dropbear_config_sets_port_2222():
    conf = hc.dropbear_initramfs_config()
    assert "2222" in conf


def test_proxmox_rbac_commands_create_restricted_user(monkeypatch):
    calls = []
    monkeypatch.setattr(hc, "run_cmd", lambda cmd, **kw: calls.append(cmd) or "")
    hc.configure_proxmox_rbac("alice", "password123")
    cmd_strings = [" ".join(c) for c in calls]
    assert any("pveum" in s for s in cmd_strings)
    assert any("alice" in s for s in cmd_strings)
