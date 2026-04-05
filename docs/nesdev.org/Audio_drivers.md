---
title: "Audio drivers"
source_url: "https://snes.nesdev.org/wiki/Audio_drivers"
pageid: 173
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

An '*audio driver* is a program that handles playing music and sound effects for a game running on a game console. In a SNES game, music is normally encoded into a sequence of notes that an audio driver then interprets, keeping track of timers in order to start and stop notes at the right times. The SNES has a dedicated processor for running audio drivers (the [[S-SMP]]) and the main program running on the 65c816 has to communicate with the audio driver on the S-SMP in order to ask it to play sound effects, switch which song is currently being played, and other tasks.

Audio drivers vary in what features they support and the way composers are intended to write music for them. Some audio drivers have streaming features, where the main program can provide new sample data or music sequence data during gameplay. Nintendo provided licensed developers with an audio driver named Kankichi-kun (known as [N-SPC](https://sneslab.net/wiki/N-SPC_Engine)) that many game studios customized to their needs, but using it without a license would be copyright infringement. Thankfully there are several audio drivers that are licensed for homebrew game use.

Quick driver comparison

|  | SNESGSS | SNESMOD | Terrific Audio Driver | qSPC | XMSNES | SNES-ProTrackerPlayer | Super Kannagi Sound |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Music channels | 8 | 8 | 8 | 8 | 8 | 6 | 8 |
| Sound effect channels | 8 | 2 | 2 | 2 | 8 | 2 | 4 |
| SFX/Music channel sharing | No | Yes | Yes | No | Yes | No | Yes? |
| Sequenced sound effects | Yes | No | Yes | Yes | No | No | No |
| Sound effect channel allocation | Manual | Auto | Auto | Manual | Auto | Auto | Manual |
| Music input format | Custom tracker | IT file | MML | MML | MOD/S3M/XM file | MOD file | MOD/S3M/XM/IT file? |

### SNESGSS

[SNESGSS](https://shiru.untergrund.net/software.shtml) uses its own custom tracker, which can show how much audio RAM is in use as the composer works on the song.

- Arbitrary number of music and sound effects, but cannot share channels between music and sound effects at the same time
- Supports the SNES's [[DSP envelopes|ADSR envelopes]]
- Audio RAM holds the driver, instrument data, sound effect sequences, and a single song. The main program uploads the data for a new song whenever it wants to play a different one.
- Sequenced sound effect support
- Audio streaming feature

See <https://nesdoug.com/2020/06/14/snes-music/> for a guide, as well as <https://github.com/NovaSquirrel/snesgss-extended> which is a fork that adds new features (but has unresolved bugs in its exporter). There is a known bug which can cause the console to lock up[[1]](#cite_note-1) but there are patched versions which fix this.

### SNESMOD

[SNESMOD](https://github.com/mukunda-/snesmod) plays Impulse Tracker files, with limitations on what effects are supported. See [snesmod\_music.txt](https://github.com/mukunda-/snesmod/blob/master/snesmod_music.txt) for more details.

- Sound effects are just samples, but sequenced sound effect support is available in a patched version
- Audio streaming feature

See <https://nesdoug.com/2022/03/02/snesmod/> for a guide.

### Terrific Audio Driver

[Terrific Audio Driver](https://github.com/undisbeliever/terrific-audio-driver) plays songs written with MML, and it provides its own MML editor which allows composers to preview the music while working on it.

- Can play music on all 8 music channels, with 2 channels interrupted as needed for sound effects
- [Elaborate sound effect support](https://github.com/undisbeliever/terrific-audio-driver/blob/v0.0.10/docs/sound-effects.md)
  - Can be played with different panning
  - Sequenced sound effect support
  - Each sound effect can be limited to playing on one channel at a time, so that sound effects that will play frequently won't take up both channels
  - Each sound effect can be marked as uninterruptible, so sounds like voice samples can be guaranteed to play to completion
  - Priority system, so that background effects like footsteps and water droplets can be interrupted
- Sound effects can be played with different panning
- Supports the SNES's ADSR envelopes
- Lots of effects
  - Vibrato and portamento
  - Volume slides, tremolo, pan slides and panbrello
  - Detune
  - Left/right channel invert (if the audio mode is set to surround)
  - Commands for editing the echo buffer parameters in the middle of a song
- Has a custom audio loader that is faster than the [[Booting the SPC700|built-in loader]]
- Can load audio data across several frames, allowing the game to do it in the background while it does other tasks
- ca65, 64tass and pvsneslib APIs
- [Furnace-to-TAD song converter](https://github.com/NovaSquirrel/fur2tad) is available

### qSPC

[qSPC](https://github.com/gyuque/snes-qspc) plays songs written with MML. It was used in *Nekotako* and *Dottie dreads nought*.

### XMSNES

[XMSNES](https://github.com/osoumen/XMSNES) was made by the same person as SNESMOD and seems to be an older driver.

### SNES-ProTrackerPlayer

[SNES-ProTrackerPlayer](https://github.com/snesdev0815/SNES-ProTrackerPlayer) is a MOD player by snesdev0815.

### Super Kannagi Sound

Available as a part of Kannagi's SDK, [SNDK](https://github.com/Kannagi/SNDK/tree/main/tools/RetroTracker)

- Can play music on all 8 channels, with the ability to play sound effects on 4 channels
- Can fit 52KB of samples at once
- Reads XM, IT and S3M formats, but doesn't handle effects
- Has a fast loader

1. [↑](#cite_ref-1) <https://forums.nesdev.org/viewtopic.php?p=229738>
