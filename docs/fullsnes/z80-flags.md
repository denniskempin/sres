# Z80 Flags

#### Flag Summary

The Flags are located in the lower eight bits of the AF register pair.

```text
  Bit Name  Set  Clr  Expl.
  0   C     C    NC   Carry Flag
  1   N     -    -    Add/Sub-Flag (BCD)
  2   P/V   PE   PO   Parity/Overflow-Flag
  3   -     -    -    Undocumented
  4   H     -    -    Half-Carry Flag (BCD)
  5   -     -    -    Undocumented
  6   Z     Z    NZ   Zero-Flag
  7   S     M    P    Sign-Flag
```

#### Carry Flag (C)

This flag signalizes if the result of an arithmetic operation exceeded the maximum range of 8 or 16 bits, ie. the flag is set if the result was less than Zero, or greater than 255 (8bit) or 65535 (16bit). After rotate/shift operations the bit that has been 'shifted out' is stored in the carry flag.

#### Zero Flag (Z)

Signalizes if the result of an operation has been zero (Z) or not zero (NZ).

Note that the flag is set (1) if the result was zero (0).

#### Sign Flag (S)

Signalizes if the result of an operation is negative (M) or positive (P), the sign flag is just a copy of the most significant bit of the result.

#### Parity/Overflow Flag (P/V)

This flag is used as Parity Flag, or as Overflow Flag, or for other purposes, depending on the instruction.

Parity: Bit7 XOR Bit6 XOR Bit5 ... XOR Bit0 XOR 1.

8bit Overflow: Indicates if the result was greater/less than +127/-128.

HL Overflow: Indicates if the result was greater/less than +32767/-32768.

After LD A,I or LD A,R: Contains current state of IFF2.

After LDI,LDD,CPI,CPD,CPIR,CPDR: Set if BC<>0 at end of operation.

BCD Flags (H,N)

These bits are solely supposed to be used by the DAA instruction. The N flag signalizes if the previous operation has be an addition or substraction. The H flag indicates if the lower 4 bits exceeded the range from 0-0Fh. (For 16bit instructions: H indicates if the lower 12 bits exceeded the range from 0-0FFFh.) After adding/subtracting two 8bit BCD values (0-99h) the DAA instruction can be used to convert the hexadecimal result in the A register (0-FFh) back to BCD format (0-99h). Note that DAA also requires the carry flag to be set correctly, and thus should not be used after INC A or DEC A.

Undocumented Flags (Bit 3,5)

The content of these undocumented bits is filled by garbage by all instructions that affect one or more of the normal flags (for more info read the chapter Garbage in Flag Register), the only way to read out these flags would be to copy the flags register onto the stack by using the PUSH AF instruction.

However, the existence of these bits makes the AF register a full 16bit register, so that for example the code sequence PUSH DE, POP AF, PUSH AF, POP HL would set HL=DE with all 16bits intact.
