# SNES Picture Processing Unit (PPU)

> **See:** [SNES PPU Control](snes-ppu-control.md)
> **See:** [SNES PPU BG Control](snes-ppu-bg-control.md)
> **See:** [SNES PPU Rotation/Scaling](snes-ppu-rotation-scaling.md)
> **See:** [SNES PPU Window](snes-ppu-window.md)
> **See:** [SNES PPU Color-Math](snes-ppu-color-math.md)
> **See:** [SNES PPU Timers and Status](snes-ppu-timers-and-status.md)
> **See:** [SNES PPU Interrupts](snes-ppu-interrupts.md)
> **See:** [SNES PPU Resolution](snes-ppu-resolution.md)
> **See:** [SNES PPU Offset-Per-Tile Mode](snes-ppu-offset-per-tile-mode.md)

#### Video Memory (OAM/VRAM/CGRAM)

> **See:** [SNES PPU Sprites (OBJs)](snes-ppu-sprites-objs.md)
> **See:** [SNES PPU Video Memory (VRAM)](snes-ppu-video-memory-vram.md)
> **See:** [SNES PPU Color Palette Memory (CGRAM) and Direct Colors](snes-ppu-color-palette-memory-cgram-and-direct-colors.md)

All video memory can be accessed only during V-Blank, or Forced Blank.

Video memory isn't mapped to the CPU bus, and be accessed only via I/O ports.

> **See:** [SNES Memory OAM Access (Sprite Attributes)](snes-memory-oam-access-sprite-attributes.md)
> **See:** [SNES Memory VRAM Access (Tile and BG Map)](snes-memory-vram-access-tile-and-bg-map.md)
> **See:** [SNES Memory CGRAM Access (Palette Memory)](snes-memory-cgram-access-palette-memory.md)

The above OAM/VRAM/CGRAM I/O ports are usually accessed via DMA,

> **See:** [SNES DMA Transfers](snes-dma-transfers.md)

#### Pinouts

> **See:** [SNES Audio/Video Connector Pinouts](snes-audio-video-connector-pinouts.md)
> **See:** [SNES Pinouts PPU Chips](snes-pinouts-ppu-chips.md)

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
