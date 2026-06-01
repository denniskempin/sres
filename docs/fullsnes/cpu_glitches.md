# CPU Glitches

#### Dummy Write Cycles in Read-Modify-Opcodes

The original 6502 CPU includes a dummy-write cycle in all Read-Modify opcodes (ie. INC, DEC, Shift/Rotate). The 65C816 does NOT reproduce this effect (neither when E=0, nor when E=1).

#### Dummy Read Cycles at Page-Wraps

```text
  [Below applies to original 6502 CPUs]
  [Unknown if 65C816 has similar effects on page- and/or bank-wraps?]
```

Dummy reads occur when reads from [nnnn+X] or [nnnn+Y] or [WORD[nn]+Y] are crossing page boundaries, this applies only to raw read-opcodes, not for write or read-and-modify opcodes (ie. only for opcodes that include an extra clock cycle on page boundaries, such as LDA, CMP, etc.) For above reads, the CPU adds the index register to the lower 8bits of the 16bit memory address, and does then read from the resulting memory address, if the addition caused a carry-out, then an extra clock cycle is used to increment the upper 8bits of the address, and to read from the correct memory location.

For example, a read from [1280h+X] with X=C0h produces a dummy read from [1240h], followed by the actual read from [1340h].

Dummy reads cause no problems with normal ROM or RAM, but may cause problems with read-sensitive I/O ports (eg. IRQ flags that are automatically cleared after reading, or data-registers that are automatically incrementing associated memory pointers, etc.)
