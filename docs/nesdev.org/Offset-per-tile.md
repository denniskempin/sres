---
title: "Offset-per-tile"
source_url: "https://snes.nesdev.org/wiki/Offset-per-tile"
pageid: 20
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Offset-per-tile is a feature of background modes 2, 4, and 6. It allows overriding a layer's horizontal and vertical scroll position on a tile-by-tile basis. This is used by *Yoshi's Island* for the dizzy effect and *Tetris Attack* to have two different playfields with different vertical positions.

This feature can be used to simulate a rotating background for small rotation angles, as seen in *Star Fox*.

See:

- [[Uncommon graphics mode games#Vertical Offset-per-tile|Uncommon graphics mode games: Vertical]]
- [[Uncommon graphics mode games#Horizontal Offset-per-tile|Uncommon graphics mode games: Horizontal]]

## Details on how the feature works

In offset-per-tile modes, BG3's tilemap data is used instead as an offset map.

This usually contains 2 rows of 32 entries (128 bytes total), the first for horizontal offset, and the second for vertical. Mode 4 only contains 1 row of 32 entries which instead choose either only horizontal or vertical per-entry.

- Horizontal entries will replace the BG HOFS value, except for the low 3 bits. The fine scroll of 0-7 pixels is retained, but the upper bits are replaced with the BG3 entry.
- Vertical entries will replace the BG VOFS value entirely.

Each entry is a 16-bit value:

```
15  bit  8   7  bit  0
 ---- ----   ---- ----
 V21. ..SS   SSSS Ssss
 |||    ||   |||| ||||
 |||    ++---++++-++++- New scroll value for this tile. For horizontal values, the bottom three bits are ignored
 ||+------------------- Override scroll value for layer 1
 |+-------------------- Override scroll value for layer 2
 +--------------------- Mode 4 only: Scroll direction (0 = horizontal, 1 = vertical)
```

The leftmost column of tiles is not affected by the offsets. The 0th entry for the offset map applies instead to the second visible column. This allows the first column's offset to be controlled by that layer's normal HOFS/VOFS setting, and the 32 entries affect the remaining 32 tiles that may be visible on that scanline. (Because the left column may be partially scrolled offscreen, up to 33 tiles can be at least partly seen on a scanline.)

Each entry affects an entire column of the image, because the screen Y position does not affect which row of BG3 is used. However, BG3VOFS does set the starting row's data position within the "tilemap", so [[HDMA]] or other timing techniques may be used to switch to multiple "sets" of scroll overrides in different parts of the screen.

The low 3 bits of HOFS are ignored for BG3's offset map, so it can be scrolled to match the other layers it applies to.

If needed, the "tilemap" can be two maps wide or tall (per [[PPU registers#BGnSC|BG3SC]]), though this would not normally have much use. Having it two-wide might allow you to scroll a 64-entry pre-computed offset row in tandem with a two-wide background layer it affects.
