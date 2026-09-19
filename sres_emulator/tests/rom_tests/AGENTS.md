# `sres_emulator/tests/rom_tests`

Assets for CPU trace-comparison and ROM-outcome tests driven by `../rom_tests.rs`. `SyncSystem` vs `System` mapping is in root Testing Strategy.

## Files

| Pattern | Owns |
|---------|------|
| `krom_*.{sfc,asm}` | krom 65816 opcode ROMs; matching `-trace.log.xz` except `krom_msc` |
| `ppu_timing.{sfc,asm}` | NOP loop for PPU cycle alignment vs BSNES |
| `play_noise.{sfc,spc}` | Mixed CPU+SPC700; `play_noise.sfc.asm` `insert`s `play_noise.spc` (`play_noise.spc.asm`) |
| `dma_{vram,cgram,oam}.{sfc,asm}` | ROM-outcome DMA round-trip through VRAM, CGRAM, or OAM |
| `wai_nmi.{sfc,asm}` | ROM-outcome WAI until vblank NMI; ordering-sensitive sentinels at `$0000`/`$0001` |
| `wai_irq.{sfc,asm}` | ROM-outcome WAI until H-IRQ with `I=1`; `$0000` stays 0, `$0001` is TIMEUP `$80` |
| `*-trace.log.xz` | XZ-compressed BSNES traces (`parse_mesen_trace`) |
| `process.py` | Renames `*.txt` → `*-trace.log`, trims self-`JMP` loops, `xz` compresses |

## Behaviors & Gotchas

1. `run_rom_test` zips one CPU step per trace line. `play_noise` uses `run_rom_test_with_spc700_trace`: buffers out-of-order CPU vs SPC700 steps (`pending_cpu` / `pending_spc`) and asserts both queues empty on APUIO (`$2140`–`$217F`).
2. `is_cpu_apuio_access` must run on `actual_cpu` before `assert_cpu_trace_eq`; that helper then sets `effective_addr` to `None` on both sides because open bus is unimplemented (root Error Handling).
3. Mixed traces: a line shorter than 100 characters parses as `Spc700State`, otherwise `CpuState`. `assert_spc_trace_eq` also clears `operand_str` and `master_cycle`.
4. `play_noise` returns after trace line `19047`.
5. `process.py` stops a raw log at the first `JMP` whose PC equals the operand effective address (`ppu_timing` is `jmp Start`). The assemble loop in that file is a commented-out string and is not run.
6. `assert_cpu_trace_eq` compares the full `CpuState` string, including `V:`/`H:` (root Testing Strategy). Extra master cycles on an idle frame fail every `krom_*` / `ppu_timing` test. That suite is the regression gate for any `advance_master_clock` cost change.

## Integration

- Crate test binary `rom_tests` in `sres_emulator/tests/rom_tests.rs`.
- CPU-only traces: `Cartridge::with_sfc_file` into `SyncSystem`, then `cpu_step_iter` vs `trace_log_from_xz_file`.
- `play_noise`: `trace_step_iter` vs `mixed_trace_log_from_xz_file`.
- Outcome: `run_test_rom` until `halted()`, asserting `< 10_000_000` steps. `dma_vram`, `dma_cgram`, `dma_oam` compare WRAM `$0000` and `$0100` after DMA copy-back. `wai_nmi` checks `$0000`/`$0001` == `$A5` and halt PC in `$00:8000–$00FFFF`. `wai_irq` checks `$0000` == 0 and `$0001` == `$80`.

## Gaps

- `test_krom_msc` is `#[ignore = "Instructions not implemented yet"]` and has no `krom_msc-trace.log.xz`.
- `play_noise` comparison is truncated at line `19047`.

## Tests

- `cargo nextest run -p sres_emulator --test rom_tests` (parent Tests).
- One test: `cargo nextest run -p sres_emulator --test rom_tests test_play_noise`
- Ignored: `cargo nextest run -p sres_emulator --test rom_tests test_krom_msc --run-ignored only`
