# APU Audio Tests

Golden-WAV audio integration tests for APU (SPC700 + DSP). The Rust test driver is `apu_tests.rs` (one directory up).

## Tests

| Data Files | What It Tests |
|---|---|
| `play_brr_sample.{sfc,spc,brr,wav}` | BRR sample decode, pitch, ADSR, mixing |
| `play_noise.{sfc,spc}` | DSP noise generator patterns |
| `ffvii_prelude.{sfc,wav}` | 5-second full-game music playback |

Tests run ROMs, capture audio, and compare against `.wav` golden files.

## Golden Files

- Mono, 32 kHz, 16-bit signed PCM.
- Missing goldens are **auto-created** on first run. Review before committing.
- Mismatch writes `<prefix>.actual.wav` for inspection.
- `ffvii_prelude.sfc` is a commercial ROM fragment. Do not redistribute.

## Assembly

- `.sfc.asm`: SNES 65816 CPU bootstrap (loads SPC program, infinite loops)
- `.spc.asm`: SPC700 program (configures DSP, triggers playback, infinite loops)
- `.brr`: Raw BRR sample data
- Uses `bass` assembler with shared includes from `tests/asm_lib/` or `tests/apu_tests/lib/`
