import os
from typing import Annotated

import typer
from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn

from phermes_build import (
    btrfs,
    exfat,
    host_config,
    luks,
    lvm,
    partitioner,
    systemd_units,  # noqa: F401  (imported for side-effects in install path)
)
from phermes_build import (
    host as host_mod,
)
from phermes_build import (
    vm as vm_mod,
)
from phermes_build.disk import compute_layout
from phermes_build.firstboot import write_firstboot_flag, write_motd
from phermes_build.models import BuildConfig
from phermes_build.runner import CommandError, run_cmd, set_verbose

app = typer.Typer(name="phermes-build", help="PHermes SSD appliance builder")
console = Console()

PVE_ROOT_MOUNT = "/mnt/pve-root"
DATA_MOUNT = "/mnt/phermes-data"
LUKS_NAME = "phermes_luks"
TEMP_PASSPHRASE = "phermes-change-me"


def validate_disk_path(disk: str) -> None:
    if not disk.startswith("/dev/"):
        console.print(f"[red]Error:[/red] {disk} is not a block device path.")
        raise SystemExit(1)
    if not os.path.exists(disk):
        console.print(f"[red]Error:[/red] {disk} does not exist.")
        raise SystemExit(1)


@app.command()
def build(
    disk: Annotated[str, typer.Argument(help="Target block device, e.g. /dev/sdb")],
    share_size: Annotated[int, typer.Option(help="PHERMES_SHARE size in GB (0 to disable)")] = 250,
    share_encrypted: Annotated[
        bool, typer.Option(help="Encrypt PHERMES_SHARE inside LUKS")
    ] = False,
    import_vm: Annotated[
        list[str] | None,
        typer.Option(help="VM image to import: flavor=<path>, e.g. linux=/tmp/disk.qcow2"),
    ] = None,
    no_vm: Annotated[
        bool, typer.Option("--no-vm", help="Skip Linux VM provisioning.")
    ] = False,
    skip_os_install: Annotated[
        bool, typer.Option(help="Run disk setup only; skip host install (for testing)")
    ] = False,
    verbose: Annotated[
        bool, typer.Option("--verbose", "-v", help="Stream every command's output live")
    ] = False,
    dev_credentials: Annotated[
        bool,
        typer.Option(
            help="INSECURE: bake known temp credentials for testing (never for production)"
        ),
    ] = False,
    luks_passphrase: Annotated[
        str | None,
        typer.Option(
            envvar="PHERMES_LUKS_PASSPHRASE",
            help="LUKS passphrase for a production build (or set PHERMES_LUKS_PASSPHRASE)",
        ),
    ] = None,
    dev_ssh_pubkey: Annotated[
        str | None,
        typer.Option(
            envvar="PHERMES_DEV_SSH_PUBKEY",
            help="DEV: SSH public key to authorize for root login (with --dev-credentials)",
        ),
    ] = None,
) -> None:
    validate_disk_path(disk)
    set_verbose(verbose)

    cfg = BuildConfig(
        disk=disk,
        share_size_gb=share_size,
        share_encrypted=share_encrypted,
        temp_luks_passphrase=_resolve_luks_passphrase(dev_credentials, luks_passphrase),
    )

    # Validate --import-vm flavors early so we fail before touching the disk.
    linux_source = _linux_source(import_vm or [])

    layout = compute_layout(disk, cfg.share_size_gb, cfg.share_encrypted)

    disk_steps = [
        ("Partitioning SSD", lambda: _partition(layout)),
        ("Creating LUKS2 container", lambda: _setup_luks(layout, cfg)),
        ("Setting up LVM", lambda: _setup_lvm(layout)),
        ("Formatting Btrfs data partition", lambda: _setup_btrfs(layout)),
        ("Formatting exFAT share", lambda: _setup_exfat(layout)),
    ]
    os_steps: list[tuple[str, object]] = [
        (
            "Installing minimal Debian host + phermesd",
            lambda: _install_minimal_host(layout),
        ),
        (
            "Setting root credentials",
            lambda: _setup_credentials(dev_credentials, dev_ssh_pubkey),
        ),
        ("Configuring PHermes host", lambda: _configure_host(layout, cfg)),
        ("Writing first-boot flag", lambda: _write_firstboot()),
    ]
    if not no_vm:
        os_steps.append(
            (
                "Provisioning Linux VM",
                lambda: _provision_linux_vm(source=linux_source),
            )
        )

    steps = disk_steps if skip_os_install else disk_steps + os_steps

    if verbose:
        _run_steps_verbose(steps)
    else:
        _run_steps_progress(steps)

    if skip_os_install:
        console.print("\n[bold yellow]Disk setup complete.[/bold yellow]")
        console.print("Partitions, LUKS2, LVM, and Btrfs are ready.")
        console.print("Re-run without [bold]--skip-os-install[/bold] to install the host.")
    else:
        console.print("\n[bold green]PHermes SSD ready.[/bold green]")
        console.print("Safely eject and boot the target machine.")
        console.print("Connect from any browser: [bold]https://phermes.local[/bold]")


def _run_steps_progress(steps) -> None:
    with Progress(
        SpinnerColumn(), TextColumn("{task.description}"), console=console
    ) as progress:
        for description, step_fn in steps:
            task = progress.add_task(description)
            try:
                step_fn()
            except CommandError as e:
                progress.stop()
                console.print(f"[red]✗ {description} failed:[/red] {e}")
                raise typer.Exit(1) from e
            progress.update(
                task, description=f"[green]✓[/green] {description}", completed=True
            )


def _run_steps_verbose(steps) -> None:
    """Run steps with plain headers and live command output (no spinner)."""
    for description, step_fn in steps:
        console.print(f"[bold cyan]==>[/bold cyan] {description}")
        try:
            step_fn()
        except CommandError as e:
            console.print(f"[red]✗ {description} failed:[/red] {e}")
            raise typer.Exit(1) from e
        console.print(f"[green]✓[/green] {description}")


def _resolve_luks_passphrase(dev_credentials: bool, luks_passphrase: str | None) -> str:
    """Pick the LUKS passphrase, refusing to ship a known key in production."""
    if dev_credentials:
        return TEMP_PASSPHRASE
    if luks_passphrase:
        return luks_passphrase
    console.print(
        "[red]Error:[/red] a production build needs a LUKS passphrase. Pass "
        "[bold]--luks-passphrase[/bold] (or set PHERMES_LUKS_PASSPHRASE), or use "
        "[bold]--dev-credentials[/bold] for testing."
    )
    raise SystemExit(1)


def _linux_source(import_vm_args: list[str]) -> str | None:
    """Parse --import-vm linux=<path> (MVP only supports linux=)."""
    for entry in import_vm_args:
        flavor, _, path = entry.partition("=")
        if flavor == "linux" and path:
            return path
        if flavor and flavor != "linux":
            raise typer.BadParameter(
                f"--import-vm flavor '{flavor}' is not supported in the MVP "
                f"(only 'linux=<path>')."
            )
    return None


def _setup_credentials(dev_credentials: bool, dev_ssh_pubkey: str | None = None) -> None:
    """Dev builds get a known temp root password (+ optional SSH key); production
    locks root entirely."""
    if dev_credentials:
        host_mod.set_root_password(PVE_ROOT_MOUNT, host_mod.TEMP_ROOT_PASSWORD)
        if dev_ssh_pubkey:
            host_mod.enable_dev_root_ssh(PVE_ROOT_MOUNT, dev_ssh_pubkey)
    else:
        host_mod.lock_root_account(PVE_ROOT_MOUNT)


def _partition(layout) -> None:
    partitioner.create_partition_table(layout)


def _setup_luks(layout, cfg: BuildConfig) -> None:
    luks_part = partitioner.partition_path(layout.disk, 3)
    luks.format_luks(luks_part, cfg.temp_luks_passphrase)
    luks.open_luks(luks_part, LUKS_NAME, cfg.temp_luks_passphrase)


def _setup_lvm(layout) -> None:
    mapper = luks.mapper_path(LUKS_NAME)
    lvm.setup_lvm(mapper, layout.lvm_gb)
    host_mod.format_root_lv("/dev/pve/root")
    lvm.create_btrfs_lv("pve", layout.data_gb)


def _setup_btrfs(layout) -> None:
    data_dev = f"/dev/pve/{lvm.BTRFS_LV_NAME}"
    btrfs.format_btrfs(data_dev)
    btrfs.mount_btrfs(data_dev, DATA_MOUNT)
    btrfs.create_subvolumes(DATA_MOUNT)


def _setup_exfat(layout) -> None:
    if layout.share_gb == 0 or layout.share_encrypted:
        return
    share_part = partitioner.partition_path(layout.disk, 4)
    exfat.format_exfat(share_part)


def _install_minimal_host(layout) -> None:
    """Install minimal Debian + phermesd into the mounted chroot."""
    os.makedirs(PVE_ROOT_MOUNT, exist_ok=True)
    run_cmd(["mount", "/dev/pve/root", PVE_ROOT_MOUNT])
    efi_part = partitioner.partition_path(layout.disk, 1)
    boot_part = partitioner.partition_path(layout.disk, 2)
    luks_part = partitioner.partition_path(layout.disk, 3)
    host_mod.install_minimal_host(
        mount_point=PVE_ROOT_MOUNT,
        disk=layout.disk,
        luks_device=luks_part,
        efi_device=efi_part,
        boot_device=boot_part,
    )


def _configure_host(layout, cfg: BuildConfig) -> None:
    nft_path = os.path.join(PVE_ROOT_MOUNT, "etc/nftables.conf")
    with open(nft_path, "w") as f:
        f.write(host_config.nftables_ruleset())

    smb_path = os.path.join(PVE_ROOT_MOUNT, "etc/samba/smb.conf")
    os.makedirs(os.path.dirname(smb_path), exist_ok=True)
    with open(smb_path, "w") as f:
        f.write(
            host_config.samba_config(
                share_path=f"{DATA_MOUNT}/@overlay",
                username="phermes",
            )
        )

    dropbear_path = os.path.join(
        PVE_ROOT_MOUNT, "etc/dropbear/initramfs/dropbear.conf"
    )
    os.makedirs(os.path.dirname(dropbear_path), exist_ok=True)
    with open(dropbear_path, "w") as f:
        f.write(host_config.dropbear_initramfs_config())

    avahi_path = os.path.join(
        PVE_ROOT_MOUNT, "etc/avahi/services/phermes.service"
    )
    os.makedirs(os.path.dirname(avahi_path), exist_ok=True)
    with open(avahi_path, "w") as f:
        f.write(host_config.avahi_service_config())

    write_motd(PVE_ROOT_MOUNT, hostname="phermes", ip_hint="<your-ip>")


def _provision_linux_vm(source: str | None) -> None:
    vm_mod.write_linux_def(PVE_ROOT_MOUNT)
    vm_mod.provision_linux_disk(source=source)


def _write_firstboot() -> None:
    write_firstboot_flag(f"{DATA_MOUNT}/@phermes")
