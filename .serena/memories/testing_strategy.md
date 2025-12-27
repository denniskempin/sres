# Testing Strategy

## Testing Philosophy
The project uses ROM-based integration testing to validate emulation accuracy:
- **PPU tests**: Compare rendered output against reference images
- **APU tests**: Verify audio sample generation
- **CPU tests**: Validate instruction execution with trace comparison against reference emulators

## Test Organization
Tests are located in `sres_emulator/tests/` and include:
- **rom_tests**: ROM-based integration tests that run actual SNES programs
  - Test ROMs are stored in `sres_emulator/tests/rom_tests/`
  - Reference traces (e.g., Mesen traces) are used for comparison
  - Traces are compressed with xz (`.log.xz` files)
- **ppu_tests**: Graphics rendering validation
- **apu_tests**: Audio processing validation

## Test Execution
```bash
# Run all tests (preferred)
cargo nextest run

# Run all tests (standard cargo)
cargo test

# Run specific test module
cargo nextest run rom_tests

# Run tests for specific crate
cargo nextest run -p sres_emulator

# Run single test
cargo nextest run -E 'test(test_function_name)'
```

## Trace Comparison
The project uses trace files from other emulators (like Mesen) for validation:
- Trace files are in Mesen format
- Files are compressed with xz to save space
- Test processing scripts are available (e.g., `sres_emulator/tests/rom_tests/process.py`)

## Benchmarking
Performance benchmarks using Criterion:
```bash
cargo bench
```

Benchmark harnesses are located in:
- `sres_emulator/benches/rom_benches.rs`
- `sres_emulator/benches/timer_benches.rs`

## Continuous Integration
The project uses GitHub Actions for CI/CD:
- Workflow: `.github/workflows/postsubmit.yml`
- Code coverage reporting via codecov

## Test Dependencies
Dev dependencies for testing include:
- `tempfile`: Temporary file creation
- `pretty_assertions`: Better assertion output
- `criterion`: Benchmarking
- `serde`, `serde_json`: Test data serialization
- `xz2`: Decompression of trace files
- `image`: Image comparison for PPU tests
- `rasciigraph`: ASCII graph rendering for debugging
