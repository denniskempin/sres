# `sres_emulator/src`

Library crate root: `SystemImpl` orchestration, `StandardController` packing, and `Debugger`.

## Files

| File | Owns |
|------|------|
| `lib.rs` | `SystemImpl<PpuT, ApuT>`. Owns CPU, `DebuggerRef`, pending `Framebuffer`. |
| `controller.rs` | `StandardController`. `to_u16()` big-endian pack for `$4218`/`$4219`. |
| `debugger.rs` | `Debugger`, `EventFilter`, `TraceStep`, `DebuggerRef`. Off until `enable()`. |

## Behaviors & Gotchas

1. `SystemImpl.debugger_enabled` is not `Debugger.enabled`. Only `debug_until` sets the former, which `sync()`s PPU/APU on every CPU `step()`. `debugger().enable()` (UI, `trace_step_iter`) writes `DEBUG_EVENTS_ENABLED` only.
2. `Debugger::enable()` / `disable()` are the only stores of process-wide `DEBUG_EVENTS_ENABLED` (zero-cost path: root). `disable()` on one instance turns collection off for the process.
3. `ExecutionResult::Halt` only if the CPU is already halted at entry. Reaching halt during `execute_until_halt` returns `Normal` after `ppu.sync()` + `apu.sync()`. `Break` skips that flush.
4. `force_headless()` skips `draw_scanline`; PPU clock still advances.
5. Do not hold `SystemDebug` across `SystemDebug::trace_step_iter()` (borrow conflict). That call clears log points and logs only `EventFilter::CpuStep` and `Spc700Step`; `pop_oldest_trace_step` panics on any other `DebugEvent`.
6. A pending video frame not consumed by `swap_video_frame` is overwritten on the next vblank rise.

## Integration

- Frontends and tests construct `System` / `SyncSystem` / `AsyncSystem` (when to pick: root).
- `SystemImpl` owns `Cpu<MainBusImpl<PpuT, ApuT>>`. Device wrappers: `main_bus/devices.rs`.
- `update_joypads` stores raw `u16` on `MainBusImpl` (`$4218`–`$421B`). Pack with `StandardController::to_u16()`.

## Gaps

- HDMA and FastROM: unimplemented; see root. `main_bus/` in this crate is DMA only.
- Serial joypad `$4016`/`$4017`: unimplemented in `main_bus` (read returns `0`). This layer packs auto-read `$4218`–`$421B` only.

## Tests

- Unit tests live in `debugger.rs` (`EventFilter` parse/format). `lib.rs` and `controller.rs` have none.
- `cargo nextest run -p sres_emulator --lib`
- Debugger-only: `cargo nextest run -p sres_emulator --lib -E 'test(debugger::)'`

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `common/` | Shared types, traits, utilities. |
| `components/` | Independent hardware: CPU, PPU, S-DSP, SPC700, cartridge, clock. |
| `apu/` | SPC700 + S-DSP orchestration. |
| `main_bus/` | Memory map, DMA, device wrappers, interrupts. |
