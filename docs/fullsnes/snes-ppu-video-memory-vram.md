# SNES PPU Video Memory (VRAM)

#### BG Map (32x32 entries)

Each BG Map Entry consists of a 16bit value as such:

```text
  Bit 0-9   - Character Number (000h-3FFh)
  Bit 10-12 - Palette Number   (0-7)
  Bit 13    - BG Priority      (0=Lower, 1=Higher)
  Bit 14    - X-Flip           (0=Normal, 1=Mirror horizontally)
  Bit 15    - Y-Flip           (0=Normal, 1=Mirror vertically)
```

In the "Offset-per-Tile" modes (Mode 2,4,6), BG3 entries are different:

```text
  Bit 15    Apply offset to H/V (0=H, 1=V)  ;-Mode 4 only
  Bit 14    Apply offset to BG2             ;\Mode 2 (... and Mode 6, though
  Bit 13    Apply offset to BG1             ;/       Mode 6 has only BG1 ?)
  Bit 12-10 Not used
  Bit 9-0   Scroll offset to be applied to BG1/BG2
  Lower 3bit of HORIZONTAL offsets are ignored.
```

In mode7, BG Maps are 128x128, and only the lower 8bit are used as BG map entries:

```text
  Bit15-8  Not used (contains tile-data; no relation to the BG-Map entry)
  Bit7-0   Character Number (00h-FFh) (without XYflip or other attributes)
```

VRAM 8x8 Pixel Tile Data (BG and OBJ) Each 8x8 tile occupies 16, 32, or 64 bytes (for 4, 16, or 256 colors). BG tiles can be 4/16/256 colors (depending on BG Mode), OBJs are always 16 color.

```text
  Color Bits (Planes)     Upper Row ........... Lower Row
  Plane 0 stored in bytes 00h,02h,04h,06h,08h,0Ah,0Ch,0Eh ;\for 4/16/256 colors
  Plane 1 stored in bytes 01h,03h,05h,07h,09h,0Bh,0Dh,0Fh ;/
  Plane 2 stored in bytes 10h,12h,14h,16h,18h,1Ah,1Ch,1Eh ;\for 16/256 colors
  Plane 3 stored in bytes 11h,13h,15h,17h,19h,1Bh,1Dh,1Fh ;/
  Plane 4 stored in bytes 20h,22h,24h,26h,28h,2Ah,2Ch,2Eh ;\
  Plane 5 stored in bytes 21h,23h,25h,27h,29h,2Bh,2Dh,2Fh ; for 256 colors
  Plane 6 stored in bytes 30h,32h,34h,36h,38h,3Ah,3Ch,3Eh ;
  Plane 7 stored in bytes 31h,33h,35h,37h,39h,3Bh,3Dh,3Fh ;/
  In each byte, bit7 is left-most, bit0 is right-most.
  Plane 0 is the LSB of color number.
```

The only exception are Mode 7 BG Tiles, which are stored as 8bit pixels (without spreading the bits across several bit-planes), and, BG VRAM is divided into BG Map at even byte addresses, and Tiles at odd addresses, an 8x8 tiles thus uses the following bytes (64 odd bytes within a 128 byte region):

```text
  Vertical Rows           Left-most .......... Right-Most
  Upper Row      in bytes 01h,03h,05h,07h,09h,0Bh,0Dh,0Fh ;\
  2nd Row        in bytes 11h,13h,15h,17h,19h,1Bh,1Dh,1Fh ;
  3rd Row        in bytes 21h,23h,25h,27h,29h,2Bh,2Dh,2Fh ;
  4th Row        in bytes 31h,33h,35h,37h,39h,3Bh,3Dh,3Fh ; 256-color
  5th Row        in bytes 41h,43h,45h,47h,49h,4Bh,4Dh,4Fh ; Mode 7
  6th Row        in bytes 51h,53h,55h,57h,59h,5Bh,5Dh,5Fh ;
  7th Row        in bytes 61h,63h,65h,67h,69h,6Bh,6Dh,6Fh ;
  Bottom Row     in bytes 71h,73h,75h,77h,79h,7Bh,7Dh,7Fh ;/
```

#### 16x16 (and bigger) Tiles

BG tiles can be up to 16x16 pixels in size, and OBJs up to 64x64. In both cases, the big tiles are combined of multiple 8x8 pixel tiles, whereas VRAM is organized as "two-dimensional" array of 16x64 BG Tiles and 16x32 OBJ Tiles:

The BG Map or OAM entry contain the Tile number (N) for the upper-left 8x8 tile. The tile(s) right of that tile are N+1 (and N+2, N+3, etc). The tile(s) under that tile are N+10h (and N+20h, N+30h, etc).

```text
  32x32 pixel OBJ Tile 000h
  Tile000h,  Tile001h,  Tile002h,  Tile003h
  Tile010h,  Tile011h,  Tile012h,  Tile013h
  Tile020h,  Tile021h,  Tile022h,  Tile023h
  Tile030h,  Tile031h,  Tile032h,  Tile033h
```

The hex-tile numbers could be thus thought of as "Yyxh", with "x" being the 4bit x-index, and "Yy" being the y-index in the array. For OBJ tiles, the are no carry-outs from "x+1" to "y", nor from "y+1" to "Y". Whilst BG tiles are processing carry-outs. For example:

```text
  16x16 BG Tile 1FFh     16x16 OBJ Tile 1FFh
  Tile1ffh Tile200h      Tile1ffh Tile1f0h
  Tile20fh Tile210h      Tile10fh Tile100h
```

#### Accessing VRAM

> **See:** [SNES Memory VRAM Access (Tile and BG Map)](snes-memory-vram-access-tile-and-bg-map.md)
