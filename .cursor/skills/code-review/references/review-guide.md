# Code Review Guide

Check catalog for the `code-review` skill and `code-reviewer` subagent. This file holds only review checks. Architecture, commands, system variants, test taxonomy, and error-handling policy live in [AGENTS.md](../../../../AGENTS.md); per-directory facts live in the nearest subdirectory `AGENTS.md`, which is authoritative for that area.

Map every **Reject if** / **Reject:** row to a blocker. Other misses are nits.

---

## 1. Architecture & Dependencies

### Layer boundaries

Reject PRs that violate this dependency graph:

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

The variant table is in [AGENTS.md](../../../../AGENTS.md#system-variants). Wrong variant choice is a common source of flaky or meaningless tests.

**Reject:**
- Trace tests using `System` instead of `SyncSystem`
- `AsyncSystem` in functional tests or UI without explicit justification
- Assuming batched `peek()` reflects flushed device state without a sync

**Examples:** `sres_emulator/src/lib.rs` (type aliases), `sres_emulator/tests/rom_tests.rs` (`SyncSystem` for traces)

### Frontend execution contract

The UI drives emulation only through the public `System` API (call chain in [AGENTS.md](../../../../AGENTS.md#entry-point-call-chain)).

**Reject:** `cpu.step()` calls or `MainBusImpl` access from `sres_egui`; presenting a frame when `swap_video_frame` returned `false`.

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

Register routing is owned by `MainBusImpl`; the range table is in `sres_emulator/src/main_bus/AGENTS.md`.

In `advance_master_clock`, ordering matters: **DMA → clock tick → PPU/APU `update_clock`**. CPU memory access advances clock inside `cycle_read_u8`/`cycle_write_u8`, then notifies devices.

**Reject:** Components calling `advance_master_clock` themselves; PPU reading `$4212` HVBJOY (that belongs to `Clock`); new register handling added outside `MainBusImpl` routing.

---

## 4. Timing & Performance Patterns

The patterns themselves (master clock, lazy APU catch-up, scanline rendering, zero-cost debug events) are described in [AGENTS.md](../../../../AGENTS.md#key-design-patterns). Checks:

### Master clock

**Reject:** A component incrementing time on its own instead of receiving `ClockInfo`; frame swap anywhere other than vblank rise in `SystemImpl::step`.

### Lazy APU catch-up

Sync points are APUIO read/write (`catch_up_to_master_clock`), audio sample boundaries (`CYCLES_PER_SAMPLE`), and the end of `Apu::update_clock`.

**Reject:** `spc700.step()` in `MainBusImpl::advance_master_clock`; APUIO access without prior catch-up.

### Batched device sync

`execute_until`, vblank rise, and debugger stepping call `ppu.sync()` / `apu.sync()`; `BatchedBusDeviceU24::read()` flushes before returning.

**Reject:** Removing `sync()` calls for performance without benchmark + trace regression evidence.

### Zero-cost debug events

Hot path is `DEBUG_EVENTS_ENABLED.load(Ordering::Relaxed)` guard, then `#[cold]` dispatch.

**Reject:** Unconditional `Mutex::lock()` on every cycle; `println!` in hot bus paths; always-on trace collection.

---

## 5. Error Handling

The policy for unimplemented and unmapped hardware is in [AGENTS.md](../../../../AGENTS.md#error-handling--unimplemented-hardware). Two tiers it does not spell out: known partial features (serial joypad) use `log::warn!`; file/ROM loading returns `anyhow::Result` at the boundary.

**Reject:**
- `panic!` on unmapped register access
- `unimplemented!()` on reachable game code paths
- Silently ignoring unmapped access without debug event
- `Result` in hot emulation paths (`step`, `bus_read`)

**Examples:** `sres_emulator/src/main_bus/mod.rs` (unmapped reads, serial-joypad warning)

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

Per-component structure is in each component's `AGENTS.md`. Invariants to enforce:

### CPU / SPC700

- Opcode table built at construction (not `static`) — closures monomorphize over `BusT`
- Operand panics only for impossible enum variants
- SPC700: explicit bus cycles via `cycle_read_u8` / `cycle_write_u8` / `cycle_io`; no `main_bus` imports

### PPU

- Rendering only in `draw_scanline`, visible area only (`screen_y < 224`)
- Register handlers named `write_<reg>`, `read_<reg>`, `peek_<reg>` after the hardware register
- Generic `TileDecoder` with `PhantomData` — no trait objects in hot path

### S-DSP

- `generate_sample(memory: &[u8])` stays pure over RAM
- New decoder logic gets a golden WAV test in isolation

### APU integration (`apu/`)

- Orchestrates SPC700 + S-DSP; does not duplicate component logic
- Timers stay in `apu/timers.rs`, not a new component

---

## 8. Testing Requirements

### Test taxonomy

The four integration test types are in [AGENTS.md](../../../../AGENTS.md#testing-strategy). Two more: unit tests in `components/*/test.rs` run on `TestBus` against TomHarte JSON traces; fuzz targets in `fuzz/` assert no panic on arbitrary input.

### Asset conventions

| Asset | Naming | On mismatch |
|-------|--------|-------------|
| Trace ROM | `{name}.sfc` + `{name}-trace.log.xz` | Test fails |
| Framebuffer | `{name}-framebuffer.png` | Write `.actual.png`, panic |
| Audio | `{name}.wav` | Write `.actual.wav`, panic |
| Snapshot | `{rom}-{scene}.snapshot` + optional `.writes` + `.png` | Test fails |

Binary assets are committed directly to git (no LFS).

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
- Cross-cutting architecture goes in the root `AGENTS.md`; directory facts in the nearest `AGENTS.md`; nothing duplicated across levels.
- `AGENTS.md` and `//!` edits follow `.cursor/skills/write-agents-docs/SKILL.md`.
- Architecture or test-strategy changes update the affected `AGENTS.md` in the same PR.

**Reject:** Stale `AGENTS.md` after architectural changes; hardware behavior described only in the PR description; a directory `AGENTS.md` restating root content.

---

## 10. Tooling & CI

All PRs pass `./check-all.sh` (commands listed in [AGENTS.md](../../../../AGENTS.md#commands)). CI runs the same script (`fmt`/`clippy`, `test`, `wasm`).

### Formatting (`rustfmt.toml`)

- `imports_granularity = "Item"` — one symbol per `use` line
- `group_imports = "StdExternalCrate"` — std → external → crate

### Clippy allows

Existing `#[allow(clippy::...)]` sites are the full list (`rg 'allow\(clippy'`); each is intentional. Do not add new ones without a comment giving the reason.

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

## Further Reading

[AGENTS.md](../../../../AGENTS.md) for architecture; the nearest subdirectory `AGENTS.md` for directory facts (`find . -name AGENTS.md`); [docs/index.md](../../../../docs/index.md) for hardware reference.
