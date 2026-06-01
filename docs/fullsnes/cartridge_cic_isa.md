# SNES Cartridge CIC Instruction Set

#### CIC Registers

```text
  A  4bit Accumulator
  X  4bit General Purpose Register
  L  4bit Pointer Register (lower 4bit of 6bit HL)
  H  2bit Pointer Register (upper 2bit of 6bit HL)
  C  1bit Carry Flag (changed ONLY by "set/clr c", not by "add/adc" or so)
  PC 10bit Program Counter (3bit bank, plus 7bit polynomial counter)
```

#### CIC Memory

```text
  ROM   512x8bit (program ROM) (NES/EUR=768x8) (max 1024x8 addressable)
  RAM   32x4bit  (data RAM) (max 64x4 addressable)
  STACK 4x10bit  (stack for call/ret opcodes)
  PORTS 4x4bit   (external I/O ports & internal RAM-like ports) (max 16x4)
```

Newer CIC Opcodes (6113, D411) (and probably F411,D413,F413)

```text
  00      nop             no operation (aka "addsk A,0" opcode)
  00+n    addsk  A,n      add, A=A+n, skip if result>0Fh
  10+n    cmpsk  A,n      compare, skip if A=n
  20+n    mov    L,n      set L=n
  30+n    mov    A,n      set A=n
  40      mov    A,[HL]   set A=RAM[HL]
  41      xchg   A,[HL]   exchange A <--> RAM[HL]
  42      xchgsk A,[HL+]  exchange A <--> RAM[HL], L=L+1, skip if result>0Fh
  43      xchgsk A,[HL-]  exchange A <--> RAM[HL], L=L-1, skip if result<00h
  44      neg    A        negate, A=0-A                 ;(used by 6113 mode)
  45      ?
  46      out    [L],A    output, PORT[L]=A
  47      out    [L],0    output, PORT[L]=0
  48      set    C        set carry, C=1
  49      clr    C        reset carry, C=0
  4A      mov    [HL],A   set RAM[HL]=A
  4B      ?
  4C      ret             return, pop PC from stack
  4D      retsk           return, pop PC from stack, skip
  4E+n    ?
  52      movsk  A,[HL+]  set A=RAM[HL], L=L+1, skip if result>0Fh
  53      ?                    (guess: movsk  A,[HL-])
  54      not    A        complement, A=A XOR 0Fh
  55      in     A,[L]    input, A=PORT[L]
  56      ?
  57      xchg   A,L      exchange A <--> L
  58+n    ?
  5C      mov    X,A      set X=A
  5D      xchg   X,A      exchange X <--> A
  5E      ???             "SPECIAL MYSTERY INSTRUCTION" ;(used by 6113 mode)
  5F      ?
  60+n    testsk [HL].n   skip if RAM[HL].Bit(n)=1
  64+n    testsk A.n      skip if A.Bit(n)=1
  68+n    clr    [HL].n   set RAM[HL].Bit(n)=0
  6C+n    set    [HL].n   set RAM[HL].Bit(n)=1
  70      add    A,[HL]   add, A=A+RAM[HL]
  71      ?                    (guess: addsk  A,[HL])
  72      adc    A,[HL]   add with carry, A=A+RAM[HL]+C
  73      adcsk  A,[HL]   add with carry, A=A+RAM[HL]+C, skip if result>0Fh
  74+n    mov    H,n      set H=n  ;2bit range, n=0..3 only (used: 0..1 only)
  78+n mm jmp    nmm      long jump, PC=nmm
  7C+n mm call   nmm      long call, push PC+2, PC=nmm
  80+nn   jmp    nn       short jump, PC=(PC AND 380h)+nn
  -       reset           PC=000h
```

Note: "skip" means "do not execute next instruction"

Older CIC Opcodes (3195) (and probably 3193,3196,3197,etc.)

```text
  Exchanged opcodes 48 <--> 49 (set/clr C)
  Exchanged opcodes 44 <--> 54 (neg/not A)
  ROM Size is 768x8 (although only 512x8 are actually used)
```

#### Note

The CIC is a 4bit Sharp CPU (maybe a Sharp SM4, but no datasheet exists) (the instruction seems to be an older version of that in the Sharp SM5K1..SM5K7 datasheets).
