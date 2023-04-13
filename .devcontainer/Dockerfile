FROM menci/archlinuxarm:base-devel

RUN pacman -Sy --noconfirm \
    curl \
    wget \
    git \
    inetutils \
    nodejs \
    yarn \
    python-pip \
    openssh \
    fish \
    rustup

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN rustup toolchain install stable --profile minimal
RUN rustup target add wasm32-unknown-unknown
RUN rustup component add clippy rustfmt

RUN cargo install -f wasm-pack wasm-opt
