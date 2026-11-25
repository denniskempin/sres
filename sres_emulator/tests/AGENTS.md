# `sres_emulator/tests`

Cargo integration tests for `sres_emulator`. Taxonomy and System-variant mapping live in root Testing Strategy.

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `integration/` | Consolidated integration test binary (`main.rs`, `rom_tests.rs`, `ppu_tests.rs`, `apu_tests.rs`) |
| `rom_tests/` | `.sfc`, BSNES `-trace.log.xz`, assembly for `integration/rom_tests.rs` |
| `ppu_tests/` | `.sfc`, golden `.png`, `.snapshot`, `.writes` for `integration/ppu_tests.rs` |
| `apu_tests/` | `.sfc`, golden `.wav`, assembly for `integration/apu_tests.rs` |
| `asm_lib/` | Shared bass includes |
| `lib/` | Symlink → `asm_lib/` |

## Behaviors & Gotchas

1. Sources are bass (`arch snes.cpu` / `arch snes.smp`). Cargo does not assemble; tests load committed `.sfc`.
2. Missing ROM or trace files fail (`Cartridge::with_sfc_file` / `File::open`); these drivers do not skip or reassemble.
3. Trace tests write `0x93` to `$000000` before reset (`run_rom_test`); reason unknown.
4. Snapshot tests call `Ppu::load_state`, replay `{name}.writes` (missing = none), then `draw_scanline` with no ROM (`run_snapshot_framebuffer_test`).
5. Drivers sample `clock_info()` in the loop (`execute_one_instruction` until the desired `v`/`f`). `Waiting` (`WAI`) can skip a scanline in one `step()`; keep going. Do not add a device latch for tests.

## Integration

- Invoked as a single crate integration-test binary `integration` (`integration/main.rs`) with modules `rom_tests`, `ppu_tests`, `apu_tests`.
- Drivers load `Cartridge::with_sfc_file` into `System` or `SyncSystem`. Snapshot tests construct `Ppu` directly.
- Trace compares `CpuState` strings from xz Mesen logs via `SystemDebug::cpu_step_iter`.
- ROM-outcome asserts `cpu.bus.peek_range` or `peek_u8` after `halted()`.
- WAV tests use `debug_until` / `execute_for_audio_samples` / `execute_frames` then `swap_audio_buffer`. `play_noise` does not.

## Tests

- Trace + DMA: `cargo nextest run -p sres_emulator --test integration rom_tests::`
- Golden-image: `cargo nextest run -p sres_emulator --test integration ppu_tests::`
- Golden-WAV: `cargo nextest run -p sres_emulator --test integration apu_tests::`
