# SNES PPU Timers and Status

```text
2137h - SLHV - Latch H/V-Counter by Software (R)
  7-0  Not used (CPU Open Bus; usually last opcode, 21h for "MOV A,[2137h]")
```

Reading from this register latches the current H/V counter values into OPHCT/OPVCT, Ports 213Ch and 213Dh.

Reading here works "as if" dragging IO7 low (but it does NOT actually output a LOW level to IO7 on joypad 2).

```text
213Ch - OPHCT - Horizontal Counter Latch (R)
213Dh - OPVCT - Vertical Counter Latch (R)
```

There are three situations that do load H/V counter values into the latches:

```text
  Doing a dummy-read from SLHV (Port 2137h) by software
  Switching WRIO (Port 4201h) Bit7 from 1-to-0 by software
  Lightgun High-to-Low transition (Pin6 of 2nd Controller connector)
```

All three methods are working only if WRIO.Bit7 is (or was) set. If so, data is latched, and (in all three cases) the latch flag in 213Fh.Bit6 is set.

```text
  1st read  Lower 8bit
  2nd read  Upper 1bit (other 7bit PPU2 open bus; last value read from PPU2)
```

There are two separate 1st/2nd-read flipflops (one for OPHCT, one for OPVCT), both flipflops can be reset by reading from Port 213Fh (STAT78), the flipflops aren't automatically reset when latching occurs.

```text
        H Counter values range from 0 to 339, with 22-277 being visible on the
        screen. V Counter values range from 0 to 261 in NTSC mode (262 is
        possible every other frame when interlace is active) and 0 to 311 in
        PAL mode (312 in interlace?), with 1-224 (or 1-239(?) if overscan is
        enabled) visible on the screen.

213Eh - STAT77 - PPU1 Status and Version Number (R)
  7    OBJ Time overflow  (0=Okay, 1=More than 8x34 OBJ pixels per scanline)
  6    OBJ Range overflow (0=Okay, 1=More than 32 OBJs per scanline)
  5    Master/Slave Mode (PPU1.Pin25) (0=Normal=Master)
  4    Not used (PPU1 open bus) (same as last value read from PPU1)
  3-0  PPU1 5C77 Version Number (only version 1 exists as far as I know)
```

The overflow flags are cleared at end of V-Blank, but NOT during forced blank!

The overflow flags are set (regardless of OBJ enable/disable in 212Ch), at following times: Bit6 when V=OBJ.YLOC/H=OAM.INDEX*2, bit7 when V=OBJ.YLOC+1/H=0.

```text
213Fh - STAT78 - PPU2 Status and Version Number (R)
  7    Current Interlace-Frame (0=1st, 1=2nd Frame)
  6    H/V-Counter/Lightgun/Joypad2.Pin6 Latch Flag (0=No, 1=New Data Latched)
  5    Not used (PPU2 open bus) (same as last value read from PPU2)
  4    Frame Rate (PPU2.Pin30)  (0=NTSC/60Hz, 1=PAL/50Hz)
  3-0  PPU2 5C78 Version Number (version 1..3 exist as far as I know)
```

Reading from this register also resets the latch flag (bit6), and resets the two OPHCT/OPVCT 1st/2nd-read flipflops.

#### Lightgun Coordinates

The screen coordinates (X,Y; with 0,0 = upper left) are related as so:

```text
  Super Scope  X=OPHCNT-40, Y=OPVCT-1   (games support software calibration)
  Justifier 1  X=OPHCNT-??, Y=OPVCT-?   (games support software calibration)
  Justifier 2  X=OPHCNT-??, Y=OPVCT-?   (games support software calibration)
  M.A.C.S.     X=OPHCNT-76, Y=OPVCT-41  (hardcoded, mechanical calibration)
```

Drawing starts at H=22, V=1 (which explains parts of the offsets). Horizontal offsets greater than 22 are explained by signal switching/transmission delays.

The huge vertical offset of the M.A.C.S. gun is caused by the lightpen being mounted above of the barrel (rather than inside of it) (and apparently not parallel to it, since V>Y instead of V<Y), this implies that the barrel cannot be aimed at screen coordinates Y=151..191 (since the lightpen would be offscreen).

Most games support software calibration. The only exception is M.A.C.S. which requires mechanical hardware calibration (assisted by a test screen, activated by pressing Button A, and then Select Button on joypad 1).

> **See:** [SNES Controllers SuperScope (Lightgun)](snes-controllers-superscope-lightgun.md)
> **See:** [SNES Controllers Konami Justifier (Lightgun)](snes-controllers-konami-justifier-lightgun.md)
> **See:** [SNES Controllers M.A.C.S. (Lightgun)](snes-controllers-m-a-c-s-lightgun.md)
