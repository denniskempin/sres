set -e

printf '\e[1m\e[32m---------------------------- Tests -------------------------------------\e[0m\n'
cargo nextest run --workspace
echo
printf '\e[1m\e[32m-------------------------- WASM Build ----------------------------------\e[0m\n'
(cd sres_egui && trunk build)
echo
printf '\e[1m\e[32m---------------------------- Clippy ------------------------------------\e[0m\n'
cargo clippy --workspace
echo
printf '\e[1m\e[32m---------------------------- Format ------------------------------------\e[0m\n'
cargo fmt --check && echo "ok"
echo
