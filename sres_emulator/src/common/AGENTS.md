# `sres_emulator/src/common` — Summary

This directory contains the foundational traits, types, and utilities shared across all components of the SRES SNES emulator. It establishes core abstractions for memory addressing, bus communication, integer arithmetic, timing, logging, debugging, and image/color handling.

---

## Module Overview

| File | Purpose |
|------|---------|
| `mod.rs` | Re-exports all submodules. |
| `address.rs` | Typed address types (`AddressU24`, `AddressU16`, `AddressU15`) with explicit wrapping semantics. |
| `bus.rs` | Generic `Bus` trait and `BusDeviceU24` for memory-mapped I/O. |
| `clock.rs` | SNES master-clock timing and scanline/frame calculations (`ClockInfo`). |
| `uint.rs` | Generic unsigned-integer traits (`UInt`, `UIntTruncate`), variable-length integers, and bit-manipulation extensions. |
| `util.rs` | General utilities: `RingBuffer`, `EdgeDetector`, memory hex-formatting. |
| `logging.rs` | Custom logger (`SresLogger`) with trace-log ring buffer and colored output. |
| `debug_events.rs` | Debugger event-collection traits and reference wrapper. |
| `test_util.rs` | WAV golden-file comparison helpers for audio tests. |
| `test_bus.rs` | Test-only `Bus` implementation with cycle recording and sparse memory. |
| `image.rs` | SNES color types (`Rgb15`, `Rgba32`, `ColorIdx`) and a generic `Image` trait. |

---

## Key Types, Traits, and Patterns

### 1. Addressing (`address.rs`)

**Core idea:** Addresses are strongly typed, and every arithmetic operation must explicitly specify how overflow wraps.

- **`Address` trait** — Implemented by all address types. Requires `Eq + Hash + Display + Ord + Copy + Clone + From<u32> + Into<u32>`. Provides:
  - `add_signed(&self, rhs: i32, wrap: Wrap) -> Self`
  - `add<T: UIntTruncate>(&self, rhs: T, wrap: Wrap) -> Self`
  - `sub<T: UIntTruncate>(&self, rhs: T, wrap: Wrap) -> Self`
  - `add_detect_page_cross<T: UIntTruncate + Copy>(&self, rhs: T, wrap: Wrap) -> (bool, Self)` — returns `(page_crossed, result)`.

- **`Wrap` enum** — `WrapPage`, `WrapBank`, `NoWrap`. Determines whether addition/subtraction wraps at the page boundary (256 bytes), bank boundary (64 KiB), or not at all.

- **`AddressU24`** — 24-bit address used by the main SNES bus. Fields: `bank: u8`, `offset: u16`. Supports all three `Wrap` modes.
- **`AddressU16`** — 16-bit address used by the SPC700 audio coprocessor. Supports `WrapPage` and `NoWrap`; `WrapBank` is `unimplemented!()`.
- **`AddressU15`** — 15-bit address (used by APU RAM). Wraps at `0x7FFF`. Implements `Add`/`Sub` for `u16`/`u32`.

- **`InstructionMeta<AddressT>`** — Metadata for disassembly: address, operation mnemonic, operand string, and optional effective address.

**Pattern:** All address arithmetic goes through the `Address` trait methods with an explicit `Wrap` argument. This prevents accidental cross-page or cross-bank bugs.

---

### 2. Bus Abstraction (`bus.rs`)

**Core idea:** A generic bus interface parameterized by address type, with default implementations for multi-byte reads/writes.

- **`Bus<AddressT: Address>`** — Main trait for any bus (main CPU bus, SPC700 bus, etc.). Required methods:
  - `peek_u8(&self, addr) -> Option<u8>` — non-mutating read.
  - `cycle_read_u8(&mut self, addr) -> u8` — mutating read (may advance clock).
  - `cycle_write_u8(&mut self, addr, value)` — mutating write.
  - `cycle_io(&mut self)` — internal operation cycle.
  - `reset(&mut self)`.

  Provided methods:
  - `cycle_read_u16`, `cycle_write_u16`, `peek_u16` — little-endian, parameterized by `Wrap`.
  - `cycle_read_generic<T: UInt>`, `cycle_write_generic<T: UInt>` — dispatch on `UIntSize`.
  - `peek_range` — read a contiguous `RangeInclusive<u32>` into a `Vec<u8>`.

- **`BusDeviceU24`** — Trait for devices mapped onto the main U24 bus (e.g., PPU, APU, cartridge). Methods: `peek`, `read`, `write`, `update_clock`, `reset`. Each device has an associated `const NAME: &'static str`.

**Pattern:** The `Bus` trait is generic over `AddressT`, allowing the same interface to be reused for the 16-bit SPC700 bus and the 24-bit main bus. Multi-byte operations are built on top of single-byte primitives with explicit `Wrap` behavior.

---

### 3. Clock & Timing (`clock.rs`)

**Core idea:** All timing is derived from a single `master_clock` counter. The module converts between master-clock cycles and SNES video timing (scanlines, dots, frames).

- **`ClockInfo`** — `#[derive(Encode, Decode)]` (bitcode-serializable). Fields:
  - `master_clock: u64`
  - `v: u64` — vertical scanline counter
  - `h_counter: u64` — horizontal counter (cycles within scanline)
  - `f: u64` — frame number

  Key methods:
  - `from_master_clock(master_clock: u64) -> Self` — converts master clock to `(v, h_counter, f)`, accounting for the short scanline on even frames.
  - `from_mesen_vhf(v, h_counter, f) -> Self` — reverse conversion from Mesen emulator trace coordinates (handles Mesen's frame-numbering quirk where frame increments at vblank start).
  - `hdot(&self) -> u64` — converts `h_counter` to dot clock (accounts for 6-cycle dots 323/327 on non-short scanlines).
  - `vblank(&self) -> bool` — true when `v >= 225`.

**Pattern:** Timing is centralized here. Other components receive `ClockInfo` updates rather than tracking their own counters.

---

### 4. Integer Abstractions (`uint.rs`)

**Core idea:** Abstract over `u8` and `u16` so CPU instructions can operate generically depending on the processor's accumulator/index register size.

- **`UIntSize` enum** — `U8`, `U16`.
- **`VariableLengthUInt` enum** — `U8(u8)` | `U16(u16)`. Used for values whose width depends on processor flags.
- **`UIntTruncate` trait** — Conversion methods: `to_u32`, `to_u16`, `to_u8`, `from_u32`, `from_u16`, `from_u8`. Implemented for `u8`, `u16`, `u32`.
- **`UInt` trait** — Super-trait combining `PrimInt`, overflow/wrapping arithmetic, bit ops, `Shl`, `UpperHex`, and `UIntTruncate`. Constants: `N_BITS`, `N_BYTES`, `SIZE`. Methods:
  - `bit(&self, index) -> bool`, `set_bit(&mut self, index, value)`
  - `msb(&self) -> bool`, `lsb(&self) -> bool`
  - `add_bcd(&self, rhs, carry) -> (Self, overflow, carry)` — BCD addition.
  - `sub_bcd(&self, rhs, carry) -> (Self, overflow, carry)` — BCD subtraction.

  Implemented for `u8` and `u16` with full BCD support.

- **Extension traits:**
  - `U32Ext` — `low_word()`, `high_word()`.
  - `U16Ext` — `low_byte()`, `high_byte()`, `set_low_byte()`, `set_high_byte()`, `with_low_byte()`, `with_high_byte()`.
  - `U8Ext` — `low_nibble()`, `high_nibble()`, `set_low_nibble()`, `set_high_nibble()`, `with_low_nibble()`, `with_high_nibble()`.

**Pattern:** CPU instruction implementations are generic over `T: UInt`, allowing a single implementation to handle both 8-bit and 16-bit modes. BCD arithmetic is self-contained here.

---

### 5. General Utilities (`util.rs`)

- **`RingBuffer<T, const N: usize>`** — Fixed-size ring buffer backed by `VecDeque`. Methods: `push`, `pop`, `iter`, `len`, `is_empty`, `drain`. Oldest elements are dropped when capacity is exceeded.
- **`EdgeDetector`** — Detects rising and falling edges of a boolean signal. Methods: `update_signal(value)`, `consume_rise() -> bool`, `consume_fall() -> bool`. Used for vblank transitions, timer triggers, etc.
- **`format_memory(memory: &[u8]) -> String`** — Hex-dumps bytes in 16-byte rows.
- **`format_memory_u16(memory: &[u16]) -> String`** — Hex-dumps words in 16-word rows.

---

### 6. Logging (`logging.rs`)

- **`SresLogger`** — Custom `log::Log` implementation wrapping `env_logger` for filtering.
  - **Trace log ring buffer:** When `trace_as_context_only` is enabled, `Trace`-level logs are stored in a ring buffer (default 20 lines). When a higher-severity log is emitted, the buffered trace lines are printed first as context, prefixed with `T`.
  - **Colored output:** Error (red `E`), Warn (yellow `W`), Info (blue `I`), Debug (blue `D`), Trace (dimmed).
- **`init()`** — Production logger init. Reads `SRES_LOG` env var; defaults to `error`.
- **`test_init(verbose: bool)`** — Test logger init. Defaults to `warn,cpu_step=info` (or includes `spc700_step=info` when verbose). In non-verbose mode, trace logs are treated as context-only.

**Pattern:** Use `log::trace!` liberally in hot paths; the ring buffer ensures performance is not impacted unless a warning/error occurs.

---

### 7. Debug Events (`debug_events.rs`)

- **`DebugErrorCollector`** — `on_error(message: String)`.
- **`DebugEventCollector<EventT>`** — Extends `DebugErrorCollector` with `on_event(event: EventT)`.
- **`DebugEventCollectorRef<EventT>`** — `Arc<Mutex<dyn DebugEventCollector<EventT> + Send>>` wrapper. Provides `on_event` and `on_error` methods that are no-ops unless `DEBUG_EVENTS_ENABLED` (global `AtomicBool`) is true. The actual dispatch is marked `#[cold]` to keep the fast path inline.
- **`mock_collector<EventT>()`** — Test helper returning a no-op collector.

**Pattern:** Emulator components hold a `DebugEventCollectorRef` and call `on_event`/`on_error` at key points. The global atomic flag ensures near-zero overhead when debugging is disabled.

---

### 8. Test Utilities (`test_util.rs`, `test_bus.rs`)

- **`test_util.rs`**:
  - `compare_wav_against_golden(data: &[i16], path_prefix: &Path)` — Compares audio output against a golden `.wav` file; writes `.actual.wav` on mismatch. Creates golden if missing.
  - `write_snes_wav(data, path)` / `read_snes_wav(path)` — 32 kHz, 16-bit mono WAV I/O.

- **`test_bus.rs`** (compiled only under `#[cfg(test)]`):
  - **`TestBus<AddressT>`** — Implements `Bus<AddressT>`. Uses `SparseMemory` and records every cycle in `Vec<Cycle<AddressT>>`.
  - **`Cycle<AddressT>`** — Enum: `Read(addr, Option<u8>)`, `Write(addr, u8)`, `Internal`. Custom `Debug` formatting: `R($addr)=XX`, `W($addr)=VV`, `I`.
  - **`SparseMemory<AddressT>`** — `HashMap<AddressT, u8>` backing. Implements `Display` for sorted hex dump.

**Pattern:** Unit tests for CPU or bus devices use `TestBus` to verify exact cycle sequences and memory side effects.

---

### 9. Image & Color (`image.rs`)

- **`ColorIdx(pub u8)`** — Palette index.
- **`Rgb15(pub u16)`** — SNES native color: 5 bits per R/G/B channel (bits 0–4, 5–9, 10–14). Methods: `r()`, `g()`, `b()`, `set_r()`, `set_g()`, `set_b()`. Implements `Add<(i16, i16, i16)>` (saturating per-channel) and `Div<u16>` (per-channel division).
- **`Rgba32(pub [u8; 4])`** — Modern 32-bit RGBA. Bidirectional `From` conversions with `Rgb15` using a `U5_TO_U8_CONVERSION` factor (`8.225806`).
- **`Image` trait** — Abstract interface for image buffers: `new(width, height) -> Self`, `set_pixel(index: (u32, u32), value: Rgba32)`. Implemented externally for `image::RgbaImage` (tests) and `egui::ColorImage` (GUI).

**Pattern:** The PPU renders into `Rgb15` (SNES format), which is converted to `Rgba32` only at presentation time. The `Image` trait decouples the emulator core from specific image libraries.

---

## Conventions & Design Patterns

1. **Explicit Wrapping** — Address arithmetic never uses raw `+`/`-`; it always goes through `Address::add`/`sub` with an explicit `Wrap` parameter.
2. **Generic Bus** — The `Bus` trait is parameterized by address width, enabling reuse across the 65816 (U24) and SPC700 (U16) buses.
3. **Zero-Cost Debugging** — Debug events are gated by a global `AtomicBool`; the dispatch path is `#[cold]` so the compiler optimizes the no-op case.
4. **Trace-as-Context Logging** — Trace logs are buffered and only emitted when a higher-severity message occurs, keeping console output manageable during emulation.
5. **Bitcode Serialization** — `ClockInfo`, `EdgeDetector`, and `Rgb15` derive `Encode`/`Decode` for save-state serialization.
6. **Sparse Memory in Tests** — `TestBus` uses a `HashMap` rather than a flat array, making tests self-documenting and avoiding large zeroed allocations.
7. **UInt Generics** — CPU instruction implementations are generic over `T: UInt`, eliminating duplication between 8-bit and 16-bit modes.
8. **Centralized Timing** — All components receive `ClockInfo` updates; they do not maintain independent frame/scanline counters.

---

## Dependencies Used

- `bitcode` — Save-state serialization (`Encode`/`Decode`).
- `intbits` — Bit-field access (`Bits::bit`, `Bits::set_bit`, `with_bits`).
- `num_traits` — `PrimInt`, `OverflowingAdd`, `OverflowingSub`, `WrappingAdd`, `WrappingSub`.
- `colored` — Terminal color output in the logger.
- `env_logger` / `log` — Logging framework.
- `hound` — WAV file I/O for audio tests.
- `itertools` — Sorted iteration for `SparseMemory` display.
