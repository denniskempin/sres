# SNES APU SPC700 I/O Ports

```text
00F0h - TEST - Testing functions (W)
  0    Timer-Enable     (0=Normal, 1=Timers don't work)
  1    RAM Write Enable (0=Disable/Read-only, 1=Enable SPC700 & S-DSP writes)
  2    Crash SPC700     (0=Normal, 1=Crashes the CPU)
  3    Timer-Disable    (0=Timers don't work, 1=Normal)
  4-5  Waitstates on RAM Access         (0..3 = 0/1/4/9 cycles) (0=Normal)
  6-7  Waitstates on I/O and ROM Access (0..3 = 0/1/4/9 cycles) (0=Normal)
```

Default setting is 0Ah, software should never change this register. Normal memory access time is 1 cycle (adding 0/1/4/9 waits gives access times of 1/2/5/10 cycles). Using 4 or 9 waits doesn't work with some opcodes (0 or 1 waits seem to work stable).

Internal cycles (those that do not access RAM, ROM, nor I/O) are either using the RAM or I/O access time (see notes at bottom of this chapter).

```text
00F1h - CONTROL - Timer, I/O and ROM Control (W)
  0-2  Timer 0-2 Enable (0=Disable, set TnOUT=0 & reload divider, 1=Enable)
  3    Not used
  4    Reset Port 00F4h/00F5h Input-Latches (0=No change, 1=Reset to 00h)
  5    Reset Port 00F6h/00F7h Input-Latches (0=No change, 1=Reset to 00h)
        Note: The CPUIO inputs are latched inside of the SPC700 (at time when
        the Main CPU writes them), above two bits allow to reset these latches.
  6    Not used
  7    ROM at FFC0h-FFFFh (0=RAM, 1=ROM) (writes do always go to RAM)
```

On power on or reset, it seems to be set to B0h.

```text
00F2h - DSPADDR - DSP Register Index (R/W)
  0-7  DSP Register Number (usually 00h..7Fh) (80h..FFh are read-only mirrors)

00F3h - DSPDATA - DSP Register Data (R/W)
  0-7  DSP Register Data (read/write the register selected via Port 00F2h)

00F4h - CPUIO0 - CPU Input and Output Register 0 (R and W)
00F5h - CPUIO1 - CPU Input and Output Register 1 (R and W)
00F6h - CPUIO2 - CPU Input and Output Register 2 (R and W)
00F7h - CPUIO3 - CPU Input and Output Register 3 (R and W)
```

These registers are used in communication with the 5A22 S-CPU. There are eight total registers accessed by these four addresses: four write-only output ports to the S-CPU and four read-only input ports from the S-CPU.

```text
  0-7  Data written to/read from corresponding register on main 5A22 CPU
```

If the SPC700 writes to an output port while the S-CPU is reading it, the S-CPU will read the logical OR of the old and new values. Possibly the same thing happens the other way around, but the details are unknown?

```text
00F8h - AUXIO4 - External I/O Port P4 (S-SMP Pins 34-27) (R/W) (unused)
00F9h - AUXIO5 - External I/O Port P5 (S-SMP Pins 25-18) (R/W) (unused)
```

Writing changes the output levels. Reading normally returns the same value as the written value; unless external hardware has pulled the pins LOW or HIGH.

Reading from AUXIO5 additionally produces a short /P5RD read signal (Pin17).

```text
  0-7  Input/Output levels (0=Low, 1=High)
```

In the SNES, these pins are unused (not connected), so the registers do effectively work as if they'd be "RAM-like" general purpose storage registers.

```text
00FAh - T0DIV - Timer 0 Divider (for 8000Hz clock source) (W)
00FBh - T1DIV - Timer 1 Divider (for 8000Hz clock source) (W)
00FCh - T2DIV - Timer 2 Divider (for 64000Hz clock source) (W)
  0-7  Divider (01h..FFh=Divide by 1..255, or 00h=Divide by 256)
```

If timers are enabled (via Port 00F1h), then the TnOUT registers are incremented at the selected rate (ie. the 8kHz or 64kHz clock source, divided by the selected value).

```text
00FDh - T0OUT - Timer 0 Output (R)
00FEh - T1OUT - Timer 1 Output (R)
00FFh - T2OUT - Timer 2 Output (R)
  0-3  Incremented at the rate selected via TnDIV (reset to 0 after reading)
  4-7  Not used (always zero)
```

#### SPC700 Waitstates on Internal Cycles

Below lists the number of I/O-Waitstates applied on Internal Cycles of SPC700 opcodes 00h..FFh (that implies: any further Internal Cycles have RAM-Waitstates).

```text
  00..1F  0,3,0,1,0,0,0,1,0,0,1,0,0,1,0,2, 2,3,0,3,1,1,1,1,0,0,0,1,0,0,0,1
  20..3F  0,3,0,1,0,0,0,1,0,0,1,0,0,1,3,2, 0,3,0,3,1,1,1,1,0,0,0,1,0,0,0,3
  40..5F  0,3,0,1,0,0,0,1,0,0,0,0,0,1,0,3, 2,3,0,3,1,1,1,1,0,0,0,1,0,0,0,0
  60..7F  0,3,0,1,0,0,0,1,0,1,0,0,0,1,2,1, 0,3,0,3,1,1,1,1,1,1,1,1,0,0,0,1
  80..9F  0,3,0,1,0,0,0,1,0,0,1,0,0,0,1,0, 2,3,0,3,1,1,1,1,0,0,1,1,0,0,10,3
  A0..BF  1,3,0,1,0,0,0,1,0,0,0,0,0,0,1,1, 0,3,0,3,1,1,1,1,0,0,1,1,0,0,1,1
  C0..DF  1,3,0,1,0,0,0,1,0,0,1,0,0,0,1,7, 2,3,0,3,1,1,1,1,0,1,0,1,0,0,4,1
  E0..FF  0,3,0,1,0,0,0,1,0,0,0,0,0,1,1,0, 0,3,0,3,1,1,1,1,0,1,0,1,0,0,3,0
  Note: For conditional jumps (with condition=true), add 2 additional cylces.
```

That is, the above list_entries are meant to be used like so:

```text
  number_of_I/O_waits = list_entry
  number_of_RAM_waits = total_number_of_internal_cycles - list_entry
```

For example, Opcode 00h (NOP) has one internal cycle (and it's having RAM timings). Opcode 01h (TCALL) has 3 internal cycles (and all 3 of them have I/O timings).
