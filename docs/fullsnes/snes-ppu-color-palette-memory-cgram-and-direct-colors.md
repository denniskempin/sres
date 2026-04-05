# SNES PPU Color Palette Memory (CGRAM) and Direct Colors

#### CGRAM Palette Entries

```text
  15    Not used (should be zero) (read: PPU2 Open Bus)
  14-10 Blue
  9-5   Green
  4-0   Red
```

For accessing CGRAM, see:

> **See:** [SNES Memory CGRAM Access (Palette Memory)](snes-memory-cgram-access-palette-memory.md)

#### CGRAM Palette Indices

```text
  00h      Main Backdrop color (used when all BG/OBJ pixels are transparent)
  01h-FFh  256-color BG palette (when not using direct-color mode)
  01h-7Fh  128-color BG palette (BG2 in Mode 7)
  01h-7Fh  Eight 16-color BG palettes
  01h-1Fh  Eight 4-color BG palettes (except BG2-4 in Mode 0)
  21h-3Fh  Eight 4-color BG palettes (BG2 in Mode 0 only)
  41h-5Fh  Eight 4-color BG palettes (BG3 in Mode 0 only)
  61h-7Fh  Eight 4-color BG palettes (BG4 in Mode 0 only)
  81h-FFh  Eight 16-color OBJ palettes (half of them with color-math disabled)
  N/A      Sub Backdrop color (not in CGRAM, set via COLDATA, Port 2132h)
```

Direct Color Mode 256-color BGs (ie. BG1 in Mode 3,4,7) can be optionally set to direct-color mode (via 2130h.Bit0; this bit hides in one of the Color-Math registers, although it isn't related to Color-Math). The 8bit Color number is interpreted as "BBGGGRRR", the 3bit Palette number (from the BG Map) contains LSBs as "bgr", together they are forming a 15bit color "BBb00:RRRr0:GGGg0". Whereas the "bgr" can defined only per tile (not per pixel), and it can be defined only in BG Modes 3-4 (Mode 7 has no palette attributes in BG Map).

```text
  Color Bit7-0 all zero --> Transparent    ;-Color "Black" is Transparent!
  Color Bit7-6   Blue Bit4-3               ;\
  Palette Bit 2  Blue Bit2                 ; 5bit Blue
  N/A            Blue Bit1-0 (always zero) ;/
  Color Bit5-3   Green Bit4-2              ;\
  Palette Bit 1  Green Bit1                ; 5bit Green
  N/A            Green Bit0 (always zero)  ;/
  Color Bit2-0   Red Bit4-2                ;\
  Palette Bit 0  Red Bit1                  ; 5bit Red
  N/A            Red Bit0 (always zero)    ;/
```

To define Black, either set the Backdrop to Black (works only if there are no layers behind BG1), or use a dark non-transparent "near-black" color (eg.

01h=Dark Red, 09h=Dark Brown).

#### Screen Border Color

The Screen Border is always Black (no matter of CGRAM settings and Sub Screen Backdrop Color). NTSC 256x224 images are (more or less) fullscreen, so there should be no visible screen border (however, a 2-3 pixel border may appear at the screen edges if the screen isn't properly centered). Both PAL 256x224 and PAL 256x239 images images will have black upper and lower screen borders (a fullscreen PAL picture would be around 256x264, which isn't supported by the SNES).

#### Forced Blank Color

In Forced Blank, the whole screen is Black (no matter of CGRAM settings, Sub Screen Backdrop Color, and Master Brightness settings). Vsync/Hsync are kept generated (sending a black picture with valid Sync signals to the TV set).
