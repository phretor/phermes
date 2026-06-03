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
        sudo phermes-build "$DISK" --share-size 0 --skip-os-install --verbose --dev-credentials
    else
        # Bind-mount live src so the container always runs current code (the
        # image only provides the toolchain + deps); run 'just docker-build'
        # after changing dependencies.
        sudo docker run --rm --privileged -v /dev:/dev -v "$PWD/src:/app/src:ro" phermes-build "$DISK" --share-size 0 --skip-os-install --verbose --dev-credentials
    fi

# Full Proxmox install (verbose). Docker by default; native=1 runs on the host
smoke-full native="0": (_smoke-build native "")

# Like smoke-full but also bundles a Debian Linux node guest (run phermes-node after boot)
smoke-full-node native="0": (_smoke-build native "--linux-node")

# Shared build invocation; `extra` carries extra phermes-build flags (e.g. --toy-vm)
[private]
_smoke-build native extra:
    #!/usr/bin/env bash
    set -euo pipefail
    [ -f {{_smoke_state}} ] || { echo "error: no active smoke session. Run 'just smoke-create' first." >&2; exit 1; }
    DISK=$(cat {{_smoke_state}})
    # Dev SSH key so root login works on the booted host (just smoke-ssh).
    mkdir -p .dev-ssh
    [ -f .dev-ssh/id_ed25519 ] || ssh-keygen -t ed25519 -N "" -C phermes-dev -f .dev-ssh/id_ed25519 >/dev/null
    PUBKEY=$(cat .dev-ssh/id_ed25519.pub)
    if [ "{{native}}" = "1" ]; then
        sudo PHERMES_DEV_SSH_PUBKEY="$PUBKEY" phermes-build "$DISK" --share-size 0 --verbose --dev-credentials {{extra}}
    else
        # Bind-mount live src so the container always runs current code (the
        # image only provides the toolchain + deps); run 'just docker-build'
        # after changing dependencies.
        sudo docker run --rm --privileged -v /dev:/dev -v "$PWD/src:/app/src:ro" -e PHERMES_DEV_SSH_PUBKEY="$PUBKEY" phermes-build "$DISK" --share-size 0 --verbose --dev-credentials {{extra}}
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

# Open an interactive shell in the build container (all tools, host /dev shared)
smoke-shell:
    sudo docker run --rm -it --privileged -v /dev:/dev --entrypoint /bin/bash phermes-build

# Mount every partition in a container and assert expected content (pass/fail)
smoke-verify:
    #!/usr/bin/env bash
    set -euo pipefail
    [ -f {{_smoke_state}} ] || { echo "error: no active smoke session." >&2; exit 1; }
    DISK=$(cat {{_smoke_state}})
    sudo docker run --rm --privileged -v /dev:/dev \
        -v "$PWD/scripts/smoke-verify.sh:/verify.sh:ro" \
        --entrypoint /bin/bash phermes-build /verify.sh "$DISK"

# Boot the smoke image in QEMU under OVMF (UEFI, KVM when available). Docker by default; native=1 host, serial=1 console
smoke-qemu native="0" serial="0":
    bash scripts/smoke-qemu.sh {{_smoke_image}} "{{native}}" "{{serial}}"

# SSH into the booted host as root (dev key); pass a command to run it: just smoke-ssh "qm list"
smoke-ssh command="":
    #!/usr/bin/env bash
    set -euo pipefail
    args=(-p 2200 -i .dev-ssh/id_ed25519 -o IdentitiesOnly=yes
          -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -o LogLevel=ERROR)
    if [ -n "{{command}}" ]; then
        ssh "${args[@]}" root@localhost "{{command}}"
    else
        ssh "${args[@]}" root@localhost
    fi

# Boot the Linux node VM and attach to its serial console (exit qm terminal with Ctrl-O)
smoke-node:
    ssh -t -p 2200 -i .dev-ssh/id_ed25519 -o IdentitiesOnly=yes \
        -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -o LogLevel=ERROR \
        root@localhost "phermes-node"

# SSH into the Linux node VM (dev@10.10.10.2) via the PHermes host as a jump
smoke-node-ssh command="":
    #!/usr/bin/env bash
    set -euo pipefail
    base=(-i .dev-ssh/id_ed25519 -o IdentitiesOnly=yes
          -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -o LogLevel=ERROR)
    jump="ssh ${base[*]} -W %h:%p -p 2200 root@localhost"
    args=("${base[@]}" -o ProxyCommand="$jump")
    if [ -n "{{command}}" ]; then
        ssh "${args[@]}" dev@10.10.10.2 "{{command}}"
    else
        ssh "${args[@]}" dev@10.10.10.2
    fi

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

# ── phermesd (Rust orchestrator) ─────────────────────────────────────────────

# Build the Rust orchestrator
phermesd-build:
    cd phermesd && cargo build

# Lint the orchestrator (clippy, deny warnings)
phermesd-check:
    cd phermesd && cargo clippy --all-targets --all-features -- -D warnings

# Unit + integration tests (no QEMU)
phermesd-test:
    cd phermesd && cargo test

# Gated end-to-end boot (needs /dev/kvm, OVMF, a disk + bridge)
phermesd-e2e disk bridge="vmbr0":
    cd phermesd && PHERMESD_E2E_DISK={{disk}} PHERMESD_E2E_BRIDGE={{bridge}} \
        cargo test --test e2e_boot -- --ignored --nocapture
