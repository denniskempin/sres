# HD64180 Timers (PRT and FRC)

#### Programmable Reload Timers (PRT) and Free Running Counter (FRC)

#### 10h - TCR - Timer Control Register (00h on Reset)

```text
  7   TIF1  Timer 1 Interrupt Flag (0=No, 1=Yes/Decrement reached 0000h) (R)
  6   TIF0  Timer 0 Interrupt Flag (0=No, 1=Yes/Decrement reached 0000h) (R)
  5   TIE1  Timer 1 Interrupt Enable (0=Disable, 1=Enable)
  4   TIE0  Timer 0 Interrupt Enable (0=Disable, 1=Enable)
  3-2 TOC   Timer 1 Output Control to A18-Pin (0=A18, 1=Toggled, 2=Low, 3=High)
  1   TDIE1 Timer 1 Decrement Enable (0=Stop, 1=Decrement; once every 20 clks)
  0   TDIE0 Timer 0 Decrement Enable (0=Stop, 1=Decrement; once every 20 clks)
```

TIF1 is reset when reading TCR or TMDR1L or TMDR1H.

TIF0 is reset when reading TCR or TMDR0L or TMDR0H.

The TOC bits control the A18/TOUT pin (it can be either A18 address line, or forced to Low or High, or "toggled": that is, inverted when TMDR1 decremts to 0.

0Ch - TMDR0L - Timer 0 Counter "Data" Register, Bit0-7 (FFh on Reset) 0Dh - TMDR0H - Timer 0 Counter "Data" Register, Bit8-15 (FFh on Reset) 0Eh - RLDR0L - Timer 0 Reload Register, Bit0-7 (FFh on Reset) 0Fh - RLDR0H - Timer 0 Reload Register, Bit8-15 (FFh on Reset) Timer 0 counter/reload values. The counter is decremented once every 20 clks, and triggers IRQ and gets reloaded when reaching 0000h. Reading TMDR0L returns current timer LSB, and latches current timer MSB. Reading TMDR0H returns that LATCHED timer MSB. Accordingly reads should be always done in order LSB, MSB.

If the timer is stopped TMDR0L/TMDR0H can be written (and read) in any order.

14h - TMDR1L - Timer 1 Counter "Data" Register, Bit0-7 (FFh on Reset) 15h - TMDR1H - Timer 1 Counter "Data" Register, Bit8-15 (FFh on Reset) 16h - RLDR1L - Timer 1 Reload Register, Bit0-7 (FFh on Reset) 17h - RLDR1H - Timer 1 Reload Register, Bit8-15 (FFh on Reset) Timer 1 counter/reload values. Same as for Timer 0 (see above).

#### 18h - FRC - Free Running Counter (FFh on Reset)

```text
  7-0 FRC   Free Running Counter (decremented every 10 clks)
```

This register should be read-only, writing to FRC may mess up DRAM refresh, ASCI and CSI/O baud rates.
