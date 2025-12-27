# Project Overview

## Purpose
SRES is a SNES (Super Nintendo Entertainment System) emulator written in Rust. This is a learning-focused side project that aims to emulate the complete SNES hardware including:
- 65816 CPU with emulation/native modes
- PPU (Picture Processing Unit) for graphics rendering
- APU (Audio Processing Unit) with SPC700 processor and S-DSP sound chip
- Related SNES hardware components (cartridge, controllers, memory management)

## Tech Stack
- **Language**: Rust (edition 2021, minimum version 1.72)
- **GUI Framework**: egui/eframe (supports both native desktop and WebAssembly)
- **Build System**: Cargo workspace with trunk for WASM builds
- **Testing Framework**: cargo-nextest (preferred) or cargo test
- **Benchmarking**: Criterion
- **Key Dependencies**:
  - anyhow: Error handling
  - intbits, packed_struct, bilge: Bit manipulation and packed structures
  - log, env_logger, colored: Logging with colored output
  - puffin: Performance profiling
  - itertools, strum: Utility libraries
  - bitcode: Serialization for save states
  - hound: Audio WAV file support

## Workspace Structure
The project is organized as a Cargo workspace with two main crates:
- **sres_emulator**: Core emulation library containing all hardware component implementations
- **sres_egui**: GUI frontend with native desktop and WebAssembly support

## Development Environment
- Developed on macOS (Darwin)
- Requires cargo-nextest for running tests: `cargo install cargo-nextest`
- Requires trunk for WASM builds: `cargo install trunk`
