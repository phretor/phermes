import os

from phermes_build.runner import run_cmd

# Minimal glibc Linux node (Debian cloud image): runs the Hermes runtime and is
# SSH-able via cloud-init. Dev/smoke only — production ships no OS image. glibc
# (not Alpine/musl) so the Python/ML wheel ecosystem works without source builds.
DEBIAN_CLOUD_URL = (
    "https://cloud.debian.org/images/cloud/bookworm/latest/"
    "debian-12-genericcloud-amd64.qcow2"
)
NODE_IMAGE_PATH = "/var/lib/phermes/linux-node.qcow2"
NODE_HELPER_PATH = "/usr/local/bin/phermes-node"
NODE_VMID = 900

# Must match the vmbr0 config in proxmox.network_interfaces_content.
NODE_IP = "10.10.10.2"
NODE_GATEWAY = "10.10.10.1"
NODE_USER = "dev"
NODE_PASSWORD = "phermes-change-me"


def node_helper_script() -> str:
    """Runtime helper (on the booted Proxmox host): create and boot the Linux
    node as a guest. User/SSH/IP come from Proxmox cloud-init; a vendor-data
    snippet installs uv on first boot so the node can run the Hermes runtime."""
    return f"""#!/bin/sh
set -e
VMID={NODE_VMID}
IMG={NODE_IMAGE_PATH}

[ -f "$IMG" ] || {{ echo "node image not found at $IMG" >&2; exit 1; }}

storage=$(pvesm status -content images 2>/dev/null | awk 'NR==2 {{print $1}}')
storage=${{storage:-local-lvm}}

# cloud-init vendor-data: install uv system-wide so the node can run Hermes.
mkdir -p /var/lib/vz/snippets
cat > /var/lib/vz/snippets/phermes-node-vendor.yaml <<'EOF'
#cloud-config
package_update: true
packages:
  - curl
runcmd:
  - curl -LsSf https://astral.sh/uv/install.sh | env UV_INSTALL_DIR=/usr/local/bin sh
EOF

if ! qm status "$VMID" >/dev/null 2>&1; then
    qm create "$VMID" --name phermes-node --memory 2048 --cores 2 --cpu host \\
        --net0 virtio,bridge=vmbr0 --serial0 socket --vga serial0 \\
        --scsihw virtio-scsi-pci --agent 1
    qm importdisk "$VMID" "$IMG" "$storage"
    vol=$(qm config "$VMID" | sed -n 's/^unused0: //p')
    qm set "$VMID" --scsi0 "$vol"
    qm disk resize "$VMID" scsi0 10G || true
    qm set "$VMID" --ide2 "$storage:cloudinit"
    qm set "$VMID" --boot order=scsi0
    qm set "$VMID" --ciuser {NODE_USER} --cipassword {NODE_PASSWORD}
    qm set "$VMID" --sshkeys /root/.ssh/authorized_keys
    qm set "$VMID" --ipconfig0 ip={NODE_IP}/24,gw={NODE_GATEWAY}
    qm set "$VMID" --nameserver 1.1.1.1
    qm set "$VMID" --cicustom "vendor=local:snippets/phermes-node-vendor.yaml"
fi

qm start "$VMID" 2>/dev/null || true
echo "phermes-node ({NODE_IP}) starting (user: {NODE_USER}); uv installs on first boot."
echo "  console: qm terminal $VMID            # Ctrl-O to exit"
echo "  ssh:     ssh {NODE_USER}@{NODE_IP}    # from this host"
"""


def install_node_vm(mount_point: str, url: str = DEBIAN_CLOUD_URL) -> None:
    """Download the node image into the build and install the boot helper (dev only)."""
    image_dest = os.path.join(mount_point, NODE_IMAGE_PATH.lstrip("/"))
    os.makedirs(os.path.dirname(image_dest), exist_ok=True)
    # -L: the Debian 'latest' link redirects to the versioned image.
    run_cmd(["wget", "-q", "-L", "-O", image_dest, url])

    helper_dest = os.path.join(mount_point, NODE_HELPER_PATH.lstrip("/"))
    os.makedirs(os.path.dirname(helper_dest), exist_ok=True)
    with open(helper_dest, "w") as f:
        f.write(node_helper_script())
    os.chmod(helper_dest, 0o755)
