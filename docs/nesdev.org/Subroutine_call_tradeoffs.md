---
title: "Subroutine call tradeoffs"
source_url: "https://snes.nesdev.org/wiki/Subroutine_call_tradeoffs"
pageid: 203
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

There is no common standard calling convention for 65c816 assembly. There are no best-fit value for the register size flags, `D` direct page register or `DB` data bank register and calling a subroutine with the incorrect flag/`D`/`DB`/`PB` value will crash the program.

## Register sizes

The length of an immediate addressing mode instruction and the number of bytes read/written by an instruction varies depending on the `m` and `x` status flags. Calling a subroutine with the wrong register size value will either desynchronize the instruction stream (typically leading to a crash), read 16-bit memory as 8-bit memory and/or cause an out-of-bounds read/write (clobbering memory).

A subroutine should return with the same register size flags it was called with to minimise the risk of crashes.

Highly optimised assembly code might require a subroutine exit with different register size flags. Subroutines that do this should not be called outside their assembly file (minimising Spaghetti code) and the register size changes should be clearly documented. Adding a prefix or suffix to the subroutine name (ie, `_AccelerateLeft_A16__returnsA8`) is a great way to document this.

Most 65816 assemblers have commands for setting the accumulator and index sizes. It is highly recommended that all subroutines are preceded with these commands to tell the assembler (and any humans reading the code) the register sizes used by the subroutines.

| ca65 | 64tass | wla-dx |
| --- | --- | --- |
| ``` .a8 .i16 .proc Subroutine     rts .endproc ``` | ``` .as .xl Subroutine .proc     rts .endproc ``` | ``` .accu 8 .index 16 Subroutine:     rts ``` |

### Register size trade-offs

8 bit Memory, 16 bit Index:

- Advantages:
  - Can write 8 and 16 bit values without changing register size
  - Can index 64KiB of data with `X` and `Y`
  - A 24 bit address can be efficiently passed to a subroutine by storing the bank-byte in `A` and the low-word in `X` (common) or `Y`.
- Disadvantages:
  - Slower 16+ bit math as it requires a switch to 16-bit memory or processing the math one byte at a time.
  - Reading an 8-bit index from memory is done via the accumulator (followed by a `tax`/`tay` with `Ah` clear) or switching to an 8-bit index (which clobbers the high byte of `X` and `Y`).

16 bit Memory, 16 bit Index:

- Advantages:
  - Faster 16 bit math.
  - Can index 64KiB of data with `X` and `Y`.
- Disadvantages:
  - 8-bit math requires a register size change.
  - 8-bit memory accesses requires a switch to an 8-bit memory and/or index.
  - Reading an 8 bit index from memory requires masking (`lda data ; and #0xff ; tax`) or switching to an 8-bit index (which clobbers the high byte of `X` and `Y`).

8 bit Memory, 8 bit Index:

- Advantages:
  - Mostly backwards compatible with 65c02 code (ignoring the `D` and `DB` register values).
  - `addr, X` and `addr, Y` addressing modes are 1 internal cycle faster if a page boundary is not crossed.
- Disadvantages:
  - Cannot efficiently pass 16-bit parameters using registers.
  - Cannot use an index register as a 16-bit pointer.
  - Slower 16 bit memory accesses.
  - Slower 16+ bit math.

16 bit Memory, 8 bit Index:

- Rarely used but useful for 8 and 16 bit PPU writes in VBlank code.
- Advantages:
  - Can write 8 and 16 bit values without changing register size
  - `addr, X` and `addr, Y` addressing modes are 1 internal cycle faster if a page boundary is not crossed.
- Disadvantages:
  - 8-bit math requires a register change
  - Cannot use an index register as a 16-bit pointer

### Register size independent subroutines

A subroutine can be made register size independent by pushing the status flags to the stack and setting the `m` and x flags with `rep` and/or `sep` instructions. At the end of the subroutine the status flags are popped off the stash before the return instruction.

```
; memory size unknown
; index size unknown
.proc SubroutineName
    ; push status flags, including register sizes, to the stack
    php

    rep  #$30
    sep  #$10
    ; 8 bit A/memory, 16 bit index

    ; [code]

    ; restore status flags, including register sizes and return
    plp
    rts
.endproc
```

Be aware, this technique will complicate any subroutine that returns a boolean value using a status flag (carry, zero, negative). To return a boolean value via the carry flag two return paths are required, one for return true and one for return false.

```
; memory size unknown
; index size unknown
; OUT: boolean in carry
.proc SubroutinesThatReturnsCarry
    ; push status flags, including register sizes, to the stack
    php

    rep  #$30
    sep  #$10
    ; 8 bit A/memory, 16 bit index

    ; [code that conditionally branches to ReturnTrue]

    ReturnFalse:
        plp
        clc
        rts

    ReturnTrue:
        plp
        sec
        rts
.endproc
```

## Subroutine arguments

There are multiple places to put the subroutine arguments.

### Registers

See also: [[Struct register tradeoffs]]

The simplest and fastest way to pass arguments to a subroutine is via the CPU registers. However, the 65c816's limited register pool and single Accumulator register can make other argument locations more appealing.

### Temporary variables

When a subroutine has more arguments then registers, the extra arguments can be stored in temporary variables. The caller will store the argument in the temporary variable and the subroutine will read from that temporary variable when required.

Advantages:

- Argument is stored in memory, allowing register use without clobbering the arguments
- Zeropage temporary variables are faster than absolute addressing global variables or stack (when Direct Page Register is 0)
- Pointer arguments stored in direct-page temporary variables can be immediately accessed via the indirect addressing modes

Disadvantages:

- Slower then passing via registers
- Uses more code than passing via registers
- Temporary variables are short lived and can be clobbered by subroutine calls
- Recursion requires dropping arguments or pushing arguments to stack

Subroutines whose inputs are the result of multiple calculations might prefer storing arguments on memory or the stack. For example, a DrawMetaSprite subroutine could require a screen position argument but the actor uses map position variables. By storing the screen position to memory in the caller code, the caller can avoid temporaries and or stack in the calculations.

| Only registers arguments | Temporary register arguments |
| --- | --- |
| ``` ; IN: X = actor .i16 .a16 .proc DrawActor     lda     actor_xpos,x     sec     sbc     camera_xpos     // Use stack to not clobber X     pha      lda     actor_ypos,x     sec     sbc     camera_ypos     tay      lda     actor_msFrame,x     plx      jmp     DrawMetaSprite .endproc  [...]  ; IN: A = MetaSprite frame to draw ; IN: X = screen x position ; IN: Y = screen y position .i16 .a16 .proc DrawMetaSprite     // Save position so it can be read in a loop     stx     tmp0     sty     tmp1      // [...] ``` | ``` ; IN: X = actor .i16 .a16 .proc DrawActor     lda     actor_xpos,x     sec     sbc     camera_xpos     sta     tmp0      lda     actor_ypos,x     sec     sbc     camera_ypos     sta     tmp1      lda     actor_msFrame,x      jmp     DrawMetaSprite .endproc  [...]    ; IN: A = MetaSprite frame to draw ; IN: tmp0 = screen x position ; IN: tmp1 = screen y position .i16 .a16 .proc DrawMetaSprite     // Position in temporary variables      // [...] ``` |

### Global variables

Advantages:

- Assigning a global variable for a subroutine argument makes them long lived and can survive subroutine calls
- If the subroutine does not modify the arguments it could be reused in other code. For example, a global `currentActor` variable could be reused across multiple actor subroutines.

Disadvantages:

- Global variables outside direct-page are slower and uses more code then direct-page temporary variables
- Increased risk of [spaghetti code](https://en.wikipedia.org/wiki/Spaghetti_code "wikipedia:Spaghetti code") when a global variable is used as an argument for multiple subroutines
- Incompatible with multithreading

### Stack

The 65c816's stack relative addressing mode makes accessing subroutine arguments on the stack more viable than the 6502.

Advantages:

- Stack variables are long lived and will not be clobbered in subroutine calls
- Subroutines with stack arguments can be recursive

Disadvantages:

- Stack arguments can only be accessed by the Accumulator register
- Pushing and popping the stack will change the stack relative offset of the argument. The changes to the stack offset will not be tracked by the assembler and will will need to manually tracked by the programmer
- The subroutine arguments must be popped off the stack by the caller
  - The 65c816 lacks an instruction that adds a constant to the Stack Pointer
  - The arguments could be popped using `pla`, `plx` or `ply` instructions but it clobbers a register
  - The stack pointer could be adjusted using `tsc ; clc ; adc #N ; tcs` with a 16-bit Accumulator
- Increased stack usage, reducing the amount available variables in the first 8 KiB of RAM
- Slower than direct-page temporary variables

```
; 2 arguments on the stack:
;  * u8 arg1
;  * u8 arg2
.a8
.i16
.proc SubroutineWithStackArguments
    // Load arg1 to A
    lda 4,s

    // Load arg2 to A
    lda 3,s

    // Push a value to the stack.
    // The stack offsets have changed
    pha
        // Load arg1 to A
        lda 5,s

        // Load arg2 to A
        lda 4,s
    pla
    rts
.endproc
```

```
.a8
.i16
    // Push arg1 to the stack
    lda     #1
    pha

    // Push arg2 to the stack
    lda     #2
    pha

    jsr     SubroutineWithStackArguments

    // Pop arguments off the stack (clobbers X)
    plx
```

The `pea` instruction can be used to push 16-bit constant values to the stack, saving code space.

```
.a8
.i16
    // Push $01 as arg1 and $02 as arg2
    pea     $0201
    jsr     SubroutineWithStackArguments

    // Pop arguments off the stack (clobbers X)
    plx
```

## Optimisations

### Tail call optimisation

A [tail call](https://en.wikipedia.org/wiki/tail_call "wikipedia:tail call") is when a subroutine's final action is a subroutine call. A `jsr` `rts` chain can be optimised into a single `jmp` instruction.

A `jmp` tail call can only be used if the caller and callee both use the same return instruction. A caller that exits with `rts` cannot jump into a subroutine that returns with `rtl`/`rti` (and vice versa).

If the callee is close to the tail call, branch instructions can be used instead.

This optimisation also applies to `jsr` `rts` chains in the middle of a subroutine. For example:

```
; IN: Y = door location
.a8
.i16
.proc PlayerCollidesWithDoor
    lda     nKeys
    beq     +
        dec     nKeys

        ; Y = door location
        lda     #Sfx::DOOR_OPENED_WITH_KEY
        jsr     OpenDoor
        rts
    +
    lda     #Sfx::DOOR_LOCKED
    jsr     PlaySoundEffect
    rts
.endproc
```

Can be optimised to:

```
; IN: Y = door location
.a8
.i16
.proc PlayerCollidesWithDoor
    lda     nKeys
    beq     +
        dec     nKeys

        ; Y = door location
        lda     #Sfx::DOOR_OPENED_WITH_KEY
        jmp     OpenDoor
    +
    lda     #Sfx::DOOR_LOCKED
    jmp     PlaySoundEffect
.endproc
```

The Mesen debugger cannot cannot track `jmp` or `jml` tail calls.

- The Debugger's call stack table will not be updated after the tail call. It will display the callee's name/address instead.
- The Profiler will not increment the call count of a subroutine called with `jmp` or `jml`.
- The Profiler will not track a `jmp`/`jml` called subroutine's exclusive execution time. The execution time will be assigned to the callee.

### Fallthrough tail call

A fallthrough tail call is a subroutine that does not end with a jump or return instruction. Instead the CPU will advance past the subroutine and fallthrough into the next subroutine.

This optimisation can only be applied once per callee subroutine. Care should be taken to ensure the most common tail call is converted to a fallthrough tail call.

Fallthrough tail calls are fragile. If the caller or callee are moved the code will compile without error and derail and/or crash when the caller exits. Fallthroughs should be clearly documented in the code. If the assembler supports forward references in assert statements, the callee should end in an static assert to induce a compile error if the caller or callee are moved.

The Mesen debugger cannot track fallthroughs.

```
; IN: X = actor
.a16
.i16
.proc DrawActor
    lda     actor_xpos,x
    sec
    sbc     camera_xpos
    sta     tmp0

    lda     actor_ypos,x
    sec
    sbc     camera_ypos
    sta     tmp1

    lda     actor_msFrame,x

    ; fallthrough into DrawMetaSprite
    .assert * = DrawMetaSprite, lderror
.endproc


; IN: A = MetaSprite frame to draw
; IN: tmp0 = screen x position
; IN: tmp1 = screen y position
.i16
.a16
.proc DrawMetaSprite
    [...]
```
