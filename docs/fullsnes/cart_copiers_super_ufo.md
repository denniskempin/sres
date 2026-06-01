# SNES Cart Copiers - Super UFO

#### UFO Super Drive Pro / Super UFO

#### UFO3

The UFO-3 is a Front Fareast clone. BIOS is 8Kbytes mapped to E000h-FFFFh, FDC Registers are at C000h-C007h, Parallel Port at C008h-C009h, Memory Control at E000h-E00Dh. For details see Front Fareast chapter.

#### UFO6

I/O ports for this version are unknown.

#### UFO7/UFO8

```text
  2184.W   ... set to 00h/0Ch/0Fh
  2185.W   ... set to 00h/0Fh
  2186.W   ... set to 00h/0Fh
  2187.W   ... set to 08h/00h/0Bh
  2188.W   ... set to 00h..0Fh or so
  2189.W   ... set to 0Fh/0Eh
  218A.W   ... set to 00h
  218B.W   ... set to 0Ah/0Fh
  218C.R   FDC Main Status Register
  218D.RW  FDC Command/Data Register (emit 03h,DFh,03h = spd/dma)(then 07h,01h)
  218E.W   FDC Motor Control (set to 00h, 29h-then-2Dh on disc access)
  218F.W   FDC Transfer Rate
  218F.R   FDC Flags (bit7=irq?,bit6=index?) (UFO8: bit5=?)
  003F68.R         warmboot flag? if A581 --> JMP 3D00
  003FD0..3FFF   cartridge header? (or copy of it?)
  003C00..003FFF SRAM 1Kbyte (BIOS settings, I/O logging?, last 32-byte OAM)
  013C00..013FFF SRAM 1Kbyte (512-byte Palette and 1st 512-byte OAM)
  008000h and up BIOS 64Kbytes (UFO7) or more 128K..256K (UFO8)
  708000h and up SRAM 32Kbytes (for game positions)
  808000h and up DRAM (variable size detected) (via calls to 9025)
```

ufo7 rom chksum calculated at 9505 (32K ROM at 8000-FFFF must sum up to 00h) ufo8 (and maybe ufo7 too) should have 8K SRAM (ie. MORE than above 2x1K...?)

UFO6 Component List - UFO Super Drive Pro (with Pro-6 BIOS)

```text
  U    3pin 7805 or so
  U    ?pin xxxx (near 7805)
  U   40pin GoldStar GM82C765B
  U   14pin xxxx
  U   20pin L GALxxxx
  U   20pin L GALxxxx
  U   20pin L GALxxxx
  U   20pin AMI xxxxx
  U   20pin L GALxxxx
  U   20pin L GALxxxx
  U   20pin L GALxxxx
  U   20pin L GALxxxx
  U   20pin LS245 (not installed, near DB-25) (8-bit 3-state transceiver)
  U   14pin HC74  (not installed, near DB-25) (dual flip-flop)
  U   16pin xxxx  (installed, near DB-25)
  U   28pin EPROM
  U   28pin Winbond W24256-10L (SRAM 32Kx8)
  U   20pin Philips PC74xxxx
  U   16pin 74LS112 (reportedly a cloned/mislabelled CIC chip)
  X    2pin oscillator (near 7805)
  BT   2pin 3V or so
  P   25pin DB-25 parallel port or so
  P    2pin power supply
  P    2pin floppy supply
  P   34pin floppy data
  P   40pin to DRAM daughterboard
  P   46pin cartridge slot (only 46 soldering points)
  P   62pin cartridge edge (has 62 soldering points, but only 46 connected?)
```

UFO8 Component List Super UFO Super Drive PRO8 (REV 7.8 2)

```text
  1x 84pin Altera EPMxxxxxxx84-15
  1x 40pin GoldStar GM82C765B (DIL) (floppy disc controller)
  1x 32pin BIOS ROM/EPROM (located on PCB solder side)
  1x 28pin UM62256D-70L (SRAM 32Kx8)
  1x 28pin UT6264PC-70LL (SRAM 8Kx8)
  1x 28pin DSP chip (not installed)
  2x 24pin NN5117405BJ-60 (DRAM, two pieces, located on daughterboard)
  1x 16pin D1
  1x 14pin FT4066
  1x 14pin DSP_74HC74 (not installed) (dual flip-flop)
  1x 14pin 74LS00 or so
  1x 14pin whatever (near oscillator)
  1x 14pin 74LSxxx whatever (near PAL/NTSC jumpers)
  2x 16pin SN74HC157N (decoder/demultiplexer)
  1x 3pin  7805 or so
  1x 34pin connector/cable to internal disc drive
  2x 62pin cartridge connectors (one male, one female)
  1x 2pin  wire (supply to internal disc drive)
  1x 2pin  connector (external power supply input)
  no battery, no parallel port
```
