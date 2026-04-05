---
title: "Signature byte"
source_url: "https://snes.nesdev.org/wiki/Signature_byte"
pageid: 146
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

In 65x parlance, a **signature byte** is the byte that follows certain instructions, including:

- BRK
- COP
- WDM

Each of these instructions will normally advance the PC by two bytes, even though the hardware does not make any direct use of the second "operand" byte.

For this reason, these instructions have been treated both as one-byte[[1]](#cite_note-1) and two-byte[[2]](#cite_note-2) instructions in various reference documents and assemblers.

## BRK and COP

Both of these instructions generate a software interrupt that will be handled by a routine designated in the [[CPU vectors|CPU vector table]].

- COP has a vector at $FFE4.
- BRK has a vector at $FFE6.

Stack contents for handler:

```
 $00, S - (empty, current stack pointer)
 $01, S - P status byte
 $02, S - return address low (BRK/COP PC + 2)
 $03, S - return address high
 $04, S - return bank K
```

The COP instruction was originally intended for use with a co-processor, for which the signature byte could indicate a command to send to the co-processor. However, there is no hardware to support this usage on the SNES, and it is simply a second software interrupt, equivalent to BRK.

### Without Signature

If the signature byte is not needed, a BRK or COP handler may wish to decrement the return address on the stack before RTI, returning as if it were a one-byte instruction.

### With Signature

A software response to BRK or COP may use the return address on the stack to deduce the location of the operand byte and inspect it.

This might be used for error codes, or as a compact system call dispatch.

## WDM

The WDM instruction was reserved for future use, but was ultimately left unused. It is simply a 2-byte alternative to NOP.

Mesen's debugger provides a break-on-WDM instruction which can make it convenient as an emulator-only breakpoint.

## Assemblers

There is no standard for how assemblers treat BRK or COP. If BRK emits only 1 byte, a signature byte can be added manually with a data byte following.

- ca65 an optional signature for BRK[[3]](#cite_note-3) and COP[[4]](#cite_note-4), allowing either 1 or 2 bytes. WDM always require the signature byte.
- wla-dx always emits 2 bytes for BRK, COP and WDM. The signature byte defaults to 0 if not given.
- asar always emits 2 bytes for BRK, COP and WDM. The signature byte defaults to 0 if not given.

## Notes

- Though the 65C816 has no unused opcodes, on the 6502 many were left open with unspecified behaviour. This allowed the use of "unofficial" illegal opcodes, including several NOP variants with an unused signature byte. See: [NESDev: CPU unofficial opcodes](https://www.nesdev.org/wiki/CPU_unofficial_opcodes)

## References

1. [↑](#cite_ref-1) Eyes, David, & Lichty, Ron. *Programming the 65816 Including the 6502, 65C02, and 65802* (2015th ed.). Page 436. Prentice Hall Press. New York, New York.
2. [↑](#cite_ref-2) *Western Design Center W65C816S 8/16–bit Microprocessor Datasheet.* Section 7.22 BRK Instruction, page 53. (2018, November 9). Retrieved February 25, 2023, from <https://www.westerndesigncenter.com/wdc/documentation/w65c816s.pdf>
3. [↑](#cite_ref-3) [cc65 github commit d13d068](https://github.com/cc65/cc65/commit/d13d068e71fb7cc08734e2c17e67e83f48d28d77) 2018-08-16 - 65C816-only BRK optional parameter
4. [↑](#cite_ref-4) [cc65 github PR 2010](https://github.com/cc65/cc65/pull/2010) 2023-03-04 - BRK, COP optional parameters, optional immediate, all CPUs
