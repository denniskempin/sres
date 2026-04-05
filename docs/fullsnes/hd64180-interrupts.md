# HD64180 Interrupts

#### Interrupts

```text
  Prio                                     Vector
  0  /RES  Reset (non-maskable)            (PC=0000h, with TRAP=0 in ITC)
  1  TRAP  Undefined Opcode (non-maskable) (PC=0000h, with TRAP=1 in ITC)
  2  /NMI  Non-maskable Interrupt          (PC=0066h)
  3  /INT0 Maskable Interrupt Level 0      (PC=[I*100h+databus], or PC=0038h)
  4  /INT1 Maskable Interrupt Level 1      (PC=[I*100h+IL*20h+00h])
  5  /INT2 Maskable Interrupt Level 2      (PC=[I*100h+IL*20h+02h])
  6  Timer 0                               (PC=[I*100h+IL*20h+04h])
  7  Timer 1                               (PC=[I*100h+IL*20h+06h])
  8  DMA Channel 0 Ready                   (PC=[I*100h+IL*20h+08h])
  9  DMA Channel 1 Ready                   (PC=[I*100h+IL*20h+0Ah])
  10 Clocked Serial I/O Port (CSI/O)       (PC=[I*100h+IL*20h+0Ch])
  11 Asynchronous SCI channel 0            (PC=[I*100h+IL*20h+0Eh])
  12 Asynchronous SCI channel 1            (PC=[I*100h+IL*20h+10h])
  Below whatever only (not HD64180 and not Z180)
  ?  Input Capture                         (PC=[I*100h+IL*20h+10h])
  ?  Output Compare                        (PC=[I*100h+IL*20h+12h])
  ?  Timer Overflow                        (PC=[I*100h+IL*20h+16h])
```

Note: "I" is a CPU-register (set via MOV I,A opcode). "IL" is new I/O port (set via OUT opcode). /INT0 works same as on real Z80 (and depends on mode set via IM 0/1/2 opcodes).

33h - IL - Interrupt Vector Low Register (00h on Reset)

```text
  7-5 IL   Bit7-5 of IM 2 Interrupt Vector Table Address
  4-0 -    Unused (should be zero)
```

#### 34h - ITC - INT/TRAP Control Register (39h on Reset)

```text
  7   TRAP Undefined Opcode occurred (0=No, 1=Yes)
  6   UFO  Addr of Undef Opcode (aka Undefined Fetch Object) (0=PC-1, 1=PC-2)
  5-3 -    Unused (should be all-ones)
  2   ITE2 Interrupt /INT2 Enable (0=Disable, 1=Enable)
  1   ITE1 Interrupt /INT1 Enable (0=Disable, 1=Enable)
  0   ITE0 Interrupt /INT0 Enable (0=Disable, 1=Enable)
```

TRAP gets set upon Undefined Opcodes (TRAP and RESET are both using vector

```text
0000h, the TRAP bit allows to sense if the vector was called by Reset or Undef
```

Opcode). The TRAP bit can be cleared by software by writing "0" to it (however, software cannot write "1" to it).
