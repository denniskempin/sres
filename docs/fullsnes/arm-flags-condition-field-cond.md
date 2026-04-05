# ARM Flags & Condition Field (cond)

#### ARM Condition Field {cond}

All ARM instructions can be conditionally executed depending on the state of the CPSR flags (C,N,Z,V). The respective suffixes {cond} must be appended to the mnemonics. For example: BEQ = Branch if Equal, MOVMI = Move if Signed.

```text
  Code Suffix Flags         Meaning
  0:   EQ     Z=1           equal (zero) (same)
  1:   NE     Z=0           not equal (nonzero) (not same)
  2:   CS/HS  C=1           unsigned higher or same (carry set)
  3:   CC/LO  C=0           unsigned lower (carry cleared)
  4:   MI     N=1           negative (minus)
  5:   PL     N=0           positive or zero (plus)
  6:   VS     V=1           overflow (V set)
  7:   VC     V=0           no overflow (V cleared)
  8:   HI     C=1 and Z=0   unsigned higher
  9:   LS     C=0 or Z=1    unsigned lower or same
  A:   GE     N=V           greater or equal
  B:   LT     N<>V          less than
  C:   GT     Z=0 and N=V   greater than
  D:   LE     Z=1 or N<>V   less or equal
  E:   AL     -             always (the "AL" suffix can be omitted)
  F:   NV     -             never (ARMv1,v2 only) (Reserved on ARMv3 and up)
```

Execution Time: If condition=false: 1S cycle. Otherwise: as specified for the respective opcode.

#### Current Program Status Register (CPSR)

```text
  Bit   Expl.
  31    N - Sign Flag       (0=Not Signed, 1=Signed)               ;\
  30    Z - Zero Flag       (0=Not Zero, 1=Zero)                   ; Condition
  29    C - Carry Flag      (0=Borrow/No Carry, 1=Carry/No Borrow) ; Code Flags
  28    V - Overflow Flag   (0=No Overflow, 1=Overflow)            ;/
  27    Q - Reserved        (used as Sticky Overflow in ARMv5TE and up)
  26-8  - - Reserved        (For future use) - Do not change manually!
  7     I - IRQ disable     (0=Enable, 1=Disable)                  ;\
  6     F - FIQ disable     (0=Enable, 1=Disable)                  ; Control
  5     T - Reserved        (used as THUMB flag in ARMv4T and up)  ; Bits
  4-0   M4-M0 - Mode Bits   (See below)                            ;/
```

CPSR Bit 27-8,5: Reserved Bits

These bits are reserved for possible future implementations. For best forwards compatibility, the user should never change the state of these bits, and should not expect these bits to be set to a specific value.

CPSR Bit 7-0: Control Bits (I,F,T,M4-M0)

These bits may change when an exception occurs. In privileged modes (non-user modes) they may be also changed manually.

The interrupt bits I and F are used to disable IRQ and FIQ interrupts respectively (a setting of "1" means disabled).

The Mode Bits M4-M0 contain the current operating mode.

```text
  Binary Hex Dec  Expl.
  0xx00b 00h 0  - Old User       ;\26bit Backward Compatibility modes
  0xx01b 01h 1  - Old FIQ        ; (supported only on ARMv3, except ARMv3G,
  0xx10b 02h 2  - Old IRQ        ; and on some non-T variants of ARMv4)
  0xx11b 03h 3  - Old Supervisor ;/
  10000b 10h 16 - User (non-privileged)
  10001b 11h 17 - FIQ
  10010b 12h 18 - IRQ
  10011b 13h 19 - Supervisor (SWI)
  10111b 17h 23 - Abort
  11011b 1Bh 27 - Undefined
  11111b 1Fh 31 - Reserved (used as System mode in ARMv4 and up)
```

Writing any other values into the Mode bits is not allowed.

#### Saved Program Status Registers (SPSR_<mode>)

Additionally to above CPSR, five Saved Program Status Registers exist:

SPSR_fiq, SPSR_svc, SPSR_abt, SPSR_irq, SPSR_und

Whenever the CPU enters an exception, the current status register (CPSR) is copied to the respective SPSR_<mode> register. Note that there is only one SPSR for each mode, so nested exceptions inside of the same mode are allowed only if the exception handler saves the content of SPSR in memory.

For example, for an IRQ exception: IRQ-mode is entered, and CPSR is copied to SPSR_irq. If the interrupt handler wants to enable nested IRQs, then it must first push SPSR_irq before doing so.
