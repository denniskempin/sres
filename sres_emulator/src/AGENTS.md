# `sres_emulator/src`

Root of the `sres_emulator` library crate. System orchestration, debugger, controller input, and module declarations.

## Files

| File | Purpose |
|------|---------|
| `lib.rs` | `SystemImpl<PpuT, ApuT>` — owns CPU, debugger, framebuffer. Three variants: `BatchedSystem` (default, batched PPU/APU), `SyncSystem` (cycle-accurate), `AsyncSystem` (threaded APU). |
| `controller.rs` | `StandardController` — 16-bit packed struct for SNES pad input. `to_u16()` for $4218/$4219 format. |
| `debugger.rs` | `Debugger` with breakpoints, log points, 16k event ring buffer. `EventFilter` for conditions (PC ranges, memory ops, instructions, interrupts). `TraceStepIter` for stepping. Off by default; call `debugger().enable()`. |

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `common/` | Shared types, traits, utilities. |
| `components/` | Reusable components: CPU, PPU, S-DSP, SPC700, cartridge, clock. |
| `apu/` | APU integration: SPC700 + S-DSP orchestration, timers, APUIO. |
| `main_bus/` | System bus: memory mapping, DMA/HDMA, device wrappers, interrupts. |

## `lib.rs` Key Details

**Execution methods** (all return `ExecutionResult`):
- `execute_one_instruction()`, `execute_until_halt()`, `execute_frames(n)`, `execute_scanlines(n)`, `execute_cycles(n)`, `execute_for_audio_samples(n)`, `execute_for_duration(s)`, `debug_until(event_filter)`

**Output:**
- `swap_video_frame(&mut Framebuffer)` — true on vblank rise
- `swap_audio_buffer(&mut AudioBuffer)` — exchanges APU sample buffer
- `update_joypads(joy1, joy2)` — writes to main bus
- `force_headless()` — disables PPU rendering

**Debug:**
- `TraceStepIter` yields `TraceStep::Cpu(CpuState)` or `Spc700(Spc700State)`
- `SystemDebug<'a>` exposes `PpuDebug` + `ApuDebug` (do not hold across `trace_step_iter()` calls)

## Patterns

- **Lazy APU catch-up**: APU advances at sample boundaries and APUIO accesses, not every cycle.
- **Vblank frame swap**: New frame available on vblank rise only.
- **Debugger zero-cost**: Events only emitted when `DEBUG_EVENTS_ENABLED` is true.
