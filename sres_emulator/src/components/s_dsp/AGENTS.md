# S-DSP Component Summary

This directory implements the **S-DSP** (Sony Digital Sound Processor) component of the SNES APU (Audio Processing Unit). The S-DSP is an 8-voice stereo sample playback synthesizer. The implementation here is a Rust translation of the hardware behavior, focusing on sample decoding, pitch interpolation, envelope generation, and voice mixing.

## Directory Layout

```
sres_emulator/src/components/s_dsp/
├── mod.rs          # Top-level SDsp orchestrator, register file, noise generator
├── voice.rs        # Per-voice state: envelope, volume, pitch, BRR decoding
├── pitch.rs        # Gaussian interpolation / pitch modulation engine
├── brr.rs          # BRR (Bit Rate Reduction) sample block decoder
├── test.rs         # Minimal register read/write sanity test
├── voice/          # Golden test assets for voice-level playback tests
│   ├── voice_brr_sample.brr
│   └── voice_brr_sample.wav
└── brr/            # Golden test assets for BRR decode tests
    ├── play_brr_sample.brr
    └── play_brr_sample.wav
```

## Architecture Overview

### 1. `SDsp` (mod.rs)

The top-level struct representing the entire S-DSP chip.

- **Fields:**
  - `raw: [u8; 128]` — mirror of the 128-byte DSP register file.
  - `voices: [Voice; 8]` — the eight independent synthesis voices.
  - `dir: u8` — sample directory page register (`$5D`).
  - `flg: Flg` — flags register (`$6C`) controlling reset, mute, echo disable, and noise frequency.
  - `noise_generator: NoiseGenerator` — pseudorandom white-noise source.
  - `global_counter: u16` — monotonic sample counter used for envelope timing and noise rate division.

- **Key Methods:**
  - `read_register(reg: u8) -> u8` / `write_register(reg: u8, value: u8)`
    - Registers `$x0`–`$x9` map to voice registers (voice index = high nibble).
    - `$5D` → `dir`, `$6C` → `flg`.
    - `$4C` (KON / Key On) is special-cased on write: each bit sets `trigger_on` for the corresponding voice.
  - `generate_sample(memory: &[u8]) -> i16`
    - Advances noise generator, iterates all 8 voices, mixes their outputs with **saturating addition** (`i16`), and increments the global counter.

- **`Flg` Register** (bit-structured with `bilge`):
  - Bit 7 (`reset`): Soft reset.
  - Bit 6 (`mute`): Mute all voices.
  - Bit 5 (`echo_disable`): Echo disable.
  - Bits 0-4 (`noise_frequency`): Noise clock divider index.

- **`NoiseGenerator`:**
  - 16-bit LFSR-based white noise.
  - Rate dividers indexed by `noise_frequency` (0 = disabled, 1 = 16 Hz, …, 31 = 32 kHz).
  - Taps at bits 14 and 13 (XOR), shifted left; zero guard reseeds to `1`.

### 2. `Voice` (voice.rs)

Each voice is a self-contained sample-playback channel.

- **Registers per voice (`$x0`–`$x9`):**
  - `$x0` / `$x1` — signed 8-bit left/right volume (`vol_l`, `vol_r`).
  - `$x2` / `$x3` — 14-bit unsigned pitch (`pitch`).
  - `$x4` — sample source index (`sample_source`), references the sample directory.
  - `$x5` — `Adsr1` (attack rate, decay rate, ADSR enable bit).
  - `$x6` — `Adsr2` (sustain level, release rate / sustain rate).
  - `$x7` — `Gain` (custom envelope mode when ADSR is disabled).
  - `$x8` — `envx` (read-only envelope level, updated each sample).
  - `$x9` — `outx` (read-only pre-volume output, updated each sample).

- **Voice State Machine:**
  - `trigger_on` / `trigger_off` are latched by `SDsp` on KON writes.
  - On `trigger_on`:
    1. Reads sample directory entry (start addr, loop addr) from APU memory.
    2. Resets `BrrDecoder` and seeds `PitchGenerator` with the first 12 decoded samples.
    3. Resets envelope to `Attack`.
  - On `trigger_off`: transitions envelope to `Release`.

- **Sample Generation (`generate_sample_with_noise`):**
  1. Handle triggers.
  2. Update envelope (`DspEnvelope::update`).
  3. Fetch next sample: either noise (`±0x4000`) or Gaussian-interpolated BRR output.
  4. Apply envelope: `(sample * envelope_value) >> 11`.
  5. Record `outx` and push to debug ring buffer.
  6. Apply left/right volume: `(enveloped * vol) >> 7` and sum channels into a single `i16`.

- **`DspEnvelope`:**
  - Internal 16-bit value range `0`–`0x7FF` (0–2047).
  - Exposed as 7-bit `ENVX` (`value >> 4`).
  - Supports both **ADSR** and **GAIN** modes.
  - States: `Attack`, `Decay`, `Sustain`, `Release`.
  - Timing uses a `should_update_at_rate(global_counter, rate)` helper that looks up a **period** and **offset** from DSP hardware tables (`DSP_PERIOD_TABLE`, `DSP_OFFSET_TABLE`).
  - ADSR behaviors:
    - Attack: linear `+32` (or `+1024` for max rate 15 → rate 31).
    - Decay / Sustain: exponential decay (`value -= 1 + (value >> 8)`).
    - Release: linear `-8` per sample.
  - GAIN behaviors (when ADSR disabled):
    - Fixed, Linear Decay, Exponential Decay, Linear Increase, Bent Increase (+32 below 0x600, +8 above).

- **`AudioRingBuffer<const N: usize>`:**
  - Simple fixed-size ring buffer storing the last `N` `i16` samples.
  - Used by `Voice` for `envx_buffer` and `outx_buffer` (debug/diagnostic only).

### 3. `PitchGenerator` (pitch.rs)

Implements the S-DSP’s **Gaussian interpolation** for arbitrary pitch playback.

- **Internal Buffer:** Circular 12-sample window (`[i16; 12]`).
- **`PitchCounter`:** Wraps a `u16` counter.
  - Top 2 bits select which 4-sample quadrant of the 12-sample buffer is active.
  - Next 8 bits are the fractional interpolation index.
  - Counter wraps modulo `0xC000` (3 quadrants * 0x4000).
  - `add_detect_4byte_cross` detects when incrementing by `pitch` crosses a 4-sample boundary, triggering a BRR block refill.
- **Interpolation:**
  - For each output sample, selects 4 taps from the buffer based on `fractional`.
  - Coefficients come from a hard-coded 512-entry `GAUSSIAN_TABLE` (symmetric around index 0x100).
  - Each tap is `(sample * coeff) >> 11`, summed with saturating arithmetic, then clamped to `i16`.
- **Refill:** When crossing a 4-byte boundary, consumes the next 4 samples from the BRR iterator and writes them into the appropriate quadrant of the 12-sample buffer.

### 4. `BrrDecoder` (brr.rs)

Decodes SNES **BRR** (Bit Rate Reduction) compressed sample blocks.

- **BRR Block Format:** 9 bytes = 1 header byte + 16 nibbles (8 bytes) of 4-bit samples.
- **`BrrBlockHeader`** (`bilge` bitfield):
  - Bit 7 (`end`): End-of-sample flag.
  - Bit 6 (`loop_flag`): If set with `end`, sample loops to `loop_addr`.
  - Bits 4-5 (`filter`): 2-bit IIR filter selector (0–3).
  - Bits 0-3 (`left_shift`): Range / left-shift amount.
- **Decode Process:**
  1. Convert each 4-bit nibble to signed `i16` (`i4_to_i16`).
  2. Left-shift by `left_shift` (with sign extension via `overflowing_shl` / `overflowing_shr(1)`).
  3. Apply IIR filter using previous two output samples (`buffer: [i16; 2]`):
     - Filter 0: none.
     - Filter 1: `15/16 * z⁻¹`.
     - Filter 2: `61/32 * z⁻¹ - 15/16 * z⁻²`.
     - Filter 3: `115/64 * z⁻¹ - 13/16 * z⁻²`.
- **Streaming:**
  - `BrrDecoder` maintains a `VecDeque<i16>` (`current_block`) of decoded samples.
  - `next_sample` lazily decodes the next 9-byte block from APU memory when the buffer empties.
  - On `end` + `loop_flag`, resets decoder state and jumps to `loop_addr`.
  - Provides `iter()` returning a `BrrIterator` for integration with `PitchGenerator`.

## Data Flow per Sample Tick

1. **SDsp::generate_sample**
   → Advance noise generator.
   → For each voice:
     - If `trigger_on`: initialize BRR decoder & pitch generator from sample directory.
     - If `trigger_off`: enter Release envelope state.
     - Update envelope (ADSR or GAIN tables).
     - Read next sample (noise or BRR→PitchGenerator).
     - Apply envelope (`>> 11`).
     - Apply stereo volume (`>> 7` per channel).
     - Return mono voice output.
   → Saturating-sum all 8 voices into final `i16`.
   → Increment global counter.

## Key Types and Structures

| Type | File | Purpose |
|------|------|---------|
| `SDsp` | `mod.rs` | Top-level DSP state + register interface |
| `Voice` | `voice.rs` | Per-channel synthesis state |
| `DspEnvelope` | `voice.rs` | ADSR/GAIN envelope processor |
| `EnvelopeState` | `voice.rs` | Attack / Decay / Sustain / Release |
| `Adsr1` / `Adsr2` | `voice.rs` | Bit-packed ADSR control registers |
| `Gain` / `GainMode` | `voice.rs` | Custom envelope mode parsing |
| `PitchGenerator` | `pitch.rs` | Gaussian-interpolated resampler |
| `PitchCounter` | `pitch.rs` | Fixed-point buffer position tracker |
| `BrrDecoder` | `brr.rs` | BRR block decompressor |
| `BrrBlock` / `BrrBlockHeader` | `brr.rs` | BRR block layout |
| `BrrIterator` | `brr.rs` | Lending iterator over decoded samples |
| `NoiseGenerator` | `mod.rs` | LFSR white-noise source |
| `Flg` | `mod.rs` | DSP flags register |
| `AudioRingBuffer<N>` | `voice.rs` | Debug ring buffer for recent samples |
| `SDspDebug` | `mod.rs` | Read-only debug accessor for UI / inspection |

## Conventions & Patterns

- **Bitfields:** Extensive use of `bilge::prelude::*` (`#[bitsize(8)]`, `#[derive(DebugBits, FromBits)]`) for hardware registers (`Adsr1`, `Adsr2`, `BrrBlockHeader`, `Flg`).
- **Bit manipulation:** `intbits::Bits` is used throughout for nibble extraction, bit tests, and field slicing (e.g., `value.bits(4..7)`).
- **Saturating arithmetic:** Audio mixing uses `saturating_add` to prevent wrap-around distortion when summing voices.
- **Fixed-point math:** Pitch is a 14-bit unsigned value; pitch counter is 16-bit with implicit fractional component.
- **Direct memory access:** The DSP reads APU RAM as a flat `&[u8]` slice. The sample directory lives at page `dir * 0x100`; each entry is 4 bytes (start addr LE, loop addr LE).
- **Golden-file testing:** Both `brr.rs` and `voice.rs` contain `#[cfg(test)]` modules that decode known BRR files and compare WAV output against checked-in golden `.wav` files (`compare_wav_against_golden`).
- **Dead-code allowances:** `#![allow(dead_code)]` appears in several modules because diagnostic fields / helper functions are present for future debugging but not yet fully wired.

## Integration Notes

- The S-DSP is instantiated inside the APU (S-SMP) subsystem. It receives register writes from the S-SMP I/O ports and requests sample data from APU RAM.
- The `generate_sample` function should be called once per output sample (32 kHz on real hardware). The mixing result (`i16`) is typically fed into an output ring buffer and then to the host audio backend.
- Echo / FIR filtering (not yet present in this implementation) would normally occur after voice mixing, using echo buffer RAM and additional DSP registers (`$0D`–`$1F`, `$2C`–`$3C`, `$4D`, `$6D`, `$7D`).
