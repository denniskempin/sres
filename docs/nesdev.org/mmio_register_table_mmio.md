---
title: "MMIO register table/MMIO"
source_url: "https://snes.nesdev.org/wiki/MMIO_register_table/MMIO"
pageid: 44
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

See: [[MMIO register table]]

| Name | Address | Bits | Type | Notes |
| --- | --- | --- | --- | --- |
| [[MMIO registers#APUIOn|APUIO0]] [[MMIO registers#APUIOn|APUIO1]] [[MMIO registers#APUIOn|APUIO2]] [[MMIO registers#APUIOn|APUIO3]] | $2140 $2141 $2142 $2143 | DDDD DDDD | RW8 | Data to/from APU. |
| [[MMIO registers#WMDATA|WMDATA]] | $2180 | DDDD DDDD | RW8 | Data to/from S-WRAM, increments WMADD. |
| [[MMIO registers#WMADD|WMADDL]] [[MMIO registers#WMADD|WMADDM]] [[MMIO registers#WMADD|WMADDH]] | $2181 $2182 $2183 | LLLL LLLL MMMM MMMM .... ...H | W24 | S-WRAM address for WMDATA access. |
| [[MMIO registers#JOYOUT|JOYOUT]] | $4016 | .... ...D | W8 | Output to joypads (latches standard controllers). |
| [[MMIO registers#JOYSER0|JOYSER0]] | $4016 | .... ..DD | R8 | Input from joypad 1. |
| [[MMIO registers#JOYSER1|JOYSER1]] | $4017 | ...1 11DD | R8 | Always 1 (1), input from joypad 2 (D). |
| [[MMIO registers#NMITIMEN|NMITIMEN]] | $4200 | N.VH ...J | W8 | Vblank NMI enable (N), timer IRQ mode (VH), joypad auto-read enable (J). |
| [[MMIO registers#WRIO|WRIO]] | $4201 | 21DD DDDD | W8 | Joypad port 2 I/O (2), joypad port 1 I/O (1), unused I/O (D). |
| [[MMIO registers#WRMPYA|WRMPYA]] | $4202 | DDDD DDDD | W8 | Unsigned multiplication factor A. |
| [[MMIO registers#WRMPYB|WRMPYB]] | $4203 | DDDD DDDD | W8 | Unsigned multiplication factor B, starts 8-cycle multiplication. |
| [[MMIO registers#WRDIV|WRDIVL]] [[MMIO registers#WRDIV|WRDIVH]] | $4204 $4205 | LLLL LLLL HHHH HHHH | W16 | Unsigned dividend. |
| [[MMIO registers#WRDIVB|WRDIVB]] | $4206 | DDDD DDDD | W8 | Unsigned divisor, starts 16-cycle division. |
| [[MMIO registers#HTIME|HTIMEL]] [[MMIO registers#HTIME|HTIMEH]] | $4207 $4208 | .... ...H LLLL LLLL | W16 | H counter target for timer IRQ. |
| [[MMIO registers#VTIME|VTIMEL]] [[MMIO registers#VTIME|VTIMEH]] | $4209 $420A | .... ...H LLLL LLLL | W16 | V counter target for timer IRQ. |
| [[DMA registers#MDMAEN|MDMAEN]] | $420B | 7654 3210 | W8 | DMA enable. |
| [[DMA registers#HDMAEN|HDMAEN]] | $420C | 7654 3210 | W8 | HDMA enable. |
| [[MMIO registers#MEMSEL|MEMSEL]] | $420D | .... ...F | W8 | FastROM enable (F). |
| [[MMIO registers#RDNMI|RDNMI]] | $4210 | N... VVVV | R8 | Vblank NMI flag (N), CPU version (V). |
| [[MMIO registers#TIMEUP|TIMEUP]] | $4211 | T... .... | R8 | Timer IRQ flag (T). |
| [[MMIO registers#HVBJOY|HVBJOY]] | $4212 | VH.. ...J | R8 | Vblank flag (V), hblank flag (H), joypad auto-read in-progress flag (J). |
| [[MMIO registers#RDIO|RDIO]] | $4213 | 21DD DDDD | R8 | Joypad port 2 I/O (2), joypad port 1 I/O (1), unused I/O (D). |
| [[MMIO registers#RDDIV|RDDIVL]] [[MMIO registers#RDDIV|RDDIVH]] | $4214 $4215 | LLLL LLLL HHHH HHHH | R16 | Unsigned quotient. |
| [[MMIO registers#RDMPY|RDMPYL]] [[MMIO registers#RDMPY|RDMPYH]] | $4216 $4217 | LLLL LLLL HHHH HHHH | R16 | Unsigned product or unsigned remainder. |
| [[MMIO registers#JOY1|JOY1L]] [[MMIO registers#JOY1|JOY1H]] [[MMIO registers#JOY2|JOY2L]] [[MMIO registers#JOY2|JOY2H]] [[MMIO registers#JOY3|JOY3L]] [[MMIO registers#JOY3|JOY3H]] [[MMIO registers#JOY4|JOY4L]] [[MMIO registers#JOY4|JOY4H]] | $4218 $4219 $421A $421B $421C $421D $421E $421F | LLLL LLLL HHHH HHHH | R16 | 16-bit joypad auto-read result (first read high to last read low). |
