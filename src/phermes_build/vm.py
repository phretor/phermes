import json
import os

from phermes_build.models import AcquisitionMode, VMConfig, VMFlavor
from phermes_build.runner import run_cmd

_VM_IDS: dict[VMFlavor, int] = {
    VMFlavor.MACOS: 100,
    VMFlavor.WINDOWS: 101,
    VMFlavor.LINUX: 102,
}

_VM_DISK_GB: dict[VMFlavor, int] = {
    VMFlavor.MACOS: 120,
    VMFlavor.WINDOWS: 100,
    VMFlavor.LINUX: 40,
}


def vm_id_for_flavor(flavor: VMFlavor) -> int:
    return _VM_IDS[flavor]


def proxmox_vm_config(flavor: VMFlavor, vm_id: int, disk_gb: int) -> str:
    base = (
        f"vmid: {vm_id}\n"
        f"machine: q35\n"
        f"bios: ovmf\n"
        f"boot: order=scsi0\n"
        f"scsihw: virtio-scsi-pci\n"
        f"scsi0: local-lvm:vm-{vm_id}-disk-0,size={disk_gb}G\n"
        f"cores: 4\n"
        f"cpu: host\n"
    )
    if flavor == VMFlavor.MACOS:
        return base + (
            "vga: vmware\n"
            "net0: vmxnet3,bridge=vmbr0\n"
            "args: -cpu Penryn,kvm=on,vendor=GenuineIntel,"
            "+kvm_pv_unhalt,+kvm_pv_eoi,+hypervisor,+invtsc\n"
        )
    return base + (
        "vga: virtio\n"
        "net0: virtio,bridge=vmbr0\n"
    )


def import_vm(cfg: VMConfig, vm_id: int, storage: str = "local-lvm") -> None:
    if cfg.mode != AcquisitionMode.IMPORT or cfg.image_path is None:
        raise ValueError("import_vm requires mode=IMPORT and image_path set")
    run_cmd(["qm", "importdisk", str(vm_id), cfg.image_path, storage])


def schedule_vm_acquisition(cfg: VMConfig, flag_dir: str = "/var/lib/phermes") -> None:
    os.makedirs(flag_dir, exist_ok=True)
    flag_path = os.path.join(flag_dir, f"acquire_{cfg.flavor.value}.json")
    with open(flag_path, "w") as f:
        json.dump({"flavor": cfg.flavor.value, "mode": cfg.mode.value}, f)


def provision_vm(cfg: VMConfig, storage: str = "local-lvm") -> None:
    vm_id = vm_id_for_flavor(cfg.flavor)
    disk_gb = _VM_DISK_GB[cfg.flavor]

    conf = proxmox_vm_config(cfg.flavor, vm_id, disk_gb)
    conf_path = f"/etc/pve/qemu-server/{vm_id}.conf"
    with open(conf_path, "w") as f:
        f.write(conf)

    if cfg.mode == AcquisitionMode.IMPORT:
        import_vm(cfg, vm_id, storage)
    elif cfg.mode in (AcquisitionMode.DOWNLOAD, AcquisitionMode.SKIP):
        schedule_vm_acquisition(cfg)
