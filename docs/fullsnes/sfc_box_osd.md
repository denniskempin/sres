# SFC-Box OSD Chip (On-Screen Display Controller)

#### OSD Command Summary

```text
  CMD, First Byte (Command+Data)   Second Byte (More Data)  Function
  BASE  b7 b6 b5 b4 b3 b2 b1 b0    b7 b6 b5 b4 b3 b2 b1 b0
  ---  +--+-----------+--------+  +--+--------------------+ -----------------
  0 80 |1 |0  0  0  0 |FL A8 A7|  |0 |A6 A5 A4 A3 A2 A1 A0| Preset VRAM Addr
  1 88 |1 |0  0  0  1 |D2 D1 D0|  |0 |C2 C1 C0 BS B2 B1 B0| Select Color
  2 90 |1 |0  0  1  0 |AT -  M7|  |0 |M6 M5 M4 M3 M2 M1 M0| Write Character
  3 98 |1 |0  0  1  1 |S2 S1 S0|  |0 |SC SC SC -  SB SB SB| Sprite Ctrl 1
  4 A0 |1 |0  1  0  0 |IE IN EB|  |0 |MM CM MP NP -  -  DC| Screen Ctrl 1
  5 A8 |1 |0  1  0  1 |LP DM SG|  |0 |FM SV SD -  W2 W1 W0| Screen Ctrl 2
  6 B0 |1 |0  1  1  0 |BK G1 G0|  |0 |BC VD DG N3 N2 N1 N0| Line Control
  7 B8 |1 |0  1  1  1 |EC XE FO|  |0 |-  -  Y4 Y3 Y2 Y1 Y0| Vertical Offset
  8 C0 |1 |1  0  0  0 |SC XS FC|  |0 |-  X5 X4 X3 X2 X1 X0| Horizontal Offset
  9 C8 |1 |1  0  0  1 |-  -  - |  |0 |-  -  -  -  -  -  - | Reserved
  A D0 |1 |1  0  1  0 |XC XB RA|  |0 |R2 R1 R0 RS U2 U1 U0| Set under-color
  B D8 |1 |1  0  1  1 |-  -  - |  |0 |-  -  -  -  -  -  - | Reserved (Used?)
  C E0 |1 |1  1  0  0 |-  XC XC|  |0 |XC XC XC XD XD XD XD| Sprite Ctrl 2
  D E8 |1 |1  1  0  1 |-  YC YC|  |0 |YC YC YD YD YD YD YD| Sprite Ctrl 3
  E F0 |1 |1  1  1  0 |-  -  - |  |0 |-  -  -  -  -  -  - | Reserved
  F F8 |1 |1  1  1  1 |-  -  - |  |0 |-  -  -  -  -  -  - | Reserved
```

Note: Below descriptions are showing only the 10bit parameter values (without command bits in 1st byte, and without zero-bit in 2nd byte).

#### OSD Command 0 (80h) - Preset VRAM Address

```text
  9    FL  Fill Mode (0=Normal, 1=Fill)
  8-5  An  Address A8-A5 (aka Bit3-0 of Y) (range 0..11)
  4-0  An  Address A4-A0 (aka Bit4-0 of X) (range 0..23)
```

#### OSD Command 1 (88h) - Select Color

```text
  9-7  Dn  Unknown Color?   ;SFCBOX/MB90089 only, not MB90075 (per CHARACTER)
  6-4  Cn  Character Color (can be GRAYSCALE or COLOR)        (per CHARACTER)
  3    BS  Unknown (Shade?) ;MB90089 only, not MB90075/SFCBOX
  2-0  Bn  Background Color (always GRAYSCALE)       (per SCREEN or per LINE?)
```

#### OSD Command 2 (90h) - Write Character

```text
  9    AT  Character Background (0=Normal/Transp, 1=Solid)  ;SFCBOX/MB90089
  8    0   Character Blink (0=Off, 1=Blink)   ;SFCBOX only, not MB90075/MB90089
  7-0  Mn  Character Tile Number (ASCII) (20h=Normal Space, FFh=Transp Space)
```

Before Command 2: Change the VRAM address and Character Color via Commands 0 and 1 (if needed).

Upon Command 2: The specified character is stored in VRAM (together with previously specified color attributes), VRAM address is automatically incremented (and wraps from X=23 to X=0 in next line). If Fill Mode is enabled, then Command 2 repeats until reaching the end of VRAM (Fill may take up to 1ms, do not send further commands during that time).

Writes aren't performed during /HSYNC period (approx 3us), as a simple workaround, configure serial access rate so that an 8-bit transfer takes more than 3us.

#### OSD Command 4 (A0h) - Screen Control 1

```text
  9    IE  Internal/External Sync (0=Internal/Color, 1=External/Mono)
  8    IN  Interlace Mode (0=On, 1=Off)
  7    EB  Unknown (EB)     ;MB90089 only, not MB90075/SFCBOX
  6    MM  Unknown (MM)     ;MB90089 only, not MB90075/SFCBOX
  5    CM  Color/Monochrome (0=Mono, 1=Color) (affects Character + Undercolor)
  4    MP  Unknown (MP)     ;MB90089 only, not MB90075/SFCBOX
  3    NP  NTSC/PAL Mode    (0=NTSC, 1=PAL)
  2-1  0   Reserved (should be 0)
  0    DC  Display Enable   (0=Backdrop, 1=Backdrop+Background+Characters)
```

#### xxx pg17 - details in IE

#### OSD Command 5 (A8h) - Screen Control 2

```text
  9    LP  Unknown    ;MB90089 only, not MB90075/SFCBOX
  8    DM  Unknown    ;MB90089 only, not MB90075/SFCBOX
  7    SG  Unknown    ;MB90089 only, not MB90075/SFCBOX
  6    FM  Unknown    ;MB90089 only, not MB90075/SFCBOX
  5    SV  Unknown    ;MB90089 only, not MB90075/SFCBOX
  4    SD  Unknown    ;MB90089 only, not MB90075/SFCBOX
  3    -   Reserved (should be 0)
  2-0  Wn  Line Spacing    ;SFCBOX/MB90089 only, not MB90075
```

Used by SFC-Box.

#### OSD Command 6 (B0h) - Line Control

```text
  9    BK  Background Type (0=Bordered, 1=Solid 12x18)        ;-per LINE
  8    G1  Character Y-Size  (0=Normal/18pix, 1=Zoomed/36pix) ;-per LINE
  7    G0  Character X-Size  (0=Normal/12pix, 1=Zoomed/24pix) ;-per LINE
           (old MB90075: G0=Unused, G1=Affects both X+Y Size)
  6    BC  Background Control (0=Transparent, 1=Displayed)    ;-per LINE
  5    VD  Analog VOUT,YOUT,COUT Video Enable (0=Off, On)     ;\per SCREEN
  4    DG  Digital VOC2-VOC0,VOB Video Enable (0=Off, On)     ;/
  3-0  Nn  Vertical Line Number (N3-N0) (range 0..11) <-- for per LINE bits
```

Note: On the screen, a double-height line at Line Y extends through Line Y and Y+1, drawing does then continue reading the next VRAM source data from line Y+2 (ie. line Y+1 is NOT drawn).

#### OSD Command 7 (B8h) - Vertical Offset

```text
  9    EC  Output on /HSYNC Pin (0=Composite Video, 1=Hsync)
  8    XE  Unknown (XE)     ;SFCBOX/MB90089 only, not MB90075
  7    FO  Output on FSCO Pin (0=Low, 1=Color Burst)  ;SFCBOX/MB90089 only
  6    0   Reserved (should be 0)
  5    Y5  MSB of Yn?       ;SFCBOX only, not MB90075/MB90089
  4-0  Yn  Vertical Display Start Position (Y4-Y0) (in 2-pixel steps)
```

Vertical Display Start is at "Y*2+1" lines after raising /VBLK. Whereas, raising /VBLK is 15h (NTSC) or 20h (PAL) lines after raising /VSYNC).

SFC-Box seems to use a 6bit offset (so maybe MB90082 has 1-pixel steps).

SFC-Box writes totally bugged values to bit7-9 (writes them to bit8-10, and replaces them by HORIZONTAL bits when changing the VERTICAL setting).

#### OSD Command 8 (C0h) - Horizontal Offset

```text
  9    SC  Input on /EXHSYN Pin (0=Composite Video, 1=Hsync)
  8    XS  Unknown (XS)     ;SFCBOX/MB90089 only, not MB90075
  7    FC  Input on /EXHSYN Pin (0=Use3usFilter, 1=NoFilter)  ;MB90089 only
  6-5  0   Reserved (should be 0)
  5    X5  MSB of Xn        ;SFCBOX/MB90089 only, not MB90075
  4-0  Xn  Horizontal Display Start Position (X4-X0)
```

MB90075: Horizontal Display Start is at "(X+15)*12" dots after raising /HSYNC.

MB90082: Horizontal Display Start is at "(X+?)*?" dots after raising /HSYNC.

MB90089: Horizontal Display Start is at "(X+?)*3" dots after raising /HSYNC.

#### OSD Command A (D0h) - Set Under-color

```text
  9    XC  Unknown (XC)     ;MB90089 only, not MB90075  ;sth on SFCBOX?
  8    XB  Unknown (XB)     ;MB90089 only, not MB90075  ;sth on SFCBOX?
  7    RA  Unknown (RA)     ;MB90089 only, not MB90075/SFCBOX
  6-4  Rn  Unknown (R2-R0)  ;MB90089 only, not MB90075/SFCBOX
  3    RS  Unknown (RS)     ;MB90089 only, not MB90075/SFCBOX
  2-0  Un  Under Color (U2-U0) (aka Backdrop (and Border?) color)
```

Under Color can be COLOR or GRAYSCALE (select via CM bit). Under Color is shown only in INTERNAL sync mode.

OSD Command B (D8h) - Reserved (Used?) This, in SFCBOX and MB90092, is similar to "Sprite Control 1" in MB90089 ???

```text
  9    -   Unknown (unused by SFC-Box)    ;not MB90075, not MB90089
  8    ?   Unknown (used by SFC-Box)      ;not MB90075, not MB90089
  7    -   Unknown (unused by SFC-Box)    ;not MB90075, not MB90089
  6-4  ?   Unknown (used by SFC-Box)      ;not MB90075, not MB90089
  3    -   Unknown (unused by SFC-Box)    ;not MB90075, not MB90089
  2-0  ?   Unknown (used by SFC-Box)      ;not MB90075, not MB90089
```

Used by SFC-Box. Is that a MB90082-only feature? The SFC-Box software contains a function for setting a 1bit flag, and two 3bit parameters (however, it's clipping the "3bit" values to range 0..3); the function is used only once (with flag=00h, and with the other two parameters each set to 01h, and "unused" bits set to zero).

OSD Command 3 (98h) - Sprite Control 1 (TileNo, Char/BG Colors?) OSD Command C (E0h) - Sprite Control 2 (Horizontal Position?) OSD Command D (E8h) - Sprite Control 3 (Vertical Position?)

```text
  Unknown    ;MB90089 only, not MB90075
```

Not used by SFC-Box. According to the poor MB90089 data sheet, TileNo seems to select char "8Fh+(0..7)*10h". And X/Y coordinates seem to consist of character coordinates & pixel-offsets within that character cell.

#### OSD Wake-Up from Reset

After Power-on, the OSD chip is held in Reset-state (with IE=0 and DC=0). To wake-up from that state, issue four /CS=LOW pulses.

#### OSD Video RAM (VRAM)

Main VRAM is 288 cells (24x12), each cell contains:

```text
  8bit Character Tile Number
  3bit Character Color
  probably also 1bit "AT" flag (on chips that do support it)
  plus maybe some more per-character stuff
```

Additionally, there's an array with 12 per-line settings:

```text
  .. zoom bit(s)
  plus maybe some more per-line stuff
```

Other settings (like background & backdrop colors) are per-screen only.

#### SFC-Box Character/Outline/Background Styles

```text
  AT=0      --> draw Background transparent (=Undercolor, or TV layer)
  AT=1      --> draw Background solid by using "Dn" Unknown Color
  AT=0, BK=1, BC=1    --> draw Background solid by "Bn" color (unless Char=FFh)
  AT=x, BK=0, BC=1    --> draw Outline by "Bn" color
```

#### OSD Character Generator ROM (CGROM)

There are 256 characters in CGROM. The Character Set is undocumented, it seems that one can order chips with different/custom character sets, chips with suffix -001 in the part number are probably containing some kind of a "standard" charset.

That "standard" charset contains normal ASCII characters (uppercase & lowercase, with normal ASCII codes eg. 41h="A", but some missing chars like "@|\"), plus japanese symbols, and some graphics symbols (volume bar, AM, PM, No, Tape and arrow symbols).

Character FFh is said to be a "blank/end" code, the meaning there is unknown.

"Blank" might refer to a normal SPACE, but maybe with BG forced to be transparent (even when using solid BG). "End" might be nonsense, or maybe it forces the remaining chars (in the current line, and/or following lines) to be hidden? And/or maybe it acts as CRLF (moving Address to X=0, Y=Y+1)?

#### OSD Color Table

Color Table (according to MB90075 datasheet):

```text
  Value   0     1     2     3       4      5      6      7
  Color   Black Blue  Red   Magenta Green  Cyan   Yellow White
  Mono    Black     ... Increasing  Gray Levels ...      White
```

(ie. Colors are RGB with Bit0=Blue, Bit1=Red, Bit2=Green)

```text
  Characters      --> Grayscale or Color (depending on CM bit)
  Background      --> Always Grayscale
  Backdrop        --> Grayscale or Color (depending on CM bit)
```

Colors should be enabled (via CM bit) only in INTERNAL sync mode.

#### OSD Datasheets / Application Manuals

```text
  MB80075 commands described on 9 pages  ;\these are all 24x12 cells
  MB80082 no datasheet exists?           ; (with extra features in
  MB80089 commands summarized on 1 page  ;/MB80082 and MB80089)
  MB80092 commands described on .. pages ;-similar/newer chip or so
  MB80050 commands summarized on 1 page  ;-different/newer chip or so
```

#### MB90092

```text
  pg80 and up
```

#### MB90089

```text
  vram 24x12 cells (288x216 pixels)
  cgrom (character generator rom) (256 chars, 12x18 pixels)
  base.x  3pix (1/4 character)  ;\relative to END of vblank/hblank
  base.y  2 pix                 ;/
  8 color/grayscales
  shaded (3d) or bordered chars or solid bg
  8 custom chars, one can be displayed
  colors only in INTERNAL sync mode
  mono in EXTERNAL sync mode
  zoom.x / zoom.y  12x18 pix ---> 24x36 pix
  pg10 --> background
  pg11 --> shade
  pg11 --> sprite
  pg11 --> base xy   (6bit x, 5bit y) line space 0..7
  pg12 --> serial access
  pg13 --> COMMANDS
  pg15 --> wake-up
  dot clock (pins EXD,XD) can be 6MHz .. 8MHz (=affects horizontal resolution)
```

FFh is "blank" or "end" code? display details pg10,11

#### MB90075

```text
  base.x is only 5bit (not 6bit)
  zoom affects BOTH x+y (cannot be done separately)
  no SHADED drawing
  vram fill function ?
  CMD 01 --> without D2..D0
  CMD 02 --> without AT
  CMD 03,0C,0D --> reserved (no sprite functions)
  CMD 04 -->
  CMD 05 --> reserved (no screen ctrl 2)
  CMD 06 --> lacks G0 (but DOES have G1 ?)
  CMD 07 --> lacks XE,FO
  CMD 08 --> lacks XS,FC,X5
  CMD 0A --> only U2..U0 (lacks XC,XB,RA,R2,R1,R0,RS)
```

#### details on pg14..21
