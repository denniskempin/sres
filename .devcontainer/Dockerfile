FROM mcr.microsoft.com/devcontainers/rust:latest

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install xa65 git-lfs

RUN curl -LsSf "https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz" | tar zxf - -C "$CARGO_HOME/bin"
RUN cargo binstall -y cargo-watch
RUN cargo binstall -y cargo-nextest

