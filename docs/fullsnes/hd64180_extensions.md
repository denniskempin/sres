# HD64180 Extensions

Port 3Eh on Z8x180 only (not HD64180).

Port 12h..13h,1Ah..1Fh,2Dh on Z8S180/Z8L180 only (not Z80180/HD64180).

3Eh - OMCR - Operation Mode Control - Z180 only (not HD64180) (FFh on Reset)

```text
  7   M1E   /M1 Enable           (0=Z180, 1=HD64180; Problems with RETI)  (R/W)
  6   /M1TE /M1 Temporary Enable (0=Z180, 1=HD64180; Problems with Z80PIO)  (W)
  5   /IOC  I/O Compatibility    (0=Z180, 1=HD64180; Delayed falling /WR) (R/W)
  4-0 -     Unused (should be all-ones)
```

Allows to fix some signal & timing glitches of the HD64180 (or to maintain them for compatibility with HD64180 based designs).

12h - ASEXT0 - ASCI Channel 0 Extension Control Reg 0 (00h on Reset) 13h - ASEXT1 - ASCI Channel 1 Extension Control Reg 1 (00h on Reset)

```text
  7   RDRF Interrupt Inhibit
  6   DCD0 Disable    ;\ASCI Channel 0 only (not Channel 1)
  5   CTS0 Disable    ;/
  4   X1 Bit Clk ASCI
  3   BRG Mode (Time Constant based Baud Rate Generator)
  2   Break Feature Enable
  1   Break Detect (RO)
  0   Send Break
```

1Ah - ASTC0L - ASCI Channel 0 Time Constant, Bit0-7 (00h on Reset) 1Bh - ASTC0H - ASCI Channel 0 Time Constant, Bit8-15 (00h on Reset) 1Ch - ASTC1L - ASCI Channel 1 Time Constant, Bit0-7 (00h on Reset) 1Dh - ASTC1H - ASCI Channel 1 Time Constant, Bit8-15 (00h on Reset) 16bit Time Constants (see BRG bit in ASEXT0/ASEXT1).

1Eh - CMR - Clock Multiplier Register (7Fh or so on Reset)

```text
  7   X2  Enable X2 Clock Multiplier Mode (0=Disable, 1=Enable)
  6-0 -   Unused (should be all-ones) (or so)
```

Purpose undocumented, maybe doubles the CPU speed (and/or Timer or ASCI or whatever speeds).

#### 1Fh - CCR - CPU Control Register (00h on Reset)

```text
  7  Clock Divide (0=XTAL/2, 1=XTAL/1)
  6  Standby/Idle Mode, Bit1
  5  BREXT     (0=Ignore BUSREQ in Standby/Idle, Exit Standby/Idle on BUSREQ)
  4  LNPHI     (0=Standard Drive, 1=33% Drive on EXTPHI Clock)
  3  Standby/Idle Mode, Bit0
  2  LNIO      (0=Standard Drive, 1=33% Drive on certain external I/O)
  1  LNCPUCTL  (0=Standard Drive, 1=33% Drive on CPU control signals)
  0  LNAD/DATA (0=Standard Drive, 1=33% Drive on A10-A0, D7-D0)
```

Standby/Idle Mode is combined of CCR bit6/bit3 (0=No Standby, 1=Idle after Sleep, 2=Standby after Sleep, 3=Standby after Sleep with 64 Cycle Exit Quick Recovery).

2Dh - IAR1B - DMA "I/O Address Ch 1" (Absurde name) (00h on Reset)

```text
  7   Alternating Channels
  6   Currently selected DMA channel when Bit7=1
  5-4 Unused (should be zero) (must be 0)
  3   TOUT/DREQ-Pin (0=DREQ Input, 1=TOUT Output)
  2-0 DMA Channel 1 (0=TOUT/DREQ, 1=ASCI0, 2=ASCI1, 3=ESCC, 7=PIA)
```
