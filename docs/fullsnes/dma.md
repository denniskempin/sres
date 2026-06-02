---
summary: Overview of the SNES DMA subsystem, covering the eight DMA channels and the two transfer modes (H-DMA and GP-DMA), their use cases, and pointers to the related register and memory documentation.
keywords: DMA, H-DMA, GP-DMA, PPU, H-Blank
importance: 5
---

# SNES DMA Transfers

The SNES includes eight DMA channels, which can be used for H-DMA or GP-DMA.

> **See:** [SNES DMA and HDMA Start/Enable Registers](dma_start_enable.md)
> **See:** [SNES DMA and HDMA Channel 0..7 Registers](dma_channel_regs.md)
> **See:** [SNES DMA and HDMA Notes](dma_notes.md)

#### H-DMA (H-Blank DMA)

H-DMA transfers are automatically invoked on H-Blank, each H-DMA is limited to a single unit (max 4 bytes) per scanline. This is commonly used to manipulate PPU I/O ports (eg. to change scroll offsets). Related registers can found here:

> **See:** [SNES I/O Map](io_map.md)
> **See:** [SNES Picture Processing Unit (PPU)](ppu.md)

#### GP-DMA (General Purpose DMA)

GP-DMA can manually invoked by software, allowing to transfer larger amounts of data (max 10000h bytes). This is commonly used to transfer WRAM or ROM (on A-Bus side) to/from WRAM, OAM, VRAM, CGRAM (on B-Bus side). Related registers are:

> **See:** [SNES Memory Work RAM Access](memory_wram.md)
> **See:** [SNES Memory OAM Access (Sprite Attributes)](memory_oam.md)
> **See:** [SNES Memory VRAM Access (Tile and BG Map)](memory_vram.md)
> **See:** [SNES Memory CGRAM Access (Palette Memory)](memory_cgram.md)
