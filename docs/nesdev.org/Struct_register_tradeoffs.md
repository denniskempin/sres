---
title: "Struct register tradeoffs"
source_url: "https://snes.nesdev.org/wiki/Struct_register_tradeoffs"
pageid: 200
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Action games usually need to keep track of the position, direction, status, and other details for multiple game objects. It follows that the game's code needs to keep track of which object in particular it's processing (commonly referred to as "[this](https://en.wikipedia.org/wiki/This_(computer_programming))"), and the reference to `this` needs to be stored in a register in order to allow accessing the object's data. Because the 65c816's registers have some major differences in what they're capable of, which register is chosen ends up being a big factor in shaping how the gameplay related parts of the engine use the 65c816.

# Differences between X and Y

The main differences between the two index registers relate to what addressing modes exist, and what instructions support which addressing modes. The main instructions (`LDA`, `STA`, `AND`, `ORA`, `EOR`, `ADC`, `SBC`, `CMP`) have very broad addressing mode support, and they can use both "absolute,x" and "absolute,y" addressing. However while `ASL`, `ROL`, `LSR`, `ROR`, `INC`, `DEC`, `BIT`, `STZ` all support indexing, they can only use addresses indexed with the X register specifically. `TRB`, `TSB`, `PEI`, `CPX`, `CPY` access memory but don't support indexing at all.

The 65c816 supports `absolute long,x` addressing for the main instructions, which allows access to tables in arbitrary banks, ignoring the data bank system. `absolute long,y` doesn't exist.

Indirect addressing modes are mostly exclusive to the Y register. `(direct page),y`, `[direct page],y`, `(d, s),y` do not have X equivalents. `(direct page, x)` does exist, but a need for it doesn't come up very often. In contrast, for jump tables, `JMP (absolute,x)` and `JSR (absolute,x)` exist, but not versions that use the Y register. Thankfully `(direct page)` and `[direct page]` do exist, and indexing a pointer isn't mandatory.

`direct page,x` is supported by many instructions, but `direct page,y` is only supported by `LDX` and `STX` and no other instructions. `direct page,x` is particularly helpful because it's one cycle faster than `absolute,x` or `absolute,y` when the index registers are 16-bit, though they're the same speed otherwise.

# X as "this"

The main appeals of using X for `this` are that `direct page,x` can save cycles when using 16-bit index registers, and `ASL`, `ROL`, `LSR`, `ROR`, `INC`, `DEC`, `BIT`, `STZ` can be used on struct fields. In exchange, it can require more complicated code than Y would. See [[Using X as a pointer]] for a guide talking about directly using X as a pointer, which allows for the `direct page,x` advantages described here.

## Pros

- `ASL`, `ROL`, `LSR`, `ROR`, `INC`, `DEC`, `BIT`, `STZ` support `absolute,x` and `direct page,x` addressing.
  - It may be useful to `DEC` a timer, `INC` a counter, or `BIT` a flag.
- `direct page,x` is available, which is faster than `absolute,x` when X is 16-bit.
- `direct page,x` allows accessing structs regardless of the data bank register's current value, because direct page is always bank zero.
- Y register is free for use with indirect addressing modes.
  - It may be convenient to use `[direct page],y` as a way to access a specific table regardless of what the data bank register is set to. This is slower than `absolute long,x` would be, but is still faster than saving/restoring X. Games with large levels may benefit from having a pointer to level data.
- `(direct page,x)` can be used to access a pointer stored in a struct, but only as-is without additional indexing.

## Cons

- `absolute long,x` requires saving and restoring X. The most straightforward way to avoid having to do that would be to use `absolute,y`, but that requires the data bank register to point at the bank the table is in, and means the programmer has to manage the data bank within their program.
  - In practice, this can imply keeping lookup tables in the same bank as the code that uses the tables, and synchronizing the data bank with the program bank, and running the code within banks $80-$BF (to allow accessing both struct data and lookup tables with `absolute,y`).
- It's inconvenient to pass `this` to code called via jump tables, because X is needed to index into the jump table. There are ways around this, including the [RTS trick](https://www.nesdev.org/wiki/RTS_Trick) and by writing the jump destination to RAM and using a non-indexed `JMP` indirect.

# Y as "this"

Using `Y` for `this` loses access to some optimization potential in exchange for potentially simpler code and some other benefits.

## Pros

- X register is free for `absolute long,x`, allowing for easy access to tables in arbitrary banks at any time. With 16-bit X, it's the same speed as `absolute,x`, so there are no extra cycles.
- Data bank register can kept at $7E, which contains 64 KiB of RAM (see [[Memory map]]) making it convenient to access large amounts of variables and RAM tables while also conveniently accessing ROM tables (thanks to `absolute long,x`). This means that the programmer also does necessarily have to deal with moving the data bank register around.
- It's trivial to pass `this` to code called via `JMP (absolute,x)` and `JSR (absolute,x)`.
- For some games, it might be more beneficial to have `ASL`, `ROL`, `LSR`, `ROR`, `INC`, `DEC`, `BIT`, `STZ` available for other tables, instead of for `this`.
  - Maybe there's some sort of bit field that would be useful to use `ASL` and other shift instructions on?

## Cons

- Struct fields for your `this` have to be done through `absolute,y` addressing due to the lack of a `direct page,y`, which is one cycle slower with 16-bit Y.
- `(direct page),y` and `[direct page],y` run into the same problem that `absolute long,x` do with X as `this`. However, indirect addressing isn't as necessary as it is on the 6502, as it's not required just to have arrays with over 256 entries.

# Direct page pointer as "this"

The 65c816 provides an alternative to using an index register at all for `this` by letting the programmer move the direct page anywhere within bank zero. This means that if the direct page is positioned at the start of a data structure, instructions that use direct page addressing will access memory relative to the start of that structure. The main downside is that if the direct page pointer is not a multiple of 256 bytes, one cycle will be added to all instructions that use direct page.

## Pros

- X and Y are free to do anything without having to save and restore them to preserve `this`. They can both be used as counters or pointers, and the programmer can access `this` alongside two tables simultaneously.
- `ASL`, `ROL`, `LSR`, `ROR`, `INC`, `DEC`, `BIT`, `STZ` can be used on struct fields, but `TRB`, `TSB`, `PEI`, `CPX`, `CPY` are also available.
- 4-cycle struct field accesses via `direct page` accesses, matching `absolute, x` for 8-bit X/Y but beating it for 16-bit X/Y.
- Like with Y as `this`, it's trivial to pass `this` to code called via `JMP (absolute,x)` and `JSR (absolute,x)`.
- It's easy to use arrays and pointers stored within a struct.

## Cons

- Most common assemblers do not provide tools to help with a non-zero direct page position. (64tass does).
- Global variables now have to use the slower absolute addressing, because they can no longer use direct page.
- Local variables have to either use absolute addressing (4 cycles) or put within reach of direct page (4 cycles with unaligned direct page).
- It's inconvenient to have global variables accessed with indirect addressing, which requires direct page.
- Direct page is limited to bank zero, so structs have to be accessed through bank zero, which means a limit of 8 KiB unless more memory has been added to that bank. This limitation would also apply to using `direct page,x` with X as `this`.

## With an aligned direct page

When the direct page is aligned to a multiple of 256, the extra cycle costs go away and direct page accesses are as fast as they normally are. This means 3-cycle struct field accesses, which are as fast as they can possibly be. This does mean that each struct is either now 256 bytes big, which can really eat into the first 8 KiB of RAM available in bank zero, or an awkward memory setup where the space between the structs is repurposed for something else. Fast local variables can be done by putting them alongside the data for the current structure.
