---
title: "65c816 for 6502 developers"
source_url: "https://snes.nesdev.org/wiki/65c816_for_6502_developers"
pageid: 11
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

This guide is meant to introduce the 65c816 to someone who is already familiar with the 6502. Most 6502 code will work on it without changes but there are additional features that you can take advantage of to write even faster and smaller code. This page is intended to highlight those features.

## 16-bit registers!

The biggest, most obvious change from the 6502 is that it can do 16-bit operations. To enable this, the accumulator and index registers can change between being 8-bit and 16-bit (sort of). This page will go into more detail about that later on.

Two additional flags are added to the flags register to control register size. One flag ("M") controls the size of accumulator operations, as well as operations that do not involve a register such as `BIT`, `INC`, or `ASL` directly on a memory location. Another flag ("X") controls the size of both the X and Y index registers.

Instead of getting dedicated instructions like `SEC`/`CLC`, register sizes are changed with the new `REP` and `SEP` instructions. They take an 8-bit immediate operand and REset or SEt bits in the flags register. This means that you can set and reset multiple bits in the flag register by having multiple bits set in that immediate. For example `SEP #$30` sets M and X simultaneously. This creates a possible optimization where a `CLC` or `SEC` (for `ADC` and `SBC`) can sometimes be removed, and merged into a `REP` or `SEP`.

Processor flags

| REP/SEP value | Flag | Purpose |
| --- | --- | --- |
| #$01 | C | Carry |
| #$02 | Z | Zero |
| #$04 | I | Interrupt disable |
| #$08 | D | Decimal mode |
| #$10 | X | 8-bit index registers when set, 16-bit when clear |
| #$20 | M | 8-bit accumulator/memory when set, 16-bit when clear |
| #$40 | V | Overflow |
| #$80 | N | Negative |

Instead of memorizing these bit values, this is a great opportunity to use macros for more readable code! [lorom-template](https://github.com/pinobatch/lorom-template/blob/master/src/snes.inc) contains [ca65](https://snes.nesdev.org/w/index.php?title=Ca65&action=edit&redlink=1 "Ca65 (page does not exist)") macros named `seta8`, `seta16`, `setxy8`, `setxy16`, `setaxy8` and `setaxy16` and these make it clear what registers are being affected how.

One very important thing to consider with register sizes is they change how the processor reads immediate operands! `LDA #1` may assemble to `A9 01` or `A9 01 00` depending on if the accumulator is supposed to be 8-bit or 16-bit at that point in the code. If the processor tries to run code that's meant for a different register size, it will not work correctly (for instance, `A9 01 00` in 8-bit mode will be interpreted as `LDA #1 \ BRK`). Because of this, it's important to let the assembler know what sizes are required, and the exact syntax for this differs from assembler to assembler.

To tell ca65 what the register sizes are at a given point in the program, the `.a8`, `.a16`, `.i8` and `.i16` directives are used. ca65 also has an optional feature (named [.smart](https://cc65.github.io/doc/ca65.html#ss11.99)) that infers size changes from `REP` and `SEP` but there are still going to be situations where it needs to be explicitly stated. It can be a good practice to specify the register sizes at the start of each subroutine, which serves both as documentation and a guarantee that the routine will be assembled correctly regardless of what code gets put earlier in the file.

Some assemblers like [asar](https://snes.nesdev.org/w/index.php?title=Asar&action=edit&redlink=1 "Asar (page does not exist)") require sizes to specified on a per-instruction basis, like `LDA.B #1` or `LDA.W #1`.

### Bigger accumulator

The accumulator is always two bytes, and register size actually determine if accumulator instructions use the low 8 bits or the all 16 bits. When the accumulator is 8-bit, the other byte can be accessed with a `XBA` instruction, which eXchanges the two bytes.

The ability to set the accumulator to use 16-bit values greatly simplifies math, comparisons, and pointer operations, and allows the same code to be written in fewer instructions (which will consequently be faster). For instance, stepping a pointer forward by one byte can become a single `INC` instruction.

When an instruction does a 16-bit operation on a memory address, it will actually act on the address you specify (containing the low 8 bits) and the address after it (containing the high 8 bits), just like how pointers work.

#### 8-bit variables in 16-bit mode

It's worth noting that you don't always need the accumulator to be in 8-bit mode to work with 8-bit variables. `LDA variable \ AND #$00FF` can often work fine.

### Bigger index registers

16-bit index registers simplify the use of arrays that are bigger than 256 items, allowing the programmer to avoid using pointers in many cases.

X and Y's size flag works differently from the accumulator's. When X and Y are 8-bit, they act as if their upper byte is always being forced to zero. Consequently, this happens if the accumulator is 16-bit and the index registers are 8-bit:

```
LDA #$1234
TAX
TXA
; A is now #$0034
```

This means that if you intend to set X or Y to 8-bit temporarily and then change it back to 16-bit, you must save and restore the other index register if you don't intend to change its value!

### Index registers as pointers

16-bit index registers are big enough to hold an entire 16-bit address, which means they can act as pointers. This means that pointers can be just as fast as regular array access, or even faster in some cases.

One technique you can do with this is storing the address of the first byte of a structure in X, and then you can access different fields in the structure using different offsets from X, using smaller (and sometimes faster) zero page instructions. A bonus is that the rarely-used `(zeropage,x)` addressing mode is a bit more useful - if the structure contains a pointer you can just access it with one instruction like this.

#### Arrays of structures

On the 6502, [parallel arrays](https://en.wikipedia.org/wiki/Parallel_array) (or "structure of arrays") are usually the most efficient way to lay out things like enemy state. However on the 65c816, it can be a good idea to consider array of structures instead!

Consider a structure that contains both 8-bit and 16-bit fields. On the 6502 the 16-bit values would just be spread across separate arrays, but here you would want to take advantage of being able to access a 16-bit number all at once, so you would probably prefer to have the two bytes sequential in memory. The biggest (or only?) downside is that now you need to do a `TXA \ CLC \ ADC #Size \ TAX` sequence to iterate through the list, but it's probably worth it considering the other advantages.

### Transfers between differently sized registers

What happens if you use `TAX`/`TAY`/`TXA`/`TYA` when the accumulator and index registers are different sizes?

Transfers between differently sized registers

| Instruction | Accumulator | Index registers | Outcome |
| --- | --- | --- | --- |
| TAX | 8-bit | 16-bit | *Both* bytes in accumulator are copied to X. |
| TAX | 16-bit | 8-bit | X = Low byte of accumulator. High byte of X is zero. |
| TXA | 8-bit | 16-bit | A = X, including changing the high byte in accumulator to zero. |
| TXA | 16-bit | 8-bit | Low byte of A = X. High byte of A is unmodified. |

### Register size saving

One practice you may find useful is to start a routine with `PHP` and end it with `PLP` if you change the register size inside of it. This way, a caller will get the registers back in the same sizes they were before.

```
php
; Insert code here
plp
rts
```

If you want to save the register values as well, you should push them before you push the register state, so that the correct sizes are restored before they are pulled. Pushing a 16-bit accumulator and then pulling an 8-bit accumulator will probably lead to a crash, as the return address `RTS` gets will be wrong.

```
pha
phx
phy
php
; Insert code here
plp
ply
plx
pla
rts
```

## Small optimizations

There are a lot of little changes over the original 6502 that make life easier, and allow you to use fewer or smaller instructions. Most of these were actually introduced with the 65c02.

- `INA` - Increment accumulator.
- `DEA` - Decrement accumulator.
- `PHX` - Push X register.
- `PHY` - Push Y register.
- `PLX` - Pull X register.
- `PLY` - Pull Y register.
- `TXY` - Copy X register to Y register.
- `TYX` - Copy Y register to X register.
- `STZ` - Store zero. Can be indexed with X, and can be zeropage or absolute.
- `BRA` - Unconditional branch. This can save a byte over `JMP` but not a cycle, as both require exactly 3.
- `Indirect addressing` - Indexing is no longer mandatory on indirect addressing. For example you can write things like `LDA ($00)`.

### Bit tests

The 65c816 provides a few more tools for bit tests. `BIT` becomes much more useful because it can now be indexed, and you can use it with an immediate operand. With an immediate operand it acts identically to `AND` without changing the accumulator.

`TRB` and `TSB` are new tools for bit operations. They take a zeropage or absolute address (non-indexed) and do a bit test, setting the zero flag as if an `AND` had been performed between the accumulator and memory. Next, the value at that memory address is changed, with `TRB` clearing all bits that are set in the accumulator, and `TSB` setting all bits that are set in the accumulator. `TSB` can be a faster replacement for most situations that called for `ORA` followed by `STA` on the same address. This means it's very helpful for piecing together an value from multiple parts:

```
lda XPos
sta Temp
lda YPos
asl
asl
asl
asl
tsb Temp
```

### Jump tables

[Jump tables](https://wiki.nesdev.com/w/index.php/Jump_table) on the 6502 require you to either [push the address you want to jump to onto the stack](https://www.nesdev.org/wiki/RTS_Trick), or store it to an address before using an indirect jump. On the 65c816, there are instructions that are specifically for jump tables. `JMP (absolute,x)` and `JSR (absolute,x)` both exist, though you'll still have to do it the old ways if you need to preserve X or want to jump to a 24-bit address.

```
asl
tax
jmp (Table,x)

Table:
.addr Routine1
.addr Routine2
.addr Routine3
```

## Banks and 24-bit addresses

Banks work very differently on the 65c816 than they would on a 6502 system like the NES. The address space is 24-bit, which provides access to a whole 16 megabytes of data. This means that it doesn't usually make sense to do bank switching, because there is already plenty of space.

### 24-bit program counter

To go with the bigger address space, the program counter has an 8-bit "bank" byte added to it, resulting in a 24-bit register. `JMP`, `JSR`, and `RTS` still use 16-bit addresses, and do not modify the bank byte. There are now `JML`, `JSL` and `RTL` instructions that jump to a full 24-bit address and do change the program counter's bank. `RTI` also now takes a 24-bit address - which is important to note if you're using RTI for the RTS trick!

This means that when you write routines, you need to decide whether it should be callable from only the current bank, or from any of them. A routine you call with `JSL` must return via `RTL` just as a routine you `JSR` needs a `RTS`.

### The data bank

Similar to the situation with `JMP`, loads, stores and other data accesses with 16-bit addresses (but not zeropage ones) get extended out to 24-bit with a bank byte. In this case it's called the "data bank" register. You can only interact with it through the `PHB` and `PLB` instructions, so setting the data bank has to involve pushing the bank number to the stack. If you want the data bank to equal the program bank, you can use the `PHK` instruction, which pushes the program bank. An example follows:

```
php ; Save register sizes
phb ; Save original data bank
phk ; Push the program counter's bank
plb ; Store it to the data bank
; Insert code that changes the data bank and register sizes to something else
plb ; Restore data bank
plp ; Restore register sizes
rtl
```

You need to keep the data bank in mind when calling code that's in another bank. If you `JSL` somewhere, the data bank won't necessarily be correct for any lookup tables in the target code bank.

In ca65, in addition to < and > to fetch the bottom 8 bits or next 8 bits of a value/label, you can use ^ to get the bank byte. This can be used both for setting up 24-bit pointers and for setting the data bank to be correct for a specific label.

In order to set the data bank to something other than the current program bank, you *can* set a register to the desired value and push it, but you an also use the `PEA` instruction, which takes a 16-bit immediate value and pushes it to the stack. It's not ideal because it's 16-bit and not 8-bit, but it will work. You can `PEA` and then `PLB` twice, but if you know what bank value you will need next you can do an optimization. Following is a ca65 macro from [lorom-template](https://github.com/pinobatch/lorom-template/blob/master/src/snes.inc) which makes this easier to work with:

```
;;
; Pushes two constant bytes in the order second, first
; to be pulled in the order first, second.
.macro ph2b first, second
.local first_, second_, arg
first_ = first
second_ = second
arg = (first_ & $FF) | ((second_ & $FF) << 8)
  pea arg
.endmacro
```

### Addressing modes

`LDA`, `STA`, `ADC`, `SBC`, `ORA`, `AND`, `EOR`, and `CMP` get new addressing modes that provide access to 24-bit addresses, ignoring the data bank. The following examples use ca65 syntax:

- **`f:absolute, x`** - 24-bit address with indexing
- **`[zeropage]`** - 24-bit version of (zeropage)
- **`[zeropage],y`** - 24-bit version of (zeropage) with indexing. [zeropage,x] does not exist.

If you need to access data from a bank different from the one you have set as the data bank, you'll have to plan out how you want to use the X and Y registers, given that only X can be used with far absolute addressing.

## Stack changes

The 65c816 stack pointer is 16-bit, so it can point anywhere in the first 64KB of the address space. On the SNES it will probably be somewhere in $0000-$1FFF, which is RAM.

### New stack addressing modes

The 65c816 makes it more feasible to put function arguments or local variables on the stack with new addressing modes on `LDA`, `STA`, `ADC`, `SBC`, `ORA`, `AND`, `EOR`, and `CMP` as well as a new 16-bit stack pointer. The new addressing modes are `stack,s` and `(stack,s),y` which index an 8-bit address with the stack pointer. Remember that the stack pointer points to the next available slot, so 1,s will go to the most recently pushed byte, 2,s will go to the next recently pushed byte and so on.

If you want to work with values on the stack, you should be aware of the `TSC` and `TCS` instructions. With these you can easily copy the stack pointer into the accumulator, subtract for however many local variables you want to make room for, and copy back to the stack pointer.

Probably the biggest downside to using the stack like this is that only the above instructions work with it. `LDX`, `INC` and such don't have the addressing modes available.

## Movable "zeropage"/direct page

The 65c816 allows you to move zeropage to anywhere in the first 64KB of the address space. As a result, it's usually renamed to the "direct page". You're provided the `TDC` and `TCD` instructions to copy the accumulator to/from the base of the direct page. Direct page does not even need to start on a page boundary, but ideally it should be because otherwise there is a cycle added to all direct page instructions.

Some useful places you might want to put direct page are $2100 (PPU registers) and $4300 (DMA registers). If you're trying to write code that executes as fast as possible during vblank, this can help by allowing smaller, faster instructions.

## Decimal mode

This isn't new to the 65c816, but will be new to NES developers. The SNES has a functional decimal mode! You should consider using it for values that are mostly for displaying, like money amounts or scores. One thing to keep in mind is that decimal mode only applies to `ADC` and `SBC`, so increments and decrements must be done using those.

## Emulation mode

The 65c816 starts out in an "emulation mode" which is supposed to simulate a 65c02 by changing a few aspects of the processor (stack pointer is 8-bit and fixed to $0001xx, registers are fixed to 8-bit, interrupts use 16-bit addresses, some instructions take longer). You can completely ignore it, aside from writing code to switch out of it (`CLC \ XCE`) as it's really only helpful for 65c816 successors to 6502 computers that are meant to be able to run old programs without changes, like the Apple IIGS. It won't help with porting NES code to the SNES.

## SNES-specific math

The SNES has [multiplication and division](https://problemkaputt.de/fullsnes.htm#snesmathsmultiplydivide) I/O registers. These are going to be significantly faster than doing the math with a general-purpose multiplication or division routine. The following math operations are available:

- unsigned 8-bit × 8-bit, with a 16-bit result
- unsigned 16-bit ÷ 8-bit, producing a 16-bit result and 16-bit remainder
- signed 16-bit × 8-bit, producing a 24-bit result

The unsigned math registers take time to work, and they need a delay before the result is valid. This can be accomplished with `nop` instructions, or (better) by doing other useful work in the meantime. The signed multiplication registers are part of the PPU, and are not usable when it is rendering Mode 7, as it repurposes hardware meant for that.

## References

- [6502.org 65c816 Opcodes](http://www.6502.org/tutorials/65c816opcodes.html) - Notes and opcodes for 65c816 from a 6502 programmer's perspective.
