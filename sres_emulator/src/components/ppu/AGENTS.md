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

1. `decode_bgmode` panics on modes 4, 6, and 7. Exception to root (never panic for unimplemented hardware): `write_bgmode` stores `BgMode::Mode4` / `Mode6` / `Mode7`; the first visible `draw_scanline` hits `_ => panic!("Unsupported BG mode")`.
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

Unimplemented PPU features follow root (ignore write / read 0) unless noted. This module uses `log::warn` / `log::error`, not a `DebugEvent`.

- Windows `$2126–$212B`, MOSAIC `$2106`: unmatched, warn + ignore.
- `INIDISP` bits 0–3 (brightness): ignored; only bit 7 (`disabled`) is used.
- Hi-res and interlace: Mode 5 still draws 256 pixels; Mode 6 panics (gotcha 1).
- Offset-per-tile (modes 2/4/6): not applied (mode 2: gotcha 2; 4/6 panic).
- Modes 4/6/7: panic in `decode_bgmode` (exception to root).
- OBJ color math: stored, ignored (gotcha 4).

## Tests

- Golden-image: `cargo nextest run -p sres_emulator --test ppu_tests` (`tests/ppu_tests`).
- Lib: `cargo nextest run -p sres_emulator --lib -E 'test(components::ppu::)'`
