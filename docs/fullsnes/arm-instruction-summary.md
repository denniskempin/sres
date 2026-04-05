# ARM Instruction Summary

Modification of CPSR flags is optional for all {S} instructions.

#### Logical ALU Operations

```text
  Instruction                      Cycles    Flags Expl.
  MOV{cond}{S} Rd,Op2              1S+x+y     NZc- Rd = Op2
  MVN{cond}{S} Rd,Op2              1S+x+y     NZc- Rd = NOT Op2
  ORR{cond}{S} Rd,Rn,Op2           1S+x+y     NZc- Rd = Rn OR Op2
  EOR{cond}{S} Rd,Rn,Op2           1S+x+y     NZc- Rd = Rn XOR Op2
  AND{cond}{S} Rd,Rn,Op2           1S+x+y     NZc- Rd = Rn AND Op2
  BIC{cond}{S} Rd,Rn,Op2           1S+x+y     NZc- Rd = Rn AND NOT Op2
  TST{cond}{P}    Rn,Op2           1S+x       NZc- Void = Rn AND Op2
  TEQ{cond}{P}    Rn,Op2           1S+x       NZc- Void = Rn XOR Op2
```

Add x=1I cycles if Op2 shifted-by-register. Add y=1S+1N cycles if Rd=R15.

Carry flag affected only if Op2 contains a non-zero shift amount.

#### Arithmetic ALU Operations

```text
  Instruction                      Cycles    Flags Expl.
  ADD{cond}{S} Rd,Rn,Op2           1S+x+y     NZCV Rd = Rn+Op2
  ADC{cond}{S} Rd,Rn,Op2           1S+x+y     NZCV Rd = Rn+Op2+Cy
  SUB{cond}{S} Rd,Rn,Op2           1S+x+y     NZCV Rd = Rn-Op2
  SBC{cond}{S} Rd,Rn,Op2           1S+x+y     NZCV Rd = Rn-Op2+Cy-1
  RSB{cond}{S} Rd,Rn,Op2           1S+x+y     NZCV Rd = Op2-Rn
  RSC{cond}{S} Rd,Rn,Op2           1S+x+y     NZCV Rd = Op2-Rn+Cy-1
  CMP{cond}{P}    Rn,Op2           1S+x       NZCV Void = Rn-Op2
  CMN{cond}{P}    Rn,Op2           1S+x       NZCV Void = Rn+Op2
```

Add x=1I cycles if Op2 shifted-by-register. Add y=1S+1N cycles if Rd=R15.

#### Multiply

```text
  Instruction                      Cycles    Flags Expl.
  MUL{cond}{S} Rd,Rm,Rs            1S+mI      NZx- Rd = Rm*Rs
  MLA{cond}{S} Rd,Rm,Rs,Rn         1S+mI+1I   NZx- Rd = Rm*Rs+Rn
  UMULL{cond}{S} RdLo,RdHi,Rm,Rs   1S+mI+1I   NZx- RdHiLo = Rm*Rs
  UMLAL{cond}{S} RdLo,RdHi,Rm,Rs   1S+mI+2I   NZx- RdHiLo = Rm*Rs+RdHiLo
  SMULL{cond}{S} RdLo,RdHi,Rm,Rs   1S+mI+1I   NZx- RdHiLo = Rm*Rs
  SMLAL{cond}{S} RdLo,RdHi,Rm,Rs   1S+mI+2I   NZx- RdHiLo = Rm*Rs+RdHiLo
```

#### Memory Load/Store

```text
  Instruction                      Cycles    Flags Expl.
  LDR{cond}{B}{T} Rd,<Address>     1S+1N+1I+y ---- Rd=[Rn+/-<offset>]
  LDM{cond}{amod} Rn{!},<Rlist>{^} nS+1N+1I+y ---- Load Multiple
  STR{cond}{B}{T} Rd,<Address>     2N         ---- [Rn+/-<offset>]=Rd
  STM{cond}{amod} Rn{!},<Rlist>{^} (n-1)S+2N  ---- Store Multiple
  SWP{cond}{B}    Rd,Rm,[Rn]       1S+2N+1I   ---- Rd=[Rn], [Rn]=Rm
```

For LDR/LDM, add y=1S+1N if Rd=R15, or if R15 in Rlist.

Jumps, Calls, CPSR Mode, and others

```text
  Instruction                      Cycles    Flags Expl.
  B{cond}   label                  2S+1N      ---- PC=$+8+/-32M
  BL{cond}  label                  2S+1N      ---- PC=$+8+/-32M, LR=$+4
  MRS{cond} Rd,Psr                 1S         ---- Rd=Psr
  MSR{cond} Psr{_field},Op         1S        (psr) Psr[field]=Op
  SWI{cond} Imm24bit               2S+1N      ---- PC=8, ARM Svc mode, LR=$+4
  The Undefined Instruction        2S+1I+1N   ---- PC=4, ARM Und mode, LR=$+4
  condition=false                  1S         ---- Opcodes with {cond}=false
  NOP                              1S         ---- R0=R0
```

#### Coprocessor Functions (if any)

```text
  Instruction                         Cycles  Flags Expl.
  CDP{cond} Pn,<cpopc>,Cd,Cn,Cm{,<cp>} 1S+bI   ----  Coprocessor specific
  STC{cond}{L} Pn,Cd,<Address>         (n-1)S+2N+bI  [address] = CRd
  LDC{cond}{L} Pn,Cd,<Address>         (n-1)S+2N+bI  CRd = [address]
  MCR{cond} Pn,<cpopc>,Rd,Cn,Cm{,<cp>} 1S+bI+1C      CRn = Rn {<op> CRm}
  MRC{cond} Pn,<cpopc>,Rd,Cn,Cm{,<cp>} 1S+(b+1)I+1C  Rn = CRn {<op> CRm}
```

#### ARM Binary Opcode Format

```text
  |..3 ..................2 ..................1 ..................0|
  |1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0|
  |_Cond__|0_0_0|___Op__|S|__Rn___|__Rd___|__Shift__|Typ|0|__Rm___| DataProc
  |_Cond__|0_0_0|___Op__|S|__Rn___|__Rd___|__Rs___|0|Typ|1|__Rm___| DataProc
  |_Cond__|0_0_1|___Op__|S|__Rn___|__Rd___|_Shift_|___Immediate___| DataProc
  |_Cond__|0_0_1_1_0|P|1|0|_Field_|__Rd___|_Shift_|___Immediate___| PSR Imm
  |_Cond__|0_0_0_1_0|P|L|0|_Field_|__Rd___|0_0_0_0|0_0_0_0|__Rm___| PSR Reg
  |_Cond__|0_0_0_0_0_0|A|S|__Rd___|__Rn___|__Rs___|1_0_0_1|__Rm___| Multiply
  |_Cond__|0_0_0_0_1|U|A|S|_RdHi__|_RdLo__|__Rs___|1_0_0_1|__Rm___| MulLong
  |_Cond__|0_0_0_1_0|B|0_0|__Rn___|__Rd___|0_0_0_0|1_0_0_1|__Rm___| TransSwap
  |_Cond__|0_1_0|P|U|B|W|L|__Rn___|__Rd___|_________Offset________| TransImm
  |_Cond__|0_1_1|P|U|B|W|L|__Rn___|__Rd___|__Shift__|Typ|0|__Rm___| TransReg
  |_Cond__|0_1_1|________________xxx____________________|1|__xxx__| Undefined
  |_Cond__|1_0_0|P|U|S|W|L|__Rn___|__________Register_List________| TransBlock
  |_Cond__|1_0_1|L|___________________Offset______________________| B,BL
  |_Cond__|1_1_0|P|U|N|W|L|__Rn___|__CRd__|__CP#__|____Offset_____| CoDataTrans
  |_Cond__|1_1_1_0|_CPopc_|__CRn__|__CRd__|__CP#__|_CP__|0|__CRm__| CoDataOp
  |_Cond__|1_1_1_0|CPopc|L|__CRn__|__Rd___|__CP#__|_CP__|1|__CRm__| CoRegTrans
  |_Cond__|1_1_1_1|_____________Ignored_by_Processor______________| SWI
```
