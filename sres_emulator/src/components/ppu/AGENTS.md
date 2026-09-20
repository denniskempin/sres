# `sres_emulator/src/components/ppu`

Ricoh 5C77 PPU. `Ppu` facade over serializable `PpuState`.

## Files

| File | Owns |
|------|------|
| `mod.rs` | `Ppu`, `PpuState`, `Framebuffer`. `$2100–$213F` decode. `update_clock` → `draw_scanline`. |
| `vram.rs` | `Vram`. `VMAIN` increment 1/32/128; remapping bits log error. |
| `cgram.rs` | `CgRam`. `CGADD`/`CGDATA` low-byte latch. |
| `oam.rs` | `Oam`, `Sprite`. Address wraps modulo 544; scanline cull 32. |
| `debug.rs` | `PpuDebug`, `VramRenderSelection`. |

## Behaviors & Gotchas

1. `decode_bgmode` does not draw BG pixels for modes 4, 6, and 7 (`PpuBgMode4`/`6`/`7` on `write_bgmode`); sprites and backdrop still composite.
2. Mode 2 sets BG3 `BitDepth::Opt` (offset-per-tile) but `decode_bgmode` only decodes BG1/BG2 as 4bpp; `Opt` is never read.
3. `update_clock`: `disabled` (`INIDISP` bit 7) returns without drawing or advancing `current_clock`. Otherwise, when `v` changes, call `draw_scanline` unless `headless`, then store `last_drawn_scanline`. `draw_scanline` returns if `screen_y >= 224`.
4. `get_all_sprites_on_scanline` iterates OAM 0..128, breaks when `len > 32`, then reverses so higher index is first and lower index wins. `CGADSUB` bit 4 is stored on `Oam.color_math_enabled`; the Object branch of `draw_scanline` never reads it.
5. `bgofs_latch` and `bghofs_latch` in `PpuState` are shared by all `BGnHOFS` / `BGnVOFS` writes.
6. `write_m7a` / `write_m7b` update `m7a_mul` / `m7b_mul` for `read_mpy` (`$2134–$2136`) only. No affine Mode 7 render.
7. `VMAIN` bits 2–3 parse address remapping and `log::error`; the address is not remapped.

## Hardware Map

| Range | Owner |
|-------|-------|
| `$2100–$213F` | this PPU (`BusDeviceU24`) |

Bitfields: `docs/index.md`.

## Integration

- `MainBusImpl` routes `$2100–$213F` to `Ppu` and calls `update_clock` after the clock tick.
- `SystemImpl` owns `Ppu` (`framebuffer`, `swap_framebuffer`, `debug`).
- Snapshot tests `load_state` then `draw_scanline` with no ROM (`tests/ppu_tests`).

## Gaps

Unimplemented PPU features follow root (ignore write / read 0) unless noted. Named variants fire on access. Unknown offsets are `on_error`. Write-only reads (`$2100–$2133`) and read-only writes (`$2134–$213F`) are explicit no-ops. `peek_*` is silent.

- Windows (`$2123–$212B`, `$212E–$212F`) / MOSAIC (`$2106`) / CGWSEL (`$2130`) / SETINI (`$2133`) / M7SEL (`$211A`) / M7C–M7Y (`$211D–$2120`): named write variants. `VMAIN` bits 2–3: `PpuVramRemap` (gotcha 7).
- `INIDISP` bits 0–3: `PpuInidispBrightness` when not `$F`; only bit 7 (`disabled`) affects rendering.
- Hi-res (modes 5/6): `PpuHiRes`. Offset-per-tile (modes 2/4/6): `PpuOffsetPerTile`.
- Modes 4/6/7: `PpuBgMode4`/`6`/`7`; BG layers draw-nothing (gotcha 1).
- OBJ color math: `PpuObjColorMath` — CGADSUB bit 4 stored, compositor ignores (gotcha 4).
- STAT77/STAT78 reads: `PpuStat77Read` / `PpuStat78Read`.

## Tests

- Golden-image: `cargo nextest run -p sres_emulator --test ppu_tests` (`tests/ppu_tests`).
- Lib: `cargo nextest run -p sres_emulator --lib -E 'test(components::ppu::)'`
