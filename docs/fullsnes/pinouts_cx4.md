# SNES Pinouts CX4 Chip

#### Capcom CX4 (used in Mega Man X2/X3)

```text
  1  A3     21 A15    41 RA8    61 /IRQ
  2  A4     22 A14    42 RA7    62 D7
  3  A5     23 A13    43 RA6    63 D6
  4  A6     24 A12    44 RA5    64 D5
  5  A7     25 /SRAM  45 RA4    65 D4
  6  A8     26 /ROM2  46 RA3    66 Vcc
  7  A9     27 /ROM1  47 RA2    67 D3
  8  A10    28 RA19   48 RA1    68 D2
  9  A11    29 RA18   49 RA0    69 D1
  10 GND    30 RA17   50 GND    70 D0
  11 XIN    31 Vcc    51 /RWE   71 Vcc
  12 XOUT   32 RA16   52 /ROE   72 /RST
  13 A23    33 RA15   53 RD7    73 GND
  14 A22    34 RA20   54 RD6    74 GNDed
  15 A21    35 RA14   55 RD5    75 GNDed
  16 A20    36 RA13   56 RD4    76 /RD
  17 A19    37 RA12   57 RD3    77 /WR
  18 A18    38 RA11   58 RD2    78 A0
  19 A17    39 RA10   59 RD1    79 A1
  20 A16    40 RA9    60 RD0    80 A2
```

SNES bus (cartridge slot) connects to Pin 1-24 and 61-80, CX4 bus (ROM/SRAM) to pin 25-60. Pin 74 and 75 are GNDed (but not interconnected to GND inside of the chip); of these, Pin 75 can be reconfigured on some PCBs (via CL and R4 options); maybe one of the pins is for HiROM mapping.
