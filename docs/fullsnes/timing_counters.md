# SNES Timing H/V Counters

#### Horizontal Timings

```text
  Scanline Length        1364 master cycles (341 dot cycles)
    Except, Line F0h in Field.Bit=1 of Interlace: 1360 master cycles
  Refresh (per scanline)   40 master cycles (10 dot cycles)

  50*312*1364 = 21.278400 MHz   // 21.281370MHz/(312*1364) = 50.00697891 Hz
  60*262*1364 = 21.442080 MHz   // 21.477270MHz/(262*1364-2) = 60.09880627 Hz
```

#### Long and Short Scanlines

A normal scanline is 1364 master cycles long. But, there are two special cases, in which lines are 4 cycles longer or shorter:

```text
  Short Line --> at 60Hz frame rate + interlace=off + field=1 + line=240
  Long Line  --> at 50Hz frame rate + interlace=on + field=1 + line=311
  (in both cases, the selected picture size, 224 or 239 lines, doesn't matter)
```

Technically, the effects work as so:

```text
  Normal Line : 1364 cycles, 340 dots (0-339), four dots are 5-cycles long
  Long Line   : 1368 cycles, 341 dots (0-340), four dots are 5-cycles long
  Short Line  : 1360 cycles, 340 dots (0-339), all dots are 4-cycles long
```

Glitch: The long scanline is placed in the last line (directly after the Hsync for line 0, thus shifting the Hsync position of Line 1, ie. of the first line of the drawing period), accordingly, the upper some scanlines in interlaced 50Hz mode are visibly shifted to the right (by around one pixel), until after a handful of scanlines the picture stablizes on the new hsync position (ie. trying to display a vertical line will appear a little curved).

#### Long and Short Scanlines (Purpose)

The Scanline Rate doesn't match up with the PAL/NTSC Color Clocks, so, for example, a red rectangle on black background will look like so:

```text
  RGB-Output             Composite-Output        Composite-Output
  Flawless               Static-Error            Flimmering-Error
  RRRRRRRRRRRRRRRR       RRRRRRRRRRRRRRRR        rRRRRRRRRRRRRRRRr
  RRRRRRRRRRRRRRRR        RRRRRRRRRRRRRRRR       rrRRRRRRRRRRRRRRrr
  RRRRRRRRRRRRRRRR         RRRRRRRRRRRRRRRR       rRRRRRRRRRRRRRRRr
  RRRRRRRRRRRRRRRR       RRRRRRRRRRRRRRRR        rRRRRRRRRRRRRRRRr
  RRRRRRRRRRRRRRRR        RRRRRRRRRRRRRRRR       rrRRRRRRRRRRRRRRrr
  RRRRRRRRRRRRRRRR         RRRRRRRRRRRRRRRR       rRRRRRRRRRRRRRRRr
```

Inserting the long/short scanlines does synchronize the Frame Rate with the PAL/NTSC color clocks:

```text
  PAL Mode        Master Clocks (21MHz)       Color Clocks (PAL:4.4MHz)
  50Hz Normal     425568 (312*1364)           88660 (425568/6*5/4)
  50Hz Interlace  426936 (313*1364+4)         88945 (426936/6*5/4)
  NTSC Mode       Master Clocks (21MHz)       Color Clocks (NTSC:3.5MHz)
  30Hz Normal     714732 ((262+262)*1364-4)   119122 (714732/6)
  30Hz Interlace  716100 ((262+263)*1364)     119350 (716100/6)
```

The result is that the composite video output is producing the "Static Error" effect (in the above example, the rectangle has sawtooth-edges). And the "Flimmering" effect is avoided (which would have blurry edges, and which would also appear as if the edges were wandering up) (Note: The flimmering effect can be seen when switching a modded 50Hz PAL console to 60Hz mode).
