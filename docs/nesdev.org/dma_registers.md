---
title: "DMA registers"
source_url: "https://snes.nesdev.org/wiki/DMA_registers"
pageid: 19
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The SNES's **DMA** (Direct Memory Access) unit allows a game to copy graphics, palettes, [[OAM]] and more at a much higher speed than the CPU can accomplish alone. This allows a game to make better use of the limited amount of time it has in vblank to change graphical memory.

The SNES has two address buses: the CPU bus (also known as the A bus, which contains cartridge ROM, cartridge RAM, and the SNES's RAM) and the peripheral bus (also known as the B bus, which contains anything in the $2100-$21FF range, including [[PPU registers]] and APU registers). These buses use the same data bus, and DMA works by having a read on one address bus act as a write on the other, so copies are always from one bus to the other.

Although it can be specified as both the source and destination, DMA cannot copy from one area of the console's 128 KiB S-WRAM to another. For that, the `MVN` and `MVP` instructions are probably the best available choice. However, DMA can copy between S-WRAM and some other kind of RAM, such as cartridge RAM.

The SNES also features **HDMA** (Horizontal-blank DMA) which runs in the background and can be set up to automatically write values to hardware registers at specific scanlines, allowing for effects.

Because some revisions of the SNES [[Errata#DMA|crash when a DMA completes just as an HDMA begins]], DMA should only be used either within vertical blank, with rendering disabled, or with HDMA disabled.

These registers are always accessed at 3.58 MHz! That means that any channels that are not currently in use can have their registers repurposed for a small amount of fast RAM.

See also:

- [[DMA examples]]
- [[HDMA examples]]

DMA register summary (n = 0..7)

| Name | Address | Bits | Type | Notes |
| --- | --- | --- | --- | --- |
| [MDMAEN](#MDMAEN) | $420B | 7654 3210 | W8 | DMA enable. |
| [HDMAEN](#HDMAEN) | $420C | 7654 3210 | W8 | HDMA enable. |
| [DMAPn](#DMAPn) | $43n0 | DI.A APPP | RW8 | Direction (D), indirect HDMA (I), address increment mode (A), transfer pattern (P). |
| [BBADn](#BBADn) | $43n1 | AAAA AAAA | RW8 | B-bus address. |
| [A1TnL](#A1TnL) [A1TnH](#A1TnH) [A1Bn](#A1Bn) | $43n2 $43n3 $43n4 | LLLL LLLL HHHH HHHH BBBB BBBB | RW24 | DMA source address / HDMA table start address. |
| [DASnL](#DASnL) [DASnH](#DASnH) [DASBn](#DASBn) | $43n5 $43n6 $43n7 | LLLL LLLL HHHH HHHH BBBB BBBB | RW24 | DMA byte count (H:L) / HDMA indirect table address (B:H:L). |
| [A2AnL](#A2AnL) [A2AnH](#A2AnH) | $43n8 $43n9 | LLLL LLLL HHHH HHHH | RW16 | HDMA table current address within bank (H:L). |
| [NLTRn](#NLTRn) | $43nA | RLLL LLLL | RW8 | HDMA reload flag (R) and scanline counter (L). |
| [UNUSEDn](#UNUSEDn) | $43nB $43nF | DDDD DDDD | RW8 | Unused shared data byte (D). |
| [[MMIO register table/DMA|table source]] | | | | |

Register types:

- **R** - Readable
- **W** - Writeable
- **8** - 8-bit access only
- **16** - 8-bit access to either address, or 16-bit access to the lower address.
- **24** - 8-bit or 16-bit access to 3 registers.

## DMA channels

The SNES contains 8 separate DMA "channels" - each one contains a set of parameters to configure a DMA transfers. They are configured with registers in the $4300-$437f range, where the first 16 addresses correspond to the first register, the second 16 addresses correspond to the next, and so on.

These channels are shared by both DMA and HDMA.

### MDMAEN - Start DMA transfer ($420B write)

---

```
7  bit  0
---- ----
7654 3210
|||| ||||
|||| |||+- Channel 0 select
|||| ||+-- Channel 1 select
|||| |+--- Channel 2 select
|||| +---- Channel 3 select
|||+------ Channel 4 select
||+------- Channel 5 select
|+-------- Channel 6 select
+--------- Channel 7 select

On power-on: MDMAEN = $00
On reset:    MDMAEN = $00
```

Upon writing to this register, a DMA transfer is started for each bit that was set, starting with the lowest selected channel number up toward the highest. The CPU is stopped until all transfers have completed. If an HDMA transfer happens while the DMA transfer is going, the DMA transfer will be temporarily paused to allow the HDMA transfer to happen.

After writing this register, DMA will normally take place in the *middle* of the following instruction, just after its opcode is read from memory.

After the DMA completes, the DMA byte count (DASnL/DASnH) will be equal to 0, and the DMA source address (A1TnL/A1TnH) will have been incremented by the number of bytes read if address increment was used (DMAPn), though the bank address (A1Bn) will not change, so the source address will wrap at a bank boundary. This means:

- Since it is automatically returned to 0, a DMA byte count must normally be rewritten before each DMA.
- If the source data for the next DMA begins at the end of the previous one, it may not be necessary to set a new source address.

### HDMAEN - Enable HDMA transfers ($420C write)

---

```
7  bit  0
---- ----
7654 3210
|||| ||||
|||| |||+- Channel 0 HDMA enable
|||| ||+-- Channel 1 HDMA enable
|||| |+--- Channel 2 HDMA enable
|||| +---- Channel 3 HDMA enable
|||+------ Channel 4 HDMA enable
||+------- Channel 5 HDMA enable
|+-------- Channel 6 HDMA enable
+--------- Channel 7 HDMA enable

On power-on: HDMAEN = $00
On reset:    HDMAEN = $00
```

This register enables HDMA for the selected channels.

## DMA channel registers

### DMAPn - DMA/HDMA parameters ($43n0 read/write) (n = 0..7)

---

```
7  bit  0
---- ----
DIxA APPP
|||| ||||
|||| |+++- Transfer pattern (see below)
|||+-+---- Address adjust mode (DMA only):
|||         0:   Increment A bus address after copy
|||         1/3: Fixed
|||         2:   Decrement A bus address after copy
||+------- (Unused)
|+-------- Indirect (HDMA only)
+--------- Direction: 0=Copy from A to B, 1=Copy from B to A

On power-on: DMAPn = $FF
```

The **transfer pattern** (P) controls the address pattern for different register types, and for HDMA also the number of bytes delivered per table entry.

For example: pattern 1 allows a DMA copy to VRAM, which requires writing to two alternating addresses.

DMA transfer patterns

| Pattern | HDMA bytes | B Bus address | Usage example |
| --- | --- | --- | --- |
| 0 | 1 | +0 | WRAM, Mode 7 graphics/tilemap |
| 1 | 2 | +0 +1 | VRAM |
| 2 | 2 | +0 +0 | OAM, CGRAM |
| 3 | 4 | +0 +0 +1 +1 | Scroll positions, Mode 7 parameters |
| 4 | 4 | +0 +1 +2 +3 | Window |
| 5 | 4 | +0 +1 +0 +1 | (Undocumented) |
| 6 | 2 | +0 +0 | (Same as 2, undocumented) |
| 7 | 4 | +0 +0 +1 +1 | (Same as 3, undocumented) |

A fixed **address adjust mode** (A) can be used to fill the DMA target with a single repeated byte of data.

### BBADn - B-bus address ($43n1 read/write) (n = 0..7)

---

```
7  bit  0
---- ----
AAAA AAAA
|||| ||||
++++-++++- Selects a hardware register to read or write from, in the $2100-$21ff range

On power-on: BBADn = $FF
```

This can be used with various [[PPU registers]], and also [[MMIO registers#WMDATA|WMDATA]].

Avoid starting a transfer with BBADn = $00. This causes some CPU revisions to silently fail to start a transfer. Instead, if controlling INIDISP through HDMA, use BBADn = $FF and pattern 1 to write the same value to $21FF and $2100.

### UNUSEDn - Unused byte ($43nB and $43nF read/write) (n = 0..7)

---

```
7  bit  0
---- ----
NNNN NNNN
|||| ||||
++++-++++- One unused byte available through two different addresses

On power-on: UNUSEDn = $FF
```

Seems to have no effect on DMA or HDMA, and this register cannot be used as a source for a DMA fill.

## Configuration registers (DMA)

### A1TnL, A1TnH, A1Bn - DMA Current Address ($43n2, $43n3, $43n4 read/write) (n = 0..7)

---

```
  A1Bn        A1TnH       A1TnL
  $43n4       $43n3       $43n2
7  bit  0   7  bit  0   7  bit  0
---- ----   ---- ----   ---- ----
BBBB BBBB   HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||   |||| ||||
++++-++++---++++-++++---++++-++++- Address on the A bus

On power-on: A1Tn = $FFFFFF
```

The low 16-bits of this address change as the DMA happens, but the bank byte is fixed. DMA can not cross banks.

DMA cannot access A-bus addresses that overlap [[MMIO registers]]: $2100-$21FF, $4000-$41FF, $4200-$421F, $4300-$437F.

[HDMA uses these registers](#HDMA-A1Tn) for its table address instead.

### DASnL, DASnH - DMA Byte-Counter ($43n5, $43n6 read/write) (n = 0..7)

---

```
  DASnH       DASnL
  $43n6       $43n5
7  bit  0   7  bit  0
---- ----   ---- ----
HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||
++++-++++---++++-++++- 16-bit number that indicates how many bytes to transfer

On power-on: DASn = $FFFF
```

A byte count of zero means 65536 bytes.

This byte count is not affected by the DMA pattern.
The SNES will stop before a pattern is completed if it runs out of bytes.

Once the DMA finishes, these registers will be zero.

[HDMA uses these registers](#HDMA-DASn) for its indirect address instead.

## Configuration registers (HDMA)

### A1TnL, A1TnH, A1Bn - HDMA Table Start Address ($43n2, $43n3, $43n4 read/write) (n = 0..7)

```
  A1Bn        A1TnH       A1TnL
  $43n4       $43n3       $43n2
7  bit  0   7  bit  0   7  bit  0
---- ----   ---- ----   ---- ----
BBBB BBBB   HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||   |||| ||||
++++-++++---++++-++++---++++-++++- 24-bit little-endian address on the A bus

On power-on: A1Tn = $FFFFFF
```

These registers control where the channel's HDMA table is. During initiation of HDMA this address gets copied into [A2AnL](#A2AnL)/[A2AnH](#A2AnH) ($43n8/$43n9).

### DASBn - Indirect HDMA Bank ($43n7 read/write) (n = 0..7)

```
7  bit  0
---- ----
BBBB BBBB
|||| ||||
++++-++++- High byte (bank) of HDMA indirect address

On power-on: DASBn = $FF
```

This must be set manually by the program to control the indirect HDMA bank. (DASnL/DASnH are automatically updated from the table.)

## Other HDMA registers

These keep track of each channel's state as HDMA is happening.

### DASnL, DASnH, DASBn - Indirect HDMA Address ($43n5, $43n6, $43n7 read/write) (n = 0..7)

---

```
  DASBn       DASnH       DASnL
  $43n7       $43n6       $43n5
7  bit  0   7  bit  0   7  bit  0
---- ----   ---- ----   ---------
BBBB BBBB   HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||   |||| ||||
++++-++++---++++-++++---++++-++++- The current indirect DMA address.

On power-on: DASn = $FFFFFF
```

With indirect HDMA, if the repeat bit is set in the table entry, then the SNES will continue to read increasing addresses starting from the one given in the table, using these registers to keep track of where it currently is.

The low 16 bits are automatically copied from the table, but the bank byte [DASBn](#DASBn) must be manually set by the program. The bank byte is fixed, and HDMA will not cross banks.

### A2AnL, A2AnH - HDMA Table Current Address ($43n8, $43n9 read/write) (n = 0..7)

---

```
  A2AnH       A2AnL
  $43n9       $43n8
7  bit  0   7  bit  0
---- ----   ---- ----
HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||
++++-++++---++++-++++- Low 16 bits of the current address within the HDMA table

On power-on: A2An = $FFFF
```

Bank byte is taken from $43n4, as it does not change.

### NLTRn - HDMA Line-Counter ($43nA read/write) (n = 0..7)

---

```
7  bit  0
---- ----
RLLL LLLL
|||| ||||
|+++-++++- Number of scanlines left
+--------- Repeat flag

On power-on: NLTRn = $FF
```

Automatically loaded from the table. Scanline count is decremented every scanline until it hits zero.

## HDMA table format

HDMA tables specify what values to write to the selected B bus register, as well as which scanlines to write the values on. The tables can either directly contain the values (Direct mode) or specify 16-bit pointers that are then used to get the values (Indirect mode).

### Direct HDMA table entries

---

```
1 byte - Line count, and repeat mode
N bytes - Data
```

The number of bytes in each Data section is determined by the pattern chosen for the channel in register $43n0.

- If repeat mode is off, one pattern's worth of bytes are written, then the SNES waits for the specified number of scanlines before continuing onto the next table entry.
- If repeat mode is on, then the total size of the data section is the number of scanlines multiplied by the number of bytes in the pattern. One pattern's worth of bytes are written for however many scanlines indicated in the table.

### Indirect HDMA table entries

---

```
1 byte - Line count, and repeat mode
2 bytes - Pointer to access the data through
```

- If repeat mode is off, one pattern's worth of bytes are written from an address starting from the pointer given.
- If repeat mode is on, the SNES continues to progress through the bytes that the pointer points to, for however many scanlines are indicated in the table.

### Line count, repeat mode byte

---

Possible values for the "Line count, and repeat mode" byte are as follows:

- $00: Stop processing HDMA on that channel for the rest of the frame.
- $01-$80: Write once, then wait for X scanlines
- $81-$FF: Write every scanline for X-$80 scanlines, repeat mode

Note that the first entry in the HDMA table is executed at the end of scanline 0, which is always hidden in blanking.

For example: if you wanted the second HDMA entry to apply to scanline 24, use X=23 for the first entry, which causes the next entry to apply at the end of scanline 23, so the change will be visible on scanline 24.

## See Also

- [[DMA examples]]
- [[HDMA examples]]

## References

- [SnesLab HDMA](https://sneslab.net/wiki/HDMA) - documenting which registers can be used with HDMA.
