---
title: "Register sizes in ca65"
source_url: "https://snes.nesdev.org/wiki/Register_sizes_in_ca65"
pageid: 79
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

When the 65c816's registers switch between being 8-bit and 16-bit, that affects how the processor expects instructions with immediate operands to be stored. For example, when the 65c816 encounters opcode `A9` (which is `LDA #value`) the current size of the accumulator determines if it will try to read one or two bytes after the opcode. This creates a problem for assemblers - they have to know whether to encode `LDA #1` as `A9 01` or `A9 01 00`. If they get it wrong, the processor will misinterpret the instruction, then probably become misaligned with the code and misinterpret following instructions as well.

Because of this, some assemblers require the programmer to explicitly write the size per-instruction. ca65 works differently - the programmer specifies that a section of code uses a particular size. This is done with the `.a8`, `.a16`, `.i8`, and `.i16` directives. These tell ca65 that for all instructions after that point, the accumulator (`.a8`/`.a16`) or index registers (`.i8`/`.i16`) are the size given in the directive name.

```
.a8       ; Assembles to...
lda #1    ; A9 01
cmp #1    ; C9 01

.a16
lda #1    ; A9 01 00
cmp #1    ; C9 01 00

.i8
ldx #1    ; A2 01
cpx #1    ; E0 01

.i16
ldx #1    ; A2 01 00
cpx #1    ; E0 01 00
```

It's a good idea to specify the register sizes at the start of a routine, which adds documentation but also means the routine doesn't rely on the registers being sized correctly by previous code.

ca65 has a "smart" mode (enabled with `.smart`) which causes `REP` and `SEP` instructions to additionally act like the size changing directives. (Smart mode also will attempt to convert `JSR`/`RTS` to `JSL`/`RTL` for "far" procedures.)

## Detecting register sizes in macros

Macros can check the current register size with `.asize` and `.isize` - which will return 8 or a 16. This lets the programmer avoid having separate 8-bit and 16-bit versions of macros.

```
; Calculate 0-Accumulator
.macro neg
  .if .asize = 8
    eor #$ff
  .else
    eor #$ffff
  .endif
  ina
.endmacro
```

## References

- [ca65 .a16](https://cc65.github.io/doc/ca65.html#.A16) - documentation
- [ca65 .smart](https://cc65.github.io/doc/ca65.html#.SMART) - documenation
