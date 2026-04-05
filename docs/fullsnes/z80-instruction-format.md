# Z80 Instruction Format

#### Commands and Parameters

Each instruction consists of a command, and optionally one or two parameters.

Usually the leftmost parameter is modified by the operation when two parameters are specified.

#### Parameter Placeholders

The following placeholders are used in the following chapters:

```text
  r      8bit  register A,B,C,D,E,H,L
  rr     16bit register BC, DE, HL/IX/IY, AF/SP   (as described)
  i      8bit  register A,B,C,D,E,IXH/IYH,IXL/IYL
  ii     16bit register IX,IY
  n      8bit  immediate 00-FFh                   (unless described else)
  nn     16bit immediate 0000-FFFFh
  d      8bit  signed offset -128..+127
  f      flag  condition nz,z,nc,c AND/OR po,pe,p,m  (as described)
  (..)   16bit pointer to byte/word in memory
```

#### Opcode Bytes

Each command (including parameters) consists of 1-4 bytes. The respective bytes are described in the following chapters. In some cases the register number or other parameters are encoded into some bits of the opcode, in that case the opcode is specified as "xx". Opcode prefix bytes "DD" (IX) and "FD" (IY) are abbreviated as "pD".

#### Clock Cycles

The clock cycle values in the following chapters specify the execution time of the instruction. For example, an 8-cycle instruction would take 2 microseconds on a CPU which is operated at 4MHz (8/4 ms). For conditional instructions two values are specified, for example, 17;10 means 17 cycles if condition true, and 10 cycles if false.

Note that in case that WAIT signals are sent to the CPU by the hardware then the execution may take longer.

#### Affected Flags

The instruction tables below are including a six character wide field for the six flags: Sign, Zero, Halfcarry, Parity/Overflow, N-Flag, and Carry (in that order). The meaning of the separate characters is:

```text
  s    Indicates Signed result
  z    Indicates Zero
  h    Indicates Halfcarry
  o    Indicates Overflow
  p    Indicates Parity
  c    Indicates Carry
  -    Flag is not affected
  0    Flag is cleared
  1    Flag is set
  x    Flag is destroyed (unspecified)
  i    State of IFF2
  e    Indicates BC<>0 for LDX(R) and CPX(R), or B=0 for INX(R) and OUTX(R)
```
