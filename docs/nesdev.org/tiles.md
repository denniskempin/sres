---
title: "Tiles"
source_url: "https://snes.nesdev.org/wiki/Tiles"
pageid: 51
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Graphics tile data is stored in VRAM, to be used by [[Backgrounds]] and [[Sprites]].

Each tile is an 8x8 pixel unit, and there are several formats available. Most are named by their **bits-per-pixel** (**bpp**), which describes how many colors they can use.

Within memory, tiles are arranged in rows of 16 columns. [[Tools#Debugging|Debugging emulators]] can usually display VRAM with this layout.

Backgrounds and sprites can sometimes work with 16x16 or larger groups of tiles. These are made up of 8x8 pixel tiles, using tiles that are adjacent horizontally (+1) or vertically (+16) in the 16-column layout.

## 2bpp

Used in all layers of BG mode 0, and for one layer in modes 1, 4 and 5.

Each word of VRAM defines two planes of data for an 8 pixel row.

The low byte contains the bit-0 plane, and the high byte contains the bit-1 plane. These combine to form a 2bpp result.

```
plane 1 ($2119)    plane 0 ($2118)
7 ... bit ... 0    7 ... bit ... 0
---------------    ---------------
0 1 0 1 0 1 0 1    0 0 0 1 1 1 0 1
```

```
2bpp result
0 .. pixel .. 7
---------------
0 2 0 3 1 3 0 3
```

8 data words (16 bytes) define one 2bpp tile, top to bottom.

Each pixel has a value of 0-3 to index a palette in [[CGRAM]].

## 4bpp

The most common format. Used for all sprites, and in BG modes 1, 2, 3, 5, and 6.

This format is essentially two 2bpp tiles. Bit planes 0 and 1 are in the first 16 bytes, and a second 16 bytes contains bit planes 2 and 3.

With 4 planes, each pixel can have a value from 0-15 to index a color palette.

## 8bpp

Used in BG modes 3 and 4.

This is like two 4bpp tiles, or four 2bpp tiles. 64 bytes per tile:

- Planes 0, 1 (16 bytes)
- Planes 2, 3
- Planes 4, 5
- Planes 6, 7

With 8 planes, the palette index value ranges from 0-255.

## 8bpp Direct Color

An alternate mode for BG modes 3 and 4, enabled via [[PPU registers#CGWSEL|CGWSEL]].

This is the same data format as 8bpp, but instead of using the result as a palette index, the bits correspond directly to RGB values.

```
8bpp pixel
7  bit  0
---------
BBGG GRRR
|||| ||||
++++-++++-- Direct RGB to 5-bit color: RRRr0 GGGg0 BBb00
   | ||
...b gr..
---------
7  bit  0
attribute
```

This allows you to specify colors directly, bypassing the palette, but with the limitation that you can only control 8 of the 15 bits that define a color.

3 additional bits come from the [[Tilemap]]. Its palette selection attribute provides 3 more bits instead of selecting a palette.
However, those 3 extra bits have to apply to the whole 8x8 tile.

Because a pixel value of 0 is always transparent, it is not possible to specify pure black as a direct color. A dark color could substitute, or CGRAM color 0 could be set to black if it is visible behind the transparent pixel.

Mode 7 also has an equivalent direct color format with the bits arranged differently (see [below](#Mode_7_Direct_Color)).

## Mode 7

Mode 7 stores all of its tile data only in the high byte of VRAM words.

If we ignore the low byte of each word and think of them as contiguous bytes, each byte corresponds to 1 pixel, giving an 8-bit palette index. This format is "chunky" rather than "planar".

Pixel bytes are in order: left to right, top to bottom.

[[Backgrounds#EXTBG|Mode 7 EXTBG]] allows tile pixels to be split into two layers, like a [[Tilemap]] priority attribute. Pixels with value 1-127 go on the lower layer, and 128-255 on the higher. The high bit of the palette index will always be replaced with 0, effectively giving a 7bpp color selection.

## Mode 7 Direct Color

An alternative color mode for mode 7. It's the same chunky 1 byte per pixel data organization, but using the direct color format (see [above](#8bpp_Direct_Color)). Enabled via [[PPU registers#CGWSEL|CGWSEL]].

```
pixel
7  bit  0
---------
BBGG GRRR
|||| ||||
++++-++++-- Direct RGB to 5-bit color: RRR00 GGG00 BB000
```

Because mode 7 tilemaps do not have attributes, we have 0 instead of the "extra" colors of the other direct color modes.

Direct color is not available on BG2 in [[Backgrounds#Mode 7 EXTBG|EXTBG]] mode, though it is always available for Mode 7 BG1.
