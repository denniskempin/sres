# SNES APU SPC700 CPU ALU Commands

#### 8bit ALU Operations

```text
  OR   a,b       OR   a,b        00+x ...    .. a=a OR b               N.....Z.
  AND  a,b       AND  a,b        20+x ...    .. a=a AND b              N.....Z.
  EOR  a,b       XOR  a,b        40+x ...    .. a=a XOR b              N.....Z.
  CMP  a,b       CMP  a,b        60+x ...    .. a-b                    N.....ZC
  ADC  a,b       ADC  a,b        80+x ...    .. a=a+b+C                NV..H.ZC
  SBC  a,b       SBC  a,b        A0+x ...    .. a=a-b-not C            NV..H.ZC
```

Above OR/AND/EOR/CMP/ADC/SBC can be used with following operands:

```text
  cmd  A,#nn     cmd  A,nn       x+08 nn     2  A,nn
  cmd  A,(X)     cmd  A,[X]      x+06        3  A,[X]
  cmd  A,aa      cmd  A,[aa]     x+04 aa     3  A,[aa]
  cmd  A,aa+X    cmd  A,[aa+X]   x+14 aa     4  A,[aa+X]
  cmd  A,!aaaa   cmd  A,[aaaa]   x+05 aa aa  4  A,[aaaa]
  cmd  A,!aaaa+X cmd  A,[aaaa+X] x+15 aa aa  5  A,[aaaa+X]
  cmd  A,!aaaa+Y cmd  A,[aaaa+Y] x+16 aa aa  5  A,[aaaa+Y]
  cmd  A,[aa]+Y  cmd  A,[[aa]+Y] x+17 aa     6  A,[[aa]+Y]
  cmd  A,[aa+X]  cmd  A,[[aa+X]] x+07 aa     6  A,[[aa+X]]
  cmd  aa,bb     cmd  [aa],[bb]  x+09 bb aa  6  [aa],[bb]
  cmd  aa,#nn    cmd  [aa],nn    x+18 nn aa  5  [aa],nn
  cmd  (X),(Y)   cmd  [X],[Y]    x+19        5  [X],[Y]
```

Compare can additionally have the following forms:

```text
  CMP  X,#nn     CMP  X,nn       C8 nn       2  X-nn                   N.....ZC
  CMP  X,aa      CMP  X,[aa]     3E aa       3  X-[aa]                 N.....ZC
  CMP  X,!aaaa   CMP  X,[aaaa]   1E aa aa    4  X-[aaaa]               N.....ZC
  CMP  Y,#nn     CMP  Y,nn       AD nn       2  Y-nn                   N.....ZC
  CMP  Y,aa      CMP  Y,[aa]     7E aa       3  Y-[aa]                 N.....ZC
  CMP  Y,!aaaa   CMP  Y,[aaaa]   5E aa aa    4  Y-[aaaa]               N.....ZC
```

Note: There's also a compare and jump if non-zero command (see jumps).

#### 8bit Increment/Decrement and Rotate/Shift Commands

```text
  ASL  a         SHL  a          00+x ..     .. Left shift, bit0=0     N.....ZC
  ROL  a         RCL  a          20+x ..     .. Left shift, bit0=C     N.....ZC
  LSR  a         SHR  a          40+x ..     .. Right shift, bit7=0    N.....ZC
  ROR  a         RCR  a          60+x ..     .. Right shift, bit7=C    N.....ZC
  DEC  a         DEC  a          80+x ..     .. a=a-1                  N.....Z.
  INC  a         INC  a          A0+x ..     .. a=a+1                  N.....Z.
```

The Increment/Decrement and Rotate/Shift commands can have following forms:

```text
  cmd  A         cmd  A          x+1C        2  A
  cmd  X         cmd  X          x+1D-80     2  X    ;\increment/decrement only
  cmd  Y         cmd  Y          x+DC-80     2  Y    ;/(not rotate/shift)
  cmd  aa        cmd  [aa]       x+0B aa     4  [aa]
  cmd  aa+X      cmd  [aa+X]     x+1B aa     5  [aa+X]
  cmd  !aaaa     cmd  [aaaa]     x+0C aa aa  5  [aaaa]
```

Note: There's also a decrement and jump if non-zero command (see jumps).

#### 16bit ALU Operations

```text
  ADDW YA,aa     ADDW YA,[aa]    7A aa       5  YA=YA+Word[aa]         NV..H.ZC
  SUBW YA,aa     SUBW YA,[aa]    9A aa       5  YA=YA-Word[aa]         NV..H.ZC
  CMPW YA,aa     CMPW YA,[aa]    5A aa       4  YA-Word[aa]            N.....ZC
  INCW aa        INCW [aa]       3A aa       6  Word[aa]=Word[aa]+1    N.....Z.
  DECW aa        DECW [aa]       1A aa       6  Word[aa]=Word[aa]-1    N.....Z.
  DIV  YA,X      DIV  YA,X       9E         12  A=YA/X, Y=YA MOD X     NV..H.Z.
  MUL  YA        MUL  YA         CF          9  YA=Y*A, NZ on Y only   N.....Z.
```

For ADDW/SUBW, H is carry from bit11 to bit12.

#### 1bit ALU Operations

```text
  CLR1 aa.b      CLR  [aa].b     b*20+12 aa  4  [aa].bit_b=0           ........
  SET1 aa.b      SET  [aa].b     b*20+02 aa  4  [aa].bit_b=1           ........
  NOT1 aaa.b     NOT  [aaa].b    EA aa ba    5  invert [aaa].bit_b     ........
  MOV1 aaa.b,C   MOV  [aaa].b,C  CA aa ba    6  [aaa].bit_b=C          ........
  MOV1 C,aaa.b   MOV  C,[aaa].b  AA aa ba    4  C=[aaa].bit_b          .......C
  OR1  C,aaa.b   OR   C,[aaa].b  0A aa ba    5  C=C OR [aaa].bit_b     .......C
  OR1  C,/aaa.b  OR   C,not[].b  2A aa ba    5  C=C OR not[aaa].bit_b  .......C
  AND1 C,aaa.b   AND  C,[aaa].b  4A aa ba    4  C=C AND [aaa].bit_b    .......C
  AND1 C,/aaa.b  AND  C,not[].b  6A aa ba    4  C=C AND not[aaa].bit_b .......C
  EOR1 C,aaa.b   XOR  C,[aaa].b  8A aa ba    5  C=C XOR [aaa].bit_b    .......C
  CLRC           CLR  C          60          2  C=0                    .......0
  SETC           SET  C          80          2  C=1                    .......1
  NOTC           NOT  C          ED          3  C=not C                .......C
  CLRV           CLR  V,H        E0          2  V=0, H=0               .0..0...
```

#### Special ALU Operations

```text
  DAA   A        DAA  A          DF          3  BCD adjust after ADC   N.....ZC
  DAS   A        DAS  A          BE          3  BCD adjust after SBC   N.....ZC
  XCN   A        XCN  A          9F          5  A = (A>>4) | (A<<4)    N.....Z.
  TCLR1 !aaaa    TCLR [aaaa],A   4E aa aa    6  [aaaa]=[aaaa]AND NOT A N.....Z.
  TSET1 !aaaa    TSET [aaaa],A   0E aa aa    6  [aaaa]=[aaaa]OR A      N.....Z.
```

For TCLR/TSET, Z+N flags are set as for "CMP A,[aaaa]" (before [aaaa] gets changed).

Reportedly, TCLR/TSET can access only 0000h..7FFFh, that is nonsense.
