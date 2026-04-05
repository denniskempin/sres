---
title: "MMIO register table"
source_url: "https://snes.nesdev.org/wiki/MMIO_register_table"
pageid: 42
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Complete MMIO register summary

| PPU registers $2100-213F ([[MMIO register table/PPU|table source]]) | | | | |
| --- | --- | --- | --- | --- |
| Name | Address | Bits | Type | Notes |
| [[PPU registers#INIDISP|INIDISP]] | $2100 | F... BBBB | W8 | Forced blanking (F), screen brightness (B). |
| [[PPU registers#OBJSEL|OBJSEL]] | $2101 | SSSN NbBB | W8 | OBJ sprite size (S), name secondary select (N), name base address (B). |
| [[PPU registers#OAMADD|OAMADDL]]  [[PPU registers#OAMADD|OAMADDH]] | $2102  $2103 | AAAA AAAA  P... ...B | W16 | OAM word address (A).  Priority rotation (P), address high bit (B). |
| [[PPU registers#OAMDATA|OAMDATA]] | $2104 | DDDD DDDD | W8x2 | OAM data write byte (2x for word) (D), increments OAMADD byte. |
| [[PPU registers#BGMODE|BGMODE]] | $2105 | 4321 PMMM | W8 | Tilemap tile size (#), BG3 priority (P), BG mode (M). |
| [[PPU registers#MOSAIC|MOSAIC]] | $2106 | SSSS 4321 | W8 | Mosaic size (S), mosaic BG enable (#). |
| [[PPU registers#BGnSC|BG1SC]]  [[PPU registers#BGnSC|BG2SC]]  [[PPU registers#BGnSC|BG3SC]]  [[PPU registers#BGnSC|BG4SC]] | $2107  $2108  $2109  $210A | AAAA AAYX | W8 | Tilemap VRAM address (A), vertical tilemap count (Y), horizontal tilemap count (X). |
| [[PPU registers#BG12NBA|BG12NBA]] | $210B | BBBB AAAA | W8 | BG2 CHR base address (B), BG1 CHR base address (A). |
| [[PPU registers#BG34NBA|BG34NBA]] | $210C | DDDD CCCC | W8 | BG4 CHR base address (D), BG3 CHR base address (C). |
| [[PPU registers#BGnHOFS|BG1HOFS]]  [[PPU registers#M7HOFS|M7HOFS]]  [[PPU registers#BGnVOFS|BG1VOFS]]  [[PPU registers#M7VOFS|M7VOFS]] | $210D    $210E | .... ..XX XXXX XXXX  ...x xxxx xxxx xxxx  .... ..YY YYYY YYYY  ...y yyyy yyyy yyyy | W8x2  W8x2  W8x2  W8x2 | BG1 horizontal scroll (X).  Mode 7 horizontal scroll (x).  BG1 vertical scroll (Y).  Mode 7 vertical scroll (y). |
| [[PPU registers#BGnHOFS|BG2HOFS]]  [[PPU registers#BGnVOFS|BG2VOFS]]  [[PPU registers#BGnHOFS|BG3HOFS]]  [[PPU registers#BGnVOFS|BG3VOFS]]  [[PPU registers#BGnHOFS|BG4HOFS]]  [[PPU registers#BGnVOFS|BG4VOFS]] | $210F  $2110  $2111  $2112  $2113  $2114 | .... ..XX XXXX XXXX  .... ..YY YYYY YYYY | W8x2  W8x2 | BG horizontal scroll (X).  BG vertical scroll (Y). |
| [[PPU registers#VMAIN|VMAIN]] | $2115 | M... RRII | W8 | VRAM address increment mode (M), remapping (R), increment size (I). |
| [[PPU registers#VMADD|VMADDL]]  [[PPU registers#VMADD|VMADDH]] | $2116  $2117 | LLLL LLLL  hHHH HHHH | W16 | VRAM word address. |
| [[PPU registers#VMDATA|VMDATAL]]  [[PPU registers#VMDATA|VMDATAH]] | $2118  $2119 | LLLL LLLL  HHHH HHHH | W16 | VRAM data write. Increments VMADD after write according to VMAIN setting. |
| [[PPU registers#M7SEL|M7SEL]] | $211A | RF.. ..YX | W8 | Mode 7 tilemap repeat (R), fill (F), flip vertical (Y), flip horizontal (X). |
| [[PPU registers#M7A|M7A]] | $211B | DDDD DDDD dddd dddd | W8x2 | Mode 7 matrix A or signed 16-bit multiplication factor. |
| [[PPU registers#M7B|M7B]] | $211C | DDDD DDDD dddd dddd | W8x2 | Mode 7 matrix B or signed 8-bit multiplication factor. |
| [[PPU registers#M7n|M7C]] | $211D | DDDD DDDD dddd dddd | W8x2 | Mode 7 matrix C |
| [[PPU registers#M7n|M7D]] | $211E | DDDD DDDD dddd dddd | W8x2 | Mode 7 matrix D |
| [[PPU registers#M7X|M7X]] | $211F | ...X XXXX XXXX XXXX | W8x2 | Mode 7 center X |
| [[PPU registers#M7Y|M7Y]] | $2120 | ...Y YYYY YYYY YYYY | W8x2 | Mode 7 center Y |
| [[PPU registers#CGADD|CGADD]] | $2121 | AAAA AAAA | W8 | CGRAM word address. |
| [[PPU registers#CGDATA|CGDATA]] | $2122 | .BBB BBGG GGGR RRRR | W8x2 | CGRAM data write, increments CGADD byte address after each write. |
| [[PPU registers#W12SEL|W12SEL]] | $2123 | DdCc BbAa | W8 | Enable (ABCD) and Invert (abcd) windows for BG1 (AB) and BG2 (CD). |
| [[PPU registers#W34SEL|W34SEL]] | $2124 | DdCc BbAa | W8 | Enable (EFGH) and Invert (efgh) windows for BG3 (EF) and BG2 (GH). |
| [[PPU registers#WOBJSEL|WOBJSEL]] | $2125 | LlKk JjIi | W8 | Enable (IJKL) and Invert (ijkl) windows for OBJ (IJ) and color (KL). |
| [[PPU registers#WH0|WH0]] | $2126 | LLLL LLLL | W8 | Window 1 left position. |
| [[PPU registers#WH1|WH1]] | $2127 | RRRR RRRR | W8 | Window 1 right position. |
| [[PPU registers#WH2|WH2]] | $2128 | LLLL LLLL | W8 | Window 2 left position. |
| [[PPU registers#WH3|WH3]] | $2129 | RRRR RRRR | W8 | Window 2 right position. |
| [[PPU registers#WBGLOG|WBGLOG]] | $212A | 4433 2211 | W8 | Window mask logic for BG layers (00=OR, 01=AND, 10=XOR, 11=XNOR). |
| [[PPU registers#WOBJLOG|WOBJLOG]] | $212B | .... CCOO | W8 | Window mask logic for OBJ (O) and color (C). |
| [[PPU registers#TM|TM]] | $212C | ...O 4321 | W8 | Main screen layer enable (PPU registers#). |
| [[PPU registers#TS|TS]] | $212D | ...O 4321 | W8 | Sub screen layer enable (#). |
| [[PPU registers#TMW|TMW]] | $212E | ...O 4321 | W8 | Main screen layer window enable. |
| [[PPU registers#TSW|TSW]] | $212F | ...O 4321 | W8 | Sub screen layer window enable. |
| [[PPU registers#CGWSEL|CGWSEL]] | $2130 | MMSS ..AD | W8 | main/sub screen color window black/transparent regions (MS), fixed/subscreen (A), direct color (D). |
| [[PPU registers#CGADSUB|CGADSUB]] | $2131 | MHBO 4321 | W8 | Color math add/subtract (M), half (H), backdrop (B), layer enable (O4321). |
| [[PPU registers#COLDATA|COLDATA]] | $2132 | BGRC CCCC | W8 | Fixed color channel select (BGR) and value (C). |
| [[PPU registers#SETINI|SETINI]] | $2133 | EX.. HOiI | W8 | External sync (E), EXTBG (X), Hi-res (H), Overscan (O), OBJ interlace (i), Screen interlace (I). |
| [[PPU registers#MPY|MPYL]]  [[PPU registers#MPY|MPYM]]  [[PPU registers#MPY|MPYH]] | $2134  $2135  $2136 | LLLL LLLL  MMMM MMMM  HHHH HHHH | R24 | 24-bit signed multiplication result. |
| [[PPU registers#SLHV|SLHV]] | $2137 | .... .... | R8 | Software latch for H/V counters. |
| [[PPU registers#OAMDATAREAD|OAMDATAREAD]] | $2138 | DDDD DDDD | R8 | Read OAM data byte, increments OAMADD byte. |
| [[PPU registers#VMDATAREAD|VMDATALREAD]]  [[PPU registers#VMDATAREAD|VMDATAHREAD]] | $2139  $213A | LLLL LLLL  HHHH HHHH | R16 | VRAM data read. Increments VMADD after read according to VMAIN setting. |
| [[PPU registers#CGDATAREAD|CGDATAREAD]] | $213B | .BBB BBGG GGGR RRRR | R8x2 | CGRAM data read, increments CGADD byte address after each write. |
| [[PPU registers#OPHCT|OPHCT]] | $213C | ...H HHHH HHHH HHHH | R8x2 | Output horizontal counter. |
| [[PPU registers#OPVCT|OPVCT]] | $213D | ...V VVVV VVVV VVVV | R8x2 | Output vertical counter. |
| [[PPU registers#STAT77|STAT77]] | $213E | TRM. VVVV | R8 | Sprite overflow (T), sprite tile overflow (R), master/slave (M), PPU1 version (V). |
| [[PPU registers#STAT78|STAT78]] | $213F | FL.M VVVV | R8 | Interlace field (F), counter latch value (L), NTSC/PAL (M), PPU2 version (V). |
| 5A22, WRAM, APU registers $2140-421F ([[MMIO register table/MMIO|table source]]) | | | | |
| Name | Address | Bits | Type | Notes |
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
| DMA registers $4300-437F ([[MMIO register table/DMA|table source]]) | | | | |
| Name | Address | Bits | Type | Notes |
| [[DMA registers#DMAPn|DMAPn]] | $43n0 | DI.A APPP | RW8 | Direction (D), indirect HDMA (I), address increment mode (A), transfer pattern (P). |
| [[DMA registers#BBADn|BBADn]] | $43n1 | AAAA AAAA | RW8 | B-bus address. |
| [[DMA registers#A1TnL|A1TnL]] [[DMA registers#A1TnH|A1TnH]] [[DMA registers#A1Bn|A1Bn]] | $43n2 $43n3 $43n4 | LLLL LLLL HHHH HHHH BBBB BBBB | RW24 | DMA source address / HDMA table start address. |
| [[DMA registers#DASnL|DASnL]] [[DMA registers#DASnH|DASnH]] [[DMA registers#DASBn|DASBn]] | $43n5 $43n6 $43n7 | LLLL LLLL HHHH HHHH BBBB BBBB | RW24 | DMA byte count (H:L) / HDMA indirect table address (B:H:L). |
| [[DMA registers#A2AnL|A2AnL]] [[DMA registers#A2AnH|A2AnH]] | $43n8 $43n9 | LLLL LLLL HHHH HHHH | RW16 | HDMA table current address within bank (H:L). |
| [[DMA registers#NLTRn|NLTRn]] | $43nA | RLLL LLLL | RW8 | HDMA reload flag (R) and scanline counter (L). |
| [[DMA registers#UNUSEDn|UNUSEDn]] | $43nB $43nF | DDDD DDDD | RW8 | Unused shared data byte (D). |

Register types:

- **R** - Readable
- **W** - Writeable
- **8** - 8-bit access only
- **16** - 8-bit access to either address, or 16-bit access to the lower address.
- **24** - 8-bit or 16-bit access to 3 registers.
- **8x2** - An internal 2-byte state accessed by two 8-bit read or writes (LSB first).

## See Also

- [[MMIO registers]]
- [[PPU registers]]
- [[DMA registers]]
