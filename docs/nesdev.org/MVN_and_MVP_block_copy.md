---
title: "MVN and MVP block copy"
source_url: "https://snes.nesdev.org/wiki/MVN_and_MVP_block_copy"
pageid: 30
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The 65C816 has two instructions, **MVN** and **MVP** intended for efficient block copy from one area of CPU memory to another.

Their mnemonics stand for "Move Memory Negative" and "Move Memory Positive".
The negative version copies bytes in increasing order, and the positive version uses decreasing order.

Inputs to the instructions:

- Register A is the 16-bit count of bytes to be copied, minus 1.
- Register X is the 16-bit source data address.
- Register Y is the 16-bit destination data address.
- Two 1-byte immediate operands specify the bank address.

Result:

- The specified data will have been copied.
- Register A will be $FFFF after counting down.
- Register X/Y will have incremented (MVN) or decremented (MVP) by the count of bytes.
- Register DB is set to the destination bank operand.
- Flags are unaffected.

We can consider this a branching instruction that does the following each time it executes:

1. Copy byte at source bank X to destination bank Y. (DB is overwritten.)
2. Increment X and Y (MVN), or decrement (MVP).
3. Decrement A.
4. If A is not $FFFF, branch to self.

Each byte takes 7 CPU cycles of the 65C816, though the SNES specific [[Timing]] is a little more complicated.

Bank boundaries will not be crossed by these instructions. If X or Y over/underflows the address will simply wrap to the other end of the same bank.

## Instruction

There is only one machine instruction opcode for each of MVN and MVP.

Though standard 65C816 assembly syntax places the source operand on the left, in the machine code the destination bank operand byte comes first.

```
Syntax            Opcode  Operand 1  Operand 2
----------------------------------------------
MVN src, dest     $54     dest       src
MVP src, dest     $44     dest       src
```

## MVN example

```
REP #$30 ; use 16-bit registers
.a16
.i16
LDX #.loword(source)
LDY #.loword(dest)
LDA #(size-1)
MVN #^source, #^dest
; DB = ^dest
; A = $FFFF
; X = source+size
; Y = dest+size
```

## MVP example

```
REP #$30
.a16
.i16
LDX #.loword(source+size-1) ; MVP starts with the last byte to copy
LDY #.loword(dest+size-1)
LDA #(size-1)
MVP #^source, #^dest
; DB = ^dest
; A = $FFFF
; X = source-1
; Y = dest-1
```

## Overlap

For copies between non-overlapping regions, choice of MVN or MVP doesn't normally matter,
but when they do, the following operations will ensure the source data is copied correctly to the destination:

- MVP if dest > source
- MVN if dest < source

Thus, MVP is for moving data "forward" in memory, and MVN is for moving it "backward".

In the forward case, since MVP moves the last byte first, it ensures that no part of source will be overwritten by the destination copy before it is read.

### Pattern Fill

Overlap and direction of copy can also be exploited to fill memory with repeating patterns.

This is usually most useful with MVN, which can conveniently repeat bytes from the beginning of an array across the rest of it:

```
LDX #.loword(source)
LDY #.loword(source+4) ; copy each byte 4 bytes forward
LDA #(size-5) ; array size -4 bytes being repeated, and -1 for MVN count
MVN #^source,#^(source+4)
; the first 4 bytes of source are now copied across the entire array
```

MVP could be used for a similar pattern fill starting from the end of an array instead.

## Assembler syntax

MVN and MVP are unique instructions because they have two unusual "immediate" operands. There is unfortunately no consistent standard assembler syntax for them.

Because they normally will use the high byte of a 3-byte address, ca65 provided two alternatives starting with V2.18[[1]](#cite_note-1):

```
MVP #^source, #^dest ; use # to specify the byte directly
MVP source, dest     ; use a far address directly to have it automatically take the high byte
```

Using # allows you to directly state the byte of operand which should be used to designate the byte.

Without # ca65 will automatically treat the operand as a 24-bit far address, and take its 3rd byte.
This looks less verbose, but because the lower 2 bytes are automatically discarded without warning, accidental usage has a tendency to select bank $00 by mistake:

```
MVN #$35,#$36      ; copies from bank $35 -> $36.
MVN $35,$36        ; copies from bank $00 -> $00, not $35 -> $36.
MVN ^source,^dest  ; copies from $00 -> $00 rather than the banks of source/dest.
```

Other assemblers usually do one of these two functions, sometimes with # and sometimes without.

Alternatively, assembler syntax can be bypassed by using the instruction opcode bytes directly. Note that the destination operand comes first:

```
.byte $54, $36, $35 ; MVN #$35, #$36
.byte $44, $36, $35 ; MVP #$35, #$36
```

## See Also

- [[65c816 for 6502 developers]]

## References

- [65C816 Opcodes](http://www.6502.org/tutorials/65c816opcodes.html#6.6) - article at 6502.org

1. [↑](#cite_ref-1) [ca65 V2.18 issue](https://github.com/cc65/cc65/issues/925#issuecomment-518318216) - MVN/MVP syntax note and assembler comparison
