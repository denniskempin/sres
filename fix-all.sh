#!/usr/bin/env bash
set -euo pipefail

cargo clippy --fix --allow-dirty --workspace --all-targets --locked
cargo fmt --all
