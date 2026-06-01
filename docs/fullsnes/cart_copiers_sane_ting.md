# SNES Cart Copiers - Sane Ting (Super Disk Interceptor)

The Super Disk Interceptor is a SNES copier from KL818 B.C./Sane Ting Co. Ltd., the company also made a copier for Mega Drive, called Mega Disk Interceptor.

#### I/O & Memory

```text
  8000-9FFF memory (SRAM/DRAM or so)
  A000.W  set to 00,03-then-01, or 40,80
  A001.W  set to 00,04 (as MSB of A000) or to 04,24,08
  A001.R  tests bit4,bit5
  A002.W  FDC Transfer Rate/Density (set to ([0B] XOR 1)*2)
  A003.W  FDC Motor Control (set to 08,0C,1C)
  A004    FDC Unused
  A005.RW FDC Command/Data
  A006.R  FDC Main Status
  A007    FDC Unused
  A008.W  set to [1802] (bit3,bit4 used)
  B000-B01F ...    I/O or RAM or RegisterFile workspace?
   B000.W  set to 00
   B000.R  checked if 00h
   B001.W  set to 00
   B002.W  set to xx OR 80h
   B002.R  read and ORed with 06h
   B003.W  set to xx OR 80h OR 03h
   B003.R  bit5 isolated, ORed with 04h, then written to A001h
   B004.W  set to 00 or 00..03h
   B004.R  whatever, if (N+1)=00..03 --> written to B004 and B005
   B005.W  set to 00
   B006.RW set to [4219h] = MSB of joypad1 (?)
   B00F.W  set to [FFDC]=00h or ([FFDC] XOR 1)=01h
   B00F.R  checked if 00h (if nonzero --> WRITE PROTECT)
   B01x    ...
  C000.R  dummy read within waitvblank
  C001.R  dummy read within waitvblank
  E000.W
  E001.W
  E002.W
  E000-FFFF BIOS (32Kbytes, in 8Kbyte units, in banks 00h-03h)
  704000
  708000
```

#### Component List - Super Disk Interceptor (version dated around 1992)

```text
  U1  40pin  GoldStar GM82C765B PL (DIP) (floppy disk controller)
  U2  84pin  MD1812 9211 (with socket)
  U3  28pin  2xxx4A-25 (PROM, presumably 8Kx8, non-eraseable)
  U4  20pin  not installed (DIP) (BANK2.3)
  U4A 20pin  not installed (DIP) (BANK2.3)
  U5  20pin  GoldStar GMxxxxx (SMD) (BANK0.1) (DRAM)
  U5A 20pin  GoldStar GMxxxxx (SMD) (BANK0.1) (DRAM)
  U6  28pin  Hyundai HY62256ALP-10 (SRAM, 32Kx8)
  U7  20pin  not installed (DIP) (BANK2.3)
  U7A 20pin  not installed (DIP) (BANK2.3)
  U8  20pin  GoldStar GMxxxxx (SMD) (BANK0.1) (DRAM)
  U8A 20pin  GoldStar GMxxxxx (SMD) (BANK0.1) (DRAM)
  U9  16pin  74HC157N (decoder/demultiplexer)
  U10 16pin  74HC157N (decoder/demultiplexer)
  U11 20pin  HY-xxxxxx-30
  U12 20pin  HY-xxxxxx-30
  XTAL 2pin  16.000MHz
  J   46pin  Cartridge edge (snes)
  J   46pin  Cartridge slot (snes)
  J   34pin  Floppy data
  J    2pin  Floppy supply
  BT  2pin   3.6V Battery
```

#### Component List - Super Disk Interceptor (version dated around 1993)

```text
  U   44pin  GoldStar GM82C765B PL (SMD) (floppy disk controller)
  U   28pin  27C64SDM (PROM, 8Kx8, non-eraseable)
  U   28pin  GoldStar GM76C256ALLFW70 (SRAM, 32Kx8)
  U   20pin  HD74HC373P (8-bit 3-state transparent latch)
  U   14pin  xxxx
  U   80pin  SD1812 349 (SMD, without socket)
  U   14pin  not installed
  U   28pin  KM48C2100J-7 (DRAM, 2Mx8)  ;\
  U   20pin  KM44C1000CJ-6 (DRAM, 1Mx4) ; all installed,
  U   20pin  KM44C1000CJ-6 (DRAM, 1Mx4) ; together = 4Mx8
  U   20pin  KM44C1000CJ-6 (DRAM, 1Mx4) ;
  U   20pin  KM44C1000CJ-6 (DRAM, 1Mx4) ;/
  X    2pin  16.000MHz
  X    2pin  16.257MHz
  J   46pin  Cartridge edge (snes)
  J   46pin  Cartridge slot (snes)
  J   34pin  not installed (alternate floppy connector?)
  J   34pin  Floppy data
  J    2pin  Floppy supply
  BT  2pin   3.6V Battery
```
