#!/usr/bin/env bash
set -euo pipefail

cargo clippy --fix --allow-dirty --workspace --lib --bins --tests --locked
cargo fmt --all
