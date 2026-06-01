# SNES Cart SPC7110 Multiply/Divide Unit

#### Unsigned Multiply/Divide Unit

```text
  4820h Dividend, Bit0-7 / Multiplicand, Bit0-7
  4821h Dividend, Bit8-15 / Multiplicand, Bit8-15
  4822h Dividend, Bit16-23
  4823h Dividend, Bit24-31
  4824h Multiplier, Bit0-7
  4825h Multiplier, Bit8-15, Start Multiply on write to this register
  4826h Divisor, Bit0-7
  4827h Divisor, Bit8-15, Start Division on write to this register
  4828h Multiply/Divide Result, Bit0-7
  4829h Multiply/Divide Result, Bit8-15
  482Ah Multiply/Divide Result, Bit16-23
  482Bh Multiply/Divide Result, Bit24-31
  482Ch Divide Remainder, Bit0-7
  482Dh Divide Remainder, Bit8-15
  482Eh Multiply/Divide Reset  (write = reset 4820h..482Dh) (write 00h)
  482Fh Multiply/Divide Status (bit7: 0=Ready, 1=Busy)
```

#### Unknown Stuff

Multiply/Divide execution time is unknown. Is it constant/faster for small values? Behaviour on Divide by 0 is unknown?

Purpose of 482Eh is unknown, does it really "reset" 4820h..482Dh? Meaning that those registers are set to zero? Is that required/optional?

Are there other modes, like support for signed-numbers, or a fast 8bit*8bit multiply mode or such?

#### Reportedly

```text
  482Eh.bit0  (0=unsigned, 1=signed)
  (un)signed div0 returns --> result=00000000h, remainder=dividend AND FFFFh
  -80000000h/-1 returns <unknown> ?
```
