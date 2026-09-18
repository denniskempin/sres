# `sres_emulator/src/components/spc700`

Sony SPC700 8-bit audio CPU. Entry points: `step()` and `catch_up_to_master_clock`.

## Files

| File | Owns |
|------|------|
| `mod.rs` | `Spc700<BusT: Spc700Bus>`; `step()`; `catch_up_to_master_clock`; `reset()` |
| `instructions.rs` | One method per mnemonic |
| `opcode_table.rs` | `build_opcode_table()`; `instruction!` macros; `InstructionDef` |
| `operands.rs` | `Operand` / `DecodedOperand` / `AddressMode`; decode then load/store |
| `status.rs` | `Spc700StatusFlags` (PSR) |
| `debug.rs` | `Spc700Debug`, `Spc700State`, `Spc700Event`; BSNES/Mesen traces |
| `test.rs` | TomHarte ProcessorTests harness |

## Behaviors & Gotchas

1. Catch-up ratio: apu AGENTS.md; implemented in `catch_up_to_master_clock` (`f64` floor, steps while `spc_cycle < exposed - 1`, returns `exposed`).
2. Direct page is `0x00xx` vs `0x01xx` from `status.direct_page` (P) (`direct_page_addr`).
3. Decode then load/store are separate (`Operand::decode` → `DecodedOperand::{load,store}`) so an instruction can decode once and access multiple times.
4. Same explicit-cycle style as `cpu/`: `cycle_io` / `cycle_read_u8` / `cycle_write_u8`.
5. `instruction!` execute closures increment `pc` by 1 after `step()` has already `cycle_read_u8(pc)` for the opcode.
6. `sleep` (`0xEF`) and `stop` (`0xFF`) consume 3 dummy read/`cycle_io` pairs and return; they do not halt.
7. `reset()` sets `pc = 0xFFC0`, `sp = 0xEF`, `status.zero = true` (IPL start; no vector fetch).
8. `build_opcode_table` runs in `new()` (not `static`) because execute closures are generic over `BusT`.
9. All 256 slots are assigned; the `ill` initializer is unused.

## Integration

- `Apu` owns `Spc700<ApuBus>` and calls `catch_up_to_master_clock`. CPUIO, timers, IPL: `apu/`.
- Production `Spc700Bus`: `ApuBus` (`apu/apu_bus.rs`). Tests: `TestBus<AddressU16>` in `test.rs`.
- `step()` emits `Spc700Event::Step` then dispatches `opcode_table`.

## Tests

- TomHarte ProcessorTests: `test/{0x..fx}.json.xz` (one JSON object per line). Each case `step()`s once and compares registers, memory, and bus cycles.
- `IGNORE_CYCLE_DETAILS`: `&[0xCA, 0xD7, 0xFE]` — count only (`0xCA` mov1 io between AbsBit R/W; `0xD7` mov `[d]+Y,A` io placement; `0xFE` dbnz open-bus vs value at the same address).
- Filter also matches `debug.rs` format tests.
- `cargo nextest run -p sres_emulator --lib -E 'test(components::spc700::)'`
