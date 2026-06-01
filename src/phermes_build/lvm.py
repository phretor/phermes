from phermes_build.runner import run_cmd

ROOT_LV_GB = 30


def compute_lvm_sizes(total_lvm_gb: int) -> dict[str, int]:
    return {"root_gb": ROOT_LV_GB, "pool_gb": total_lvm_gb - ROOT_LV_GB}


def create_pv(device: str) -> None:
    run_cmd(["pvcreate", "--force", device])


def create_vg(device: str, vg_name: str = "pve") -> None:
    run_cmd(["vgcreate", vg_name, device])


def create_root_lv(vg_name: str, size_gb: int = ROOT_LV_GB, lv_name: str = "root") -> str:
    run_cmd(["lvcreate", "-L", f"{size_gb}G", "-n", lv_name, vg_name])
    return f"/dev/{vg_name}/{lv_name}"


def create_thin_pool(vg_name: str, pool_name: str, size_gb: int) -> str:
    run_cmd(["lvcreate", "--thin", "-L", f"{size_gb}G", f"{vg_name}/{pool_name}"])
    return f"{vg_name}/{pool_name}"


def create_thin_volume(vg_name: str, pool_name: str, vol_name: str, size_gb: int) -> str:
    run_cmd(
        ["lvcreate", "--virtualsize", f"{size_gb}G", "--thin",
         f"{vg_name}/{pool_name}", "-n", vol_name]
    )
    return f"/dev/{vg_name}/{vol_name}"


BTRFS_LV_NAME = "btrfs-data"


def create_btrfs_lv(vg_name: str, data_gb: int) -> str:
    run_cmd(["lvcreate", "-L", f"{data_gb}G", "-n", BTRFS_LV_NAME, vg_name])
    return f"/dev/{vg_name}/{BTRFS_LV_NAME}"


def setup_lvm(mapper_device: str, total_lvm_gb: int, vg_name: str = "pve") -> dict[str, str]:
    sizes = compute_lvm_sizes(total_lvm_gb)
    create_pv(mapper_device)
    create_vg(mapper_device, vg_name)
    root_lv = create_root_lv(vg_name, sizes["root_gb"])
    pool = create_thin_pool(vg_name, "data", sizes["pool_gb"])
    return {"root_lv": root_lv, "thin_pool": pool, "vg": vg_name}
