# CPU Clock Cycles

#### SNES Memory Speed

```text
  Memory Area      Speed          Clks Comment
  00-3F:0000-1FFF  Medium 2.68MHz 8    WRAM (8K mirror of 7E:0000-1FFF)
  00-3F:2000-3FFF  Fast   3.58MHz 6    I/O and Expansion
  00-3F:4000-41FF  Slow   1.78MHz 12   Manual Joypad Reading
  00-3F:4200-5FFF  Fast   3.58MHz 6    I/O and Expansion
  00-3F:6000-7FFF  Medium 2.68MHz 8    SRAM and Expansion
  00-3F:8000-FFFF  Medium 2.68MHz 8    ROM (32K banks)
  40-7F:0000-FFFF  Medium 2.68MHz 8    ROM/SRAM/WRAM
  80-BF:0000-1FFF  Medium 2.68MHz 8    WRAM (8K mirror of 7E:0000-1FFF)
  80-BF:2000-3FFF  Fast   3.58MHz 6    I/O and Expansion
  80-BF:4000-41FF  Slow   1.78MHz 12   Manual Joypad Reading
  80-BF:4200-5FFF  Fast   3.58MHz 6    I/O and Expansion
  80-BF:6000-7FFF  Medium 2.68MHz 8    SRAM and Expansion
  80-BF:8000-FFFF  Variable       6/8  ROM (32K banks)  ;\speed selectable
  C0-FF:0000-FFFF  Variable       6/8  ROM (64K banks)  ;/via port 420Dh
  Internal Cycles  Fast   3.58MHz 6    Internal cycles (eg. 2nd cycle in NOPs)
```

#### Implied or Immediate Operands

```text
  CN        2     Opcodes without memory/immediate parameters
  CNN       3     XBA/WAI/STP (swap A, wait irq, stop)
  CPp       2,3   nn or nnnn                                    ;+p if 16bit
  CPN       3     nn  (REP/SEP)
```

#### Memory Operands

```text
  CPnDd     3,4,5 [nn+d]                 ;+n if (D AND 00FFh)>0, +d  if 16bit
  CPnDdNDd  5..8  [nn+d] (RMW)           ;+n if (D AND 00FFh)>0, +dd if 16bit
  CPnNDd    4,5,6 [nn+x+d] or [nn+y+d]   ;+n if (D AND 00FFh)>0, +d  if 16bit
  CPnNDdNDd 6..9  [nn+x+d] (RMW)         ;+n if (D AND 00FFh)>0, +dd if 16bit
  CPNDd     4,5   [nn+s]                                        ;+d  if 16bit
  CPPDd     4,5   [nnnn]                                        ;+d  if 16bit
  CPPDdNDd  6,8   [nnnn] (RMW)                                  ;+dd if 16bit
  CPPyDd    4,5,6 [nnnn+x] or [nnnn+y] (RD)    +y     xxx       ;+d  if 16bit
  CPPNDd    5,6   [nnnn+x] or [nnnn+y] (WR)                     ;+d  if 16bit
  CPPNDdNDd 7,9   [nnnn+x] (RMW)                                ;+dd if 16bit
  CPPPDd    5,6   [nnnnnn] or [nnnnnn+x]                        ;+d  if 16bit
  CPnAADd   5,6,7 [[nn+d]]               ;+n if (D AND 00FFh)>0, +d  if 16bit
  CPnAAyDd  5..8  [[nn+d]+y] (RD)              +n+y   xxx       ;+d  if 16bit
  CPnAANDd  6..8  [[nn+d]+y] (WR)        ;+n if (D AND 00FFh)>0, +d  if 16bit
  CPnNAADd  6,7,8 [[nn+d+x]]             ;+n if (D AND 00FFh)>0, +d  if 16bit
  CPNAANDd  7,8   [[nn+s]+y]                                    ;+d  if 16bit
  CPnAAADd  6,7,8 [far[nn+d]] or [far[nn+d]+y]  ;+n if (D ..)>0, +d  if 16bit
  CPPDDNN   7     ldir/lddr
```

#### Push/Pop

```text
  CNsS      3,4   PUSH register                                 ;+s  if 16bit
  CNNSs     4,5   POP register                                  ;+s  if 16bit
  CPnDDSS   6,7   PUSH word[nn+d] ("PEI") ;+n if (D AND 00FFh)>0
  CPPSS     5     PUSH nnnn       ("PEA")
  CPPNSS    6     PUSH $+/-nnnn   ("PER")
```

#### Jumps

```text
  CP        2     relative 8bit jump (condition false)
  CPNx      3,4   relative 8bit jump   ;+x if "E=1 and crossing 100h-boundary"
  CPPN      4     relative 16bit jump
  CPP       3     Jump nnnn
  CPPP      4     Jump nnnnnn
  CPPNSS    6     Call nnnn
  CPPSNPSS  8(7?) Call nnnnnn
  CPPDD     5     jump [nnnn]
  CPPNDD    6     jump [nnnn+X]
  CPPDDD    6     jump far[nnnn]
  CPSSPNDD  8     call [nnnn+X]
  CNNSSN    6     RTS (ret)
  CNNSSS    6     RTL (retf)
  CNNSSSs   6,7   RTI (reti)                               ;+s if E=0
  NNsSSSDD  7,8   Exception (/ABORT, /IRQ, /NMI, /RES)     ;+s if E=0
  CPsSSSDD  7,8   Exception (BRK, COP)                     ;+s if E=0
```

#### Legend

```text
  C   Opcode command
  P   Opcode parameter (immediate or address)
  A   Address cycles (on double-indirect addresses)
  D   Data cycles
  N   Internal cycles
  S   Stack cycles
```

#### Additional cycles

```text
  d   Data (MSB in 16bit modes, ie. when M/X=0)
  s   Stack (MSB in 16bit modes, or BANK in 65C816-mode exceptions)
  n   Internal cycle, when (D AND 00FFh)>0
  x   Internal cycle, when E=1 and rel-jump crossing 100h-boundary
  y   Internal cycle, when X=0 or indexing across page boundaries
```
