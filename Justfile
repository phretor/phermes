# List all available recipes
default:
    @just --list

# ── Dev ──────────────────────────────────────────────────────────────────────

# Install dependencies
install:
    uv sync

# Run unit tests
test:
    uv run pytest -q

# Run a single test file or pattern: just test-one disk
test-one pattern:
    uv run pytest -v -k "{{pattern}}"

# Run integration tests (requires root + loop device, Linux only)
test-integration:
    sudo $(which uv) run pytest -m integration -v

# Lint
lint:
    uv run ruff check src/ tests/

# Type check
typecheck:
    uv run ty check src/

# Full pre-commit gate: lint + typecheck + unit tests
check: lint typecheck test

# ── Docker ───────────────────────────────────────────────────────────────────

# Build the phermes-build Docker image
docker-build:
    docker build -t phermes-build .

# Run phermes-build via Docker against a real block device: just docker-run /dev/sdX
docker-run disk:
    #!/usr/bin/env bash
    set -euo pipefail
    # Pre-load kernel targets the container can't modprobe itself
    for m in dm-mod dm-crypt dm-thin-pool btrfs vfat; do
        sudo modprobe "$m" 2>/dev/null || true
    done
    # -v /dev:/dev shares the host device tree so partition nodes are visible; --privileged for cryptsetup/LVM/mount
    sudo docker run --rm --privileged -v /dev:/dev phermes-build {{disk}}

# ── Smoke testing (loop device) ───────────────────────────────────────────────
#
# Workflow:
#   just smoke-create       # create 500G sparse image, attach as loop device
#   just smoke-run          # disk layers only, fast (~10s), Docker by default
#   just smoke-inspect      # show partitions, LVM, Btrfs state
#   just smoke-clean        # tear everything down, delete image
#
#   just smoke-full         # full Proxmox install when ready
#   just smoke-run native=1 # use native phermes-build instead of Docker

_smoke_image := "/tmp/phermes-smoke.img"
_smoke_state := ".smoke"

# Create 500G sparse image and attach as loop device
smoke-create:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -f {{_smoke_state}} ]; then
        echo "error: smoke session already active ($(cat {{_smoke_state}})). Run 'just smoke-clean' first." >&2
        exit 1
    fi
    # The container shares the host kernel but can't modprobe (no module files
    # for the host kernel inside it). Pre-load every target the build needs:
    # dm-thin-pool (LVM thin), dm-crypt (LUKS), btrfs and vfat (mounts).
    for m in dm-mod dm-crypt dm-thin-pool btrfs vfat; do
        sudo modprobe "$m" 2>/dev/null || true
    done
    truncate -s 500G {{_smoke_image}}
    DISK=$(sudo losetup --find --show --partscan {{_smoke_image}})
    echo "$DISK" > {{_smoke_state}}
    echo "Ready: $DISK"

# Disk setup only — skip Proxmox install. Docker by default; override with native=1
smoke-run native="0":
    #!/usr/bin/env bash
    set -euo pipefail
    [ -f {{_smoke_state}} ] || { echo "error: no active smoke session. Run 'just smoke-create' first." >&2; exit 1; }
    DISK=$(cat {{_smoke_state}})
    if [ "{{native}}" = "1" ]; then
        sudo phermes-build "$DISK" --share-size 0 --skip-os-install
    else
        sudo docker run --rm --privileged -v /dev:/dev phermes-build "$DISK" --share-size 0 --skip-os-install
    fi

# Full Proxmox install. Docker by default; override with native=1
smoke-full native="0":
    #!/usr/bin/env bash
    set -euo pipefail
    [ -f {{_smoke_state}} ] || { echo "error: no active smoke session. Run 'just smoke-create' first." >&2; exit 1; }
    DISK=$(cat {{_smoke_state}})
    if [ "{{native}}" = "1" ]; then
        sudo phermes-build "$DISK" --share-size 0
    else
        sudo docker run --rm --privileged -v /dev:/dev phermes-build "$DISK" --share-size 0
    fi

# Show partition table, LVM volumes, and Btrfs filesystems on the smoke disk
smoke-inspect:
    #!/usr/bin/env bash
    set -euo pipefail
    [ -f {{_smoke_state}} ] || { echo "error: no active smoke session." >&2; exit 1; }
    DISK=$(cat {{_smoke_state}})
    echo "=== Block devices ==="
    sudo lsblk "$DISK"
    echo ""
    echo "=== LVM ==="
    sudo pvs 2>/dev/null && sudo vgs 2>/dev/null && sudo lvs 2>/dev/null || echo "(no LVM volumes)"
    echo ""
    echo "=== Btrfs ==="
    sudo btrfs filesystem show 2>/dev/null || echo "(no Btrfs volumes)"

# Tear down smoke session: deactivate VG, close LUKS, detach loop, delete image
smoke-clean:
    #!/usr/bin/env bash
    set -euo pipefail
    # Teardown is the reverse of setup: the pve VG sits on the LUKS mapping, which
    # sits on the loop partition. Closing LUKS before deactivating the VG fails
    # ("device in use"), so order matters.
    #
    # The pve VG is removed when it belongs to a smoke run: its PV is our LUKS
    # mapping, or the PV is missing/unknown (orphaned by a crashed run whose loop
    # is gone). A real Proxmox pve VG sits on a present physical disk and is left
    # untouched. With udev disabled, LVM can also leave an empty /dev/pve behind.
    DISK=$(cat {{_smoke_state}} 2>/dev/null || true)
    pv=$(sudo vgs --noheadings -o pv_name pve 2>/dev/null | tr -d '[:space:]' || true)
    if [ "$pv" = "/dev/mapper/phermes_luks" ] || printf '%s' "$pv" | grep -q 'unknown'; then
        sudo vgchange -an pve >/dev/null 2>&1 || true
        sudo vgremove -f pve >/dev/null 2>&1 || true
    fi
    sudo cryptsetup luksClose phermes_luks 2>/dev/null || true
    if ! sudo vgs pve >/dev/null 2>&1; then
        sudo rm -rf /dev/pve 2>/dev/null || true
    fi
    [ -n "$DISK" ] && sudo losetup -d "$DISK" 2>/dev/null || true
    rm -f {{_smoke_image}} {{_smoke_state}}
    echo "Done."
