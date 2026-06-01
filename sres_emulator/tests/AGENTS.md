# `sres_emulator/tests` — Summary

This directory contains the integration test suite for `sres_emulator`. Tests are organized by subsystem and use a mix of trace comparison, golden-image validation, ROM-outcome verification, and audio golden-file testing.

---

## Top-Level Test Files

These `.rs` files are the main test drivers that import from their corresponding subdirectories:

| File | Purpose |
|------|---------|
| `rom_tests.rs` | CPU instruction trace comparison against BSNES, plus DMA ROM-outcome tests. |
| `ppu_tests.rs` | Framebuffer golden-image tests for PPU rendering (BG modes, sprites, HDMA, blending). |
| `apu_tests.rs` | Audio golden-WAV tests for SPC700 + S-DSP playback (BRR samples, noise, full music). |

---

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `rom_tests/` | Test ROMs (`.sfc`, `.asm`) and BSNES trace logs (`.json.xz`) for `rom_tests.rs`. |
| `ppu_tests/` | Test ROMs, reference `.png` images, PPU state snapshots, and assembly helpers for `ppu_tests.rs`. |
| `apu_tests/` | Test ROMs, reference `.wav` files, and assembly for `apu_tests.rs`. |
| `asm_lib/` | Shared assembly library for constructing test ROMs (SNES register definitions, headers, macros). |
| `lib/` | Symlink to `asm_lib/`; used as an include path by test assembly files. |

Each subdirectory has its own `SUMMARY.md` with full details.

---

## Test Categories

### 1. Trace-Comparison Tests (`rom_tests.rs`)

- Load a `.sfc` ROM into `SyncSystem`.
- Stream a BSNES execution trace line-by-line.
- Advance the emulator one CPU step per trace line and compare `CpuState` string representations.
- Any mismatch fails immediately with a `pretty_assertions` diff.

**Coverage:** Peter Lemon's (krom) 65816 CPU test suite (`krom_adc`, `krom_and`, `krom_asl`, etc.) covering most instructions in all addressing modes.

**Special case:** `play_noise` uses a mixed CPU+SPC700 trace and asserts sync at APUIO boundaries.

### 2. ROM-Outcome Tests (`rom_tests.rs`)

- Load a hand-written assembly ROM into `System`.
- Run until CPU halt (`stp`).
- Inspect memory/buses with `cpu.bus.peek_range(...)`.

**Coverage:** `dma_vram`, `dma_cgram`, `dma_oam` — verify DMA transfer correctness to/from PPU memory.

### 3. Golden-Image PPU Tests (`ppu_tests.rs`)

- Run a test ROM for a number of frames.
- Capture the framebuffer and compare against a reference `.png`.
- On mismatch, an `.actual.png` is written for visual diff.

**Coverage:**
- KROM tests: HDMA, BG modes (2bpp/4bpp/8bpp), blending, hi-color mode, hello-world, sprites
- Hand-written: sprite rendering (sizes, flips, priority, alt palettes)
- Colour math: multi-scene ROM advanced via simulated joypad input
- Debug render: isolated PPU utilities (`render_sprite`, `render_background`, `render_vram`)
- Commercial ROM snapshots: SMW and Zelda scenes (no copyrighted ROMs needed at test time)

### 4. Golden-WAV APU Tests (`apu_tests.rs`)

- Boot the APU and run a test ROM.
- Capture audio output into a buffer.
- Compare against a reference `.wav` file using `compare_wav_against_golden`.
- Missing golden files are auto-created on first run; `.actual.wav` is written on mismatch.

**Coverage:**
- `test_play_brr_sample` — custom BRR sample playback
- `test_play_noise` — DSP noise generation
- `test_ffvii_prelude` — full-game music emulation

---

## Assembly Conventions

Test ROMs are written in two assembler dialects:

1. **bass** — Used by krom tests and many PPU tests. Syntax: `arch snes.cpu`, `seek(...)`, macros.
2. **xa65** — Used by hand-written tests (DMA tests, some PPU/ROM tests). Simpler syntax.

Shared assembly lives in `asm_lib/` (and `lib/`):
- `snes.inc` — Full SNES register map and constants
- `snes_header.asm` / `snes_header_ret.asm` — ROM headers and interrupt vectors
- `snes_gfx.inc` — Graphics initialization macros (`SNES_INIT`, `LoadPAL`, `WaitNMI`)
- `snes_spc700.inc` — SPC700 register definitions and transfer macros
- `base.asm` — Minimal startup for xa65-based tests
- `font8x8.asm` — 1BPP 8×8 font tile data

---

## Dependencies & Tooling

- **Git LFS**: `.sfc`, `.png`, `.wav`, and `.json.xz` files are stored in Git LFS. If LFS objects are missing, some tests may be skipped.
- **xa65 assembler**: Required for some test ROM builds. Install via `sudo apt-get install xa65`.
- **Golden files**: First test run auto-creates missing `.png` / `.wav` references. Commit these to LFS.

---

## For AI Agents

- **Adding a new CPU test?** Write assembly → assemble to `.sfc` → generate BSNES trace → compress to `.xz` → add `#[test]` calling `run_rom_test("name")` in `rom_tests.rs`.
- **Adding a new PPU test?** Write assembly → assemble to `.sfc` → run test to generate `.actual.png` → verify visually → rename to golden `.png` → add test in `ppu_tests.rs`.
- **Adding a new APU test?** Write CPU bootstrap + SPC700 program → assemble → run test to generate `.actual.wav` → verify aurally → rename to golden `.wav` → add test in `apu_tests.rs`.
- **All three test drivers use `System` (default `BatchedSystem`)**, except trace-comparison tests which require `SyncSystem` for cycle accuracy.
