#!/usr/bin/env bash
# Boot the smoke image in QEMU under OVMF. Docker by default (no host QEMU or
# OVMF install needed); native=1 runs QEMU directly on the host.
#
# The actual QEMU launch lives in qemu-boot.sh, shared between native mode and
# the phermes-qemu container so the boot configuration stays in one place.
set -euo pipefail

IMAGE="${1:?usage: smoke-qemu.sh <image> [native:0|1]}"
NATIVE="${2:-0}"
HERE="$(cd "$(dirname "$0")" && pwd)"

[ -f "$IMAGE" ] || {
  echo "error: image not found: $IMAGE (run 'just smoke-create' first)" >&2
  exit 1
}

# Quiesce the host so QEMU reads a consistent disk: deactivate any pve VG and
# close the LUKS mapping sitting on this image. -snapshot never writes it anyway.
sudo vgchange -an --config 'activation { udev_sync = 0 udev_rules = 0 }' pve >/dev/null 2>&1 || true
sudo cryptsetup luksClose phermes_luks 2>/dev/null || true
# The image is root-owned after the build; -snapshot only reads it.
sudo chmod a+r "$IMAGE"

if [ "$NATIVE" = "1" ]; then
  if [ -n "${DISPLAY:-}" ]; then
    export QEMU_DISPLAY=window
  else
    export QEMU_DISPLAY=vnc
  fi
  exec bash "$HERE/qemu-boot.sh" "$IMAGE"
fi

# Docker mode: build the lean qemu image on first use, then boot with VNC.
if ! sudo docker image inspect phermes-qemu >/dev/null 2>&1; then
  echo "Building phermes-qemu image (first run)…"
  sudo docker build -f Dockerfile.qemu -t phermes-qemu .
fi

echo "Booting in Docker — connect a VNC viewer to localhost:5900"
exec sudo docker run --rm -p 5900:5900 \
  -e QEMU_VNC=0.0.0.0:0 \
  -v "$IMAGE:/image.img:ro" \
  phermes-qemu /image.img
