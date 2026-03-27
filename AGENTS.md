# AGENTS.md

## Cursor Cloud specific instructions

### Project Overview

SRES is a SNES emulator in Rust. See `CLAUDE.md` for full architecture and command reference.

### Services

| Component | Description | How to run |
|---|---|---|
| **sres_emulator** | Core emulation library | Library crate, no standalone process |
| **sres_egui (native)** | Desktop GUI (eframe/egui) | `cargo run` (or `cargo run -- rom.sfc`) |
| **sres_egui (WASM)** | Web build via Trunk | `cd sres_egui && trunk serve` (port 8080) |

No external services (databases, Docker, etc.) are required.

### Common commands

All standard build/test/lint commands are documented in `CLAUDE.md`. Key ones:

- **Build**: `cargo build`
- **Run**: `DISPLAY=:1 cargo run` (headless VM needs `DISPLAY=:1`)
- **Lint**: `cargo clippy --workspace`
- **Format check**: `cargo fmt --check`
- **Tests**: `cargo nextest run --workspace` (or `cargo test`)
- **Full check**: `./check-all.sh`
- **Auto-fix**: `./fix-all.sh`

### Non-obvious caveats

- **Nightly toolchain required**: The project uses `build-std` (rebuilds stdlib from source), which requires the nightly channel specified in `rust-toolchain.toml`. The `rust-src` component must be installed.
- **`DISPLAY=:1`**: When running the GUI in a headless Cloud Agent VM, set `DISPLAY=:1` so eframe can connect to the X11 server.
- **`libxkbcommon-x11`**: The eframe/egui native build requires `libxkbcommon-x11-0` at runtime. If the emulator panics with "Library libxkbcommon-x11.so could not be loaded", install it: `sudo apt-get install -y libxkbcommon-x11-0`.
- **Git LFS**: Test ROMs (`.sfc`), trace logs (`.xz`), and reference images (`.png`) are stored in Git LFS. If LFS objects are unavailable (404 on the server), ROM-based integration tests will still pass using assembled test ROMs (via `xa65`), but some tests may be skipped.
- **`xa65` assembler**: Required by some test ROM assembly. Installed via `sudo apt-get install -y xa65`.
- **`cargo-nextest`**: The preferred test runner. Install via `curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-$HOME/.cargo}/bin`.
