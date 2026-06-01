# CPU Component Summary

This directory implements the **W65C816 (65816)** CPU core — the main processor of the SNES.

## Overview

The CPU is a generic `Cpu<BusT: MainBus>` that executes instructions cycle-by-cycle using a 256-entry opcode lookup table. It is fully cycle-accurate (driven by `cycle_read_*`, `cycle_write_*`, and `cycle_io` calls on the bus) and supports both Native and Emulation modes.

## File Breakdown

| File | Purpose |
|------|---------|
| `mod.rs` | Core `Cpu` struct, register definitions, `step()` loop, reset/interrupt handling, stack helpers, and `MainBus` trait. |
| `status.rs` | `StatusFlags` struct (N, V, M, X, D, I, Z, C) with `u8` ↔ `StatusFlags` conversions. |
| `operands.rs` | `AddressMode`, `AccessMode`, and `Operand` enums. Decodes operands for all addressing modes, including page-cross penalties and extra IO cycles. Shares logic between execution and disassembly via `ReadOrPeekWrapper`. |
| `opcode_table.rs` | `build_opcode_table()` constructs the 256-entry table. Uses macros to generate monomorphized `execute` and `meta` closures per opcode for compiler optimization. |
| `instructions.rs` | One function per mnemonic (e.g., `adc`, `lda`, `php`). Most data operations are generic over `T: UInt` (u8/u16) so the same code handles 8-bit and 16-bit register sizes. |
| `debug.rs` | `CpuDebug`, `CpuState`, `CpuEvent`, and trace formatting compatible with Mesen/BSNES. Includes a large map of SNES MMIO register names (`ADDR_ANNOTATIONS`). |
| `test.rs` | Integration tests against [TomHarte/ProcessorTests](https://github.com/TomHarte/ProcessorTests) 65816 JSON data (stored as `.json.xz` in `test/`). |

## Key Types

### `Cpu<BusT: MainBus>` (`mod.rs`)
- **Registers:**
  - `pc: AddressU24` — 24-bit program counter.
  - `a: VariableLengthRegister` — Accumulator (u8 or u16 depending on `M` flag).
  - `x, y: VariableLengthRegister` — Index registers (u8 or u16 depending on `X` flag).
  - `s: u16` — Stack pointer.
  - `d: u16` — Direct page register.
  - `db: u8` — Data bank register.
  - `status: StatusFlags` — Processor status.
  - `emulation_mode: bool` — Emulation vs Native mode.
- **State:** `halt: bool` (set by `stp` instruction).
- **Trait bound:** `MainBus` extends `Bus<AddressU24>` and adds `consume_nmi_interrupt`, `consume_timer_interrupt`, and `clock_info`.

### `StatusFlags` (`status.rs`)
Bit layout matches the 65816 P register:
```
7 6 5 4 3 2 1 0
N V M X D I Z C
```
In emulation mode, the `X` bit acts as the Break (`B`) flag.

### `VariableLengthRegister` (`mod.rs`)
A private helper holding a `u16`. Reads mask to u8 when the relevant status bit is set; writes in u8 mode only touch the low byte, preserving the high byte.

### `Operand` / `AddressMode` (`operands.rs`)
`AddressMode` enumerates all 65816 addressing modes (Implied, ImmediateA, ImmediateXY, Absolute, AbsoluteLong, DirectPage, StackRelative, etc.).

`Operand` is the decoded result:
- `Implied`
- `Accumulator`
- `ImmediateU8(u8)` / `ImmediateU16(u16)`
- `Address(u32, AddressMode, AddressU24)` — raw operand data, mode, and effective address
- `MoveAddressPair(u8, u8)` — for MVN/MVP

`Operand::decode` performs the bus reads and IO cycles needed to resolve an operand. `Operand::peek` does the same without mutating state (used for disassembly).

### `Instruction<BusT>` (`opcode_table.rs`)
Each opcode table entry holds:
- `execute: fn(&mut Cpu<BusT>)` — runs the instruction.
- `meta: fn(&Cpu<BusT>, AddressU24) -> (InstructionMeta, AddressU24)` — returns disassembly metadata and the next PC.

## Instruction Dispatch Pattern

The opcode table is generated at startup (not a static const) because it uses closures over the generic `BusT`. Macros create a unique function per opcode so the compiler can inline and specialize the constant `AddressMode`/`AccessMode` parameters into `Operand::decode`.

Macro variants in `opcode_table.rs`:
1. `instruction!(nop)` — implied, no operand.
2. `instruction!(lda, AbsoluteData, Read, A)` — operand + access mode + variable register (`A`/`X`/`Y`). The register determines whether the generic instruction is called as `<u8>` or `<u16>`.
3. `instruction!(jmp, AbsoluteJump, Read)` — operand + access mode, fixed size.

## Instruction Implementation Patterns (`instructions.rs`)

- **Generic width:** ALU and load/store instructions use `T: UInt` generics. The opcode table dispatches to `u8` or `u16` based on the `M` or `X` flag.
- **Bus cycles:** Most implied instructions start with `cpu.bus.cycle_io()`. Read-modify-write instructions do `load` → `cycle_io` → `store`. Branches do `cycle_io` only on taken.
- **Flags:** `update_negative_zero_flags<T: UInt>` is the standard helper.
- **Decimal mode:** `adc` and `sbc` branch on `cpu.status.decimal` and call `add_bcd` / `sub_bcd` from the `UInt` trait.
- **Emulation mode quirks:** `xce`, `txs`, `brk`, `cop`, `rti`, etc. check/apply emulation-mode behavior (e.g. forcing M/X to 8-bit, setting stack high byte to 0x01).

## Addressing Mode Details (`operands.rs`)

- **Direct Page:** If `d.low_byte() != 0`, an extra IO cycle is inserted.
- **Absolute X/Y Indexed:** A page-cross detection adds an IO cycle unless reading with 8-bit index registers and no cross. Write/Modify always incur the penalty.
- **Stack Relative:** Always adds an IO cycle.
- **Relative / RelativeLong:** Target is computed from PC + operand + instruction size.
- **Absolute Indirect Jump:** Fetches target from memory; uses current PC bank.
- **Absolute Indirect Long:** Fetches 24-bit target.

The `ReadOrPeekWrapper` trait and its two implementations (`ReadWrapper` / `PeekWrapper`) allow the same decode code to be used for both execution (mutable, real bus cycles) and debug disassembly (immutable, peeks).

## Reset & Interrupts (`mod.rs`)

- **Reset:** Sets PC from the emulation vector at `0xFFFC` (via `EmuVectorTable::Reset`).
- **NMI/IRQ:** Checked at the end of `step()`. `interrupt()` pushes PC and status, clears D, sets I, and vectors through `NativeVectorTable` (or `EmuVectorTable` for `brk`/`cop` in emulation mode).
- **Vectors:** `NativeVectorTable` (0xFFE4–0xFFEE) and `EmuVectorTable` (0xFFF4–0xFFFE).

## Debug & Tracing (`debug.rs`)

- `CpuState` captures full snapshot: registers, status, instruction metadata, and clock info.
- `parse_mesen_trace` / `Display for CpuState` support cross-referencing with Mesen/BSNES trace logs.
- `ADDR_ANNOTATIONS` maps SNES MMIO addresses (PPU, APU, DMA, etc.) to symbolic names for disassembly.

## Test Conventions (`test.rs`)

Tests are **integration tests** against TomHarte’s 65816 ProcessorTests:
- Data files are `test/{0x,1x,...,fx}.json.xz` (Git LFS), containing thousands of per-instruction test cases.
- Each test function calls `run_tomharte_test("Nx")`.
- `TestCpuState` deserializes JSON into a `Cpu<TestBus<AddressU24>>`.
- After `step()`, the test compares:
  1. `CpuState` equality
  2. `TestBus::memory` equality
  3. Cycle count equality (order is ignored)
- **Skipped opcodes:** `0x44` (MVP) and `0x54` (MVN) are skipped because test expectations follow a different implementation model.
- Line-by-line JSON streaming is used to keep memory usage low.

## How It Fits Into the Emulator

- **Instantiated by:** The system top-level creates `Cpu<MainBusImpl>` where `MainBusImpl` is the SNES main bus (combines WRAM, cartridge ROM, PPU, APU, DMA, etc.).
- **Clocking:** `Cpu::step()` performs one instruction, driving bus cycles. The bus is responsible for advancing the system clock on each cycle.
- **Interrupts:** The bus raises NMI/IRQ flags; the CPU consumes them at the end of the current instruction.
- **Debugging:** `CpuEvent::Step` and `CpuEvent::Interrupt` are emitted to a `DebugEventCollectorRef`, consumed by the GUI/debugger.
