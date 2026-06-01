---
title: "Tilemaps"
source_url: "https://snes.nesdev.org/wiki/Tilemaps"
pageid: 52
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

With the exception of [mode 7](#Mode_7) and [[Offset-per-tile]], a **tilemap** is a 2-kilobyte (1-kiloword) block of data in VRAM that defines a 32x32 tile region that can be used by a [[Backgrounds|background]]. A tilemap may also be known as a **nametable**.

Tilemaps are manually allocated within VRAM. With registers [[PPU registers#BGnSC|BGnSC]], background layers can select a single tilemap, and can also combine contiguous pairs of tilemaps horizontally (2x1), vertically (1x2), or four tilemaps (2x2) into a larger background. In all combined versions, each individual tilemap is still only 32x32 tiles.

The tiles of a tilemap can also count "double" with [[PPU registers#BGMODE|BGMODE]], selecting a 2x2 group of adjacent 8x8 pixel [[Tiles]] for a 16x16 pixel region in the background, instead of just a single 8x8 tile. This doubles the tilemap's size in each dimension without increasing memory usage. ([[Mode 5|Mode 5 and 6]] also have a special 16x8 tile mode, that selects tiles in horizontal pairs.)

## Format

```
  VMDATAH     VMDATAL
   $4119       $4118
15  bit  8   7  bit  0
 ---- ----   ---- ----
 VHPC CCTT   TTTT TTTT
 |||| ||||   |||| ||||
 |||| ||++---++++-++++- Tile index
 |||+-++--------------- Palette selection
 ||+------------------- Priority
 ++-------------------- Flip vertical (V) or horizontal (H)
```

Each tilemap entry is a 16-bit word in VRAM. Each row is 32 tiles, left to right, and the rows are top to bottom.

- Tile index - 10 bits selecting one of 1024 tiles from VRAM relative to the base address given at: [[PPU registers#BG12NBA|BG12NBA]] or [[PPU registers#BG34NBA|BG34NBA]].
- Palette selection - 0-7 selects one of up to 8 palettes from [[CGRAM]], depending on the [[Background modes|background mode]].
- Priority - tilemaps are separated into background (0) and foreground (1) layers which can allow sprites to appear between these layers. See [[Backgrounds]].
- Flip - each tile can be flipped horizontally or vertically.

When using 16x16 tile modes, the tile index specifies the top left 8x8 tile from VRAM, and it will automatically include the adjacent tiles horizontally (+1) and vertically (+16, +17).

## Mode 7

Mode 7 has a special format for its tilemaps.

A mode 7 tilemap is 128x128 tiles. It always begins at 0 in VRAM, covering the first half of the VRAM memory space.

The tilemap is stored only in the low byte of each word of VRAM, and the tile data is stored in the high bytes. See [[Tiles#Mode 7|Tiles, Mode 7]].

Each byte of the tilemap is simply an 8-bit index to one of the 256 tiles. There are no attributes, though a per-pixel priority can be enabled via [[Backgrounds#EXTBG|EXTBG]].

The order is contiguous rows of 128 tiles, top to bottom.

## Offset-per-tile

Modes 2, 4 and 6 each use BG3 as an [[Offset-per-tile]] map. This tilemap is actually an offset map, usually containing one row of 32 entries for horizontal, and a second row of 32 entries for vertical (or just a single row in mode 4).

See: [[Offset-per-tile]]
