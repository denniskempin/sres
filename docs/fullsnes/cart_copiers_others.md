# SNES Cart Copiers - Others

Component List - Board "GP-003 REV. B" (used in Special Partner)

```text
  U   14pin  GD74HC04 (hex inverters)
  U   16pin  LR74HC158 (decoder/demultiplexer)
  U   16pin  LR74HC158 (decoder/demultiplexer)
  U   16pin  GD74HC138 (decoder/demultiplexer)
  U   16pin  GD74HC138 (decoder/demultiplexer)
  U   20pin  PALCE16V8H-25
  U   28pin  K-105                                               ;DSP clone?
  U   28pin  EPROM (28pin 27C512 64Kx8 installed, optionally 32pin possible)
  U   28pin  NEC D43256BGU-70L (uPD43256BGU) (SRAM 32Kx8)
  U   28pin  NEC 4364C-20L (SRAM 8Kx8)
  U   44pin  GoldStar GM82C765B PL (SMD)
  U   44pin  Lattice ispLSI 1016-60LJ B501B06 (SMD)
  U    3pin  7805 or so
  J2  62pin  to cartridge edge (snes)
  J   62pin  cartridge slot (snes game cartridge)
  J   62pin  cartridge slot (an expansion slot, not for any game carts)
  J   40pin  to DRAM daughterboard
  J   34pin  to internal floppy drive
  J2   2pin  floppy power supply
  J    2pin  external power supply
  J6   2pin  jumper (near 34pin floppy cable)
  J8   3pin  jumper (near EPROM; maybe ROM size select?)
  X    2pin  oscillator (160)
  X    2pin  oscillator (?)                                      ;DSP clock?
  BT   2pin  NiCd 3.6V
```

The board doesn't contain a CIC-clone (unless it's 'hidden' in one of the chips).

Components - Supercom, 24m DSP, CD-ROM, FX-32, High Density, Real Time Save

```text
  U1  40pin GoldStar GM82C765B (floppy disc controller)
  U2  20pin 16V8 (not installed)
  U3? 20pin PALCE16V8H-25PC/4
  U4  24pin PALCE20V8H-25PC/4
  U5  28pin not installed (probably for DSP clone)
  U6? 14pin xxx (below U5)
  U7? 20pin LS245 (not installed) (8-bit 3-state transceiver)
  U8? 24pin PALCE20V8H-25PC/4
  U9  20pin 74HC273 (8bit latch with reset)
  U10 28pin 27C256G-20 (EPROM 32Kx8) (boots as "FX-32 CD-ROM & DSP, 1994 H.K.")
  U11 28pin ST MK4864 (SRAM 8Kx8)
  U12 28pin xxx (SRAM ?Kx8)
  U13 20pin xxx
  U14 20pin xxx
  U15 20pin xxx
  U16 20pin xxx
  U17 20pin xxx
  U18 20pin xxx
  U19  ?pin Toshiba xxx (16pin chip, mounted in a 20pin socket)
  U20 16pin ST101xxx
  Q1   3pin 7805
  Y1   2pin oscillator
  Y2   2pin oscillator (not installed, probably for DSP chip)
  J?  34pin floppy data
  J?   2pin floppy power
  J?   2pin power supply
  J3? 25pin DB-25 (parallel port and/or external CD-ROM drive?)
  J4  62pin cartridge edge
  J5  62pin cartridge slot
  J6  40pin to DRAM daughterboard
```

#### Component List - Double Pro Fighter (CCL) (1994)

```text
  U1  28pin  Hyundai HY62256ALP-10 (SRAM 32Kx8)
  U2  28pin  AM27C512-205DC (EPROM 64Kx8)
  U3  N/A    N/A
  U4  N/A    N/A
  U5  20pin  HD74LS245P (8-bit 3-state transceiver)
  U6  20pin  HD74LS245P (8-bit 3-state transceiver)
  U7  20pin  HD74LS245P (8-bit 3-state transceiver)
  U8  24pin  GoldStar GM76C28A-10 (SRAM 2Kx8)
  U9  16pin  noname-chip-without-part-number (or, marked 10198 on other boards)
  U10  3pin  AN7805 (voltage regulator)
  U11 14pin  HD74HC00P (quad 2-input NAND gates)
  U12 40pin  Goldstar GM82C765B (floppy disc controller)
  U13 68pin  Altera EP1810LC-45 D9407
  U14 16pin  74HC139 (decoder/demultiplexer)
  U15 24pin  PALCE20V8H
  U16 20pin  GAL16V8xxx
  U17 20pin  PALCxxx
  U18 16pin  74HC139 (decoder/demultiplexer)
  Y1   2pin  16.00 TDX (16 MHz oscillator)
  J1   2pin  power supply input
  J2   2pin  power supply connector (alternately to J1 or so, not installed)
  P4  50pin  ro dram daughterboard ?
  SL1  64pin connector for remove-able snes-or-sega? cartridge edge
  SL2  64pin connector for remove-able sega-or-snes? cartridge edge
  SL3  62pin cartridge slot (snes)
  SL4  64pin cartridge slot (sega genesis)
  ?     2pin connector for disc drive (supply)
  ?    34pin connector for disc drive (data)
  DRAM Daughterboard
  -    40pin connector (to 40pins of the 50pin socket on Double Pro Fighter)
  -    20pin NEC 424400-80 (EIGHT pieces)
  Optional Parallel Port (plugged into SL3-socket, ie. into SNES slot):
  U1   20pin PALCE16V8H-25PC/4
  U2   20pin HD74HC245P (8-bit 3-state transceiver) (no latch here ???)
  P1   25pin DB-25 parallel port connector
  -    62pin cartridge edge (to be plugged into SL3 of Double Pro Fighter)
```

Component List - Super Smart Disc (same as Pro Fighter X?)

```text
  U   16pin 10198
  U   28pin GRAPHIC DSP1-1  (or is it "DCP1-1" or so?)
  U   28pin STxxxx (SRAM, ?x8)
  U   28pin xxxxxx (SRAM, ?x8)
  U   28pin EPROM (28pin chip mounted in 32pin socket)
  U   14pin xxxx
  U   40pin ICT PA7140T CTM42027JC
  U   40pin ICT PA7140T CTM42027JC
  U   40pin GoldStar GM82C765B
  U   24pin xxxxx (PAL or so)
  U    3pin 7805 or so
  X    2pin oscillator
  P1  64pin cartridge edge (via remove-able adaptor) (snes)
  P   62pin cartridge slot (snes)
  P   32pin cartridge slot (gameboy)
  P   34pin floppy data
  P    2pin power supply
  P    2pin floppy supply
  P   50pin to DRAM daughterboard
```
