# `sres_emulator/src/components`

Independent SNES hardware components. No cross-component dependencies allowed.

## Rules (enforced in `mod.rs`)

1. Components cannot depend on each other.
2. Components can only import from `common/`.
3. Exported types are minimal; inner modules are private.
4. Use `self`/`super` for inner modules only.

Integration happens at `main_bus/` and `lib.rs`.

## Files

| File | Purpose |
|------|---------|
| `mod.rs` | Module declarations and independence rules. |
| `cartridge.rs` | ROM parsing, SNES header extraction, LoRom/HiRom detection. |
| `clock.rs` | Master clock, scanline/horizontal timing, NMI, H/V timer IRQs. |

## Subdirectories

| Directory | Hardware |
|-----------|----------|
| `cpu/` | W65C816 (main CPU) |
| `ppu/` | Ricoh 5C77 (graphics) |
| `s_dsp/` | Sony S-DSP (audio synthesis) |
| `spc700/` | Sony SPC700 (audio CPU) |

## `cartridge.rs`

- **`Cartridge`** — Holds `SnesHeader`, ROM bytes, SRAM bytes.
- **`SnesHeader`** — Parsed from `0x7FC0` (LoRom) or `0xFFC0` (HiRom). Contains name, mapping mode, ROM/SRAM size, fast-ROM flag.
- **`MappingMode`** — `LoRom` or `HiRom`.
- **`RawSnesHeader`** — Packed struct for binary parsing.

Factory methods:
- `Cartridge::with_sfc_file(path)` — Loads `.sfc` + optional `.srm`.
- `Cartridge::with_sfc_data(data, srm_data)` — From raw bytes.
- `Cartridge::with_program(program)` — Minimal cartridge for test ROMs (no header).

Header parser uses heuristics (non-empty name, matching mapping mode) to choose between LoRom/HiRom headers.

## `clock.rs`

Tracks master clock and generates interrupts. Driven by `main_bus`.

- **`Clock`** — Tracks `master_clock`, `v` (scanline), `h_counter`, `f` (frame).
- **`HVTimerMode`** — `Off`, `TriggerH`, `TriggerV`, `TriggerHV`.

Key behaviors:
- Short scanline: line 240 on odd frames is 1360 cycles (vs 1364).
- DRAM refresh: ~40-cycle pause at ~536 cycles into each scanline.
- NMI: Triggered on vblank rise (V >= 225) if `$4200` enabled.
- H/V timers: Triggered when dot/scanline matches `$4207-$420A`.
- NMI read quirk: Reading `$4210` in first 2 cycles of V=225 does not clear flag.

Registers:

| Address | Register |
|---------|----------|
| `$4200` | NMITIMEN |
| `$4207/$4208` | HTIMEL/HTIMEH |
| `$4209/$420A` | VTIMEL/VTIMEH |
| `$4210` | RDNMI (read clears) |
| `$4211` | TIMEUP (read clears) |
| `$4212` | HVBJOY |

`advance_master_clock` chunks advances into ≤64-cycle ticks to avoid missing events. Timing is sensitive; see tests for exact reference behavior.

## Notes

- All components derive `bitcode::Encode/Decode` for save-state serialization where applicable.
- When modifying a component, only import from `common/`. Do not add cross-component dependencies.
