---
title: "Version differences"
source_url: "https://snes.nesdev.org/wiki/Version_differences"
pageid: 183
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Nintendo stopped incrementing version numbers after 2/1/3.

There is no known method to programmatically detect 1-CHIP consoles (the known differences are graphical in nature).

See also:

- [[Errata]] for a list hardware bugs that affect multiple chip revisions

## S-CPU

### S-CPU (5A22-01)

**RDNMI version**: 1

Known bugs:

- Crashes if a DMA finishes just before HDMA happens

### S-CPU A (5A22-02)

**RDNMI version**: 2

This revision:[[1]](#cite_note-1)

- Fixes the DMA/HDMA crash
- Changes the S-WRAM refresh position
- Changes the HDMA setup position

Known bugs:

- A recent HDMA transfer with BBADn=0 can make a DMA transfer fail.

### S-CPU B (5A22-02)

**RDNMI version**: 2

This revision:

- Fixes the DMA failure after a recent BBADn=0 HDMA transfer bug

### S-CPUN A (RF5A122)

**RDNMI version**: 2

Also known as 1-CHIP.

This revision:

- Combines S-CPU, S-PPU1 and S-PPU2 onto a single IC

## S-PPU1 (5C77-01)

**STAT77 version**: 1

There is only 1 known version of the S-PPU1.

It is unknown if the S-PPU1 in a 3-chip console is the same as the PPU1 in the S-CPUN A 1CHIP console.

## S-PPU2

### S-PPU2 (5C78-01)

**STAT78 version**: 1

### S-PPU2 A (5C78-02)

**STAT78 version**: 2

This is the rarest revision of the S-PPU2.

### S-PPU2 B (5C78-03)

**STAT78 version**: 3

### S-PPU2 C (5C78-03)

**STAT78 version**: 3

### S-CPUN A (RF5A122)

**STAT78 version**: 3

Also known as 1-CHIP.

Found in 1-CHIP and SNES Jr motherboards.

This revision:

- Combines S-CPU, S-PPU1 and S-PPU2 onto a single IC
- The DAC has been changed and the image is sharper.[[2]](#cite_note-2)
- The INIDISP early read bug does not corrupt tile slivers.[[3]](#cite_note-3)

Known bugs:

- Glitches on the first visible scanline on some games that extend VBlank with force-blank.[[4]](#cite_note-4)
- Brightness DAC has large rise-time.[[5]](#cite_note-5)
- Reports of ghosting
- The output signal is too bright.[[6]](#cite_note-6)
- The SNS-CPU-1CHIP-03 motherboard does not output a composite sync signal.[[7]](#cite_note-7)

## Audio

### SHVC-SOUND

The first revision of the SNES's motherboard had a separate SHVC-SOUND module.

### On motherboard

In 1992 Nintendo moved the SHVC-SOUND chips onto the SNS-CPU-GPM-01 motherboard.

This revision replaces the S-DSP chip with the S-DSP A chip. Any differences between the two revisions are unknown.

### S-APU

Found on SNS-CPU-APU-01 and 1CHIP motherboards, the S-APU combines the S-SMP, S-DSP and Audio-RAM into a single S-APU IC.

The S-APU fixes the S-SMP timer glitch.[[8]](#cite_note-8)

### DAC

Nintendo used 3 different DAC chips:

- NEC µPD6376, in NTSC motherboards dated 1990-1994 (SHVC-SOUND, SNS-CPU-GPM-01, SNS-CPU-GPM-02, SNS-CPU-RGB-01)
- NEC 6376, in PAL motherboards dated 1992 (SNSP-CPU-01, SNSP-CPU-02)
- NEC 6379A, in motherboards dated 1995 or 1997 (SNS-CPU-RGB-02, SNS-CPU-APU-01, SNS-CPU-1CHIP-02, SNN-CPU-01, SNSP-CPU-1CHIP-01, SNSP-CPU-1CHIP-02)

Consoles with the NEC µPD6376 DAC have a slight audio imbalance. Nintendo tied the 2 Vref pins to a single single 47μF capacitor, while the µPD6376 datasheet shows the 2 Vref pins are supposed to be connected to 2 separate capacitors.[[9]](#cite_note-9)

## References

- [Console5 Tech Wiki - SNES](https://wiki.console5.com/wiki/SNES) - List of Chips, ICs and capacitors for the various SNES motherboards.

1. [↑](#cite_ref-1) ares source code, ares/ares/sfc/cpu/timing.cpp, by Near and ares team
2. [↑](#cite_ref-2) [1st-gen Super Famicom / SNES vs. Super Famicom Jr. / SNES Jr](https://www.chrismcovell.com/gotRGB/snesblur.html) by Chris Covell
3. [↑](#cite_ref-3) [INIDISP Register - INIDISP Early Read Glitch](https://undisbeliever.net/snesdev/registers/inidisp.html#early-read-glitch) by undisbeliever
4. [↑](#cite_ref-4) [Demon's Crest 1CHIP glitching comparison](https://www.youtube.com/watch?v=1Lm0mhOhtzM) by eightbitminiboss (YouTube video)
5. [↑](#cite_ref-5) [INIDISP Register - Brightness Delay](https://undisbeliever.net/snesdev/registers/inidisp.html#brightness-delay-glitch) by undisbeliever
6. [↑](#cite_ref-6) [RetroRGB - SNES 1CHIP](https://www.retrorgb.com/snes1chip.html) Brightness / Signal correction
7. [↑](#cite_ref-7) [About ConsoleMods Wiki - SNES:SNES Model Differences](https://consolemods.org/wiki/SNES:SNES_Model_Differences#%E2%80%9CSNS-CPU-1CHIP-01/02/03%E2%80%9D_(1995-1997))
8. [↑](#cite_ref-8) [Forum post](https://forums.nesdev.org/viewtopic.php?p=240687#p240687): list of version differences by Near
9. [↑](#cite_ref-9) [ConsoleMods Wiki - SNES:Audio Balance Fix](https://consolemods.org/wiki/SNES:Audio_Balance_Fix)
