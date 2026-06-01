# CPU Arithmetic/Logical Operations

#### ALU Opcodes

```text
  Base  Flags    Native   Nocash     Operands    Name      Function
  00    nz----   ORA op   OR  A,op   <alu_types> OR        A=A OR op
  20    nz----   AND op   AND A,op   <alu_types> AND       A=A AND op
  40    nz----   EOR op   XOR A,op   <alu_types> XOR       A=A XOR op
  60    nzc--v   ADC op   ADC A,op   <alu_types> Add       A=A+C+op
  E0    nzc--v   SBC op   SBC A,op   <alu_types> Subtract  A=A+C-1-op
  C0    nzc---   CMP op   CMP A,op   <alu_types> Compare   A-op
  E0    nzc---   CPX op   CMP X,op   <cpx_types> Compare   X-op
  C0    nzc---   CPY op   CMP Y,op   <cpx_types> Compare   Y-op
```

alu_types (Operands for OR,AND,XOR,ADC,SBC,CMP Opcodes)

```text
  Opcode        Clks  Native   Nocash     Name          Effect
  Base+09 nn      2   #nn      nn         Immediate     nn
  Base+05 nn      3   nn       [nn]       Zero Page     [D+nn]
  Base+15 nn      4   nn,X     [nn+X]     Zero Page,X   [D+nn+X]
  Base+0D nn nn   4   nnnn     [nnnn]     Absolute      [DB:nnnn]
  Base+1D nn nn   4*  nnnn,X   [nnnn+X]   Absolute,X    [DB:nnnn+X]
  Base+19 nn nn   4*  nnnn,Y   [nnnn+Y]   Absolute,Y    [DB:nnnn+Y]
  Base+01 nn      6   (nn,X)   [[nn+X]]   (Indirect,X)  [WORD[D+nn+X]]
  Base+11 nn      5*  (nn),Y   [[nn]+Y]   (Indirect),Y  [WORD[D+nn]+Y]
  Base+12 nn          (nn)     [[nn]]     (Indirect)    [WORD[D+nn]]
  Base+03 nn          nn,S     [nn+S]                   [nn+S]
  Base+13 nn          (nn,S),Y [[nn+S]+Y]               [WORD[nn+S]+Y]
  Base+07 nn          [nn]     [FAR[nn]]                [FAR[D+nn]]
  Base+17 nn          [nn],y   [FAR[nn]+Y]              [FAR[D+nn]+Y]
  Base+0F nn nn nn    nnnnnn   [nnnnnn]                 [nnnnnn]
  Base+1F nn nn nn    nnnnnn,X [nnnnnn+X]               [nnnnnn+X]
```

* Add one cycle if indexing crosses a page boundary.

cpx_types (Operands for CMP Opcodes with X,Y Operand)

```text
  Opcode        Clks  Native   Nocash     Name                Effect
  Base+00 nn      2   #nn      nn         Immediate           nn
  Base+04 nn      3   nn       [nn]       Zero Page           [D+nn]
  Base+0C nn nn   4   nnnn     [nnnn]     Absolute            [DB:nnnn]
```

#### Bit Test

```text
  Opcode    Flags Clks  Native     Nocash             Operand
  24 nn     xz---x  3   BIT nn     TEST A,[nn]        [D+nn]
  2C nn nn  xz---x  4   BIT nnnn   TEST A,[nnnn]      [DB:nnnn]
  34 nn     xz---x      BIT nn,X   TEST A,[nn+X]      [D+nn+X]
  3C nn nn  xz---x      BIT nnnn,X TEST A,[nnnn+X]    [DB:nnnn+X]
  89 nn     -z----      BIT #nn    TEST A,nn          nn
```

Flags are set as "Z=((A AND op)=0)", and "N=op.Bit(MSB)", and "V=op.Bit(MSB-1)". Where MSB=bit15/bit7, and MSB-1=bit14/bit6 (depending on M-flag). Note that N and V do rely only on "op" (ie. not on "A AND op").

#### Increment by one

```text
  Opcode          Clks  Native     Nocash       Effect
  E6 nn     nz----  5   INC nn     INC [nn]     [D+nn]=[D+nn]+1
  F6 nn     nz----  6   INC nn,X   INC [nn+X]   [D+nn+X]=[D+nn+X]+1
  EE nn nn  nz----  6   INC nnnn   INC [nnnn]   [DB:nnnn]=[DB:nnnn]+1
  FE nn nn  nz----  7   INC nnnn,X INC [nnnn+X] [DB:nnnn+X]=[DB:nnnn+X]+1
  E8        nz----  2   INX        INC X        X=X+1
  C8        nz----  2   INY        INC Y        Y=Y+1
  1A        nz----  2   INA        INC A        A=A+1
```

#### Decrement by one

```text
  Opcode          Clks  Native     Nocash       Effect
  C6 nn     nz----  5   DEC nn     DEC [nn]     [D+nn]=[D+nn]-1
  D6 nn     nz----  6   DEC nn,X   DEC [nn+X]   [D+nn+X]=[D+nn+X]-1
  CE nn nn  nz----  6   DEC nnnn   DEC [nnnn]   [DB:nnnn]=[DB:nnnn]-1
  DE nn nn  nz----  7   DEC nnnn,X DEC [nnnn+X] [DB:nnnn+X]=[DB:nnnn+X]-1
  CA        nz----  2   DEX        DEC X        X=X-1
  88        nz----  2   DEY        DEC Y        Y=Y-1
  3A        nz----  2   DEA        DEC A        A=A-1
```

#### TSB/TRB (Test and Set/Reset)

```text
  Opcode          Clks  Native      Nocash
  04 nn     -z----  5   TSB nn      SET [nn],A    ;\"TEST op,A" --> z
  0C nn nn  -z----  6   TSB nnnn    SET [nnnn],A  ;/then "OR op,A"
  14 nn     -z----  5   TRB nn      CLR [nn],A    ;\"TEST op,A" --> z
  1C nn nn  -z----  6   TRB nnnn    CLR [nnnn],A  ;/then "AND op,NOT A"
```
