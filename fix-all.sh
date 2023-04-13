set -e

cargo +nightly fmt

cargo clippy --fix --allow-dirty

