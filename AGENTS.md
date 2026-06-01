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

### Reference documentation

The `docs/` directory contains useful reference information. Read `docs/index.md` for how to access the information.

### Codebase Summaries (AGENTS.md)

Each significant directory in `sres_emulator` contains an `AGENTS.md` file written for AI agents. Use these to quickly understand a module's purpose, key types, patterns, and conventions.

| Path | What it covers |
|------|----------------|
| `sres_emulator/src/AGENTS.md` | System orchestration (`lib.rs`), controller format, debugger |
| `sres_emulator/src/common/AGENTS.md` | Shared types, traits, utilities, test doubles |
| `sres_emulator/src/components/AGENTS.md` | Component independence rules, cartridge, clock |
| `sres_emulator/src/components/cpu/AGENTS.md` | W65C816 CPU core |
| `sres_emulator/src/components/ppu/AGENTS.md` | Picture Processing Unit (graphics rendering) |
| `sres_emulator/src/components/s_dsp/AGENTS.md` | Sony S-DSP (audio synthesis) |
| `sres_emulator/src/components/spc700/AGENTS.md` | Sony SPC700 audio CPU |
| `sres_emulator/src/apu/AGENTS.md` | APU integration (SPC700 + S-DSP + timers + APUIO) |
| `sres_emulator/src/main_bus/AGENTS.md` | Central SNES system bus, DMA, memory mapping |
| `sres_emulator/tests/AGENTS.md` | Integration test suite overview |
| `sres_emulator/tests/rom_tests/AGENTS.md` | CPU trace-comparison & ROM-outcome tests |
| `sres_emulator/tests/ppu_tests/AGENTS.md` | Golden-image rendering tests |
| `sres_emulator/tests/apu_tests/AGENTS.md` | Golden-WAV audio tests |
| `sres_emulator/tests/asm_lib/AGENTS.md` | Test ROM assembly library |
| `sres_emulator/benches/AGENTS.md` | Criterion benchmarks |
| `sres_emulator/fuzz/AGENTS.md` | Fuzzing setup (`cargo-fuzz`) |

### Non-obvious caveats

- **Nightly toolchain required**: The project uses `build-std` (rebuilds stdlib from source), which requires the nightly channel specified in `rust-toolchain.toml`. The `rust-src` component must be installed.
- **`DISPLAY=:1`**: When running the GUI in a headless Cloud Agent VM, set `DISPLAY=:1` so eframe can connect to the X11 server.
- **`libxkbcommon-x11`**: The eframe/egui native build requires `libxkbcommon-x11-0` at runtime. If the emulator panics with "Library libxkbcommon-x11.so could not be loaded", install it: `sudo apt-get install -y libxkbcommon-x11-0`.
- **Git LFS**: Test ROMs (`.sfc`), trace logs (`.xz`), and reference images (`.png`) are stored in Git LFS. If LFS objects are unavailable (404 on the server), ROM-based integration tests will still pass using assembled test ROMs (via `xa65`), but some tests may be skipped.
- **`xa65` assembler**: Required by some test ROM assembly. Installed via `sudo apt-get install -y xa65`.
- **`cargo-nextest`**: The preferred test runner. Install via `curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-$HOME/.cargo}/bin`.
