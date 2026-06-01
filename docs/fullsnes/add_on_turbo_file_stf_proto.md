# SNES Add-On Turbo File - STF Mode Transmission Protocol

#### FileTwinSendCommand (28bits)

```text
  set strobe=1
  8x sendbit (LSB first)    ;command (24h=read or 75h=write)
  20x sendbit (LSB first)   ;address (00000h..FFFFFh)
  set strobe=0
  FileTwinRecvStatusAndID
  error if bad-ID or general-error-flag (for write: also write protect-error)
  retry "FileTwinSendCommand" if desired Read (or Write) Mode bit isn't set
```

Thereafter, send/receive data byte(s), and finish by TerminateCommand

#### TerminateCommand

```text
  set strobe=1
  if command was READ then issue clk (don't do that on WRITE command)
  set strobe=0
  FileTwinRecvStatusAndID
  retry "TerminateCommand" if Data Read/Write Mode bits are still nonzero
```

#### FileTwinSendDataByte

```text
  set strobe=1
  8x sendbit (LSB first)
  set strobe=0
```

#### FileTwinRecvDataByte

```text
  set strobe=1
  set strobe=0
  8x recvbit (from joy4) (LSB first) (inverted)
```

#### FileTwinRecvStatusAndID (32bits)

```text
  set strobe=1
  set strobe=0
  12x recvbit (from joy2) (ignored)
  4x  recvbit (from joy2) (MSB first) (major ID, must be 0Eh)
  8x  recvbit (from joy2) (MSB first) (minor ID, must be FEh)
  1x  recvbit (from joy2) Data Write Mode (0=No/Idle, 1=Yes/Command 75h)
  1x  recvbit (from joy2) Data Read Mode  (0=No/Idle, 1=Yes/Command 24h)
  1x  recvbit (from joy2) General-Hardware-Error (1=Error)
  1x  recvbit (from joy2) Write-Protect-Error    (1=Error/Protected)
  4x  recvbit (from joy2) (MSB first) (capacity) (usually/always 0=128K)
```

#### Low Level Functions

```text
  set strobe=1        --> [004016h]=1
  set strobe=0        --> [004016h]=0
  recvbit (from joy2) --> bit=[004017h].bit0
  recvbit (from joy4) --> bit=NOT [004017h].bit1
  sendbit             --> [004201h]=bit*80h, dummy=[004017h]
  issue clk           --> dummy=[004017h]
```
