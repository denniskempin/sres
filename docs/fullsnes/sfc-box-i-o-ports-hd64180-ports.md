# SFC-Box I/O Ports (HD64180 Ports)

#### System Clock

The CPU and Timer/Baudrate-Prescalers are clocked at PHI=4.608MHz (derived from a 9.216MHz oscillator, and internally divided by 2 in the HD64180).

asci_ch0 - implemented, tx is "used" for UNUSED joypad recording asci_ch1 - implemented, but rx+tx both unused

```text
  initialized to 8N1 with baudrate 28.8 kbit/s (PHI/10/16 SHR 0)
```

csio - implemented, tx is used - OSD video chip

```text
  initialized to 230.4 kbit/s (PHI/20 SHR 0)
  chipselect is controlled via port [81h].W.Bit7 (1=select, 0=deselect)
  the OSD chip is having an unknown dotclock (higher than the SNES)
  (12 pixels on OSD are having roughly the same width as 8 pixels on SNES)
```

#### timer0

```text
  timer0 should run at 4.608MHz/20/130 --> 1772.3 Hz
```

#### timer1

```text
  timer1 should run at 4.608MHz/20/3840 --> 60.0 Hz
```

#### external interrupts

```text
  reset  power-up, and maybe watchdog? (but, probably not "RESET" button?)
  nmi    unknown/unused
  int0   coin ? (must be low for 78..140 timer0 ticks) (44ms..80ms)
  int1   joypad is/was accessed by snes ?
  int2   unknown/unused
```

#### CPU Registers

```text
  Port Name      Expl.                                    (On Reset)
  [00] CNTLA0    ASCI Channel 0 Control Reg A             (10h, bit3=var)
  [01] CNTLA1    ASCI Channel 1 Control Reg A             (10h, bit3=var)
  [02] CNTLB0    ASCI Channel 0 Control Reg B             (07h, bit7/bit5=var)
  [03] CNTLB1    ASCI Channel 1 Control Reg B             (07h, bit7=var)
  [04] STAT0     ASCI Channel 0 Status Register           (00h, bit1/2=var)
  [05] STAT1     ASCI Channel 1 Status Register           (02h)
  [06] TDR0      ASCI Channel 0 Transmit Data Register
  [07] TDR1      ASCI Channel 1 Transmit Data Register
  [08] RDR0      ASCI Channel 0 Receive Data Register
  [09] RDR1      ASCI Channel 1 Receive Data Register

  [0A] CNTR      CSI/O Control Register                      (0Fh)
  [0B] TRDR      CSI/O Transmit/Receive Data Register

  [0C] TMDR0L    Timer 0 Counter "Data" Register, Bit0-7     (FFh)
  [0D] TMDR0H    Timer 0 Counter "Data" Register, Bit8-15    (FFh)
  [0E] RLDR0L    Timer 0 Reload Register, Bit0-7             (FFh)
  [0F] RLDR0H    Timer 0 Reload Register, Bit8-15            (FFh)
  [10] TCR       Timer Control Register                      (00h)
  [14] TMDR1L    Timer 1 Counter "Data" Register, Bit0-7     (FFh)
  [15] TMDR1H    Timer 1 Counter "Data" Register, Bit8-15    (FFh)
  [16] RLDR1L    Timer 1 Reload Register, Bit0-7             (FFh)
  [17] RLDR1H    Timer 1 Reload Register, Bit8-15            (FFh)

  [18] FRC       Free Running Counter (not used by SFC-Box)  (FFh)
  [20-31] (DMA)  DMA Registers        (not used by SFC-Box)
  [36] RCR       Refresh Control Reg  (not used by SFC-Box)  (FCh)
  [3F] ICR       I/O Control Register (not used by SFC-Box)  (1Fh)
  [11-13]        Reserved             (not used by SFC-Box)
  [19-1F]        Reserved             (not used by SFC-Box)
  [35]           Reserved             (not used by SFC-Box)
  [37]           Reserved             (not used by SFC-Box)
  [3B-3E]        Reserved             (not used by SFC-Box)

  [32] DCNTL     DMA/WAIT Control Register                   (F0h)
  [33] IL        Interrupt Vector Low Register               (00h)
  [34] ITC       INT/TRAP Control Register                   (39h)
  [38] CBR       MMU Common Base Register (Common Area 1)    (00h)
  [39] BBR       MMU Bank Base Register (Bank Area)          (00h)
  [3A] CBAR      MMU Common/Bank Area Register               (F0h)
```

#### OSD_INIT

```text
  OUT[81h]=00h                  ;osd chip deselect
  OUT[0Ah]=00h                  ;init CSIO
  for i=1 to 4,OUT[81H]=80h,OUT[81H]=00h,next  ;osd wake-up from reset-state
```

#### OSD_SEND_CMD

```text
  ;in: HL=param10bit, A=(80h OR cmd*8)
  SHL  L    ;move bit7 to cy
  RCL  H    ;shift-in cy
  SHR  L    ;undo SHL (now bit7=0 for second byte)
  OR   A,H  ;merge command and 3bit data
  CALL osd_send_byte_a
  LD   A,L  ;7bit data
  JMP  osd_send_byte_a
```

#### OSD_SEND_BYTE

```text
  set OUT[81h]=80h                ;osd chip select
  set OUT[0Bh]=data               ;prepare TX data
  set OUT[0Ah]=10h                ;start TX
  wait until (IN[0Ah] AND 10h)=0  ;wait until TX ready
  set OUT[81h]=00h                ;osd chip deselect
```
