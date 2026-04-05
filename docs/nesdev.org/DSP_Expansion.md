---
title: "DSP Expansion"
source_url: "https://snes.nesdev.org/wiki/DSP_Expansion"
pageid: 160
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The **DSP** series of coprocessors. **DSP-1** was used in many games for computational support, often to assist games with 3D features.
**DSP-2**, **DSP-3**, and **DSP-4** were used in a single game each. This family of chips shares a similar interface, but they differ in their command functions.

The SNES CPU would write computation commands to the DSP's memory mapper registers, then read back the computed result.

DSP is an acronym for "Digital Signal Processor", but the name seems poorly chosen. It does not operate on a continuous signal like most [DSP](https://en.wikipedia.org/wiki/Digital_signal_processor)s.

Not to be confused with the [[S-SMP|S-DSP]] sound device built into the SNES.

| Game | Chip |
| --- | --- |
| *Soukou Kihei Votoms: The Battling Road* | DSP-1 |
| *Bike Daisuki! Hashiriya Kon - Rider's Spirits* | DSP-1 |
| *Final Stretch* | DSP-1 |
| *Lock On* / *Super Air Diver* | DSP-1 |
| *Michael Andretti's Indy Car Challenge* | DSP-1/1A |
| *Pilotwings* | DSP-1/1B |
| *Shutokou Battle '94: Keichii Tsuchiya Drift King* | DSP-1B |
| *Shutokou Battle 2: Drift King Keichii Tsuchiya & Masaaki Bandoh* | DSP-1B |
| *Suzuka 8 Hours* | DSP-1 |
| *Super Air Diver 2* | DSP-1 |
| *Super Bases Loaded 2* / *Super 3D Baseball* / *Korean League* | DSP-1 |
| *Super F1 Circus Gaiden* | DSP-1 |
| *Battle Racers* | DSP-1 |
| *Super Mario Kart* | DSP-1/1B |
| *Ace o Nerae! 3D Tennis* | DSP-1A |
| *Ballz 3D* | DSP-1B |
| *Dungeon Master* | DSP-2 |
| *SD Gundam GX* | DSP-3 |
| *Top Gear 3000* / *The Planet's Champ TG 3000* | DSP-4 |

## References

- [[SNES Development Manual]]: Book II 3-1-1
- [bsnes dsp1](https://github.com/bsnes-emu/bsnes/tree/master/bsnes/sfc/coprocessor/dsp1) - emulator source
- [bsnes dsp2](https://github.com/bsnes-emu/bsnes/tree/master/bsnes/sfc/coprocessor/dsp2) - emulator source
- [bsnes dsp4](https://github.com/bsnes-emu/bsnes/tree/master/bsnes/sfc/coprocessor/dsp4) - emulator source
- [Snes9x dsp1](https://github.com/snes9xgit/snes9x/blob/master/dsp1.cpp) - emulator source
- [Snes9x dsp2](https://github.com/snes9xgit/snes9x/blob/master/dsp2.cpp) - emulator source
- [Snes9x dsp3](https://github.com/snes9xgit/snes9x/blob/master/dsp3.cpp) - emulator source
- [Snes9x dsp4](https://github.com/snes9xgit/snes9x/blob/master/dsp4.cpp) - emulator source
- [List of Super NES enhancement chips: DSP](https://en.wikipedia.org/wiki/List_of_Super_NES_enhancement_chips#DSP) - Wikipedia article
