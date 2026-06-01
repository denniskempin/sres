# SNES Cart GSU-n CPU Pseudo Opcodes

#### Official GSU Pseudo/Macro Opcodes

```text
  --            3 000---- LEA Rn,yyxx    ;Alias for IWT, without "#"
  --            - 000---- MOVE Rn,#hilo  ;Alias for IBT/IWT (depending on size)
  --            - 000---- MOVE Rn,(xx)   ;Alias for LM/LMS (depending on size)
  --            - 000---- MOVE (xx),Rn   ;Alias for SM/SMS (depending on size)
  --            - 000---- MOVEB Rn,(Rm)  ;Alias for LDB/TO+LDB (depending Rn)
  --            - 000---- MOVEB (Rm),Rn  ;Alias for STB/FROM+STB
  --            - 000---- MOVEW Rn,(Rm)  ;Alias for LDW/TO+LDW (depending Rn)
  --            - 000---- MOVEW (Rm),Rn  ;Alias for STW/FROM+STW
```

Above are official pseudo opcodes for native syntax (the nocash syntax "MOV" opcode is doing that things by default).

#### Nocash GSU Pseudo Opcodes

```text
   jmp  nnnn      alias for "mov r15,nnnn"
   jz/jnz/jae/jb  alias for "je/jne/jc/jnc"
```

Further possible pseudo opcodes (not yet supported in a22i):

```text
   push rs        mov [r10],rs, 2xinc_r10     ;\INCREASING on PUSH? or MEMFILL?
   pop  rd        2xdec_r10, mov rd,[r10]     ;/ (see Star Fox 1:ACA4)
   cmp  rn,0      alias for "sub rn,rn,0"
   call           alias for link+jmp
   ret            alias for jmp r11
   alu  rd,op     short for "alu rd,rs,op"
   and  rd,rs,n   alias for "bic rd,rs,not n"
```
