# SNES Cart SPC7110 Decompression I/O Ports

```text
4800h - Decompressed Data Read
```

Reading from this register returns one decompressed byte, and does also decrease the 16bit length counter [4809h] by one.

```text
4801h - Compressed Data ROM Directory Base, bit0-7
4802h - Compressed Data ROM Directory Base, bit8-15
4803h - Compressed Data ROM Directory Base, bit16-23
4804h - Compressed Data ROM Directory Index
```

Selects a directory entry in Data ROM at [Base+Index*4]. Each entry is 4-bytes in size:

```text
  Byte0  Decompression Mode (00h,01h,02h)
  Byte1  Compressed Data ROM Source Pointer, bit16-23  ;\ordered as so
  Byte2  Compressed Data ROM Source Pointer, bit8-15   ; (ie. big-endian)
  Byte3  Compressed Data ROM Source Pointer, bit0-7    ;/

4805h - Decompressed Data RAM Target Offset, bit0-7    OFFSET IN BANK $50
4806h - Decompressed Data RAM Target Offset, bit8-15   OFFSET IN BANK $50
```

Reportedly: Destination address in bank 50h, this would imply that the SPC7110 chip contains around 64Kbytes on-chip RAM, which is probably utmost nonsense.

Or, reportedly, too: Causes the first "N" decompressed bytes to be skipped, before data shows up at 4800h. That sounds more or less reasonable. If so, unknown if the hardware does decrement the offset value?

```text
4807h - DMA Channel for Decompression
```

Unknown. Reportedly "DMA CHANNEL FOR DECOMPRESSION, set to match snes dma channel used for compressed data". That info seems to be nonsense; the registers seems to be always set to 00h, no matter if/which DMA channel is used.

```text
4808h - C r/w option, unknown
```

Unknown. Reportedly "C r/w option, unknown".

```text
4809h - Decompressed Data Length Counter, bit0-7
480Ah - Decompressed Data Length Counter, bit8-15
```

This counter is decremented on reads from [4800h]. One can initialize the counter before decompression & check its value during decompression. However, this doesn't seem to be required hardware-wise; the decompression seems to be working endless (as long as software reads [4800h]), and doesn't seem to "stop" when the length counter becomes zero.

```text
480Bh - Decompression Mode
```

Reportedly:

```text
  00 - manual decompression, $4800 is used to read directly from the data rom

  02 - hardware decompression, decompressed data is mapped to $50:0000,
       $4800 can be used to read sequentially from bank $50

480Ch - Decompression Status (bit7: 0=Busy/Inactive, 1=Ready/DataAvailable)
```

Reportedly:

```text
  DECOMPRESSION FINISHED STATUS:
  high bit set = done, high bit clear = processing,
  cleared after successful read,
  high bit is cleared after writing to $4806,
  $4809/A is set to compressed data length
  ---
  decompression mode is activated after writing to $4806
  and finishes after reading the high bit of $480C
```
