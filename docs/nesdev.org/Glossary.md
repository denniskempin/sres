---
title: "Glossary"
source_url: "https://snes.nesdev.org/wiki/Glossary"
pageid: 85
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

## CPU

5A22
:   Ricoh's designation for the S-CPU.

A Bus
:   Memory address bus. ROM, WRAM, and coprocessors are on this bus.

B Bus
:   I/O address bus, used mostly for communication with the S-PPU and S-SMP. The S-WRAM also has a port on the B Bus, primarily used for block copies from ROM to WRAM.

DMA
:   Direct Memory Access. A unit of the S-CPU's memory controller capable of briefly pausing the 65C816 to perform copies, either as a block copy or as HDMA.

HDMA
:   Horizontal blank Direct Memory Access. Sends a word of data to the PPU between one scanline and the next to produce line scrolling, window shaping, and other raster effects. (Compare Copper on Amiga.)

S-CPU
:   The central processing unit. It consists of a 65C816 core licensed from Western Design Center and a custom memory controller.

S-WRAM
:   Work RAM. A 128 KiB DRAM.

## Graphics

[[Backgrounds|Background]]
:   A grid-based layer containing a tileset, one or more tilemaps, a scroll offset and other attributes.

CGRAM
:   256 15-bit words of memory located in the PPU that holds the palette data.

CHR

Character
:   Another word for tile.

Nametable
:   Another word for tilemap.

Object
:   Nintendo's term for [[Sprites|Sprite]].

OAM

[[Sprites#OAM|Object Attribute Memory]]
:   544 bytes of memory located in the PPU that holds the sprite table.

[[Palettes|Palette]]
:   A color lookup table.

Raster
:   A horizontal line of the output picture. (Also: scanline)

Scanline
:   A horizontal line of the output picture. (Also: raster)

Sliver
:   An 8x1 pixel segment of a tile.

[[Sprites|Sprite]]
:   A 16-colour (4bpp) square of tiles, usually 8×8, 16×16, or 32×32 pixels in size, that can be drawn anywhere on the screen.

[[Tiles|Tile]]
:   A small square graphic.
:   SNES tiles are 8×8 pixels in size and are stored in a variety of formats in VRAM. Adjacent tiles can be combined to create larger tiles for backgrounds and sprites.

[[Tilemaps|Tilemap]]
:   A grid (in VRAM) that describe which tiles and palettes to draw for a background layer.

[[Tilemaps#Mode 7|Mode 7 Tilemap]]
:   A 128×128 grid of 8-bit tile indexes for the Mode-7 background (in VRAM).

[[Tilemap#Format|Tilemap Entry]]
:   A 16 bit word containing the tile index, flip, palette index and priority.

PPU

Picture Processing Unit
:   Generates video output from VRAM, CGRAM and OAM memory.

VRAM
:   Video-RAM. The SNES has two 32 KiB Video-RAM chips (64 KiB total VRAM).
:   The VRAM holds [[Tiles|tile]] data, [[Tilemaps|tilemap]] data and [[Offset-per-tile]] data

## Sound

ARAM
:   Audio RAM. A 64 KiB memory containing the sound driver, sound driver work RAM, sequences, samples, and echo buffer.

S-DSP
:   The digital signal processor in SHVC-Sound. It plays BRR samples and updates the echo buffer.

S-SMP
:   The CPU core in SHVC-Sound. It communicates with the S-CPU and translates musical sequences into commands to the S-DSP.

SHVC-Sound
:   The audio subsystem of the SNES, designed by Sony. It consists of the S-SMP, S-DSP, and ARAM.

SPC700
:   The 8-bit 65C02-like architecture of the S-SMP.
