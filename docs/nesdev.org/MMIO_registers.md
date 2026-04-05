---
title: "MMIO registers"
source_url: "https://snes.nesdev.org/wiki/MMIO_registers"
pageid: 39
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

This page covers 5A22, APU, and WRAM memory-mapped registers.

- For PPU-related MMIO registers **$2100-$213F**, see [[PPU registers]].
- For DMA-related MMIO registers **$4300-$437F**, see [[DMA registers]].
- For a complete summary table of all MMIO registers, see [[MMIO register table]].

5A22, APU, and WRAM register summary

| Name | Address | Bits | Type | Notes |
| --- | --- | --- | --- | --- |
| [APUIO0](#APUIOn) [APUIO1](#APUIOn) [APUIO2](#APUIOn) [APUIO3](#APUIOn) | $2140 $2141 $2142 $2143 | DDDD DDDD | RW8 | Data to/from APU. |
| [WMDATA](#WMDATA) | $2180 | DDDD DDDD | RW8 | Data to/from S-WRAM, increments WMADD. |
| [WMADDL](#WMADD) [WMADDM](#WMADD) [WMADDH](#WMADD) | $2181 $2182 $2183 | LLLL LLLL MMMM MMMM .... ...H | W24 | S-WRAM address for WMDATA access. |
| [JOYOUT](#JOYOUT) | $4016 | .... ...D | W8 | Output to joypads (latches standard controllers). |
| [JOYSER0](#JOYSER0) | $4016 | .... ..DD | R8 | Input from joypad 1. |
| [JOYSER1](#JOYSER1) | $4017 | ...1 11DD | R8 | Always 1 (1), input from joypad 2 (D). |
| [NMITIMEN](#NMITIMEN) | $4200 | N.VH ...J | W8 | Vblank NMI enable (N), timer IRQ mode (VH), joypad auto-read enable (J). |
| [WRIO](#WRIO) | $4201 | 21DD DDDD | W8 | Joypad port 2 I/O (2), joypad port 1 I/O (1), unused I/O (D). |
| [WRMPYA](#WRMPYA) | $4202 | DDDD DDDD | W8 | Unsigned multiplication factor A. |
| [WRMPYB](#WRMPYB) | $4203 | DDDD DDDD | W8 | Unsigned multiplication factor B, starts 8-cycle multiplication. |
| [WRDIVL](#WRDIV) [WRDIVH](#WRDIV) | $4204 $4205 | LLLL LLLL HHHH HHHH | W16 | Unsigned dividend. |
| [WRDIVB](#WRDIVB) | $4206 | DDDD DDDD | W8 | Unsigned divisor, starts 16-cycle division. |
| [HTIMEL](#HTIME) [HTIMEH](#HTIME) | $4207 $4208 | .... ...H LLLL LLLL | W16 | H counter target for timer IRQ. |
| [VTIMEL](#VTIME) [VTIMEH](#VTIME) | $4209 $420A | .... ...H LLLL LLLL | W16 | V counter target for timer IRQ. |
| [[DMA registers#MDMAEN|MDMAEN]] | $420B | 7654 3210 | W8 | DMA enable. |
| [[DMA registers#HDMAEN|HDMAEN]] | $420C | 7654 3210 | W8 | HDMA enable. |
| [MEMSEL](#MEMSEL) | $420D | .... ...F | W8 | FastROM enable (F). |
| [RDNMI](#RDNMI) | $4210 | N... VVVV | R8 | Vblank NMI flag (N), CPU version (V). |
| [TIMEUP](#TIMEUP) | $4211 | T... .... | R8 | Timer IRQ flag (T). |
| [HVBJOY](#HVBJOY) | $4212 | VH.. ...J | R8 | Vblank flag (V), hblank flag (H), joypad auto-read in-progress flag (J). |
| [RDIO](#RDIO) | $4213 | 21DD DDDD | R8 | Joypad port 2 I/O (2), joypad port 1 I/O (1), unused I/O (D). |
| [RDDIVL](#RDDIV) [RDDIVH](#RDDIV) | $4214 $4215 | LLLL LLLL HHHH HHHH | R16 | Unsigned quotient. |
| [RDMPYL](#RDMPY) [RDMPYH](#RDMPY) | $4216 $4217 | LLLL LLLL HHHH HHHH | R16 | Unsigned product or unsigned remainder. |
| [JOY1L](#JOY1) [JOY1H](#JOY1) [JOY2L](#JOY2) [JOY2H](#JOY2) [JOY3L](#JOY3) [JOY3H](#JOY3) [JOY4L](#JOY4) [JOY4H](#JOY4) | $4218 $4219 $421A $421B $421C $421D $421E $421F | LLLL LLLL HHHH HHHH | R16 | 16-bit joypad auto-read result (first read high to last read low). |
| [[MMIO register table/MMIO|table source]] | | | | |

Register types:

- **R** - Readable
- **W** - Writeable
- **8** - 8-bit access only
- **16** - 8-bit access to either address, or 16-bit access to the lower address.
- **24** - 8-bit or 16-bit access to 3 registers.

## Interrupts

### NMITIMEN - Interrupts and Joypad reading ($4200 write)

---

```
7  bit  0
---- ----
N.VH ...J
| ||    |
| ||    +- Joypad auto-read enable
| ++------ H/V timer IRQ:
|           00 = Disable timer
|           01 = IRQ when H counter == HTIME
|           10 = IRQ when V counter == VTIME and H counter == 0
|           11 = IRQ when V counter == VTIME and H counter == HTIME
+--------- Vblank NMI enable

On power-on: NMITIMEN = $00
On reset:    NMITIMEN = $00
```

- If the vblank NMI is enabled, then an NMI will occur at the start of vblank. More precisely, an NMI occurs whenever the bitwise AND of vblank NMI enable and RDNMI's vblank flag becomes 1. As a result, it is possible to get multiple NMIs for a single vblank if vblank NMI is disabled and enabled again while the vblank flag is still 1. This can be prevented by reading RDNMI, which clears the vblank flag.
- TODO: Details on the exact conditions for an IRQ.
- Auto-read runs shortly after the beginning of vblank, reading the current state of the two controller ports into the [JOY1-4](#JOY1) registers without halting the CPU. It is equivalent to writing 1 and then 0 to [JOYOUT](#JOYOUT) (used for latching standard controllers) and then reading [JOYSER0](#JOYSER0) and [JOYSER1](#JOYSER1) 16 times each. The controllers are then left in this state so that additional reads can be done manually from JOYSER0 and JOYSER1 to get any additional data, though this is unnecessary for standard controllers. Auto-read takes approximately 3 scanlines, during which the JOYOUT, JOYSER0, and JOYSER1 registers should not be manually accessed.

### Screen timer values

---

#### HTIMEL, HTIMEH - H timer target ($4207, $4208 write)

```
  HTIMEH      HTIMEL
  $4208       $4207
7  bit  0   7  bit  0
---- ----   ---- ----
.... ...H   LLLL LLLL
        |   |||| ||||
        +---++++-++++- H counter target for timer IRQ

On power-on: HTIME = $1FF
```

Note that setting a value larger than the maximum H counter value of 339 will prevent the timer's H condition from being met.

#### VTIMEL, VTIMEH - V timer target ($4209, $420A write)

```
  VTIMEH      VTIMEL
  $420A       $4209
7  bit  0   7  bit  0
---- ----   ---- ----
.... ...H   LLLL LLLL
        |   |||| ||||
        +---++++-++++- V counter target for timer IRQ

On power-on: VTIME = $1FF
```

Note that setting a value larger than the maximum V counter value will prevent the timer's V condition from being met. The maximum depends on the region (261 for NTSC, 311 for PAL) and interlacing (1 additional scanline every other frame).

### Status

---

#### RDNMI - Vblank flag and CPU version ($4210 read)

```
7  bit  0
---- ----
Nxxx VVVV
|||| ||||
|||| ++++- CPU version
|+++------ (Open bus)
+--------- Vblank flag

On power-on: RDNMI = RDNMI & $7F
On reset:    RDNMI = RDNMI & $7F
On read:     RDNMI = RDNMI & $7F
```

The vblank flag is set at the start of vblank and cleared at the end of vblank or on read.

ERRATA:

- Nintendo stopped incrementing the version field after 2. S-CPU A, S-CPU B and S-CPUN A all report CPU version 2.

#### TIMEUP - Timer flag ($4211 read)

```
7  bit  0
---- ----
Txxx xxxx
|||| ||||
|+++-++++- (Open bus)
+--------- Timer flag

On power-on: TIMEUP = TIMEUP & $7F
On reset:    TIMEUP = TIMEUP & $7F
On read:     TIMEUP = TIMEUP & $7F
```

The timer flag is set when the timer condition specified in NMITIMEN becomes true and is cleared on read.

#### HVBJOY - Screen and Joypad status ($4212 read)

```
7  bit  0
---- ----
VHxx xxxJ
|||| ||||
|||| |||+- Joypad auto-read in-progress flag
||++-+++-- (Open bus)
|+-------- Hblank flag
+--------- Vblank flag
```

- J - Set during joypad auto-read.
- H - Set during horizontal blank period.
- V - Set during vertical blank period.

When enabled via [NMITIMEN](#NMITIMEN), auto-read begins between H=32.5 and H=95.5 of the first vblank scanline, and ends 4224 master cycles later.[[1]](#cite_note-1)

## APU

### APUIOn - Data-to-APU register n ($214n write) (n = 0..3)

```
7  bit  0
---- ----
DDDD DDDD
|||| ||||
++++-++++- Data to APU
```

### APUIOn - Data-from-APU register n ($214n read) (n = 0..3)

```
7  bit  0
---- ----
DDDD DDDD
|||| ||||
++++-++++- Data from APU
```

When the SPC700 reads from $F4+n in its address space, it receives the last value written to APUIOn. When APUIOn is read, the value received is the last one written by the SPC700 to $F4+n. If APUIOn is read while its corresponding $F4+n register is being written, the value read will be the bitwise OR of the old and new values.

These registers are mirrored across $2140-217F.

## WRAM

### WMDATA - S-WRAM data access ($2180 read/write)

---

```
7  bit  0
---- ----
DDDD DDDD
|||| ||||
++++-++++- S-WRAM data

On write: [WMADD] = value
          WMADD = WMADD + 1
```

This register's presence on the peripheral bus allows DMA between S-WRAM and another, different source.

Because DMA simultaneously accesses the source and destination, S-WRAM cannot succesfully be both because it cannot simultaneously read from and write to itself. DMA from S-WRAM to this register has no effect, and DMA from this register to S-WRAM writes open bus. In both cases, the address is not incremented.

### WMADDL, WMADDM, WMADDH - S-WRAM address ($2181, $2182, $2183 write)

---

```
  WMADDH      WMADDM      WMADDL
  $2183       $2182       $2181
7  bit  0   7  bit  0   7  bit  0
---- ----   ---- ----   ---- ----
.... ...H   MMMM MMMM   LLLL LLLL
        |   |||| ||||   |||| ||||
        +---++++-++++---++++-++++- S-WRAM address for WMDATA
```

DMA from S-WRAM to these registers has no effect.

## ROM

### MEMSEL - ROM access speed ($420D write)

```
 7  bit  0
 ---- ----
 .... ...F
         |
         +- FastROM enable

On power-on: MEMSEL = $00
```

If enabled, ROM access to banks $80-FF takes only 6 system clock cycles instead of 8.

## Joypads

### Joypad NES-style interface

---

#### JOYOUT - Joypad output ($4016 write)

```
7  bit  0
---- ----
.... .210
      |||
      ||+- OUT0
      ++-- OUT2-1 (not connected)
```

OUT0 is used by standard controllers to latch the current button state. OUT2-1 are not connected in standard consoles, but may be used in the [Super Famicom Box](https://snes.nesdev.org/w/index.php?title=Super_Famicom_Box&action=edit&redlink=1 "Super Famicom Box (page does not exist)") hotel system.

#### JOYSER0 - Joypad serial data port 1 ($4016 read)

```
7  bit  0
---- ----
xxxx xxDD
|||| ||||
|||| ||++- Joypad port 1 data 2-1
++++-++--- (Open bus)

On read: Joypad port 1 is clocked (via joypad 1 /OE)
```

#### JOYSER1 - Joypad serial data port 2 ($4017 read)

```
7  bit  0
---- ----
xxx1 11DD
|||| ||||
|||| ||++- Joypad port 2 data 2-1
|||+-++--- Joypad 2 D4-2 (always 1)
+++------- (Open bus)

On read: Joypad port 2 is clocked (via joypad 2 /OE)
```

The CPU has 5 joypad 2 inputs. Joypad port 2's data 2 and 1 pins connect to D1-0, while D4-2 are tied to ground (and thus read as 1).

### Joypad I/O interface

---

#### WRIO - Write I/O ($4201 write)

```
7  bit  0
---- ----
21DD DDDD
|||| ||||
||++-++++- CPU I/O D5-0 (not connected)
|+-------- Joypad port 1 I/O
+--------- Joypad port 2 I/O, and
           PPU /EXTLATCH light pen input

On power-on: WRIO = $FF
```

Used by the [[Multitap]] to select reading of controllers 2/3 vs. 4/5.

#### RDIO - Read I/O ($4213 read)

```
7  bit  0
---- ----
21DD DDDD
|||| ||||
||++-++++- CPU I/O D5-0 (not connected)
|+-------- Joypad port 1 I/O
+--------- Joypad port 2 I/O
```

The I/O pins allow bidirectional communication between the CPU and joypads on a single wire per bit. Either side is able to set the bits to 0, so to read the value being sent by the other side, the reader must set its own corresponding bits to 1 before reading.

The not-connected D5-0 bits can be used as general-purpose storage on standard consoles, but are used in the [Super Famicom Box](https://snes.nesdev.org/w/index.php?title=Super_Famicom_Box&action=edit&redlink=1 "Super Famicom Box (page does not exist)") hotel system to communicate with its HD64180 CPU.

Joypad port 2's I/O bit is also connected to the PPU's /EXTLATCH input, allowing the PPU's H and V counters to be latched when this bit is set to 0 by the CPU or joypad. This is intended to support a light pen or gun device. This should normally be set to 1 by the CPU to allow the counters to be latched. (See [[PPU registers#OPHCT]])

### Auto-read results

---

#### JOY1L, JOY1H - Joypad port 1 data 1 ($4218, $4219 read)

```
  JOY1H       JOY1L
  $4219       $4218
7  bit  0   7  bit  0
---- ----   ---- ----
HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||
++++-++++---++++-++++- Joypad port 1 data 1 (first read on left to last read on right)
```

#### JOY2L, JOY2H - Joypad port 2 data 1 ($421A, $421B read)

```
  JOY2H       JOY2L
  $421B       $421A
7  bit  0   7  bit  0
---- ----   ---- ----
HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||
++++-++++---++++-++++- Joypad port 2 data 1 (first read on left to last read on right)
```

#### JOY3L, JOY3H - Joypad port 1 data 2 ($421C, $421D read)

```
  JOY3H       JOY3L
  $421D       $421C
7  bit  0   7  bit  0
---- ----   ---- ----
HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||
++++-++++---++++-++++- Joypad port 1 data 2 (first read on left to last read on right)
```

#### JOY4L, JOY4H - Joypad port 2 data 2 ($421E, $421F read)

```
  JOY4H       JOY4L
  $421F       $421E
7  bit  0   7  bit  0
---- ----   ---- ----
HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||
++++-++++---++++-++++- Joypad port 2 data 2 (first read on left to last read on right)
```

## Math

### Multiplication

---

#### WRMPYA - Multiplication factor A ($4202 write)

```
7  bit  0
---- ----
DDDD DDDD
|||| ||||
++++-++++- Multiplication factor (8-bit unsigned)

On power-on: WRMPYA = $FF
```

#### WRMPYB - Multiplication factor B ($4203 write)

```
7  bit  0
---- ----
DDDD DDDD
|||| ||||
++++-++++- Multiplication factor (8-bit unsigned)

On write: Begins multiplication process
```

The multiplication process takes up to 8 cycles and the result is written to RDMPY as it goes. See 5A22 [[Multiplication]] for more information.

### Division

---

#### WRDIVL, WRDIVH - Dividend ($4204, $4205 write)

```
  WRDIVH      WRDIVL
  $4205       $4204
7  bit  0   7  bit  0
---- ----   ---- ----
HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||
++++-++++---++++-++++- Dividend (16-bit unsigned)

On power-on: WRDIV = $FFFF
```

#### WRDIVB - Divisor ($4206 write)

```
7  bit  0
---- ----
DDDD DDDD
|||| ||||
++++-++++- Divisor (8-bit unsigned)

On write: Begins division process
```

The division process takes up to 16 CPU cycles and the result is written to RDDIV (quotient) and RDMPY (remainder) as it goes. Dividing by 0 results in a quotient of $FFFF and a remainder equal to the dividend (WRDIV). See 5A22 [[Division]] for more information.

### Result

---

#### RDDIVL, RDDIVH - Quotient ($4214, $4215 read)

```
  RDDIVH      RDDIVL
  $4215       $4214
7  bit  0   7  bit  0
---- ----   ---- ----
HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||
++++-++++---++++-++++- Quotient (16-bit unsigned)
```

#### RDMPYL, RDMPYH - Product or Remainder ($4216, $4217 read)

```
  RDMPYH      RDMPYL
  $4217       $4216
7  bit  0   7  bit  0
---- ----   ---- ----
HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||
++++-++++---++++-++++- Product (16-bit unsigned), or
                       Remainder (16-bit unsigned)
```

## References

1. [↑](#cite_ref-1) Auto Joypad Read timings from Anomie's timing.txt document.
