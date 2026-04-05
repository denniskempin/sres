---
title: "MMIO register table/PPU"
source_url: "https://snes.nesdev.org/wiki/MMIO_register_table/PPU"
pageid: 43
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

See: [[MMIO register table]]

| Name | Address | Bits | Type | Notes |
| --- | --- | --- | --- | --- |
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
