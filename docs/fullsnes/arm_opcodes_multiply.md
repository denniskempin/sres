# ARM Opcodes: Multiply and Multiply-Accumulate (MUL, MLA)

#### Opcode Format

```text
  Bit    Expl.
  31-28  Condition
  27-25  Must be 000b for this instruction
  24-21  Opcode
          0000b: MUL{cond}{S}   Rd,Rm,Rs        ;multiply   Rd = Rm*Rs
          0001b: MLA{cond}{S}   Rd,Rm,Rs,Rn     ;mul.& accumulate Rd = Rm*Rs+Rn
          0100b: UMULL{cond}{S} RdLo,RdHi,Rm,Rs ;multiply   RdHiLo=Rm*Rs
          0101b: UMLAL{cond}{S} RdLo,RdHi,Rm,Rs ;mul.& acc. RdHiLo=Rm*Rs+RdHiLo
          0110b: SMULL{cond}{S} RdLo,RdHi,Rm,Rs ;sign.mul.  RdHiLo=Rm*Rs
          0111b: SMLAL{cond}{S} RdLo,RdHi,Rm,Rs ;sign.m&a.  RdHiLo=Rm*Rs+RdHiLo
  20     S - Set Condition Codes (0=No, 1=Yes) (Must be 0 for Halfword mul)
  19-16  Rd (or RdHi) - Destination Register (R0-R14)
  15-12  Rn (or RdLo) - Accumulate Register  (R0-R14) (Set to 0000b if unused)
  11-8   Rs - Operand Register               (R0-R14)
  7-4    Must be 1001b for these instructions
  3-0    Rm - Operand Register               (R0-R14)
```

Multiply and Multiply-Accumulate (MUL, MLA)

Restrictions: Rd may not be same as Rm. Rd,Rn,Rs,Rm may not be R15.

Note: Only the lower 32bit of the internal 64bit result are stored in Rd, thus no sign/zero extension is required and MUL and MLA can be used for both signed and unsigned calculations!

Execution Time: 1S+mI for MUL, and 1S+(m+1)I for MLA. Whereas 'm' depends on whether/how many most significant bits of Rs are all zero or all one. That is m=1 for Bit 31-8, m=2 for Bit 31-16, m=3 for Bit 31-24, and m=4 otherwise.

Flags (if S=1): Z=zeroflag, N=signflag, C=destroyed (ARMv4 and below) or C=not affected (ARMv5 and up), V=not affected. MUL/MLA supported by ARMv2 and up.

Multiply Long and Multiply-Accumulate Long (MULL, MLAL) Optionally supported, INCLUDED in ARMv3M, EXCLUDED in ARMv4xM/ARMv5xM.

Restrictions: RdHi,RdLo,Rm must be different registers. R15 may not be used.

Execution Time: 1S+(m+1)I for MULL, and 1S+(m+2)I for MLAL. Whereas 'm' depends on whether/how many most significant bits of Rs are "all zero" (UMULL/UMLAL) or "all zero or all one" (SMULL,SMLAL). That is m=1 for Bit 31-8, m=2 for Bit 31-16, m=3 for Bit 31-24, and m=4 otherwise.

Flags (if S=1): Z=zeroflag, N=signflag, C=destroyed (ARMv4 and below) or C=not affected (ARMv5 and up), V=destroyed??? (ARMv4 and below???) or V=not affected (ARMv5 and up).
