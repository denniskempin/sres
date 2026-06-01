---
title: "PPU registers"
source_url: "https://snes.nesdev.org/wiki/PPU_registers"
pageid: 9
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The SNES PPU is accessed through [[MMIO register table|memory-mapped registers]] at $2100-213F.

PPU register summary

| Name | Address | Bits | Type | Notes |
| --- | --- | --- | --- | --- |
| [INIDISP](#INIDISP) | $2100 | F... BBBB | W8 | Forced blanking (F), screen brightness (B). |
| [OBJSEL](#OBJSEL) | $2101 | SSSN NbBB | W8 | OBJ sprite size (S), name secondary select (N), name base address (B). |
| [OAMADDL](#OAMADD)  [OAMADDH](#OAMADD) | $2102  $2103 | AAAA AAAA  P... ...B | W16 | OAM word address (A).  Priority rotation (P), address high bit (B). |
| [OAMDATA](#OAMDATA) | $2104 | DDDD DDDD | W8x2 | OAM data write byte (2x for word) (D), increments OAMADD byte. |
| [BGMODE](#BGMODE) | $2105 | 4321 PMMM | W8 | Tilemap tile size (#), BG3 priority (P), BG mode (M). |
| [MOSAIC](#MOSAIC) | $2106 | SSSS 4321 | W8 | Mosaic size (S), mosaic BG enable (#). |
| [BG1SC](#BGnSC)  [BG2SC](#BGnSC)  [BG3SC](#BGnSC)  [BG4SC](#BGnSC) | $2107  $2108  $2109  $210A | AAAA AAYX | W8 | Tilemap VRAM address (A), vertical tilemap count (Y), horizontal tilemap count (X). |
| [BG12NBA](#BG12NBA) | $210B | BBBB AAAA | W8 | BG2 CHR base address (B), BG1 CHR base address (A). |
| [BG34NBA](#BG34NBA) | $210C | DDDD CCCC | W8 | BG4 CHR base address (D), BG3 CHR base address (C). |
| [BG1HOFS](#BGnHOFS)  [M7HOFS](#M7HOFS)  [BG1VOFS](#BGnVOFS)  [M7VOFS](#M7VOFS) | $210D    $210E | .... ..XX XXXX XXXX  ...x xxxx xxxx xxxx  .... ..YY YYYY YYYY  ...y yyyy yyyy yyyy | W8x2  W8x2  W8x2  W8x2 | BG1 horizontal scroll (X).  Mode 7 horizontal scroll (x).  BG1 vertical scroll (Y).  Mode 7 vertical scroll (y). |
| [BG2HOFS](#BGnHOFS)  [BG2VOFS](#BGnVOFS)  [BG3HOFS](#BGnHOFS)  [BG3VOFS](#BGnVOFS)  [BG4HOFS](#BGnHOFS)  [BG4VOFS](#BGnVOFS) | $210F  $2110  $2111  $2112  $2113  $2114 | .... ..XX XXXX XXXX  .... ..YY YYYY YYYY | W8x2  W8x2 | BG horizontal scroll (X).  BG vertical scroll (Y). |
| [VMAIN](#VMAIN) | $2115 | M... RRII | W8 | VRAM address increment mode (M), remapping (R), increment size (I). |
| [VMADDL](#VMADD)  [VMADDH](#VMADD) | $2116  $2117 | LLLL LLLL  hHHH HHHH | W16 | VRAM word address. |
| [VMDATAL](#VMDATA)  [VMDATAH](#VMDATA) | $2118  $2119 | LLLL LLLL  HHHH HHHH | W16 | VRAM data write. Increments VMADD after write according to VMAIN setting. |
| [M7SEL](#M7SEL) | $211A | RF.. ..YX | W8 | Mode 7 tilemap repeat (R), fill (F), flip vertical (Y), flip horizontal (X). |
| [M7A](#M7A) | $211B | DDDD DDDD dddd dddd | W8x2 | Mode 7 matrix A or signed 16-bit multiplication factor. |
| [M7B](#M7B) | $211C | DDDD DDDD dddd dddd | W8x2 | Mode 7 matrix B or signed 8-bit multiplication factor. |
| [M7C](#M7n) | $211D | DDDD DDDD dddd dddd | W8x2 | Mode 7 matrix C |
| [M7D](#M7n) | $211E | DDDD DDDD dddd dddd | W8x2 | Mode 7 matrix D |
| [M7X](#M7X) | $211F | ...X XXXX XXXX XXXX | W8x2 | Mode 7 center X |
| [M7Y](#M7Y) | $2120 | ...Y YYYY YYYY YYYY | W8x2 | Mode 7 center Y |
| [CGADD](#CGADD) | $2121 | AAAA AAAA | W8 | CGRAM word address. |
| [CGDATA](#CGDATA) | $2122 | .BBB BBGG GGGR RRRR | W8x2 | CGRAM data write, increments CGADD byte address after each write. |
| [W12SEL](#W12SEL) | $2123 | DdCc BbAa | W8 | Enable (ABCD) and Invert (abcd) windows for BG1 (AB) and BG2 (CD). |
| [W34SEL](#W34SEL) | $2124 | DdCc BbAa | W8 | Enable (EFGH) and Invert (efgh) windows for BG3 (EF) and BG2 (GH). |
| [WOBJSEL](#WOBJSEL) | $2125 | LlKk JjIi | W8 | Enable (IJKL) and Invert (ijkl) windows for OBJ (IJ) and color (KL). |
| [WH0](#WH0) | $2126 | LLLL LLLL | W8 | Window 1 left position. |
| [WH1](#WH1) | $2127 | RRRR RRRR | W8 | Window 1 right position. |
| [WH2](#WH2) | $2128 | LLLL LLLL | W8 | Window 2 left position. |
| [WH3](#WH3) | $2129 | RRRR RRRR | W8 | Window 2 right position. |
| [WBGLOG](#WBGLOG) | $212A | 4433 2211 | W8 | Window mask logic for BG layers (00=OR, 01=AND, 10=XOR, 11=XNOR). |
| [WOBJLOG](#WOBJLOG) | $212B | .... CCOO | W8 | Window mask logic for OBJ (O) and color (C). |
| [TM](#TM) | $212C | ...O 4321 | W8 | Main screen layer enable (PPU registers#). |
| [TS](#TS) | $212D | ...O 4321 | W8 | Sub screen layer enable (#). |
| [TMW](#TMW) | $212E | ...O 4321 | W8 | Main screen layer window enable. |
| [TSW](#TSW) | $212F | ...O 4321 | W8 | Sub screen layer window enable. |
| [CGWSEL](#CGWSEL) | $2130 | MMSS ..AD | W8 | main/sub screen color window black/transparent regions (MS), fixed/subscreen (A), direct color (D). |
| [CGADSUB](#CGADSUB) | $2131 | MHBO 4321 | W8 | Color math add/subtract (M), half (H), backdrop (B), layer enable (O4321). |
| [COLDATA](#COLDATA) | $2132 | BGRC CCCC | W8 | Fixed color channel select (BGR) and value (C). |
| [SETINI](#SETINI) | $2133 | EX.. HOiI | W8 | External sync (E), EXTBG (X), Hi-res (H), Overscan (O), OBJ interlace (i), Screen interlace (I). |
| [MPYL](#MPY)  [MPYM](#MPY)  [MPYH](#MPY) | $2134  $2135  $2136 | LLLL LLLL  MMMM MMMM  HHHH HHHH | R24 | 24-bit signed multiplication result. |
| [SLHV](#SLHV) | $2137 | .... .... | R8 | Software latch for H/V counters. |
| [OAMDATAREAD](#OAMDATAREAD) | $2138 | DDDD DDDD | R8 | Read OAM data byte, increments OAMADD byte. |
| [VMDATALREAD](#VMDATAREAD)  [VMDATAHREAD](#VMDATAREAD) | $2139  $213A | LLLL LLLL  HHHH HHHH | R16 | VRAM data read. Increments VMADD after read according to VMAIN setting. |
| [CGDATAREAD](#CGDATAREAD) | $213B | .BBB BBGG GGGR RRRR | R8x2 | CGRAM data read, increments CGADD byte address after each write. |
| [OPHCT](#OPHCT) | $213C | ...H HHHH HHHH HHHH | R8x2 | Output horizontal counter. |
| [OPVCT](#OPVCT) | $213D | ...V VVVV VVVV VVVV | R8x2 | Output vertical counter. |
| [STAT77](#STAT77) | $213E | TRM. VVVV | R8 | Sprite overflow (T), sprite tile overflow (R), master/slave (M), PPU1 version (V). |
| [STAT78](#STAT78) | $213F | FL.M VVVV | R8 | Interlace field (F), counter latch value (L), NTSC/PAL (M), PPU2 version (V). |
| [[MMIO register table/PPU|table source]] | | | | |

Register types:

- **R** - Readable
- **W** - Writeable
- **8** - 8-bit access only
- **16** - 8-bit access to either address, or 16-bit access to the lower address.
- **24** - 8-bit or 16-bit access to 3 registers.
- **8x2** - An internal 2-byte state accessed by two 8-bit read or writes (LSB first).

## Display configuration

### INIDISP - Screen display ($2100 write)

---

```
7  bit  0
---- ----
F... BBBB
|    ||||
|    ++++- Screen brightness (linear steps from 0 = none to $F = full)
+--------- Force blanking
```

The *screen brightness* bits control the brightness of the Blue-Green-Red DACs inside S-PPU2.

The *force blanking* bit disables screen output (S-PPU2 will output black) and allows the S-CPU to safely access the PPU registers and memory outside the blanking periods.

CAUTIONS:

- Clearing the *force blanking* bit outside the Vertical Blanking Period will output graphical glitches.
  - The current and next scanline will contain sprite glitches.
  - If *force blank* is cleared outside the Horizontal Blanking Period there will be a small background tile glitch.
- *force blanking* does not disable HDMA.

ERRATA:

- Changing the brightness is not instant. On a 3-chip SNES, it may only take a few pixels to change the brightness, but on a 1-chip SNES it may be a gradual fade that takes 72 pixels or more.
  - This can be a problem for games that extend vblank by disabling rendering and enabling it several scanlines into the frame. For this use-case, it's recommended to disable rendering by writing `$8F` (or $80 ORed with whatever the desired brightness is) to INIDISP instead of `$80`, so that the brightness is not changed as rendering is enabled.
- INIDISP early read bug: When INIDISP is written to, the PPU doesn't wait for the value to be put on the bus before attempting to read it. This means that the SNES will end up rendering about one pixel where INIDISP has been set to whatever was on the data bus before the correct value. For instructions that don't use indirect addressing, this will likely be the last byte of the instruction.
  - INIDISP writes during the Vertical Blanking Period will not encounter this glitch.
  - Workaround: Use long addressing to write to INIDISP during rendering, and take advantage of how PPU registers are available in many different banks. `STA $8F2100` will put $8f on the bus before the written value, and `STA $0F2100` will put $0f on the bus before the written value, and so on.
- On version 2 of the 5A22 chip ("S-CPU-A"), a recent HDMA transfer to/from INIDISP (Meaning that BBADn is set to zero $00) can make a DMA transfer fail. Nothing will happen and the DMA size registers (DASnL, DASnH) will be unchanged, instead of zero like they normally are after a DMA has been completed.
  - Workaround: Set BBADn to $ff instead, and set the transfer pattern to 1. This will cause HDMA to write to $21ff (nothing) and then $2100 (INIDISP). Both bytes should be set to the same value to prevent the INIDISP early read bug.
  - S-CPU (the first version), S-CPU-B and the 1-CHIP SNES are not affected by this bug.

### BGMODE - BG mode and Character size ($2105 write)

---

```
7  bit  0
---- ----
4321 PMMM
|||| ||||
|||| |+++- BG mode (see below)
|||| +---- Mode 1 BG3 priority (0 = normal, 1 = high)
|||+------ BG1 character size (0 = 8x8¹, 1 = 16x16)
||+------- BG2 character size (0 = 8x8¹, 1 = 16x16)
|+-------- BG3 character size (0 = 8x8¹, 1 = 16x16)
+--------- BG4 character size (0 = 8x8, 1 = 16x16)
```

```
                                                      BG Modes
Mode| BG bit depth  |Offsets |     Priorities (front -> back)       |                     Notes                      
    |BG1 BG2 BG3 BG4|per tile|                                      |                                                
 0  | 2   2   2   2 |   No   |   S3 1H 2H S2 1L 2L S1 3H 4H S0 3L 4L|                                                
 1  | 4   4   2     |   No   |   S3 1H 2H S2 1L 2L S1 3H    S0 3L   |BG3 priority = 0                                
    |               |        |3H S3 1H 2H S2 1L 2L S1       S0 3L   |BG3 priority = 1                                
 2  | 4   4         |  Yes   |   S3 1H    S2 2H    S1 1L    S0 2L   |                                                
 3  | 8   4         |   No   |   S3 1H    S2 2H    S1 1L    S0 2L   |                                                
 4  | 8   2         |  Yes   |   S3 1H    S2 2H    S1 1L    S0 2L   |                                                
 5  | 4   2         |   No   |   S3 1H    S2 2H    S1 1L    S0 2L   |Fixed 16 pixel char width. Forced high-res mode.
 6  | 4             |  Yes   |   S3 1H    S2       S1 1L    S0      |Fixed 16 pixel char width. Forced high-res mode.
 7  | 8             |   No   |   S3       S2       S1 1L    S0      |Fixed 8x8 char size.                            
7EXT| 8   7         |   No   |   S3       S2 2H    S1 1L    S0 2L   |Fixed 8x8 char size. BG2 bit 7 acts as priority.
```

¹: In modes 5 and 6, characters and OPT entries are always 16 pixels wide.

See: [[Backgrounds]].

### MOSAIC - Screen pixelation ($2106 write)

---

```
7  bit  0
---- ----
SSSS 4321
|||| ||||
|||| |||+- Enable BG1 mosaic
|||| ||+-- Enable BG2 mosaic
|||| |+--- Enable BG3 mosaic
|||| +---- Enable BG4 mosaic
++++------ Mosaic size in pixels (0 = 1x1, ..., 15 = 16x16)
```

### BGnSC - BG1-4 tilemap address and size ($2107-$210A write)

---

```
7  bit  0
---- ----
AAAA AAYX
|||| ||||
|||| |||+- Horizontal tilemap count (0 = 1 tilemap, 1 = 2 tilemaps)
|||| ||+-- Vertical tilemap count (0 = 1 tilemap, 1 = 2 tilemaps)
++++-++--- Tilemap VRAM address (word address = AAAAAA << 10)
```

Tilemaps may be placed at any 2 KiB (1 KiW) page.

### CHR word base address

---

The tile base address for background CHR can start at any 8 KiB (4 KiW) page.

Tilemap offsets that go past the end of VRAM are allowed to wrap around to the beginning.

#### BG12NBA - BG1 and BG2 CHR word base address ($210B write)

```
7  bit  0
---- ----
BBBB AAAA
|||| ||||
|||| ++++- BG1 CHR word base address (word address = AAAA << 12)
++++------ BG2 CHR word base address (word address = BBBB << 12)
```

#### BG34NBA - BG3 and BG4 CHR word base address ($210C write)

```
7  bit  0
---- ----
DDDD CCCC
|||| ||||
|||| ++++- BG3 CHR word base address (word address = CCCC << 12)
++++------ BG4 CHR word base address (word address = DDDD << 12)
```

### Scroll

---

Each of these scroll registers is normally updated by two single-byte writes to the same address. After two consecutive writes the scroll value is fully updated.

The two-write mechanism internally keeps shared latch values, so these registers should not normally be written in mixed order. Complete both writes to one register before moving on to the next.

The scroll offset is always relative to the top-left of the screen, even when updating mid-frame with HDMA.

Because the first line of rendering is always a blank line, with vertical scroll of 0 the top line of the BG will be hidden. In the default [224-lines mode](#SETINI) an extra (224th) line of BG is also visible at the bottom to compensate.

#### BGnHOFS - BG1-4 horizontal scroll offset ($210D, $210F, $2111, $2113 write twice)

```
15  bit  8   7  bit  0
 ---- ----   ---- ----
 .... ..XX   XXXX XXXX
        ||   |||| ||||
        ++---++++-++++- BGn horizontal scroll

On write: BGnHOFS = (value << 8) | (bgofs_latch & ~7) | (bghofs_latch & 7)
          bgofs_latch = value
          bghofs_latch = value

Note: BG1HOFS uses the same address as M7HOFS
```

#### BGnVOFS - BG1-4 vertical scroll offset ($210E, $2110, $2112, $2114 write twice)

```
15  bit  8   7  bit  0
 ---- ----   ---- ----
 .... ..YY   YYYY YYYY
        ||   |||| ||||
        ++---++++-++++- BGn vertical scroll

On write: BGnVOFS = (value << 8) | bgofs_latch
          bgofs_latch = value

Note: BG1VOFS uses the same address as M7VOFS
```

### Layer enable

---

#### TM - Main screen layer enable ($212C write)

```
7  bit  0
---- ----
...O 4321
   | ||||
   | |||+- Enable BG1 on main screen
   | ||+-- Enable BG2 on main screen
   | |+--- Enable BG3 on main screen
   | +---- Enable BG4 on main screen
   +------ Enable OBJ on main screen
```

#### TS - Subscreen layer enable ($212D write)

```
7  bit  0
---- ----
...O 4321
   | ||||
   | |||+- Enable BG1 on subscreen
   | ||+-- Enable BG2 on subscreen
   | |+--- Enable BG3 on subscreen
   | +---- Enable BG4 on subscreen
   +------ Enable OBJ on subscreen
```

### SETINI - Screen Mode/Video Select ($2133 write)

---

```
7  bit  0
---- ----
EX.. HOiI
||   ||||
||   |||+- Screen interlacing
||   ||+-- OBJ interlacing
||   |+--- Overscan mode
||   +---- High-res mode
|+-------- EXTBG mode
+--------- External sync
```

- **Screen interlacing** causes every odd frame to lower its picture scanlines half a line between the even frames. When enabled, this produces a 480i picture composed of 2 frames (fields), instead of the default 240p progressive picture where each frame appears at the same vertical level.
  - [**STAT78**](#STAT78_-_PPU2_status_flags_and_version_($213F_read)) ($213F) can be used to check whether the current frame is an even or odd field.
  - When interlacing is enabled for BG mode 5 or 6, the BG layers are automatically interlaced to give a view of the background that has double the vertical resolution in 480i, effectively making every BG pixel half as tall.
    - The [BGMODE](#BGMODE) character size bits still choose between 16x8 and 16x16px tiles even when interlacing is true.
- **OBJ interlacing** interlaces the sprites to double their vertical resolution in 480i. Sprite pixels will appear half as tall.
- **Overscan mode** enables the full 239 line picture when set, instead of only 224. On NTSC televisions this extra area is not normally visible, but on PAL it is very visible. Setting this causes NMI/vblank to begin 8 lines later, and end 8 lines earlier, dramatically reducing the vblank length in NTSC. Sprite and scroll positions are relative to the end of the blanking period, so enabling this automatically shifts everything up 8 lines. Using this feature makes the SNES drawing positions similar to the NES.
- **High-res mode** doubles the horizontal output resolution from 256 to 512 pixels.
  - In most BG modes this causes the sub screen to render pixels on even columns (assuming zero-based column indices), and the main screen to render on odd columns. This is sometimes called "pseudo-hires". Some games use this for a transparency effect (*Kirby's Dreamland 3*, *Jurassic Park*), relying on blurring from the composite video signal to blend the columns.
  - In BG modes 5 and 6, this high-res is forced, but the BG layers are automatically interleaved to double their horizontal resolution, making every BG pixel half as wide.
- **EXTBG** controls a second-layer effect in BG [[Mode 7]] only. In other modes, enabling EXTBG will display garbage.
- **External sync** is used for super-imposing images from an external device. Normally 0.

## VRAM

### VMAIN - Video Port Control ($2115 write)

---

```
7  bit  0
---- ----
M... RRII
|    ||||
|    ||++- Address increment amount:
|    ||     0: Increment by 1 word
|    ||     1: Increment by 32 words
|    ||     2: Increment by 128 words
|    ||     3: Increment by 128 words
|    ++--- Address remapping: (VMADD -> Internal)
|           0: None
|           1: Remap rrrrrrrr YYYccccc -> rrrrrrrr cccccYYY (2bpp)
|           2: Remap rrrrrrrY YYcccccP -> rrrrrrrc ccccPYYY (4bpp)
|           3: Remap rrrrrrYY YcccccPP -> rrrrrrcc cccPPYYY (8bpp)
+--------- Address increment mode:
            0: Increment after writing $2118 or reading $2139
            1: Increment after writing $2119 or reading $213A
```

- **Address remapping** allows redirection of the write address to update 32-tile rows horizontally when using `II` = 0. Within a 32-tile group, sequential access iterates through the same 8-pixel row of each tile horizontally. After 32 spans, it will reach the second row of the first tile. Finally after a group of 32 tiles has been updated, it advances to the next group of 32 tiles..
  - This is suitable for a 32x32 tilemap in 8x8 tile mode. By filling each row of the tilemap with sequential values, each group of 32 tiles now corresponds to a contiguous horizontal span of pixels.
  - P = tile bitplane-word, c = group column, Y = tile pixel row, r = group row.
  - When setting the starting address, the starting tile of a 32-tile group will always be the at the same position as its remapped address.
  - With 4bpp or 8bpp modes, each increment advances through the 2 or 4 plane-words of a single tile before advancing to the next tile.
  - Simplified explanation:
    - 1. Write all planes for an 8 pixel span before proceeding horizontally to the next.
    - 2. After completing a row of 256 pixels (32 spans), proceed vertically to the next.

### VRAM address

---

#### VMADDL, VMADDH - VRAM word address ($2116, $2117 write)

```
 VMADDH      VMADDL
  $2117       $2116
7  bit  0   7  bit  0
---- ----   ---- ----
hHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||
++++-++++---++++-++++- VRAM word address

On write: Update VMADD
          vram_latch = [VMADD]
```

Because the SNES only has 64 KiB of VRAM, VRAM address bit 15 has no effect.

The VRAM can only be read during vertical-blank or force-blank. If the PPU is in horizontal-blank or active-display then the VRAM will not be read and vram\_latch will contain invalid data.

### VRAM data

---

#### VMDATAL, VMDATAH - VRAM data write ($2118, $2119 write)

```
 VMDATAH     VMDATAL
  $2119       $2118
7  bit  0   7  bit  0
---- ----   ---- ----
HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||
++++-++++---++++-++++- VRAM data word

On $2118 write: If address increment mode == 0: increment VMADD
On $2119 write: If address increment mode == 1: increment VMADD
```

The VRAM can only be written to in vertical-blank or force-blank. Any VRAM writes during horizontal-blank or active-display will be ignored.

[VMADD](#VMADD) will always increment, depending on the state of [VMAIN](#VMAIN), even if the VRAM write is ignored.

#### VMDATALREAD, VMDATAHREAD - VRAM data read ($2139, $213A read)

```
VMDATAHREAD VMDATALREAD
   $213A       $2139
 7  bit  0   7  bit  0
 ---- ----   ---- ----
 HHHH HHHH   LLLL LLLL
 |||| ||||   |||| ||||
 ++++-++++---++++-++++- VRAM data word from vram_latch

On $2139 read: value = vram_latch.low
               If address increment mode == 0:
                 vram_latch = [VMADD]
                 Increment VMADD
On $213A read: value = vram_latch.high
               If address increment mode == 1:
                 vram_latch = [VMADD]
                 Increment VMADD
```

When reading multiple bytes/words with increment, we normally have to do 1 extra read at the start to account for the vram\_latch behaviour.

The vram\_latch is loaded immediately after you set an address with [VMADD](#VMADD), and the word value at that address will be available for the next reads from VMDATAxREAD.

When incrementing due to VMDATAxREAD, the next word value is loaded into vram\_latch *before* the increment. This means that the first 2 reads after setting VMADD will *both* return the same word stored at that address, before the increment takes effect and allows you to read the subsequent bytes/words.

So:

- When reading a single byte/word of data: simply set the address with VMADD, and then read the data via VMDATAxREAD.
- When reading a block of contiguous data: after writing VMADD do one dummy read to VMDATAxREAD to pre-load the vram\_latch. After this you can simply reach each byte/word sequentially with auto-increment.

The VRAM can only be read during vertical-blank or force-blank. If the PPU is in horizontal-blank or active-display then the VRAM will not be read and vram\_latch will contain invalid data.

[VMADD](#VMADD) will always increment, depending on the state of [VMAIN](#VMAIN), even if the VRAM is not read.

## CGRAM

### CGADD - CGRAM word address ($2121 write)

---

```
7  bit  0
---- ----
AAAA AAAA
|||| ||||
++++-++++- CGRAM word address

On write: cgram_byte = 0
```

### CGRAM data

---

#### CGDATA - CGRAM data write ($2122 write twice)

```
15  bit  8   7  bit  0
 ---- ----   ---- ----
 .BBB BBGG   GGGR RRRR
  ||| ||||   |||| ||||
  ||| ||||   |||+-++++- Red component 
  ||| ||++---+++------- Green component
  +++-++--------------- Blue component

On write: If cgram_byte == 0: cgram_latch = value
          If cgram_byte == 1: CGDATA = (value << 8) | cgram_latch
          cgram_byte = ~cgram_byte
```

Two single-byte writes to this register will update a single CGRAM word. The effect is applied only once the second byte is written.

Each write will increment the internal byte address. After two writes it will automatically have incremented to the next word.

The S-CPU can only access CGRAM during [[Timing#Vertical Blank|Vertical Blank]], [[Timing#Horizontal Blank|Horizontal Blank]] or [Force Blank](#INIDISP). Writing to CGRAM during active-display will write the data to the wrong CGRAM address.

#### CGDATAREAD - CGRAM data read ($213B read twice)

```
15  bit  8   7  bit  0
 ---- ----   ---- ----
 xBBB BBGG   GGGR RRRR
 |||| ||||   |||| ||||
 |||| ||||   |||+-++++- Red component 
 |||| ||++---+++------- Green component
 |+++-++--------------- Blue component
 +--------------------- PPU2 open bus

On read: If cgram_byte == 0: value = CGDATA.low
         If cgram_byte == 1: value = CGDATA.high
         cgram_byte = ~cgram_byte
```

The S-CPU can only access CGRAM during [[Timing#Vertical Blank|Vertical Blank]], [[Timing#Horizontal Blank|Horizontal Blank]] or [Force Blank](#INIDISP). Reading CGRAM during active-display will read from the wrong CGRAM address.

## OAM

### OBJSEL - Object size and Character address ($2101 write)

---

```
7  bit  0
---- ----
SSSN NbBB
|||| ||||
|||| |+++- Name base address (word address = bBB << 13)
|||+-+---- Name select (word offset = (NN+1) << 12)
+++------- Object size:
            0:  8x8  and 16x16
            1:  8x8  and 32x32
            2:  8x8  and 64x64
            3: 16x16 and 32x32
            4: 16x16 and 64x64
            5: 32x32 and 64x64
            6: 16x32 and 32x64
            7: 16x32 and 32x32
```

- **Name base address** selects a 16 KiB-aligned quarter of VRAM for the first 8 KiB of available sprite tiles. Bit 2 was reserved for a planned but never implemented expansion to 128 KiB VRAM, so is normally 0.
- **Name select** controls a relative offset from the name base address in NN+1 8 KiB increments, selecting a second 8 KiB of available sprite tiles. With name select of 0, the second half follows the base 8 KiB contiguously.
- **Object size** controls the sizes available for sprites. The two modes featuring rectangular sizes (6, 7) were not documented by the SNES development manual.

Fullsnes refers to this register as **OBSEL**.

### OAM address

---

#### OAMADDL, OAMADDH - OAM word address ($2102, $2103 write)

```
 OAMADDH     OAMADDL
  $2103       $2102
7  bit  0   7  bit  0
---- ----   ---- ----
P... ...B   AAAA AAAA
|       |   |||| ||||
|       |   ++++-++++- OAM word address
|       |   ++++-+++0- OAM priority rotation index
|       +------------- OAM table select (0 = 256 word table, 1 = 16 word table)
+--------------------- OAM priority rotation (1 = enable)

On write: Update OAMADD
          internal_oamadd = (OAMADD & $1FF) << 1
```

- **Priority rotation** causes the highest priority sprite to be at the last OAMADD set before the visible picture (bits 1-7 only). Otherwise OAM 0 is the highest priority sprite. This can be used for a simple sprite priority rotation.

### OAM data

---

#### OAMDATA - OAM data write ($2104 write)

```
7  bit  0
---- ----
DDDD DDDD
|||| ||||
++++-++++- OAM data

On write: If (internal_oamadd & 1) == 0: oam_latch = value
          If internal_oamadd < $200 and (internal_oamadd & 1) == 1:
            [internal_oamadd-1] = oam_latch
            [internal_oamadd] = value
          If internal_oamadd >= $200: [internal_oamadd] = value
          internal_oamadd = internal_oamadd + 1
```

When the OAM byte address is less than 512:

:   Two single-byte writes to this register will update a single OAM word. The effect is applied only once the second byte is written.

When the OAM byte address is 512 or above:

:   Each write immediately applies to the current byte.

Each write will increment the internal byte address.

The S-CPU can only access OAM during [[Timing#Vertical Blank|Vertical Blank]] or [Force Blank](#INIDISP).

#### OAMDATAREAD - OAM data read ($2138 read)

```
7  bit  0
---- ----
DDDD DDDD
|||| ||||
++++-++++- OAM data

On read: value = [internal_oamadd]
         internal_oamadd = internal_oamadd + 1
```

The S-CPU can only access OAM during [[Timing#Vertical Blank|Vertical Blank]] or [Force Blank](#INIDISP).

## Mode 7

### M7SEL - Mode 7 settings ($211A write)

---

```
7  bit  0
---- ----
RF.. ..YX
||     ||
||     |+- Flip screen horizontally (backgrounds only)
||     +-- Flip screen vertically (backgrounds only)
|+-------- Non-tilemap fill (0 = transparent, 1 = character 0)
+--------- Tilemap repeat (0 = tilemap repeats, 1 = Non-tilemap fill beyond tilemap boundaries)
```

### Scroll

---

#### M7HOFS - Mode 7 horizontal scroll offset ($210D write twice)

```
15  bit  8   7  bit  0
 ---- ----   ---- ----
 ...X XXXX   XXXX XXXX
    | ||||   |||| ||||
    +-++++---++++-++++- Mode 7 horizontal scroll (signed)

On write: M7HOFS = (value << 8) | mode7_latch
          mode7_latch = value

Note: This register uses the same address as BG1HOFS
```

#### M7VOFS - Mode 7 vertical scroll offset ($210E write twice)

```
15  bit  8   7  bit  0
 ---- ----   ---- ----
 ...Y YYYY   YYYY YYYY
    | ||||   |||| ||||
    +-++++---++++-++++- Mode 7 vertical scroll (signed)

On write: M7VOFS = (value << 8) | mode7_latch
          mode7_latch = value

Note: This register uses the same address as BG1VOFS
```

### Matrices

---

#### M7A - Mode 7 matrix A and Multiplication factor 1 ($211B write twice)

```
15  bit  8   7  bit  0
 ---- ----   ---- ----
 DDDD DDDD   dddd dddd
 |||| ||||   |||| ||||
 ++++-++++---++++-++++- Mode 7 matrix A (8.8 fixed point)
 ++++-++++---++++-++++- 16-bit multiplication factor (signed)

On write: M7A = (value << 8) | mode7_latch
          mode7_latch = value
```

The last 16-bit value (signed) written here is also used to provide a 24-bit multiplication result at [MPY](#MPY).

#### M7B - Mode 7 matrix B and Multiplication factor 2 ($211C write twice)

```
15  bit  8   7  bit  0
 ---- ----   ---- ----
 DDDD DDDD   dddd dddd
 |||| ||||   |||| ||||
 ++++-++++---++++-++++- Mode 7 matrix B (8.8 fixed point)
             ++++-++++- 8-bit multiplication factor (signed)

On write: M7B = (value << 8) | mode7_latch
          mode7_latch = value
```

The last 8-bit value (signed) written here is also used to provide a 24-bit multiplication result at [MPY](#MPY).

#### M7n - Mode 7 matrix C-D ($211D-211E write twice)

```
15  bit  8   7  bit  0
 ---- ----   ---- ----
 DDDD DDDD   dddd dddd
 |||| ||||   |||| ||||
 ++++-++++---++++-++++- Mode 7 matrix n (8.8 fixed point)

On write: M7n = (value << 8) | mode7_latch
          mode7_latch = value
```

### Center

---

#### M7X - Mode 7 center X ($211F write twice)

```
15  bit  8   7  bit  0
 ---- ----   ---- ----
 ...X XXXX   XXXX XXXX
    | ||||   |||| ||||
    +-++++---++++-++++- Mode 7 center X (signed)

On write: M7X = (value << 8) | mode7_latch
          mode7_latch = value
```

#### M7Y - Mode 7 center Y ($2120 write twice)

```
15  bit  8   7  bit  0
 ---- ----   ---- ----
 ...Y YYYY   YYYY YYYY
    | ||||   |||| ||||
    +-++++---++++-++++- Mode 7 center Y (signed)

On write: M7Y = (value << 8) | mode7_latch
          mode7_latch = value
```

## Windows

**See: [[Windows]]**

### Window mask settings

---

#### W12SEL - Window Mask Settings for BG1 and BG2 ($2123 write)

```
7  bit  0
---- ----
DdCc BbAa
|||| ||||
|||| |||+- Invert window 1 for BG1
|||| ||+-- Enable window 1 for BG1
|||| |+--- Invert window 2 for BG1
|||| +---- Enable window 2 for BG1
|||+------ Invert window 1 for BG2
||+------- Enable window 1 for BG2
|+-------- Invert window 2 for BG2
+--------- Enable window 2 for BG2
```

#### W34SEL - Window Mask Settings for BG3 and BG4 ($2124 write)

```
7  bit  0
---- ----
HhGg FfEe
|||| ||||
|||| |||+- Invert window 1 for BG3
|||| ||+-- Enable window 1 for BG3
|||| |+--- Invert window 2 for BG3
|||| +---- Enable window 2 for BG3
|||+------ Invert window 1 for BG4
||+------- Enable window 1 for BG4
|+-------- Invert window 2 for BG4
+--------- Enable window 2 for BG4
```

#### WOBJSEL - Window Mask Settings for OBJ and Color Window ($2125 write)

```
7  bit  0
---- ----
LlKk JjIi
|||| ||||
|||| |||+- Invert window 1 for OBJ
|||| ||+-- Enable window 1 for OBJ
|||| |+--- Invert window 2 for OBJ
|||| +---- Enable window 2 for OBJ
|||+------ Invert window 1 for color
||+------- Enable window 1 for color
|+-------- Invert window 2 for color
+--------- Enable window 2 for color
```

The color window is used to black areas of the main or sub screen, see: [CGWSEL](#CGWSEL).

### Window positions

---

#### WH0 - Window 1 left position ($2126 write)

```
7  bit  0
---- ----
LLLL LLLL
|||| ||||
++++-++++- Window 1 left edge position
```

#### WH1 - Window 1 right position ($2127 write)

```
7  bit  0
---- ----
RRRR RRRR
|||| ||||
++++-++++- Window 1 right edge position
```

#### WH2 - Window 2 left position ($2128 write)

```
7  bit  0
---- ----
LLLL LLLL
|||| ||||
++++-++++- Window 2 left edge position
```

#### WH3 - Window 2 right position ($2129 write)

```
7  bit  0
---- ----
RRRR RRRR
|||| ||||
++++-++++- Window 2 right edge position
```

### Window mask logic

---

#### WBGLOG - Window BG mask logic ($212A write)

```
7  bit  0
---- ----
4433 2211
|||| ||||
|||| ||++- BG1 window mask logic
|||| ++--- BG2 window mask logic
||++------ BG3 window mask logic
++-------- BG4 window mask logic
```

#### WOBJLOG - Window OBJ and color math mask logic ($212B write)

```
7  bit  0
---- ----
.... CCOO
     ||||
     ||++- OBJ window mask logic
     ++--- Color window mask logic
```

```
Mask logic types
Value|Logic
   0 | OR
   1 | AND
   2 | XOR
   3 | XNOR
```

The color window is used to mask regions of the main and sub-screens, see: [CGWSEL](#CGWSEL).

### Window enable

---

#### TMW - Main screen layer window enable ($212E write)

```
7  bit  0
---- ----
...O 4321
   | ||||
   | |||+- Apply enabled windows to main screen BG1
   | ||+-- Apply enabled windows to main screen BG2
   | |+--- Apply enabled windows to main screen BG3
   | +---- Apply enabled windows to main screen BG4
   +------ Apply enabled windows to main screen OBJ
```

#### TSW - Subscreen layer window enable ($212F write)

```
7  bit  0
---- ----
...O 4321
   | ||||
   | |||+- Apply enabled windows to subscreen BG1
   | ||+-- Apply enabled windows to subscreen BG2
   | |+--- Apply enabled windows to subscreen BG3
   | +---- Apply enabled windows to subscreen BG4
   +------ Apply enabled windows to subscreen OBJ
```

## Color math

### CGWSEL - Color addition select ($2130 write)

---

```
7  bit  0
---- ----
MMSS ..AD
||||   ||
||||   |+- Direct color mode
||||   +-- Addend (0 = fixed color, 1 = subscreen)
||++------ Sub screen color window transparent region
++-------- Main screen color window black region
```

```
Region types
Value|Region
   0 |Nowhere
   1 |Outside color window
   2 |Inside color window
   3 |Everywhere
```

- The window region settings will replace the main-screen color with black, or sub-screen with transparent, on pixels according to the color windows ([WOBJSEL](#WOBJSEL) high nibble). If the color windows are not enabled by WOBJSEL, everything is "outside" them. The main-screen setting is used to force a region of the main screen to black. The sub-screen setting is for masking [[Color math]].
- **Addend** selects either the fixed color ([COLDATA](#COLDATA)) or sub-screen for color math. Both can be masked by the window region.
- **Direct color mode** is not directly related to color math, but for 8-bpp background modes it selects between palettes and [[Direct color]].
- Some older emulators have known inaccurate implementations of the MM bits:
  - Snes9x 1.43 ignores color math for the entire line if either bit is 1.
  - ZSNES ignores color math for any pixels where the main screen was replaced with black. This means that the final result for those pixels is always black.

### CGADSUB - Color math designation ($2131 write)

---

```
7  bit  0
---- ----
MHBO 4321
|||| ||||
|||| |||+- BG1 color math enable
|||| ||+-- BG2 color math enable
|||| |+--- BG3 color math enable
|||| +---- BG4 color math enable
|||+------ OBJ color math enable (palettes 4-7 only)
||+------- Backdrop color math enable
|+-------- Half color math
+--------- Operator type (0 = add, 1 = subtract)
```

This designates which elements of the main screen will have color math applied to them. After layering, if the visible pixel belongs to a color-math enabled layer, the chosen operation will be applied with the subscreen (or fixed color).

### COLDATA - Fixed color data ($2132 write)

---

```
7  bit  0
---- ----
BGRC CCCC
|||| ||||
|||+-++++- Color value
||+------- Write color value to red channel
|+-------- Write color value to green channel
+--------- Write color value to blue channel
```

COLDATA requires one, two or three writes to set the fixed color to a target color value. For example:

- Black - 1 write: %111\_00000 *(bgr=0)*
- White - 1 write: %111\_11111 *(bgr=31)*
- Dark Blue - 2 writes: %100\_10010 *(b=18)*, %011\_00000 *(gr=0)*
- Light Green - 2 writes: %101\_10010 *(br=20)*, %010\_11111 *(g=31)*
- Light Blue - 3 writes: %100\_11110 *(b=30)*, %010\_11011 *(g=27)*, %001\_10110 *(r=22)*
- Gold - 3 writes: %100\_00000 *(b=0)*, %010\_11011 *(g=27)*, %001\_11111 *(r=31)*

## Multiplication result

### MPYL, MPYM, MPYH - Multiplication result ($2134, $2135, $2136 read)

```
  MPYH        MPYM        MPYL
  $2136       $2135       $2134
7  bit  0   7  bit  0   7  bit  0
---- ----   ---- ----   ---- ----
HHHH HHHH   MMMM MMMM   LLLL LLLL
|||| ||||   |||| ||||   |||| ||||
++++-++++---++++-++++---++++-++++- 24-bit multiplication result (signed)
```

The MPY register contains the signed 24-bit result of the signed 16-bit [M7A](#M7A) multiplied by the signed 8-bit value last written to [M7B](#M7B).

This result may be read back after writing M7A with a signed 16-bit value (write twice), and M7B with a signed 8-bit value (write once).

There is no delay, the result is available immediately.

See: [Multiplication](https://snes.nesdev.org/wiki/Multiplication#"PPU"_multiplier "Multiplication")

CAUTION:  
The [M7A](#M7A) register shares mode7\_latch with [M7HOFS](#M7HOFS) and [M7VOFS](#M7VOFS) and the Mode 7 offset registers use the same address as the BG1 offset registers.

If an interrupt or HDMA transfer writes to a BG1 offset register or Mode7 Matrix in-between the two M7A writes the internal M7A value will be corrupted and MPY outputs the wrong value.

## H/V counters

### SLHV - Software latch for H/V counters ($2137 read)

---

```
7  bit  0
---- ----
xxxx xxxx
|||| ||||
++++-++++- CPU Open bus

On read: counter_latch = 1
```

### Counters

---

#### OPHCT - Output horizontal counter ($213C read twice)

```
15  bit  8   7  bit  0
 ---- ----   ---- ----
 xxxx xxxH   HHHH HHHH
 |||| ||||   |||| ||||
 |||| |||+---++++-++++- Horizontal counter value
 ++++-+++-------------- PPU2 open bus

On read: If ophct_byte == 0: value = OPHCT.low
         If ophct_byte == 1: value = OPHCT.high
         ophct_byte = ~ophct_byte
```

#### OPVCT - Output vertical counter ($213D read twice)

```
15  bit  8   7  bit  0
 ---- ----   ---- ----
 xxxx xxxV   VVVV VVVV
 |||| ||||   |||| ||||
 |||| |||+---++++-++++- Vertical counter value
 ++++-+++-------------- PPU2 open bus

On read: If opvct_byte == 0: value = OPVCT.low
         If opvct_byte == 1: value = OPVCT.high
         opvct_byte = ~opvct_byte
```

When counter\_latch transitions from 0 to 1, these registers are latched with the current counter values. counter\_latch is set when SLHV is read or /EXTLATCH (PPU2 pin 29) is asserted, and is cleared when STAT78 is read. /EXTLATCH is connected to joypad IO D7 and can be controlled by the CPU via WRIO or by a joypad.

counter\_latch behavior has not been fully confirmed.

## Status

### STAT77 - PPU1 status flags and version ($213E read)

---

```
7  bit  0
---- ----
TRMx VVVV
|||| ||||
|||| ++++- PPU1 version
|||+------ PPU1 open bus
||+------- Master/slave mode (PPU1 pin 25)
|+-------- Range over flag (sprite tile overflow)
+--------- Time over flag (sprite overflow)
```

### STAT78 - PPU2 status flags and version ($213F read)

---

```
7  bit  0
---- ----
FLxM VVVV
|||| ||||
|||| ++++- PPU2 version
|||+------ 0: 262 or 525i lines = 60Hz, 1: 312 or 625i lines = 50Hz (PPU2 pin 30)
||+------- PPU2 open bus
|+-------- Counter latch value
+--------- Interlace field

On read: counter_latch = 0
         ophct_byte = 0
         opvct_byte = 0
```

If a condition that sets counter\_latch is active when STAT78 is read, it is not known if counter\_latch is cleared. Existing documentation suggests it is not cleared and the counters are not relatched.

ERRATA:

- Nintendo stopped incrementing the version field after 3. S-PPU2 B, S-PPU2 C and S-CPUN A all report PPU2 version 3.
