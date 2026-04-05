# SNES Controllers I/O Ports - Automatic Reading

```text
4218h/4219h - JOY1L/JOY1H - Joypad 1 (gameport 1, pin 4) (R)
421Ah/421Bh - JOY2L/JOY2H - Joypad 2 (gameport 2, pin 4) (R)
421Ch/421Dh - JOY3L/JOY3H - Joypad 3 (gameport 1, pin 5) (R)
421Eh/421Fh - JOY4L/JOY4H - Joypad 4 (gameport 2, pin 5) (R)
  Register    Serial     Default
  Bit         Transfer   Purpose
  Number______Order______(Joypads)_____
  15          1st        Button B          (1=Low=Pressed)
  14          2nd        Button Y
  13          3rd        Select Button
  12          4th        Start Button
  11          5th        DPAD Up
  10          6th        DPAD Down
  9           7th        DPAD Left
  8           8th        DPAD Right
  7           9th        Button A
  6           10th       Button X
  5           11th       Button L
  4           12th       Button R
  3           13th       0 (High)
  2           14th       0 (High)
  1           15th       0 (High)
  0           16th       0 (High)
```

Before reading above ports, set Bit 0 in port 4200h to request automatic reading, then wait until Bit 0 of port 4212h gets set-or-cleared? Once 4200h enabled, seems to be automatically read on every retrace?

Be sure that Out0 in Port 4016h is zero (otherwise the shift register gets stuck on the first bit, ie. all 16bit will be equal to the B-button state.

#### AUTO JOYPAD READ

```text
 ----------------
```

When enabled, the SNES will read 16 bits from each of the 4 controller port  data lines into registers $4218-f. This begins between H=32.5 and H=95.5 of  the first V-Blank scanline, and ends 4224 master cycles later. Register $4212  bit 0 is set during this time. Specifically, it begins at H=74.5 on the first  frame, and thereafter some multiple of 256 cycles after the start of the  previous read that falls within the observed range.

Reading $4218-f during this time will read back incorrect values. The only  reliable value is that no buttons pressed will return 0 (however, if buttons  are pressed 0 could still be returned incorrectly). Presumably reading $4016/7  or writing $4016 during this time will also screw things up.
