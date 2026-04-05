# HD64180 Serial I/O Ports (ASCI and CSI/O)

#### Asynchronous Serial Communication Interfaces (ASCI) and Clocked Serial I/O (CSI/I)

#### XXX pg 51... 56

00h - CNTLA0 - ASCI Channel 0 Control Reg A (10h on Reset, bit3=var) 01h - CNTLA1 - ASCI Channel 1 Control Reg A (10h on Reset, bit3/bit4=var)

```text
  7   MPE   RX Multi Processor Filter (0=RX all bytes, 1=RX flagged bytes)
  6   RE    RX Receiver Enable        (0=Disable, 1=Enable)
  5   TE    TX Transmitter Enable     (0=Disable, 1=Enable)
  4   /RTS0 for Ch0: Request to Send output (0=Low, 1=High) (/RTS pin)
      CKL1D for Ch1: CKA1 Clock Disable (CKA1/TEND pin)
  3   MPBR  Read:  RX Multi Processor Bit (Received Flag-Bit)
      EFR   Write: RX Error Flag Reset (0=Reset OVRN,PE,FE-Flags, 1=No Change)
  2   MOD2  Number of Data bits   (0=7bit, 1=8bit)
  1   MOD1  Number of Parity bits (0=None, 1=1bit) (only if MP=0)
  0   MOD0  Number of Stop bits   (0=1bit, 1=2bit)
```

02h - CNTLB0 - ASCI Channel 0 Control Reg B (07h on Reset, bit7/bit5=var) 03h - CNTLB1 - ASCI Channel 1 Control Reg B (07h on Reset, bit7=var)

```text
  7   MPBT  TX Multi Processor Bit (Flag-Bit to be Transmitted)
  6   MP    Multiprocessor Mode (0=Off/Normal, 1=Add Flag-bit to all bytes)
  5   CTS   Read: /CTS-pin (0=Low, 1=High),
      PS    Write: Prescaler (0=Div10, 1=Div30)
  4   PEO   Parity Even/Odd (0=Even, 1=Odd) (ignored when MOD1=0 or MP=1)
  3   DR    Divide Ratio (0=Div16, 1=Div64)
  2-0 SS    Speed Select (0..6: "(PHI SHR N)", 7=External clock)
```

The baudrate is "SS div PS div DR" (or "External_Clock div DR").

04h - STAT0 - ASCI Channel 0 Status Register (00h on Reset, bit2/bit1=var) 05h - STAT1 - ASCI Channel 1 Status Register (02h on Reset)

```text
  7   RDRF  RX Receive Data Register Full (0=No, 1=Yes)             (R)
  6   OVRN  RX Overrun Error (0=Okay, 1=Byte received while RDRF=1) (R)
  5   PE    RX Parity Error  (0=Okay, 1=Wrong Parity Bit)           (R)
  4   FE    RX Framing Error (0=Okay, 1=Wrop Stop Bit)              (R)
  3   RIE   RX Receive Interrupt Enable                 (R/W)
  2   /DCD0 For Ch0: Data Carrier Detect (/DCD pin)     (R)
      CTS1E For Ch1: CTS input enable (/CTS pin)        (R/W)
  1   TDRE  TX Transmit Data Register Empty             (R)
  0   TIE   TX Transmit Interrupt Enable                (R/W)
```

Note: RDRD/TDRE can be used as DRQ signal for DMA channel 0.

06h - TDR0 - ASCI Channel 0 Transmit Data Register 07h - TDR1 - ASCI Channel 1 Transmit Data Register 08h - RDR0 - ASCI Channel 0 Receive Data Register 09h - RDR1 - ASCI Channel 1 Receive Data Register

```text
  7-0  Data
```

The hardware can hold one byte in the data register (plus one byte currently processed in a separate shift register).

#### 0Ah - CNTR - CSI/O Control Register (0Fh on Reset)

```text
  7   EF    End Flag, completion of Receive/Transmit (0=No/Busy, 1=Yes/Ready)
  6   EIE   End Interrupt Enable (0=Disable, 1=Enable)
  5   RE    Receive Enable  (0=Off/Ready, 1=Start/Busy)
  4   TE    Transmit Enable (0=Off/Ready, 1=Start/Busy)
  3   -     Unused (should be all-ones)
  2-0 SS    Speed Select (0..6: "(20 shl N) clks per bit", 7=External clock)
```

The select "speed" is output on CKS pin (or input from CKS pin when selecting External clock). Bit7 is read-only (cleared when reading/writing TRDR).

#### 0Bh - TRDR - CSI/O Transmit/Receive Data Register

```text
  7-0  Data (8bit) (called TRDR by Hitachi, called TRD by Zilog)
```

Data is output on TXS pin, and input on RXS pin (both LSB first). Despite of the separate pins, one may NOT set RE and TE simultanoulsy (for whatever reason... or maybe it's meant to WORK ONLY if RX and TX are STARTED simultaneously). The RXS pin is also used as /CTS1 (for ASCI channel 1).

#### ASCI Multi Processor "Network" Feature

This feature allows to share the serial bus by multiple computers. Each byte is transferred with a "MPB" Multi Processor Flag Bit (located between Data and Stop bits) (Parity is forcefully disabled in Multi Processor Mode).

Assume broadcasting "Header+Data" Packets (with "Header" bytes flagged as MPB=1, and "Data" as MPB=0): The RX-Filter can select to receive only "Header" bytes, and, if the receiver treats itself to be addressed by the header, it can change the filter setting depending on whether it wants to receive/skip the following "Data" bytes.
