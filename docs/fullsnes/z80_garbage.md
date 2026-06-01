# Z80 Garbage in Flag Register

#### Nocash Z80-flags description

This chapter describes the undocumented Z80 flags (bit 3 and 5 of the Flags Register), these flags are affected by ALL instructions that modify one or more of the normal flags - all OTHER instructions do NOT affect the undocumented flags.

For some instructions, the content of some flags has been officially documented as 'destroyed', indicating that the flags contain garbage, the exact garbage calculation for these instructions will be described here also.

All information below just for curiosity. Keep in mind that Z80 compatible CPUs (or emulators) may not supply identical results, so that it wouldn't be a good idea to use these flags in any programs (not that they could be very useful anyways).

#### Normal Behaviour for Undocumented Flags

In most cases, undocumented flags are copied from the Bit 3 and Bit 5 of the result byte. That is "A AND 28h" for:

```text
  RLD; CPL; RLCA; RLA; LD A,I; ADD OP; ADC OP; XOR OP; AND OP;
  RRD; NEG; RRCA; RRA; LD A,R; SUB OP; SBC OP; OR OP ; DAA.
```

When other operands than A may be modified, "OP AND 28h" for:

```text
  RLC OP; RL OP; SLA OP; SLL OP; INC OP; IN OP,(C);
  RRC OP; RR OP; SRA OP; SRL OP; DEC OP
```

For 16bit instructions flags are calculated as "RR AND 2800h":

```text
  ADD RR,XX; ADC RR,XX; SBC RR,XX.
```

#### Slightly Special Undocumented Flags

For 'CP OP' flags are calculated as "OP AND 28h", that is the unmodified operand, and NOT the internally calculated result of the comparision.

For 'SCF' and 'CCF' flags are calculated as "(A OR F) AND 28h", ie. the flags remain set if they have been previously set.

For 'BIT N,R' flags are calculated as "OP AND 28h", additionally the P-Flag is set to the same value than the Z-Flag (ie. the Parity of "OP AND MASK"), and the S-flag is set to "OP AND MASK AND 80h".

#### Fatal MEMPTR Undocumented Flags

For 'BIT N,(HL)' the P- and S-flags are set as for BIT N,R, but the undocumented flags are calculated as "MEMPTR AND 2800h", for more info about MEMPTR read on below.

The same applies to 'BIT N,(ii+d)', but the result is less unpredictable because the instruction sets MEMPTR=ii+d, so that undocumented flags are "<ii+d> AND 2800h".

#### Memory Block Command Undocumented Flags

For LDI, LDD, LDIR, LDDR, undocumented flags are "((A+DATA) AND 08h) + ((A+DATA) AND 02h)*10h".

For CPI, CPD, CPIR, CPDR, undocumented flags are "((A-DATA-FLG_H) AND 08h) + ((A-DATA-FLG_H) AND 02h)*10h", whereas the CPU first calculates A-DATA, and then internally subtracts the resulting H-flag from the result.

#### Chaotic I/O Block Command Flags

The INI, IND, INIR, INDR, OUTI, OUTD, OTIR, OTDR instructions are doing a lot of obscure things, to simplify the description a placeholder called DUMMY is used in the formulas.

```text
  DUMMY = "REG_C+DATA+1"    ;for INI/INIR
  DUMMY = "REG_C+DATA-1"    ;for IND/INDR
  DUMMY = "REG_L+DATA"      ;for OUTI,OUTD,OTIR,OTDR
  FLG_C = Carry  of above "DUMMY" calculation
  FLG_H = Carry  of above "DUMMY" calculation (same as FLG_C)
  FLG_N = Sign   of "DATA"
  FLG_P = Parity of "REG_B XOR (DUMMY AND 07h)"
  FLG_S = Sign   of "REG_B"
  UNDOC = Bit3,5 of "REG_B AND 28h"
```

The above registers L and B are meant to contain the new values which are already incremented/decremented by the instruction.

Note that the official docs mis-described the N-Flag as set, and the C-Flag as not affected.

#### DAA Flags

Addition (if N was 0):

```text
  FLG_H = (OLD_A AND 0Fh) > 09h
  FLG_C = Carry of result
```

Subtraction (if N was 1):

```text
  FLG_H = (NEW_A AND 0Fh) > 09h
  FLG_C = OLD_CARRY OR (OLD_A>99h)
```

For both addition and subtraction, N remains unmodified, and S, Z, P contain "Sign", Zero, and Parity of result (A). Undocumented flags are set to (A AND 28h) as normal.

#### Mis-documented Flags

For all XOR/OR: H=N=C=0, and for all AND: H=1, N=C=0, unlike described else in Z80 docs. Also note C,N flag description bug for I/O block commands (see above).

#### Internal MEMPTR Register

This is an internal Z80 register, modified by some instructions, and usually completely hidden to the user, except that Bit 11 and Bit 13 can be read out at a later time by BIT N,(HL) instructions.

The following list specifies the resulting content of the MEMPTR register caused by the respective instructions.

```text
  Content Instruction
  A*100h  LD (xx),A               ;xx=BC,DE,nn
  xx+1    LD A,(xx)               ;xx=BC,DE,nn
  nn+1    LD (nn),rr; LD rr,(nn)  ;rr=BC,DE,HL,IX,IY
  rr      EX (SP),rr              ;rr=HL,IX,IY (MEMPTR=new value of rr)
  rr+1    ADD/ADC/SBC rr,xx       ;rr=HL,IX,IY (MEMPTR=old value of rr+1)
  HL+1    RLD and RRD
  dest    JP nn; CALL nn; JR nn   ;dest=nn
  dest    JP f,nn; CALL f,nn      ;regardless of condition true/false
  dest    RET; RETI; RETN         ;dest=value read from (sp)
  dest    RET f; JR f,nn; DJNZ nn ;only if condition=true
  00XX    RST n
  adr+1   IN A,(n)                ;adr=A*100h+n, memptr=A*100h+n+1
  bc+1    IN r,(BC); OUT (BC),r   ;adr=bc
  ii+d    All instructions with operand (ii+d)
```

Also the following might or might not affect MEMPTR, not tested yet:

```text
  OUT (N),A and block commands LDXX, CPXX, INXX, OUTXX
  and probably interrupts in IM 0, 1, 2
```

All other commands do not affect the MEMPTR register - this includes all instructions with operand (HL), all PUSH and POP instructions, not executed conditionals JR f,d, DJNZ d, RET f (ie. with condition=false), and the JP HL/IX/IY jump instructions.
