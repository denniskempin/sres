# SFC-Box Memory & I/O Maps

most of KROM is high-level-language based crap, this is ACTUALLY WORSE than 6502-code compiled to run on a Z80 CPU.

#### Physical 19bit Memory Map

```text
  00000h..00FFFFh   KROM
  20000h..207FFFh   WRAM (mainly 204000h..207FFFh used)
                           (area at 200000h..203FFFh used as battery-ram?
                           with read/write-protect via [A0].7?)
  40000h..407FFFh   GROM-Slot 0
  60000h..607FFFh   GROM-Slot 1
```

#### Virtual 16bit Memory Map

```text
  0000h..7FFFh      KROM (first 32K)
  8000h..BFFFh      Bank Area (16K banks, KROM,GROM,WRAM)
  C000h..FFFFh      WRAM (last 16K)
```

RAM (as used by KROM1)  [8000...]               <-- extra 16K RAM bank (unchanged on

```text
                             reset/entrypoint... probably battery backed?)
  ...                        (that 16K are read/write-protected via [A0].7 ?)
```

[C000..FFFF]  work ram

#### I/O Map

```text
  [00h..3Fh]  HD64180 (CPU on-chip I/O ports)
  [40h..7Fh]  Unused (reading returns FFh)
  [80h].R     Keyswitch and Button Inputs
  [80h].W     SNES Transfer and Misc Output
  [81h].R     SNES Transfer and Misc Input
  [81h].W     Misc Output
  [82h].R/W   Unknown/unused
  [83h].R     Joypad Input/Status
  [83h].W     Joypad Output/Control
  [84h].R/W   Joypad 1, MSB (1st 8 bits) (eg. Bit7=ButtonB, 0=Low=Pressed)
  [85h].R/W   Joypad 1, LSB (2nd 8 bits) (eg. Bit0=LSB of ID, 0=Low=One)
  [86h].R/W   Joypad 2, MSB (1st 8 bits) (eg. Bit7=ButtonB, 0=Low=Pressed)
  [87h].R/W   Joypad 2, LSB (2nd 8 bits) (eg. Bit0=LSB of ID, 0=Low=One)
  [88h..9Fh]  Unused (mirrors of Port 80h..87h)
  [A0h].R     Real Time Clock Input
  [A0h].W     Real Time Clock Output
  [A1h..BFh]  Unused (mirror of Port A0h)
  [C0h].R     Unknown/unused (reading returns FFh)
  [C0h].W     SNES Mapping Register 0
  [C1h].R     Unknown/unused (reading returns FFh)
  [C1h].W     SNES Mapping Register 1
  [C2h..FFh]  Unused (maybe mirrors of Port C0h..C1h) (reading returns FFh)
```

16bit I/O Space (when address MSB=xx=nonzero):

```text
  [xx00h..xx7Fh]  Unused (reading returns FFh) (no mirror of 0000h..003Fh)
  [xx80h..xxBFh]  Mirror of 0080h..00BFh
  [xxC0h..xxFFh]  Unknown (probably mirror of 00C0h..00FFh)
```
