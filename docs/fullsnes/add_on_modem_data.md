# SNES Add-On SFC Modem - Data I/O

The modem is intended to be connected to controller port 2. RX Data, TX Data, and Modem Status are simultaneously transferred via three I/O lines. The overall transfer length (with ID bits) is 16-bit, however, after checking the ID bits, one can abbreviate the transfer to 9-bit length.

#### JOY2: (4017h.Bit0) - RX Data and ID Bits

```text
  1st          RX Data Bit7    (0=High=Zero, 1=Low=One) ;\
  2nd          RX Data Bit6    (0=High=Zero, 1=Low=One) ;
  3rd          RX Data Bit5    (0=High=Zero, 1=Low=One) ; to be ignored when
  4th          RX Data Bit4    (0=High=Zero, 1=Low=One) ; no RX Data Present
  5th          RX Data Bit3    (0=High=Zero, 1=Low=One) ;
  6th          RX Data Bit2    (0=High=Zero, 1=Low=One) ;
  7th          RX Data Bit1    (0=High=Zero, 1=Low=One) ;
  8th          RX Data Bit0    (0=High=Zero, 1=Low=One) ;/
  9th          RX Data Present (0=High=None, 1=Low=Yes)
  10th         Unknown/Unused
  11th         Unknown/Unused
  12th         Unknown/Unused
  13th         ID Bit3 (always 0=High)
  14th         ID Bit2 (always 0=High)
  15th         ID Bit1 (always 1=Low)
  16th         ID Bit0 (always 1=Low)
  17th and up  Unknown/Unused (probably always whatever)
```

#### JOY4: (4017h.Bit1) - Modem Status

```text
  1st          Unknown Flags Bit7  (1=Low=Busy or so, 0=Ready to get TX Data)
  2nd          Unknown Flags Bit6  (0=High=Error/Abort or so)
  3rd          Unknown Flags Bit5  (1=Low=Busy or so)
  4th          Unknown Flags Bit4  (1=Low=Busy or so)
  5th          Unknown Flags Bit3  Unused?
  6th          Unknown Flags Bit2  Unused?
  7th          Unknown Flags Bit1  Unused?
  8th          Unknown Flags Bit0  Unused?
  9th and up   Unknown/Unused (probably always whatever)
```

IOBIT (4201h.Bit7) - TX Data 1st bit should be output immediately after strobing 4016h.Output, 2nd..9th bit should be output immediately after reading 1st..8th data/status bits from 4017h.

```text
  1st          TX Data Present (0=Low=Yes, 1=HighZ=None)
  2nd          TX Data Bit7    (0=Low=Zero, 1=HighZ=One) ;\
  3rd          TX Data Bit6    (0=Low=Zero, 1=HighZ=One) ; should be DATA
  4th          TX Data Bit5    (0=Low=Zero, 1=HighZ=One) ; when Data Present,
  5th          TX Data Bit4    (0=Low=Zero, 1=HighZ=One) ; or otherwise,
  6th          TX Data Bit3    (0=Low=Zero, 1=HighZ=One) ; should be FFh,
  7th          TX Data Bit2    (0=Low=Zero, 1=HighZ=One) ; or "R" or "C" ?
  8th          TX Data Bit1    (0=Low=Zero, 1=HighZ=One) ; (RTS/CTS or so?)
  9th          TX Data Bit0    (0=Low=Zero, 1=HighZ=One) ;/
  10th and up  Should be "1"   (1=HighZ)
```
