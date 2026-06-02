# `sres_emulator/tests`

Integration test suite for `sres_emulator`.

## Test Drivers

| File | Tests |
|------|-------|
| `rom_tests.rs` | CPU trace comparison vs BSNES, DMA ROM-outcome tests |
| `ppu_tests.rs` | Framebuffer golden-image PPU rendering tests |
| `apu_tests.rs` | Audio golden-WAV SPC700 + S-DSP tests |

## Subdirectories

| Directory | Contents |
|-----------|----------|
| `rom_tests/` | ROMs, traces, assembly for `rom_tests.rs` |
| `ppu_tests/` | ROMs, reference PNGs, snapshots for `ppu_tests.rs` |
| `apu_tests/` | ROMs, reference WAVs, assembly for `apu_tests.rs` |
| `asm_lib/` | Shared assembly library for test ROMs |
| `lib/` | Symlink to `asm_lib/` |

## Test Types

- **Trace-comparison**: Load `.sfc` into `SyncSystem`, step against BSNES trace, compare `CpuState` strings. Covers krom 65816 instruction tests.
- **ROM-outcome**: Load assembly ROM into `System`, run until `stp`, inspect memory with `cpu.bus.peek_range(...)`. Covers DMA correctness.
- **Golden-image**: Run test ROM for N frames, compare framebuffer against reference `.png`. Mismatch writes `.actual.png`.
- **Golden-WAV**: Boot APU, capture audio, compare against reference `.wav`. Missing golden files auto-created on first run. Mismatch writes `.actual.wav`.

## Assembly

Test ROMs use **bass** (krom/PPU tests) or **xa65** (hand-written/DMA tests).
Shared assembly in `asm_lib/`: `snes.inc`, `snes_header.asm`, `snes_gfx.inc`, `snes_spc700.inc`, `base.asm`, `font8x8.asm`.

## Tooling

- **Git LFS**: `.sfc`, `.png`, `.wav`, `.json.xz` stored in LFS. Missing objects may skip tests.
- **xa65**: Install via `apt-get install xa65`.
- **Golden files**: First run auto-creates missing `.png`/`.wav`. Commit to LFS after verification
