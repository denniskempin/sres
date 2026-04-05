# SNES Pinouts APU Chips

#### S-DSP Pinouts (Sound Chip)

```text
  1     CK     DKD (NC)              (5 clks = 1.2us) 4.096MHz (24.576MHz/6)
  2     CK     MXK or MYX or so (NC) (5 clks = 1.6us) 3.072MHz (24.576MHz/8)
  3-5   CK     MX1-MX3 (NC) 1.024MHz (24.576MHz/24) (3pins: phase/duty shifted)
  6-8   SRAM   MD2-MD0 (SRAM Data)
  9-11  SRAM   MA0-MA2 (SRAM Address)
  12    Supply GND
  13-19 SRAM   MA3-MA7,MA12,MA14 (SRAM Address)
  20    SRAM   MA15 (NC) (instead, upper/lower 32K selected via /CE1 and /CE0)
  21    ?      DIP (NC) (always high?)
  22-32 SRAM   MD3,MD4,MD5,MD6,MD7,/CE1,/CE0,MA10,/OE,MA11,MA9
  33    Supply VCC
  34-36 SRAM   MA8,MA13,/WE
  37    ?      TF (GND)    ;\wiring TF and/or TK to VCC crashes the SPC700
  38    ?      TK (GND)    ;/(ie. they seem to mess up CPUK clock or SRAM bus)
  39    Audio  /MUTE (to/after amplifier)
  40    CK     MCK (NC) 64000Hz  (24.576MHz/24/16)
  41    CIC    SCLK     3.072MHz (24.576MHz/8) (via inverters to "CIC" chips)
  42    Audio  BCK      1.536MHz (24.576/16)       BitClk    ;\to uPD6376
  43    Audio  LRCK     32000Hz  (24.576MHz/16/48) StereoClk ; D/A converter
  44    Audio  DATA Data Bits (8xZeroPadding+16xData)        ;/
  45-46 Osc    XTAO,XTAI (24.576MHz)
  47    System /RESET
  48    SPC700 CPUK     2.048MHz (24.576MHz/12) (to S-SMP)
  49    SPC700 PD2 (on boot: always high?)
  50    SPC700 PD3 (on boot: always low?)
  51    SPC700 D0
  52    Supply GND
  53-59 SPC700 D1-D7
  60-72 SPC700 A0-A12
  73    Supply VCC
  74-76 SPC700 A13-A15
  77    CK     XCK 24.576MHz (24.576MHz/1) (NC)
  78    Exp.   DCK  8.192MHz (24.576MHz/3) (to Expansion Port Pin 21, SMPCLK)
  79    CK     CK1 12.288MHz (24.576MHz/2) (NC)
  80    CK     CK2  6.144MHz (24.576MHz/4) (NC)
```

#### S-SMP Pinouts (SPC700 CPU)

```text
  1-5   DSP    A4..A0 Address Bus
  6-13  DSP    D7..D0 Data Bus
  14    DSP    PD3 (maybe R/W signal or RAM/DSP select?)
  15    DSP    PD2 (maybe R/W signal or RAM/DSP select?)
  16    DSP    CPUK (2.048MHz from DSP chip)
  17    AUX    /P5RD (NC)
  18-25 AUX    P57..P50 (NC)
  26    Supply GND
  27-34 AUX    P47..P40 (NC)
  35    ?      T1 (NC or wired to VCC) (maybe test or timer?)
  36    ?      T0 (NC or wired to VCC) (maybe test or timer?)
  37    System /RESET
  38-45 CPU    D7..D0 Data Bus
  46-51 CPU    /PARD,/PAWR,PA1,PA0,PA6 (aka CS), PA7 (aka /CS) B-Address Bus
  52-56 DSP    A15-A11 Address Bus
  57    Supply VCC (5V)
  58    Supply GND
  59-64 DSP    A10-A5 Address Bus
```

Pin 1-16 and 52-64 to S-DSP chip, Pin 17-34 Aux (not connected), Pin35-36 are NC on real hardware (but are wired to VCC in schematic), pin 37-51 to main CPU.

CPUK caution:

scope measure with "x10" ref (gives the correct signal):

```text
   -_-_-_-_-_-_-_-_ 2.048MHz (0.5us per cycle)
```

during (and AFTER) "x1" ref (this seems to "crash" the clock generator):

```text
   ---_---_---_---_ 1.024MHz (1.0us per cycle) (with triple-high duty)
```

S-APU Pinouts (S-SMP, S-DSP, 64Kx8 Sound RAM) This 100pin chip is used in newer cost-down SNES consoles.

```text
  1-100 Unknown
```

It combines the S-SMP, S-DSP, and the two 32Kx8 SRAMs in one chip. And, possibly also the NEC uPD6376 D/A converter?
