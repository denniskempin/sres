---
title: "Emulator tests"
source_url: "https://snes.nesdev.org/wiki/Emulator_tests"
pageid: 94
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Test ROMs that may be helpful for developing an emulator.

## General

- [undisbeliever SNES Test ROMS](https://github.com/undisbeliever/snes-test-roms)
- [blargg hardware tests](https://snescentral.com/article.php?id=1115)
- [TASVideos SNES Accuracy Tests](https://tasvideos.org/Emulatorresources/SNESaccuracytests)

## CPU

- [Forum thread](https://forums.nesdev.org/viewtopic.php?t=24087): "Writing $4203 twice too fast gives erroneous result (not emulated)" - contains some tests for obscure multiplier behaviour, work in progress.

## PPU

| ROM | Author | Notes |
| --- | --- | --- |
| **[gradient-test](https://bin.smwcentral.net/u/1780/gradient-test.sfc)** | [NovaSquirrel](https://www.smwcentral.net/?p=files&u=1780) | Test of [[PPU registers#CGWSEL|CGWSEL]] to demonstrate inaccurate emulator behaviour. |
| **[Two Ship](https://forums.nesdev.org/viewtopic.php?p=279303)** | rainwarrior | Comparing mode 5 + interlacing against mode 1 graphics. |
| **[PPU bus activity](https://forums.nesdev.org/viewtopic.php?p=174494)** | lidnariq | Demonstrates all modes 0-6 on a single screen. |
| **[Elasticity](https://forums.nesdev.org/viewtopic.php?p=278742)** | rainwarrior | Mode 3 techniques for improving color depth beyond 8bpp. |

## Input

| ROM | Author | Notes |
| --- | --- | --- |
| **[ctrltest](https://github.com/bbbradsmith/SNES_stuff/tree/main/ctrltest)** | rainwarrior | Generic controller read test. |
| **[mset](https://github.com/bbbradsmith/SNES_stuff/tree/main/mset)** | rainwarrior | [[Mouse]] peripheral test. |

## See Also

- [[Tricky-to-emulate games]]
- [[Uncommon graphics mode games]]
