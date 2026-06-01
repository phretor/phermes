import os
from typing import Annotated

import typer
from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn

from phermes_build import btrfs, exfat, host_config, luks, lvm, partitioner, proxmox, vm
from phermes_build.disk import compute_layout
from phermes_build.firstboot import write_firstboot_flag, write_motd
from phermes_build.models import AcquisitionMode, BuildConfig, VMConfig, VMFlavor
from phermes_build.runner import CommandError, run_cmd

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
    import_vm_macos: Annotated[
        str | None, typer.Option(help="Path to macOS QCOW2 to import")
    ] = None,
    download_vm: Annotated[
        list[str] | None, typer.Option(help="VM flavors to download at build time")
    ] = None,
) -> None:
    validate_disk_path(disk)

    cfg = BuildConfig(
        disk=disk,
        share_size_gb=share_size,
        share_encrypted=share_encrypted,
    )

    if import_vm_macos:
        cfg.vms.append(
            VMConfig(
                flavor=VMFlavor.MACOS,
                mode=AcquisitionMode.IMPORT,
                image_path=import_vm_macos,
            )
        )
    for flavor_name in download_vm or []:
        cfg.vms.append(
            VMConfig(
                flavor=VMFlavor(flavor_name),
                mode=AcquisitionMode.DOWNLOAD,
            )
        )

    layout = compute_layout(disk, cfg.share_size_gb, cfg.share_encrypted)

    steps = [
        ("Partitioning SSD", lambda: partitioner.create_partition_table(layout)),
        ("Creating LUKS2 container", lambda: _setup_luks(layout, cfg)),
        ("Setting up LVM", lambda: _setup_lvm(layout)),
        ("Formatting Btrfs data partition", lambda: _setup_btrfs(layout)),
        ("Formatting exFAT share", lambda: _setup_exfat(layout)),
        ("Installing Proxmox VE", lambda: _install_proxmox(layout)),
        ("Configuring PHermes host", lambda: _configure_host(layout, cfg)),
        ("Provisioning VMs", lambda: _provision_vms(cfg)),
        ("Writing first-boot flag", lambda: _write_firstboot()),
    ]

    with Progress(SpinnerColumn(), TextColumn("{task.description}"), console=console) as progress:
        for description, step_fn in steps:
            task = progress.add_task(description)
            try:
                step_fn()
                progress.update(task, description=f"[green]✓[/green] {description}")
            except CommandError as e:
                progress.stop()
                console.print(f"[red]✗ {description} failed:[/red] {e}")
                raise typer.Exit(1) from e
            finally:
                progress.remove_task(task)

    console.print("\n[bold green]PHermes SSD ready.[/bold green]")
    console.print("Safely eject and boot the target machine.")
    console.print("Connect from any browser: [bold]https://phermes.local[/bold]")


def _setup_luks(layout, cfg: BuildConfig) -> None:
    luks_part = partitioner.partition_path(layout.disk, 3)
    luks.format_luks(luks_part, TEMP_PASSPHRASE)
    luks.open_luks(luks_part, LUKS_NAME, TEMP_PASSPHRASE)


def _setup_lvm(layout) -> None:
    mapper = luks.mapper_path(LUKS_NAME)
    lvm.setup_lvm(mapper, layout.lvm_gb)
    proxmox.format_root_lv("/dev/pve/root")


def _setup_btrfs(layout) -> None:
    data_part = f"/dev/mapper/{LUKS_NAME}_data"
    btrfs.format_btrfs(data_part)
    btrfs.mount_btrfs(data_part, DATA_MOUNT)
    btrfs.create_subvolumes(DATA_MOUNT)


def _setup_exfat(layout) -> None:
    if layout.share_gb == 0 or layout.share_encrypted:
        return
    share_part = partitioner.partition_path(layout.disk, 4)
    exfat.format_exfat(share_part)


def _install_proxmox(layout) -> None:
    os.makedirs(PVE_ROOT_MOUNT, exist_ok=True)
    run_cmd(["mount", "/dev/pve/root", PVE_ROOT_MOUNT])
    luks_part = partitioner.partition_path(layout.disk, 3)
    proxmox.install_proxmox(PVE_ROOT_MOUNT, layout.disk, luks_part)


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


def _provision_vms(cfg: BuildConfig) -> None:
    for vm_cfg in cfg.vms:
        vm.provision_vm(vm_cfg)


def _write_firstboot() -> None:
    write_firstboot_flag(f"{DATA_MOUNT}/@phermes")
