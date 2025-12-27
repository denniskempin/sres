# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

SRES is a SNES (Super Nintendo Entertainment System) emulator written in Rust. It's a learning-focused project that emulates the 65816 CPU, PPU (Picture Processing Unit), APU (Audio Processing Unit with SPC700 and S-DSP), and related SNES hardware components.

## Architecture

### Workspace Structure
- **sres_emulator**: Core emulation library containing all hardware component implementations
- **sres_egui**: GUI frontend with both native desktop and WebAssembly support

### Core Components
- **CPU (65816)**: Full processor implementation with emulation/native modes and comprehensive instruction set
- **PPU**: Graphics rendering with VRAM/OAM/CGRAM, background layers (BG1-BG4), sprite rendering
- **APU**: Audio system with SPC700 processor and S-DSP sound chip supporting 8-channel BRR sample playback
- **Main Bus**: Memory management and component interconnection with proper SNES memory mapping
- **Cartridge**: ROM loading with LoROM/HiROM support and memory banking
- **Debugger**: Comprehensive debugging system with event tracing, breakpoints, and state inspection

## Documentation

- A full documentation of the SNES can be found at https://problemkaputt.de/fullsnes.htm
- A wiki with additional information is at https://snes.nesdev.org/wiki/SNESdev_Wiki
- Use context7 to fetch documentation about rust crates used by this project

## Common Development Commands

### Building and Testing
```bash
# Run all tests (requires cargo-nextest: cargo install cargo-nextest)
cargo nextest run

# Run all tests with regular cargo test
cargo test

# Build native GUI
cargo build

# Build WebAssembly version
cd sres_egui && trunk build

# Run all checks (tests, WASM build, clippy, format)
./check-all.sh

# Auto-fix clippy and formatting issues
./fix-all.sh
```

### Testing Specific Components
```bash
# Run tests for specific crate
cargo nextest run -p sres_emulator
cargo nextest run -p sres_egui

# Run single test
cargo nextest run -E 'test(test_specific_function)'

# Run specific test module
cargo nextest run rom_tests
cargo nextest run ppu_tests
cargo nextest run apu_tests

# Run benchmarks
cargo bench
```

### Code Quality
```bash
# Run clippy (linting)
cargo clippy

# Fix clippy issues automatically
cargo clippy --fix --allow-dirty

# Check formatting
cargo fmt --check

# Apply formatting
cargo fmt
```

### Running the Emulator
```bash
# Run native GUI
cargo run

# Run with specific ROM
cargo run -- rom_file.sfc

# Run web version locally
cd sres_egui && trunk serve
```

## Development Notes

### Component Architecture
Each hardware component is implemented as an independent module with clear interfaces:
- Components communicate through the main bus system
- Memory access goes through proper address decoding and banking
- All components support comprehensive debug tracing
- State can be serialized/deserialized for save states

### Testing Strategy
The project uses ROM-based integration testing:
- PPU tests compare rendered output against reference images
- APU tests verify audio sample generation
- CPU tests validate instruction execution with trace comparison

### Memory Management
SNES memory mapping is complex with multiple address spaces:
- CPU address space: 24-bit addressing with banking
- PPU memory: VRAM, OAM, CGRAM with specific access patterns
- APU memory: 64KB address space for SPC700 with shared communication ports
- Cartridge memory: ROM/SRAM with various mapping modes

### Audio System
Audio processing involves multiple stages:
- SPC700 executes audio program code
- S-DSP processes 8 voices with BRR sample decompression
- Real-time audio buffer generation for host system playback
- Debug visualization of voice envelopes and waveforms

### Cross-Platform Considerations
- Native builds use eframe/egui for desktop GUI
- WASM builds target web browsers with Trunk build system
- Audio output adapts to platform (CPAL for native, Web Audio for WASM)
- ROM loading supports both file system and embedded resources

### Debugging Features
The debugger provides extensive introspection:
- Event filtering and collection for all bus transactions
- Memory viewers for all address spaces
- CPU state inspection with disassembly
- PPU state with VRAM/palette visualization
- APU state with voice analysis and sample playback
- Breakpoint support on memory access and instruction execution

## Performance Considerations

The emulator aims for real-time performance:
- CPU instruction timing matches original hardware
- PPU rendering maintains 60 FPS output
- APU generates audio at 32kHz sample rate
- Memory access patterns optimized for cache efficiency
