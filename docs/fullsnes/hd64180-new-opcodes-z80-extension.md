# HD64180 New Opcodes (Z80 Extension)

#### New HD64180 Opcodes

```text
  ED 00 nn  IN0 B,(nn)    ED 01 nn  OUT0 (nn),B    ED 04     TST B
  ED 08 nn  IN0 C,(nn)    ED 09 nn  OUT0 (nn),C    ED 0C     TST C
  ED 10 nn  IN0 D,(nn)    ED 11 nn  OUT0 (nn),D    ED 14     TST D
  ED 18 nn  IN0 E,(nn)    ED 19 nn  OUT0 (nn),E    ED 1C     TST E
  ED 20 nn  IN0 H,(nn)    ED 21 nn  OUT0 (nn),H    ED 24     TST H
  ED 28 nn  IN0 L,(nn)    ED 29 nn  OUT0 (nn),L    ED 2C     TST L
  ED 30 nn  IN0 (nn)                               ED 34     TST (HL)
  ED 38 nn  IN0 A,(nn)    ED 39 nn  OUT0 (nn),A    ED 3C     TST A
  ED 4C     MULT BC       ED 83     OTIM           ED 64 nn  TST nn
  ED 5C     MULT DE       ED 8B     OTDM           ED 70     IN (C)
  ED 6C     MULT HL       ED 93     OTIMR          ED 74 nn  TSTIO nn
  ED 7C     MULT SP       ED 9B     OTDMR          ED 76     SLP
```

On a real Z80, ED-4C/5C/6C/7C and ED-64/74 have been mirrors of NEG.

On a real Z80, ED-70 did the same (but was undocumented).

On a real Z80, ED-76 has been mirror of IM 2.

On a real Z80, ED-00..3F and ED-80..9F have acted as NOP.

#### Notes

IN0/OUT0/OTxMx same as IN/OUT/OTxx but with I/O-address bit8-15 forced 00h.

TST op: Test A,op.  ;non-destructive AND (only flags changed) TSTIO nn: Test Port[C],nn  ;\hitachi lists BOTH definitions (page 75) TSTIO nn: Test Port[nn],A  ;/zilog also lists BOTH definitions (page 173,174) TSTIO nn: Test Port[C],nn  ;<-- this is reportedly the correct definition MLT xy: xy=x*y   ;unsigned multiply (flags=unchanged) SLP (SLEEP) stops internal clock (including stopping DRAM refresh and DMAC).

IOSTOP: stops ASCI, CSI/O, PRT.

#### Z80 incompatible opcodes (according to Zilog's Z180 Application Note)

```text
  Opcode    Z80                     Z180
  DAA       Checks Cy and A>99h     Checks Cy only? (when N=1)
  RLD/RRD   Sets flags for A        Sets flags for [HL]
```

#### Opcode Execution Time

Some opcodes are slightly faster as on real Z80. For example, some (not all) 4-cycle Z80 opcodes take only 3-cyles on HD64180.

#### Undefined Opcodes

On the HD64180, undefined opcodes are causing a TRAP exception (this feature cannot be disabled). So, while the real Z80 does have some useful (and some useless) undocumented opcodes, none (?) of these is working on HD64180 (except for the now-official ED-70 opcode).

The HD64180 datasheet doesn't list "SLL" as valid opcode.

The HD64180 datasheet doesn't list the "SET-and-LD" or "RES-and-LD" opcodes.

The HD64180 datasheet doesn't list opcodes with "IXL,IXH,IYL,IYH" operands, however, it does mention existence of "IXL" here and there (however, that seems to refer only to 16bit operations like "PUSH IX" (which do internally split 16bit IX into two 8bit units).

The HD64180 datasheet lists EX DE,HL with IX/IY-prefix as invalid.

NEWER INFO:

The HD64180 is actually trapping all undocumented opcodes, even those that are more or less commonly used on Z80 CPUs, ie. the HD64180 doesn't support accessing IX/IY 16bit registers as 8bit fragments (IXH,IXL,IYH,IYL), doesn't support "SLL" opcode, nor useless opcode mirrors (like alternate NEG/IM/RETN/RETI/NOP mirrors).
