# `sres_emulator/fuzz`

Isolated `cargo-fuzz` crate `sres_emulator-fuzz`. libfuzzer bins feed arbitrary bytes into CPU execution and SFC loading.

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `fuzz_targets/` | Bins `program` and `sfc` (`[[bin]]` names in `Cargo.toml`) |

## Behaviors & Gotchas

1. This crate sets `[workspace] members = ["."]` and is absent from the root workspace `members` list. `cargo test --workspace` and `cargo clippy --workspace` from the repo root never build it.
2. `[profile.release] debug = 1` keeps line info for crash backtraces.
3. `program` returns early when `data.len() > 0x4000000` (`MainBusImpl` ROM buffer limit), sets `cartridge.sram` to `0x70000` bytes (covers LoRom `$70..=$7D`), calls `force_headless()`, and runs `system.cpu.step()` 1000 times.
4. `sfc` calls `Cartridge::with_sfc_data(data, None)` and discards the `Result`; a parse `Err` is a successful fuzzer iteration.
5. `corpus/`, `artifacts/`, `coverage/`, and `target/` are gitignored. Crash reproducers stay local.

## Integration

- Path-depends on `sres_emulator` (`path = ".."`).
- Invoked with `cargo fuzz` or `cargo check --locked` from this directory, not via the root workspace.
- `program` constructs `Cartridge::with_program` and a headless `System`, then steps `system.cpu.step()`.
- `sfc` invokes `Cartridge::with_sfc_data` on raw `.sfc` bytes.

## Tests

Compile check from this directory:

- `cargo check --locked`

Fuzzing requires `cargo-fuzz` (`cargo install cargo-fuzz`). From this directory, after install:

- `cargo fuzz run program`
- `cargo fuzz run sfc`

Bin names match `[[bin]]` `name` in `Cargo.toml`. The `cargo fuzz run` commands were not executed here.
