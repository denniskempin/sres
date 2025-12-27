# Suggested Commands

## Testing
```bash
# Run all tests (preferred method)
cargo nextest run

# Run all tests with standard cargo
cargo test

# Run tests for specific crate
cargo nextest run -p sres_emulator
cargo nextest run -p sres_egui

# Run single test by name
cargo nextest run -E 'test(test_specific_function)'

# Run specific test module
cargo nextest run rom_tests
cargo nextest run ppu_tests
cargo nextest run apu_tests

# Run benchmarks
cargo bench
```

## Building
```bash
# Build native version
cargo build

# Build release version
cargo build --release

# Build WebAssembly version
cd sres_egui && trunk build

# Serve WebAssembly version locally
cd sres_egui && trunk serve
```

## Code Quality
```bash
# Run clippy (linting)
cargo clippy

# Run clippy on entire workspace
cargo clippy --workspace

# Auto-fix clippy issues
cargo clippy --fix --allow-dirty

# Check formatting
cargo fmt --check

# Apply formatting
cargo fmt

# Auto-fix all issues (clippy + formatting)
./fix-all.sh
```

## Comprehensive Checks
```bash
# Run all checks (tests, WASM build, clippy, format)
./check-all.sh
```
This script runs:
1. `cargo nextest run --workspace` - All tests
2. `cd sres_egui && trunk build` - WASM build
3. `cargo clippy --workspace` - Linting
4. `cargo fmt --check` - Format check

## Running the Emulator
```bash
# Run native GUI
cargo run

# Run with specific ROM
cargo run -- rom_file.sfc

# Run web version locally
cd sres_egui && trunk serve
```

## Installation Requirements
```bash
# Install cargo-nextest (required for testing)
cargo install cargo-nextest

# Install trunk (required for WASM builds)
cargo install trunk
```

## macOS-Specific (Darwin) Commands
Standard Unix commands are available:
- `git` - Version control
- `ls` - List files
- `cd` - Change directory
- `grep` - Search text
- `find` - Find files
- `cat` - View files
- `vim`/`nano` - Text editors
