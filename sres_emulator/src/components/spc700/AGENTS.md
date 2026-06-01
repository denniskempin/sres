# SPC700 Component Summary

## What is the SPC700?

The SPC700 is an 8-bit microprocessor that serves as the audio co-processor inside the SNES APU. It runs at 2.048 MHz (derived from the 32 kHz audio sample rate × 64) and communicates with the main CPU via four APUIO channels. In this emulator, the SPC700 lives inside `Apu` (`sres_emulator/src/apu/mod.rs`) and is advanced lazily—only when APUIO registers are accessed or at audio sample boundaries.

---

## Directory Layout

```
sres_emulator/src/components/spc700/
├── mod.rs           # Core CPU struct, bus trait, step/reset logic
├── instructions.rs  # All instruction implementations (ALU, shift, branch, etc.)
├── opcode_table.rs  # 256-entry opcode table built by macro
├── operands.rs      # Operand & address-mode decoding/loading/storing
├── status.rs        # PSR (Program Status Register) flags
├── debug.rs         # Debug state, trace formatting, disassembly
├── test.rs          # TomHarte ProcessorTests harness
└── test/
    ├── 0x.json.xz   # Test ROM traces per opcode nibble (0x–fx)
    ├── 1x.json.xz
    ...
    └── fx.json.xz
```

---

## Architecture

### Core Struct

```rust
pub struct Spc700<BusT: Spc700Bus> {
    pub bus: BusT,
    opcode_table: [InstructionDef<BusT>; 256],
    pc: AddressU16,
    a: u8,   // accumulator
    x: u8,   // index X
    y: u8,   // index Y
    sp: u8,  // stack pointer (offset into page 0x01)
    status: Spc700StatusFlags,
}
```

- The CPU is **generic over the bus** (`BusT: Spc700Bus`), allowing both the real `ApuBus` and `TestBus<AddressU16>` to be used.
- Stack lives in page `0x0100..0x01FF`; only the low byte (`sp`) is stored.

### Bus Trait (`Spc700Bus`)

```rust
pub trait Spc700Bus: Bus<AddressU16> {
    fn spc_cycle(&self) -> u64;
    fn master_clock(&self) -> u64;
    fn update_master_clock(&mut self, cycles: u64);
}
```

- The real implementation is `ApuBus` (`src/apu/apu_bus.rs`), which adds RAM, DSP, timers, and APUIO channels.
- `Bus<AddressU16>` provides `cycle_read_u8`, `cycle_write_u8`, `cycle_io`, etc.

### Status Flags (`status.rs`)

`Spc700StatusFlags` is an 8-bit PSR with fields:

| Bit | Flag | Name |
|-----|------|------|
| 7 | `negative` | N |
| 6 | `overflow` | V |
| 5 | `direct_page` | P (direct page bank: 0x00 or 0x01) |
| 4 | `break_command` | B |
| 3 | `half_carry` | H |
| 2 | `irq_enable` | I |
| 1 | `zero` | Z |
| 0 | `carry` | C |

Implements `From<u8>` and `Into<u8>`, plus `Display`/`FromStr` as an 8-char string (`nvpbhiZc`).

### Register Enum (`operands.rs`)

```rust
pub enum Register {
    A, X, Y, YA, Psw, Sp
}
```

`YA` is the 16-bit pair `(Y, A)` used by wide instructions (`ADDW`, `SUBW`, `MOVW`, etc.).

---

## Instruction Set

Instructions are implemented as methods on `Spc700<BusT>` in `instructions.rs`. They are grouped as follows:

### 1. Arithmetic / Logic

| Instruction | Description |
|-------------|-------------|
| `adc`, `sbc` | Add/subtract with carry, half-carry, overflow |
| `and`, `or`, `eor` | Bitwise operations (all use `alu_operation` helper) |
| `cmp` | Compare, sets N/Z/C |
| `inc`, `dec` | Increment/decrement memory or register |
| `mul` | `YA = Y * A` (8 cycles) |
| `div` | `YA / X`, complex overflow behavior (12 cycles) |

### 2. Shifts

| Instruction | Description |
|-------------|-------------|
| `rol`, `ror` | Rotate through carry |
| `asl`, `lsr` | Logical shift left/right |

### 3. 16-Bit (Wide)

| Instruction | Description |
|-------------|-------------|
| `addw`, `subw` | `YA` ± 16-bit memory |
| `cmpw` | Compare `YA` with memory |
| `incw`, `decw` | Increment/decrement 16-bit memory |
| `movw` | Move between `YA` and 16-bit memory |

Wide operations read/write low then high byte separately to match hardware cycle order.

### 4. Branches

All branches take a `Relative` operand and add an extra 2 `cycle_io()` on taken branches.

| Instruction | Condition |
|-------------|-----------|
| `bra` | unconditional |
| `beq`/`bne` | `status.zero` |
| `bpl`/`bmi` | `status.negative` |
| `bvc`/`bvs` | `status.overflow` |
| `bcs`/`bcc` | `status.carry` |
| `bbs`/`bbc` | Bit test on memory |
| `cbne` | Compare-A-then-branch |
| `dbnz` | Decrement-then-branch-if-not-zero |

### 5. Status & Stack

| Instruction | Effect |
|-------------|--------|
| `clrp`/`setp` | Clear/set `direct_page` |
| `clrc`/`setc` | Clear/set `carry` |
| `clrv` | Clear overflow & half-carry |
| `ei`/`di` | Enable/disable IRQ (has extra `cycle_io()`) |
| `push`/`pop` | Stack ops for A, X, Y, PSW |

### 6. Jumps / Calls / Returns

| Instruction | Description |
|-------------|-------------|
| `jmp` | Jump to absolute address |
| `call` | Subroutine call |
| `tcall` | Table call via `0xFFDE` vector table |
| `pcall` | Page-zero call to `0xFF00`+offset |
| `ret` / `reti` | Return / return from interrupt |
| `brk` | Software interrupt |

### 7. Single-Bit & Bit Memory

| Instruction | Description |
|-------------|-------------|
| `set1`/`clr1` | Set/clear bit in direct-page memory |
| `tset1`/`tclr1` | Test-and-set/clear, updates flags on `A - mem` |
| `not1` | Toggle absolute bit (or `carry`) |
| `or1`/`and1`/`eor1` | Bitwise ops on `carry` with absolute memory bit |
| `mov1` | Move bit between carry and absolute memory |

### 8. Misc

| Instruction | Description |
|-------------|-------------|
| `mov` | Move between registers/memory |
| `nop` | No-op |
| `xcn` | Swap nibbles of A |
| `daa` / `das` | Decimal adjust after add/sub |
| `sleep` / `stop` | Low-power states |

---

## Addressing Modes (`operands.rs`)

`AddressMode` enum defines 13 modes. Each mode knows:
- How to **decode** itself (consume program bytes, compute effective address).
- How to display itself in **disassembly**.
- Its **operand size** (0, 1, or 2 bytes).
- Its **wrap mode** (`Wrap::WrapPage` for direct-page, `NoWrap` for absolute).

| Mode | Syntax | Size | Wrap |
|------|--------|------|------|
| `Dp` | `$xx` | 1 | WrapPage |
| `DpXIdx` | `$xx+x` | 1 | WrapPage |
| `DpYIdx` | `$xx+y` | 1 | WrapPage |
| `DpXIdxIndirect` | `[$xx+x]` | 1 | WrapPage |
| `DpIndirectYIdx` | `[$xx]+y` | 1 | WrapPage |
| `XIndirect` | `(x)` | 0 | NoWrap |
| `YIndirect` | `(y)` | 0 | NoWrap |
| `XIndirectAutoInc` | `(x++)` | 0 | NoWrap |
| `Abs` | `$xxxx` | 2 | NoWrap |
| `AbsXIdx` | `$xxxx+x` | 2 | NoWrap |
| `AbsYIdx` | `$xxxx+y` | 2 | NoWrap |
| `AbsXIdxIndirect` | `[$xxxx+x]` | 2 | NoWrap |

`Direct page` is `0x00xx` when `P=0`, else `0x01xx`.

### Operand Encoding

Instructions use two operand enums:
- **`Operand`** — the static definition attached to each opcode entry.
- **`DecodedOperand`** — the runtime result after consuming program bytes.

This split allows:
1. **Decode once** → may load/store multiple times in one instruction.
2. **Disassembly** without mutating state (using `peek_u8`/`peek_u16`).

`DecodedOperand` provides:
- `load(&self, cpu)` → `u8`
- `store(&self, cpu, value)`
- `load_u16` / `store_u16` for wide operations
- `bit()` for single-bit operands

---

## Opcode Table (`opcode_table.rs`)

The table is built at CPU construction time by `build_opcode_table()`, returning `[InstructionDef<BusT>; 256]`.

A `macro_rules!` macro `instruction!` generates entries with three arities:
- **No operands**: `instruction!(nop)`
- **One operand**: `instruction!(asl, InMemory(Dp))`
- **Two operands**: `instruction!(mov, Register(A), InMemory(Dp))`

Each entry contains:
- `execute: fn(&mut Spc700<BusT>)` — the actual instruction body.
- `disassembly: fn(&Spc700<BusT>, AddressU16) -> (InstructionMeta, AddressU16)` — for debug/tracing.

All 256 opcodes are explicitly mapped; there are no "illegal" opcodes. The `execute` closure increments `pc` by one before calling the instruction method so the instruction itself works with the *next* bytes as operands.

---

## Cycle Accuracy

The SPC700 is **cycle-accurate** at the bus-transaction level. Every memory read/write/idle cycle is explicit via:
- `bus.cycle_read_u8(addr)` / `bus.cycle_read_u16(addr, wrap)`
- `bus.cycle_write_u8(addr, value)`
- `bus.cycle_io()` — idle cycle

Instructions in `instructions.rs` manually insert the correct number of `cycle_io()` and `cycle_read_u8` calls to match hardware timing. This is verified against the TomHarte test suite (see Tests below).

### Clocking & Catch-Up

The SPC700 runs slower than the main CPU. `catch_up_to_master_clock(master_cycles)` converts master cycles to SPC cycles:

```rust
const SPC_CLOCK_FREQUENCY: u64 = 32000 * 64;      // 2.048 MHz
const MASTER_CLOCK_FREQUENCY: u64 = 21_477_272;   // ~21.48 MHz
let target = (master_cycles as f64 * ratio).floor() as u64 - 1;
while bus.spc_cycle() < target { cpu.step(); }
```

The APU deliberately does **not** call `catch_up_to_master_clock` on every master clock tick; it only advances the SPC on:
1. **APUIO read/write** — so the SPC sees the correct old/new value.
2. **Audio sample boundaries** — so DSP sample generation is accurate.

---

## Debug & Tracing (`debug.rs`)

### State Snapshot

`Spc700Debug::state()` returns `Spc700State`, containing:
- `instruction: InstructionMeta<AddressU16>` (disassembly)
- Registers `a`, `x`, `y`, `sp`
- `status: String` (8-char format)
- `spc_cycle`, `master_cycle`

### Trace Formats

- **BSNES-style out**: `00000088 [FFC5]  MOV (X),A                         A:00 X:EF Y:00 S:EF P:nvpbhiZc C:54`
- **Mesen parse in**: `FFC5  MOV (X),A [$00EF] = $71          A:00 X:EF Y:00 S:EF P:nvpbhiZc C:54`

`Spc700State::parse_mesen_trace()` is used in APU tests to assert against Mesen boot-ROM traces.

### Debug Events

The CPU emits `Spc700Event::Step(Spc700State)` on every step via a `DebugEventCollectorRef<Spc700Event>`. These events are consumed by the global debugger for log points, breakpoints, and trace capture.

---

## Tests (`test.rs`)

### TomHarte ProcessorTests

The primary correctness validation is the **TomHarte/ProcessorTests** data set. Each JSON line (one per test case) is compressed with `xz` and split into 16 files (`0x.json.xz` through `fx.json.xz`) by high opcode nibble.

Each test file has a dedicated `#[test]` function:
```rust
#[test]
pub fn test_spc700_opcodes_0x() { run_tomharte_test("0x"); }
```

`run_tomharte_test(name)`:
1. Reads the `.json.xz` line-by-line (one object per line for fast parsing).
2. Constructs a `Spc700<TestBus<AddressU16>>` from the `initial` state.
3. Calls `cpu.step()`.
4. Compares **CPU state**, **memory contents**, and **bus cycles** against `final`.
5. On mismatch, prints detailed diffs using `pretty_assertions` and aggregates failures by opcode.

### Skip / Ignore Lists

```rust
const SKIP_OPCODES: &[u8] = &[];
const IGNORE_CYCLE_DETAILS: &[u8] = &[0xCA, 0xD7, 0xFE];
```

- `SKIP_OPCODES`: currently empty; opcodes here would be entirely skipped.
- `IGNORE_CYCLE_DETAILS`: for these opcodes, only the *count* of cycles is checked, not the exact addresses/values, because some edge-case open-bus behaviors differ.

### APU-Level Tests (`src/apu/test.rs`)

In addition to the processor tests, the APU module has integration tests that run Mesen boot-ROM traces through the full `ApuBus`, verifying:
- SPC700 register states after each step.
- APUIO channel behavior.
- Timer and DSP interactions.

---

## Key Patterns for Future Agents

1. **Generic Bus Trait** — `Spc700<BusT>` lets the same CPU core run under both `ApuBus` (production) and `TestBus` (tests) without code duplication.
2. **Macro-Generated Opcode Table** — All 256 opcodes are declared in a single macro block. To add/fix an opcode, edit the `instruction!(...)` call.
3. **Decode-Load-Store Separation** — Addressing modes are decoded first (`operand.decode(cpu)`) yielding a `DecodedOperand`, then loaded/stored. This mirrors hardware cycles and enables disassembly.
4. **Explicit Cycle Calls** — Every memory access or idle cycle is a method call on the bus. This is tedious but necessary for cycle accuracy.
5. **Lazy SPC Catch-Up** — The SPC is not advanced on every master cycle; it is only caught up when needed (APUIO access or sample boundary). This is critical for correct APUIO timing.
6. **Status Flags as Struct** — The PSR is a plain struct with `impl From<u8>` rather than manual bit twiddling throughout instructions.

---

## How It Fits Into the Emulator

```
Main CPU (65816) ──► APUIO read/write ──► Apu::read_apuio / write_apuio
                                               │
                                               ▼
                                    catch_up_to_master_clock()
                                               │
                                               ▼
                                          Spc700::step()
                                               │
                                               ▼
                                          ApuBus (RAM, DSP, Timers)
                                               │
                                               ▼
                                         generate_sample()
```

The `Spc700` is instantiated inside `Apu::new()` with both a `DebugEventCollectorRef` (for debugger integration) and the `ApuBus` (for memory and I/O). The APU module owns the SPC700 and is responsible for advancing it at the right times.
