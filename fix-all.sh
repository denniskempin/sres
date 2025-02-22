set -e

cargo clippy --fix --allow-dirty

cargo fmt
