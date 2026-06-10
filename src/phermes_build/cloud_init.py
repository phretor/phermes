"""Cloud-init NoCloud seed generation.

Builds a seed.iso (labelled CIDATA) that bootstraps a Linux guest at first boot
with a `dev` user, the operator's SSH key, DHCP, and `uv` (so the Hermes runtime
can run). The seed is written by phermes-build at install time and attached to
the guest as a CDROM (see slice #1's DiskInterface::Cdrom).

Linux-only. Windows + macOS don't use NoCloud — they return in #5.
"""

from dataclasses import dataclass, field


@dataclass(frozen=True)
class SeedConfig:
    """Input to seed generation. Populated by cli.py from build-time flags."""

    hostname: str = "phermes-linux"
    username: str = "dev"
    ssh_authorized_keys: list[str] = field(default_factory=list)
    install_uv: bool = True


def meta_data_yaml(cfg: SeedConfig) -> str:
    """/CIDATA/meta-data — instance-id + hostname."""
    return (
        f"instance-id: phermes-{cfg.hostname}\n"
        f"local-hostname: {cfg.hostname}\n"
    )


def user_data_yaml(cfg: SeedConfig) -> str:
    """/CIDATA/user-data — user + SSH key + key-only login + disable_root."""
    keys = "\n".join(f"      - {k.strip()}" for k in cfg.ssh_authorized_keys)
    return (
        "#cloud-config\n"
        "ssh_pwauth: false\n"
        "disable_root: true\n"
        f"hostname: {cfg.hostname}\n"
        "users:\n"
        f"  - name: {cfg.username}\n"
        "    sudo: ALL=(ALL) NOPASSWD:ALL\n"
        "    shell: /bin/bash\n"
        "    lock_passwd: true\n"
        "    ssh_authorized_keys:\n"
        f"{keys}\n"
    )


def vendor_data_yaml(cfg: SeedConfig) -> str:
    """/CIDATA/vendor-data — installs uv if cfg.install_uv."""
    if not cfg.install_uv:
        return "#cloud-config\n{}\n"
    return (
        "#cloud-config\n"
        "package_update: true\n"
        "packages:\n"
        "  - curl\n"
        "  - ca-certificates\n"
        "runcmd:\n"
        "  - [ sh, -c, "
        "'curl -LsSf https://astral.sh/uv/install.sh | "
        "env UV_INSTALL_DIR=/usr/local/bin sh' ]\n"
    )
