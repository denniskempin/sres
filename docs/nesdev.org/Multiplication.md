---
title: "Multiplication"
source_url: "https://snes.nesdev.org/wiki/Multiplication"
pageid: 35
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The 65c816 does not have a multiplication instruction, but the SNES includes multiplication hardware that can be accessed with registers. These are faster than implementing general-purpose multiplication in software.

There are two available multipliers:

- 5A22 multiplier
  - 8-bit number × 8-bit number = 16-bit result
  - Uses unsigned numbers: $ff (255) × $ff (255) = $fe01 (65025)
  - There is a delay between writing the parameters and correct results being available
  - Shares result register with [[Division]].
  - Can be used any time

- PPU multiplier
  - 16-bit number × 8-bit number = 24-bit result
  - Uses signed numbers: $ffff (-1) × $ff (-1) = $000001 (1)
  - Results can be read immediately after writing the parameters
  - Can not be used while rendering [[Mode 7]]

See also:

- [[Division]]

## 5A22 multiplier

The multiplier in the 5A22 chip (which also contains the SNES's main processor) is accessed through the following registers:

```
  WRMPYA 
  $4202   
7  bit  0 
---- ---- 
NNNN NNNN 
|||| |||| 
++++-++++- First number to multiply (8-bit, unsigned)

  WRMPYB  
  $4203   
7  bit  0 
---- ---- 
NNNN NNNN 
|||| |||| 
++++-++++- Second number to multiply (8-bit unsigned)

  RDMPYH      RDMPYL
  $4217       $4216
7  bit  0   7  bit  0
---- ----   ---- ----
HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||
++++-++++---++++-++++- Multiplication result (16-bit unsigned)
```

Writing to `WRMPYB` starts the multiplication process. This takes 8 CPU clock cycles before it's finished (regardless of if those clock cycles are 2.68MHz or 3.58MHz) and reading the result early will return an intermediate result.

The time spent reading the instruction that reads `RDMPYL` and/or `RDMPYH` counts toward the wait. For instance, if the registers are read with `LDA absolute`, that instruction will spend 3 cycles before reading the result, and `LDA long` will spend 4 cycles before reading the result. This means the program only effectively needs to wait 4 or 5 cycles.

A program can choose to simply fill the required wait time with `NOP` (2 cycles each). A better option is to spend that time on something useful to the program. The following example spends it on a single instruction that fetches the next number the program intends to multiply:

```
; Accumulator is 8-bit, index registers are 16-bit
; Multiply eight different numbers by WRMPYA and store the results in RAM
lda 0,y
.repeat 8, I
  sta WRMPYB   ; Kick off the multiplier
  lda I+1,y    ; 5 cycles
  ldx RDMPYL   ; 3 cycles before the read
  stx I*2      ; =8 cycles waiting
.endrep
```

Another thing to be aware of is that if the most significant bit written to `WRMPYA` is always known to be zero, the result is valid one cycle earlier, and if the two most significant bits are zero, the result is valid two cycles earlier, and so on.

Also, writing to `MRMPYB` a second time before 8 cycles have elapsed will not correctly restart the process, and the result will be corrupted[[1]](#cite_note-1). For the same reason we should not write `MRMPYB` while a division is in progress.

## "PPU" multiplier

The other multiplier in the SNES reuses hardware meant for Mode 7, so it can only be used in vblank or in background modes 0 to 6. It's accessed through the the following registers, also described on [[PPU registers]]:

```
          M7A
         $211B
15  bit  8   7  bit  0
 ---- ----   ---- ----
 DDDD DDDD   dddd dddd
 |||| ||||   |||| ||||
 ++++-++++---++++-++++- 16-bit multiplication factor (signed)

On write: M7A = (value << 8) | mode7_latch
         mode7_latch = value
This means that $211B must be written to twice, with the lower 8 bits of the number being written before the upper 8 bits.
"STA M7A \ STZ M7A" can be used to just write an 8-bit number here with zero as the upper byte.

  M7B
 $211C
7  bit  0
---- ----
dddd dddd
|||| ||||
++++-++++- 8-bit multiplication factor (signed)

  MPYH        MPYM        MPYL
  $2136       $2135       $2134
7  bit  0   7  bit  0   7  bit  0
---- ----   ---- ----   ---- ----
HHHH HHHH   MMMM MMMM   LLLL LLLL
|||| ||||   |||| ||||   |||| ||||
++++-++++---++++-++++---++++-++++- Multiplication result (signed)
```

The advantages to using this multiplier are that the results are available immediately and it can use a 16-bit number for one of the parameters. It does not have to be started like the other one - writing to `M7A` or `M7B` will immediately and instantly calculate the result.

It's important to note that this multiplier uses signed numbers, so numbers like 128 are out of range for `M7B`, and anything with the most significant bit set will be interpreted to be negative.

Example usage:

```
.a8
   lda s16_l
   sta M7A
   lda s16_h
   sta M7A
   
   lda s8
   sta M7B

   ; MPY now contains s16 * s8
```

CAUTION: The [[PPU registers#M7A|M7A]] register shares mode7\_latch with [[PPU registers#M7HOFS|M7HOFS]] and [[PPU registers#M7VOFS|M7VOFS]] and the mode 7 offset registers use the same address as the BG1 offset registers.

If an interrupt or HDMA transfer writes to a BG1 offset register or Mode7 Matrix in-between the two M7A writes, the internal M7A value will be corrupted and MPY will output the wrong value.

## Multiplication by a constant

The multiplication registers are the fastest option when code needs to multiply two arbitrary numbers together. However, when code can rely on one of the numbers always being the same it can sometimes be faster to use tables or shifts. For instance, multiplication by three can be as simple as:

```
sta Temp
asl
adc Temp ; Assume that carry got cleared by the ASL
```

## References

- [16-bit multiplication and division](https://wiki.superfamicom.org/16-bit-multiplication-and-division) - Example of using the 5a22 multiplier to do multiplications with bigger numbers
- [untech-engine multiplication.inc](https://github.com/undisbeliever/untech-engine/blob/master/src/math/multiplication.inc) - Additional examples of how to multiply bigger numbers.
- [multest/mul16](https://github.com/bbbradsmith/SNES_stuff/blob/main/multest/test_mul16.s) - Reference implementation of 16x16 bit multiply, with hardware test ROM.

1. [↑](#cite_ref-1) [Forum thread](https://forums.nesdev.org/viewtopic.php?p=282493#p282493) - Writing $4203 twice too fast gives erroneous result (not emulated)
