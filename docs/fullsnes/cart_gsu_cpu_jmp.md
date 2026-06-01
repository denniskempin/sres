# SNES Cart GSU-n CPU JMP and Prefix Opcodes

#### GSU Special Opcodes

```text
  Opcode     Clks Flags   Native    Nocash
  00            1 000---- STOP      stop  ;SFR.GO=0, SFR.IRQ=1, R15=$+2
  01            1 000---- NOP       nop   ;NOP (often used as dummy after jump)
  02           1* 000---- CACHE     cache ;IF CBR<>PC&FFF0 then CBR=PC&FFF0
```

STOP at $+0 does prefetch another opcode byte at $+1 (but without executing it), and does then stop with R15=$+2, SFR.GO=0, SFR.IRQ=1 (that, even if IRQ is disabled in CFGR.IRQ).

BUG: On MC1 (maybe also GSU1), STOP hangs when executed after a RAM write (there must be at least 2 cycles after write, eg. insert two NOPs before STOP; the required delay might vary depending on CPU speed or code cache? the bug doesn't occur on GSU2).

#### GSU Jump Opcodes

```text
  Opcode     Clks Flags   Native    Nocash
  05 nn         2 ------- BRA addr  jr  addr   ;Always, R15=R15+signed(nn)
  06 nn         2 ------- BGE addr  jge addr   ;If (S XOR V)=0 then ..
  07 nn         2 ------- BLT addr  jl  addr   ;If (S XOR V)=1 then ..
  08 nn         2 ------- BNE addr  jne addr   ;If ZF=0 then R15=R15+signed(nn)
  09 nn         2 ------- BEQ addr  je  addr   ;If ZF=1 then R15=R15+signed(nn)
  0A nn         2 ------- BPL addr  jns addr   ;If SF=0 then R15=R15+signed(nn)
  0B nn         2 ------- BMI addr  js  addr   ;If SF=1 then R15=R15+signed(nn)
  0C nn         2 ------- BCC addr  jnc addr   ;If CY=0 then R15=R15+signed(nn)
  0D nn         2 ------- BCS addr  jc  addr   ;If CY=1 then R15=R15+signed(nn)
  0E nn         2 ------- BVC addr  jno addr   ;If OV=0 then R15=R15+signed(nn)
  0F nn         2 ------- BVS addr  jo  addr   ;If OV=1 then R15=R15+signed(nn)
  9n            1 000---- JMP Rn    jmp Rn     ;R15=Rn                ;n=8..13!
  3D 9n         2 000---- LJMP Rn   jmp Rn:Rs  ;R15=Rs, PBR=Rn, CBR=? ;n=8..13!
  3C            1 000-s-z LOOP    loop r12,r13 ;r12=r12-1, if Zf=0 then R15=R13
  9n            1 000---- LINK #n link r11,addr;R11=R15+n             ;n=1..4
```

Jumps can be also implemented by using R15 (PC) as destination register (eg. in MOV/ALU commands).

Observe that the NEXT BYTE after any jump/branch opcodes is fetched before continuing at the jump destination address. The fetched byte is executed after the jump, but before executing following opcodes at the destination (in case of multi-byte opcodes, this results in a 1-byte-fragment being located after the jump-origin, and the remaining byte(s) at the destination).

#### GSU Prefix Opcodes

ALT1/ALT2/ALT3 prefixes do change the operation of an opcode, these are usually implied in the opcode description (for example, "3F 6n" is "CMP R0,Rn").

TO/WITH/FROM prefixes allow to select source/destination registers (otherwise R0 is used as default register) (for example, "Bs 3F 6n" is "CMP Rs,Rn").

The prefixes are normally reset after execution of any opcode, the only exception are the Bxx (branch) opcodes, these leave prefixes unchanged (allowing to "split" opcodes, for example placing ALT1/TO/etc. before Bxx, and the next opcode byte after Bxx).

Aside from setting Sreg+Dreg, WITH does additionally set the B-flag, this causes any following 1nh/Bnh bytes to act as MOVE/MOVES opcodes (rather than as TO/FROM prefixes).

```text
  Opcode Clks Flags   Name    Bflg ALT1 ALT2 Rs Rd
  3D        1 -1----- ALT1     -    1    -   -  -  ;prefix for 3D xx opcodes
  3E        1 --1---- ALT2     -    -    1   -  -  ;prefix for 3E xx opcodes
  3F        1 -11---- ALT3     -    1    1   -  -  ;prefix for 3F xx opcodes
  1n        1 ------- TO Rn    -    -    -   -  Rn ;select Rn as Rd
  2n        1 1------ WITH Rn  1    -    -   Rn Rn ;select Rn as Rd & Rs
  Bn        1 ------- FROM Rn  -    -    -   Rn -  ;select Rn as Rs
  05..0F nn 2 ------- Bxx addr -    -    -   -  -  ;branch opcodes (no change)
  other    .. 000---- other    0    0    0   R0 R0 ;other opcodes (reset all)
```

Other opcodes do reset B=0, ALT1=0, ALT2=0, Sreg=R0, Dreg=R0; that does really apply to ALL other opcodes, namely including JMP/LOOP (unlike Bxx branches), NOP (ie. NOP isn't exactly <no> operation), MOVE/MOVES (where 1n/Bn are treated as 'real' opcodes rather than as TO/FROM prefixes).

#### Ignored Prefixes

ALT1/ALT2 prefixes are ignored if the opcode doesn't exist (eg. if "3D xx" doesn't exist, then the CPU does instead execute "xx") (normally, doing that wouldn't make any sense, however, "Doom" is using ALT1/ALT2 alongside with conditional jumps, resulting in situations where the prefix is used/ignored depending on the jump condition).

ALT3 does reportedly mirror to ALT1 (eg. if "3F xx" doesn't exist, then it acts as "3D xx", and, if that doesn't either, as "xx").

TO/WITH/FROM are ignored if the following opcode doesn't use Dreg/Sreg.

#### Program Counter (R15) Notes

R15 can be used as source operand in MOV/ALU opcodes (and is also implied as such in Bxx,LINK,CACHE opcodes); in all cases R15 contains the address of the next opcode.
