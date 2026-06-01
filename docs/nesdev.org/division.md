---
title: "Division"
source_url: "https://snes.nesdev.org/wiki/Division"
pageid: 36
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The 65c816 does not include a division instruction. However, the SNES includes division hardware for games to use, and games can access it with registers. It's capable of dividing a 16-bit number by an 8-bit number, and it produces a 16-bit result and a 16-bit remainder. All inputs and outputs are unsigned.

See also:

- [[Multiplication]]

## Division registers

```
  WRDIVH      WRDIVL
  $4205       $4204
7  bit  0   7  bit  0
---- ----   ---- ----
HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||
++++-++++---++++-++++- Dividend (16-bit unsigned)

              WRDIVB  
              $4206   
            7  bit  0 
            ---- ---- 
            NNNN NNNN 
            |||| |||| 
            ++++-++++- Divisor (8-bit unsigned)

  RDDIVH      RDDIVL
  $4215       $4214
7  bit  0   7  bit  0
---- ----   ---- ----
HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||
++++-++++---++++-++++- Division result (16-bit unsigned)

  RDMPYH      RDMPYL
  $4217       $4216
7  bit  0   7  bit  0
---- ----   ---- ----
HHHH HHHH   LLLL LLLL
|||| ||||   |||| ||||
++++-++++---++++-++++- Division remainder (16-bit unsigned)
```

Writing to `WRDIVB` starts the division process. The division process takes 16 CPU clock cycles before it's finished (regardless of if those clock cycles are 2.68MHz or 3.58MHz). Reading the result registers before division has completed will return intermediate results.

The time spent reading the instruction that reads the result counts toward the wait. For instance, if the registers are read with `LDA absolute`, that instruction will spend 3 cycles before reading the result, and `LDA long` will spend 4 cycles before reading the result. This means the program only effectively needs to wait 12 or 13 cycles. A program can simply wait the required time with `NOP` (2 cycles each instruction) but ideally that time is spent doing something else the program needs to do, like preparing registers or flags for something it intends to do with the division result.

Division by zero will produce a result of $FFFF, and a remainder that is equal to the dividend.

Important note: The unsigned [[Multiplication]] registers will replace the value at `RDDIVL` with `WRMPYB`, and `RDDIVH` with zero. Combined with the fact that the remainder and multiplication result registers are the same, that means that the SNES can only hold one unsigned multiplication result or one unsigned division result at any given time.

A divison should not be started by writing `WRDIVB` while a multiplication or division is already in progress. This will not restart the operation, but instead corrupt the result.[[1]](#cite_note-1)

## References

- [16-bit multiplication and division](https://wiki.superfamicom.org/16-bit-multiplication-and-division) - Example of using the divider to do divisions with bigger numbers
- [multest/div16](https://github.com/bbbradsmith/SNES_stuff/blob/main/multest/test_div16.s) - Reference implementation of 16/16 bit divide, with hardware test ROM.

1. [↑](#cite_ref-1) [Forum thread](https://forums.nesdev.org/viewtopic.php?p=282493#p282493) - Writing $4203 twice too fast gives erroneous result (not emulated)
