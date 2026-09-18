# `sres_emulator/fuzz/fuzz_targets`

libfuzzer bins for crate `sres_emulator-fuzz`. One steps CPU on arbitrary program bytes; the other feeds bytes to the cartridge SFC loader.

## Files

| File | Owns |
|------|------|
| `program.rs` | Bin `program`. Loads fuzzer input as a 65816 program, constructs `Cpu`, loops `cpu.step()` with a 1000-step cap. |
| `sfc.rs` | Bin `sfc`. Cartridge SFC loader over fuzzer input. |

## Integration

- `[[bin]]` names and `cargo fuzz run` live in parent `sres_emulator/fuzz`.

## Gaps

- Bins stale; see parent Gaps.

## Tests

- Parent `## Tests`: `cargo fuzz run program`, `cargo fuzz run sfc`.
