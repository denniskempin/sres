# Z80 Register Set

#### Register Summary

```text
  16bit Hi   Lo   Name/Function
  ---------------------------------------
  AF    A    -    Accumulator & Flags
  BC    B    C    BC
  DE    D    E    DE
  HL    H    L    HL
  AF'   -    -    Second AF
  BC'   -    -    Second BC
  DE'   -    -    Second DE
  HL'   -    -    Second HL
  IX    IXH  IXL  Index register 1
  IY    IYH  IYL  Index register 2
  SP    -    -    Stack Pointer
  PC    -    -    Program Counter/Pointer
  -     I    R    Interrupt & Refresh
```

#### Normal 8bit and 16bit Registers

The Accumulator (A) is the allround register for 8bit operations. Registers B, C, D, E, H, L are normal 8bit registers, which can be also accessed as 16bit register pairs BC, DE, HL.

The HL register pair is used as allround register for 16bit operations. B and BC are sometimes used as counters. DE is used as DEstination pointer in block transfer commands.

#### Second Register Set

The Z80 includes a second register set (AF',BC',DE',HL') these registers cannot be accessed directly, but can be exchanged with the normal registers by using the EX AF,AF and EXX instructions.

#### Refresh Register

The lower 7 bits of the Refresh Register (R) are incremented with every instruction. Instructions with at least one prefix-byte (CB,DD,ED,FD, or DDCB,FDCB) will increment the register twice. Bit 7 can be used by programmer to store data. Permanent writing to this register will suppress memory refresh signals, causing Dynamic RAM to lose data.

#### Interrupt Register

The Interrupt Register (I) is used in interrupt mode 2 only (see command "im 2"). In other modes it can be used as simple 8bit data register.

#### IX and IY Registers

IX and IY are able to manage almost all the things that HL is able to do. When used as memory pointers they are additionally including a signed index byte (IX+d). The disadvantage is that the opcodes occupy more memory bytes, and that they are less fast than HL-instructions.

#### Undocumented 8bit Registers

IXH, IXL, IYH, IYL are undocumented 8bit registers which can be used to access high and low bytes of the IX and IY registers (much like H and L for HL). Even though these registers do not officially exist, they seem to be available in all Z80 CPUs, and are quite commonly used by various software.
