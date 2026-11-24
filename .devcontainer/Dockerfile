FROM debian:trixie-slim@sha256:a72b4433de1cf7d6bae0c4cee74d0547f58601a643b3375f070c51600d63b6b2

LABEL org.opencontainers.image.source="https://github.com/denniskempin/sres"
LABEL org.opencontainers.image.description="Reproducible SRES DevPod and CI container with Antigravity CLI"

ARG DEBIAN_FRONTEND=noninteractive
ARG RUSTUP_VERSION=1.29.3
ARG NEXTEST_VERSION=0.9.152
ARG TRUNK_VERSION=v0.22.2
ARG WASM_BINDGEN_VERSION=0.2.128

# CLI/dev utilities, SRES Linux GUI/audio libs, and Mesa software GL/Vulkan (llvmpipe/lavapipe) for headless egui_kittest.
RUN apt-get update && apt-get install -y --no-install-recommends \
    binaryen \
    bubblewrap \
    build-essential \
    ca-certificates \
    curl \
    fd-find \
    gh \
    git \
    jq \
    less \
    libasound2-dev \
    libegl-mesa0 \
    libegl1 \
    libgl1 \
    libgl1-mesa-dri \
    libvulkan1 \
    libwayland-client0 \
    libx11-6 \
    libxcb1 \
    libxcursor1 \
    libxi6 \
    libxkbcommon-x11-0 \
    libxkbcommon0 \
    libxrandr2 \
    libxrender1 \
    mesa-vulkan-drivers \
    openssh-client \
    pkg-config \
    procps \
    python3 \
    ripgrep \
    xauth \
    xvfb \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

ENV PATH="/home/dev/.local/bin:/usr/local/bin:/usr/bin:/bin" \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/cargo \
    TRUNK_SKIP_VERSION_CHECK=true \
    LIBGL_ALWAYS_SOFTWARE=1 \
    WGPU_BACKEND=gl,vulkan

# Ensure SSH login shells (e.g. `devpod ssh`) and non-login bash shells inherit container tool paths and headless GPU env.
RUN printf '%s\n' \
    'export PATH="/home/dev/.local/bin:/usr/local/bin:${PATH}"' \
    'export RUSTUP_HOME="/usr/local/rustup"' \
    'export CARGO_HOME="${CARGO_HOME:-/cargo}"' \
    'export TRUNK_SKIP_VERSION_CHECK="true"' \
    'export LIBGL_ALWAYS_SOFTWARE="1"' \
    'export WGPU_BACKEND="gl,vulkan"' \
    > /etc/profile.d/sres-dev.sh \
    && chmod 0644 /etc/profile.d/sres-dev.sh \
    && printf '\n[ -f /etc/profile.d/sres-dev.sh ] && . /etc/profile.d/sres-dev.sh\n' >> /etc/bash.bashrc

# Install checksum-pinned rustup, cargo-nextest, trunk, and wasm-bindgen into /usr/local/bin
# so mounting a persistent cache volume at /cargo never shadows them.
RUN set -eux; \
    arch="$(uname -m)"; \
    case "$arch" in \
      x86_64) \
        rust_triple="x86_64-unknown-linux-gnu"; \
        rustup_sha256="b1c8cb14e30889e7e2999e6b69e62d9e9b9ffc0f4df8180564d306d9680e6c55"; \
        nextest_sha256="d039b43c3e0958441b6ad0a39c8884d018b3d2e8fb9928f209074b1100b5e17a"; \
        trunk_sha256="26fca8a9952e46cd7d8b8b9a2e2c1786c090fc14b8e10b6678ab4d0f9f1a385f"; \
        wb_triple="x86_64-unknown-linux-musl"; \
        wb_sha256="b51f0208fdff83515a787bd8ab9ac5865ed84dabb66d0c709957bb59793c645f"; \
        ;; \
      aarch64) \
        rust_triple="aarch64-unknown-linux-gnu"; \
        rustup_sha256="9f608c142e0e38a78957d0661ba1dfaf0b6fb37184cfaa4b9a0aa57af6184282"; \
        nextest_sha256="ca1f81a5b86533eb45ab97e0064aa88e925e53aab9a698ca6331a7cad1f85f07"; \
        trunk_sha256="be7b50f2d24e9cfbaea5551be2abf3fbabac10412113213a8efe94448e514414"; \
        wb_triple="aarch64-unknown-linux-gnu"; \
        wb_sha256="2f61267bbef279c2d6357b199649468b073223febde4ae108d30ac3d6b5540af"; \
        ;; \
      *) echo "Unsupported architecture: $arch" >&2; exit 1 ;; \
    esac; \
    \
    curl -fsSL -o /tmp/rustup-init "https://static.rust-lang.org/rustup/archive/${RUSTUP_VERSION}/${rust_triple}/rustup-init"; \
    echo "${rustup_sha256}  /tmp/rustup-init" | sha256sum -c -; \
    chmod +x /tmp/rustup-init; \
    CARGO_HOME=/usr/local/cargo /tmp/rustup-init -y --no-modify-path --profile minimal --default-toolchain none; \
    rm /tmp/rustup-init; \
    \
    curl -fsSL -o /tmp/nextest.tar.gz "https://get.nexte.st/${NEXTEST_VERSION}/${rust_triple}.tar.gz"; \
    echo "${nextest_sha256}  /tmp/nextest.tar.gz" | sha256sum -c -; \
    tar -xzf /tmp/nextest.tar.gz -C /usr/local/bin cargo-nextest; \
    rm /tmp/nextest.tar.gz; \
    \
    curl -fsSL -o /tmp/trunk.tar.gz "https://github.com/trunk-rs/trunk/releases/download/${TRUNK_VERSION}/trunk-${rust_triple}.tar.gz"; \
    echo "${trunk_sha256}  /tmp/trunk.tar.gz" | sha256sum -c -; \
    tar -xzf /tmp/trunk.tar.gz -C /usr/local/bin trunk; \
    rm /tmp/trunk.tar.gz; \
    \
    curl -fsSL -o /tmp/wasm-bindgen.tar.gz "https://github.com/rustwasm/wasm-bindgen/releases/download/${WASM_BINDGEN_VERSION}/wasm-bindgen-${WASM_BINDGEN_VERSION}-${wb_triple}.tar.gz"; \
    echo "${wb_sha256}  /tmp/wasm-bindgen.tar.gz" | sha256sum -c -; \
    tar -xzf /tmp/wasm-bindgen.tar.gz --strip-components=1 -C /usr/local/bin \
      "wasm-bindgen-${WASM_BINDGEN_VERSION}-${wb_triple}/wasm-bindgen" \
      "wasm-bindgen-${WASM_BINDGEN_VERSION}-${wb_triple}/wasm-bindgen-test-runner"; \
    rm /tmp/wasm-bindgen.tar.gz; \
    \
    groupadd --gid 1000 dev; \
    useradd --uid 1000 --gid 1000 --create-home --shell /bin/bash dev; \
    mkdir -p /cargo /target /workspace /home/dev/.gemini /home/dev/.local/bin; \
    chown -R dev:dev /cargo /target /workspace /home/dev; \
    chmod 0755 /home/dev; \
    chmod 0777 /cargo /target /workspace

# Install the pinned nightly toolchain, components, and wasm32 target from rust-toolchain.toml.
COPY rust-toolchain.toml /etc/sres-rust-toolchain.toml
RUN set -eux; \
    mkdir -p /tmp/toolchain-setup; \
    cp /etc/sres-rust-toolchain.toml /tmp/toolchain-setup/rust-toolchain.toml; \
    active_tc="$(cd /tmp/toolchain-setup && CARGO_HOME=/usr/local/cargo /usr/local/cargo/bin/rustup show active-toolchain | awk '{print $1}')"; \
    CARGO_HOME=/usr/local/cargo /usr/local/cargo/bin/rustup default "$active_tc"; \
    CARGO_HOME=/usr/local/cargo /usr/local/cargo/bin/rustup component add rust-analyzer || true; \
    rm -rf /tmp/toolchain-setup; \
    ln -sf /usr/local/cargo/bin/* /usr/local/bin/; \
    chmod -R a+rX /usr/local/rustup /usr/local/cargo

USER dev
WORKDIR /workspace

# Install Antigravity CLI (`agy`) owned by `dev` so background self-updates work inside long-lived DevPod workspaces.
RUN curl -fsSL https://antigravity.google/cli/install.sh | bash -s -- --dir /home/dev/.local/bin \
    && rm -rf /home/dev/.cache/antigravity \
    && /home/dev/.local/bin/agy --version
