FROM rust:slim-trixie@sha256:3b199fd6db7d709274df56565b8f4003f9e30d7bfcff9d53148c86053b7855e2 AS ci

LABEL org.opencontainers.image.source="https://github.com/denniskempin/sres"
LABEL org.opencontainers.image.description="Reproducible SRES CI container"

ARG DEBIAN_FRONTEND=noninteractive
ARG NEXTEST_VERSION=0.9.152
ARG TRUNK_VERSION=v0.22.2

# SRES Linux GUI/audio libs and Mesa software GL/Vulkan (llvmpipe/lavapipe) for headless egui_kittest.
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    libasound2-dev \
    libegl-mesa0 \
    libgl1-mesa-dri \
    libxkbcommon-x11-0 \
    mesa-vulkan-drivers \
    pkg-config \
    xauth \
    xvfb \
    xz-utils \
    && rm -rf /var/lib/apt/lists/* \
    # Hardening: strip setuid/setgid bits from base and installed packages.
    && (find / -xdev -perm /6000 -exec chmod a-s {} + 2>/dev/null || true)

# Install SHA-256 verified cargo-nextest and trunk binaries into /usr/local/bin (unaffected by CARGO_HOME=/cargo).
RUN set -eux; \
    case "$(uname -m)" in \
      x86_64) \
        triple="x86_64-unknown-linux-gnu"; \
        nextest_sha256="d039b43c3e0958441b6ad0a39c8884d018b3d2e8fb9928f209074b1100b5e17a"; \
        trunk_sha256="26fca8a9952e46cd7d8b8b9a2e2c1786c090fc14b8e10b6678ab4d0f9f1a385f"; \
        ;; \
      aarch64) \
        triple="aarch64-unknown-linux-gnu"; \
        nextest_sha256="ca1f81a5b86533eb45ab97e0064aa88e925e53aab9a698ca6331a7cad1f85f07"; \
        trunk_sha256="be7b50f2d24e9cfbaea5551be2abf3fbabac10412113213a8efe94448e514414"; \
        ;; \
      *) echo "Unsupported architecture: $(uname -m)" >&2; exit 1 ;; \
    esac; \
    \
    curl --proto '=https' --tlsv1.2 -fsSL -o /tmp/nextest.tar.gz \
      "https://get.nexte.st/${NEXTEST_VERSION}/${triple}.tar.gz"; \
    echo "${nextest_sha256}  /tmp/nextest.tar.gz" | sha256sum -c -; \
    tar -xzf /tmp/nextest.tar.gz -C /usr/local/bin cargo-nextest; \
    rm /tmp/nextest.tar.gz; \
    \
    curl --proto '=https' --tlsv1.2 -fsSL -o /tmp/trunk.tar.gz \
      "https://github.com/trunk-rs/trunk/releases/download/${TRUNK_VERSION}/trunk-${triple}.tar.gz"; \
    echo "${trunk_sha256}  /tmp/trunk.tar.gz" | sha256sum -c -; \
    tar -xzf /tmp/trunk.tar.gz -C /usr/local/bin trunk; \
    rm /tmp/trunk.tar.gz

# Pre-install the pinned nightly toolchain from rust-toolchain.toml and lock down toolchain dirs read-only.
COPY rust-toolchain.toml /tmp/rust-toolchain.toml
RUN (cd /tmp && rustup toolchain install) \
    && rm /tmp/rust-toolchain.toml \
    && chmod -R a-w,a+rX /usr/local/rustup /usr/local/cargo

ENV CARGO_HOME=/cargo \
    TRUNK_SKIP_VERSION_CHECK=true \
    LIBGL_ALWAYS_SOFTWARE=1 \
    WGPU_BACKEND=gl,vulkan


FROM ci AS dev

LABEL org.opencontainers.image.source="https://github.com/denniskempin/sres"
LABEL org.opencontainers.image.description="Reproducible SRES DevPod container"

# Dev utilities not needed by CI.
RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    jq \
    openssh-client \
    python3 \
    ripgrep \
    && rm -rf /var/lib/apt/lists/* \
    # Hardening: strip setuid/setgid bits from installed packages (e.g. openssh-client).
    && (find / -xdev -perm /6000 -exec chmod a-s {} + 2>/dev/null || true)

# Add rust-analyzer to the pinned nightly toolchain and keep toolchain dirs read-only.
COPY rust-toolchain.toml /tmp/rust-toolchain.toml
RUN (cd /tmp && rustup component add rust-analyzer || true) \
    && rm /tmp/rust-toolchain.toml \
    && chmod -R a-w,a+rX /usr/local/rustup /usr/local/cargo

# Create non-root dev user and writable workspace/cache directories.
RUN groupadd --gid 1000 dev \
    && useradd --uid 1000 --gid 1000 --create-home --shell /bin/bash dev \
    && mkdir -p /cargo /target /workspace /home/dev/.gemini \
    && chown -R dev:dev /cargo /target /workspace /home/dev \
    && chmod 0755 /home/dev

USER dev
WORKDIR /workspace
