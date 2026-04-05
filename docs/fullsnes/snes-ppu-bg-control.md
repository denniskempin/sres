# SNES PPU BG Control

```text
2105h - BGMODE - BG Mode and BG Character Size (W)
  7    BG4 Tile Size (0=8x8, 1=16x16)  ;\(BgMode0..4: variable 8x8 or 16x16)
  6    BG3 Tile Size (0=8x8, 1=16x16)  ; (BgMode5: 8x8 acts as 16x8)
  5    BG2 Tile Size (0=8x8, 1=16x16)  ; (BgMode6: fixed 16x8?)
  4    BG1 Tile Size (0=8x8, 1=16x16)  ;/(BgMode7: fixed 8x8)
  3    BG3 Priority in Mode 1 (0=Normal, 1=High)
  2-0  BG Screen Mode (0..7 = see below)
```

The BG Screen Modes are:

```text
  Mode   BG1         BG2         BG3         BG4
  0      4-color     4-color     4-color     4-color   ;Normal
  1      16-color    16-color    4-color     -         ;Normal
  2      16-color    16-color    (o.p.t)     -         ;Offset-per-tile
  3      256-color   16-color    -           -         ;Normal
  4      256-color   4-color     (o.p.t)     -         ;Offset-per-tile
  5      16-color    4-color     -           -         ;512-pix-hires
  6      16-color    -           (o.p.t)     -         ;512-pix plus Offs-p-t
  7      256-color   EXTBG       -           -         ;Rotation/Scaling
```

Mode 7 supports rotation/scaling and EXTBG (but doesn't support hv-flip).

Mode 5/6 don't support screen addition/subtraction.

CG Direct Select is support on BG1 of Mode 3/4, and on BG1/BG2? of Mode 7.

```text
2106h - MOSAIC - Mosaic Size and Mosaic Enable (W)
```

Allows to divide the BG layer into NxN pixel blocks, in each block, the hardware picks the upper-left pixel of each block, and fills the whole block by the color - thus effectively reducing the screen resolution.

```text
  7-4  Mosaic Size        (0=Smallest/1x1, 0Fh=Largest/16x16)
  3    BG4 Mosaic Enable  (0=Off, 1=On)
  2    BG3 Mosaic Enable  (0=Off, 1=On)
  1    BG2 Mosaic Enable  (0=Off, 1=On)
  0    BG1 Mosaic Enable  (0=Off, 1=On)
```

Horizontally, the first block is always located on the left edge of the TV screen. Vertically, the first block is located on the top of the TV screen.

When changing the mosaic size mid-frame, the hardware does first finish current block (using the old vertical size) before applying the new vertical size.

Technically, vertical mosaic is implemented as so: subtract the veritical index (within the current block) from the vertical scroll register (BGnVOFS).

```text
2107h - BG1SC - BG1 Screen Base and Screen Size (W)
2108h - BG2SC - BG2 Screen Base and Screen Size (W)
2109h - BG3SC - BG3 Screen Base and Screen Size (W)
210Ah - BG4SC - BG4 Screen Base and Screen Size (W)
  7-2  SC Base Address in VRAM (in 1K-word steps, aka 2K-byte steps)
  1-0  SC Size (0=One-Screen, 1=V-Mirror, 2=H-Mirror, 3=Four-Screen)
                   (0=32x32, 1=64x32, 2=32x64, 3=64x64 tiles)
               (0: SC0 SC0    1: SC0 SC1  2: SC0 SC0  3: SC0 SC1   )
               (   SC0 SC0       SC0 SC1     SC1 SC1     SC2 SC3   )
```

Specifies the BG Map addresses in VRAM. The "SCn" screens consists of 32x32 tiles each.

Ignored in Mode 7 (Base is always zero, size is always 128x128 tiles).

```text
210Bh/210Ch - BG12NBA/BG34NBA - BG Character Data Area Designation (W)
  15-12 BG4 Tile Base Address (in 4K-word steps)
  11-8  BG3 Tile Base Address (in 4K-word steps)
  7-4   BG2 Tile Base Address (in 4K-word steps)
  3-0   BG1 Tile Base Address (in 4K-word steps)
```

Ignored in Mode 7 (Base is always zero).

```text
210Dh - BG1HOFS - BG1 Horizontal Scroll (X) (W) and M7HOFS
210Eh - BG1VOFS - BG1 Vertical Scroll (Y) (W) and M7VOFS
210Fh - BG2HOFS - BG2 Horizontal Scroll (X) (W)
2110h - BG2VOFS - BG2 Vertical Scroll (Y) (W)
2111h - BG3HOFS - BG3 Horizontal Scroll (X) (W)
2112h - BG3VOFS - BG3 Vertical Scroll (Y) (W)
2113h - BG4HOFS - BG4 Horizontal Scroll (X) (W)
2114h - BG4VOFS - BG4 Vertical Scroll (Y) (W)
  1st Write: Lower 8bit  ;\1st/2nd write mechanism uses "BG_old"
  2nd Write: Upper 2bit  ;/
```

Note: Port 210Dh/210Eh are also used as M7HOFS/M7VOFS, these registers have a similar purpose, but internally they are separate registers: Writing to 210Dh does BOTH update M7HOFS (via M7_old mechanism), and also updates BG1HOFS (via BG_old mechanism). In the same fashion, 210Eh updates both M7VOFS and BG1VOFS.

```text
          BGnHOFS = (Current<<8) | (Prev&~7) | ((Reg>>8)&7);
          Prev = Current;
            or
          BGnVOFS = (Current<<8) | Prev;
          Prev = Current;
```
