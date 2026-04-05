# SNES PPU Rotation/Scaling

```text
211Ah - M7SEL - Rotation/Scaling Mode Settings (W)
  7-6   Screen Over (see below)
  5-2   Not used
  1     Screen V-Flip (0=Normal, 1=Flipped)     ;\flip 256x256 "screen"
  0     Screen H-Flip (0=Normal, 1=Flipped)     ;/
```

Screen Over (when exceeding the 128x128 tile BG Map size):

```text
  0=Wrap within 128x128 tile area
  1=Wrap within 128x128 tile area (same as 0)
  2=Outside 128x128 tile area is Transparent
  3=Outside 128x128 tile area is filled by Tile 00h

211Bh - M7A - Rotation/Scaling Parameter A (and Maths 16bit operand) (W)
211Ch - M7B - Rotation/Scaling Parameter B (and Maths 8bit operand) (W)
211Dh - M7C - Rotation/Scaling Parameter C (W)
211Eh - M7D - Rotation/Scaling Parameter D (W)
  1st Write: Lower 8bit  ;\1st/2nd write mechanism uses "M7_old"
  2nd Write: Upper 8bit  ;/
```

Signed 16bit values in 1/256 pixel units  (1bit sign, 7bit integer, 8bit fraction).

```text
210Dh - M7HOFS/BG1HOFS - BG1 Horizontal Scroll (X) (W)
210Eh - M7VOFS/BG1VOFS - BG1 Vertical Scroll (Y) (W)
211Fh - M7X - Rotation/Scaling Center Coordinate X (W)
2120h - M7Y - Rotation/Scaling Center Coordinate Y (W)
  1st Write: Lower 8bit  ;\1st/2nd write mechanism uses "M7_old"
  2nd Write: Upper 5bit  ;/
```

Signed 13bit values in pixel units (1bit sign, 12bit integer, 0bit fraction).

#### Formula

Formula for Rotation/Enlargement/Reduction in Matrix Form:

```text
  ( VRAM.X )  =  ( M7A M7B )  *  ( SCREEN.X+M7HOFS-M7X )  +  ( M7X )
  ( VRAM.Y )     ( M7C M7D )     ( SCREEN.Y+M7VOFS-M7Y )     ( M7Y )
```

Parameters:

```text
  M7A=+COS(angle)*ScaleX, M7B=+SIN(angle)*ScaleX
  M7C=-SIN(angle)*ScaleY, M7D=+COS(angle)*ScaleY
  M7X,M7Y       = Center Coordinate
  M7HOFS,M7VOFS = Scroll Offset
  SCREEN.X = Display (Target) X-Coordinate: (0..255) XOR (xflip*FFh)
  SCREEN.Y = Display (Target) Y-Coordinate: (1..224 or 1..239) XOR (yflip*FFh)
  VRAM.X,Y = BG Map (Source) Coordinates (in 1/256 pixel units)
```

To calculate VRAM coordinates for any SCREEN coordinates:

```text
  IF xflip THEN SCREEN.X=((0..255) XOR FFh), ELSE SCREEN.X=(0..255)
  IF yflip THEN SCREEN.Y=((1..224/239) XOR FFh), ELSE SCREEN.Y=(1..224/239)
  ORG.X = (M7HOFS-M7X) AND NOT 1C00h, IF ORG.X<0 THEN ORG.X=ORG.X OR 1C00h
  ORG.Y = (M7VOFS-M7Y) AND NOT 1C00h, IF ORG.Y<0 THEN ORG.Y=ORG.Y OR 1C00h
  VRAM.X = ((M7A*ORG.X) AND NOT 3Fh) + ((M7B*ORG.Y) AND NOT 3Fh) + M7X*100h
  VRAM.Y = ((M7C*ORG.X) AND NOT 3Fh) + ((M7D*ORG.Y) AND NOT 3Fh) + M7Y*100h
  VRAM.X = VRAM.X + ((M7B*SCREEN.Y) AND NOT 3Fh) + (M7A*SCREEN.X)
  VRAM.Y = VRAM.Y + ((M7D*SCREEN.Y) AND NOT 3Fh) + (M7C*SCREEN.X)
```

After calculating the left-most pixel of a scanline, the following pixels on that scanline can be also calculated by increasing VRAM coordinates as so:

```text
  IF xflip THEN VRAM.X=VRAM.X-M7A, ELSE VRAM.X=VRAM.X+M7A
  IF xflip THEN VRAM.Y=VRAM.Y-M7C, ELSE VRAM.Y=VRAM.Y+M7C
  (The result is same as on hardware, although the real hardware doesn't seem
  to use that method, instead it seems to contain an excessively fast multiply
  unit that recalculates (M7A*SCREEN.X) and (M7C*SCREEN.X) on every pixel.)
```

The VRAM coordinates are then: bit0-7=Fraction, bit8-10=pixel index (within a tile), bit11-17=map index (within BG map), bit18-and-up=nonzero when exceeding BG map size (do "screen over" handling).

#### M7A/M7B Port Notes

Port 211Bh/211Ch can be also used for general purpose math multiply:

> **See:** [SNES Maths Multiply/Divide](snes-maths-multiply-divide.md)

When in BG Mode 7, general purpose multiply works only during V-Blank and Forced-Blank. During drawing period at SCREEN.Y=0..224/239 (including SCREEN.Y=0, and during all 340 dots including H-Blank), MPYL/M/H receives two multiplication results per pixel (one per half-pixel):

```text
  MPY = M7A * ORG.X / 8                                ;at SCREEN.X=-3.0
  MPY = M7D * ORG.Y / 8                                ;at SCREEN.X=-2.5
  MPY = M7B * ORG.Y / 8                                ;at SCREEN.X=-2.0
  MPY = M7C * ORG.X / 8                                ;at SCREEN.X=-1.5
  MPY = M7B * ((SCREEN.Y-MOSAIC.Y) XOR (yflip*FFh))/ 8 ;at SCREEN.X=-1.0
  MPY = M7D * ((SCREEN.Y-MOSAIC.Y) XOR (yflip*FFh))/ 8 ;at SCREEN.X=-0.5
  MPY = M7A * ((SCREEN.X AND FFh) XOR (xflip*FFh)) / 8 ;at SCREEN.X=0.0..336.0
  MPY = M7C * ((SCREEN.X AND FFh) XOR (xflip*FFh)) / 8 ;at SCREEN.X=0.5..336.5
  MPY = M7A * (M7B/100h)                   ;during in V-Blank and Forced-Blank
```

Note: The "/8" suggests that the hardware strips the lower 3bit, however, before summing up the multiply results, it DOES strip the lower 6bit (hence the AND NOT 3Fh in the formula).

#### M7HOVS/M7VOFS Port Notes

Port 210Dh/210Eh are also used as BG1HOFS/BG1VOFS, these registers have a similar purpose, but internally they are separate registers: Writing to 210Dh does BOTH update M7HOFS (via M7_old mechanism), and also updates BG1HOFS (via BG_old mechanism). In the same fashion, 210Eh updates both M7VOFS and BG1VOFS.

M7xx - Write-twice mechanism for Mode 7 Writing a <new> byte to one of the write-twice M7'registers does:

```text
  M7_reg = new * 100h + M7_old
  M7_old = new
```

M7_old is an internal 8bit register, shared by 210Dh-210Eh and 211Bh-2120h.

#### EXTBG

EXTBG is an "external" BG layer (replacing BG2???) enabled via SETINI.6. On the SNES, the 8bit external input is simply shortcut with one half of the PPUs 16bit data bus. So, when using EXTBG in BG Mode 0-6, one will just see garbage.

However, in BG Mode 7, it's receiving the same 8bit value as the current BG1 pixel - but, unlike BG1, with bit7 treated as priority bit (and only lower 7bit used as BG2 pixel color).
