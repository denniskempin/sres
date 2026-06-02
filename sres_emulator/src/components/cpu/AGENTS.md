# CPU

W65C816 (65816) CPU core. Cycle-accurate, generic over `BusT: MainBus`.

## Files

| File | Purpose |
|------|---------|
| `mod.rs` | `Cpu` struct, registers, `step()` loop, reset/interrupts, `MainBus` trait. |
| `status.rs` | `StatusFlags` (N, V, M, X, D, I, Z, C). |
| `operands.rs` | `AddressMode`, `Operand`, decode/peek logic. Page-cross and direct-page penalties handled here. |
| `opcode_table.rs` | 256-entry opcode table. Macros generate monomorphized closures per opcode for inlining. |
| `instructions.rs` | One function per mnemonic. Generic over `T: UInt` (u8/u16) for 8/16-bit modes. |
| `debug.rs` | `CpuDebug`, `CpuState`, trace formatting (Mesen/BSNES compatible), MMIO register names. |
| `test.rs` | Integration tests against TomHarte/ProcessorTests 65816 JSON data. |

## Key Types

- **`Cpu<BusT: MainBus>`** — Core CPU. Registers: `pc` (24-bit), `a`/`x`/`y` (`VariableLengthRegister`, width depends on M/X flags), `s` (stack), `d` (direct page), `db` (data bank), `status`, `emulation_mode`.
- **`VariableLengthRegister`** — Private u16 helper. Reads mask to u8 when flag set; u8 writes preserve high byte.
- **`StatusFlags`** — P register. In emulation mode, X bit acts as Break flag.
- **`Operand`** — Decoded operand. `decode()` does bus reads + IO cycles; `peek()` is immutable (for disassembly).
- **`Instruction<BusT>`** — Table entry holding `execute` fn and `meta` fn (for disassembly).

## Dispatch

Opcode table built at startup (not static) because closures are generic over `BusT`. Macros create unique functions per opcode so `AddressMode`/`AccessMode` constants inline into `Operand::decode`.

Macro patterns:
- `instruction!(nop)` — implied, no operand.
- `instruction!(lda, AbsoluteData, Read, A)` — operand + access + register (determines u8/u16 dispatch).
- `instruction!(jmp, AbsoluteJump, Read)` — operand + access, fixed size.

## Instruction Patterns

- ALU/load/store are generic `T: UInt`. Dispatched u8/u16 based on M/X flags.
- Implied instructions start with `cycle_io()`. RMW: load → `cycle_io` → store. Branches: `cycle_io` only on taken.
- `update_negative_zero_flags<T: UInt>` standard helper.
- `adc`/`sbc` branch on `status.decimal` for BCD mode.
- Emulation mode quirks in `xce`, `txs`, `brk`, `cop`, `rti`, etc. (forces M/X to 8-bit, stack high byte = 0x01).

## Addressing Quirks

- **Direct Page:** Extra IO cycle if `d.low_byte() != 0`.
- **Absolute X/Y:** Page-cross adds IO cycle unless 8-bit index and no cross. Write/Modify always penalty.
- **Stack Relative:** Always extra IO cycle.
- **MVN/MVP:** `MoveAddressPair` operand.

## Reset & Interrupts

- **Reset:** PC from `EmuVectorTable::Reset` at `0xFFFC`.
- **NMI/IRQ:** Checked end of `step()`. `interrupt()` pushes PC/status, clears D, sets I, vectors through `NativeVectorTable` (or `EmuVectorTable` for `brk`/`cop` in emulation mode).
- **Vectors:** Native `0xFFE4–0xFFEE`, Emu `0xFFF4–0xFFFE`.

## Tests

TomHarte 65816 ProcessorTests JSON (`test/*.json.xz`, Git LFS). `run_tomharte_test("Nx")` per opcode block. Compares `CpuState`, memory, and cycle count. **Skipped:** `0x44` (MVP), `0x54` (MVN) — different implementation model.

## Integration

- Instantiated as `Cpu<MainBusImpl>` by system top-level.
- `step()` performs one instruction; bus advances clock per cycle.
- Bus raises NMI/IRQ; CPU consumes at end of instruction.
- `CpuEvent::Step`/`Interrupt` emitted to `DebugEventCollectorRef`.
