#!/usr/bin/env bash
# Boot the smoke image in QEMU under OVMF. Docker by default (no host QEMU or
# OVMF install needed); native=1 runs QEMU directly on the host.
#
# QEMU needs no elevated privileges. Native mode runs entirely as your user.
# Docker mode runs QEMU unprivileged inside the container — the only root is
# Docker-daemon access (sudo, or membership in the docker group).
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
# No host teardown needed: -snapshot opens the image read-only, so the build's
# LUKS/LVM mappings can stay active. If a previous build left the image
# mid-write, run 'just smoke-clean' and rebuild first.

if [ "$NATIVE" = "1" ]; then
  if [ -n "${DISPLAY:-}" ]; then
    export QEMU_DISPLAY=window
  else
    export QEMU_DISPLAY=vnc
  fi
  exec bash "$HERE/qemu-boot.sh" "$IMAGE"
fi

# Reach the Docker daemon without sudo when the user is in the docker group,
# falling back to sudo otherwise — QEMU itself needs no root.
if docker info >/dev/null 2>&1; then
  docker=(docker)
else
  docker=(sudo docker)
fi

# Docker mode: build the lean qemu image on first use, then boot with VNC.
if ! "${docker[@]}" image inspect phermes-qemu >/dev/null 2>&1; then
  echo "Building phermes-qemu image (first run)…"
  "${docker[@]}" build -f Dockerfile.qemu -t phermes-qemu .
fi

# Run QEMU as the invoking user, not container-root, so the process is
# unprivileged and killable on the host. --init gives a real PID 1 that reaps
# and forwards signals, so Ctrl-C tears the container down cleanly.
run_args=(--rm --init -p 5900:5900 --user "$(id -u):$(id -g)")

# Pass /dev/kvm through when present so QEMU accelerates; grant its group to the
# non-root container user so it can open the device. Without it: TCG.
if [ -e /dev/kvm ]; then
  run_args+=(--device /dev/kvm --group-add "$(stat -c '%g' /dev/kvm)")
fi

echo "Booting in Docker — connect a VNC viewer to localhost:5900"
exec "${docker[@]}" run "${run_args[@]}" \
  -e QEMU_VNC=0.0.0.0:0 \
  -v "$IMAGE:/image.img:ro" \
  phermes-qemu /image.img
