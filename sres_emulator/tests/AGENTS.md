# `sres_emulator/tests`

Cargo integration tests for `sres_emulator`. Taxonomy and System-variant mapping live in root Testing Strategy.

## Files

| File | Owns |
|------|------|
| `rom_tests.rs` | Trace-comparison (`SyncSystem`, `run_rom_test`) and ROM-outcome (`System`, `run_test_rom` until `stp`, `< 10_000_000` steps) |
| `ppu_tests.rs` | ROM framebuffer and debug-render (`System`); snapshot tests (`Ppu` only, no ROM) |
| `apu_tests.rs` | Golden-WAV (`System`, `compare_wav_against_golden`); `play_noise` RAM/DSP asserts |

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `rom_tests/` | `.sfc`, BSNES `-trace.log.xz`, assembly for `rom_tests.rs` |
| `ppu_tests/` | `.sfc`, golden `.png`, `.snapshot` for `ppu_tests.rs` |
| `apu_tests/` | `.sfc`, golden `.wav`, assembly for `apu_tests.rs` |
| `asm_lib/` | Shared bass includes |
| `lib/` | Symlink → `asm_lib/` |

## Behaviors & Gotchas

1. Sources are bass (`arch snes.cpu` / `arch snes.smp`). Cargo does not assemble; tests load committed `.sfc`.
2. Missing ROM or trace files fail (`Cartridge::with_sfc_file` / `File::open`); these drivers do not skip or reassemble.
3. Trace tests write `0x93` to `$000000` before reset (`run_rom_test`); reason unknown.
4. Snapshot tests call `Ppu::load_state` then `draw_scanline` with no ROM (`run_snapshot_framebuffer_test`).

## Integration

- Invoked as crate integration-test binaries `rom_tests`, `ppu_tests`, `apu_tests`.
- Drivers load `Cartridge::with_sfc_file` into `System` or `SyncSystem`. Snapshot tests construct `Ppu` directly.
- Trace compares `CpuState` strings from xz Mesen logs via `SystemDebug::cpu_step_iter`.
- ROM-outcome asserts `cpu.bus.peek_range` after `halted()`.
- WAV tests use `debug_until` / `execute_for_audio_samples` / `execute_frames` then `swap_audio_buffer`. `play_noise` does not.

## Tests

- Trace + DMA: `cargo nextest run -p sres_emulator --test rom_tests`
- Golden-image: `cargo nextest run -p sres_emulator --test ppu_tests`
- Golden-WAV: `cargo nextest run -p sres_emulator --test apu_tests`
