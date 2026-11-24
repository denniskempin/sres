ARG CI_IMAGE=ghcr.io/denniskempin/sres-ci:latest
FROM ${CI_IMAGE}

LABEL org.opencontainers.image.source="https://github.com/denniskempin/sres"
LABEL org.opencontainers.image.description="Reproducible SRES DevPod container with Antigravity CLI"

ARG DEBIAN_FRONTEND=noninteractive

# Dev utilities not needed by CI.
RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    jq \
    openssh-client \
    python3 \
    ripgrep \
    && rm -rf /var/lib/apt/lists/*

# Hardening: strip setuid/setgid bits from installed packages (e.g. openssh-client).
RUN find / -xdev -perm /6000 -exec chmod a-s {} + 2>/dev/null || true

# Add rust-analyzer to the pinned nightly toolchain
COPY rust-toolchain.toml /tmp/rust-toolchain.toml
RUN (cd /tmp && rustup component add rust-analyzer || true) \
    && rm /tmp/rust-toolchain.toml \
    && rm -rf /usr/local/cargo/registry /usr/local/cargo/git \
    # Hardening: Read-only toolchain
    && chmod -R a-w,a+rX /usr/local/rustup /usr/local/cargo

# Create non-root dev user and writable workspace/cache directories.
RUN groupadd --gid 1000 dev \
    && useradd --uid 1000 --gid 1000 --create-home --shell /bin/bash dev \
    && mkdir -p /cargo /target /workspace /home/dev/.gemini \
    && chown -R dev:dev /cargo /target /workspace /home/dev \
    && chmod 0755 /home/dev

USER dev
WORKDIR /workspace
