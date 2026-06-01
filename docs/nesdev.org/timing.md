---
title: "Timing"
source_url: "https://snes.nesdev.org/wiki/Timing"
pageid: 50
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Timing of the SNES hardware.

## Master Clock

The SNES master clock:

- NTSC: 945/44 MHz ≈ 21.4773 MHz (6 times chroma).
- PAL: 21.281370 MHz (4.8 times chroma). This is synthesized by multiplying a 17.734475 MHz (4 times chroma) crystal by 6/5.

## CPU

A 65816 CPU "cycle" can take 6, 8 or 12 master clocks on the SNES, depending on the memory region accessed, and the [[MMIO registers#MEMSEL|MEMSEL]] fast-ROM setting.

This gives some commonly quoted SNES CPU speeds, though none of them tell a complete story:

- 3.58 MHz fast-ROM (6-clocks per cycle)
- 2.68 MHz slow-ROM (8-clocks per cycle)
- 1.79 MHz other (12-clocks per cycle)

The speed of access depends on the memory region:[[1]](#cite_note-1)

- 6-clocks for fast-ROM access, enabled via MEMSEL and accessed at an address of $800000 of higher.
- 8-clocks for slow-ROM access.
- 8-clocks for internal S-WRAM.
- 6-clocks for most [[MMIO registers]].
- 12-clocks for [[MMIO registers#JOYSER0|JOYSER0 and JOYSER1]].
- 6-clocks for "internal" cycles not accessing memory (e.g. 2nd cycle of NOP)

## Video

### Scanline

- 1364 master clocks = 341 dot cycles.
- The CPU pauses for 40 master clocks in the middle of each scanline for DRAM refresh, leaving 1324 active clocks per line.
- The left edge of the picture begins at clock 88 of the line and continues until 1112.
- Scanline 0 is the end of vblank and beginning of rendering. It is hidden, and displays as a blank line.
- Scanlines 1-224 or 239 will normally render the visible image, unless force blanking is applied.
- Scanline 261 or 311 is the last line of vertical blank (NTSC or PAL), after which the next frame begins rendering.
- With [[PPU registers#SETINI|interlacing]] on, 1 extra scanline will appear with each even frame, and one scanline outside the visible picture will be slightly shortened or lengthened for color synchronization.

#### Short and Long Scanlines

Scanlines are normally 1364 master clocks, but there are two special cases:

- Short scanline: NTSC with interlace off, field=1, V=240.
- Long scanline: PAL with interlace on, field=1, V=311.

This means every odd frame may have one adjusted scanline, depending on the interlacing setting and region. It affects the same line regardless of the overscan setting.

These adjusted scanlines are always during vertical blank, so they do not affect the visible picture directly, but they are necessary to maintain synchronization with the colour signal.

Timings:

- Normal: 1364 clocks, 340 dots. 336 dots of 4-clocks, 4 dots of 5-clocks.
- Long: 1368 clocks, 341 dots. 337 dots of 4-clocks, 4 dots of 5-clocks.
- Short: 1360 clocks, 340 dots. 340 dots of 4-clocks.

### Vertical Blank

- Begins on vertical line V=225 or V=240 based on ([[PPU registers#SETINI|SETINI]] overscan setting).
- Ends after vertical line V=261 (NTSC) or V=311 (PAL). The next line is V=0.
- This allows 37 (NTSC) or 87 (PAL) lines of vertical blank normally, or 22 (NTSC) / 72 (PAL) with overscan.
- With 1324 master clocks per line available, and [[DMA]] taking 8 clocks per byte, this gives an upper bound on DMA bandwidth per blank without [[PPU registers#INIDISP|forced blanking]]:

| Region | [[PPU registers#SETINI|Overscan]] | VBlank Lines | VBlank Clocks | DMA Bandwidth |
| --- | --- | --- | --- | --- |
| NTSC | 224 | 37 | 48,988 | 5.9 kb |
| NTSC | 239 | 22 | 29,128 | 3.5 kb |
| PAL | 224 | 87 | 115,188 | 14.0 kb |
| PAL | 239 | 72 | 95,328 | 11.6 kb |

### Frame

The total clocks per frame is dependent on many factors:

- Some scanlines are slightly short or long.
- [[PPU registers#SETINI|Interlacing]] adds an extra scanline on even frames.
- DRAM refresh uses 40 clocks in the middle of each line.
- [[HDMA]] uses varying amounts of clocks just past the active part of each line.

Approximating a total, ignoring the factors above and assuming a constant number of clocks per scanline:

| Region | Scanlines | Master Clocks | Available Clocks |
| --- | --- | --- | --- |
| NTSC | 262 | 357,368 | 346,888 |
| PAL | 312 | 425,568 | 413,088 |

## Audio

The [[S-SMP]] runs on its own clock, independent of the main CPU. Its ceramic resonator is less precise, usually found to be ~0.25% higher than specified, and may also rise by ~0.04% as the console warms up.

- S-DSP clock: 24.576MHz (ceramic resonator)
- S-DSP internal clock: 3.072 MHz (÷8)
- SPC-700 processor: 1.024 MHz (÷24)
- DAC samplerate: 32000 Hz by specification (÷(24×32)), varies from console to console anywhere from 32000 Hz to 32160 Hz [[2]](#cite_note-2)

## Tools

- [SnesInstructionCycleTool](https://novasquirrel.github.io/SnesInstructionCycleTool/) - calculates CPU cycles and master clock cycles under different conditions

## References

1. [↑](#cite_ref-1) [Fullsnes](https://problemkaputt.de/fullsnes.htm#cpuclockcycles): CPU Clock Cycles
2. [↑](#cite_ref-2) [Forum post](https://forums.nesdev.org/viewtopic.php?t=24610): S-SMP clock speed measurement tool
