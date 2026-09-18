# `sres_emulator/src/components`

Independent SNES hardware. Isolation rules are in `mod.rs`; `main_bus/` and `lib.rs` integrate (root).

## Files

| File | Owns |
|------|------|
| `mod.rs` | Module declarations; isolation rules (comments in this file). |
| `cartridge.rs` | `Cartridge`, `SnesHeader`, `MappingMode`. `.sfc` / SRAM load; LoRom vs HiRom header heuristic. |
| `clock.rs` | `Clock`. Master clock, NMI, H/V timers. Entry: `advance_master_clock`. |

## Behaviors & Gotchas

1. Header pick tries LoRom at `0x7FC0` and HiRom at `0xFFC0`. Keep a candidate only if the name is non-empty and `mapping_mode` matches that slot; both-ok or both-fail is an error (`find_header_in_rom`).
2. Header `rom_size` < 5 becomes 32 KiB (`parse_header`); test ROMs under-report size.
3. Short scanline: V=240 on odd frames (`f % 2 == 1`) is 1360 master cycles, otherwise 1364 (`tick_master_clock`).
4. DRAM refresh: when `h_counter` crosses `dram_refresh_position`, add 40 to `h_counter` and `master_clock`. Position starts at 538, then `538 - ((master_clock - h_counter) & 7)` each line.
5. Dots 323 and 327 take 6 cycles on non-short scanlines (`hdot` after `h_counter` 1292 and 1310).
6. `$4210` read does not clear `nmi_flag` when `v == 225 && h_counter <= 2` (`read_rdnmi`).
7. `advance_master_clock` ticks in chunks of ≤64 so NMI/timer edges are not skipped.
8. H/V IRQ fires on the rise of the match (`EdgeDetector`), not while the match stays true.
9. `Clock` owns `master_clock` and emits `ClockInfo`; other components consume `ClockInfo` (`common`).

## Hardware Map

- `Clock` MMIO: `$4200`, `$4207`–`$420A`, `$4210`–`$4212` (routed by `main_bus`). Semantics: `docs/index.md`.

## Integration

- `MainBusImpl` owns `Clock`, calls `advance_master_clock`, and `consume_nmi_interrupt` / `consume_timer_interrupt`.
- `MainBusImpl` copies ROM/SRAM and `MappingMode` from `Cartridge` for LoRom/HiRom decode.
- `SystemImpl::with_cartridge` and tests construct the system from `Cartridge`.

## Gaps

- `SnesHeader.fast_rom` (mapping bit 5) is parsed and never used. FastROM speed: root.
- NMITIMEN bit 0 and HVBJOY bit 0 (joypad auto-read) are ignored; `$4212` bit 0 stays 0. Serial/auto-read packing: parent.

## Tests

- Unit tests in `cartridge.rs` and `clock.rs` (inline BSNES `(v, h)` log; no extra assets).
- `cargo nextest run -p sres_emulator --lib -E 'test(components::clock::) or test(components::cartridge::)'`

## Subdirectories

| Directory | Hardware |
|-----------|----------|
| `cpu/` | W65C816 |
| `ppu/` | Ricoh 5C77 |
| `s_dsp/` | Sony S-DSP |
| `spc700/` | Sony SPC700 |
