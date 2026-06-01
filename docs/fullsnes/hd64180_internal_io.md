# HD64180 Internal I/O Map

#### HD64180 Internal Registers

Internal I/O Ports are initially mapped to Port 0000h..003Fh (but can be reassigned to 0040h..007Fh, 0080h..00BFh, or 00C0h..00FFh via ICR register).

```text
  Port Name      Expl.                                    (On Reset)
  00h  CNTLA0    ASCI Channel 0 Control Reg A             (10h, bit3=var)
  01h  CNTLA1    ASCI Channel 1 Control Reg A             (10h, bit3/bit4=var)
  02h  CNTLB0    ASCI Channel 0 Control Reg B             (07h, bit7/bit5=var)
  03h  CNTLB1    ASCI Channel 1 Control Reg B             (07h, bit7=var)
  04h  STAT0     ASCI Channel 0 Status Register           (00h, bit2/bit1=var)
  05h  STAT1     ASCI Channel 1 Status Register           (02h)
  06h  TDR0      ASCI Channel 0 Transmit Data Register
  07h  TDR1      ASCI Channel 1 Transmit Data Register
  08h  RDR0      ASCI Channel 0 Receive Data Register
  09h  RDR1      ASCI Channel 1 Receive Data Register
  0Ah  CNTR      CSI/O Control Register                   (0Fh)
  0Bh  TRDR      CSI/O Transmit/Receive Data Register
  0Ch  TMDR0L    Timer 0 Counter "Data" Register, Bit0-7  (FFh)
  0Dh  TMDR0H    Timer 0 Counter "Data" Register, Bit8-15 (FFh)
  0Eh  RLDR0L    Timer 0 Reload Register, Bit0-7          (FFh)
  0Fh  RLDR0H    Timer 0 Reload Register, Bit8-15         (FFh)
  10h  TCR       Timer Control Register                   (00h)
  11h-13h        Reserved
   12h  ASEXT0    ASCI Channel 0 Extension Control Reg ;\Z8S180/Z8L180 only
   13h  ASEXT1    ASCI Channel 0 Extension Control Reg ;/(not Z80180/HD64180)
  14h  TMDR1L    Timer 1 Counter "Data" Register, Bit0-7  (FFh)
  15h  TMDR1H    Timer 1 Counter "Data" Register, Bit8-15 (FFh)
  16h  RLDR1L    Timer 1 Reload Register, Bit0-7          (FFh)
  17h  RLDR1H    Timer 1 Reload Register, Bit8-15         (FFh)
  18h  FRC       Free Running Counter                     (FFh)
  19h-1Fh        Reserved
   1Ah  ASTC0L    ASCI Channel 0 Time Constant, Bit0-7  ;\
   1Bh  ASTC0H    ASCI Channel 0 Time Constant, Bit8-15 ; Z8S180/Z8L180 only
   1Ch  ASTC1L    ASCI Channel 1 Time Constant, Bit0-7  ; (not Z80180/HD64180)
   1Dh  ASTC1H    ASCI Channel 1 Time Constant, Bit8-15 ;
   1Eh  CMR       Clock Multiplier Register             ;
   1Fh  CCR       CPU Control Register                  ;/
  20h  SAR0L     DMA Channel 0 Source Address, Bit0-7 (Memory or I/O)
  21h  SAR0H     DMA Channel 0 Source Address, Bit8-15 (Memory or I/O)
  22h  SAR0B     DMA Channel 0 Source Address, Bit16-19 (Memory or DRQ)
  23h  DAR0L     DMA Channel 0 Destination Address, Bit0-7 (Memory or I/O)
  24h  DAR0H     DMA Channel 0 Destination Address, Bit8-15 (Memory or I/O)
  25h  DAR0B     DMA Channel 0 Destination Address, Bit16-19 (Memory or DRQ)
  26h  BCR0L     DMA Channel 0 Byte Count Register, Bit0-7
  27h  BCR0H     DMA Channel 0 Byte Count Register, Bit8-15
  28h  MAR1L     DMA Channel 1 Memory Address, Bit0-7 (Source or Dest)
  29h  MAR1H     DMA Channel 1 Memory Address, Bit8-15 (Source or Dest)
  2Ah  MAR1B     DMA Channel 1 Memory Address, Bit16-19 (Source or Dest)
  2Bh  IAR1L     DMA Channel 1 I/O Address, Bit0-7 (Dest or Source)
  2Ch  IAR1H     DMA Channel 1 I/O Address, Bit8-15 (Dest or Source)
   2Dh            Reserved ;IAR1B on Z8S180/Z8L180 (not Z80180/HD64180)
  2Eh  BCR1L     DMA Channel 1 Byte Count Register, Bit0-7
  2Fh  BCR1H     DMA Channel 1 Byte Count Register, Bit8-15
  30h  DSTAT     DMA "Status" Register                   (32h on Reset)
  31h  DMODE     DMA Mode Register                       (C1h on Reset)
  32h  DCNTL     DMA/WAIT Control Register               (F0h on Reset)
  33h  IL        Interrupt Vector Low Register           (00h on Reset)
  34h  ITC       INT/TRAP Control Register               (39h on Reset)
  35h            Reserved
  36h  RCR       Refresh Control Register                (FCh on Reset)
  37h            Reserved
  38h  CBR       MMU Common Base Register (Common Area 1)(00h on Reset)
  39h  BBR       MMU Bank Base Register (Bank Area)      (00h on Reset)
  3Ah  CBAR      MMU Common/Bank Area Register           (F0h on Reset)
  3Bh-3Dh        Reserved
  3Eh  OMCR      Operation Mode, Z180 only (not HD64180) (FFh on Reset)
  3Fh  ICR       I/O Control Register                    (1Fh on Reset)
```
