# HD64180 Direct Memory Access (DMA)

20h - SAR0L - DMA Channel 0 Source Address, Bit0-7 (Memory or I/O) 21h - SAR0H - DMA Channel 0 Source Address, Bit8-15 (Memory or I/O) 22h - SAR0B - DMA Channel 0 Source Address, Bit16-19 (Memory or DRQ) 23h - DAR0L - DMA Channel 0 Destination Address, Bit0-7 (Memory or I/O) 24h - DAR0H - DMA Channel 0 Destination Address, Bit8-15 (Memory or I/O) 25h - DAR0B - DMA Channel 0 Destination Address, Bit16-19 (Memory or DRQ) 26h - BCR0L - DMA Channel 0 Byte Count Register, Bit0-7 27h - BCR0H - DMA Channel 0 Byte Count Register, Bit8-15 DMA Channel 1 Source/Dest/Len. Direction can be Memory-to-Memory, Memory-to-I/O, I/O-to-Memory, or I/O-to-I/O, Memory-Address can be Fixed, Incrementing, or Decrementing, I/O-Address is Fixed (see DMODE Register).

For I/O transfers, Bit16-17 of SAR/DAR are selecting the DRQ type:

```text
  00h DRQ by /DREQ0-Pin (normal case)
  01h DRQ by ASCI Channel 0 (RDRF-Bit for Source, or TDRE-Bit for Dest)
  02h DRQ by ASCI Channel 1 (RDRF-Bit for Source, or TDRE-Bit for Dest)
  03h Reserved
```

Memory-to-Memory DMA clock can be selected in MMOD bit ("Burst" pauses CPU until transfer is completed, "Cycle Steal" keeps the CPU running at roughly half-speed during DMA).

28h - MAR1L - DMA Channel 1 Memory Address, Bit0-7 (Source or Dest) 29h - MAR1H - DMA Channel 1 Memory Address, Bit8-15 (Source or Dest) 2Ah - MAR1B - DMA Channel 1 Memory Address, Bit16-19 (Source or Dest) 2Bh - IAR1L - DMA Channel 1 I/O Address, Bit0-7 (Dest or Source) 2Ch - IAR1H - DMA Channel 1 I/O Address, Bit8-15 (Dest or Source) 2Eh - BCR1L - DMA Channel 1 Byte Count Register, Bit0-7 2Fh - BCR1H - DMA Channel 1 Byte Count Register, Bit8-15 DMA Channel 1 Source/Dest/Len. Direction can be Memory-to-I/O or I/O-to-Memory, Memory-Address can be Incrementing or Decrementing, I/O-Address is Fixed (see DCNTL Register). DRQ is taken from /DREQ1-Pin.

#### 30h - DSTAT - DMA "Status" Register (32h on Reset)

```text
  7   DE1   DMA Channel 1 Enable (0=Ready, 1=Start/Busy)
  6   DE0   DMA Channel 0 Enable (0=Ready, 1=Start/Busy)
  5   /DWE1 Writing to DE1 (0=Allowed, 1=Ignored, keep Bit7 unchanged)
  4   /DWE0 Writing to DE0 (0=Allowed, 1=Ignored, keep Bit6 unchanged)
  3   DIE1  DMA Channel 1 Interrupt Enable (0=Disable, 1=Enable)
  2   DIE0  DMA Channel 0 Interrupt Enable (0=Disable, 1=Enable)
  1   -     Unused (should be all-ones)
  0   DME   DMA Main Enable
```

#### 31h - DMODE - DMA Mode Register (E1h on Reset)

```text
  7-6 -     Unused (should be all-ones)
  5-4 DM    DMA Channel 0 Dest (0=Mem/Inc, 1=Mem/Dec, 2=Mem/Fix, 3=IO/Fix)
  3-2 SM    DMA Channel 0 Src  (0=Mem/Inc, 1=Mem/Dec, 2=Mem/Fix, 3=IO/Fix)
  1   MMOD  DMA Channel 0 Mem-to-Mem Mode (0=Cycle Steal, 1=Burst)
  0   -     Unused (should be all-ones)
```

#### 32h - DCNTL - DMA/WAIT Control Register (F0h on Reset)

```text
  7-6 MW   Memory Waitstates (0..3 = 0..3)
  5-4 IW   External I/O Waitstates (0..3 = 1..4) and /INT/LIR and more XXX
  3   DMS1 DMA Channel 1 Sense /DREQ1-Pin (0=Sense Level, 1=Sense Edge)
  2   DMS0 DMA Channel 0 Sense /DREQ0-Pin (0=Sense Level, 1=Sense Edge)
  1   DIM1 DMA Channel 1 Src-to-Dest Direction (0=Mem-to-I/O, 1=I/O-to-Mem)
  0   DIM0 DMA Channel 1 Memory-Step Direction (0=Increment, 1=Decrement)
```

#### Note

On some chip versions address bus is only 19bits, namely that does apply on 64pin chips (68pin/80pin chips should have 20bits). Regardless of the pin-outs, the extra bit might (maybe) exist internally on newer 64pin chips(?)
