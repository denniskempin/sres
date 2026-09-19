# `sres_emulator/tests/asm_lib`

Shared include files for test ROMs. This `AGENTS.md` is the same through `tests/lib/`, `rom_tests/lib/`, and `ppu_tests/lib/` (all symlink here).

## Files

| File | Owns |
|------|------|
| `base.asm` | LoROM `seek`; fills Bank 0; includes `snes_header.asm` then `snes.inc`; origin at `$8000` |
| `snes_header.asm` | LoROM header at `$FFC0`. Native BRK `$0000`. Emulation RESET `$8000` |
| `snes_header_ret.asm` | Same header; native BRK is label `RTIBreak` (including ROM must define it) |
| `snes_header_nmi.asm` | Same header; native NMI is label `NmiHandler` (including ROM must define it) |
| `snes.inc` | `REG_*` MMIO constants; `SLOWROM`/`FASTROM`; `SNES_INIT(ROMSPEED)` |
| `snes_gfx.inc` | PPU DMA, wait, fade, and Mode 7 macros |
| `snes_spc700.inc` | SPC700 `REG_*`/`DSP_*` constants; CPU APUIO transfer macros and SMP `SPC_INIT`/`WDSP` |
| `font8x8.asm` | 1BPP 8×8 glyphs, ASCII `$20`–`$7E` |

## Behaviors & Gotchas

1. `seek(offset)` (`base.asm:3`) maps a LoROM bus address to file origin: `origin ((offset & $7F0000) >> 1) | (offset & $7FFF)`.
2. `SNES_INIT(SLOWROM)` / `SNES_INIT(FASTROM)` (`snes.inc:338`) enters native mode, sets SP `$1FFF`, writes `ROMSPEED` to `$420D` (`MEMSEL`), force-blanks (`INIDISP=$8F`), CPU-loop-clears OAM (`REG_OAMDATA`), then DMA-clears WRAM, VRAM, and CGRAM.
3. `base.asm` always includes `snes_header.asm`, not the vector variants. `rom_tests/krom_ret.asm` includes `snes_header_ret.asm` and defines `RTIBreak`. `rom_tests/wai_nmi.asm` includes `snes_header_nmi.asm` and defines `NmiHandler`.
4. `snes_spc700.inc` is bass (`macro Name() { }`). CPU sources include it for `SPCWaitBoot` / `TransferBlockSPC` / `SPCExecute` (`lda.w REG_APUIO0`). SMP sources (`arch snes.smp`) include it for `SPC_INIT` / `WDSP` (`str REG_DSPADDR=…`).
5. `snes_gfx.inc` and CPU-side SPC macros require `snes.inc` `REG_*` already in scope. `base.asm` includes the header then `snes.inc`; callers that skip `base.asm` must include `snes.inc` first.
6. `base.asm` starts with `arch snes.cpu`. SMP programs must not include it.
7. All headers declare SlowROM LoROM (`$20`) and 32KB (`$01`), matching `base.asm`'s single-bank fill.

## Integration

- `rom_tests/*.asm` and `ppu_tests/sprite_rendering.asm` `include "lib/…"`, resolved by `rom_tests/lib` and `ppu_tests/lib`.
- `apu_tests/play_brr_sample.{sfc,spc}.asm` uses `include "../asm_lib/…"`. `apu_tests/` has no `lib` symlink; `play_noise.*.asm` still uses `include "lib/…"`.
- `base.asm` includes siblings by bare filename (`snes_header.asm`, `snes.inc`).
- Rust drivers load committed `.sfc`. Taxonomy lives in root Testing Strategy.

## Tests

No Rust tests in this directory. Parent commands:

- `cargo nextest run -p sres_emulator --test rom_tests`
- `cargo nextest run -p sres_emulator --test ppu_tests`
- `cargo nextest run -p sres_emulator --test apu_tests`
