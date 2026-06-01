# Z80 Compatibility

The Z80 CPU is (almost) fully backwards compatible to older 8080 and 8085 CPUs.

#### Instruction Format

The Z80 syntax simplifies the chaotic 8080/8085 syntax. For example, Z80 uses the command "LD" for all load instructions, 8080/8085 used various different commands depending on whether the operands are 8bit registers, 16bit registers, memory pointers, and/or an immediates. However, these changes apply to the source code only - the generated binary code is identical for both CPUs.

#### Parity/Overflow Flag

The Z80 CPU uses Bit 2 of the flag register as Overflow flag for arithmetic instructions, and as Parity flag for other instructions. 8080/8085 CPUs are always using this bit as Parity flag for both arithmetic and non-arithmetic instructions.

#### Z80 Specific Instructions

The following instructions are available for Z80 CPUs only, but not for older 8080/8085 CPUs:

All CB-prefixed opcodes (most Shift/Rotate, all BIT/SET/RES commands).

All ED-prefixed opcodes (various instructions, and all block commands).

All DD/FD-prefixed opcodes (registers IX and IY).

As well as DJNZ nn; JR nn; JR f,nn; EX AF,AF; and EXX.

#### 8085 Specific Instructions

The 8085 instruction set includes two specific opcodes in addition to the 8080 instruction set, used to control 8085-specifc interrupts and SID and SOD input/output signals. These opcodes, RIM (20h) and SIM (30h), are not supported by Z80/8080 CPUs.

#### Z80 vs Z80A

Both Z80 and Z80A are including the same instruction set, the only difference is the supported clock frequency (Z80 = max 2.5MHz, Z80A = max 4MHz).

#### NEC-780 vs Zilog-Z80

These CPUs are apparently fully compatible to each other, including for undocumented flags and undocumented opcodes.
