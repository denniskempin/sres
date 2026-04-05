# ARM Instruction Cycle Times

#### Instruction Cycle Summary

```text
  Instruction      Cycles      Additional
  ---------------------------------------------------------------------
  ALU              1S          +1S+1N if R15 loaded, +1I if SHIFT(Rs)
  MSR,MRS          1S
  LDR              1S+1N+1I    +1S+1N if R15 loaded
  STR              2N
  LDM              nS+1N+1I    +1S+1N if R15 loaded
  STM              (n-1)S+2N
  SWP              1S+2N+1I
  B,BL             2S+1N
  SWI,trap         2S+1N
  MUL              1S+ml
  MLA              1S+(m+1)I
  MULL             1S+(m+1)I
  MLAL             1S+(m+2)I
  CDP              1S+bI
  LDC,STC          (n-1)S+2N+bI
  MCR              1N+bI+1C
  MRC              1S+(b+1)I+1C
  {cond} false     1S
```

Whereas,

```text
  n = number of words transferred
  b = number of cycles spent in coprocessor busy-wait loop
  m = depends on most significant byte(s) of multiplier operand
```

Above 'trap' is meant to be the execution time for exceptions. And '{cond} false' is meant to be the execution time for conditional instructions which haven't been actually executed because the condition has been false.

The separate meaning of the N,S,I,C cycles is:

#### N - Non-sequential cycle

Requests a transfer to/from an address which is NOT related to the address used in the previous cycle. (Called 1st Access in GBA language).

The execution time for 1N is 1 clock cycle (plus non-sequential access waitstates).

#### S - Sequential cycle

Requests a transfer to/from an address which is located directly after the address used in the previous cycle. Ie. for 16bit or 32bit accesses at incrementing addresses, the first access is Non-sequential, the following accesses are sequential. (Called 2nd Access in GBA language).

The execution time for 1S is 1 clock cycle (plus sequential access waitstates).

#### I - Internal Cycle

CPU is just too busy, not even requesting a memory transfer for now.

The execution time for 1I is 1 clock cycle (without any waitstates).

#### C - Coprocessor Cycle

The CPU uses the data bus to communicate with the coprocessor (if any), but no memory transfers are requested.

#### Memory Waitstates

Ideally, memory may be accessed free of waitstates (1N and 1S are then equal to 1 clock cycle each). However, a memory system may generate waitstates for several reasons: The memory may be just too slow. Memory is currently accessed by DMA, eg. sound, video, memory transfers, etc. Or when data is squeezed through a 16bit data bus (in that special case, 32bit access may have more waitstates than 8bit and 16bit accesses). Also, the memory system may separate between S and N cycles (if so, S cycles would be typically faster than N cycles).

#### Memory Waitstates for Different Memory Areas

Different memory areas (eg. ROM and RAM) may have different waitstates. When executing code in one area which accesses data in another area, then the S+N cycles must be split into code and data accesses: 1N is used for data access, plus (n-1)S for LDM/STM, the remaining S+N are code access. If an instruction jumps to a different memory area, then all code cycles for that opcode are having waitstate characteristics of the NEW memory area.
