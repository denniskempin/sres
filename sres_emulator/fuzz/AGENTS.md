# Fuzzing

Standard `cargo-fuzz` setup in a separate workspace crate.

## Targets

- **`program`** (`fuzz_targets/program.rs`): Fuzzes the CPU emulator with arbitrary 65816 machine code. Runs up to 1000 steps. Should never panic.
- **`sfc`** (`fuzz_targets/sfc.rs`): Fuzzes the cartridge ROM loader with arbitrary bytes. Should never panic.

## Running

```bash
cargo install cargo-fuzz
cargo fuzz run program
cargo fuzz run sfc
```

## Notes

- Both targets crash on empty input. Known issue.
- `corpus/`, `artifacts/`, `coverage/`, `target/` are gitignored.
- `release` profile has `debug = 1` for better crash backtraces.
