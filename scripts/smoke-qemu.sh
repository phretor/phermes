#!/usr/bin/env bash
# Boot the smoke disk image in QEMU under OVMF (UEFI). No KVM required — TCG
# emulation boots the same code, just slower. Writes go to a throwaway overlay
# (-snapshot), so the image file is never modified and can be re-inspected.
set -euo pipefail

IMAGE="${1:?usage: smoke-qemu.sh <image> [vnc:0|1] [serial:0|1]}"
VNC="${2:-0}"
SERIAL="${3:-0}"

[ -f "$IMAGE" ] || {
  echo "error: image not found: $IMAGE (run 'just smoke-create' first)" >&2
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

# Quiesce the host so QEMU reads a consistent disk: deactivate any pve VG and
# close the LUKS mapping sitting on this image. -snapshot means QEMU never
# writes to the image regardless.
sudo vgchange -an --config 'activation { udev_sync = 0 udev_rules = 0 }' pve >/dev/null 2>&1 || true
sudo cryptsetup luksClose phermes_luks 2>/dev/null || true

# QEMU runs as the current user; the image is root-owned after the build.
# -snapshot only reads it, so read access is enough — no need to run QEMU as root.
sudo chmod a+r "$IMAGE"

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

if [ "$SERIAL" = "1" ]; then
  args+=(-serial mon:stdio)
fi

if [ "$VNC" = "1" ] || { [ -z "${DISPLAY:-}" ] && [ "$SERIAL" != "1" ]; }; then
  args+=(-display none -vnc :0)
  echo "QEMU serving VNC on localhost:5900 (display :0) — connect a VNC viewer."
else
  echo "QEMU starting with a graphical window."
fi

echo "Booting $IMAGE under OVMF (TCG, no KVM). LUKS passphrase: phermes-change-me"
qemu-system-x86_64 "${args[@]}"
