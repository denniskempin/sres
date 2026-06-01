# SNES Cart Capcom CX4 - I/O Ports

#### CX4 I/O Map

```text
  6000h..6BFFh R/W  CX4RAM (3Kbytes)
  6C00h..7F3Fh ?    Unknown/unused
  7F40h..7F42h ?/W  DMA source, 24bit SNES LoROM address
  7F43h..7F44h ?/W  DMA length, 16bit, in bytes (eg. 0800h = 2Kbytes)
  7F45h..7F46h ?/W  DMA destination, 16bit in CX4RAM (6000h = 1st byte)
  7F47h        ?/W  DMA start (write 00h to transfer direction SNES-to-CX4)
  7F48h        ?/W  Unknown "toggle" (set to 00h/01h, maybe cache load/on/off?)
  7F49h..7F4Bh R/W  Program ROM Base, 24bit LoROM addr (028000h in Mega Man)
  7F4Ch        ?/W  Unknown (set to 00h or 01h) soft_reset? maybe flush_cache?
  7F4Dh..7F4Eh ?/W  Program ROM Instruction Page (PC/200h)
  7F4Fh        ?/W  Program ROM Instruction Pointer (PC/2), starts execution
  7F50h..7F51h R/W  Unknown, set to 0144h (maybe config flags or waitstates?)
  7F52h        R/W  Unknown (set to 00h) hard_reset? maybe force stop?
  7F53h..7F5Dh ?    Unknown/unused
  7F5Eh        R/?  Status (bit6=busy, set upon [7F47],[7F48],[7F4F] writes)
  7F5Fh        ?    Unknown/unused
  7F60h..7F69h ?    Unknown/unused (maybe [FFE0..FFE9])
  7F6Ah..7F6Bh R/W  SNES NMI Vector       [FFEA..FFEB]
  7F6Ch..7F6Dh ?    Unknown/unused (maybe [FFEC..FFED])
  7F6Eh..7F6Fh R/W  SNES IRQ Vector       [FFEE..FFEF]
  7F70h..7F7Fh ?    Unknown/unused (maybe [FFF0..FFFF])
  7F80h..7FAFh R/W  Sixteen 24bit CX4 registers (R0..R15, at 7F80h+N*3)
  7FB0h..7FFFh ?    Unknown/unused
  8000h..FFFFh R    ROM (32Kbyte LoROM Banks) (disabled when CX4 is busy)
  FFExh..FFxxh R/?  Exception Vectors (from above I/O Ports, when CX4 is busy)
```

#### Exception Vectors

Unknown if these can be manually enabled, or if they are automatically enabled when the CX4 is "busy". In the latter case, they would be REQUIRED to be same as the ROM vectors (else LSB/MSB might be accidently fetched from different locations when busy-flag changes at same time).
