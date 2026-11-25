# `sres_emulator/fuzz/fuzz_targets`

libfuzzer bins for crate `sres_emulator-fuzz`. One steps CPU on arbitrary program bytes; the other feeds bytes to the cartridge SFC loader.

## Files

| File | Owns |
|------|------|
| `program.rs` | Bin `program`. Builds headless `System` via `Cartridge::with_program` with `0x70000` SRAM; runs 1000 `cpu.step()` calls. |
| `sfc.rs` | Bin `sfc`. Calls `Cartridge::with_sfc_data` over fuzzer input. |

## Integration

- `[[bin]]` names and `cargo fuzz run` / `cargo check --locked` live in parent `sres_emulator/fuzz`.

## Tests

- Parent `## Tests`: `(cd sres_emulator/fuzz && cargo check --locked)`, `cargo fuzz run program`, `cargo fuzz run sfc`.
