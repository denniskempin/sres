# SNES Pinouts SA1 Chip

#### SA-1

```text
  1 SNES./IRQ
  2 SNES.D7
  3 SNES.D3
  4 SNES.D6
  5 SNES.D2
  6 SNES.D5
  7 SNES.D1
  8 SNES.D4
  9 SNES.D0
  10 VCC
  11 GND
  12 SNES.A23
  13 SNES.A0
  14 SNES.A22
  15 SNES.A1
  16 SNES.A21
  17 SNES.A2
  18 SNES.A20
  19 SNES.A3
  20 SNES.A19
  21 SNES.A4
  22 SNES.A18
  23 SNES.A5
  24 SNES.A17
  25 SNES.A6
  26 SNES.A16
  27 SNES.A7
  28 SNES.A15
  29 SNES.A8
  30 SNES.A14
  31 SNES.A9
  32 SNES.A13
  33 SNES.A10
  34 SNES.A12
  35 SNES.A11
  36 VCC
  37 GND
  38 REFRESH
 ---
  39 GND
  40 X.?     MasterClock (21.477MHz)
  41 X.?     MasterClock (21.477MHz)
  42 GND
  43 ROM.D15 pin31 (D15/A0)
  44 ROM.D7  pin30
  45 ROM.D14 pin29
  46 ROM.D6  pin28
  47 ROM.D11 pin22
  48 ROM.D3  pin21
  49 ROM.D10 pin20
  50 ROM.D2  pin19
  51 ROM.D13 pin27
  52 ROM.D5  pin26
  53 ROM.D12 pin25
  54 ROM.D4  pin24
  55 ROM.D9  pin18
  56 ROM.D1  pin17
  57 ROM.D8  pin16
  58 ROM.D0  pin15
  59 ROM.A1  pin11
  60 ROM.A2  pin10
  61 ROM.A3  pin9
  62 ROM.A4  pin8
  63 ROM.A5  pin7
  64 ROM.A6  pin6
 ---
  65 ROM.A7  pin5
  66 ROM.A8  pin4
  67 ROM.A9  pin42
  68 ROM.A10 pin41
  69 ROM.A11 pin40
  70 ROM.A12 pin39
  71 ROM.A13 pin38
  72 ROM.A14 pin37
  73 ROM.A15 pin36
  74 ROM.A16 pin35
  75 ROM.A17 pin34
  76 ROM.A19 pin2
  77 ROM.A18 pin3
  78 ROM.A20 pin43
  79 ROM.A21 pin44
  80 ROM.A22 pin1
  81           maybe A23 ?
  82 GND?
  83 VCC
  84 GND
  85 GND?
  86 SRAM. A16?  pin1-1 (extra pin)
  87 SRAM. A14   pin1
  88 SRAM. A12   pin2
  89 SRAM.A7  pin3
  90 SRAM.A6  pin4
  91 SRAM.A5  pin5
  92 SRAM.A4  pin6
  93 SRAM.A3  pin7
  94 SRAM.A2  pin8
  95 SRAM.A1  pin9
  96 SRAM.A0  pin10
  97 SRAM. A10   pin21
  98 SRAM. A11   pin23
  99 SRAM. A9    pin24
  100 GND
  101 VCC
  102 SRAM. A8   pin25
 ---
  103 to left-solder pads (U4.3.CS) (aka SRAM.A13)
  104 SRAM. A18?? pin1-2 (extra pin)
  105 SRAM. A15  pin28+1 (extra pin)
  106
  107
  108 SRAM. /OE  pin22
  109 SRAM. /WE  pin27
  110 SRAM.D0 pin11
  111 SRAM.D1 pin12
  112 SRAM.D2 pin13
  113 SRAM.D3 pin15
  114 SRAM.D4 pin16
  115 SRAM.D5 pin17
  116 SRAM.D6 pin18
  117 SRAM.D7 pin19
  118 GND
  119 VCC
  120 SNES./RESET
  121 SNES.SYSCK
  122 SNES.CIC3 (3.072MHz)
  123 SNES.CIC2
  124 SNES.CIC1
  125 SNES.CIC0
  126 SNES./WR
  127 PAL/NTSC (GND=NTSC, VCC=PAL) (for CIC mode and/or HV-timer?)
  128 SNES./RD
```

ROM-Chip Note: ROM./CE and ROM./OE are wired to GND (always enabled).

ROM-Chip Note: ROM.BHE is wired to VCC (always 16bit databus mode).

```text
  U4.Pin5./CS ---> SRAM./CS   pin20  (U4:6129A aka PCB:MM1026AF)
  U4.Pin3.CS  ---> SRAM.A13?  pin26
  (left 4 solder-pads near U4 --> SRAM.pin26 = CS or A14)
  (right 4 solder-pads near U4 --> SRAM.pin28 = CS or Vbat)
```

#### Cart Slot Unused: /ROMSEL

#### Cart Slot Used: SHIELD (!)
