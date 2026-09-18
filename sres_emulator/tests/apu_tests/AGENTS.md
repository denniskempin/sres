# `sres_emulator/tests/apu_tests`

Assets for `../apu_tests.rs`. No `.rs` here.

## Files

| Prefix | Owns |
|--------|------|
| `play_brr_sample` | `.sfc`/`.spc`/`.brr` and golden `.wav` |
| `play_noise` | `.sfc`/`.spc`; no `.wav` |
| `ffvii_prelude` | `.sfc` and golden `.wav`; no `.asm` |

## Behaviors & Gotchas

1. `play_noise` is not a WAV test. The driver compares APU RAM to `play_noise.spc` at `$0200`, then asserts DSP `voice(0)` after Kick at `$02DD`. The SPC program continues into hi-hat and snare; the test does not.
2. `play_brr_sample` idle loop is at `$02E9` (`jmp Loop` in `play_brr_sample.spc.asm`). The driver waits there, then captures 7936 samples. Moving `Loop` requires a matching PC filter in `../apu_tests.rs`.
3. `play_brr_sample*.asm` includes `../asm_lib/`. `play_noise*.asm` includes `lib/` (no `apu_tests/lib`; that path needs parent `tests/lib` as include root).
4. `ffvii_prelude` has no source in this tree. The driver runs 5 × 60 frames, then diffs the golden `.wav`.

## Tests

- `cargo nextest run -p sres_emulator --test apu_tests`
