# `sres_emulator/src/components/s_dsp`

Sony S-DSP: register file, per-voice BRR/envelope/pitch, noise LFSR, and mix.

## Files

| File | Owns |
|------|------|
| `mod.rs` | `SDsp`, `Flg`, `NoiseGenerator`, `SDspDebug`. `read_register` / `write_register`; `generate_sample`. |
| `voice.rs` | `Voice`, `DspEnvelope`. KON start; ADSR/GAIN; L+R fold to `i16`. |
| `pitch.rs` | `PitchGenerator` Gaussian interpolator; 12-sample buffer. |
| `brr.rs` | `BrrDecoder`. 9-byte blocks; 4 IIR filters. |
| `test.rs` | `SDsp` register read/write sanity. |
| `voice/` | Golden `voice_brr_sample.brr` / `.wav`. |
| `brr/` | Golden `play_brr_sample.brr` / `.wav`. |

## Behaviors & Gotchas

1. Write `$4C` (KON) sets each `Voice.trigger_on` from that bit. The next `generate_sample_with_noise` resets `BrrDecoder`, inits `PitchGenerator`, and `DspEnvelope::key_on`. The write is not stored in `raw` (`SDspDebug::key_on` stays `0`).
2. `$5D` (DIR) page is `dir * 0x100`; SRCN indexes a 4-byte LE (start, loop) pair (`Voice::dir_info`).
3. `$6C` (FLG) is `Flg`; `generate_sample` uses only `noise_frequency`.
4. `BrrDecoder::next_sample` fills a `VecDeque<i16>` from one 9-byte block only when the queue is empty.
5. `NoiseGenerator` is a 16-bit LFSR (bit 14 XOR 13); divider index `0` is off.
6. Voices mix with `i16::saturating_add`. Each voice already summed L+R with a wrapping `as i16`.
7. `PitchCounter` phase wraps at `0xC000` so `index()` stays in the 12-sample buffer.

## Hardware Map

| Range | Owner |
|-------|-------|
| `$x0–$x9` | `Voice` (high nibble = voice) |
| `$4C` | KON |
| `$5D` | DIR |
| `$6C` | FLG |

Semantics: `docs/index.md`.

## Integration

- Who owns `SDsp` and when `generate_sample` runs: `sres_emulator/src/apu/AGENTS.md`.
- `generate_sample` walks `voices`; NON `$3D` in `raw` selects noise vs `PitchGenerator` / `BrrDecoder`.
- Debugger reads `SDspDebug`.

## Gaps

- Echo/FIR: `DspEcho`. `$5C` KOFF: `DspKoff` (does not set `trigger_off`). `$0C`/`$1C` MVOL: `DspMvol`. `$2D` PMON: `DspPmon`. `$7C` ENDX: `DspEndx`. `Flg.mute` / `Flg.reset`: `DspFlgMute` / `DspFlgReset`. `$3D` NON is implemented. KON is only `$4C`. Unused `$1D`/`$xA`/`$xB`/`$xE` are explicit (store in `raw`). Unknown is `on_error`.

## Tests

- Lib unit tests plus golden WAV in `voice/` and `brr/` (`compare_wav_against_golden`).
- `cargo nextest run -p sres_emulator --lib -E 'test(components::s_dsp::)'`
