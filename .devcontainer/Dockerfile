FROM gitpod/workspace-rust:2023-05-08-21-16-55

RUN sudo install-packages xa65

# Install cargo-binstall
RUN curl -LsSf "https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz" | tar zxf - -C "$HOME/.cargo/bin"

RUN cargo binstall -y cargo-nextest
