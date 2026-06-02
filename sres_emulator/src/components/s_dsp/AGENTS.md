# S-DSP

Sony Digital Sound Processor. 8-voice stereo sample playback synthesizer.

## Files

| File | Purpose |
|------|---------|
| `mod.rs` | Top-level state, 128-byte register file, noise generator |
| `voice.rs` | Per-voice envelope, volume, pitch, BRR decoding |
| `pitch.rs` | Gaussian interpolation / pitch modulation |
| `brr.rs` | BRR (Bit Rate Reduction) sample block decoder |
| `test.rs` | Register read/write sanity test |
| `voice/` | Golden test assets (`.brr`, `.wav`) |
| `brr/` | Golden test assets (`.brr`, `.wav`) |

## Key Types

| Type | File | Purpose |
|------|------|---------|
| `SDsp` | `mod.rs` | Top-level DSP state |
| `Voice` | `voice.rs` | Per-channel synthesis |
| `DspEnvelope` | `voice.rs` | ADSR/GAIN envelope |
| `PitchGenerator` | `pitch.rs` | Gaussian-interpolated resampler |
| `BrrDecoder` | `brr.rs` | BRR decompressor |
| `NoiseGenerator` | `mod.rs` | LFSR white noise |

## Notable Details

- **Register map:** `$x0`-`$x9` = voice registers (voice = high nibble). `$5D` = dir, `$6C` = flg. `$4C` (KON) triggers voice start.
- **Pitch:** 14-bit unsigned; counter is 16-bit fixed-point with 12-sample buffer.
- **Mixing:** Saturating `i16` addition across 8 voices.
- **Memory:** Reads APU RAM as flat `&[u8]`. Sample directory at `dir * 0x100`, 4-byte entries (start LE, loop LE).
- **BRR:** 9-byte blocks (1 header + 16 nibbles). 4 IIR filters. `BrrDecoder` uses `VecDeque<i16>` with lazy block decode.
- **Noise:** 16-bit LFSR, rate dividers indexed by `noise_frequency` (0=off, 1=16Hz, ..., 31=32kHz).
- **Testing:** Golden-file tests compare decoded BRR/WAV output against checked-in `.wav` files.
- **Bitfields:** Uses `bilge` for registers (`Adsr1`, `Adsr2`, `BrrBlockHeader`, `Flg`).

## Integration

Instantiated inside APU (S-SMP). `generate_sample(memory)` called once per 32kHz sample. Result fed to host audio backend. Echo/FIR not yet implemented.
