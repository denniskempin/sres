# SNES Cart SA-1 Interrupt/Control on SNES Side

#### 2200h SNES CCNT - SA-1 CPU Control (W)

```text
  0-3 Message from SNES to SA-1 (4bit value)
  4   NMI from SNES to SA-1   (0=No Change?, 1=Interrupt)
  5   Reset from SNES to SA-1 (0=No Reset, 1=Reset)
  6   Wait from SNES to SA-1  (0=No Wait, 1=Wait)
  7   IRQ from SNES to SA-1   (0=No Change?, 1=Interrupt)
```

Unknown if Wait freezes the whole SA1 (CPU, plus Timer and DMA?).

Unknown if Reset resets any I/O Ports (such like DMA or interrupts) or if it does only reset the CPU?

#### 2201h SNES SIE - SNES CPU Int Enable (W)

```text
  0-4 Not used (should be 0)
  5   IRQ Enable (Character conversion DMA) (0=Disable, 1=Enable)
  6   Not used (should be 0)
  7   IRQ Enable (from SA-1) (0=Disable, 1=Enable)
```

#### 2202h SNES SIC - SNES CPU Int Clear (W)

```text
  0-4 Not used (should be 0)
  5   IRQ Acknowledge (Character conversion DMA) (0=No change, 1=Clear)
  6   Not used (should be 0)
  7   IRQ Acknowledge (from SA-1) (0=No change, 1=Clear)
```

2203h SNES CRV - SA-1 CPU Reset Vector Lsb (W) 2204h SNES CRV - SA-1 CPU Reset Vector Msb (W) 2205h SNES CNV - SA-1 CPU NMI Vector Lsb (W) 2206h SNES CNV - SA-1 CPU NMI Vector Msb (W) 2207h SNES CIV - SA-1 CPU IRQ Vector Lsb (W) 2208h SNES CIV - SA-1 CPU IRQ Vector Msb (W) Exception Vectors on SA-1 side (these are ALWAYS replacing the normal vectors in ROM).

#### 2300h SNES SFR - SNES CPU Flag Read (R)

```text
  0-3 Message from SA-1 to SNES (4bit value)          (same as 2209h.Bit0-3)
  4   NMI Vector for SNES (0=ROM FFExh, 1=Port 220Ch) (same as 2209h.Bit4)
  5   IRQ from Character Conversion DMA (0=None, 1=Interrupt) (ready-to-do-DMA)
  6   IRQ Vector for SNES (0=ROM FFExh, 1=Port 220Eh) (same as 2209h.Bit6)
  7   IRQ from SA-1 to SNES   (0=None, 1=Interrupt) (triggered by 2209h.Bit7)
```

Bit0-3,4,6 are same as in Port 2209h. Bit5 is set via ..DMA..? Bit7 is set via Port 2209h. Bit5,7 can be cleared via Port 2202h.

#### 230Eh SNES VC - Version Code Register (R)

```text
  0-7  SA-1 Chip Version
```

Existing value(s) are unknown. There seems to be only one chip version (labeled SA-1 RF5A123, used for both PAL and NTSC). The "VC" register isn't read by any games (except, accidently, by a bugged memcopy function at 059E92h in Derby Jockey 2).
