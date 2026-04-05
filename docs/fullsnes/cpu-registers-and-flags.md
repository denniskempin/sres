# CPU Registers and Flags

The 65XX CPUs are equipped with not more than three general purpose registers (A, X, Y). However, the limited number of registers is parts of covered by comfortable memory operations, especially page 0 of memory (address 0000h-00FFh) may be used for relative fast and complicated operations, in so far one might say that the CPU has about 256 8bit 'registers' (or 128 16bit 'registers') in memory. For details see Memory Addressing chapter.

#### Registers

```text
  Bits  Name  Expl.
  8/16  A     Accumulator
  8/16  X     Index Register X
  8/16  Y     Index Register Y
  16    PC    Program Counter
  8/16  S     Stack Pointer (see below)
  8     P     Processor Status Register (see below)
  16    D     Zeropage Offset      ;expands 8bit  [nn]   to 16bit [00:nn+D]
  8     DB    Data Bank            ;expands 16bit [nnnn] to 24bit [DB:nnnn]
  8     PB    Program Counter Bank ;expands 16bit PC     to 24bit PB:PC
```

#### Stack Pointer

The stack pointer is addressing 256 bytes in page 1 of memory, ie. values 00h-FFh will address memory at 0100h-01FFh. As for most other CPUs, the stack pointer is decrementing when storing data. However, in the 65XX world, it points to the first FREE byte on stack, so, when initializing stack to top set S=(1)FFh (rather than S=(2)00h).

#### Processor Status Register (Flags)

```text
  Bit  Name  Expl.
  0    C     Carry         (0=No Carry, 1=Carry)
  1    Z     Zero          (0=Nonzero, 1=Zero)
  2    I     IRQ Disable   (0=IRQ Enable, 1=IRQ Disable)
  3    D     Decimal Mode  (0=Normal, 1=BCD Mode for ADC/SBC opcodes)
  4    X/B   Break Flag    (0=IRQ/NMI, 1=BRK/PHP opcode)  (0=16bit, 1=8bit)
  5    M/U   Unused        (Always 1)                     (0=16bit, 1=8bit)
  6    V     Overflow      (0=No Overflow, 1=Overflow)
  7    N     Negative/Sign (0=Positive, 1=Negative)
  -    E                                                  (0=16bit, 1=8bit)
```

Emulation Flag (E) (can accessed only via XCE opcode) Activates 6502 emulation mode. In that mode, A/X/Y are 8bit (as when X/M are set). S is 8bit (range 0100h..01FFh). The X/M flags are replaced by B/U flags (B=Break/see below, U=Unused/always 1). Exception Vectors are changed to 6502 addresses, and exceptions push only 16bit return addresses (rather than 24bit).

Conditional Branches take one extra cycle when crossing pages.

The new 65C02 and 65C816 opcodes can be kept used; respectively, none of the undocumented opcodes of the original 6502 CPU are supported.

#### Accumulator/Memory Flag (M)

Switches A to 8bit mode (the upper 8bit of A can be still accessed via XBA).

Additionally switches all opcodes that do not use A,X,Y operands (eg. STZ, and read-modify-write opcodes like inc/dec, Rotate/Shift) to 8bit mode.

#### Index Register Flag (X)

Switches X and Y to 8bit mode (the upper 8bit are forcefully set to 00h).

#### Carry Flag (C)

Caution: When used for subtractions (SBC and CMP), the carry flag is having opposite meaning as for normal 80x86 and Z80 CPUs, ie. it is SET when above-or-equal. For all other instructions (ADC, ASL, LSR, ROL, ROR) it works as normal, whereas ROL/ROR are rotating <through> carry (ie. much like 80x86 RCL/RCR and not like ROL/ROR).

Zero Flag (Z), Negative/Sign Flag (N), Overflow Flag (V) Works just as everywhere, Z it is set when result (or destination register, in case of some 'move' instructions) is zero, N is set when signed (ie. same as Bit 7 of result/destination). V is set when an addition/subtraction exceeded the maximum range for signed numbers (-128..+127).

#### IRQ Disable Flag (I)

Disables IRQs when set. NMIs (non maskable interrupts) and BRK instructions cannot be disabled.

#### Decimal Mode Flag (D)

Packed BCD mode (range 00h..99h) for ADC and SBC opcodes.

#### Break Flag (B)

The Break flag is intended to separate between IRQ and BRK which are both using the same vector, [FFFEh]. The flag cannot be accessed directly, but there are 4 situations which are writing the P register to stack, which are then allowing the examine the B-bit in the pushed value: The BRK and PHP opcodes always write "1" into the bit, IRQ/NMI execution always write "0".

#### ---

#### Common Confusing Aliases for Register Names

```text
  DPR   D  (used in homebrew specs)
  K     PB (used by PHK)
  PBR   PB (used in specs)
  DBR   DB (used in specs)
  B     DB (used by PLB,PHB)
  B     Upper 8bit of A (used by XBA)
  C     Full 16bit of A (used by TDC,TCD,TSC,TCS)
```
