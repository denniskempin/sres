# ARM 26bit Memory Interface

The 26bit Memory Interface was used by ARMv1 and ARMv2. The 32bit interface is used by ARMv3 and newer, however, 26bit backward compatibility was included in all ARMv3 (except ARMv3G), and optionally in some non-T variants of ARMv4.

#### Format of R15 in 26bit Mode (Program Counter Register)

```text
  Bit   Name     Expl.
  31-28 N,Z,C,V  Flags (Sign, Zero, Carry, Overflow)
  27-26 I,F      Interrupt Disable bits (IRQ, FIQ) (1=Disable)
  25-2  PC       Program Counter, 24bit, Step 4 (64M range)
  1-0   M1,M0    Mode (0=User, 1=FIQ, 2=IRQ, 3=Supervisor)
```

Branches with +/-32M range wrap the PC register, and can reach all 64M memory.

#### Reading from R15

If R15 is specified in bit16-19 of an opcode, then NZCVIF and M0,1 are masked (zero), otherwise the full 32bits are used.

#### Writing to R15

ALU opcodes with S=1, and LDM opcodes with PSR=1 can write to all 32bits in R15 (in 26bit mode, that is allowed even in user mode, though it does then affect only NZCF, not the write protected IFMM bits ???), other opcodes which write to R15 will modify only the program counter bits. Also, special CMP/CMN/TST/TEQ{P} opcodes can be used to write to the PSR bits in R15 without modifying the PC bits.

#### Exceptions

SWIs, Reset, Data/Prefetch Aborts and Undefined instructions enter Supervisor mode. Interrupts enter IRQ and FIQ mode. Additionally, a special 26bit Address Exception exists, which enters Supervisor mode on accesses to memory addresses>=64M as follows:

```text
  R14_svc = PC ($+8, including old PSR bits)
  M1,M0 = 11b = supervisor mode, F=same, I=1, PC=14h,
  to continue at the fault location, return by SUBS PC,LR,8.
```

32bit CPUs with 26bit compatibility mode can be configured to switch into 32bit mode when encountering exceptions.
