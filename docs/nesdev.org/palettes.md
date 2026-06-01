---
title: "Palettes"
source_url: "https://snes.nesdev.org/wiki/Palettes"
pageid: 67
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The SNES has a palette of 256 colors, stored as 256 15-bit words in **CGRAM**.

A SNES color entry gives an RGB color with 5-bit precision in each component.

## CGRAM access

1. Write a byte to [[PPU registers#CGADD|CGADD]] ($2121) to select one of the 256 entries in CGRAM.
2. Write two bytes to [[PPU registers#CGDATA|CGDATA]] ($2122) to set the 15-bit RGB color for that entry. (Low byte first.)

```
    CGDATA ($2122)
15  bit  8   7  bit  0
 ---- ----   ---- ----
 .BBB BBGG   GGGR RRRR
  ||| ||||   |||| ||||
  ||| ||||   |||+-++++- Red component 
  ||| ||++---+++------- Green component
  +++-++--------------- Blue component
```

After two writes to CGDATA, CGADD will automatically be incremented to the next entry.

Entries can also be read back through [[PPU registers#CGDATAREAD|CGDATAREAD]] ($213B). Note that because the high bit is unused, it returns an unreliable value that should be ignored.

## Assignment

- CGRAM entry 0 is always used as the backdrop color, beneath all the [[Background]] layers and [[Sprites]], or where all other rendering is disabled or windowed.
- [[Tiles#2bpp|2bpp]] tiles use groups of 4 from CGRAM. The first of 4 will be unseen, as a tile pixel of 0 is always transparent, but the remaining 3 will be used to color the visible tile.
- [[Tiles#4bpp|4bpp]] tiles use groups of 16. Again the first entry is always transparent and unseen. [[Backgrounds]] use the first 8 groups of 16 (0-127) and [[Sprites]] use the last 8 groups (128-255).
- [[Tiles#8bpp|8bpp]] tiles can use the entire contents of CGRAM, except entry 0 which is always transparent. This also includes [[Mode 7]] tiles.
- [[Direct color]] tiles do not use the CGRAM palette at all, instead specifying an 8-bit color directly with their bits.

## See Also

- [[Tiles]]
- [[Backgrounds]]
- [[Sprites]]

## References

- [[SNES Development Manual]] - Book I A-17 CG-RAM
