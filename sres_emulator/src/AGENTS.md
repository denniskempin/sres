# `sres_emulator/src` — Summary

This is the root source directory of the `sres_emulator` library crate. It contains the high-level system orchestration, debugger integration, controller definitions, and module declarations for all emulator subsystems.

---

## Top-Level Files

| File | Purpose |
|------|---------|
| `lib.rs` | **Core system abstraction.** Defines `SystemImpl`, the three system variants (`BatchedSystem`, `SyncSystem`, `AsyncSystem`), execution control, and frame/audio swapping. |
| `controller.rs` | SNES controller input format (`StandardController` bitfield). |
| `debugger.rs` | Interactive debugger: breakpoints, log points, trace buffers, event filtering. |

---

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `common/` | Shared types, traits, and utilities (addresses, bus interfaces, clock, logging, test doubles). |
| `components/` | Independent reusable emulator components (CPU, PPU, S-DSP, SPC700, cartridge, clock). |
| `apu/` | Audio Processing Unit integration: orchestrates SPC700 + S-DSP, timers, APUIO bridge. |
| `main_bus/` | Central SNES system bus: memory mapping, DMA, HDMA, device wrappers, interrupt delegation. |

---

## `lib.rs` — System Orchestration

`lib.rs` is the public API surface of the emulator. It wires the 65816 CPU, PPU, and APU together through the main bus and provides execution control.

### System Types

Three type aliases provide different PPU/APU update strategies:

| Type | Description |
|------|-------------|
| `BatchedSystem` (default) | Batches PPU/APU updates for performance. Used by the UI. |
| `SyncSystem` | Updates PPU/APU on every bus cycle. Used for trace-comparison tests. |
| `AsyncSystem` | Runs APU on a separate thread. |

All three are `SystemImpl<PpuT, ApuT>` with different `ManagedBusDeviceU24` wrappers.

### Key Types

- **`SystemImpl<PpuT, ApuT>`** — Owns the `Cpu<MainBusImpl<PpuT, ApuT>>`, debugger, vblank detector, and a pending video frame buffer.
- **`TraceStepIter`** — Iterator yielding `TraceStep::Cpu(CpuState)` or `TraceStep::Spc700(Spc700State)` from the debugger log. Advances emulation when the log is empty.
- **`SystemDebug<'a, PpuT, ApuT>`** — Read-only debug view exposing `PpuDebug` and `ApuDebug`.
- **`ExecutionResult`** — `Normal`, `Halt`, or `Break(BreakReason)`.

### Execution Control

All methods return `ExecutionResult`:

| Method | Breaks when... |
|--------|----------------|
| `execute_one_instruction()` | After one CPU instruction |
| `execute_until_halt()` | CPU halts (`stp`) |
| `execute_frames(n)` | Frame counter reaches target |
| `execute_scanlines(n)` | Scanline counter reaches target |
| `execute_cycles(n)` | Master clock reaches target |
| `execute_for_audio_samples(n)` | APU sample buffer has `n` new samples |
| `execute_for_duration(s)` | Time elapsed (in seconds) |
| `debug_until(event_filter)` | Debugger hits the specified event |

### Frame & Audio Output

- **`swap_video_frame(&mut Framebuffer)`** — Returns `true` if a new frame is available (swapped on vblank rise).
- **`swap_audio_buffer(&mut AudioBuffer)`** — Exchanges the APU's internal sample buffer.
- **`update_joypads(joy1, joy2)`** — Writes controller state to the main bus.
- **`force_headless()`** — Disables PPU rendering for benchmark/headless modes.

### Vblank Timing

The `step()` method:
1. Runs one CPU step.
2. If debugger is enabled, syncs PPU and APU.
3. Detects vblank rise; on rise, syncs PPU/APU and swaps the framebuffer.

---

## `controller.rs` — Controller Input

- **`StandardController`** — 16-bit packed struct using `packed_struct`.
  - Buttons: B, Y, Select, Start, Up, Down, Left, Right, A, X, L, R
  - 4 signature bits (`sig0..sig3`)
  - `to_u16()` packs into the SNES register format ($4218/$4219).

---

## `debugger.rs` — Debugger & Event System

The debugger is a central `Arc<Mutex<Debugger>>` shared across all components. Components emit typed events; the debugger filters and logs them.

### Key Types

- **`Debugger`** — Holds log points, break points, a ring buffer of `DebugEvent` (capacity 16,384), and an optional `BreakReason`.
- **`EventFilter`** — Break/log filter conditions:
  - `CpuStep`, `CpuProgramCounter(range)`, `CpuInstruction(String)`, `CpuMemoryRead/Write(range)`
  - `Spc700Step`, `Spc700ProgramCounter(range)`, `Spc700MemoryRead/Write(range)`
  - `ExecutionError`, `Interrupt(Option<NativeVectorTable>)`
- **`TraceStep`** — `Cpu(CpuState)` or `Spc700(Spc700State)`
- **`BreakReason`** — `{ trigger: EventFilter, event: DebugEvent }`
- **`DebugEvent`** — Union of `Cpu(CpuEvent)`, `MainBus(MainBusEvent)`, `ApuBus(ApuBusEvent)`, `Spc700(Spc700Event)`, `Error(String)`

### Event Collection

`Debugger` implements `DebugEventCollector<T>` for all event types (`CpuEvent`, `MainBusEvent`, `ApuBusEvent`, `Spc700Event`, `()`). Events are collected via `#[cold]` methods to keep the hot path fast.

### Parsing & Formatting

- `EventFilter::from_str` parses debugger commands like `"pc 0"`, `"r 10:1F"`, `"irq nmi"`, `"step"`.
- `Display for EventFilter` formats them back.

---

## Patterns & Conventions

1. **System Variants for Performance vs Accuracy** — `BatchedSystem` is fast (default UI), `SyncSystem` is cycle-accurate (tests), `AsyncSystem` uses threading.
2. **Debugger Shared State** — `DebuggerRef = Arc<Mutex<Debugger>>` is cloned into every component. Events are only emitted when `DEBUG_EVENTS_ENABLED` is true (zero-cost when disabled).
3. **Lazy APU Catch-Up** — The APU is not advanced on every cycle; it catches up at sample boundaries and APUIO accesses. See `apu/SUMMARY.md`.
4. **Vblank-Driven Frame Swapping** — Frames are produced on vblank rise, not continuously.
5. **Generic Bus Device Wrappers** — `SyncBusDevice`, `BatchedBusDeviceU24`, `AsyncBusDeviceU24` wrap PPU/APU to control update timing without changing their internals.

---

## For AI Agents

- **To add a new execution mode**, create a new `SystemImpl` type alias with different `ManagedBusDeviceU24` wrappers.
- **To debug a failing test**, use `debug_until(EventFilter::CpuProgramCounter(...))` or `trace_step_iter()` to step through execution.
- **The debugger is off by default.** Call `debugger().enable()` before adding log/break points.
- **Do not hold `SystemDebug` across `trace_step_iter()` calls** — they borrow the same `SystemImpl` mutably.
