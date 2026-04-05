# ARM Opcodes: Branch and Branch with Link (B, BL, SWI)

Branch and Branch with Link (B, BL) Branch (B) is supposed to jump to a subroutine. Branch with Link is meant to be used to call to a subroutine, return address is then saved in R14/LR (and can be restored via MOV PC,LR aka MOV R15,R14) (for nested subroutines, use PUSH LR and POP PC).

```text
  Bit    Expl.
  31-28  Condition
  27-25  Must be "101" for this instruction
  24     Opcode (0-1)
          0: B{cond} label    ;branch      (jump)    PC=PC+8+nn*4
          1: BL{cond} label   ;branch/link (call)    PC=PC+8+nn*4, LR=PC+4
  23-0   nn - Signed Offset, step 4      (-32M..+32M in steps of 4)
```

#### Execution Time: 2S + 1N

Return: No flags affected.

Branch via ALU, LDR, LDM

Most ALU, LDR, LDM opcodes can also change PC/R15.

#### Mis-aligned PC/R15 (MOV/ALU/LDR with Rd=R15)

For ARM code, the low bits of the target address should be usually zero, otherwise, R15 is forcibly aligned by clearing the lower two bits.

In short, R15 will be always forcibly aligned, so mis-aligned branches won't have effect on subsequent opcodes that use R15, or [R15+disp] as operand.

#### Software Interrupt (SWI) (svc exception)

SWI supposed for calls to the operating system - Enter Supervisor mode (SVC).

```text
  Bit    Expl.
  31-28  Condition
  27-24  Opcode
          1111b: SWI{cond} nn   ;software interrupt
  23-0   nn - Comment Field, ignored by processor (24bit value)
```

#### Execution Time: 2S+1N

The exception handler may interprete the Comment Field by examining the lower 24bit of the 32bit opcode opcode at [R14_svc-4].

For Returning from SWI use "MOVS PC,R14", that instruction does restore both PC and CPSR, ie. PC=R14_svc, and CPSR=SPSR_svc.

Nesting SWIs: SPSR_svc and R14_svc should be saved on stack before either invoking nested SWIs, or (if the IRQ handler uses SWIs) before enabling IRQs.

#### Undefined Instruction (und exception)

```text
  Bit    Expl.
  31-28  Condition
  27-25  Must be 011b for this instruction
  24-5   Reserved for future use
  4      Must be 1b for this instruction
  3-0    Reserved for future use
```

No assembler mnemonic exists, following bitstreams are (not) reserved.

```text
  cond011xxxxxxxxxxxxxxxxxxxx1xxxx - reserved for future use (except below).
  cond01111111xxxxxxxxxxxx1111xxxx - free for user.
```

Execution time: 2S+1I+1N.
