FROM rust:slim-trixie@sha256:3b199fd6db7d709274df56565b8f4003f9e30d7bfcff9d53148c86053b7855e2 AS ci

LABEL org.opencontainers.image.source="https://github.com/denniskempin/sres"
LABEL org.opencontainers.image.description="Reproducible SRES CI container"

ARG DEBIAN_FRONTEND=noninteractive
ARG NEXTEST_VERSION=0.9.152
ARG TRUNK_VERSION=v0.22.2
ARG DENY_VERSION=0.22.1

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
    && rm -rf /var/lib/apt/lists/*

# Install SHA-256 verified cargo-nextest, trunk, and cargo-deny binaries into /usr/local/bin (unaffected by CARGO_HOME=/cargo).
RUN set -eux; \
    case "$(uname -m)" in \
      x86_64) \
        triple="x86_64-unknown-linux-gnu"; \
        musl_triple="x86_64-unknown-linux-musl"; \
        nextest_sha256="d039b43c3e0958441b6ad0a39c8884d018b3d2e8fb9928f209074b1100b5e17a"; \
        trunk_sha256="26fca8a9952e46cd7d8b8b9a2e2c1786c090fc14b8e10b6678ab4d0f9f1a385f"; \
        deny_sha256="13f7e825e9cb811c2f8de63f29b0f45851dfb53004029f0f2501af524728364d"; \
        ;; \
      aarch64) \
        triple="aarch64-unknown-linux-gnu"; \
        musl_triple="aarch64-unknown-linux-musl"; \
        nextest_sha256="ca1f81a5b86533eb45ab97e0064aa88e925e53aab9a698ca6331a7cad1f85f07"; \
        trunk_sha256="be7b50f2d24e9cfbaea5551be2abf3fbabac10412113213a8efe94448e514414"; \
        deny_sha256="0bac0838a1653ce693e4602b8634d15ca179edd2198e438af9c35ab1c551d509"; \
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
    rm /tmp/trunk.tar.gz; \
    \
    curl --proto '=https' --tlsv1.2 -fsSL -o /tmp/cargo-deny.tar.gz \
      "https://github.com/EmbarkStudios/cargo-deny/releases/download/${DENY_VERSION}/cargo-deny-${DENY_VERSION}-${musl_triple}.tar.gz"; \
    echo "${deny_sha256}  /tmp/cargo-deny.tar.gz" | sha256sum -c -; \
    tar -xzf /tmp/cargo-deny.tar.gz -C /usr/local/bin --strip-components=1 "cargo-deny-${DENY_VERSION}-${musl_triple}/cargo-deny"; \
    rm /tmp/cargo-deny.tar.gz

# Pre-install the pinned nightly toolchain from rust-toolchain.toml.
COPY rust-toolchain.toml /tmp/rust-toolchain.toml
RUN (cd /tmp && rustup toolchain install) \
    && rm /tmp/rust-toolchain.toml

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
    sudo \
    && rm -rf /var/lib/apt/lists/*

# Add rust-analyzer to the pinned nightly toolchain.
COPY rust-toolchain.toml /tmp/rust-toolchain.toml
RUN (cd /tmp && rustup component add rust-analyzer || true) \
    && rm /tmp/rust-toolchain.toml

# Create non-root dev user with passwordless sudo and writable workspace/cache directories.
RUN groupadd --gid 1000 dev \
    && useradd --uid 1000 --gid 1000 --create-home --shell /bin/bash dev \
    && echo "dev ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/dev \
    && chmod 0440 /etc/sudoers.d/dev \
    && mkdir -p /cargo /target /workspace /home/dev/.gemini \
    && chown -R dev:dev /cargo /target /workspace /home/dev \
    && chmod 0755 /home/dev

USER dev
WORKDIR /workspace
