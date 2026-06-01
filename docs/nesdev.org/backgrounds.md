---
title: "Backgrounds"
source_url: "https://snes.nesdev.org/wiki/Backgrounds"
pageid: 73
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The SNES backgrounds consist of one or more layers of [[Tilemaps]].

There are 8 available background modes, which can be changed at any time via [[PPU registers#BGMODE|BGMODE]], even mid-screen.

Each mode has 1-4 layers, consisting of [[Tiles]] that are 2bpp (4-color), 4bpp (16-color), or 8bpp (256-color).

| Mode | BG1 | BG2 | BG3 | BG4 | [Hi-res](#High_resolution) | [Priority](#Priority) (front ↔ back) |
| --- | --- | --- | --- | --- | --- | --- |
| [0](#Mode_0) | 2 | 2 | 2 | 2 |  | S3 1H 2H S2 1L 2L S1 3H 4H S0 3L 4L |
| [1](#Mode_1) | 4 | 4 | 2 |  |  | 3H S3 1H 2H S2 1L 2L S1 3H    S0 3L |
| [2](#Mode_2) | 4 | 4 | [[Offset-per-tile|OPT]] |  |  | S3 1H    S2 2H    S1 1L    S0 2L |
| [3](#Mode_3) | 8 | 4 |  |  |  | S3 1H    S2 2H    S1 1L    S0 2L |
| [4](#Mode_4) | 8 | 2 | OPT |  |  | S3 1H    S2 2H    S1 1L    S0 2L |
| [5](#Mode_5) | 4 | 2 |  |  | ✔ | S3 1H    S2 2H    S1 1L    S0 2L |
| [6](#Mode_6) | 4 |  | OPT |  | ✔ | S3 1H    S2       S1 1L    S0 |
| [7](#Mode_7) | 8 | (7) |  |  |  | S3       S2 2H    S1 1L    S0 2L |

## Properties

Each of the 4 BG layers has several independent properties:

- [[Tilemap]] VRAM address: [[PPU registers#BGnSC|BGnSC]]
- [[Tiles|Tileset]] (CHR) VRAM address: [[PPU registers#BG12NBA|BG12NBA]], [[PPU registers#BG34NBA|BG34NBA]]
- Multiple tilemaps in 1x1, 2x1, 1x2, 2x2 arrangements: [[PPU registers#BGnSC|BGnSC]]
- Tile size of 8x8 (or 16x8) or 16x16: [[PPU registers#BGMODE|BGMODE]]
- Scroll position: [[PPU registers#BGnHOFS|BGnHOFS]], [[PPU registers#BGnVOFS|BGnVOFS]]
- [[Windows]]
- [Mosaic](https://snes.nesdev.org/w/index.php?title=Mosaic&action=edit&redlink=1 "Mosaic (page does not exist)"): [[PPU registers#MOSAIC|MOSAIC]]

Each of the 4 BG layers can be independently activated for the main and sub-screen: [[PPU registers#TM|TM]], [[PPU registers#TS|TS]]

- If [[Color math]] is enabled, the main screen can be blended with the subscreen.
- If high resolution is enabled ([[PPU registers#SETINI|SETINI]]) the main-screen appears on even columns, and the sub-screen appears on odd columns.
- If neither color math nor high resolution is enabled, only the main screen is seen.

The 4 BG layers and sprites (OBJ) are composited to make the main and sub-screens, layered according to their [[Priority]]

## Priority

The way background layers and [[Sprites]] are composited on top of each other is different for each mode.

Each of the background layers BG1-4 is further subdivided into a high and low priority layer, using [[Tilemap]] attributes.
Sprites (OBJ) are also subdivided into 4 layers of their own using their [[OAM]] attributes.

In the [mode table above](#Priority_table):

- S3 S2 S1 S0 are the [[Sprite]] layers with priority 3, 2, 1 and 0.
- 1H 1L is BG1 layers with high (H) and low (L) priority.
- 2H 2L is BG2 layers with high (H) and low (L) priority.
- 3H 3L is BG3 layers with high (H) and low (L) priority. In mode 1 only, one of two different high priority positions can be chosen via [[PPU registers#BGMODE|BGMODE]].
- 4H 4L is BG4 layers with high (H) and low (L) priority.

In mode 7 BG1 only has one layer (a), but [EXTBG](#EXTBG) can enable BG2 split into two layers (b, a).

## High resolution

High resolution is automatically used in mode 5 and 6, but can be manually enabled for other modes via [[PPU registers#SETINI|SETINI]].

This doubles the horizontal resolution of the SNES from 256 to 512 pixels.

The main-screen appears on every even column, and the sub-screen appears on every odd column.
[[Color math]] with a fixed color is still available.

In [modes 5 and 6](#Mode_5) high-resolution is forced, and the background layers are automatically de-interleaved into the main and sub-screens,
rendering each tile at half its usual width, but with twice the density of pixels.

In other modes you would have to compose the main/sub-screen with alternating columns.

However, because 512px output tends to get blurred significantly on TVs through normal composite output,
some games use the alternating columns as an alternative to [[Color math]], providing something like a 50% "blend" of the main and sub screens.
This usage is sometimes known as **pseudo hi-res**. (See: Jurassic Park, Kirby's Dream Land 3.)

In modes 5 and 6, horizontal scrolling ([[PPU registers#BGnHOFS|BGnHOFS]]) is in coarse increments of 2 high-resolution pixels. (Interlacing, on the other hand, automatically provides a fine vertical scroll.)

### Interlacing

Interlacing can also be enabled via [[PPU registers#SETINI|SETINI]].

Interlacing shifts every second frame (field) down by half a line. The result is that over 2 frames you get twice the vertical resolution, but at half the framerate.
[[PPU registers#STAT78|STAT78]] can be used to determine whether you're currently on an even or odd field.

In [modes 5 and 6](#Mode_5), if interlacing is enabled the vertical tile density is automatically doubled across the two fields.

In other modes, you would have to manually change the picture between even and odd fields to make use of the increased vertical resolution.

In modes 5 and 6, vertical scrolling ([[PPU registers#BGnVOFS|BGnVOFS]]) is automatically adjusted for interlacing, providing a fine high-resolution vertical scroll position, unlike horizontal high-resolution.

The lowered framerate has the drawback of a visible "flickering". Perception of this flickering varies from person to person,
but it is intensified by images with sharp vertical contrast. Normally interlaced video for broadcast is vertically filtered/blurred to reduce this effect,
but this is harder to accomplish on the SNES with its palette limitations.

Care should be taken not to switch interlacing on or off too frequently in the middle of a game.
On most modern televisions and capture devices, doing so will often cause the signal to drop and re-synchronize for a few seconds (or more).
You might wish to wait for user input to confirm before proceeding into action, after swiching into or out of interlaced mode.
Older CRT televisions can generally switch to interlacing instantly, though it cannot be changed mid-frame.

## Mode 0

The only mode which has 4 independent BG layers. Its drawback is that each layer is only 2bpp (4 color).

The first 128 colors of [[CGRAM]] are divided into 32 4-color palettes, and each BG layer uses its own sequential non-overlapping 8-palette subset:

- BG0 uses CGRAM indices 0-31 (which is the same as the 2bpp layers in modes 1, 4, and 5)
- BG1 uses CGRAM indices 32-63
- BG2 uses CGRAM indices 64-95
- BG3 uses CGRAM indices 96-127

## Mode 1

The most commonly used mode, which has two main 4bpp (16 color) layers BG1 and BG2, and one auxiliar 2bpp (4 color) layer BG3.

BG3 has an additional [[Priority]] control in mode 1. Its priority bit in [[PPU registers#BGMODE|BGMODE]] allows it to be rendered either above or below BG1 and BG2.

BG3 selects a palette from the first 32 entries of [[CGRAM]].

In many games, BG1 and BG2 are used for a colourful main background with parallax, and BG3 to overlay a HUD or text box.

BG3 can also be useful for things like a blended cloud or fog in the foreground, or a third parallax layer in the deep background. (Super Metroid has many good examples.)

## Mode 2

Has two 4bpp layers like mode 1, but BG3 is used to encode [[Offset-per-tile]] for BG1 and BG2, instead of being a visible layer.

## Mode 3 & 4

Mode 3 has an 8bpp BG1 layer, allowing use of all 256 colors of [[CGRAM]]. This can also be used as [[Direct color]], bypassing CGRAM entirely.

Mode 3 also has a 4bpp (16 color) auxiliary layer BG2.

Mode 4 instead has a 2bpp (2 color) auxiliary layer BG2, and BG3 is used to encode [[Offset-per-tile]]. (BG2 palettes are stored in the first 32 entries of [[CGRAM]].)

## Mode 5 & 6

Mode 5 and 6 force [high resolution](#High_resolution) on, and automatically divide the background so that the tiles appear at double horizontal density.

The 8x8 pixel tilemap tile size selectable via [[PPU registers#BGMODE|BGMODE]] is replaced with a 16x8 pixel tile mode instead. The 16x16 mode is still 16x16.

If interlacing is enabled ([[PPU registers#SETINI|SETINI]]), the vertical tile density is automatically doubled as well, giving the option for vertical high-resolution as well. Note that the [[PPU registers#BGMODE|BGMODE]] register still chooses between 16x8 and 16x16. The provides the only way to have insufficient nametable to fill a whole screen: the combination of mode 5 or 6 with interlacing and 16x8 tiles and the smallest size nametable (32x32).

Horizontal scrolling in these modes only has a resolution of 2 hi-res pixels (i.e. 1/256 of the screen per increment), but when interlaced the vertical scrolling has fine control (1/480).

Mode 5 has a 4bpp (16 color) BG1, and an auxiliary 2bpp (4 color) BG2.

Mode 6 is like mode 5 but replaces BG2 with an invisible BG3 providing [[Offset-per-tile]].

## Mode 7

Mode 7 ignores most of the layer configuration options, and instead always occupies the entire first half of VRAM with a 128x128 tilemap, and 256 available 8x8 tiles.

:   See: [[Tilemaps#Mode 7|Mode 7 tilemaps]], [[Tiles#Mode 7|Mode 7 tiles]].

This provides a single background layer that has a unique transformation property.

:   See: [[Mode 7 transform]]

The [[PPU registers#M7SEL|M7SEL]] register provides a few unusual properties just for mode 7:

- The entire screen can be horizontally or vertically flipped.
- Outside the boundary of the 1024x1024 pixel tilemap, three options are provided:
  - Transparency.
  - Fill with tile 0.
  - Infinite horizontal and vertical wrapping (repetition) of the tilemap.

Mode 7 tiles can use [[Direct color]], if desired.

Sometimes mode 7 is used for large rotating boss characters. Even though it has only 1 background layer, sprites can be used to draw things around it that would normally be "background", and another BG mode might be switched to with HDMA to enable a solid floor background at the bottom, for example.

### EXTBG

Normally mode 7 is a single layer on BG1 only, but [[PPU registers#SETINI|SETINI]] can be used to enable **Mode 7 EXTBG** which activates BG2 as a duplicate of BG1, but split into two layers.

EXTBG BG2 treats the high bit of each tile's pixel value like a tilemap priority bit, allowing BG2 to be split into two layers which can appear above and below sprites. This sort of makes BG2 into two "7bpp" layers but they are not independently transformed or scrolled.

EXTBG does not support [[Direct color]] for BG2. BG2 ignores the [[PPU registers#CGWSEL|CGWSEL]] direct color setting and will always use indexed color, though direct color can still be used for BG1.

## See Also

- [[Tilemaps]]
- [[Tiles]]
- [[Uncommon graphics mode games]]

## References

- [[SNES Development Manual]] Book I 2-5-1 Rotation/Enlargement/Reduction
- [Retro Game Mechanics Explained: SNES Background Modes 0-6](https://www.youtube.com/watch?v=5SBEAZIfDAg) - video
- [Retro Game Mechanics Explained: SNES Background Mode 7](https://www.youtube.com/watch?v=3FVN_Ze7bzw) - video
- [Retro Game Mechanics Explained: SNES Background Modes Higher Resolutions](https://www.youtube.com/watch?v=AnEuk8Vj3w0) - video
