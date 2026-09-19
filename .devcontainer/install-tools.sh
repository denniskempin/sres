#!/usr/bin/env bash
# Shared toolchain install for local VS Code (.devcontainer).
# Cloud Agent setup is dashboard-managed and does not use this file.
set -euo pipefail

BIN_DIR="${CARGO_HOME:-${HOME}/.cargo}/bin"
mkdir -p "${BIN_DIR}"

if ! command -v cargo-binstall >/dev/null 2>&1; then
  curl -LsSf "https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz" \
    | tar zxf - -C "${BIN_DIR}"
fi

command -v cargo-nextest >/dev/null 2>&1 || cargo binstall -y cargo-nextest
command -v trunk >/dev/null 2>&1 || cargo binstall -y trunk

if ! command -v bass >/dev/null 2>&1; then
  tmp="$(mktemp -d)"
  git clone --depth 1 https://github.com/ARM9/bass.git "${tmp}/bass"
  # Current g++ needs this include; upstream nall omits it.
  sed -i '1i#include <stdexcept>' "${tmp}/bass/nall/arithmetic/natural.hpp"
  make -C "${tmp}/bass/bass"
  if [ "$(id -u)" -eq 0 ]; then
    # `make install` refuses root; place the binary and table architectures together.
    install -d /usr/local/lib/bass/architectures
    install -m 0755 "${tmp}/bass/bass/out/bass" /usr/local/lib/bass/bass
    cp -R "${tmp}/bass/bass/data/architectures/"* /usr/local/lib/bass/architectures/
    ln -sfn /usr/local/lib/bass/bass /usr/local/bin/bass
    # bass resolves architectures relative to argv[0]; keep a copy next to the symlink too.
    install -d /usr/local/bin/architectures
    cp -R "${tmp}/bass/bass/data/architectures/"* /usr/local/bin/architectures/
  else
    mkdir -p "${HOME}/.local/bin" "${HOME}/.local/share/bass/architectures"
    make -C "${tmp}/bass/bass" install
    install -m 0755 "${HOME}/.local/bin/bass" "${BIN_DIR}/bass"
  fi
  rm -rf "${tmp}"
fi
