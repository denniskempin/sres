# SNES Pinouts CPU Chip

#### CPU 5A22

```text
  1     In  VCC (supply)
  2-17  Out A8..A23
  18    In  GND (supply)
  19-26 I/O JOY-IO-0..7 (Port 4201h/4213h.Bit0..7)
  27-31 In  JOY-2 (4017h.Read.Bit0..4) (Pin 29-31 wired to GND)
  32-33 In  JOY-1 (4016h.Read.Bit0..1)
  34    In   "VCC" (unknown, this is NOT 4016h.Bit2) (wired to VCC)
  35    Out JOY-1-CLK (strobed on 4016h.Read)
  36    Out JOY-2-CLK (strobed on 4017h.Read)
  37-39 Out JOY-OUT0..2 (4016h.Write.Bit0-2, OUT0=JOY-STROBE,OUT1-OUT2=UNUSED?)
  40    Out REFRESH (DRAM refresh for WRAM, four HIGH pulses per scanline)
  41-42 In  TCKSEL0,TCKSEL1 (wired to GND) (purpose unknown)
  43-44 In  HBLANK,VBLANK (from PPU, for H/V-timers and V-Blank NMI)
  45    In  /NMI (wired to VCC)
  46    In  /IRQ (wired to Cartridge and Expansion Port)
  47    In  GND (supply)
  48    In  MCK ;21.47727 MHz master clock ;measured ca.21.666666MHz? low volts
  49    In  /DRAMMODE (wired to GND) (allows to disable DRAM refresh)
  50    In  /RESET
  51-58 Out PA0..PA7
  59    In  VCC (supply)
  60-67 I/O D0-D7
  68    Out /PARD
  69    Out /PAWR
  70    Out /DMA (NC)    HI
  71    Out CPUCK (NC)      LOOKS SAME AS PIN72 (MAYBE PHASE-SHIFTED?)
  72    Out SYSCK (to WRAM and Cartridge) FAST CLK... TYPICALLY 2xHI, 1xLO
  73    In  TM (wired to GND) (purpose unknown)
  74    In  HVCMODE (wired to GND) (purpose unknown)
  75    In  HALT (wired to GND) (purpose unknown) (related to RDY?)
  76    In  /ABORT (wired to VCC)
  77    Out /ROMSEL (access to 00-3F/80-BF:8000-FFFF or 40-7D/C0-FF:0000-FFFF)
  78    Out /WRAMSEL (access to 00-3F/80-BF:0000-1FFF or 7E-7F:0000-FFFF)
  79    In  GND (supply)
  80    Out R/W (NC) (ie. almost same as /WR, but with longer LOW-duty)
  81    In  RDY (wired to VCC) (schematic says "PE" or RE" or so?)
  82    Out /ML (NC) (memory lock,low on read-modify,ie.inc/dec/shift/etc)
  83    Out MF (NC) (CPU's M-Flag, 8bit/16bit mode)
  84    Out XF (NC) (CPU's X-Flag, 8bit/16bit mode)
  85    In  VCC (supply)
  86    Out VFB or VPB or so (NC)    RAPID PULSED
  87    Out VFA or VPA or so (NC)    RAPID PULSED
  88    Out ALCK                     LOOKS LIKE INVERSE OF PIN71 or PIN72
  89    Out /VP (NC) (vector pull, low when executing exception vector)
  90    In  GND
  91    Out /WR (low on any memory write, including io-writes to 21xxh/4xxxh)
  92    Out /RD
```

#### 93-100 Out  A0..A7

The three VCC pins are interconnected inside of the chip (verified).

The various GND pins are not verified (some may be supply, or inputs).
