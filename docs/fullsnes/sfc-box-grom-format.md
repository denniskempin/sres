# SFC-Box GROM Format

All SFC-Box Cartridges are containing a 32Kbyte "GROM" EPROM, the chip contains info about the ROMs in the cartridge (title, instructions, etc).

#### GROM Overall Memory Map

```text
  0000h - Root Header and HD64180 Code
  1000h - ROM File Info Block(s)  ;usually at 1000h,2000h,3000h,etc.
  7FFCh - Checksum (above bytes at 0000h..7FFBh added together)
  7FFEh - Complement (same as above Checksum, XORed by FFFFh)
```

The various unused locations are usually FFh-filled. Checksum/Complement are located at the end of the EPROM (7FFCh in case of the usual 32Kbyte EPROMs).

#### GROM - Root Header (located at 0000h)

```text
  0000h 1   Number of ROMs (01h..08h) (usually 02h or 04h) (NumROMs)
  0001h 1   GROM size (1 SHL N) kbytes (usually 05h=32Kbytes) (FFh=None)
  0002h 1   Unknown (00h or 01h or 09h)
  0003h 1   Unknown (00h)
  0004h 1   Chipset (07h or 00h) (Bit0:SRAM, Bit1:DSP, Bit2:GSU?)
  0005h 1   Unknown (01h or 00h)   Menu Flag?
  0006h 2   Offset to HD64180 code  (usually 0020h or 0030h)
  0008h 2   Offset to ROM Directory (usually 0010h)
  000Ah 2   Unknown (0000h)
  000Ch 1   Unknown (78h) (aka 120 decimal)
  000Dh 1   Unknown (B0h or 00h) (theoretically unused... but isn't FFh)
  000Eh 2   Unknown (FFFFh) (probably unused)
```

ROM Directory (usually located at 0010h):

```text
  NumROM words  Offset to ROM info, div 1000h (usually 0001h..NumROMs)
  NumROM bytes  Physical Socket on PCB (usually 00h..03h or 01h..02h)
```

The above byte-values (usually located at 0010h+NumROMs*2) can have following known values:

```text
  00h ROM5            (upper-right) (AttractionMenu)
  01h ROM1/ROM7/ROM12 (lower-right or upper-left) (MarioKar,Mahjong,Donkey)
  02h ROM3/ROM9       (middle-right) (MarioCol,Waiarae,Tetris)
  03h IC20            (special GSU ROM location) (StarFox)
```

HD64180 code (usually at 0020h or 0030h): Around 256-bytes of HD64180 code, called with following parameters:

```text
  A=function (00h=Change Mapping, FFh=Boot Callback, other=Reserved)
  BC=ptr to 10-bytes
  E=same as [BC+5]
  [BC+0]  ROM Slot (0 or 1)                                ;Cartridge Slot 0-1
  [BC+1]  ROM Socket (0..3)                                ;from GROM[8]
  [BC+2]  Mapmode (0=LoROM, 1=HiROM, 2=GSU)                ;from GROM[P0+16h]
  [BC+3]  Used Chipset (bit0=SRAM, bit1=DSP)               ;from GROM[P0+2Ah]
  [BC+4]  SRAM Size (0=None, 1=2K, 3=8K, 5=32K)            ;from GROM[P0+17h]
  [BC+5]  SRAM Base (0..3) (0..7 when 2 chips)             ;from GROM[P0+1Ch]
  [BC+6]  Slot 1 Chipset (bit0=SRAM, bit1=DSP, bit2=GSU?)  ;from Slot1.GROM[4]
  [BC+7]  Slot 0 Chipset (bit0=SRAM, bit1=DSP, bit2=GSU?)  ;from Slot0.GROM[4]
  [BC+8]  Copy of Port[C0h]  ;\the function must update these values alongside
  [BC+9]  Copy of Port[C1h]  ;/with the new values written to Port C0h/C1h
```

Note: During execution, the 1st 16K of GROM are mapped to 8000h..BFFFh.

GROM - ROM File n Info (located at 1000h,2000h,3000h,etc.)

```text
  x000h 2  P0 Offset to ASCII Title and Configuration (usually 000Eh)
  x002h 2  P1 Offset to Bitmap-Title-Tiles  ;\bitmap 128x24 pix (16x3 tiles)
  x004h 2  P2 Offset to Bitmap-Padding-Tile ; padded to 160x24 pix (20x3 tiles)
  x006h 2  P3 Offset to Bitmap-Palette      ;/with 16-color (4bpp) palette
  x008h 2  P4 Offset to Shift-JIS Instruction Pages
  x00Ah 2  P5 Offset to Demo-Joypad-Data (for demo/preview feature)
  x00Ch 2  P6 Offset to Unused-Joypad-Data  ;<-- not included in Attraction ROM
```

ASCII Title and Configuration Field (at P0):

```text
  00h 22 ASCII Title (uppercase ASCII, 22 bytes, padded with spaces)
  16h 1  ROM/SRAM mapping/speed? (00h=SlowLoROM, 01h=FastHiROM, 02h=GSU/NoSRAM)
  17h 1  SRAM Size (1 SHL N Kbytes) (but for Menu: ATROM Header claims NoSRAM?)
  18h 1  Coprocessor is DSP1 (00h=No, 01h=Yes)
  19h 1  ROM Size (in 1MBit units, aka in 128Kbyte Units)
  1Ah 1   Unknown (01h or 02h or 03h)
  1Bh 1  Demo/Preview enable (00h=Off, 01h=On)
  1Ch 1  SRAM Base (0..3) (or 0..7 when SRAMs in BOTH slots) (CHANGED by KROM1)
  1Dh 1  Preferred Title (00h=SNES[FFC0h]/Destroys SNES stack, 01h=GROM[P0])
  1Eh 1   Unknown ("strange values") (can be edited in menu point "2-4-1:3")
  1Fh 1  Always Zero (00h) (seems to be MSB of above entry) (always zero)
  20h 4  Whatever (01h,00h,00h,01h=Menu or 00h,30h,30h,05h=Game)
  24h 1   Unknown (00h or 01h or 02h) (maybe... num players/joypads?)
  25h 1   Unknown (00h or 05h or 1Eh) (aka decimal 0,5,30)
  26h 1  Game Flag (00h=Menu, 01h=Game)
  27h 1   Unknown (00h or 05h or 1Eh) (aka decimal 0,5,30)
  28h 1   Unknown (00h or 80h or 90h or A0h or D0h)
  29h 1   Unknown (00h or 21h or 22h or 23h)
  2Ah 1  Chipset (bit0=Uses SRAM, bit1=Uses DSP)     ;<-- missing in Star Fox
  2Bh 22 Unknown (all 2Eh-filled)       ;<-- located at index 2Ah in Star Fox
```

Bitmap-Title-Tiles (at P1):

```text
  1 byte - Unknown (Should be 80h) (probably bit7=compression flag?)
  2 bytes - Number of following bytes (N) (varies 02D5h..0573h) (max=600h)
  N bytes - Compressed Title Bitmap (128x24 pix, 4bpp) (16x3 Tiles)
  (the uncompressed bitmap consists of 3 rows of 16 bit-planed 4bpp SNES tiles)
  (see below for the compression format)
```

Bitmap-Padding-Tile (at P2):

```text
  32 bytes - Uncompressed Padding Tile (8x8 pix, 4bpp)
  (used to pad the 128x24 pix title bitmap, centered within a 160x24 pix area)
  (should be usually uni-colored tile, with same color as bitmap's background)
  (or, alternately, one could probably also use a "hatched" background pattern)
```

Bitmap-Palette (at P3):

```text
  32 bytes - 16-color Palette for Title Bitmap (words in range 0000h..7FFFh)
  (color 0 is unused/transparent, usually contains 0038h as dummy value)
```

Shift-JIS Instruction Pages (at P4):

```text
  1 byte - Number of Pages
  N bytes - Page(s) ;max 1372 bytes per page  ;21 lines = max 6+(32*2+1)*21+1
  Each page starts with a 6-byte header (usually 8,2,4,1,4,4), followed
  by Text (mixed 7bit ASCII, 8bit JIS, and 2x8bit Shift-JIS), lines are
  terminated by chr(09h), each page terminated by chr(00h).
```

#### Demo-Joypad-Data (at P5):  ;<-- if none: eight 00h-bytes

```text
  1 byte - Unknown (usually 05h)
  2 byte - Number of following 4-Byte Pairs (N)
  N*4 bytes - data (most 4-byte pairs are "xx,FF,FF,FF")
  (controller-data for demo/preview, in format: Time,Lsb,Msb,FFh)
  (or rather: 8bit time, 12bit joy1, 12bit joy2 ...?)
```

#### Unused-Joypad-Data (at P6):   ;<-- if none: four 00h-bytes

```text
  Unknown purpose. The GROMs have more controller data here (similar as
  at P5), but the existing KROM/ATROM do not seem to use that extra data.
```

#### Title Bitmap Compression

> **See:** [SNES Decompression Formats](snes-decompression-formats.md)
