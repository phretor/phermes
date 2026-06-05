"""Systemd unit text + chroot installer for phermesd.

Kept separate from host.py so the unit text is testable in isolation and so a
later hardening pass (immutable host / OpenRC) can swap this module without
touching the OS installer.
"""

import os


def phermesd_service_unit() -> str:
    """Return the text of /etc/systemd/system/phermesd.service.

    `After=lvm2-activation.service` is intentionally omitted: on a fresh Debian
    bookworm root the canonical unit name is `lvm2-monitor.service`; the
    dependency on `local-fs.target` already established by systemd's default
    ordering is sufficient for our use (phermesd opens `/etc/phermes/vms/` and
    `/run/phermesd/`, not LVM devices directly — the LVM activation happens at
    initramfs unlock time, long before this unit starts).
    """
    return (
        "[Unit]\n"
        "Description=PHermes VM Orchestrator\n"
        "Documentation=https://github.com/phretor/phermes\n"
        "After=network-online.target\n"
        "Wants=network-online.target\n"
        "ConditionPathExists=/dev/kvm\n"
        "\n"
        "[Service]\n"
        "Type=simple\n"
        "ExecStart=/usr/local/sbin/phermesd"
        " --vms-dir /etc/phermes/vms"
        " --run-dir /run/phermesd"
        " --socket /run/phermesd/control.sock\n"
        "Restart=on-failure\n"
        "RestartSec=5s\n"
        "\n"
        "[Install]\n"
        "WantedBy=multi-user.target\n"
    )


def install_phermesd_unit(mount_point: str) -> None:
    """Write the unit into the chroot and create the multi-user.target.wants symlink.

    Mirrors the trick used by the old install_pve_firstboot_init: there is no
    running systemd in the chroot, so `systemctl enable` is unavailable.
    """
    unit_path = os.path.join(mount_point, "etc/systemd/system/phermesd.service")
    os.makedirs(os.path.dirname(unit_path), exist_ok=True)
    with open(unit_path, "w") as f:
        f.write(phermesd_service_unit())

    wants = os.path.join(mount_point, "etc/systemd/system/multi-user.target.wants")
    os.makedirs(wants, exist_ok=True)
    link = os.path.join(wants, "phermesd.service")
    if not os.path.islink(link):
        os.symlink("/etc/systemd/system/phermesd.service", link)
