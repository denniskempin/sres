# APU Integration Tests Summary

This directory contains high-level integration tests for the SRES APU (Audio Processing Unit), validating BRR sample playback, noise generation, and full-game music emulation against golden audio references.

## Directory Layout

```
apu_tests/
├── .DS_Store                  # macOS metadata (ignore)
├── play_brr_sample.sfc       # Assembled SNES ROM for BRR test
├── play_brr_sample.sfc.asm   # Source: SNES 65816 CPU bootstrap
├── play_brr_sample.spc       # Assembled SPC700 program for BRR test
├── play_brr_sample.spc.asm   # Source: SPC700 DSP setup & BRR playback
├── play_brr_sample.brr       # Raw BRR sample data (inserted into .spc)
├── play_brr_sample.wav       # Golden reference audio for BRR test
├── play_noise.sfc            # Assembled SNES ROM for noise test
├── play_noise.sfc.asm        # Source: SNES 65816 CPU bootstrap
├── play_noise.spc            # Assembled SPC700 program for noise test
├── play_noise.spc.asm        # Source: SPC700 DSP noise setup
├── ffvii_prelude.sfc         # Commercial game ROM for music test
└── ffvii_prelude.wav         # Golden reference audio for music test
```

The Rust test driver lives one directory up at `sres_emulator/tests/apu_tests.rs` (not inside this folder).

## Test Inventory

| Rust Test | ROM / Data | What It Tests |
|---|---|---|
| `test_play_brr_sample` | `play_brr_sample.sfc` + `.spc` + `.brr` | BRR sample decode, voice pitch, ADSR, and DSP mixing. Runs until SPC700 PC `0x02E9`, asserts voice state, generates 7,936 samples, and compares to `play_brr_sample.wav`. |
| `test_play_noise` | `play_noise.sfc` + `.spc` | DSP noise generator (kick, hi-hat, snare patterns). Loads SPC program at `0x0200`, verifies RAM match, runs until PC `0x02DD`, and asserts voice state string. |
| `test_ffvii_prelude` | `ffvii_prelude.sfc` | End-to-end music playback. Runs 5 seconds (5 × 60 frames), collects audio via buffer swapping, and compares to `ffvii_prelude.wav`. |

## Test Patterns & Helpers

### 1. ROM Loading & System Setup
All tests construct a `System` from an `.sfc` ROM:
```rust
let mut system = System::with_cartridge(
    &Cartridge::with_sfc_file(&path).unwrap(),
);
```

### 2. Synchronization via Debugger
Tests use `system.debug_until(EventFilter::Spc700ProgramCounter(range))` to pause execution at a known SPC700 program counter. This ensures the DSP is in a deterministic state before assertions or audio capture.

### 3. Intermediate DSP State Assertions
Before collecting audio, tests often assert the string form of a DSP voice for quick regression detection:
```rust
assert_eq!(
    system.debug().apu().dsp().voice(0),
    "vol:127/127 pitch:4096 adsr:(10,7,7,0) src:$00 env:0 out:0"
);
```

### 4. Audio Capture
Two patterns are used:
- **Fixed sample count**: `system.execute_for_audio_samples(N)` then `system.swap_audio_buffer(...)`.
- **Frame-based**: `system.execute_frames(60)` in a loop, swapping buffers each iteration and draining samples into a `Vec<i16>`.

### 5. Golden WAV Comparison
Audio is verified with:
```rust
use sres_emulator::common::test_util::compare_wav_against_golden;
compare_wav_against_golden(&samples, &path_prefix);
```
Rules:
- Expected golden file: `<path_prefix>.wav`.
- Format: mono, 32 kHz, 16-bit signed PCM.
- If the golden file is missing, it is **auto-created** from the actual output (first-run seeding).
- If samples differ, the test panics and writes `<path_prefix>.actual.wav` for inspection.

## Test ROM Assembly Conventions

### CPU Bootstrap (`.sfc.asm`)
- Uses `arch snes.cpu` and a `seek()` macro for LoROM mapping.
- Typical boilerplate:
  1. `SNES_INIT(SLOWROM)`
  2. `SPCWaitBoot()`
  3. `TransferBlockSPC(SPCROM, SPCRAM, SPCROM.size)`
  4. `SPCExecute(SPCRAM)` (usually `$0200`)
  5. Infinite `jmp Loop`
- Includes `snes.inc`, `snes_header.asm`, and `snes_spc700.inc` from shared asm libraries.
- `play_brr_sample` uses `../asm_lib/`; `play_noise` uses `lib/` (note the path difference).

### SPC700 Program (`.spc.asm`)
- Uses `arch snes.smp` and a `seek()` macro relative to `SPCRAM`.
- Typical flow:
  1. `SPC_INIT()`
  2. Configure master volumes (`MVOLL`, `MVOLR`)
  3. Optionally set up echo (`ESA`, `EDL`, `EON`, `FLG`, `EFB`, `FIR0-7`, `EVOLL/R`)
  4. Configure voice parameters (`V0VOLL`, `V0VOLR`, `V0PITCHL/H`, `V0SRCN`, `V0ADSR1/2`, `V0GAIN`)
  5. Trigger with `KON`
  6. Spin in `jmp Loop`
- BRR test: embeds a sample directory at `$0300` and inserts raw `.brr` data at `$0400`.
- Noise test: manipulates `NON` and `FLG` noise frequency bits to emulate drum patterns.

### Raw Data
- `.brr` files are binary blobs inserted directly by the assembler (`insert BRRSample, "..."`).
- `.spc` files are produced by assembling the SPC700 source and then inserted into the SNES ROM.

## Key API Surface Used by Tests

| API | Location | Purpose |
|---|---|---|
| `System::with_cartridge` | `src/lib.rs` | Boot the emulator with a ROM. |
| `System::debug_until` | `src/lib.rs` | Run until debugger event fires. |
| `System::execute_for_audio_samples` | `src/lib.rs` | Run until APU has produced N samples. |
| `System::execute_frames` | `src/lib.rs` | Run for N video frames. |
| `System::swap_audio_buffer` | `src/lib.rs` | Zero-copy exchange of APU `AudioBuffer`. |
| `AudioBuffer` | `src/apu/mod.rs` | Typed `Vec<i16>` wrapper with `into_vec()` and `iter()`. |
| `compare_wav_against_golden` | `src/common/test_util.rs` | Golden-file compare / seeding helper. |
| `format_memory` | `src/common/util.rs` | Hex-dump helper used to compare SPC RAM slices. |

## Notes for Future Agents

- **Golden file seeding**: If you add a new audio test, the first run will *create* the `.wav` golden file rather than fail. Always review the generated golden manually before committing.
- **Debugger events**: `EventFilter::Spc700ProgramCounter(range)` is the primary synchronization primitive for APU tests. Choose stable infinite-loop addresses or known milestones in the SPC700 program.
- **Sample counts**: `execute_for_audio_samples` counts the APU’s internal sample buffer size, so clear or swap the buffer first if you need an exact sample window.
- **Assembly toolchain**: The `.asm` files are written for bass (or a compatible 65xx/SPC700 assembler). Rebuilding them requires the shared include libraries under `tests/asm_lib/` or `tests/apu_tests/lib/`.
- **Commercial ROM**: `ffvii_prelude.sfc` is a real-game ROM fragment. Do not redistribute it.
