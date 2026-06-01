---
title: "Sprites"
source_url: "https://snes.nesdev.org/wiki/Sprites"
pageid: 56
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Sprites allow 16-color graphics tiles to be rendered at freely placed locations on the screen, independent of the tile grids that [[Backgrounds]] are constrained to.

Terms:

- **OAM** - the "object attribute memory", the memory storage area for sprite data.
- **OBJ** - the rendering layer for sprites.

## OBJ

Each sprite places a rectangular group of tiles at a given location on the screen.

All sprites use 4bpp 16-color [[Tiles]]. Each sprite selects one of 8 palettes from the last half of [[CGRAM]].

The [[PPU registers#OBJSEL|OBJSEL]] register selects the VRAM location for sprite tiles, and also two rectangular sizes that sprites can use during rendering.

Sprites can be squares of 8x8, 16x16, 32x32, 64x64, and rectangular 16x32 or 32x64 pixels in various allowed combinations. There is no 8x16 sprite size like the NES has.
They are made of groups of tiles, adjacent horizontally (+1) and vertically (+16) on a 16 tile wide grid in VRAM.

Sprite can be placed above or below various background layers using their priority attribute.

Within the OBJ layer, sprites with lower index within OAM always appear on top of sprites with a higher index\*.
This happens independently of the priority attribute, which only affects their interaction with BG layers.
Where they overlap, the priority attribute of the lowest-index sprite is used, so if you want to hide part of a sprite behind the background, you can use a lower-index sprite with priority attribute of 0 to "mask" away that part of it ("cookie cutter" effect). This same quirk applied to the NES.

:   \* [[PPU registers#OAMADD|OAMADD]] can adjust this with "priority rotation".

**[[Color math]]:**

- The OBJ layer cannot be subdivided between the main and sub screens, so sprites cannot use color math against each other.
- On the main screen, color math only applies to sprites using palettes 4-7, allowing palettes 0-3 to act as opaque sprites while the others are blending.
- On the sub screen, color math applies to all sprites, which can be used as an alternative if all palettes need to participate.

### Rendering

Sprites are all combined into a single OAM layer. The sprites with the lowest OAM index will appear on top of sprites with higher index. There are two per-line limitations applied during the process of rendering.

#### 1. 32 Sprites per Line

32 sprites can appear on a single scanline, taken from lowest to highest OAM index. When this limit is exceeded, sprites with the highest OAM index will be discarded.

Due to a bug in the PPU, off-screen sprites with an X coordinate -256 ($100) will count towards this limit.[[1]](#cite_note-1)

#### 2. 34 Slivers per Line

The sprites on a scanline are further decomposed into 8 pixel wide slivers, and there is a second limit of 34 slivers. However, during this phase the 32 sprites will be evaluated *in reverse*, from highest OAM index to lowest. This means that slivers with the *lowest* OAM index are discarded first.

Within individual sprites, the slivers are evaluated in left-to-right onscreen order (even if flipped), so if the limit of 34 is met mid-sprite its rightmost slivers will be discarded.

Only slivers which are onscreen count toward this limit, with one exception: if the sprite is placed at X coordinate -256 ($100) all of its slivers will be counted against the limit, regardless (see: [[Errata#Video]]).[[2]](#cite_note-2)

Because of the reverse ordering of the 34 sliver limit, when using sprites wider than 8 pixels, we are much more likely to see dropout of low-index sprites. This is inconvenient because it causes the topmost sprites to disappear first.

#### 3. Draw Order

Once collected, the remaining slivers are rendered with the lowest OAM index on top, before deciding whether each pixel appears above or below the topmost visible background layer.

## OAM

OAM is a 544 byte internal memory, defining the properties of 128 sprites to be rendered.

Internally the memory is divided into words. A single word is not updated until both of its bytes are written. OAM is written through [[PPU registers#OAMDATA|OAMDATA]].

The first 512 bytes (256 words) of OAM are 4-byte groups which define most of the properties of a sprite: (i = sprite index 0-127)

```
byte   7  bit  0
-----  ---------
i*4+0: XXXX XXXX - Low 8 bits of X position
i*4+1: YYYY YYYY - Y position
i*4+2: TTTT TTTT - Low 8 bits of tile
i*4+3: VHPP CCCt
       |||| |||+-- High bit of tile a.k.a. "name select"
       |||| +++--- Palette selection
       ||++------- Priority (3..0 = highest..lowest)
       ++--------- Flip vertical (V) and horizontal (H)
```

The final 32 bytes (16 words) contain some additional properties, with 4 sprites packed into each byte:

```
byte   7  bit  0
-----  ---------
i/4:   DdCc BbAa
       |||| |||+-- Sprite i high bit of x
       |||| ||+--- Sprite i size selection
       |||| ++---- Sprite i+1 x/size
       ||++------- Sprite i+2 x/size
       ++--------- Sprite i+3 x/size
```

- X position gives the top left corner of the sprite. The high bit acts as a -256, allowing a sprite to be placed partially off the left side of the screen.
- Y position is only 8 bits, but will wrap across the top of the screen. With normal [[PPU registers#SETINI|overscan blanking]], sprites up to size 32x32 can be completely hidden at Y=224.
- Tile selection is 9 bits, but the high bit is also selecting from the second sprite page, which does not have to be contiguous with the first (See: [[PPU registers#OBSEL|OBSEL]]).
- Priority allows placement in front or behind various [[Backgrounds|background]] layers.
- Flip can be applied horizontally or vertically to a sprite.
- The size selection allows one of 2 sprite sizes to be used for each sprite, chosen via [[PPU registers#OBSEL|OBSEL]].

There is no visibility flag for sprites, so all 128 are always active, but they can be placed offscreen. Y=224 is sufficiently offscreen for most cases, but 64x64 sprites might be a problem. The high bit of X can be used as well to move the sprite offscreen, but make sure the low byte of X is not $00, as a hardware bug causes sprites at X=$100 to count against the per-scanline limit (see: [[Errata]]).

For simple needs, it can be useful to set the last 32 bytes to a suitable default instead of dealing with the inconvenience of recalculating and repacking the information each time.

Like the NES, sprites appear 1 line lower than their Y value, however because the first line of rendering is always hidden on SNES, a sprite with Y=0 will appear to begin on the first visible line. However, a background with Y scroll of 0 will appear to have its top pixel cut off by the hidden line. Thus either sprite Y should be adjusted 1 line higher, or background scroll 1 line lower so that the two will correspond correctly.

## See Also

- [[OAM layout]] - diagram of OAM memory tables

## References

- [Forum Post](https://forums.nesdev.org/viewtopic.php?p=292526#p292526): Hardware test by undisbeliever verifying 32 sprite, 34 sliver, and x=-256 behaviours.

1. [↑](#cite_ref-1) [bsnes object.cpp](https://github.com/bsnes-emu/bsnes/blob/4faca659c12ffc81d932cb0d23fea477f227d9d1/bsnes/sfc/ppu/object.cpp#L53): object on-screen test
2. [↑](#cite_ref-2) [bsnes object.cpp](https://github.com/bsnes-emu/bsnes/blob/4faca659c12ffc81d932cb0d23fea477f227d9d1/bsnes/sfc/ppu/object.cpp#L138): 34 sliver culling behaviour.
