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
  curl -fsSL -o "${tmp}/bass-ubuntu.zip" \
    "https://github.com/ARM9/bass/releases/download/v18/bass-ubuntu.zip"
  unzip -q "${tmp}/bass-ubuntu.zip" -d "${tmp}"
  install -m 0755 "${tmp}/bass" "${BIN_DIR}/bass"
  mkdir -p "${HOME}/.local/share/bass/architectures"
  cp -a "${tmp}/architectures/." "${HOME}/.local/share/bass/architectures/"
  rm -rf "${tmp}"
fi
