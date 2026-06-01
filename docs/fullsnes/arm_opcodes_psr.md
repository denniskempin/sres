# ARM Opcodes: PSR Transfer (MRS, MSR)

#### Opcode Format

These instructions occupy an unused area (TEQ,TST,CMP,CMN with S=0) of ALU opcodes.

```text
  Bit    Expl.
  31-28  Condition
  27-26  Must be 00b for this instruction
  25     I - Immediate Operand Flag  (0=Register, 1=Immediate) (Zero for MRS)
  24-23  Must be 10b for this instruction
  22     Psr - Source/Destination PSR  (0=CPSR, 1=SPSR_<current mode>)
  21     Opcode
           0: MRS{cond} Rd,Psr          ;Rd = Psr
           1: MSR{cond} Psr{_field},Op  ;Psr[field] = Op
  20     Must be 0b for this instruction (otherwise TST,TEQ,CMP,CMN)
  For MRS:
    19-16   Must be 1111b for this instruction (otherwise SWP)
    15-12   Rd - Destination Register  (R0-R14)
    11-0    Not used, must be zero.
  For MSR:
    19      f  write to flags field     Bit 31-24 (aka _flg)
    18      s  write to status field    Bit 23-16 (reserved, don't change)
    17      x  write to extension field Bit 15-8  (reserved, don't change)
    16      c  write to control field   Bit 7-0   (aka _ctl)
    15-12   Not used, must be 1111b.
  For MSR Psr,Rm (I=0)
    11-4    Not used, must be zero.
    3-0     Rm - Source Register <op>  (R0-R14)
  For MSR Psr,Imm (I=1)
    11-8    Shift applied to Imm   (ROR in steps of two 0-30)
    7-0     Imm - Unsigned 8bit Immediate
    In source code, a 32bit immediate should be specified as operand.
    The assembler should then convert that into a shifted 8bit value.
```

MSR/MRS and CPSR/SPSR supported by ARMv3 and up.

ARMv2 and below contained PSR flags in R15, accessed by CMP/CMN/TST/TEQ{P}.

The field mask bits specify which bits of the destination Psr are write-able (or write-protected), one or more of these bits should be set, for example, CPSR_fsxc (aka CPSR aka CPSR_all) unlocks all bits (see below user mode restriction though).

Restrictions:

In non-privileged mode (user mode): only condition code bits of CPSR can be changed, control bits can't.

Only the SPSR of the current mode can be accessed; In User and System modes no SPSR exists.

Unused Bits in CPSR are reserved for future use and should never be changed (except for unused bits in the flags field).

Execution Time: 1S.

Note: The A22i assembler recognizes MOV as alias for both MSR and MRS because it is practically not possible to remember whether MSR or MRS was the load or store opcode, and/or whether it does load to or from the Psr register.
