# CPU Jump and Control Instructions

#### Normal Jumps

```text
  Opcode       Flags Clks  Native       Nocash        Effect
  80 dd        ------  3xx BRA disp8    JMP disp      PC=PC+/-disp8
  82 dd dd     ------  4   BRL disp16   JMP disp      PC=PC+/-disp16
  4C nn nn     ------  3   JMP nnnn     JMP nnnn      PC=nnnn
  5C nn nn nn  ------  4   JMP nnnnnn   JMP nnnnnn    PB:PC=nnnnnn
  6C nn nn     ------  5   JMP (nnnn)   JMP [nnnn]    PC=WORD[00:nnnn]
  7C nn nn     ------  6   JMP (nnnn,X) JMP [nnnn+X]  PC=WORD[PB:nnnn+X]
  DC nn nn     ------  6   JML ...      JMP FAR[nnnn] PB:PC=[00:nnnn]
  20 nn nn     ------  6   JSR nnnn     CALL nnnn     [S]=PC+2,PC=nnnn
  22 nn nn nn  ------  4   JSL nnnnnn   CALL nnnnnn   PB:PC=nnnnnn [S]=PB:PC+3
  FC nn nn     ------  6   JSR (nnnn,X) CALL [nnnn+X] PC=WORD[PB:nnnn+X] [S]=PC
  40           nzcidv  6   RTI          RETI          P=[S+1],PB:PC=[S+2],S=S+4
  6B           ------  ?   RTL          RETF          PB:PC=[S+1]+1, S=S+3
  60           ------  6   RTS          RET           PC=[S+1]+1, S=S+2
```

Note: RTI cannot modify the B-Flag or the unused flag.

Glitch: For JMP [nnnn] the operand word cannot cross page boundaries, ie. JMP [03FFh] would fetch the MSB from [0300h] instead of [0400h]. Very simple workaround would be to place a ALIGN 2 before the data word.

Conditional Branches (Branch on condition, to PC=PC+/-nn)

```text
  Opcode    Flags Clks  Native  Nocash        Condition (jump if)
  10 dd     ------  2** BPL     JNS     disp  ;N=0 (plus/positive)
  30 dd     ------  2** BMI     JS      disp  ;N=1 (minus/negative/signed)
  50 dd     ------  2** BVC     JNO     disp  ;V=0 (no overflow)
  70 dd     ------  2** BVS     JO      disp  ;V=1 (overflow)
  90 dd     ------  2** BCC/BLT JNC/JB  disp  ;C=0 (less/below/no carry)
  B0 dd     ------  2** BCS/BGE JC/JAE  disp  ;C=1 (above/greater/equal/carry)
  D0 dd     ------  2** BNE/BZC JNZ/JNE disp  ;Z=0 (not zero/not equal)
  F0 dd     ------  2** BEQ/BZS JZ/JE   disp  ;Z=1 (zero/equal)
```

** The execution time is 2 cycles if the condition is false (no branch executed). Otherwise, 3 cycles if the destination is in the same memory page, or 4 cycles if it crosses a page boundary (see below for exact info).

Note: After subtractions (SBC or CMP) carry=set indicates above-or-equal, unlike as for 80x86 and Z80 CPUs.

Interrupts, Exceptions, Breakpoints

```text
  Opcode                                                      6502     65C816
  00   BRK    Break      B=1 [S]=$+2,[S]=P,D=0 I=1, PB=00, PC=[00FFFE] [00FFE6]
  02   COP    ;65C816    B=1 [S]=$+2,[S]=P,D=0 I=1, PB=00, PC=[00FFF4] [00FFE4]
  --   /ABORT ;65C816                               PB=00, PC=[00FFF8] [00FFE8]
  --   /IRQ   Interrupt  B=0 [S]=PC, [S]=P,D=0 I=1, PB=00, PC=[00FFFE] [00FFEE]
  --   /NMI   NMI        B=0 [S]=PC, [S]=P,D=0 I=1, PB=00, PC=[00FFFA] [00FFEA]
  --   /RESET Reset      D=0 E=1 I=1 D=0000, DB=00  PB=00, PC=[00FFFC] N/A
```

Notes: IRQs can be disabled by setting the I-flag, a BRK command, a NMI, and a /RESET signal cannot be masked by setting I.

Exceptions do first change the B-flag (in 6502 mode), then write P to stack, and then set the I-flag, the D-flag IS cleared (unlike as on original 6502).

In 6502 mode, the same vector is shared for BRK and IRQ, software can separate between BRK and IRQ by examining the pushed B-flag only.

The RTI opcode can be used to return from BRK/IRQ/NMI, note that using the return address from BRK skips one dummy/parameter byte following after the BRK opcode.

Software or hardware must take care to acknowledge or reset /IRQ or /NMI signals after processing it.

```text
  IRQs are executed whenever "/IRQ=LOW AND I=0".
  NMIs are executed whenever "/NMI changes from HIGH to LOW".
```

If /IRQ is kept LOW then same (old) interrupt is executed again as soon as setting I=0. If /NMI is kept LOW then no further NMIs can be executed.

#### CPU Control

```text
  Opcode    Flags Clks  Native    Nocash    Effect
  18        --0---  2   CLC       CLC       C=0    ;Clear carry flag
  58        ---0--  2   CLI       EI        I=0    ;Clear interrupt disable bit
  D8        ----0-  2   CLD       CLD       D=0    ;Clear decimal mode
  B8        -----0  2   CLV       CL?       V=0    ;Clear overflow flag
  38        --1---  2   SEC       STC       C=1    ;Set carry flag
  78        ---1--  2   SEI       DI        I=1    ;Set interrupt disable bit
  F8        ----1-  2   SED       STD       D=1    ;Set decimal mode
  C2 nn     xxxxxx  3   REP #nn   CLR P,nn  P=P AND NOT nn
  E2 nn     xxxxxx  3   SEP #nn   SET P,nn  P=P OR nn
  FB        --c---  2   XCE       XCE       C=E, E=C
```

#### Special Opcodes

```text
  Opcode    Flags Clks  Native      Nocash
  DB        ------  -   STP         KILL      ;STOP/KILL
  EB        nz----  3   XBA         SWAP A    ;A=B, B=A, NZ=LSB
  CB        ------  3x  WAI         HALT      ;HALT
  42 nn             2   WDM #nn     NUL nn    ;No operation
  EA        ------  2   NOP         NOP       ;No operation
```

WAI/HALT stops the CPU until an exception (usually an IRQ or NMI) request occurs; in case of IRQs this works even if IRQs are disabled (via I=1).

#### Conditional Branch Page Crossing

The branch opcode with parameter takes up two bytes, causing the PC to get incremented twice (PC=PC+2), without any extra boundary cycle. The signed parameter is then added to the PC (PC+disp), the extra clock cycle occurs if the addition crosses a page boundary (next or previous 100h-page).
