# Z80 Meaningless and Duplicated Opcodes

#### Mirrored Instructions

NEG  (ED44) is mirrored to ED4C,54,5C,64,6C,74,7C.

RETN (ED45) is mirrored to ED55,65,75.

RETI (ED4D) is mirrored to ED5D,6D,7D.

#### Mirrored IM Instructions

IM 0,X,1,2 (ED46,4E,56,5E) are mirrored to ED66,6E,76,7E.

Whereas IM X is an undocumented mirrored instruction itself which appears to be identical to either IM 0 or IM 1 instruction (?).

#### Duplicated LD HL Instructions

LD (nn),HL (opcode 22NNNN) is mirrored to ED63NNNN.

LD HL,(nn) (opcode 2ANNNN) is mirrored to ED6BNNNN.

Unlike the other instructions in this chapter, these two opcodes are officially documented. The clock/refresh cycles for the mirrored instructions are then 20/2 instead of 16/1 as for the native 8080 instructions.

Mirrored BIT N,(ii+d) Instructions

Unlike as for RES and SET, the BIT instruction does not support a third operand, ie. DD or FD prefixes cannot be used on a BIT N,r instruction in order to produce a BIT r,N,(ii+d) instruction. When attempting this, the 'r' operand is ignored, and the resulting instruction is identical to BIT N,(ii+d).

Except that, not tested yet, maybe undocumented flags are then read from 'r' instead of from ii+d(?).

#### Non-Functional Opcodes

The following opcodes behave much like the NOP instruction.

ED00-3F, ED77, ED7F, ED80-9F, EDA4-A7, EDAC-AF, EDB4-B7, EDBC-BF, EDC0-FF.

The execution time for these opcodes is 8 clock cycles, 2 refresh cycles.

Note that some of these opcodes appear to be used for additional instructions by the R800 CPU in newer turbo R (MSX) models.

#### Ignored DD and FD Prefixes

In some cases, DD-prefixes (IX) and FD-prefixes (IY) may be ignored by the CPU.

This happens when using one (or more) of the above prefixes prior to instructions that already contain an ED, DD, or FD prefix, or prior to any instructions that do not support IX, IY, IXL, IXH, IYL, IYH operands. In such cases, 4 clock cycles and 1 refresh cycle are counted for each ignored prefix byte.
