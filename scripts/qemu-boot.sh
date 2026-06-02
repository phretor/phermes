#!/usr/bin/env bash
# Boot a raw disk image in QEMU under OVMF (UEFI). TCG — no KVM required.
# Writes go to a throwaway overlay (-snapshot), so the image is never modified.
#
# Used both on the host (native mode) and as the phermes-qemu container
# entrypoint. Display via QEMU_DISPLAY: vnc (default) | window. For VNC the
# bind address comes from QEMU_VNC (default 127.0.0.1:0; the container sets
# 0.0.0.0:0 so the published port is reachable).
set -euo pipefail

IMAGE="${1:?usage: qemu-boot.sh <image>}"
DISPLAY_MODE="${QEMU_DISPLAY:-vnc}"
VNC_BIND="${QEMU_VNC:-127.0.0.1:0}"

[ -r "$IMAGE" ] || {
  echo "error: image not readable: $IMAGE" >&2
  exit 1
}
command -v qemu-system-x86_64 >/dev/null 2>&1 || {
  echo "error: qemu-system-x86_64 not found — install qemu-system-x86" >&2
  exit 1
}

# OVMF code/vars must be a matching pair; names differ across distros.
pairs=(
  "/usr/share/OVMF/OVMF_CODE_4M.fd:/usr/share/OVMF/OVMF_VARS_4M.fd"
  "/usr/share/OVMF/OVMF_CODE.fd:/usr/share/OVMF/OVMF_VARS.fd"
  "/usr/share/edk2/x64/OVMF_CODE.4m.fd:/usr/share/edk2/x64/OVMF_VARS.4m.fd"
  "/usr/share/edk2-ovmf/x64/OVMF_CODE.fd:/usr/share/edk2-ovmf/x64/OVMF_VARS.fd"
)
code=""
vars_template=""
for pair in "${pairs[@]}"; do
  c="${pair%%:*}"
  v="${pair##*:}"
  if [ -f "$c" ] && [ -f "$v" ]; then
    code="$c"
    vars_template="$v"
    break
  fi
done
[ -n "$code" ] || {
  echo "error: OVMF firmware not found — install ovmf" >&2
  exit 1
}

vars=$(mktemp --suffix=-ovmf-vars.fd)
trap 'rm -f "$vars"' EXIT
cp "$vars_template" "$vars"

args=(
  -machine q35
  -m 4096
  -smp 2
  -no-reboot
  -drive "if=pflash,format=raw,readonly=on,file=$code"
  -drive "if=pflash,format=raw,file=$vars"
  -drive "file=$IMAGE,format=raw,if=virtio"
  -snapshot
)

# Acceleration: prefer KVM, fall back to TCG when /dev/kvm is absent or not
# writable. -cpu host is only valid with KVM, so the two are added together.
if [ -w /dev/kvm ]; then
  args+=(-enable-kvm -cpu host)
  accel="KVM (accelerated)"
else
  accel="TCG (emulated — no accessible /dev/kvm)"
fi

# Report whether the guest will see nested vmx/svm — needed for the inner
# Proxmox VMs to accelerate. Informational only.
nested="unknown"
for p in /sys/module/kvm_intel/parameters/nested /sys/module/kvm_amd/parameters/nested; do
  [ -r "$p" ] || continue
  case "$(cat "$p" 2>/dev/null)" in
  Y | 1) nested="enabled" ;;
  N | 0) nested="disabled" ;;
  esac
  break
done

if [ "$DISPLAY_MODE" = "window" ]; then
  echo "QEMU: graphical window"
else
  args+=(-display none -vnc "$VNC_BIND")
  echo "QEMU: VNC on display :0 (port 5900)"
fi

echo "Acceleration: $accel"
if [ "${accel%% *}" = "KVM" ]; then
  echo "Host nested virtualization: $nested (inner Proxmox VMs accelerate only when enabled)"
fi
echo "Booting $IMAGE under OVMF. LUKS passphrase: phermes-change-me"
# Run QEMU in the background and forward termination signals so Ctrl-C (or
# `docker stop`) tears it down cleanly instead of orphaning it.
qemu-system-x86_64 "${args[@]}" &
qemu_pid=$!
trap 'kill -TERM "$qemu_pid" 2>/dev/null || true' INT TERM
wait "$qemu_pid"
