# SNES APU SPC700 CPU Overview

The sound unit is controlled by a S-SMP chip: an 8bit CPU that uses Sony's SPC700 instruction set. The SPC700's opcodes and it's three main registers (A,X,Y) are clearly inspired by 6502 instruction set, although the opcodes are numbered differently, and it has some new/changed instructions.

Aside from the SNES sound processor, the SPC700 is also found in Sony's CXP8nnnn single-chip microprocessors. Another variant, called SPC700 alpha II, is also found in their CXP7nnnn chips, which have only 211 opcodes (two less than the normal SPC700).

#### Registers

```text
  A     8bit accumulator
  X     8bit index
  Y     8bit index
  SP    8bit stack pointer (addresses memory at 0100h..01FFh)
  PSW   8bit flags
  YA    16bit combination of Y=MSB, and A=LSB
  PC    16bit program counter
```

#### Flags (PSW)

```text
  Bit7  N  Sign Flag          (0=Positive, 1=Negative)
  Bit6  V  Overflow Flag      (0=None, 1=Overflow)
  Bit5  P  Zero Page Location (0=00xxh, 1=01xxh)
  Bit4  B  Break Flag         (0=Reset, 1=BRK opcode; set <after> BRK opcode)
  Bit3  H  Half-carry         (0=Borrow, or no-carry, 1=Carry, or no-borrow)
  Bit2  I  Interrupt Enable   (0=Disable, 1=Enable) (no function in SNES APU)
  Bit1  Z  Zero Flag          (0=Non-zero, 1=Zero)
  Bit0  C  Carry Flag         (0=Borrow, or no-carry, 1=Carry, or no-borrow)
```

#### Addressing Modes

```text
  Native Syntax  Nocash Syntax
  aa             [aa]           ;\addresses memory at [0000..00FF]
  aa+X           [aa+X]         ; (or at [0100..01FF when flag P=1)
  aa+Y           [aa+Y]         ; (aa+X and aa+Y wrap within that area,
  (X)            [X]            ; ie. addr = (aa+X) AND 0FFh)
  (Y)            [Y]            ;/
  aaaa           [aaaa]         ;\
  aaaa+X         [aaaa+X]       ; addresses memory at [0000..FFFF]
  aaaa+Y         [aaaa+Y]       ;/
  [aa]+Y         [[aa]+Y]       ;-Byte[Word[aa]+Y]  ;\double-indirect (using
  [aa+X]         [[aa+X]]       ;-Byte[Word[aa+X]]  ;/16bit pointer in RAM)
  aa.b           [aa].b         ;-Bit0..7 of address [0000..00FF] (8bit addr)
  aaa.b          [aaa].b        ;-Bit0..7 of address [0000..1FFF] (13bit addr)
  stack (push/pop/call/ret)     ;-addresses memory at [0100..01FF] (SP+100h)
```

#### Formatting of Command Descriptions

```text
  Native Syntax  Nocash Syntax   Opcode     Clk Expl                   NVPBHIZC
```
