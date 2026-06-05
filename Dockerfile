# ── Stage 1: build phermesd + phermesctl binaries from the working tree ──────
FROM rust:1.95-slim-bookworm AS phermesd-builder
WORKDIR /work
COPY phermesd/ ./
RUN cargo build --release --bin phermesd --bin phermesctl

# ── Stage 2: phermes-build runtime image (toolchain + Python + phermesd bins) ─
FROM debian:bookworm-slim

# Linux toolchain required by phermes-build (host-side disk operations)
RUN apt-get update && apt-get install -y --no-install-recommends \
    btrfs-progs \
    cryptsetup \
    debootstrap \
    dosfstools \
    exfatprogs \
    fdisk \
    lvm2 \
    udev \
    util-linux \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=ghcr.io/astral-sh/uv:0.11.17 /uv /usr/local/bin/uv

# phermesd binaries from the builder stage; phermes-build copies them into the chroot
COPY --from=phermesd-builder /work/target/release/phermesd /app/bin/phermesd
COPY --from=phermesd-builder /work/target/release/phermesctl /app/bin/phermesctl

WORKDIR /app
COPY .python-version pyproject.toml uv.lock ./
# Install deps only — cached layer unaffected by src/ changes
RUN uv sync --frozen --no-dev --no-install-project

COPY src/ ./src/
# Install the local package now that source files are present
RUN uv sync --frozen --no-dev

ENTRYPOINT ["uv", "run", "--no-sync", "phermes-build"]
