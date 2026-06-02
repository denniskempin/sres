# `asm_lib` — SNES Test ROM Assembly Utilities

Reusable 65816/SNES assembly for integration test ROMs. Used by `rom_tests.rs`, `ppu_tests.rs`, and `apu_tests.rs`.

Assembled with **bass** (v14+). SPC700 programs use **xa65**.

## Files

| File | Purpose |
|------|---------|
| `base.asm` | Minimal LoROM boilerplate. Defines `seek` macro, fills Bank 0, includes header + `snes.inc`. |
| `snes_header.asm` | Standard LoROM header. Reset vector `$8000`. BRK vector `$0000`. |
| `snes_header_ret.asm` | Same, but BRK vector points to `RTIBreak` label. Use if test triggers `brk`. |
| `snes.inc` | All SNES MMIO register constants (`REG_*`) and `SNES_INIT(ROMSPEED)` macro. |
| `snes_gfx.inc` | Graphics/DMA helpers: `LoadPAL`, `LoadVRAM`, `ClearVRAM`, `WaitNMI`, `FadeIN`/`FadeOUT`, `Mode7CALC`. |
| `snes_spc700.inc` | SPC700 definitions: `SPC_INIT`, `SPCWaitBoot`, `TransferBlockSPC`, `SPCExecute`, `WDSP`, pitch macros. |
| `font8x8.asm` | 1BPP 8×8 font (ASCII `$20`–`$7E`). Used by CPU tests to print "PASS"/"FAIL". |

## Key Macros

- `seek(offset)` — Maps SNES bus address to LoROM file offset. All ROMs use this.
- `SNES_INIT(SLOWROM)` or `SNES_INIT(FASTROM)` — Native mode, stack at `$1FFF`, clears WRAM/VRAM/CGRAM/OAM, force blank.

## Usage Patterns

**Simple CPU/DMA test (no graphics):**
```asm
output "test.sfc", create
include "lib/base.asm"
// Code at $8000
```

**Visual test (manual setup):**
```asm
seek($8000); fill $8000
include "lib/snes.inc"
include "lib/snes_header.asm"
include "lib/snes_gfx.inc"
seek($8000); Start:
  SNES_INIT(SLOWROM)
  // ...
```

## Integration

Assembled `.sfc` files are loaded by Rust tests in `tests/rom_tests.rs`, `tests/ppu_tests.rs`, `tests/apu_tests.rs`.

- Trace tests: compare against BSNES trace logs.
- Outcome tests: run until `stp`, inspect memory.
- Golden tests: compare framebuffer/audio output against reference `.png`/`.wav`.

A symlink at `tests/lib/` points to this directory for include paths.
