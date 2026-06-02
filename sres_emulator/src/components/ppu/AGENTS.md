# PPU (Picture Processing Unit)

Software rasterizer for the SNES PPU. Draws one scanline at a time into a 256×224 `Rgb15` framebuffer.

## Files

| File | Responsibility |
|------|----------------|
| `mod.rs` | `Ppu` struct, register I/O, scanline renderer, background/sprite decoding, tile decoders |
| `vram.rs` | 64 KB VRAM, port control, read/write latches, auto-increment |
| `cgram.rs` | 512-byte palette RAM, `CGADD`/`CGDATA` access |
| `oam.rs` | 544-byte OAM (128 sprites), sprite construction, scanline culling |
| `debug.rs` | Debug visualizers: VRAM tiles, backgrounds, sprites, palettes |

## Architecture

- **`Ppu`** — facade. Holds `disabled`, `headless`, and `state: PpuState`
- **`PpuState`** — fully serializable inner state (saved/loaded via `bitcode`)

Code reading PPU state should take `&PpuState` or use `PpuDebug`.

## Bus Interface

Mapped at Bank `$00`/`$80` addresses `$2100`–`$213F` (and mirrors).

### Writes

| Register | Address | Handler |
|----------|---------|---------|
| `INIDISP` | `$2100` | `write_inidisp` |
| `OBJSEL` | `$2101` | `Oam::write_objsel` |
| `OAMADDL/H` | `$2102`/`$2103` | `Oam::write_oamaddl/h` |
| `OAMDATA` | `$2104` | `Oam::write_oamdata` |
| `BGMODE` | `$2105` | `write_bgmode` |
| `BGnSC` | `$2107`–`$210A` | `write_bgnsc` |
| `BG12NBA` | `$210B` | `write_bg12nba` |
| `BG34NBA` | `$210C` | `write_bg34nba` |
| `BGnHOFS` | `$210D`/`$210F`/`$2111`/`$2113` | `write_bgnhofs` |
| `BGnVOFS` | `$210E`/`$2110`/`$2112`/`$2114` | `write_bgnvofs` |
| `VMAIN` | `$2115` | `Vram::write_vmain` |
| `VMADDL/H` | `$2116`/`$2117` | `Vram::write_vmaddl/h` |
| `VMDATAL/H` | `$2118`/`$2119` | `Vram::write_vmdatal/h` |
| `M7A`/`M7B` | `$211B`/`$211C` | `write_m7a/b` |
| `CGADD` | `$2121` | `CgRam::write_cgadd` |
| `CGDATA` | `$2122` | `CgRam::write_cgdata` |
| `TM` | `$212C` | `write_tm` |
| `TS` | `$212D` | `write_ts` |
| `CGADSUB` | `$2131` | `write_cdadsub` |
| `COLDATA` | `$2132` | `write_coldata` |

### Reads

| Register | Address | Handler |
|----------|---------|---------|
| `OAMDATAREAD` | `$2138` | `Oam::read_oamdataread` |
| `VMDATALREAD` | `$2139` | `Vram::read_vmdatalread` |
| `VMDATAHREAD` | `$213A` | `Vram::read_vmdatahread` |
| `CGDATAREAD` | `$213B` | `CgRam::read_cgdataread` |
| `MPYL/M/H` | `$2134`–`$2136` | `read_mpy` |
| `SHVL` | `$2137` | `read_shvl` |
| `OPHCT` | `$213C` | `read_ophct` |
| `OPVCT` | `$213D` | `read_opvct` |
| `STAT77` | `$213E` | `peek_stat77` |
| `STAT78` | `$213F` | `read_stat78` |

Every read has a matching `peek_*` for non-mutating inspection.

## Memories

- **VRAM** — 64 KB, word-addressed via `AddressU15`. `write_vmain` configures increment (1/32/128 words) and mode (after low/high byte). Address remapping is parsed but unimplemented (logs error).
- **CGRAM** — 512 bytes → 256 × 15-bit `Rgb15` colors. Byte-latched writes; reads toggle low/high byte.
- **OAM** — 544 bytes: `$000`–`$1FF` (128 sprites × 4 bytes), `$200`–`$21F` (32 attribute bytes). `write_oamdata` has hardware-accurate latch behavior. Address wraps modulo 544.

## Backgrounds

`Background` struct fields: `main_enabled`, `subscreen_enabled`, `color_math_enabled`, `bit_depth`, `palette_addr`, `tile_size`, `tilemap_addr`, `tileset_addr`, `tilemap_size`, `h_offset`, `v_offset`.

`Background::get_tile`: compute quadrant → read 16-bit tilemap entry → extract tile index, palette, priority, flip H/V.

### Tile Decoding

Generic `TileDecoder` trait with `PhantomData`. Implemented: `Bpp2Decoder`, `Bpp4Decoder`, `Bpp8Decoder`. `Tile<T>` and `TileRow<T>` decode on-demand from VRAM bit planes.

## Background Modes

Supported: **0, 1, 2, 3, 5**. Not supported: **4, 6, 7** (panic on `decode_bgmode`).

- Mode 0: 4 BGs, all 2bpp, fixed palette offsets (BG1=0, BG2=32, BG3=64, BG4=96)
- Mode 1: BG1/BG2 4bpp, BG3 2bpp, BG4 disabled. `bg3_priority` bit
- Mode 2: BG1/BG2 4bpp, offset-per-tile (decoded as 4bpp)
- Mode 3: BG1 8bpp, BG2 4bpp
- Mode 5: BG1 4bpp, BG2 2bpp

`decode_bgmode` returns front-to-back `Layer` priority slice. Renderer composites back-to-front.

## Sprites

`Oam::get_all_sprites_on_scanline`: iterates 128 sprites, checks Y-range, stops at 32. Returns higher OAM indices first so lower indices win (hardware priority).

`decode_obj`: for each sprite on scanline, determine 8×8 row, loop coarse X tiles, compute tile index, decode with `Tile::<Bpp4Decoder>`, write `(palette_index, priority)` to `obj_data[256]` if non-transparent. Sprites always 4bpp (palettes 128–255).

## Rendering Pipeline (`draw_scanline`)

Called per visible scanline (`screen_y < 224`) on `update_clock` when `v` changes.

1. **Decode Backgrounds** — `bg_data: [[(u8, bool); 256]; 4]`
2. **Decode Sprites** — `obj_data: [(u8, u8); 256]`
3. **Render Sub Screen** — start with `fixed_color`, walk layers back-to-front, write `Rgb15` to `raw_sub[256]`
4. **Color Math Pre-processing** — convert `raw_sub` to signed `(r, g, b)` tuples; compute `div_factor` (2 if `color_math_half`, else 1)
5. **Render Main Screen** — initialize with backdrop + optional math, walk layers back-to-front, apply color math per layer
6. **Write Framebuffer** — copy 256 pixels to `Framebuffer` at `(x, screen_y)`

Sprites never have color math applied (OBJ math bit stored but ignored).

## Color Math

Controlled by `CGADSUB` (`$2131`) and `COLDATA` (`$2132`). Operation: `Add` or `Subtract`. `color_math_half`: divide by 2. `color_math_backdrop_enabled`: apply to backdrop. Per-layer flags on `Background` and `Oam`.

## Mode 7

Stubbed: `write_m7a`/`write_m7b` update multiplier registers; `read_mpy` returns signed 24-bit product. Actual affine rendering not implemented.

## Scroll Register Latches

Shared across all backgrounds and Mode 7. `bgofs_latch` and `bghofs_latch` live in `PpuState`.

## Debug (`debug.rs`)

`PpuDebug<'a>` provides non-mutating read-only helpers: `background_info`, `sprite_info`, `sprites`, `render_sprite`, `render_vram`, `render_background`, `render_palette`.

## Conventions

- Bit-manipulation via `intbits::Bits`
- Generic `TileDecoder` with `PhantomData` (zero-cost, no dynamic dispatch)
- State split: `Ppu` (transient) vs `PpuState` (serializable)
- Peek / Read split
- Scanline-at-a-time rendering (no intermediate frame buffers)
- Hardware-accurate latch behavior

## Gaps / TODOs

- Modes 4, 6, 7 rendering
- Mode 7 affine transformation
- Offset-per-tile (Modes 2/4/6)
- Interlace / Hi-Res (Modes 5/6 512-pixel horizontal)
- Windowing / masking (`$2126`–`$212B`)
- MOSAIC (`$2106`)
- STAT77/STAT78 fully implemented
- OBJ color math enable bit used
- VRAM address remapping implemented
- Brightness / fade (`INIDISP` bits 0–3) applied to framebuffer
