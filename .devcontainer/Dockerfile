FROM mcr.microsoft.com/devcontainers/rust:latest

# Local VS Code only. Cloud Agent environments are dashboard-managed and do not
# build this image.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        libasound2-dev \
        libxkbcommon-x11-0 \
        pkg-config \
        unzip \
    && rm -rf /var/lib/apt/lists/*

COPY install-tools.sh /tmp/install-tools.sh
USER vscode
RUN bash /tmp/install-tools.sh
