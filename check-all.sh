#!/usr/bin/env bash
# PR checks. Same commands as `.github/workflows/postsubmit.yml` (`health` / `test` / `wasm`).
# Usage: ./check-all.sh [fmt|clippy|deny|vet|test|wasm]...
# No arguments runs all six.
set -euo pipefail

banner() {
  printf '\e[1m\e[32m---------------------------- %s -------------------------------------\e[0m\n' "$1"
}

run_fmt() {
  banner "Format"
  cargo fmt --all -- --check && echo ok
}

run_clippy() {
  banner "Clippy"
  cargo clippy --workspace --lib --bins --tests --locked
}

run_deny() {
  banner "Deny"
  cargo deny check
}

run_vet() {
  banner "Cargo Vet"
  cargo vet --locked
}

run_test() {
  banner "Tests"
  # egui_kittest/wgpu needs an X display. GitHub-hosted runners have none.
  if [[ -z "${DISPLAY:-}" ]] && command -v xvfb-run >/dev/null; then
    xvfb-run --auto-servernum cargo nextest run --workspace --locked
  else
    cargo nextest run --workspace --locked
  fi
}

run_wasm() {
  banner "WASM Check"
  cargo check --target wasm32-unknown-unknown -p sres_egui --locked
}

if [[ $# -eq 0 ]]; then
  set -- fmt clippy deny vet test wasm
fi

for check in "$@"; do
  case "$check" in
    fmt) run_fmt ;;
    clippy) run_clippy ;;
    deny) run_deny ;;
    vet) run_vet ;;
    test) run_test ;;
    wasm) run_wasm ;;
    -h | --help | help)
      echo "Usage: $0 [fmt|clippy|deny|vet|test|wasm]..."
      echo "No arguments runs all PR checks (fmt, clippy, deny, vet, test, wasm)."
      exit 0
      ;;
    *)
      echo "Unknown check: $check" >&2
      echo "Usage: $0 [fmt|clippy|deny|vet|test|wasm]..." >&2
      exit 1
      ;;
  esac
  echo
done
