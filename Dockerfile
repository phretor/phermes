FROM debian:bookworm-slim

# Linux toolchain required by phermes-build
RUN apt-get update && apt-get install -y --no-install-recommends \
    btrfs-progs \
    cryptsetup \
    debootstrap \
    dosfstools \
    exfatprogs \
    fdisk \
    gdisk \
    lvm2 \
    udev \
    util-linux \
    wget \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=ghcr.io/astral-sh/uv:0.11.17 /uv /usr/local/bin/uv

WORKDIR /app
COPY .python-version pyproject.toml uv.lock ./
RUN uv sync --frozen --no-dev

COPY src/ ./src/

ENTRYPOINT ["uv", "run", "--no-sync", "phermes-build"]
