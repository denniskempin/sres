---
title: "Using X as a pointer"
source_url: "https://snes.nesdev.org/wiki/Using_X_as_a_pointer"
pageid: 23
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Because the 65c816's index registers can be 16-bit, they can hold a 16-bit address. For programs that already use 16-bit index registers for other reasons (such as accessing level data), this can sometimes create an optimization opportunity, allowing for the use of faster addressing modes and smaller code.

Addressing mode comparison

| Instruction | Bytes | Cycles |
| --- | --- | --- |
| `LDA directpage,x` | 2 | - 4 - + 1 (16-bit A) - + 1 (low byte of direct page base not zero) |
| `LDA absolute,x` | 3 | - 4 - + 1 (16-bit A) - + 1 (page crossed, or 16-bit X) |
| `LDA (directpage)` | 2 | - 5 - + 1 (16-bit A) - + 1 (low byte of direct page base not zero) |
| `LDA (directpage),y` | 2 | - 5 - + 1 (16-bit A) - + 1 (low byte of direct page base not zero) - + 1 (page crossed, or 16-bit X) |

Additional things to keep in mind:

- `BIT`, `ASL`, `LSR`, `ROL`, `ROR`, `INC`, `DEC`, `LDY`, and `STZ` can use `directpage,x` and `absolute,x` but not any indirect addressing modes.
- `STY directpage,x` exists, but not `STY absolute,x`
- This optimization is mutually exclusive with the optimization where `DEX \ BPL` is used at the end of a loop to avoid a compare.
- ROM can be accessed at 3.58 MHz, whereas RAM in the SNES is always 2.68 MHz; this means that cycles that access RAM are slower (8 master clocks) than ones that access ROM (6 master clocks).

## Data in the first 8KB of RAM

This technique works best for data that's located within the first 8KB of RAM. Direct page instructions are limited to the first 64KB of address space which [[Memory map|contains a mirror of the first 8KB of RAM]].

If the direct page is at $0000 (or another multiple of $100) and index registers are 16-bit, every instruction that can be switched from `absolute,x` to `directpage,x` is a byte and a cycle saved.

## Data somewhere within a bank

For memory outside of the first 8KB of RAM, `directpage,x` is not available, so this technique is much less likely to help if the code already uses `absolute,x`.

If the code is using an indirect addressing mode, however, the additional instructions allowed by `absolute,x` may justify switching to it in some situations.

## Array of structures example (ca65)

This example demonstrates how X can be used as a pointer within an [array of structures](https://snes.nesdev.org/w/index.php?title=Array_of_structures&action=edit&redlink=1 "Array of structures (page does not exist)"). When X points to the first byte in a structure, `directpage,x` can access a field in that struct.

```
.struct Actor
  Type .word
  PositionX .word
  PositionY .word
  VelocityX .word
  VelocityY .word
.endstruct

.segment "BSS"
  ActorCount = 10
  ActorStart: .res ActorCount * .sizeof(Actor)
  ActorEnd:

.segment "CODE"

.a16
.i16
; For each actor slot that's in use (type isn't zero), apply the velociy
.proc ApplyVelocityToActiveActors
  ldx #ActorStart
Loop:
  ; Do not process actors with a zero type
  lda Actor::Type,x
  beq Skip

  lda Actor::PositionX,x
  clc
  adc Actor::VelocityX,x
  sta Actor::PositionX,x

  lda Actor::PositionY,x
  clc
  adc Actor::VelocityY,x
  sta Actor::PositionY,x

Skip:
  ; Move onto the next entry
  txa
  clc
  adc #.sizeof(Actor)
  tax

  ; Is it past the end yet?
  cpx #ActorEnd
  bcc Loop
  rts
.endproc
```
