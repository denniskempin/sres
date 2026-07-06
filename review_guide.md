# Code Review Guide

Patterns and conventions a code owner should enforce when reviewing SRES pull requests. This guide complements [AGENTS.md](AGENTS.md) with review-focused checklists. When in doubt, the nearest subdirectory `AGENTS.md` is authoritative for that area.

---

## 1. Architecture & Dependencies

### Layer boundaries

SRES is organized in strict layers. Reject PRs that violate this dependency graph:

```
sres_egui ──► sres_emulator (public API only)
lib.rs ──► apu, main_bus, components, common, debugger, controller
main_bus ──► components, common  (+ generic PpuT/ApuT wired in lib.rs)
apu ──► components/{spc700, s_dsp}, common
components/* ──► common ONLY
common ──► (no emulator layers)
```

| Check | Reject if |
|-------|-----------|
| Component imports | Any `use crate::components::*` from inside another component |
| Main bus imports | `use crate::apu::Apu` in `main_bus/` |
| Frontend imports | `sres_egui` reaching into `main_bus`, `components`, or calling `cpu.step()` directly |
| Integration placement | Memory-map decode, DMA side effects, or clock advance inside a component |

**Why:** Components are independently testable. Cross-deps create compile-time coupling and make trace/golden tests harder to isolate.

**Examples:** `sres_emulator/src/components/mod.rs`, `sres_emulator/src/lib.rs`, `sres_egui/src/app.rs`

### Component isolation

- Inner modules (`instructions`, `vram`, `cgram`, etc.) must be private (`mod`, not `pub mod`).
- Re-export only facade types and debug/state types from `mod.rs` (e.g. `CpuDebug`, `PpuState`).
- Component-specific bus traits (`MainBus`, `Spc700Bus`) are defined in the component; implemented by integration layers.
- Wiring (who owns whom, address routing) lives in `main_bus/` or `apu/`, never inside components.

### Public API surface

- `lib.rs` is the orchestration point; frontend uses `System`, `execute_frames`, `swap_video_frame`, `swap_audio_buffer`, `update_joypads`.
- Do not widen `pub mod` visibility to avoid writing an accessor.
- `main_bus`: `devices` is public; `dma`, `multiplication` stay private.

---

## 2. System Variants

Choose the correct `SystemImpl` wrapper. This is a common source of flaky or meaningless tests.

| Variant | Alias | PPU/APU timing | Use when |
|---------|-------|----------------|----------|
| `BatchedSystem` | `System` | Buffered; flushed at sync points | Default: UI, golden-image/WAV, ROM-outcome tests |
| `SyncSystem` | — | Cycle-accurate every CPU step | BSNES trace comparison, register-boundary timing bugs |
| `AsyncSystem` | — | APU on background thread | Benchmarks / perf exploration only |

**Reject:**
- Trace tests using `System` instead of `SyncSystem`
- `AsyncSystem` in functional tests or UI without explicit justification
- Assuming batched `peek()` reflects flushed device state without a sync

**Examples:** `sres_emulator/src/lib.rs` (type aliases), `sres_emulator/tests/rom_tests.rs` (`SyncSystem` for traces)

### Frontend execution contract

The UI must drive emulation through the public `System` API:

1. `execute_frames` / `execute_for_audio_samples` — advance emulation
2. `swap_video_frame` — present only when it returns `true` (vblank rise)
3. `swap_audio_buffer` — exchange audio buffer
4. `update_joypads` — input

Do not call `cpu.step()` or reach into `MainBusImpl` from the frontend.

---

## 3. Bus Abstractions & Integration

### Two-level bus model

| Trait | Role | Used by |
|-------|------|---------|
| `Bus<AddressT>` | Cycle-accurate CPU memory: `cycle_read_u8`, `cycle_write_u8`, `cycle_io`, `peek_u8` | CPU, SPC700 (via `ApuBus`) |
| `BusDeviceU24` | Memory-mapped device: `read`, `write`, `peek`, `update_clock`, `reset` | PPU, APU |
| `MainBus` | CPU extension: NMI/IRQ consume, `clock_info` | `MainBusImpl` |
| `Spc700Bus` | APU extension: `spc_cycle`, `master_clock`, `update_master_clock` | `ApuBus` |
| `ManagedBusDeviceU24` | Sync/batch/async wrapper with `inner()`, `sync()` | Device wrappers in `main_bus/devices.rs` |

**Reject:** Hard-coding `MainBusImpl` inside CPU instruction bodies; using raw `Bus` cycle methods inside `BusDeviceU24` implementations.

### Peek vs read

- `peek` — non-mutating inspection (debugger, disassembly, test memory checks)
- `read` — may latch, clear flags, or advance state

Every mutating `read_*` should have a matching `peek_*`. Tests inspecting RAM after halt use `peek_range`, not destructive reads.

### Main bus routing & clock ordering

Register routing is owned by `MainBusImpl`:

| Range | Handler |
|-------|---------|
| `$2100–$213F` | PPU |
| `$2140–$217F` | APU |
| `$4200`, `$4207–$420A`, `$4210–$4212` | Clock |
| `$4202–$4206` write / `$4214–$4217` read | Multiplication unit |
| `$420B`, `$420C`, `$4300–$43FF` | DMA controller |
| `$4218–$421B` | Joypads |

In `advance_master_clock`, ordering matters: **DMA → clock tick → PPU/APU `update_clock`**. CPU memory access advances clock inside `cycle_read_u8`/`cycle_write_u8`, then notifies devices.

**Reject:** Components calling `advance_master_clock` themselves; PPU reading `$4212` HVBJOY (that belongs to `Clock`).

---

## 4. Timing & Performance Patterns

### Master clock

- `Clock` in `components/clock.rs` is the single timing source (`master_clock`, scanline `v`, frame `f`).
- Components receive `ClockInfo`; they do not increment global time independently.
- PPU renders one scanline at a time on scanline change; frame swap happens at vblank rise in `SystemImpl::step`.

### Lazy APU catch-up

SPC700 advances only at sync points — **not every CPU cycle**:

- APUIO read/write (via `catch_up_to_master_clock`)
- Audio sample boundaries (`CYCLES_PER_SAMPLE` ≈ 671 master cycles)
- End of `Apu::update_clock`

**Reject:** `spc700.step()` in `MainBusImpl::advance_master_clock`; APUIO access without prior catch-up.

### Batched device sync

- `BatchedBusDeviceU24::read()` flushes before returning.
- `execute_until`, vblank rise, and debugger stepping call `ppu.sync()` / `apu.sync()`.
- Writes are buffered; reads force flush.

**Reject:** Removing `sync()` calls for performance without benchmark + trace regression evidence.

### Zero-cost debug events

- Components hold `DebugEventCollectorRef<T>`; emit via `on_event` / `on_error`.
- Hot path: `DEBUG_EVENTS_ENABLED.load(Ordering::Relaxed)` guard, then `#[cold]` dispatch.
- Debugger off by default.

**Reject:** Unconditional `Mutex::lock()` on every cycle; `println!` in hot bus paths; always-on trace collection.

---

## 5. Error Handling

Three-tier model — this is non-negotiable:

| Situation | Behavior |
|-----------|----------|
| Unimplemented/unmapped hardware | Return `0` on read; silently ignore write; emit `on_error` |
| Known partial features (HDMA, serial joypad) | `log::warn!` |
| Internal logic bugs (impossible operand, invariant violation) | `panic!` / `unreachable!` acceptable |
| Fuzz targets / arbitrary input | Must never panic |
| File/ROM loading | `anyhow::Result` at boundary |

**Reject:**
- `panic!` on unmapped register access
- `unimplemented!()` on reachable game code paths
- Silently ignoring unmapped access without debug event
- `Result` in hot emulation paths (`step`, `bus_read`)

**Examples:** `sres_emulator/src/main_bus/mod.rs` (unmapped reads), `sres_emulator/src/main_bus/dma.rs` (HDMA warning)

### Panic vs warn decision table

| Situation | Expected |
|-----------|----------|
| Game reads unmapped `$42FF` | Return `0`, `on_error` |
| Unsupported BG mode in render path | `panic!` (not yet supported — internal invariant) |
| Wrong operand enum in CPU | `panic!` (programmer error) |
| STAT77/STAT78 stub | `log::warn!`, return stub value |

---

## 6. Type & Style Conventions

### Typed addresses

Use `AddressU24`, `AddressU16`, `AddressU15` — not bare `u32`/`u16`. All arithmetic via `.add(rhs, wrap)` / `.sub(rhs, wrap)` with explicit `Wrap::{WrapPage, WrapBank, NoWrap}`.

**Reject:** `(addr + 1) as u16` for PC advancement; mixing bank/offset without helpers.

### Generic 8/16-bit dispatch (`UInt`)

- ALU/load/store instructions generic over `T: UInt`; dispatch via M/X status flags.
- Bus access via `cycle_read_generic` / `cycle_write_generic`.
- CPU registers that depend on M/X use `VariableLengthRegister`.

**Reject:** Duplicated `lda_u8` / `lda_u16`; raw `as u8`/`as u16` at instruction sites.

### Color types

- Internal: `Rgb15` (PPU framebuffer, CGRAM)
- Presentation: `Rgba32` via `Framebuffer::to_rgba()` at UI/test boundary only

**Reject:** Storing `Rgba32` in PPU scanline renderer.

### Bit fields

- Cartridge header / DMA / controller: `packed_struct` with `bit_numbering = "msb0"`
- S-DSP registers: `bilge`
- Ad-hoc register bytes: `intbits::Bits`

### Save states

- Persistent hardware state derives `bitcode::Encode` / `Decode`.
- Transient fields (debug collectors, opcode tables, headless flags) stay outside serializable structs (facade pattern: `Ppu` vs `PpuState`).
- Use `bitcode`, not `serde`.

**Reject:** Serializing batched write caches or debug-only `Arc<Mutex<...>>` types.

### `unsafe` code

The codebase currently has zero `unsafe` blocks. New `unsafe` requires strong justification, isolation in `common/`, safety comments, and tests.

---

## 7. Component-Specific Checks

### CPU / SPC700

- Expected file layout: `mod.rs`, `status.rs`, `operands.rs`, `opcode_table.rs`, `instructions.rs`, `debug.rs`, `test.rs`
- Opcode table built at construction (not `static`) — closures monomorphize over `BusT`
- Operand panics only for impossible enum variants
- SPC700: explicit bus cycles via `cycle_read_u8` / `cycle_write_u8` / `cycle_io`; no `main_bus` imports

### PPU

- Scanline renderer: `draw_scanline` on scanline change; visible area only (`screen_y < 224`)
- Register handlers: `write_<REG>`, `read_<reg>`, `peek_<reg>` using hardware names
- Generic `TileDecoder` with `PhantomData` — no trait objects in hot path
- Submodules: `vram.rs`, `cgram.rs`, `oam.rs`, `debug.rs`

### S-DSP

- `generate_sample(memory: &[u8])` pure over RAM
- Voice logic in `voice.rs`, BRR in `brr.rs`
- Golden WAV tests for decoder isolation

### APU integration (`apu/`)

- Orchestrates SPC700 + S-DSP; does not duplicate component logic
- `ApuBus` implements `Spc700Bus` + `Bus<AddressU16>`
- Timers in `apu/timers.rs`, not a new component

---

## 8. Testing Requirements

### Test taxonomy

| Test type | Location | System | Assertion |
|-----------|----------|--------|-----------|
| Trace-comparison | `tests/rom_tests/` | `SyncSystem` | `CpuState`/`Spc700State` string vs BSNES |
| ROM-outcome | `tests/rom_tests/` | `System` | Memory at `stp` via `peek_range` |
| Golden-image | `tests/ppu_tests/` | `System` | PNG byte match |
| Golden-WAV | `tests/apu_tests/` | `System` | WAV byte match |
| Unit (CPU/SPC) | `components/*/test.rs` | `TestBus` / mock collector | JSON traces, cycles |
| Fuzz | `fuzz/` | — | No panic on arbitrary input |

### Asset conventions

| Asset | Naming | LFS | On mismatch |
|-------|--------|-----|-------------|
| Trace ROM | `{name}.sfc` + `{name}-trace.log.xz` | Yes | Test fails |
| Framebuffer | `{name}-framebuffer.png` | Yes | Write `.actual.png`, panic |
| Audio | `{name}.wav` | Yes | Write `.actual.wav`, panic |
| Snapshot | `{rom}-{scene}.snapshot` + `.png` | — | Test fails |

**Reject:**
- Committing `.actual.png`/`.actual.wav` as goldens
- Commercial ROM binaries in git (use snapshots + `.gitignore`)
- Auto-created goldens committed without visual/audio verification
- Trace tests on `BatchedSystem`

### Test ROM assembly

- Shared includes via `tests/asm_lib/` (symlinked as `tests/lib/`, `*/lib/`)
- Pre-assembled `.sfc` committed; keep `.asm` and `.sfc` in sync
- ROM-outcome tests must end with `stp`
- Trace pipeline: BSNES log → `process.py` → `{name}-trace.log.xz`

### Documenting known divergences

When hardware is unimplemented or behavior differs from reference:

- Use `#[ignore = "reason"]` for blocked tests
- Inline comments at test site explaining why test still passes
- Normalization in compare helpers (e.g. clearing `effective_addr` for open-bus divergence) with `TODO`
- Skip lists (`SKIP_OPCODES`, `IGNORE_CYCLE_DETAILS`) with comments explaining the gap

**Reject:** Silently weakening assertions; `#[ignore]` without reason string.

### Fuzz targets

| Target | Validates |
|--------|-----------|
| `program` | `Cpu::step()` × 1000 never panics on arbitrary bytes |
| `sfc` | `Cartridge::load_sfc_data()` never panics |

Do not commit `corpus/`, `artifacts/`, `coverage/`.

### Benchmarks

- Registered in `Cargo.toml` as `[[bench]]` with `harness = false`
- Compare `SyncSystem` / `BatchedSystem` / `AsyncSystem` variants — do not use as regression tests
- ROM paths point at existing test ROMs, not duplicated binaries

---

## 9. Documentation & Comments

- Module files start with `//!` describing purpose (1–4 lines).
- Hardware registers documented with bit diagrams where non-obvious (see DMA in `main_bus/dma.rs`).
- Non-obvious timing/quirks get a short comment — do not restate the code.
- Cross-cutting architecture goes in nearest `AGENTS.md`, not duplicated in every file.
- Update `AGENTS.md` when changing architecture or test strategy.

**Reject:** Stale AGENTS.md after architectural changes; hardware behavior described only in PR description.

---

## 10. Tooling & CI

All PRs should pass `./check-all.sh` (or CI equivalent):

| Check | Command |
|-------|---------|
| Tests | `cargo nextest run --workspace` |
| WASM | `cd sres_egui && trunk build` |
| Clippy | `cargo clippy --workspace --all-targets` |
| Format | `cargo fmt --check` |

### Formatting (`rustfmt.toml`)

- Nightly Rust required (`rust-toolchain.toml`)
- `imports_granularity = "Item"` — one symbol per `use` line
- `group_imports = "StdExternalCrate"` — std → external → crate

### Clippy allows (existing — do not expand without reason)

| Lint | Where | Why |
|------|-------|-----|
| `new_without_default` | `System::new`, `Apu::new`, PPU | Constructors need injected deps |
| `single_match` | `main_bus`, `s_dsp`, `test_bus` | Intentional partial match |
| `enum_variant_names` | debugger | Event enum naming |
| `suspicious_arithmetic_impl` | address types | Intentional wrapping |

**Reject:** `#![allow(clippy::all)]` on modules; stable-only APIs; disabling CI checks instead of fixing root cause.

---

## 11. PR Review Checklist

Use this as a quick gate before approving:

### Architecture
- [ ] No new cross-component dependencies
- [ ] Integration logic in `main_bus/` or `apu/`, not components
- [ ] Correct system variant for any new/changed tests
- [ ] Public API changes are intentional and minimal

### Correctness
- [ ] Unimplemented hardware returns safe defaults + debug events (no panics)
- [ ] Lazy APU catch-up preserved at sync points
- [ ] Clock/DMA ordering unchanged or explicitly justified
- [ ] Peek/read split maintained for new registers

### Types & style
- [ ] Typed addresses with explicit `Wrap`
- [ ] `UInt` generic dispatch for new CPU instructions
- [ ] `Rgb15` internally, `Rgba32` at presentation boundary
- [ ] `cargo fmt --check` and `cargo clippy` pass

### Tests
- [ ] Right test type and system variant
- [ ] ROM + golden/trace assets paired with matching basenames
- [ ] Goldens verified before commit (not blind auto-create)
- [ ] Known divergences documented
- [ ] Fuzz-safe if touching arbitrary-input paths

### Documentation
- [ ] Relevant `AGENTS.md` updated if architecture changed
- [ ] Register comments for new hardware

---

## Quick Reference

| Situation | Mechanism |
|-----------|-----------|
| Unmapped SNES register | Return `0` / ignore write + `on_error` |
| Known missing feature (HDMA) | `log::warn!` |
| Wrong internal enum/operand | `panic!` / `unreachable!` |
| Load `.sfc` file | `anyhow::Result` |
| CPU 8 vs 16 bit | `T: UInt` + M/X dispatch |
| Address increment | `addr.add(n, Wrap::WrapBank)` |
| Save state field | `#[derive(Encode, Decode)]` on inner state struct |
| Default emulation | `BatchedSystem` (`System`) |
| BSNES trace test | `SyncSystem` |
| Present frame | `swap_video_frame()` when `true` |
| Inspect test memory | `peek_range`, not `cycle_read` |

---

## Further Reading

| Topic | Document |
|-------|----------|
| Architecture overview | [AGENTS.md](AGENTS.md) |
| System orchestration | [sres_emulator/src/AGENTS.md](sres_emulator/src/AGENTS.md) |
| Component rules | [sres_emulator/src/components/AGENTS.md](sres_emulator/src/components/AGENTS.md) |
| Bus & DMA | [sres_emulator/src/main_bus/AGENTS.md](sres_emulator/src/main_bus/AGENTS.md) |
| APU integration | [sres_emulator/src/apu/AGENTS.md](sres_emulator/src/apu/AGENTS.md) |
| Shared types | [sres_emulator/src/common/AGENTS.md](sres_emulator/src/common/AGENTS.md) |
| Test strategy | [sres_emulator/tests/AGENTS.md](sres_emulator/tests/AGENTS.md) |
| Hardware reference | [docs/index.md](docs/index.md) |
