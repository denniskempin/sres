# PPU (Picture Processing Unit) — Summary

## What is this?

This directory implements the **SNES Picture Processing Unit** (PPU), a pair of custom chips (Ricoh 5C77 “PPU1” and 5C78 “PPU2”) responsible for all tile-based rendering, sprites, palettes, and color math. The implementation is a software rasterizer that draws one scanline at a time into a 256×224 `Rgb15` framebuffer.

---

## File Map

| File | Responsibility |
|------|----------------|
| `mod.rs` | Core `Ppu` struct, register I/O (`BusDeviceU24`), scanline renderer, background/sprite decoding, tile decoders |
| `vram.rs` | 64 KB VRAM (word-addressed), port control (`VMAIN`), read/write latches, auto-increment |
| `cgram.rs` | 512-byte Color Generator RAM (palette memory), `CGADD`/`CGDATA` access |
| `oam.rs` | 544-byte Object Attribute Memory (128 sprites + attribute table), sprite construction, scanline culling |
| `debug.rs` | Debug visualizers: render VRAM tiles, backgrounds, sprites, and palettes to `Image` |

---

## Top-Level Architecture

### `Ppu` vs `PpuState`

- **`Ppu`** is the facade. It holds:
  - `disabled: bool` — force-blank flag (register `$2100` bit 7).
  - `headless: bool` — skips rendering (used for benchmarks).
  - `state: PpuState` — the fully serializable inner state.
- **`PpuState`** holds every bit of mutable PPU state and is what gets saved/loaded via `bitcode` for save states.

> **Pattern:** Facade + inner serializable state. Any code that only reads PPU state should take `&PpuState` or use `PpuDebug`.

---

## Bus Interface (`BusDeviceU24`)

The PPU is mapped at **Bank `$00`/`$80` addresses `$2100`–`$213F`** (and mirrors). The main dispatch lives in `mod.rs`.

### Register Writes (partial list)

| Register | Address | Handler | What it controls |
|----------|---------|---------|------------------|
| `INIDISP` | `$2100` | `write_inidisp` | Force blank + master brightness |
| `OBJSEL` | `$2101` | `Oam::write_objsel` | Sprite sizes, nametable bases |
| `OAMADDL/H` | `$2102`/`$2103` | `Oam::write_oamaddl/h` | OAM word address |
| `OAMDATA` | `$2104` | `Oam::write_oamdata` | OAM byte writes with latch |
| `BGMODE` | `$2105` | `write_bgmode` | BG mode (0–7), BG3 priority, tile sizes |
| `BGnSC` | `$2107`–`$210A` | `write_bgnsc` | Tilemap base + size per background |
| `BG12NBA` | `$210B` | `write_bg12nba` | Tileset base for BG1/BG2 |
| `BG34NBA` | `$210C` | `write_bg34nba` | Tileset base for BG3/BG4 |
| `BGnHOFS` | `$210D`/`$210F`/`$2111`/`$2113` | `write_bgnhofs` | Horizontal scroll (with latch quirks) |
| `BGnVOFS` | `$210E`/`$2110`/`$2112`/`$2114` | `write_bgnvofs` | Vertical scroll (with latch quirks) |
| `VMAIN` | `$2115` | `Vram::write_vmain` | VRAM increment mode/amount |
| `VMADDL/H` | `$2116`/`$2117` | `Vram::write_vmaddl/h` | VRAM word address |
| `VMDATAL/H` | `$2118`/`$2119` | `Vram::write_vmdatal/h` | VRAM data write |
| `M7A`/`M7B` | `$211B`/`$211C` | `write_m7a/b` | Mode 7 matrix values + multiplier latch |
| `CGADD` | `$2121` | `CgRam::write_cgadd` | CGRAM address |
| `CGDATA` | `$2122` | `CgRam::write_cgdata` | CGRAM data write (16-bit latch) |
| `TM` | `$212C` | `write_tm` | Main screen layer enables |
| `TS` | `$212D` | `write_ts` | Sub screen layer enables |
| `CGADSUB` | `$2131` | `write_cdadsub` | Color math control |
| `COLDATA` | `$2132` | `write_coldata` | Fixed color for color math |

### Register Reads

| Register | Address | Handler | What it returns |
|----------|---------|---------|-----------------|
| `OAMDATAREAD` | `$2138` | `Oam::read_oamdataread` | OAM byte at current address |
| `VMDATALREAD` | `$2139` | `Vram::read_vmdatalread` | VRAM low byte (with latch/inc) |
| `VMDATAHREAD` | `$213A` | `Vram::read_vmdatahread` | VRAM high byte (with latch/inc) |
| `CGDATAREAD` | `$213B` | `CgRam::read_cgdataread` | CGRAM byte (toggle latch) |
| `MPYL/M/H` | `$2134`–`$2136` | `read_mpy` | Signed 24-bit multiply result (`M7A * M7B`) |
| `SHVL` | `$2137` | `read_shvl` | Software latch H/V counters |
| `OPHCT` | `$213C` | `read_ophct` | Latched horizontal counter (toggle byte) |
| `OPVCT` | `$213D` | `read_opvct` | Latched vertical counter (toggle byte) |
| `STAT77` | `$213E` | `peek_stat77` | PPU1 status (not fully implemented) |
| `STAT78` | `$213F` | `read_stat78` | PPU2 status + resets latch flags |

> **Convention:** Every read register has a matching `peek_*` for non-mutating inspection.

---

## Memories: VRAM, CGRAM, OAM

### VRAM (`vram.rs`)

- 64 KB physical → 32 Ki × 16-bit words.
- Addressed via `AddressU15` (15-bit word address).
- `write_vmain` configures:
  - **Increment amount**: 1, 32, or 128 words.
  - **Increment mode**: increment after low byte (`$2118`) or high byte (`$2119`).
  - **Address remapping** (2bpp/4bpp/8bpp) is parsed but **logged as error** — not implemented.
- Read path uses a `read_latch` so the first read after setting `VMADD` returns the old buffer contents without advancing the address.

### CGRAM (`cgram.rs`)

- 512 bytes → 256 × 15-bit colors stored as `Rgb15`.
- Writes are latched: first byte is low, second is high; the pair forms a little-endian word.
- Reads toggle between low/high byte and auto-increment after high.
- Indexing by `u8` returns the `Rgb15` color directly.

### OAM (`oam.rs`)

- 544 bytes total:
  - `$000`–`$1FF`: 128 sprites × 4 bytes (x, y, tile, attributes).
  - `$200`–`$21F`: 32 attribute bytes (x MSB, size bit for groups of 4 sprites).
- `write_oamdata` has hardware-accurate latch behavior:
  - Even addresses latch.
  - Odd addresses in low table write latched + current byte as a word pair.
  - High table (`$200`+) writes single bytes directly.
- Address wraps modulo 544.
- Sprites support **two programmable sizes** (e.g. 8×8 & 16×16) selected per-sprite via attribute bits.
- Y-position wrap-around is supported for off-screen sprites (`y + 256` scanline check).

---

## Backgrounds & Tile System

### `Background` struct

Fields:
- `main_enabled` / `subscreen_enabled` / `color_math_enabled`
- `bit_depth: BitDepth` — `Disabled`, `Bpp2`, `Bpp4`, `Bpp8`, `Opt`
- `palette_addr: u8` — base palette index (Mode 0 only uses offsets; other modes start at 0)
- `tile_size: TileSize` — 8×8 or 16×16 (from `BGMODE` bits 4–7)
- `tilemap_addr: AddressU15` — word address of tilemap
- `tileset_addr: AddressU15` — word address of tileset (CHR)
- `tilemap_size: TilemapSize` — 32×32, 64×32, 32×64, 64×64
- `h_offset` / `v_offset: u32` — scroll values (10-bit, hardware is 13-bit but stored as `u32`)

### Tile Fetching (`Background::get_tile`)

1. Compute which tilemap quadrant the coarse coordinate falls into based on `tilemap_size`.
2. Read the 16-bit tilemap entry from VRAM.
3. Extract:
   - Tile index (bits 0–9)
   - Palette (bits 10–12)
   - Priority (bit 13)
   - Flip H (bit 14), Flip V (bit 15)
4. Compute `tile_addr = tileset_addr + tile_idx * words_per_row * 8`.

### Tile Decoders

The decoder pattern uses Rust generics and zero-cost abstractions:

```rust
trait TileDecoder {
    const WORDS_PER_ROW: u32;
    const NUM_COLORS: u8;
    fn new(tile_addr: AddressU15, vram: &Vram) -> Self;
    fn pixel(&self, pixel_idx: u32) -> u8;
}
```

Implemented decoders:
- `Bpp2Decoder` — 2 planes, 1 word/row (4 colors)
- `Bpp4Decoder` — 4 planes, 2 words/row (16 colors)
- `Bpp8Decoder` — 8 planes, 4 words/row (256 colors)

`Tile<TileDecoderT>` and `TileRow<TileDecoderT>` handle per-tile and per-row decode, including horizontal/vertical flipping and palette offset (`palette * NUM_COLORS`).

> **Pattern:** PhantomData + generic `TileDecoder` means no dynamic dispatch and no tile data is copied; pixels are decoded on-demand from VRAM bit planes.

---

## Background Modes (`BgMode`)

Supported modes:
- **Mode 0**: 4 backgrounds, all 2bpp. Unique fixed palette offsets: BG1=0, BG2=32, BG3=64, BG4=96.
- **Mode 1**: BG1/BG2 4bpp, BG3 2bpp, BG4 disabled. BG3 can have high priority via `bg3_priority`.
- **Mode 2**: BG1/BG2 4bpp, offset-per-tile (not fully modeled here as a distinct path; decoded as 4bpp).
- **Mode 3**: BG1 8bpp, BG2 4bpp.
- **Mode 5**: BG1 4bpp, BG2 2bpp.
- **Modes 4, 6, 7**: Not yet implemented (will `panic!`).

### Layer Priority Tables

`decode_bgmode` returns a static slice of `Layer` enums in **front-to-back order**. The renderer composites in reverse (back-to-front), so the first element in the slice is drawn last (top-most).

Priority symbols used:
- `S0..S3`: Sprite priority levels (0 = lowest, 3 = highest)
- `L1..L4`: Background low priority
- `H1..H4`: Background high priority

Example Mode 0 priority (front → back):
```
[S3, H1, H2, S2, L1, L2, S1, H3, H4, S0, L3, L4]
```

---

## Sprite (Object) Rendering

### Scanline Culling (`Oam::get_all_sprites_on_scanline`)

- Iterates all 128 sprites.
- Checks if `scanline` (or `scanline + 256` for wrap-around) falls within the sprite’s Y-range.
- Stops after 32 sprites (hardware limit).
- **Returns higher OAM indices first** so that when the renderer overwrites pixel data, lower indices win — matching hardware priority.

### Per-Scanline Decode (`decode_obj`)

- For each sprite on the scanline:
  1. Determine which 8×8 row of the sprite corresponds to `screen_y`.
  2. Loop over coarse X tiles (`coarse_width`).
  3. Compute tile index considering nametable, coarse position, and flip.
  4. Use `Tile::<Bpp4Decoder>` to decode fine pixels.
  5. Write `(palette_index, priority)` into `obj_data[256]` if pixel != 0 (transparent).

Sprites are always 4bpp (16 colors), and palettes 128–255 in CGRAM.

---

## Rendering Pipeline (`draw_scanline`)

Called once per visible scanline (`screen_y < 224`) on `update_clock` when `v` changes.

### Stage 1: Decode Backgrounds
- Allocate `bg_data: [[(u8, bool); 256]; 4]` (pixel index + priority).
- `decode_bgmode` selects the correct `TileDecoder` and fills the arrays.
- Each background is scrolled via `h_offset` / `v_offset` (wrapped implicitly by 32/64 tilemap sizes).

### Stage 2: Decode Sprites
- Allocate `obj_data: [(u8, u8); 256]` (pixel index + priority).
- `decode_obj` culls and writes sprite pixels.

### Stage 3: Render Sub Screen
- Start with `fixed_color` everywhere.
- Walk layers **back-to-front** (reverse of priority slice).
- Only layers enabled for subscreen contribute.
- Writes `Rgb15` colors into `raw_sub[256]`.

### Stage 4: Color Math Pre-processing
- Convert `raw_sub` into signed `(r, g, b)` tuples:
  - **Add**: positive channels.
  - **Subtract**: negated channels.
- Compute `div_factor` = 2 if `color_math_half`, else 1.

### Stage 5: Render Main Screen
- Initialize scanline:
  - If backdrop color math enabled: `cgram[0] + sub / div_factor`.
  - Else: plain `cgram[0]`.
- Walk layers back-to-front again.
- For each opaque pixel:
  - If `color_math_enabled` for this layer: `(cgram[pal + pixel] + sub[x]) / div_factor`.
  - Else: plain `cgram[pal + pixel]`.
- Sprites never have color math applied in this implementation ( OBJ math bit is read but ignored in main-screen path; note that hardware only applies math to OBJ palettes 4–7).

### Stage 6: Write to Framebuffer
- Copy the 256-pixel scanline into `Framebuffer` at `(x, screen_y)`.

---

## Color Math

Controlled by `CGADSUB` (`$2131`) and `COLDATA` (`$2132`).

- `color_math_operation`: `Add` or `Subtract`.
- `color_math_half`: divide result by 2.
- `color_math_backdrop_enabled`: apply math to the backdrop (color 0).
- Per-layer math enable flags exist on each `Background` and `Oam`.

> **Note:** The implementation pre-processes the subscreen into signed tuples so the per-pixel main-screen loop is branch-free on the operation type.

---

## Mode 7 Support

Partially stubbed:
- `write_m7a` / `write_m7b` update signed multiplier registers (`m7a_mul: i16`, `m7b_mul: i8`).
- `read_mpy` returns the signed 24-bit product across `$2134`–`$2136`.
- Actual Mode 7 rendering (affine/transformed background) is **not implemented**.

---

## Scroll Register Latch Quirks

### Horizontal Scroll (`BGnHOFS`)
Hardware formula:
```
BGnHOFS = (value << 8) | (bgofs_latch & ~7) | (bghofs_latch & 7)
bgofs_latch = value
bghofs_latch = value
```
Note the old latch contributes bits 0–2, creating a “corruption” pattern on the low byte.

### Vertical Scroll (`BGnVOFS`)
Hardware formula:
```
BGnVOFS = (value << 8) | bgofs_latch
bgofs_latch = value
```

These latches are shared across all backgrounds and also with Mode 7 registers.

---

## Debug Module (`debug.rs`)

`PpuDebug<'a>` provides non-mutating read-only helpers for GUI/debug views:

- `background_info(id)` → scroll coordinates.
- `sprite_info(id)` → formatted string.
- `sprites()` → all 128 `Sprite` structs.
- `render_sprite(id)` → `ImageT` of the sprite’s tiles.
- `render_vram(selection)` → tileset viewer for Background or Sprite nametables.
- `render_background(id)` → full tilemap image.
- `render_palette()` → 16×16 palette grid image.

`VramRenderSelection` enum picks which tileset/palette to visualize.

---

## Key Patterns & Conventions

1. **Bit-manipulation via `intbits::Bits`** — `value.bit(n)`, `value.bits(lo..=hi)` used extensively in register handlers.
2. **Generic `TileDecoder` with `PhantomData`** — zero-cost tile decoding without virtual calls.
3. **State split for serialization** — `Ppu` (transient flags) vs `PpuState` (persistent, encoded by `bitcode`).
4. **Peek / Read split** — `read_*` advances latches/addresses; `peek_*` is pure inspection.
5. **Scanline-at-a-time rendering** — no frame buffers for intermediate layers; everything is decoded fresh per scanline.
6. **Hardware-accurate latch behavior** — VRAM read latch, CGRAM byte toggle, OAM word-pair write, scroll register latches.
7. **Shared latch state** — `bgofs_latch` and `bghofs_latch` live in `PpuState` and are mutated by multiple register writes.

---

## Notable Gaps / TODOs

- **Modes 4, 6, 7** are not implemented (panic on `decode_bgmode`).
- **Mode 7 rendering** (affine transformation) is missing entirely.
- **Offset-per-tile** (Modes 2/4/6) is not implemented.
- **Interlace / Hi-Res** (Mode 5/6 512-pixel horizontal) is not supported.
- **Windowing / masking** registers (`$2126`–`$212B`) are unimplemented.
- **MOSAIC** (`$2106`) is unimplemented.
- **STAT77/STAT78** return 0 with a warning; no time-over / range-over tracking.
- **OBJ color math** enable bit is stored but not used in the main-screen path.
- **VRAM address remapping** (2bpp/4bpp/8bpp) logs an error and does nothing.
- **Brightness / fade** (`INIDISP` bits 0–3) is parsed but not applied to the final framebuffer.

---

## Quick Reference: Types

| Type | Location | Purpose |
|------|----------|---------|
| `Ppu` | `mod.rs` | Public API + bus device |
| `PpuState` | `mod.rs` | Full serializable state |
| `Framebuffer` | `mod.rs` | 256×224 `Rgb15` buffer |
| `Background` | `mod.rs` | Per-BG config + scroll |
| `BgMode` | `mod.rs` | Mode 0–7 enum |
| `BitDepth` | `mod.rs` | Bpp2 / Bpp4 / Bpp8 / Opt / Disabled |
| `TileDecoder` | `mod.rs` | Trait for generic tile decode |
| `Bpp2Decoder` | `mod.rs` | 2bpp tile decoder |
| `Bpp4Decoder` | `mod.rs` | 4bpp tile decoder |
| `Bpp8Decoder` | `mod.rs` | 8bpp tile decoder |
| `Tile<T>` | `mod.rs` | Generic tile descriptor |
| `TileRow<T>` | `mod.rs` | Generic row decoder |
| `Vram` | `vram.rs` | VRAM memory + port state |
| `CgRam` | `cgram.rs` | Palette memory + port state |
| `Oam` | `oam.rs` | Sprite memory + port state |
| `Sprite` | `oam.rs` | Decoded sprite descriptor |
| `SpriteSize` | `oam.rs` | 8×8, 16×16, 32×32, 64×64, 16×32, 32×64 |
| `PpuDebug` | `debug.rs` | Debug read-only accessors |
| `VramRenderSelection` | `debug.rs` | Background / Sprite0 / Sprite1 selector |
