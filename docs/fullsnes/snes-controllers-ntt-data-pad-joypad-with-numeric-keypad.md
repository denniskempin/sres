# SNES Controllers NTT Data Pad (joypad with numeric keypad)

Special joypad with numeric keypad, for use with SFC Modem:

> **See:** [SNES Add-On SFC Modem (for JRA PAT)](snes-add-on-sfc-modem-for-jra-pat.md)

NTT Data Controller Pad - 27-buttons (including the 4 direction keys)

```text
               _______________
    __--L--___/               \__--R--__
   /    _        (<)(>)   (=)           \
  |   _| |_   (1)(2)(3)(*)(C)     (X)    |
  |  |_   _|  (4)(5)(6)(#)(.)  (Y)   (A) |
  |    |_|    (7)(8)(9)( 0  )     (B)    |
  |                                      |
   \__________.---------------._________/
```

#### NTT Data Controller Pad Bits

```text
  1st   Bit31  Button (B)        (0=High=Released, 1=Low=Pressed)
  2nd   Bit30  Button (Y)        (0=High=Released, 1=Low=Pressed)
  3rd   Bit29  Button (<) Select (0=High=Released, 1=Low=Pressed)
  4th   Bit28  Button (>) Start  (0=High=Released, 1=Low=Pressed)
  5th   Bit27  Direction Up      (0=High=Released, 1=Low=Pressed)
  6th   Bit26  Direction Down    (0=High=Released, 1=Low=Pressed)
  7th   Bit25  Direction Left    (0=High=Released, 1=Low=Pressed)
  8th   Bit24  Direction Right   (0=High=Released, 1=Low=Pressed)
  9th   Bit23  Button (A)        (0=High=Released, 1=Low=Pressed)
  10th  Bit22  Button (X)        (0=High=Released, 1=Low=Pressed)
  11th  Bit21  Button (L)        (0=High=Released, 1=Low=Pressed)
  12th  Bit20  Button (R)        (0=High=Released, 1=Low=Pressed)
  13th  Bit19  ID Bit3           (always 0=High)
  14th  Bit18  ID Bit2           (always 1=Low)
  15th  Bit17  ID Bit1           (always 0=High)
  16th  Bit16  ID Bit0           (always 0=High)
  17th  Bit15  Button (0)        (0=High=Released, 1=Low=Pressed)
  18th  Bit14  Button (1)        (0=High=Released, 1=Low=Pressed)
  19th  Bit13  Button (2)        (0=High=Released, 1=Low=Pressed)
  20th  Bit12  Button (3)        (0=High=Released, 1=Low=Pressed)
  21th  Bit11  Button (4)        (0=High=Released, 1=Low=Pressed)
  22th  Bit10  Button (5)        (0=High=Released, 1=Low=Pressed)
  23th  Bit9   Button (6)        (0=High=Released, 1=Low=Pressed)
  24th  Bit8   Button (7)        (0=High=Released, 1=Low=Pressed)
  25th  Bit7   Button (8)        (0=High=Released, 1=Low=Pressed)
  26th  Bit6   Button (9)        (0=High=Released, 1=Low=Pressed)
  27th  Bit5   Button (*)        (0=High=Released, 1=Low=Pressed)
  28th  Bit4   Button (#)        (0=High=Released, 1=Low=Pressed)
  29th  Bit3   Button (.) Dot    (0=High=Released, 1=Low=Pressed)
  30th  Bit2   Button (C) Clear  (0=High=Released, 1=Low=Pressed)
  31th  Bit1   Unknown/Unused    (unknown, probably always 1 or always 0)
  32th  Bit0   Button (=) End    (0=High=Released, 1=Low=Pressed)
  33th and up  Padding           (unknown, probably always 1 or always 0)
```

Note: The "(=)" button is sunken, somewhat preventing accidently pressing it.
