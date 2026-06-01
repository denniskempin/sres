# NSS On-Screen Controller (OSD)

#### On-Screen Display Controller M50458-001SP (Mitsubishi Microcomputers)

#### OSD Addresses

The OSD Address is transferred as first word (after chip select):

```text
  0000h..011Fh  Character RAM (24x12 tiles, aka 288 tiles, aka 120h tiles)
  0120h..0127h  Configuration Registers (8 registers)
```

Further words are then written to the specified address (which is auto-incremented after each word).

#### Character Codes (for OSD Address 000h..011Fh)

```text
  0-6   Character Number (non-ASCII)
  7     Unused (zero)
  8-10  Text Color     (on NSS: 3bit RGB) (Bit0=Red, Bit1=Green, Bit2=Blue)
  11    Blinking flag  (0=Normal, 1=Blink)
  12    Underline flag (0=Normal, 1=Underline)
  13-15 Unused (zero)  (on NSS: used as hidden PROM check flags by NSS BIOS)
```

The M50458-001SP charset has been dumped by DogP, letters & punctuation marks are:

```text
  Character  <---00h..0Fh---><---10h..1Fh---><---20h..2Fh---><---30h..3Fh--->
  00h..3Fh  "0123456789-:/.,'ABCDEFGHIJKLMNOPQRSTUVWXYZ[]();?| "
  40h..7Fh  "_abcdefghijklmnopqrstuvwxyz+*=# "
```

All characters are 12x18 pixels in size.

#### OSD M50458 Register 0 - Port Output Control

```text
  0     P0 Usage (0=Manual Control, 1=YM; Luminance)
  1     P1 Usage (0=Manual Control, 1=BLNK; Blanking)
  2     P2 Usage (0=Manual Control, 1=B; Blue)
  3     P3 Usage (0=Manual Control, 1=G; Green)
  4     P4 Usage (0=Manual Control, 1=R; Red)
  5     P5 Usage (0=Manual Control, 1=CSYN; Composite Sync)
  6-11  Manual P0-P5 Output Level (0=Low, 1=High)
  12    Synchronize Port Output with Vsync (0=No, 1=Yes)
  13-15 Unused (zero)
```

NSS uses values 003Fh (whatever/maybe SNES as backdrop), and 00BDh (maybe solid backdrop).

#### OSD M50458 Register 1 - Horizontal Display Start/Zoom

```text
  0-5   Horizontal Display Start in 4-pixel (?) units
  6-7   Horizontal Character Size in Line 1     (0..3 = 1,2,3,4 pixels/dot)
  8-9   Horizontal Character Size in Line 2..11 (0..3 = 1,2,3,4 pixels/dot)
  10-11 Horizontal Character Size in Line 12    (0..3 = 1,2,3,4 pixels/dot)
  12    PAL: Interlace Lines (0=625 Lines, 1=627 Lines) NTSC: Unused (zero)
  13-15 Unused (zero)
```

NSS uses 0018h (normal centered display) and 011Bh (fine-adjusted position in intro screen).

#### OSD M50458 Register 2 - Vertical Display Start/Zoom

```text
  0-5   Vertical Display Start in 4-scanline (?) units
  6-7   Vertical Character Size in Line 1     (0..3 = 1,2,3,4 pixels/dot)
  8-9   Vertical Character Size in Line 2..11 (0..3 = 1,2,3,4 pixels/dot)
  10-11 Vertical Character Size in Line 12    (0..3 = 1,2,3,4 pixels/dot)
  12    Halftone in Superimpose Display (0=Halftone Off, Halftone On)
  13-15 Unused (zero)
```

NSS uses 0009h (normal centered display) and 0107h (fine-adjusted position in intro screen).

#### OSD M50458 Register 3 - Character Size

```text
  0-4   Vertical Scroll Dot Offset (within char) (0..17) (18..31=Reserved)
  5-6   Vertical Space between Line 1 and 2 (0..3 = 0,18,36,54 scanlines)
  7     Control RS,CB Terminals (0=Both Off, 1=Both On)
  8-11  Vertical Scroll Char Offset (0=No Scroll, 1..11=Line 2-12, 12..15=Res.)
  12    PAL: Revise 25Hz Vsync (0=No, 1=Yes/Revice)  NTSC: Unused
  13-15 Unused (zero)
```

NSS uses 0000h (normal 1x1 pix size) and 082Ah (large 2x2 pix "NINTENDO" in intro), 0y20h (in-demo: instructions with double-height headline? and y-scroll on 2nd..10th line), 0y00h (in-game: instructions without headline and fullscreen scroll).

#### Verical Scroll OFF: Show 12 lines

Verical Scroll ON: Show 11 lines (1st line fixed, 10 lines scrolled) (in scroll mode only 11 lines are shown) (allowing to update the hidden 12th line without disturbing the display)

#### OSD M50458 Register 4 - Display Mode

```text
  0-11  Display Mode Flags for Line 1..12 (0=Via BLK0,BLK1, 1=Via Different)
  12    LINEU - Underline Display (0=Off, 1=On) "depends on above bit0-bit11"
  13-15 Unused (zero)
```

NSS uses 0000h.

#### OSD M50458 Register 5 - Blinking and so on

```text
  0-1   Blink Duty  (0=Off, 1=25%, 2=50%, 3=75%) (WHAT color during WHAT time?)
  2     Blink Cycle (0=64 Frames, 1=32 Frames)
  3     Horizontal Border Size (0..1 = 1,2 dots)
  4-5   Blink/Inverse Mode (0=Cursor, 1=ReverseChr, 2=ReverseBlink, 3=AltBlink)
           aka EXP0,EXP1 (see details below)
  6     Horizontal Display Range when all chars are in matrix-outline (0..1=?)
  7     OSCIN frequency (0=4*fsec, 1=2*fsec) (for NTSC only)
  8     Color Burst Width (0=Standard, 1=Altered)
  9     Vsync Signal separated from Composite Sync (0=No, 1=Separated Circut)
  10-12 Test Register "Exception video RAM display mode" (should be zero)
  13-15 Unused (zero)
```

NSS uses 0240h, 0241h, 0247h.

#### OSD M50458 Register 6 - Raster Color

```text
  0-2   Raster Color    (on NSS: 3bit RGB) (Bit0=Red, Bit1=Green, Bit2=Blue)
          (aka Backdrop color?)
  3     Composite Signal BIAS (0=Internal BIAS Off, 1=Internal BIAS On)
  4-6   Character Background Color         (Bit0=Red, Bit1=Green, Bit2=Blue)
  7     Blanking Level (0=White, 1=Black)
  8-10  Cursor and Underline Display Color (Bit0=Red, Bit1=Green, Bit2=Blue)
  11    Cursor/Underline Color for Dot 1  (0=From VRAM, 1=From above bit8-10)
  12    Cursor/Underline Color for Dot 18 (0=From VRAM, 1=From above bit8-10)
  13-15 Unused (zero)
```

NSS uses 1804h, 1880h, 1882h, 1884h.

#### OSD M50458 Register 7 - Control Display

```text
  0     Raster (backdrop?) blanking (0=By Mode;bit2-3?, 1=Whole TV full raster)
  1     Background Color Brightness for RGB (0=Normal, 1=Variable) huh?
  2-3   Mode (0=Blanking OFF, 1=Chr Size, 2=Border Size, 3=Matrix-outline Size)
            aka special meanings in conjunction with register 4 (?)
  4     Mode (0=External Sync, 1=Internal Sync)
  5     Erase RAM (0=No, 1=Erase RAM) (=clear screen?)
  6     Display Output Enable for Composite Signal (0=Off, 1=On)
  7     Display Output Enable for RGB Signal       (0=Off, 1=On)
  8     Stop OSCIN/OSCOUT (0=Oscillate, 1=Stop) (for sync signals)
  9     Stop OSC1/OSC2    (0=Oscillate, 1=Stop) (for display)
  10    Exchange External C by Internal C in Y-C Mode (0=Normal, 1=Exchange)
  11    Video Signal (0=Composite, 1=Y-C output)
  12    Interlace Enable (0=Enable, 1=Disable) (only in Internal Sync mode)
  13-15 Unused (zero)
```

NSS uses 1289h, 12A9h and 12B9h.

#### NSS OSD Dotclock

The OSD chip is having an unknown dotclock (somewhat higher than the SNES dotclock: 12 pixels on OSD are having roughly the same width as 8 pixels on SNES).

#### Blink/Underline

```text
  <Register> <VramAttr> Shape
  EXP1 EXP0  EXP BLINK
  x    x     0   0      " A "             Normal
  x    x     0   1      " A " <--> "   "  Character is blinking
  0    0     1   0      "_A_"             Underlined
  0    0     1   1      "_A_" <--> " A "  Underline is blinking
  0    1     1   0      "[A]"             Inverted Character
  0    1     1   1      "[A]" <--> " A "  Inversion is blinking
  1    0     1   0      "[A]"             Inverted Character
  1    0     1   1      "[A]" <--> " A "  Inversion is blinking
  1    1     1   0      "   " <--> " A "  Character is blinking, duty swapped
  1    1     1   1      " A " <--> "_ _"  Character and Underline alternating
```
