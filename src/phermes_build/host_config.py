def nftables_ruleset() -> str:
    return """\
#!/usr/sbin/nft -f

flush ruleset

table inet filter {
    chain input {
        type filter hook input priority 0; policy drop;

        iif lo accept
        ct state established,related accept

        # LUKS unlock via Dropbear
        tcp dport 2222 accept

        # PHermes web UI
        tcp dport 443 accept

        # Proxmox web UI: localhost only
        tcp dport 8006 ip saddr != 127.0.0.1 drop
        tcp dport 8006 ip saddr 127.0.0.1 accept

        ip protocol icmp accept
    }

    chain forward {
        type filter hook forward priority 0; policy drop;

        iifname "vmbr0" accept
        oifname "vmbr0" ct state established,related accept
    }

    chain output {
        type filter hook output priority 0; policy accept;

        oifname != "vmbr0" tcp dport 445 drop
        oifname != "vmbr0" udp dport 445 drop
    }
}

table ip nat {
    chain postrouting {
        type nat hook postrouting priority 100;
        ip saddr 10.10.10.0/24 oifname "eth0" masquerade
    }
}
"""


def samba_config(share_path: str, username: str) -> str:
    return f"""\
[global]
   workgroup = WORKGROUP
   server string = PHermes Data
   server role = standalone server
   interfaces = vmbr0
   bind interfaces only = yes
   log level = 1
   smb ports = 445

[PHermesData]
   path = {share_path}
   valid users = {username}
   read only = no
   browseable = yes
   create mask = 0644
   directory mask = 0755
"""


def dropbear_initramfs_config() -> str:
    return 'DROPBEAR_OPTIONS="-p 2222 -s"\n'


def avahi_service_config() -> str:
    return """\
<?xml version="1.0" standalone='no'?>
<!DOCTYPE service-group SYSTEM "avahi-service.dtd">
<service-group>
  <name replace-wildcards="yes">PHermes on %h</name>
  <service>
    <type>_https._tcp</type>
    <port>443</port>
  </service>
  <service>
    <type>_smb._tcp</type>
    <port>445</port>
  </service>
</service-group>
"""
