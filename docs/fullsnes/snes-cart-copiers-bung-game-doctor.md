# SNES Cart Copiers - Bung (Game Doctor)

#### Game Doctor SF7

#### Memory Map (in BIOS mode)

```text
  00:8000-807F     I/O Ports
  00:8080-FFFF     BIOS ROM (1st 32kBytes)
  01:8000-FFFF     BIOS ROM (2nd 32kBytes) (if any)
  02:8000-FFFF     unused
  03:8000-FFFF     unused
  04:8000-FFFF     SRAM for game positions (32Kbyte)
  05:8000-FFFF     SRAM for real time save data (4kByte)
  06:8000-FFFF     SRAM for copier settings (4kByte)
  07:8000-FFFF     DRAM for ROM-image (32Kbyte page, selected via Port 8030h)
  08-7D:8000-FFFF  Mirror of above banks 00-07
  80-FF:8000-FFFF  Mirror of above banks 00-07 or Cartridge banks 00-7F/80-FF

  FFBFh compared to FFh ?
```

#### I/O Ports (in BIOS mode) in bank 00h

```text
  8000h-800Fh RW 512Kbyte DRAM chunk, mapped to upper 32Kbyte of Bank 0xh-Fxh
  8010h-8013h RW 512Kbyte DRAM chunk, mapped to lower 32Kbyte of Bank 4xh-7xh
  8014h-8017h RW 512Kbyte DRAM chunk, mapped to lower 32Kbyte of Bank Cxh-Fxh
  8018h-8019h W  SRAM Flags (bit0-15=Enable SRAM at 6000-7000 in banks 0xh-Fxh)
  8018h       R  bit1 = realtime.$4016.bit0, read bit7 = ? , bit = ?
  8018h.R  Flags (bit7/6 FDC IRQ?, and more)
  8019h       R  bit1 = ?
  801Ah       R  realtime.word, latch settings for double write word registers
  801Ah       W  write ?
  801Bh       W  write ?
  801Dh       W  BIOS mode mapping: changes what is mapped into banks $80-$FF
                   only bit0-bit1 seem to matter
                   0 = use cartridge banks $00-$7F
                   1 = use cartridge banks $80-$FF
                   2 = mirror banks $00-$7F (BIOS regs and all?)
                   3 = mirror banks $00-$7F (BIOS regs and all?)
  801Eh write ?
  __Floppy Disc__
  8020h       R  FDC Main Status
  8021h       RW FDC Command/Data
  8022h       W  FDC Transfer Rate/Density (?) (set to 00h,01h)
  8023h       -  FDC Unused
  8024h       W  FDC Motor Control (set to 00h,08h,0Ch,1Ch,2Dh)
  8025h-8027h -  FDC Unused
  8028h       W  set to same value (ANY VALUE?) as 8022/8029)
  8029h       W  set to same value (ANY VALUE?) as 8029)
  802Ah       W  set to 01-then-00 (once) (thereafter do sth to 8022)
  802Bh       W  set to 01h during FDC COMMAND-BYTEs (else to 00h) (maybe LED?)
  __Parallel Port__
  802Ch       RW Parallel Port Data Lines
  802Dh       RW Parallel Port Status Lines
  802Eh       RW Parallel Port Control Lines
  802Fh       W  Parallel Port? Unknown (set to 00h,01h) (data direction?)
  802Fh       R  Parallel Port? Unused (reads same as $00802D)
  __Memory__
  8030h-8031h W  Select 32Kbyte-DRAM-Page (0000h..01FFh) mapped to 078000h
  8030h-803Dh R  this is a 7 word table?? (gotten from code at 80/AE80)
  8040h-805Fh R  read same as 802Dh (uh, but, some are used for sth else?)
  8040h       R  used, parallel port related (or other mainboard version?)
  8043h       W  used, parallel port related (or other mainboard version?)
  8060h-807Fh R  read = FFh
  80xFh          any access to 0080xFh (x=8..F) switches to cartridge mode

802Dh - Parallel Port status (not direct pin reading?)
```

#### read

```text
  bit0 = /C1 (direct pin14, /AutoLF) (/Ctrl.Bit1 on PC side)
  bit1 = C2  (direct pin16, /INIT)   (Ctrl.Bit2 on PC side)
  bit2 = /C3 (direct pin17, /Select) (/Ctrl.Bit3 on PC side)
  bit3 = "write bit3"
  bit4 = "write bit4"
  bit5 = "write bit4" (uh, not bit5 here?)
  bit6 = "write bit4" (uh, not bit6 here?)
  bit7 = /S7 (direct pin11) = "write bit7" AND not "write bit0"
```

#### write

```text
  bit0 Enable/Disable Busy bit (0=Enable, 1=Disable) (?)
  bit3 => S3 (direct pin15, /ERR) (Stat.bit3 on PC side)
  bit4 => S4 (direct pin13, SLCT) (Stat.bit4 on PC side)
  bit5 => S5 (direct pin12, PE)   (Stat.bit5 on PC side)
  bit6 => S6 (direct pin10, /ACK) (Stat.bit6 on PC side)
  bit7 ...   (direct pin11, BUSY  (/Stat.bit7 on PC side) (ANDed with /bit0?)
802Eh - Parallel Port control (not direct control reg values)
```

often write 12h-then-10h  read/write?

```text
  bit0 = /C1 (direct pin14, /AutoLF) (/Ctrl.Bit1 on PC side)   W
  bit1 = C2  (direct pin16, /INIT)   (Ctrl.Bit2 on PC side)    W
  bit2 = /C3 (direct pin17, /Select) (/Ctrl.Bit3 on PC side)   W
  bit3-bit6, read = bit3-bit6 of $00802D                       W
  bit7 = /C0 (direct pin1,  /STB) (/Ctrl.bit0 on PC side)      R
```

#### Component List - Bung Game Doctor SF6 (Board CT401)

```text
  U    3pin 7805 or so
  U   40pin GoldStar xxx (=probably GM82C765B) (floppy disc controller)
  U  ???pin huge chip (200 pins or so)
  U   18pin 265111
  U   20pin 74LS744 or so (not installed)
  U   28pin SRAM or so
  U   28pin SRAM or so
  U   28pin EPROM (GDSF_6.0)
  U   14pin whatever/modded chip (wired top-down near EPROM)
  P   40pin to DRAM daughterboard 1 (2x10 male pins, 2x10 female pins)
  P   40pin to DRAM daughterboard 2 (2x10 male pins, 2x10 female pins)
  P   62pin cartridge port
  P   62pin cartridge port (on PCB back side)
  P   25pin DB-25 parallel port (on PCB back side)
  P    2pin power supply (on PCB back side)
  P    2pin floppy supply (on PCB back side)
  P   34pin floppy data
  X    2pin oscillator
```
