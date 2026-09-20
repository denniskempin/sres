# `sres_egui/src/debug`

Debugger panels composed by parent `DebugUi` (`debug.rs`). Inspection is peek-only; `debug_controls_widget` returns `DebugCommand` and does not execute.

## Files

| File | Owns |
|------|------|
| `cpu.rs` | `cpu_state_widget`, `disassembly_widget`, `debug_controls_widget` |
| `ppu.rs` | `PpuDebugWindow` (Background / Sprites / VRAM / Palette), `clock_info_widget` |
| `apu.rs` | `ApuDebugWindow` (S-DSP voices + KON/KOF/noise) |
| `memory.rs` | `MemoryViewer` 24-bit hex dump |
| `log_viewer.rs` | `LogViewer` over `debugger.log` |
| `unimplemented.rs` | `UnimplementedViewer` hit counts, break-on-hit, reset |
| `syntax.rs` | `cpu_disassembly_line`, `log_line`, `ADDR_ANNOTATIONS` |
| `event.rs` | `event_filter_widget`, `EventFilterInputState` |

## Behaviors & Gotchas

1. Panels do not call `execute_*` or `cpu.step()`. `MemoryViewer::show` takes `peek: Fn(AddressU24) -> Option<u8>` (`memory.rs:39`); `None` prints `XX` (`memory.rs:75`). CPU lookahead uses `CpuDebug::peek_next_operations` (`cpu.rs:39`). PPU/APU take `PpuDebug` / `ApuDebug` from `emulator.debug()` (`ppu.rs:72-76`, `apu.rs:32-33`). CPU registers use `emulator.cpu.debug()` (`cpu.rs:16`), not `SystemDebug`.
2. `disassembly_widget` walks `debugger().cpu_trace().skip(100)` then peeks 20 ops (`cpu.rs:36-40`). `cpu_trace` is newest-first (`RingBuffer::push` front); skip drops the 100 newest `CpuEvent::Step` entries. With fewer than 100 logged CPU steps the history block is empty.
3. Step / Step Frame / Step Scanline in `debug_controls_widget` are enabled only while paused and return `DebugCommand` (`cpu.rs:52-81`).
4. `event_filter_widget` mutates the `Vec<EventFilter>` the caller passes and reads caller-owned `EventFilterInputState`: parent right panel uses `break_points` (`debug.rs:168-172`); `LogViewer` uses `log_points` (`log_viewer.rs:40`). Quick-add "Step" pushes `CpuProgramCounter(0..u32::MAX)` / `Spc700ProgramCounter(0..u16::MAX)`, not `EventFilter::CpuStep` / `Spc700Step` (`event.rs:88`, `event.rs:114`).
5. `LogViewer` iterates `log.stack` reversed (oldest first) with `stick_to_bottom` (`log_viewer.rs:51-64`). `UnimplementedViewer` lists `debugger.unimplemented_hits()` only (not the ring).
6. `PpuSpritesWidget` updates textures only for visible table rows (`ppu.rs:189-199`). `PpuDebugWindow` textures start as `ColorImage::example` until first `show` (`ppu.rs:97`).
7. `label_cpu_effective_addr` / `label_cpu_pc` write `InternalLink::CpuMemory` / `CpuProgramCounter` (`syntax.rs:168-177`). `label_spc700_pc` writes `Spc700ProgramCounter` (`syntax.rs:180-183`).

## Integration

- Parent `DebugUi` in `debug.rs` owns the window types, calls `show` / the CPU widgets, and routes `InternalLink`.
- `debug_controls_widget` is the only producer of `DebugCommand` in this directory.

## Gaps

- `ApuBus` `log_line` rows call `label_addr` without writing `InternalLink` (`syntax.rs:73-84`).
- Sample start/loop in `ApuDebugWindow` is omitted when `sample_source == 0` (`apu.rs:139`).

## Tests

- Parent crate command: `cargo test -p sres_egui`.
- `cpu_widgets`, `ppu_widgets`, `apu_widgets`, `log_line_events`, `unimplemented_widget_snapshot`.
