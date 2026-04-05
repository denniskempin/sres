# SNES Cart SA-1 Arithmetic Maths

#### 2250h SA-1 MCNT - Arithmetic Control (W)

```text
  0-1 Arithmetic Mode (0=Multiply, 1=Divide, 2=MultiplySum, 3=Reserved)
  2-7 Not used (should be "..") (whatever ".." means, maybe "0"?)
```

Note: Writing Bit1=1 does reset the Sum (aka "Cumulative Sum" aka "Accumulative Sum") to zero.

2251h SA-1 MA - Arithmetic Parameter A Lsb (Multiplicand/Dividend) (W) 2252h SA-1 MA - Arithmetic Parameter A Msb (Multiplicand/Dividend) (W)

```text
  0-15  SIGNED multiplicand or dividend (that is, both are signed)
```

The value in this register is kept intact after multiplaction, but gets destroyed after division.

2253h SA-1 MB - Arithmetic Parameter B Lsb (Multiplier/Divisor) (W) 2254h SA-1 MB - Arithmetic Parameter B Msb (Multiplier/Divisor)/Start (W)

```text
  0-15  SIGNED multiply parameter, or UNSIGNED divisor
```

The value in this register gets destroyed after both multiplaction and division. Writing to 2254h starts the operation. Execution time is 5 cycles (in 10.74MHz units) for both Multiply and Divide, and 6 cycles for Multiply/Sum.

2306h SA-1 MR - Arithmetic Result, bit0-7   (Sum/Product/Quotient) (R) 2307h SA-1 MR - Arithmetic Result, bit8-15  (Sum/Product/Quotient) (R) 2308h SA-1 MR - Arithmetic Result, bit16-23 (Sum/Product/Remainder) (R) 2309h SA-1 MR - Arithmetic Result, bit24-31 (Sum/Product/Remainder) (R) 230Ah SA-1 MR - Arithmetic Result, bit32-39 (Sum) (R)

```text
  32bit Multiply Result    (SIGNED)
  40bit Multiply/Sum       (SIGNED)
  16bit Division Result    (SIGNED)
  16bit Division Remainder (UNSIGNED !!!)
```

230Bh SA-1 OF - Arithmetic Overflow Flag (R) This bit is reportedly set on 40bit multiply/addition overflows (rather than on more useful 32bit overflows), thereby overflow can't occur unless one is doing at least 512 continous multiply/additions.

```text
  0-6 Not used (reportedly "..") (whatever ".." means, maybe 0 or open bus?)
  7   Arithmetic Sum Overflow Flag (0=No overflow, 1=Overflow)
```

Unknown when this bit gets cleared (all operations, or mode changes)?

Division by zero returns result=0000h and remainder=0000h (other info claims other values?) (but, as far as known, doesn't set set overflow flag).
