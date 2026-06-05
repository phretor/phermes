import os

from phermes_build import systemd_units as sd_mod


def test_phermesd_service_unit_text_has_required_fields():
    unit = sd_mod.phermesd_service_unit()
    # Identity + docs
    assert "Description=PHermes VM Orchestrator" in unit
    # Dependencies
    assert "After=network-online.target" in unit
    assert "Wants=network-online.target" in unit
    assert "ConditionPathExists=/dev/kvm" in unit
    # ExecStart points at the binary we ship and uses production paths
    assert "/usr/local/sbin/phermesd" in unit
    assert "--vms-dir /etc/phermes/vms" in unit
    assert "--socket /run/phermesd/control.sock" in unit
    # Restart policy + Install
    assert "Restart=on-failure" in unit
    assert "WantedBy=multi-user.target" in unit


def test_install_phermesd_unit_writes_file_and_creates_wants_symlink(tmp_path):
    mount_point = str(tmp_path / "chroot")
    os.makedirs(mount_point)

    sd_mod.install_phermesd_unit(mount_point)

    unit_path = tmp_path / "chroot" / "etc/systemd/system/phermesd.service"
    assert unit_path.exists()
    assert "ExecStart=" in unit_path.read_text()

    wants_link = (
        tmp_path / "chroot" / "etc/systemd/system/multi-user.target.wants" / "phermesd.service"
    )
    assert wants_link.is_symlink()
    assert os.readlink(str(wants_link)) == "/etc/systemd/system/phermesd.service"


def test_install_phermesd_unit_is_idempotent(tmp_path):
    mount_point = str(tmp_path / "chroot")
    os.makedirs(mount_point)
    sd_mod.install_phermesd_unit(mount_point)
    # second call must not raise (symlink already exists)
    sd_mod.install_phermesd_unit(mount_point)
