# SNES Maths Multiply/Divide

```text
4202h - WRMPYA - Set unsigned 8bit Multiplicand (W)
4203h - WRMPYB - Set unsigned 8bit Multiplier and Start Multiplication (W)
```

Set WRMPYA (or leave it unchanged, if it already contains the desired value), then set WRMPYB, wait 8 clk cycles, then read the 16bit result from Port 4216h-4217h. For some reason, the hardware does additionally set RDDIVL=WRMPYB, and RDDIVH=00h.

```text
4204h - WRDIVL - Set unsigned 16bit Dividend (lower 8bit) (W)
4205h - WRDIVH - Set unsigned 16bit Dividend (upper 8bit) (W)
4206h - WRDIVB - Set unsigned 8bit Divisor and Start Division (W)
```

Set WRDIVL/WRDIVH (or leave it unchanged, if it already contains the desired value), then set WRDIVB, wait 16 clk cycles, then read the 16bit result and/or 16bit remainder from Port 4214h-4217h.

Division by zero returns Result=FFFFh, Remainder=Dividend. Note: Almost all commercial SNES games are zero-filling I/O ports upon initialization, thereby causing division by zero (so, debuggers should ignore division errors).

```text
4214h - RDDIVL - Unsigned Division Result (Quotient) (lower 8bit) (R)
4215h - RDDIVH - Unsigned Division Result (Quotient) (upper 8bit) (R)
```

See Ports 4204h-4206h (divide). Destroyed by 4203h (multiply).

```text
4216h - RDMPYL - Unsigned Division Remainder / Multiply Product (lo.8bit) (R)
4217h - RDMPYH - Unsigned Division Remainder / Multiply Product (up.8bit) (R)
```

See Ports 4204h-4206h (divide), and 4202h-4203h (multiply).

#### Timing Notes

The 42xxh Ports are clocked by the CPU Clock, meaning that one needs the same amount of "wait" opcodes no matter if the CPU Clock is 3.5MHz or 2.6MHz. When reading the result, the "MOV r,[421xh]" opcode does include 3 cycles (spent on reading the 3-byte opcode), meaning that one needs to insert only 5 cycles for MUL and only 13 for DIV.

Some special cases: If the the upper "N" bits of 4202h are all zero, then it seems that one may wait "N" cycles less. If memory REFRESH occurs (once and when), then the result seems to be valid within even less wait opcodes.

The maths operations are started only on WRMPYB/WRDIVB writes (not on WRMPYA/WRDIVL/WRDIVH writes; unlike the PPU maths which start on any M7A/M7B write).

### PPU Ports

Below Ports 21xxh are PPU registers. The registers are also used for rotation/scaling effects in BG Mode 7. In BG Mode 0-6 they can be used freely for multiplications. In Mode 7 they are usable ONLY during V-Blank and Forced-Blank (during the Mode 7 Drawing & H-Blank periods, they return garbage in MPYL/MPYM/MPYH, and of course writing math-parameters to M7A/M7B would also mess-up the display).

```text
211Bh - M7A - Rotation/Scaling Parameter A (and Maths 16bit operand) (W)
  1st Write: Lower 8bit of signed 16bit Multiplicand  ;\1st/2nd write mechanism
  2nd Write: Upper 8bit of signed 16bit Multiplicand  ;/uses "M7_old" (Mode7)

211Ch - M7B - Rotation/Scaling Parameter B (and Maths 8bit operand) (W)
  Any Write: Signed 8bit Multiplier                   ;-also affects "M7_old"
```

After writing to 211Bh or 211Ch, the result can be read immediately from 2134h-2136h (the 21xxh Ports are rapidly clocked by the PPU, there's no delay needed when reading via "MOV A,[211Ch]" or via "MOV A,[1Ch]" (with D=2100h), both works even when the CPU runs at 3.5MHz).

```text
2134h - MPYL - Signed Multiply Result (lower 8bit) (R)
2135h - MPYM - Signed Multiply Result (middle 8bit) (R)
2136h - MPYH - Signed Multiply Result (upper 8bit) (R)
```

See Ports 211Bh-211Ch.

#### Notes

Some cartridges contain co-processors with further math functions:

> **See:** [SNES Cartridges](snes-cartridges.md)
