# `sres_emulator/fuzz`

Isolated `cargo-fuzz` crate `sres_emulator-fuzz`. libfuzzer bins feed arbitrary bytes into CPU execution and SFC loading.

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `fuzz_targets/` | Bins `program` and `sfc` (`[[bin]]` names in `Cargo.toml`) |

## Behaviors & Gotchas

1. This crate sets `[workspace] members = ["."]` and is absent from the root workspace `members` list. `cargo test --workspace` and `cargo clippy --workspace` from the repo root never build it.
2. `[profile.release] debug = 1` keeps line info for crash backtraces.
3. `program` calls `cpu.step()` at most 1000 times; it does not run until halt.
4. `sfc` discards the loader result (`let _ = ...`); a parse `Err` is a successful fuzzer iteration.
5. `corpus/`, `artifacts/`, `coverage/`, and `target/` are gitignored. Crash reproducers stay local.

## Integration

- Path-depends on `sres_emulator` (`path = ".."`).
- Invoked with `cargo fuzz` from this directory, not via the root workspace.
- `program` is meant to load the input as a program and step the CPU.
- `sfc` is meant to parse the input as an `.sfc` image.

## Gaps

- Both bins are stale against the current library and do not compile. Until rewritten, they do not run the root "arbitrary input never panics" check.
- `program` (`fuzz_targets/program.rs`): imports `crate::components::cpu::Cpu` (this crate has no `components` module) and `sres_emulator::bus::SresBus` (no crate-root `bus`, no `SresBus`). `Cpu::new` now takes `(BusT, DebugEventCollectorRef<CpuEvent>)`.
- `sfc` (`fuzz_targets/sfc.rs`): imports `sres_emulator::cartridge::Cartridge` (type is `sres_emulator::components::cartridge::Cartridge`). Calls `Cartridge::new()` and `load_sfc_data`; current constructors are `with_sfc_data`, `with_sfc_file`, and `with_program`. `with_sfc_data` returns `Result` and bails when the ROM is shorter than the LoRom header window (`0x7FC0 + 0x20`).
- Older docs said both targets crash on empty input. The bins have no empty-input guard. Not reproduced: `cargo-fuzz` is missing and the bins do not compile.

## Tests

`cargo-fuzz` is not part of the workspace toolchain (`cargo fuzz` is not a cargo subcommand until `cargo install cargo-fuzz`).

From this directory, after install:

- `cargo fuzz run program`
- `cargo fuzz run sfc`

Bin names match `[[bin]]` `name` in `Cargo.toml`. These commands were not executed here.
