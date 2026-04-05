# SNES DMA Transfers

The SNES includes eight DMA channels, which can be used for H-DMA or GP-DMA.

> **See:** [SNES DMA and HDMA Start/Enable Registers](snes-dma-and-hdma-start-enable-registers.md)
> **See:** [SNES DMA and HDMA Channel 0..7 Registers](snes-dma-and-hdma-channel-0-7-registers.md)
> **See:** [SNES DMA and HDMA Notes](snes-dma-and-hdma-notes.md)

#### H-DMA (H-Blank DMA)

H-DMA transfers are automatically invoked on H-Blank, each H-DMA is limited to a single unit (max 4 bytes) per scanline. This is commonly used to manipulate PPU I/O ports (eg. to change scroll offsets). Related registers can found here:

> **See:** [SNES I/O Map](snes-i-o-map.md)
> **See:** [SNES Picture Processing Unit (PPU)](snes-picture-processing-unit-ppu.md)

#### GP-DMA (General Purpose DMA)

GP-DMA can manually invoked by software, allowing to transfer larger amounts of data (max 10000h bytes). This is commonly used to transfer WRAM or ROM (on A-Bus side) to/from WRAM, OAM, VRAM, CGRAM (on B-Bus side). Related registers are:

> **See:** [SNES Memory Work RAM Access](snes-memory-work-ram-access.md)
> **See:** [SNES Memory OAM Access (Sprite Attributes)](snes-memory-oam-access-sprite-attributes.md)
> **See:** [SNES Memory VRAM Access (Tile and BG Map)](snes-memory-vram-access-tile-and-bg-map.md)
> **See:** [SNES Memory CGRAM Access (Palette Memory)](snes-memory-cgram-access-palette-memory.md)
