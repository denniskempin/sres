# SNES Memory

> **See:** [SNES Memory Map](snes-memory-map.md)

> **See:** [SNES Memory Control](snes-memory-control.md)

#### Work RAM (WRAM)

Work RAM is mapped directly to the CPU bus, and can be additionally accessed indirectly via I/O ports (mainly for DMA transfer purposes).

> **See:** [SNES Memory Work RAM Access](snes-memory-work-ram-access.md)

#### Video Memory (OAM/VRAM/CGRAM)

All video memory can be accessed only during V-Blank, or Forced Blank.

Video memory isn't mapped to the CPU bus, and can be accessed only via I/O ports (for bigger transfers, this would be usually done via DMA).

> **See:** [SNES Memory OAM Access (Sprite Attributes)](snes-memory-oam-access-sprite-attributes.md)
> **See:** [SNES Memory VRAM Access (Tile and BG Map)](snes-memory-vram-access-tile-and-bg-map.md)
> **See:** [SNES Memory CGRAM Access (Palette Memory)](snes-memory-cgram-access-palette-memory.md)

Access during H-Blank doesn't seem to work too well - it is possible to change palette entries during H-Blank, but seems to work only during a few clock cycles, not during the full H-blank period.

#### Sound RAM

Sound RAM is mapped to a separate SPC700 CPU, not to the Main CPU. Accordingly, Sound RAM cannot be directly accessed by the Main CPU (nor by DMA). Instead, data transfers must be done by using some CPU-to-CPU software communication protocol. Upon Reset, this done by a Boot-ROM on the SPC700 side. For details, see:

> **See:** [SNES APU Main CPU Communication Port](snes-apu-main-cpu-communication-port.md)

#### DMA Transfers

DMA can be used to quickly transfer memory blocks to/from most memory locations (except Sound RAM isn't accessible via DMA, and WRAM-to-WRAM transfers don't work).

> **See:** [SNES DMA Transfers](snes-dma-transfers.md)
