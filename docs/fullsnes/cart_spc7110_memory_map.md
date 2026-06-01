# SNES Cart SPC7110 Memory and I/O Map

#### Memory Map

```text
  4800h..4842h      SPC7110 I/O Ports
  6000h..7FFFh      Battery-backed SRAM (8K bytes, in all 3 games)
  8000h..FFFFh      Exception Handlers (Program ROM offset 8000h..FFFFh)
  C00000h..CFFFFFh  Program ROM (1MByte) (HiROM)
  D00000h..DFFFFFh  Data ROM (1MByte-fragment mapped via Port 4831h)
  E00000h..EFFFFFh  Data ROM (1MByte-fragment mapped via Port 4832h)
  F00000h..FFFFFFh  Data ROM (1MByte-fragment mapped via Port 4833h)
```

I/O Ports and SRAM are probably mirrored to banks 00h-3Fh and 80h-BFh.

Program/Data ROM is probably mirrored to 400000h-7FFFFFh, the upper 32K fragments of each 64K bank probably also to banks 00h-3Fh and 80h-BFh.

Reportedly (probably nonsense?) "data decompressed from data rom by spc7110 mapped to $50:0000-$50:FFFF".

That info would imply that "decompressed data" Port 4800h is mirrored to 500000h-5FFFFFh (though more likely, the "un-decompressed data" is mirrored from D00000h-DFFFFFh).

#### ROM-Image Format

The existing SPC7110 games are 2MB, 3MB, 5MB in size. Stored like so:

```text
  000000h..0FFFFFh  Program ROM (1MByte) (HiROM)
  100000h..xFFFFFh  Data ROM (1MByte, 2MByte, or 4MByte max)
```

Observe that the SPC7110 ROM checksums at [FFDCh..FFDFh] are calculated unconventionally: 3MB/5MB aren't "rounded-up" to 4MB/8MB. Instead, 3MB is checksummed twice (rounded to 6MB). 2MB/5MB are checksummed as 2MB/5MB (without rounding).

#### Data ROM Decompression Ports

```text
  4800h --  Decompressed Data Read
  4801h 00  Compressed Data ROM Directory Base, bit0-7
  4802h 00  Compressed Data ROM Directory Base, bit8-15
  4803h 00  Compressed Data ROM Directory Base, bit16-23
  4804h 00  Compressed Data ROM Directory Index
  4805h 00  Decompressed Data RAM Target Offset, bit0-7    OFFSET IN BANK $50
  4806h 00  Decompressed Data RAM Target Offset, bit8-15   OFFSET IN BANK $50
  4807h 00  Unknown ("DMA Channel for Decompression")
  4808h 00  Unknown ("C r/w option, unknown")
  4809h 00  Decompressed Data Length Counter, bit0-7
  480Ah 00  Decompressed Data Length Counter, bit8-15
  480Bh 00  Unknown ("Decompression Mode")
  480Ch 00  Decompression Status (bit7: 0=Busy/Inactive, 1=Ready/DataAvailable)
```

#### Direct Data ROM Access

```text
  4810h 00  Data ROM Read from [Base] or [Base+Offs], and increase Base or Offs
  4811h 00  Data ROM Base, bit0-7   (R/W)
  4812h 00  Data ROM Base, bit8-15  (R/W)
  4813h 00  Data ROM Base, bit16-23 (R/W)
  4814h 00  Data ROM Offset, bit0-7   ;\optionally Base=Base+Offs
  4815h 00  Data ROM Offset, bit8-15  ;/on writes to both of these registers
  4816h 00  Data ROM Step, bit0-7
  4817h 00  Data ROM Step, bit8-15
  4818h 00  Data ROM Mode
  481Ah 00  Data ROM Read from [Base+Offset], and optionally set Base=Base+Offs
```

#### Unsigned Multiply/Divide Unit

```text
  4820h 00  Dividend, Bit0-7 / Multiplicand, Bit0-7
  4821h 00  Dividend, Bit8-15 / Multiplicand, Bit8-15
  4822h 00  Dividend, Bit16-23
  4823h 00  Dividend, Bit24-31
  4824h 00  Multiplier, Bit0-7
  4825h 00  Multiplier, Bit8-15, Start Multiply on write to this register
  4826h 00  Divisor, Bit0-7
  4827h 00  Divisor, Bit8-15, Start Division on write to this register
  4828h 00  Multiply/Divide Result, Bit0-7
  4829h 00  Multiply/Divide Result, Bit8-15
  482Ah 00  Multiply/Divide Result, Bit16-23
  482Bh 00  Multiply/Divide Result, Bit24-31
  482Ch 00  Divide Remainder, Bit0-7
  482Dh 00  Divide Remainder, Bit8-15
  482Eh 00  Multiply/Divide Reset  (write = reset 4820h..482Dh) (write 00h)
  482Fh 00  Multiply/Divide Status (bit7: 0=Ready, 1=Busy)
```

#### Memory Mapping

```text
  4830h 00  SRAM Chip Enable/Disable (bit7: 0=Disable, 1=Enable)
  4831h 00  Data ROM Bank for D00000h-DFFFFFh (1MByte, using HiROM mapping)
  4832h 01  Data ROM Bank for E00000h-EFFFFFh (1MByte, using HiROM mapping)
  4833h 02  Data ROM Bank for F00000h-FFFFFFh (1MByte, using HiROM mapping)
  4834h 00  SRAM Bank Mapping?, workings unknown
```

#### Real-Time Clock Ports (for external RTC-4513)

```text
  4840h 00  RTC Chip Enable/Disable (bit0: 0=Disable, 1=Enable)
  4841h --  RTC Command/Index/Data Port
  4842h --  RTC Ready Status
```
