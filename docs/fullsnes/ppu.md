---
summary: Overview of the SNES Picture Processing Unit (PPU), serving as an entry point to its sub-topics including control registers, background modes, sprites, video memory (OAM/VRAM/CGRAM), window/color-math, and interrupts.
keywords: PPU, video memory, sprites, backgrounds, priority
importance: 5
---

# SNES Picture Processing Unit (PPU)

> **See:** [SNES PPU Control](ppu_control.md)
> **See:** [SNES PPU BG Control](ppu_bg.md)
> **See:** [SNES PPU Rotation/Scaling](ppu_rotation.md)
> **See:** [SNES PPU Window](ppu_window.md)
> **See:** [SNES PPU Color-Math](ppu_color_math.md)
> **See:** [SNES PPU Timers and Status](ppu_timers.md)
> **See:** [SNES PPU Interrupts](ppu_interrupts.md)
> **See:** [SNES PPU Resolution](ppu_resolution.md)
> **See:** [SNES PPU Offset-Per-Tile Mode](ppu_offset_per_tile.md)

#### Video Memory (OAM/VRAM/CGRAM)

> **See:** [SNES PPU Sprites (OBJs)](ppu_sprites.md)
> **See:** [SNES PPU Video Memory (VRAM)](ppu_vram.md)
> **See:** [SNES PPU Color Palette Memory (CGRAM) and Direct Colors](ppu_cgram.md)

All video memory can be accessed only during V-Blank, or Forced Blank.

Video memory isn't mapped to the CPU bus, and be accessed only via I/O ports.

> **See:** [SNES Memory OAM Access (Sprite Attributes)](memory_oam.md)
> **See:** [SNES Memory VRAM Access (Tile and BG Map)](memory_vram.md)
> **See:** [SNES Memory CGRAM Access (Palette Memory)](memory_cgram.md)

The above OAM/VRAM/CGRAM I/O ports are usually accessed via DMA,

> **See:** [SNES DMA Transfers](dma.md)

#### Pinouts

> **See:** [SNES Audio/Video Connector Pinouts](av_connector.md)
> **See:** [SNES Pinouts PPU Chips](pinouts_ppu.md)

#### Background Priority Chart

```text
  Mode0    Mode1    Mode2    Mode3    Mode4    Mode5    Mode6    Mode7
  -        BG3.1a   -        -        -        -        -        -
  OBJ.3    OBJ.3    OBJ.3    OBJ.3    OBJ.3    OBJ.3    OBJ.3    OBJ.3
  BG1.1    BG1.1    BG1.1    BG1.1    BG1.1    BG1.1    BG1.1    -
  BG2.1    BG2.1    -        -        -        -        -        -
  OBJ.2    OBJ.2    OBJ.2    OBJ.2    OBJ.2    OBJ.2    OBJ.2    OBJ.2
  BG1.0    BG1.0    BG2.1    BG2.1    BG2.1    BG2.1    -        BG2.1p
  BG2.0    BG2.0    -        -        -        -        -        -
  OBJ.1    OBJ.1    OBJ.1    OBJ.1    OBJ.1    OBJ.1    OBJ.1    OBJ.1
  BG3.1    BG3.1b   BG1.0    BG1.0    BG1.0    BG1.0    BG1.0    BG1
  BG4.1    -        -        -        -        -        -        -
  OBJ.0    OBJ.0    OBJ.0    OBJ.0    OBJ.0    OBJ.0    OBJ.0    OBJ.0
  BG3.0    BG3.0a   BG2.0    BG2.0    BG2.0    BG2.0    -        BG2.0p
  BG4.0    BG3.0b   -        -        -        -        -        -
  Backdrop Backdrop Backdrop Backdrop Backdrop Backdrop Backdrop Backdrop
```

Whereas,

```text
  .N     per-tile priority setting (in BG Map and OAM entries)
  .Np    per-pixel priority setting (for 128-color BG2 in Mode7)
  .Na/b  per-screen priority bit (in port 2105h) (plus .N as usually)
```
