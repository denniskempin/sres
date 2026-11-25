#!/usr/bin/env bash
set -euo pipefail

if [[ -z "${RUSTC_WRAPPER:-}" ]] && command -v sccache >/dev/null 2>&1; then
  export RUSTC_WRAPPER=sccache
fi

cargo clippy --fix --allow-dirty --workspace --lib --bins --tests --locked
cargo fmt --all
