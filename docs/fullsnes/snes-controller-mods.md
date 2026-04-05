# SNES Controller Mods

#### Shift Registers

SNES Joypads are basically consisting of buttons wired to a 16bit shift register. This could be reproduced using two 4021 chips (two 8bit parallel-in serial-out shift registers).

#### SNES PAL vs NTSC Controllers

For using SNES NTSC controllers on SNES PAL consoles:

> **See:** [SNES Controllers Pinouts](snes-controllers-pinouts.md)

SNESPAD (SNES Controller to PC Parallel Port) This is a circuit for connecting up to five SNES joypads to a PC Parallel Port, using 25pin DSUB or 36pin Centronics connector. The circuit can be used with drivers like "Direct Pad Pro" or "PPJoy", or by emulators with built-in SNESPAD support.

```text
  Pin DB25  CNTR
  d3  5     5 ---|>|--.            .---.
  d4  6     6 ---|>|--+------------| O | 1 vcc
  d5  7     7 ---|>|--|  .---------| O | 2 clk
  d6  8     8 ---|>|--|  | .-------| O | 3 stb
  d7  9     9 ---|>|--'  | | .-----|_O_| 4 dta1
  d0  2     2 -----------' | |     | O | 5 dta3
  d1  3     3 -------------' |     | O | 6 io
  x   x     x ---------------' .---| O | 7 gnd
  gnd 18-25 19-30 -------------'    \_/
```

For Pad 1..5, wire Pin "x" to ack,pe,slct,err,busy (aka DB25 pin 10, 12, 13, 15, 11) (aka CNTR pin 10, 12, 13, 32, 11) (aka bit6, bit5, bit4, bit3, NOT(bit7) in the PC's I/O Port). The circuit is pretty well standarized (there is only one variant, a so-called "Linux" circuit with messed-up pin ordering: ack,busy,pe,slct,err for pad 1..5).

#### 7pin Connectors (1mm pin diameter)

With some efforts, these can be made pulling contacts from regular DSUB connectors (which have same pin diameter, but different pin spacing). Solder the contacts onto a piece of board, and eventually build some plastic block with holes/notches as in real SNES connectors. Alternately, SNES extension cables (with one male & one female connector) are reportedly available.
