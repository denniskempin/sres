# SNES Controllers Exertainment - RS232 Controller

Texas Instruments TL16C550AN - Asynchronous Communications Element (ACE) The ACE uses eight I/O addresses (mapped to 21C0h-21C7h in the SNES), the meaning of the first two addresses depends on the "DLAB" bit (which can be changed via 21C3h.Bit7).

#### 21C0h (when DLAB=0) - TL16C550AN - RX Data FIFO (R)

```text
  0-7  Data (with 16-byte FIFO)
```

#### 21C0h (when DLAB=0) - TL16C550AN - TX Data FIFO (W)

```text
  0-7  Data (with 16-byte FIFO)
```

#### 21C1h (when DLAB=0) - TL16C550AN - Interrupt Control (R/W)

```text
  0    Received Data Available Interrupt            (0=Disable, 1=Enable)
  1    Transmitter Holding Register Empty Interrupt (0=Disable, 1=Enable)
  2    Receiver Line Status Interrupt               (0=Disable, 1=Enable)
  3    Modem Status Interrupt                       (0=Disable, 1=Enable)
  4-7  Not used (always zero)
```

21C0h (when DLAB=1) - TL16C550AN - Baudrate Divisor Latch LSB, Bit0-7 (R/W) 21C1h (when DLAB=1) - TL16C550AN - Baudrate Divisor Latch MSB, Bit8-15 (R/W)

```text
  0-7  Divisor Latch LSB/MSB, should be set to "divisor = XIN / (baudrate*16)"

21C2h - TL16C550AN - Interrupt Status (R)
  0    Interrupt Pending Flag (0=Pending, 1=None)
  1-3  Interrupt ID, 3bit     (0..7=see below) (always 00h when Bit0=1)
  4-5  Not used (always zero)
  6    FIFOs Enabled (always zero in TL16C450 mode) ;\these bits have same
  7    FIFOs Enabled (always zero in TL16C450 mode) ;/value as "FIFO Enable"
```

The 3bit Interrupt ID can have following values:

```text
  ID Prio Expl.
  00h 4   Handshaking inputs CTS,DSR,RI,DCD have changed      (Ack: Read 21C6h)
  01h 3   Transmitter Holding Register Empty   (Ack: Write 21C0h or Read 21C2h)
  02h 2   RX FIFO has reached selected trigger level          (Ack: Read 21C0h)
  03h 1   RX Overrun/Parity/Framing Error, or Break Interrupt (Ack: Read 21C5h)
  06h 2   RX FIFO non-empty & wasn't processed for longer time(Ack: Read 21C0h)
```

Interrupt ID values 04h,05h,07h are not used.

```text
21C2h - TL16C550AN - FIFO Control (W)
  0    FIFO Enable (0=Disable, 1=Enable) (Enables access to FIFO related bits)
  1    Receiver FIFO Reset      (0=No Change, 1=Clear RX FIFO)
  2    Transmitter FIFO Reset   (0=No Change, 1=Clear TX FIFO)
  3    DMA Mode Select (Mode for /RXRDY and /TXRDY) (0=Mode 0, 1=Mode 1)
  4-5  Not used (should be zero)
  6-7  Receiver FIFO Trigger    (0..3 = 1,4,8,14 bytes)

21C3h - TL16C550AN - Character Format Control (R/W)
  0-1  Character Word Length    (0..3 = 5,6,7,8 bits)
  2    Number of Stop Bits      (0=1bit, 1=2bit; for 5bit chars: only 1.5bit)
  3    Parity Enable            (0=None, 1=Enable Parity or 9th data bit)
  4-5  Parity Type/9th Data bit (0=Odd, 1=Even, 2=Set9thBit, 3=Clear9thBit)
  6    Set Break                (0=Normal, 1=Break, Force SOUT to Low)
  7    Divisor Latch Access     (0=Normal I/O, 1=Divisor Latch I/O) (DLAB)

21C4h - TL16C550AN - Handshaking Control (R/W)
  0    Output Level for /DTR pin  (Data Terminal Ready) (0=High, 1=Low)
  1    Output Level for /RTS pin  (Request to Send)     (0=High, 1=Low)
  2    Output Level for /OUT1 pin (General Purpose)     (0=High, 1=Low)
  3    Output Level for /OUT2 pin (General Purpose)     (0=High, 1=Low)
  4    Loopback Mode (0=Normal, 1=Testmode, loopback TX to RX)
  5-7  Not used (always zero)

21C5h - TL16C550AN - RX/TX Status (R/W, but should accessed as read-only)
  0    RX Data Ready (DR)       (0=RX FIFO Empty, 1=RX Data Available)
  1    RX Overrun Error (OE)    (0=Okay, 1=Error) (RX when RX FIFO Full)
  2    RX Parity Error (PE)     (0=Okay, 1=Error) (RX parity bad)
  3    RX Framing Error (FE)    (0=Okay, 1=Error) (RX stop bit bad)
  4    RX Break Interrupt (BI)  (0=Normal, 1=Break) (RX line LOW for long time)
  5    Transmitter Holding Register (THRE) (1=TX FIFO is empty)
  6    Transmitter Empty (TEMT) (0=No, 1=Yes, TX FIFO and TX Shift both empty)
  7    At least one Overrun/Parity/Framing Error in RX FIFO (0=No, 1=Yes/Error)
```

Bit7 is always zero in TL16C450 mode. Bit1-3 are automatically cleared after reading. In FIFO mode, bit2-3 reflect to status of the current (=oldest) character in the FIFO (unknown/unclear if bit2-3 are also auto-cleared when in FIFO mode).

```text
21C6h - TL16C550AN - Handshaking Status (R/W? - should accessed as read-only)
  0    Change flag for /CTS pin (Clear to Send)       ;\change flags (0=none,
  1    Change flag for /DSR pin (Data Set Ready)      ; 1=changed since last
  2    Change flag for /RI pin  (Ring Indicator)      ; read) (automatically
  3    Change flag for /DCD pin (Data Carrier Detect) ;/cleared after reading)
  4    Input Level on /CTS pin (Clear to Send)        ;\
  5    Input Level on /DSR pin (Data Set Ready)       ; current levels
  6    Input Level on /RI pin  (Ring Indicator)       ; (inverted ?)
  7    Input Level on /DCD pin (Data Carrier Detect)  ;/

21C7h - TL16C550AN - Scratch (R/W)
  0-7  General Purpose Storage (eg. read/write-able for chip detection)
```

#### Note

The TL16C550AN doesn't seem to support a TX FIFO Full flag, nor automatic RTS/CTS handshaking.

Note on Nintendo DSi (newer handheld console, not SNES related) The DSi's AR6002 wifi chip is also using a TL16C550AN-style UART (for TTY debug messages).
