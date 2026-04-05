---
title: "SNES PPU for NES developers"
source_url: "https://snes.nesdev.org/wiki/SNES_PPU_for_NES_developers"
pageid: 12
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The SNES PPU and NES PPU use every similar concepts for displaying graphics. This article will summarize some important differences and similarities, and lay out some basics before you learn about things more in-depth.

## VRAM

[Video RAM](https://snes.nesdev.org/w/index.php?title=VRAM&action=edit&redlink=1 "VRAM (page does not exist)") is 64KB and contained within the console. Additional video memory cannot be added via cartridge. VRAM contains the tilemaps used for backgrounds as well as the graphics data for background tiles and sprite tiles. You can decide how much VRAM to dedicate to each use, and there are [[PPU registers|registers]] that set the base address for different things.

Like the NES, VRAM is accessed by setting an address via registers, and then writing the data you want to store to other registers, with the address automatically increasing after each write. Unlike the NES, the VRAM address is spread across two registers, and VRAM is 16-bit, so the VRAM data is also spread across two registers (for accessing the low and high byte at each VRAM address). This means that you're writing addresses between $0000-$7FFF instead of $0000-$FFFF like you might expect.

## Palettes/[[CGRAM]]

The palette consists of 256 RGB colors, with 5 bits used for the red, green and blue values of each color. You can choose to think of it as 8 background palettes and 8 sprite palettes, each with 15 colors plus a "transparency" color, though different [[Background modes]] can change the specifics.

VRAM and CGRAM are accessed with different registers. Colors in the palette are RGB values (with 5 bits per color channel).

## DMA

[[DMA]] is much more flexible than it is on the NES. You can configure the SNES's DMA unit to write to *any* PPU register, as well as to other registers in the same address range as them. As a result, DMA can transfer data directly from the cartridge (or CPU RAM) into VRAM, CGRAM, OAM, or into [CPU RAM](https://snes.nesdev.org/w/index.php?title=WRAM&action=edit&redlink=1 "WRAM (page does not exist)").

DMA can work in reverse (copy from VRAM into CPU RAM) as well as do fills. It cannot, however, copy from one section of CPU RAM to another. For that, you can use the new [[MVN]] or [[MVP]] instructions.

### HDMA

[[HDMA]] is a special case of DMA, in which a small amount of data is transferred at the end of a scanline, during [HBlank](https://snes.nesdev.org/w/index.php?title=HBlank&action=edit&redlink=1 "HBlank (page does not exist)"). This makes raster effects much easier, as they no longer require precise timing. Instead, you choose a register and provide a table that lists both a series of values to write into that register, as well a series of scanline counts to wait before the next value is written.

There is still [an interrupt](https://snes.nesdev.org/w/index.php?title=H/V_IRQ&action=edit&redlink=1 "H/V IRQ (page does not exist)") that allows you to do things the old way, and you can specify a horizontal coordinate to trigger the interrupt at too.

## Background modes

There are a variety of [[Background modes]] that change how the background layers work. The most common (mode 1) provides two layers with 16 colors per tile, and a third layer with 4 colors per tile. The third layer may be given priority over everything else, which makes it useful as a HUD or status bar.

While the SNES produces a similar 240 line picture to the NES, by default it blanks the top and bottom 8 rows to increase vblank time. PPU register $2133 [[PPU registers#SETINI|SETINI]] can be used to enable "overscan" to make it more like the NES' picture viewport.

The SNES always blanks the first scanline of BG rendering, so the 240 line mode is actually only 239 visible lines. The 224 line mode similarly starts rendering 1 "blank" scanline higher than might be expected. A Y scroll position of 0 will *not* show the top pixel row of the BG because of this blank first line, and in 224 line mode one extra (224th) line of BG will be seen at the bottom.

Positions given to the scroll registers are relative to the top left of the active picture (minus the blanked 1st line), so switching between 224 and 239 lines doesn't just crop the picture with letterboxing, but the 224 line version is also shifted 8 lines lower on the screen. On SNES mid-screen scroll changes are also still relative to the top left, unlike the NES.

## Tilemaps/Nametables

SNES tilemaps consist of a series of 16-bit values, containing the palette, priority bit, horizontal/vertical flips, and a tile number. There is no separate attribute table, and a tilemap can access 1024 different tiles instead of only 256.

Tilemaps are 32x32 (instead of the NES's 32x30) and a background layer can decide to have a single tilemap, arrange two horizontally, arrange two vertically, or have a square of four different tilemaps. A background layer can also choose to have 16x16 tiles instead of 8x8.

## Sprites/OBJ/[[OAM]]

Sprite limits are much higher. There are 128 [sprites](https://snes.nesdev.org/w/index.php?title=Sprite_(hardware)&action=edit&redlink=1 "Sprite (hardware) (page does not exist)"), and 32 can be on the same [scanline](https://snes.nesdev.org/w/index.php?title=Scanline&action=edit&redlink=1 "Scanline (page does not exist)") before sprites start to disappear. There is additionally a limit of 34 sprite tiles per scanline.

Sprites can be several different sizes, e.g. 8x8, 16x16, 32x32, 64x64. You can pick two sprite sizes for the PPU to use at one time, and each individual sprite can pick one of those two sizes for itself. There is no 8x16 sprite size.

Sprite data is similar in structure to the NES (X, Y, tile, attributes) but in a different order, and with some additional properties:

- X has a 9th bit allowing a sprite to be placed partially off the left side.
- You have 8 15-color palettes to choose from. Sprite tiles are always 4bpp.
- Priority now has 4 values, allowing each sprite to be placed above, below, or between the various BG layers.

The sprite rendering layer is referred to as OBJ, though the sprite priority attribute potentially makes it work like several layers. [[Color math]] and [[Windowing]] can apply to the OBJ layer in the same way as the BG layers.

Sprites are delayed vertically by 1 scanline, just as on NES, so scroll and sprite positions will work unmodified on SNES, though in the default 224 lines rendering mode, you may wish to move both the sprites and BG 8 lines higher. Unlike the NES, because the first active scanline is always hidden, sprites can appear on the first scanline of the visible picture. (See [Background modes](#Background_modes) above.)

## Register reference

### PPUCTRL ($2000 write)

```
7  bit  0
---- ----
NPHB SIYX
|||| ||||
|||| |||+- Horizontal scroll high bit. See BG1HOFS ($210D)
|||| ||+-- Vertical scroll high bit. See BG1VOFS ($210E)
|||| |+--- VRAM address increment. See VMAIN ($2115)
|||| +---- Sprite pattern table base address. See OBSEL ($2101)
|||+------ Background pattern table base address. See BG12NBA
||+------- Sprite size select. See OBSEL ($2101)
|+-------- EXTBG control. See SETINI ($2133)
+--------- Vertical blanking NMI enable. See NMITIMEN ($4200)
```

Corresponds to [[PPU registers#BGnHOFS|BG1HOFS]] ($210D write twice), [[PPU registers#BGnVOFS|BG1VOFS]] ($210E write twice), [[PPU registers#VMAIN|VMAIN]] ($2115), [[PPU registers#OBJSEL|OBSEL]] ($2101 write), [[PPU registers#BG12NBA|BG12NBA]] ($210B), [[PPU registers#SETINI|SETINI]] ($2133 write), and [[MMIO registers#NMITIMEN|NMITIMEN]] ($4200 write).

EXTBG input is useful this time. The stock Control Deck configures it to receive the same texture data that mode 7 reads, to provide pixel-by-pixel control of background to sprite priority.

### PPUMASK ($2001 write)

```
7  bit  0
---- ----
TTTs bMmG
|||| ||||
|||| |||+- Gray
|||| ||+-- Show BG in left 8 dots. See TMW ($212E)
|||| |+--- Show sprites in left 8 dots. See TMW ($212E)
|||| +---- Render BG. See TM ($212C) and INIDISP ($2100)
|||+------ Render sprites. See TM ($212C) and INIDISP ($2100)
+++------- Set tint color. See COLDATA ($2132)
```

Corresponds to [[PPU registers#TMW|TMW]] ($212E write), [[PPU registers#TM|TM]] ($212C), [[PPU registers#INIDISP|INIDISP]] ($2100 write), and [[PPU registers#COLDATA|COLDATA]] ($2132 write).

Gray has no direct counterpart on S-PPU. For brightening the whole screen, use additive color math with COLDATA.

Bits 1 and 2 are for clipping the left side. This is less necessary on S-PPU because sprites' horizontal position has a sign bit. The effect can be done with a window.

Bits 3 and 4 are for enabling and disabling the background and sprites as a whole. When both are off, the PPU stops rendering entirely and lets the CPU read and write the VRAM port. This is often used to load entirely new data at the start of a scene and sometimes used for letterboxing to give more time to write to VRAM. To turn off rendering, use INIDISP.

### PPUSTATUS ($2002 read)

```
7  bit  0
---- ----
432. ....
|||
||+------- Range/time over. See STAT77 ($213E)
|+-------- Sprite 0 hit
+--------- Unacknowledged vblank NMI. See RDNMI ($4210)
```

There is no direct counterpart to sprite 0 hit on S-PPU. For raster timing, use H/V IRQ through [[MMIO registers#NMITIMEN|NMITIMEN]] ($4200 write), [[MMIO registers#HTIME|HTIMEL/HTIMEH]] ($4207-$4208 write), and [[MMIO registers#VTIME|VTIMEL/VTIMEH]] ($4209/$420A) instead, or use HDMA to write to PPU registers in the background.

### OAMADDR ($2003 write)

Corresponds to [[PPU registers#OAMADD|OAMADDL/OAMADDH ($2102 write)]], except that it is word-addressed, OAM entries have a different layout (X is the first byte), and there is an additional table at the end holding X sign bit and sprite sizes. Manually changing the address is nowhere near as finicky as the NES PPU OAM DRAM interface.

### OAMDATA ($2004 write)

Corresponds to [[PPU registers#OAMDATA|OAMDATA ($2104 write)]]. Some later NES PPU revisions have an unreliable read port here as well; use OAMDATAREAD ($2138 read).

### PPUSCROLL ($2005 write)

Sets the fine horizontal scroll and the bits of the secondary VRAM address representing the coarse X and Y scroll position. Corresponds to [BG1HOFS](https://snes.nesdev.org/w/index.php?title=/PPU_registers&action=edit&redlink=1 "/PPU registers (page does not exist)") ($210D write twice) and [[PPU registers#BGnVOFS|BG1VOFS]] ($210E write twice).

### PPUADDR ($2006 write)

Corresponds to [[PPU registers#VMADD|VMADDL and VMADDH]] ($2116-$2117 write). This is word addressed, meaning the written value is in units of 2 bytes.

On the NES, addresses $3F00-$3FFF refer to CGRAM (the palette). This corresponds to [[PPU registers#CGADD|CGADD]] ($2121).

### PPUDATA ($2007 read/write)

Corresponds to [[PPU registers#VMDATA|VMDATAL and VMDATAH]] ($2118-$2119 write) and [[PPU registers#VMDATAREAD|VMDATALREAD and VMDATAHREAD]] ($2139-$213A read). The address increments only if the low or high byte is written, depending on the state of VMAIN.

On the NES PPU, writes to video memory $3F00-$3FFF instead go to CGRAM (the palette). This corresponds to [[PPU registers#CGDATA|CGDATA]] ($2122 write twice). Some later NES PPU revisions have a read port here as well; use [[PPU registers#CGDATAREAD|CGDATAREAD]] ($213B read twice).
