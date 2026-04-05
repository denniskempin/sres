---
title: "BRR samples"
source_url: "https://snes.nesdev.org/wiki/BRR_samples"
pageid: 109
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Sound samples played by the [[S-SMP]] DSP are stored in the [BRR](https://en.wikipedia.org/wiki/Bit_Rate_Reduction) (bit-rate-reduction) data format.

BRR is composed of 16-sample blocks, each of which is stored in 9 bytes of data. This is a 1 byte control block, followed by 8 bytes containing 16 4-bit samples to be decoded (high nibble first).

| Offset | Bits | Notes |
| --- | --- | --- |
| 0 | SSSS FFLE | Left-shift (S), decoding filter (F), loop (L), end (E). |
| 1 | AAAA BBBB | Samples 0 (A) and 1 (B). |
| 2 | CCCC DDDD | Samples 2 (C) and 3 (D). |
| 3 | EEEE FFFF | Samples 4 (E) and 5 (F). |
| 4 | GGGG HHHH | Samples 6 (G) and 7 (H). |
| 5 | IIII JJJJ | Samples 8 (I) and 9 (J). |
| 6 | KKKK LLLL | Samples 10 (K) and 11 (L). |
| 7 | MMMM NNNN | Samples 12 (M) and 13 (N). |
| 8 | OOOO PPPP | Samples 14 (O) and 15 (P). |

When a block with the end (E) flag set finishes, the channel will automatically stop unless the loop (L) flag is also set for this block. If the loop flag is set, the loop point will be read based on the channel's current **SCRN** and sample playback will continue from there. Because of the block structure, loop points must lie on 16-sample boundaries. The loop flag has no effect on blocks without the end flag set.

The left-shift (S) value specifies the overall magnitude of values. This is simply a left shift applied to each sample nibble before BRR filter decoding.

Each sample nibble is a signed 4-bit value in the range of -8 to +7. After being shifted, one of four BRR decoding filters (F) is applied, which may include the last 0-2 decoded samples in the result. Filter 0 will just use the shifted nibble directly.

Sound samples should normally start with a filter-0 block, to prevent leftover BRR samples from the previously playing sample from having an effect on decoding.

### Filters

The four BRR filters form an [adaptive PCM](https://en.wikipedia.org/wiki/Adaptive_differential_pulse-code_modulation) (ADPCM) encoding scheme, using the previous few samples to predict the next one with a low amount of input data. By choosing the best fitting filter (i.e. adapting) for each block of 16 samples, the result can more closely match a desired waveform.

Each filter adds the shifted nibble to two previously decoded samples, each multiplied by a coefficient:

| Filter | Shifted Nibble | Sample -1 | Sample -2 |
| --- | --- | --- | --- |
| 0 | 1 | 0 | 0 |
| 1 | 1 | 15/16 | 0 |
| 2 | 1 | 61/32 | 15/16 |
| 3 | 1 | 115/64 | 13/16 |

Filter 0 is the non-filter, which just uses the shifted nibble result directly. The previous samples are not used. Sound samples should normally begin with a filter 0 block to prevent leftover results from the previous sound affecting the new one.

Filter 1 includes some of the previously encoded sample.

Filters 2 and 3 include some of two previously encoded samples.

As the filter number increases, the high frequency bandwidth (sharpness) of the 4-bit sample encoding is traded for better low frequency (smoothness) encoding. Because most natural sounds have stronger low frequencies and weaker high frequencies, this allows the bandwidth allocation of each block to be adapted to better suit the frequency spectrum of the sound at that moment.

### Gaussian Interpolation

Finally, after the BRR sample data is decoded, when a sample is played back, because the rate (P) is adjustable, an filter is used to smoothly interpolate values between the decoded samples. This is a gaussian interpolator which blends a combination of a neighbourhood of 4 adjacent samples according to a lookup table. This reduces high frequency aliasing across a range of playback rates.

Playback begins with the interpolator centred on the second sample. If your key-on does not begin with an ADSR attack envelope, you may wish to have three 0 samples at the beginning of your sound sample to ensure it begins at 0 and smoothly moves from there (avoiding clicks/pops).

Because of a bug in the DSP's gaussian interpolation filter, 3 maximum-negative values in a row can cause an overflow of the resulting sample, creating a loud pop. This can be intentionally exploited, but normally it is advised avoid this.

## Links

- [Bit Rate Reduction](https://wiki.superfamicom.org/bit-rate-reduction-(brr)) - Superfamicom.org wiki article, documents BRR sample format.
- [SNES APU DSP BRR Samples](http://problemkaputt.de/fullsnes.htm#snesapudspbrrsamples) - Fullsnes documentation of BRR sample format.
