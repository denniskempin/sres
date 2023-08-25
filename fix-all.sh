set -e

cargo clippy --fix --allow-dirty

cargo +nightly fmt
