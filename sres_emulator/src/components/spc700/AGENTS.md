# sres_emulator/src/components/spc700/

## What It Is

Sony SPC700 audio co-processor emulator. 8-bit, ~2.0496 MHz (Mesen2-calibrated catch-up ratio). Lives inside `Apu` (`src/apu/mod.rs`). Advanced lazily via `Apu::update_clock` — not every CPU cycle.

## Files

| File | Purpose |
|------|---------|
| `mod.rs` | Core `Spc700<BusT>` struct, `Spc700Bus` trait, step/reset |
| `instructions.rs` | All 256 instruction bodies |
| `opcode_table.rs` | Macro-built 256-entry opcode table |
| `operands.rs` | Addressing modes, operand decode/load/store |
| `status.rs` | PSR flags (`nvpbhiZc`) |
| `debug.rs` | Trace formatting, disassembly, debugger events |
| `test.rs` | TomHarte ProcessorTests harness |
| `test/0x.json.xz` … `fx.json.xz` | Per-nibble opcode test traces |

## Core Struct

```rust
pub struct Spc700<BusT: Spc700Bus> {
    bus: BusT,
    pc: AddressU16, a: u8, x: u8, y: u8, sp: u8,
    status: Spc700StatusFlags,
    opcode_table: [InstructionDef<BusT>; 256],
}
```

- Generic over `Spc700Bus` (real `ApuBus` or `TestBus`).
- Stack in page `0x0100..0x01FF`.

## Bus Trait

```rust
pub trait Spc700Bus: Bus<AddressU16> {
    fn spc_cycle(&self) -> u64;
    fn master_clock(&self) -> u64;
    fn update_master_clock(&mut self, cycles: u64);
}
```

Real impl: `ApuBus` (`src/apu/apu_bus.rs`)—adds RAM, DSP, timers, APUIO.

## Status Flags

| Bit | Flag | Name |
|-----|------|------|
| 7 | N | Negative |
| 6 | V | Overflow |
| 5 | P | Direct page bank (0x00 or 0x01) |
| 4 | B | Break |
| 3 | H | Half-carry |
| 2 | I | IRQ enable |
| 1 | Z | Zero |
| 0 | C | Carry |

`From<u8>`/`Into<u8>`, `Display`/`FromStr` as `nvpbhiZc`.

## Instruction Categories

All in `instructions.rs`:

- **ALU**: `adc`, `sbc`, `and`, `or`, `eor`, `cmp`, `inc`, `dec`, `mul`, `div`
- **Shifts**: `rol`, `ror`, `asl`, `lsr`
- **16-bit**: `addw`, `subw`, `cmpw`, `incw`, `decw`, `movw` (on `YA`)
- **Branches**: `bra`, `beq`/`bne`, `bpl`/`bmi`, `bvc`/`bvs`, `bcs`/`bcc`, `bbs`/`bbc`, `cbne`, `dbnz`
- **Status/Stack**: `clrp`/`setp`, `clrc`/`setc`, `clrv`, `ei`/`di`, `push`/`pop`
- **Jumps/Calls**: `jmp`, `call`, `tcall`, `pcall`, `ret`, `reti`, `brk`
- **Bit**: `set1`/`clr1`, `tset1`/`tclr1`, `not1`, `or1`/`and1`/`eor1`, `mov1`
- **Misc**: `mov`, `nop`, `xcn`, `daa`/`das`, `sleep`, `stop`

## Addressing Modes

13 modes in `operands.rs`: `Dp`, `DpXIdx`, `DpYIdx`, `DpXIdxIndirect`, `DpIndirectYIdx`, `XIndirect`, `YIndirect`, `XIndirectAutoInc`, `Abs`, `AbsXIdx`, `AbsYIdx`, `AbsXIdxIndirect`.

- `Direct page` = `0x00xx` if `P=0`, else `0x01xx`.
- Two enums: `Operand` (static definition) and `DecodedOperand` (runtime after decode).
- `DecodedOperand` provides `load`, `store`, `load_u16`, `store_u16`, `bit()`.

## Opcode Table

Built at construction by `build_opcode_table()`. Macro `instruction!` generates entries:

- `instruction!(nop)`            — no operands
- `instruction!(asl, InMemory(Dp))`  — one operand
- `instruction!(mov, Register(A), InMemory(Dp))` — two operands

Each entry: `execute` closure (increments `pc` by 1, then calls instruction method) and `disassembly` closure.

## Cycle Accuracy

Every bus transaction is explicit:

- `bus.cycle_read_u8(addr)` / `bus.cycle_read_u16(addr, wrap)`
- `bus.cycle_write_u8(addr, value)`
- `bus.cycle_io()` — idle cycle

Instructions manually insert correct `cycle_io()` / `cycle_read_u8` counts to match hardware.

## Clocking

Catch-up uses Mesen2's calibrated ratio (see `Spc::UpdateClockRatio`):

```rust
const SPC_CLOCK_FREQUENCY: u64 = 32040 * 64;      // 2,049,600 Hz
const MASTER_CLOCK_FREQUENCY: u64 = 21_477_270;   // NTSC master clock
```

`catch_up_to_master_clock(master_cycles)` converts master → SPC cycles, steps until caught up, and **returns** the exposed SPC cycle. The APU integration layer (`Apu::catch_up_and_promote_channel_out`) uses that return value to promote deferred CPUIO out-port writes in `ApuBus` — the SPC700 component itself does not handle port visibility.

Audio sample boundaries in `apu/mod.rs` still use the nominal 32 kHz / 21,477,272 Hz ratio for `CYCLES_PER_SAMPLE`.

## Debug & Tracing

- `Spc700Debug::state()` → `Spc700State` (registers, disassembly, cycles).
- Formats: BSNES-style (`00000088 [FFC5] MOV (X),A …`) and Mesen parse-in (`FFC5 MOV (X),A [$00EF] = $71 …`).
- Emits `Spc700Event::Step(Spc700State)` to `DebugEventCollectorRef`.

## Tests

### TomHarte ProcessorTests

- 16 `.json.xz` files (`0x`–`fx`), one line per test case.
- One `#[test]` per file; constructs `Spc700<TestBus>`, calls `step()`, compares CPU state + memory + bus cycles.
- Skip lists:
  - `SKIP_OPCODES`: `&[]`
  - `IGNORE_CYCLE_DETAILS`: `&[0xCA, 0xD7, 0xFE]` (only cycle count checked, not exact addresses/values)

### APU-Level Tests

`src/apu/test.rs` runs Mesen boot-ROM traces through full `ApuBus`, verifying SPC700 state, APUIO, timers, DSP.

## Key Patterns

1. **Generic bus** — `Spc700<BusT>` runs under `ApuBus` (prod) and `TestBus` (tests).
2. **Macro opcode table** — All 256 opcodes declared in one macro block.
3. **Decode-Load-Store separation** — Decode first, then load/store; mirrors hardware and enables disassembly.
4. **Explicit cycle calls** — Every memory access or idle cycle is a bus method call.
5. **Lazy catch-up** — SPC only advanced when needed (APUIO or sample boundary).
6. **Status struct** — PSR is a plain struct with `From<u8>`, no manual bit twiddling.

## Integration Diagram

```
Main CPU ──► APUIO read/write ──► Apu (via MainBusImpl + update_clock)
                                      │
                                      ▼
                           catch_up_and_promote_channel_out()
                                      │
                    ┌─────────────────┴─────────────────┐
                    ▼                                   ▼
         Spc700::catch_up_to_master_clock()    ApuBus::promote_channel_out()
                    │                                   │
                    ▼                                   │
               Spc700::step()                           │
                    │                                   │
                    ▼                                   │
               ApuBus (RAM, DSP, Timers) ◄──────────────┘
                    │
                    ▼
              generate_sample()
```
