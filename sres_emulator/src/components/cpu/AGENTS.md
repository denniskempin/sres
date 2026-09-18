# `sres_emulator/src/components/cpu`

W65C816 (65816) CPU core.

## Files

| File | Owns |
|------|------|
| `mod.rs` | `Cpu`, `step()`, `reset()`, `MainBus`, `NativeVectorTable`, `EmuVectorTable` |
| `status.rs` | `StatusFlags` (P) |
| `operands.rs` | `AddressMode`, `AccessMode`, `Operand` (decode clocks; peek does not) |
| `opcode_table.rs` | `Instruction`, `build_opcode_table`, `instruction!` macros |
| `instructions.rs` | One function per mnemonic |
| `debug.rs` | `CpuDebug`, `CpuState`, `CpuEvent`; Mesen parse / BSNES `Display` |
| `test.rs` | TomHarte harness (`run_tomharte_test`) |

## Behaviors & Gotchas

1. `instruction!` emits a unique execute fn per opcode so `AddressMode`/`AccessMode` constants inline into `Operand::decode`: `instruction!(nop)`; `instruction!(lda, AbsoluteData, Read, A)`; `instruction!(jmp, AbsoluteJump, Read)`.
2. Direct page: extra `cycle_io` if `d.low_byte() != 0`.
3. Absolute X/Y: extra `cycle_io` on page-cross, 16-bit index, or Write/Modify.
4. Stack-relative: always extra `cycle_io` (`StackRelativeIndirectYIndexed` does two).
5. Implied ops start with `cycle_io()`. RMW: load → `cycle_io` → store. Conditional branches: `cycle_io` only when taken (`bra`/`brl` always).
6. `MVN`/`MVP` rewind `pc` by 3 until `a == 0xFFFF` so the next `step()` repeats the opcode.
7. `step()` executes one instruction, then `consume_nmi_interrupt` / `consume_timer_interrupt` (IRQ also requires `!status.irq_disable`).
8. `reset()` loads PC from `EmuVectorTable::Reset` via `peek_u16` (no cycles). `interrupt()` pushes 24-bit PC + P, clears D, sets I, vectors through `NativeVectorTable`. `brk`/`cop` use `EmuVectorTable` when `emulation_mode`.
9. `xce` into emulation forces M/X 8-bit and `s = 0x0100 + (s & 0x00ff)`. `update_register_sizes` zeros X/Y high bytes when the X flag is set; `VariableLengthRegister::set::<u8>` keeps A's high byte.

## Integration

- `SystemImpl` owns `Cpu` (parent).
- `MainBus` extends `Bus<AddressU24>` with `consume_nmi_interrupt`, `consume_timer_interrupt`, `clock_info`.
- `CpuEvent::Step` / `Interrupt` go to `DebugEventCollectorRef`.

## Gaps

- `wai` burns three `cycle_io` and returns; does not halt until interrupt.
- Emulation-mode NMI/IRQ: `interrupt()` always uses `NativeVectorTable` and `stack_push_u24`; `EmuVectorTable::Nmi` / `Irq` are unused.

## Tests

- TomHarte 65816 JSON (`test/{0x–fx}.json.xz`). `run_tomharte_test` compares `CpuState`, memory, and cycle count.
- `SKIP_OPCODES`: `0x44` MVP, `0x54` MVN — test model differs; both instructions are implemented.
- `cargo nextest run -p sres_emulator --lib -E 'test(components::cpu::)'`
