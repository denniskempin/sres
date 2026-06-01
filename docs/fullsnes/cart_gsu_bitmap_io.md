# SNES Cart GSU-n Bitmap I/O Ports

```text
3038h - SCBR - Screen Base Register (8bit, in 1Kbyte units) (W)
  0-7  Screen Base in 1K-byte Units (Base = 700000h+N*400h)

303Ah - SCMR - Screen Mode Register (W)
  0-1 MD0-1 Color Gradient (0=4-Color, 1=16-Color, 2=Reserved, 3=256-Color)
  2   HT0   Screen Height  (0=128-Pixel, 1=160-Pixel, 2=192-Pixel, 3=OBJ-Mode)
  3   RAN   Game Pak RAM bus access (0=SNES, 1=GSU)
  4   RON   Game Pak ROM bus access (0=SNES, 1=GSU)
  5   HT1   Screen Height  (MSB of HT0 bit)
  6-7 -     Not used (should be zero)
```

RON/RAN can be temporarily cleared during GSU operation, this causes the GSU to enter WAIT status (if it accesses ROM or RAM), and continues when RON/RAN are changed back to 1.

Note that "OBJ Mode" can be also selected by POR.Bit4 (if so, HT0/HT1 bits are ignored).

```text
  256x128 pixels   256x160 pixels   256x192 pixels   OBJ Mode 256x256 pixel
  000 010 .. 1F0 | 000 014 .. 26C | 000 018 .. 1E8 | 000 .. 00F 100 .. 10F
  001 011 .. 1F1 | 001 015 .. 26D | 001 019 .. 1E9 | ..  .. ..  ..  .. ..
  ..  ..  .. ..  | ..  ..  .. ..  | ..  ..  .. ..  | 0F0 .. 0FF 1F0 .. 1FF
  ..  ..  .. ..  | ..  ..  .. ..  | ..  ..  .. ..  | 200 .. 20F 300 .. 30F
  00E 01E .. 1FE | 012 026 .. 27E | 016 02E .. 2FE | ..  .. ..  ..  .. ..
  00F 01F .. 1FF | 013 027 .. 27F | 017 02F .. 2FF | 2F0 .. 2FF 3F0 .. 3FF
```

In the first three cases, BG Map is simply filled with columns containing increasing tile numbers. The fourth case is matched to the SNES two-dimensional OBJ mapping; it can be used for BG Map (with entries 0..3FF as shown above), or for OBJ tiles (whereas, mind that the SNES supports only 0..1FF OBJs, not 200..3FF).

The Tile Number is calculated as:

```text
  Height 128 --> (X/8)*10h + (Y/8)
  Height 160 --> (X/8)*14h + (Y/8)
  Height 192 --> (X/8)*18h + (Y/8)
  OBJ Mode --> (Y/80h)*200h + (X/80h)*100h + (Y/8 AND 0Fh)*10h + (X/8 AND 0Fh)
```

The Tile-Row Address is:

```text
  4 Color Mode    TileNo*10h + SCBR*400h + (Y AND 7)*2
  16 Color Mode   TileNo*20h + SCBR*400h + (Y AND 7)*2
  256 Color Mode  TileNo*40h + SCBR*400h + (Y AND 7)*2
```

With Plane0,1 stored at Addr+0, Plane 2,3 at Addr+10h, Plane 4,5 at Addr+20h, Plane 6,7 at Addr+30h.

#### N/A - COLR - Color Register

```text
  0-7 CD0-7 Color Data
```

#### N/A - POR - Plot Option Register (CMODE)

```text
  0   PLOT Transparent       (0=Do Not Plot Color 0, 1=Plot Color 0)
  1   PLOT Dither            (0=Normal, 1=Dither; 4/16-color mode only)
  2   COLOR/GETC High-Nibble (0=Normal, 1=Replace incoming LSB by incoming MSB)
  3   COLOR/GETC Freeze-High (0=Normal, 1=Write-protect COLOR.MSB)
  4   OBJ Mode               (0=Normal, 1=Force OBJ mode; ignore SCMR.HT0/HT1)
  5-7 Not used (should be zero)
```

Can be changed by CMODE opcode, used for COLOR/GETC/PLOT opcodes.

Dither can mix transparent & non-transparent pixels.

Bit0=0 (Transparent) causes PLOT to skip color 0 (so PLOT does only increment R1 (X-coordinate), but doesn't draw a pixel). Depending on color depth, the color 0 check tests the lower 2/4/8 bits of the drawing color (if POR.Bit3 (Freeze-High) is set, then it checks only the lower 2/4 bits, and ignores upper 4bit even when in 256-color mode).

Bit1=1 (Dither) causes PLOT to use dithering, that is, if "(r1.bit0 XOR r2.bit0)=1" then COLOR/10h is used as drawing color; using Color 0 as one of the two colors can produce a semi-transparency effect. Dither is ignored in 256-color mode.

Bit2=1 (High-Nibble) causes COLOR/GETC to replace the LSB of the incoming data by the MSB of the incoming data; this allows two 4bit bitmaps being stored at the same memory area (one in the LSBs, the other in MSBs).

Bit3=1 (Freeze-High) causes COLOR/GETC to change only the LSB of the color register; this allows the MSB to be used as fixed palette-like value in 256-color mode, it might be also useful for fixed dither-colors in 4/16 color mode.

Bit3=1 forces OBJ Mode (same as when setting SCMR.HT0/HT1 to OBJ Mode).

```text
  <------- COLOR/GETC TO COLOR ------->    <------- PLOT COLOR TO RAM -------->
               ______              __________                    ______
  Bit7-4 --+--|Freeze|- - - - - ->|          |---+------------->|      |
           |  |POR.3 |            |  COLOR   |   |              |Transp|
           |  |______|  ______    | _ _ _ _  |   |    ______    |POR.0 |--> RAM
           '---------->|Nibble|   |          |   '-->|Dither|   |      |
                       |POR.2 |-->| Register |       |POR.1 |-->|      |
  Bit3-0 ------------->|______|   |__________|------>|______|   |______|
```
