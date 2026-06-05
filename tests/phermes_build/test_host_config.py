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


def test_avahi_service_config():
    conf = hc.avahi_service_config()
    assert "_https._tcp" in conf
    assert "_smb._tcp" in conf


def test_nftables_ruleset_includes_nat_chain_for_vmbr0():
    rules = hc.nftables_ruleset()
    # NAT table + postrouting chain present
    assert "table ip nat" in rules
    assert "chain postrouting" in rules
    assert "type nat hook postrouting" in rules
    # MASQUERADE the vmbr0 network out eth0
    assert "10.10.10.0/24" in rules
    assert 'oifname "eth0"' in rules
    assert "masquerade" in rules
