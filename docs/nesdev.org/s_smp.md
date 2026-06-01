---
title: "S-SMP"
source_url: "https://snes.nesdev.org/wiki/S-SMP"
pageid: 106
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The **SHVC-SOUND** module of the SNES includes several components:[[1]](#cite_note-1)

- **S-SMP**: Sony sub CPU
  - Sony SPC-700 processor, clocked at 1.024MHz (see [[SPC-700 instruction set]])
  - 3 timers, two running at 8kHz and one at 64kHz
  - [[MMIO registers#APUIOn|APUIO]] and [CPUIOx](#CPUIOx) registers for communication between the 65816 S-CPU and the SPC700 code
  - [IPL Boot ROM](#IPL_Boot_ROM)
- **S-DSP**: Sony sound chip (see [[S-DSP registers]])
  - Digital signal processor that generates the sound data
  - 8 voice channels
  - Decodes [[BRR samples]] at a variable sample rate using 4-point Gaussian interpolation
  - FIR filtered echo buffer
  - Manages the clock of the S-DSP, S-SMP and DAC
    - Clocked by a 24.576MHz ceramic resonator
    - Generates 1 stereo sample every 768 resonator cycles
    - Internally clocked at 3.072MHz
  - Bus Arbiter for the Audio-RAM. The Audio-RAM is time-shared between the S-SMP and S-DSP, with 1 S-SMP memory access for every 2nd S-DSP memory access.
- **Audio-RAM**: 64KiB RAM from two 32K x 8bit PSRAM chips. (Also known as ARAM.)
- 16-bit stereo DAC
  - Digital to analog converter, outputting the analog audio signal from the DSP's stereo 16-bit 32000 Hz digital audio. Actual rate varies from console to console anywhere from 32000 Hz to 32160 Hz[[2]](#cite_note-2)
  - The DAC is an NEC µPD6376, NEC 6376 or NEC 6379A (see [[Version differences#DAC|Version\_differences#DAC]])
- Stereo Amplifier
  - Mixes DAC output with the [[Cartridge connector]] analog audio input and the [EXT connector](https://snes.nesdev.org/w/index.php?title=EXT_connector&action=edit&redlink=1 "EXT connector (page does not exist)") analog audio input
  - The amplifier can be disabled by the S-SMP's /MUTE pin (bit 6 of the [[S-DSP registers#FLG|S-DSP FLG register]])
  - The output of the stereo amplifier is mixed into a mono output for the RF modulator and EXT connector

The SPC-700 is an 8-bit CPU that is almost like an extended MOS-6502.

The SPC-700 driven by a 1.024 MHz clock, though the SPC-700 instruction timing is very similar to a 6502 in that each instruction takes at least 2 cycles. The exact clock rate is independent from the rest of the SNES, and may drift slightly with temperature. The nominal 32000 Hz sample rate is actually 32 clocks per sample.

Related reference:

- [[SPC-700 instruction set]]
- [[S-DSP registers]]
- [[DSP envelopes]]
- [[BRR samples]]

The term SPC is also used to describe a [SPC file format](https://snes.nesdev.org/w/index.php?title=SPC_file_format&action=edit&redlink=1 "SPC file format (page does not exist)") (.SPC) for storing SNES music. The S-DSP is not to be confused with the [[DSP-1]] cartridge expansion hardware. The term S-SMP is often contrasted with [[S-CPU]] when describing communication with the SNES main CPU.

## Memory Layout

64 kilobytes of RAM are mapped across the 16-bit memory space of the SPC-700. Some regions of this space are overlaid with special hardware functions.

| Range | Note |
| --- | --- |
| $0000-00EF | Zero Page RAM |
| $00F0-00FF | Sound CPU Registers |
| $0100-01FF | Stack Page RAM |
| $0200-FFBF | RAM |
| $FFC0-FFFF | IPL ROM or RAM |

The region at **$FFC0-FFFF** will normally read from the 64-byte IPL ROM, but the underlying RAM can always be written to, and the high bit of the Control register **$F1** can be cleared to unmap the IPL ROM and allow read access to this RAM.

## Registers

Aside from the SPC-700 CPU registers (see: [[SPC-700 instruction set]]), there are a collection of memory-mapped registers in the last 16 bytes of the zero-page.

S-SMP register summary

| Name | | Address | Bits | Type | Notes |
| --- | --- | --- | --- | --- | --- |
| [TEST](#TEST) | Test | $F0 | IIEE TRWH | W8 | Undocumented test register. |
| [CONTROL](#CONTROL) | Control | $F1 | I.CC .210 | W8 | Enable IPL ROM (I), Clear data ports (C), timer enable (2,1,0). |
| [DSPADDR](#DSPADDR) | Register Address | $F2 | RAAA AAAA | RW8 | Selects a DSP register address. |
| [DSPDATA](#DSPDATA) | Register Data | $F3 | DDDD DDDD | RW8 | Reads or writes data to the selected DSP address. |
| [CPUIO0](#CPUIO) | Port 0 | $F4 | DDDD DDDD | RW8 | Reads or writes data to [[MMIO registers#APUIO0|APUIO0]]. |
| [CPUIO1](#CPUIO) | Port 1 | $F5 | DDDD DDDD | RW8 | Reads or writes data to [[MMIO registers#APUIO1|APUIO1]]. |
| [CPUIO2](#CPUIO) | Port 2 | $F6 | DDDD DDDD | RW8 | Reads or writes data to [[MMIO registers#APUIO2|APUIO2]]. |
| [CPUIO3](#CPUIO) | Port 3 | $F7 | DDDD DDDD | RW8 | Reads or writes data to [[MMIO registers#APUIO3|APUIO3]]. |
|  | --- | $F8 | .... .... | RW8 | Unused (normal RAM). |
|  | --- | $F9 | .... .... | RW8 | Unused (normal RAM). |
| [T0TARGET](#TxTARGET) | Timer 0 | $FA | TTTT TTTT | W8 | 8KHz timer 0 interval. |
| [T1TARGET](#TxTARGET) | Timer 1 | $FB | TTTT TTTT | W8 | 8KHz timer 1 interval. |
| [T2TARGET](#TxTARGET) | Timer 2 | $FC | TTTT TTTT | W8 | 64KHz timer 2 interval. |
| [T0OUT](#TxOUT) | Counter 0 | $FD | 0000 CCCC | R8 | Timer 0 count-up. |
| [T1OUT](#TxOUT) | Counter 1 | $FE | 0000 CCCC | R8 | Timer 1 count-up. |
| [T2OUT](#TxOUT) | Counter 2 | $FF | 0000 CCCC | R8 | Timer 2 count-up. |
| ([[APU register table/SMP|table source]]) | | | | | |

Write-only registers will read back as $00.

### TEST - Undocumented test register ($F0, write-only)

```
7  bit  0
---- ----
IIEE TRWH
|||| ||||
|||| |||+- Halt timers
|||| ||+-- RAM writable
|||| |+--- Disable RAM reads
|||| +---- Enable timers
||++------ External wait state
++-------- Internal wait state

On power-on: TEST = $0A
```

**DO NOT write to this register!!!**

- Setting any of the wait state bits will slow the clock speed of the SPC-700 processor.
- Changing the wait state bits can cause crashes on real hardware.[[3]](#cite_note-3)
- The TEST register can disable Audio-RAM reads and/or writes.
- The TEST register can disable the timers.

This undocumented register responds to writes only when the P flag is clear.

### CONTROL - Control register ($F1, write-only)

```
7  bit  0
---- ----
I.CC .210
| ||  |||
| ||  ||+- Enable timer 0
| ||  |+-- Enable timer 1
| ||  +--- Enable timer 2
| |+------ Clear CPUIO read ports 0 & 1
| +------- Clear CPUIO read ports 2 & 3
+--------- IPL ROM enable

On power-on: CONTROL = $B0
On reset:    CONTROL = $B0
```

This provides a way for the SPC-700 to reset the read ports ($F4-F7) without the SNES CPU having to write them externally. It also starts and stops the 3 timers.

- 0/1/2 (bits 0-2) - Enables each of the 3 timers.
  - A transition from clear to set (0 -> 1) will reset the timer's internal counter and TxOUT to 0.
- C (bit 4) - If set the CPUIO0 and CPUIO1 Data-from-CPU read registers ($F4, $F5) are reset to $00.
  - CPUIOx Data-from-CPU is cleared on any CONTROL write with a clear bit set.
  - CPUIOx is only cleared on CONTROL writes. Future APUIO writes to the APU will not be changed.
  - The 65816 S-CPU [[MMIO registers#APUIOn|APUIOn]] Data-from-APU registers will not be changed.
- C (bit 5) - If set the CPUIO2 and CPUIO3 Data-from-CPU read registers ($F6, $F7) are reset to $00.
- I (bit 7) - Enables the IPL ROM if set.
  - This does not disable Audio-RAM writes to the IPL ROM memory addresses.

At reset this register is initialized as if $80 was written to it.

The function of bit 7 enabling the IPL ROM is not documented in the [[SNES Development Manual]].

### DSPADDR - DSP register address ($F2)

```
7  bit  0
---- ----
RAAA AAAA
|||| ||||
|+++-++++- S-DSP register address
+--------- Read only flag
```

Write $F2 to select a DSP register, then a value can be read or written to that DSP register via $F3.

- Writing $F2 with the high bit set will select a DSP register according to the lower 7 bits, but it will be read-only.
- The high bit of $F2 will always read back as 0.

See: [[S-DSP registers#Reading and writing S-DSP registers|Reading and writing S-DSP registers]] and [[S-DSP registers]]

### DSPDATA - DSP register data ($F3)

```
7  bit  0
---- ----
DDDD DDDD
|||| ||||
++++-++++- S-DSP register data

On write: if DSPADDR bit 7 clear: write to S-DSP address DSPADDR

On read: read from S-DSP register address DSPADDR
```

A DSP register can be selected with DSPADDR ($F2), after which it can be read or written at DSPDATA ($F3). Often it is useful to load the register address into A, and the value to send in Y, so that **MOV $F2, YA** can be used to do both in one 16-bit instruction.

The DSP register address space only has 7 bits. The high bit of DSPADDR ($F2), if set, will make the selected register read-only via DSPDATA ($F3).

When initializing the DSP registers for the first time, take care not to accidentally enable echo writeback via [[S-DSP registers#FLG|FLG]], because it will immediately begin overwriting values in RAM.

See: [[S-DSP registers#Reading and writing S-DSP registers|Reading and writing S-DSP registers]] and [[S-DSP registers]]

### CPUIOx - APU-to-Data register x ($F4 - $F7, write)

```
7  bit  0
---- ----
DDDD DDDD
|||| ||||
++++-++++- Data to CPU
```

### CPUIOx - Data-from-CPU register x ($F4 - $F7, read)

```
7  bit  0
---- ----
DDDD DDDD
|||| ||||
++++-++++- Data from CPU
```

These 4 ports allow communication with the SNES CPU. There are 8 stored values, each is a one-way communication written from one side, and readable only from the other side. Each port therefore has two separate one-way values, each seen from only either the SNES CPU or the SPC-700.

If a port is read on the same cycle it is written, an incorrect value will result. For this reason, common practice is to read a port in a loop until the value changes, and then read it once more to ensure the correct value is read. (A single port can be used this way to indicate that a message is ready, and the other 3 ports could be safely read only once with the assumption that the other CPU will not write to them once the ready indication was given.)

At reset these registers are initialized to $00.

### TxTARGET - Timer x target ($FA - $FC, write-only)

```
7  bit  0
---- ----
TTTT TTTT
|||| ||||
++++-++++- Timer target

On power-on: TxTARGET = 0
```

When enabled via $F1, the 3 timers will internally count at a rate of 8 KHz (timers 0,1) or 64 KHz (timer 2), and when this interval value has been exceeded, they will increment their external counter result ($FD-FF) and begin again.

### TxOUT - Timer x output ($FD - $FF, read-only)

```
7  bit  0
---- ----
0000 CCCC
     ||||
     ++++- Timer counter

On power-on: TxOUT = 0
On reset:    TxOUT = 0
On read:     TxOUT = 0
```

The 4-bit result of the three timers counts up every time the interval is reached.

Reading these registers resets each counter to 0 immediately after the read. The upper 4 bits will always read as 0.

## IPL Boot ROM

**See: [[Booting the SPC700]]**

The IPL Boot ROM is a small built-in program responsible for initializing the SPC-700, and making it ready to transfer a program from the SNES CPU, then execute it. It normally resides at $FFC0, with its code beginning at this address, but if desired it can be unmapped using the $F1 control register, and replaced with the underlying RAM.

When the SNES is reset, the SPC-700 will also reset and begin executing the IPL.

IPL might stand for "Initial Program Load".

The high level process of the IPL is described below:

1. **Reset**: Stack pointer = $EF. Zero-page from $00-$EF is set to $00. (Note: this leaves the top 16-bytes of the stack page unused by default.)
2. **Signal ready:** Port 0 = $AA. Port 1 = $BB.
3. **Wait for signal:** Loop until $CC is read from port 0.
4. **Read address:** Read a 2 byte address from port 2 (low) and 3 (high).
5. **Acknowledge:** Read value from port 0 and write it to port 0, confirming the signal was received.
6. **Begin:** Read value from port 1, if 0 begin executing code at address read in **step 4**, otherwise begin reading data (**step 7**).
7. **Begin Transfer:** Loop until read port 0 reads 0. Set 8-bit counter to 0, then proceed to **step 8**.
8. **Transfer Loop:**
   - Read a byte from port 1 and write to the destination address.
   - Write the value read from port 0 to port 0 to acknowledge receipt of the byte.
   - Increment the destination address, and increment the 8-bit counter.
   - Wait until port 0 reads equal to the new counter (SNES will increment its own counter and send it to port 0 to signal the next byte), and repeat **step 8**, otherwise...
   - If port 0 reads greater than the new counter (i.e. SNES increments by 2 or more before writing port 0): write back port 0 to acknowledge, transfer ends and return to **step 4**.

On the SNES side, to load your program into the SPC:

1. Wait for the ready signal: port 0 = $AA, port 1 = $BB (IPL 2).
2. Write the first destination address to ports 2+3, write any non-zero value to port 1, then write $CC to port 0 to begin (IPL 3).
3. Wait for port 0 to read back $CC (IPL 5).
4. Set an 8-bit counter to $00. Write the first byte of data to port 1, then write the counter value ($00) to port 0 (IPL 7). Read port 0 until it is equal to the counter $00.
5. For each byte of data: write the data to port 1, increment the 8-bit counter and write it to port 0, then read port 0 until is becomes equal to the counter. Repeat until finished. (IPL 8)
6. After the last byte is written, write the next destination address to ports 2+3, write $00 to port 1, then increment your counter twice and write it to port 0. (IPL 4)
7. The SPC-700 will begin executing code at the destination address now.

Most often only one data transfer is necessary, but by writing a non-zero value to port 1 in SNES **step 6**, we can instead initiate another transfer at the new address.
When doing this, if your counter is 0 after incrementing twice, increment it a third time to be non-zero before writing it to port 0. This is because a value of 0 in port 0 will also signal the first byte of the transfer in the next step.

It is most common to place the data program at $0200, just above the stack page, and start execution from there, but this is not a requirement.

IPL loading takes about 520 master clocks per byte transferred. This allows about 650 bytes in a 60hz frame, if the CPU is dedicated to this activity.

## References

1. [↑](#cite_ref-1) Super Nintendo (NTSC) Revision 2 Motherboard schematic by Jonathon W. Donaldson
2. [↑](#cite_ref-2) [Forum post](https://forums.nesdev.org/viewtopic.php?t=24610): S-SMP clock speed measurement tool
3. [↑](#cite_ref-3) ares source code, [ares/sfc/smp/timing.cpp](https://github.com/ares-emulator/ares/tree/master/ares/ares/sfc/smp/timing.cpp), by Near

## See Also

- [[SPC-700 instruction set]]
- [[S-DSP registers]]
- [[DSP envelopes]]
- [[BRR samples]]
- [[Booting the SPC700]] - Guide for setting up the SPC program after reset.
- [[Tools]] - Lists tools for building SPC700 code.
- [[APU pinout]] - Chip pinouts.

## Links

- [SPC 700 Documentation](http://snesmusic.org/files/spc700_documentation.html) - Article by Gau.
- [SPC700 Reference](https://wiki.superfamicom.org/spc700-reference) - Superfamicom.org wiki article.
- [How to Write to DSP Registers Without any SPC-700 Code](https://wiki.superfamicom.org/how-to-write-to-dsp-registers-without-any-spc-700-code) - Superfamicom.org wiki, demonstrating that the DSP can be written directly through the IPL.
