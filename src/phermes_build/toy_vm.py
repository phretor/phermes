import os

from phermes_build.runner import run_cmd

# Tiny Alpine cloud image used to prove nested virtualization in smoke testing.
# Dev/smoke only — production never bundles an OS image.
ALPINE_QCOW2_URL = (
    "https://dl-cdn.alpinelinux.org/alpine/v3.21/releases/cloud/"
    "nocloud_alpine-3.21.7-x86_64-bios-cloudinit-r0.qcow2"
)
TOY_IMAGE_PATH = "/var/lib/phermes/toy-linux.qcow2"
TOY_HELPER_PATH = "/usr/local/bin/phermes-toy-vm"
TOY_VMID = 900


def toy_helper_script() -> str:
    """Runtime helper (runs on the booted Proxmox host): register and boot the
    bundled toy Linux as a guest, then attach to its serial console. Proves the
    L1 Proxmox host can run an L2 guest with nested KVM."""
    return f"""#!/bin/sh
set -e
VMID={TOY_VMID}
IMG={TOY_IMAGE_PATH}

[ -f "$IMG" ] || {{ echo "toy image not found at $IMG" >&2; exit 1; }}

# Pick a storage that accepts VM images (local-lvm on a normal install).
storage=$(pvesm status -content images 2>/dev/null | awk 'NR==2 {{print $1}}')
storage=${{storage:-local-lvm}}

if ! qm status "$VMID" >/dev/null 2>&1; then
    # cpu host exposes the nested virt extensions; serial console so we can watch it.
    qm create "$VMID" --name toy-linux --memory 512 --cores 1 \\
        --cpu host --serial0 socket --vga serial0 --scsihw virtio-scsi-pci
    qm importdisk "$VMID" "$IMG" "$storage"
    vol=$(qm config "$VMID" | sed -n 's/^unused0: //p')
    qm set "$VMID" --scsi0 "$vol"
    qm set "$VMID" --boot order=scsi0
fi

qm start "$VMID" 2>/dev/null || true
echo "=== toy-linux serial console — exit with Ctrl-O ==="
qm terminal "$VMID"
"""


def install_toy_vm(mount_point: str, url: str = ALPINE_QCOW2_URL) -> None:
    """Download the toy image into the build and install the boot helper (dev only)."""
    image_dest = os.path.join(mount_point, TOY_IMAGE_PATH.lstrip("/"))
    os.makedirs(os.path.dirname(image_dest), exist_ok=True)
    run_cmd(["wget", "-qO", image_dest, url])

    helper_dest = os.path.join(mount_point, TOY_HELPER_PATH.lstrip("/"))
    os.makedirs(os.path.dirname(helper_dest), exist_ok=True)
    with open(helper_dest, "w") as f:
        f.write(toy_helper_script())
    os.chmod(helper_dest, 0o755)
