# Codebase Structure

## Root Directory
```
.
├── sres_emulator/      # Core emulation library
├── sres_egui/          # GUI frontend
├── check-all.sh        # Run all checks (tests, WASM, clippy, format)
├── fix-all.sh          # Auto-fix clippy and formatting issues
├── rustfmt.toml        # Rust formatting configuration
├── rust-toolchain.toml # Rust toolchain specification
├── Cargo.toml          # Workspace manifest
└── CLAUDE.md           # Project documentation for Claude Code
```

## sres_emulator Structure
```
sres_emulator/
├── src/
│   ├── components/     # Hardware component implementations
│   │   ├── cpu/        # 65816 CPU implementation
│   │   ├── ppu/        # Picture Processing Unit
│   │   ├── spc700/     # SPC700 audio processor
│   │   └── s_dsp/      # S-DSP sound chip
│   ├── apu/            # Audio Processing Unit
│   ├── main_bus/       # Memory management and bus interconnection
│   ├── debugger.rs     # Debugging system with event tracing
│   ├── controller.rs   # Input controller handling
│   └── lib.rs          # Main library entry point
├── tests/              # Integration tests (ROM-based)
├── benches/            # Performance benchmarks
└── fuzz/               # Fuzzing tests
```

## Key Modules
- **components**: Individual hardware component implementations
  - **cpu**: Full 65816 processor with comprehensive instruction set
  - **ppu**: Graphics rendering with VRAM/OAM/CGRAM, background layers (BG1-BG4), sprites
  - **spc700**: Audio processor for executing audio program code
  - **s_dsp**: Sound chip with 8-channel BRR sample playback
- **apu**: Audio system integration
- **main_bus**: SNES memory mapping with proper address decoding and banking
- **debugger**: Event tracing, breakpoints, and state inspection
- **controller**: Input handling

## Architecture Principles
- Each hardware component is implemented as an independent module with clear interfaces
- Components communicate through the main bus system
- Memory access goes through proper address decoding and banking
- All components support comprehensive debug tracing
- State can be serialized/deserialized for save states
