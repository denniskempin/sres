# SNES DMA and HDMA Channel 0..7 Registers

For below ports, x = Channel number (0-7)

#### 43x0h - DMAPx - DMA/HDMA Parameters (R/W)

```text
  7     Transfer Direction (0=A:CPU to B:I/O, 1=B:I/O to A:CPU)
  6     Addressing Mode    (0=Direct Table, 1=Indirect Table)    (HDMA only)
  5     Not used (R/W) (unused and unchanged by all DMA and HDMA)
  4-3   A-BUS Address Step  (0=Increment, 2=Decrement, 1/3=Fixed) (DMA only)
  2-0   Transfer Unit Select (0-4=see below, 5-7=Reserved)
```

DMA Transfer Unit Selection:

```text
  Mode  Bytes              B-Bus 21xxh Address   ;Usage Examples...
  0  =  Transfer 1 byte    xx                    ;eg. for WRAM (port 2180h)
  1  =  Transfer 2 bytes   xx, xx+1              ;eg. for VRAM (port 2118h/19h)
  2  =  Transfer 2 bytes   xx, xx                ;eg. for OAM or CGRAM
  3  =  Transfer 4 bytes   xx, xx,   xx+1, xx+1  ;eg. for BGnxOFS, M7x
  4  =  Transfer 4 bytes   xx, xx+1, xx+2, xx+3  ;eg. for BGnSC, Window, APU..
  5  =  Transfer 4 bytes   xx, xx+1, xx,   xx+1  ;whatever purpose, VRAM maybe
  6  =  Transfer 2 bytes   xx, xx                ;same as mode 2
  7  =  Transfer 4 bytes   xx, xx,   xx+1, xx+1  ;same as mode 3
```

A HDMA transfers ONE unit per scanline (=max 4 bytes). General Purpose DMA has a 16bit length counter, allowing to transfer up to 10000h bytes (ie. not 10000h units).

43x1h - BBADx - DMA/HDMA I/O-Bus Address (PPU-Bus aka B-Bus) (R/W) For both DMA and HDMA:

```text
  7-0   B-Bus Address (selects an I/O Port which is mapped to 2100h-21FFh)
```

For normal DMA this should be usually 04h=OAM, 18h=VRAM, 22h=CGRAM, or 80h=WRAM. For HDMA it should be usually some PPU register (eg. for changing scroll offsets midframe).

43x2h - A1TxL - HDMA Table Start Address (low) / DMA Current Addr (low) (R/W) 43x3h - A1TxH - HDMA Table Start Address (hi)  / DMA Current Addr (hi) (R/W) 43x4h - A1Bx - HDMA Table Start Address (bank) / DMA Current Addr (bank) (R/W) For normal DMA:

```text
  23-16  CPU-Bus Data Address Bank (constant, not incremented/decremented)
  15-0   CPU-Bus Data Address (incremented/decremented/fixed, as selected)
```

For HDMA:

```text
  23-16  CPU-Bus Table Address Bank (constant, bank number for 43x8h/43x9h)
  15-0   CPU-Bus Table Address      (constant, reload value for 43x8h/43x9h)
```

43x5h - DASxL - Indirect HDMA Address (low) / DMA Byte-Counter (low) (R/W) 43x6h - DASxH - Indirect HDMA Address (hi)  / DMA Byte-Counter (hi)  (R/W) 43x7h - DASBx - Indirect HDMA Address (bank) (R/W) For normal DMA:

```text
  23-16  Not used
  15-0   Number of bytes to be transferred (1..FFFFh=1..FFFFh, or 0=10000h)
  (This is really a byte-counter; with a 4-byte "Transfer Unit", len=5 would
  transfer one whole Unit, plus the first byte of the second Unit.)
  (The 16bit value is decremented during transfer, and contains 0000h on end.)
```

For HDMA in direct mode:

```text
  23-0   Not used     (in this mode, the Data is read directly from the Table)
```

For HDMA in indirect mode:

```text
  23-16  Current CPU-Bus Data Address Bank   (this must be set by software)
  16-0   Current CPU-Bus Data Address (automatically loaded from the Table)
```

43x8h - A2AxL - HDMA Table Current Address (low) (R/W) 43x9h - A2AxH - HDMA Table Current Address (high) (R/W) For normal DMA:

```text
  15-0  Not used
```

For HDMA:

```text
  -     Current Table Address Bank (taken from 43x4h)
  15-0  Current Table Address (reloaded from 43x2h/43x3h) (incrementing)
```

43xAh - NTRLx - HDMA Line-Counter (from current Table entry) (R/W) For normal DMA:

```text
  7-0   Not used
```

For HDMA:

```text
  7     Repeat-flag                         ;\(loaded from Table, and then
  6-0   Number of lines to be transferred   ;/decremented per scanline)
```

#### 43xBh - UNUSEDx - Unused Byte (R/W)

```text
  7-0   Not used (read/write-able)
```

Can be used as a fast RAM location (but NOT as a fixed DMA source address for memfill). Storing any value in this register seems to have no effect on the transfer (and the value is left intact, not modified by DMA nor direct nor indirect HDMAs).

#### 43xCh..43xEh - Unused region (open bus)

Unused. Reading returns garbage (open bus), writing seems to have no effect, even when trying to "disturb" HDMAs.

43xFh - MIRRx - Read/Write-able mirror of 43xBh (R/W) Mirror of 43xBh.

HDMA Table Formats (in Direct and Indirect Mode) In Direct Mode, the table consists of entries in following format:

```text
  1 byte   Repeat-flag & line count
  N bytes  Data (where N=unit size, if repeat=1: multiplied by line count)
```

In Indirect Mode, the table consists of entries in following format:

```text
  1 byte   Repeat-flag & line count
  2 bytes  16bit pointer to N bytes of Data (where N = as for Direct HDMA)
```

In either mode: The "repeat-flag & line count" bytes can be:

```text
  00h       Terminate this HDMA channel (until it restarts in next frame)
  01h..80h  Transfer 1 unit in 1 line, then pause for next "X-01h" lines
  81h..FFh  Transfer X-80h units in X-80h lines ("repeat mode")
```

The "count" and "pointer" values are always READ from the table. The "data" values are READ or WRITTEN depending on the transfer direction. The transfer step is always INCREMENTING for HDMA (for both the table itself, as well as for any indirectly addressed data blocks).
