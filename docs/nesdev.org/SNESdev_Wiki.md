---
title: "SNESdev Wiki"
source_url: "https://snes.nesdev.org/wiki/SNESdev_Wiki"
pageid: 1
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

**SNES Development Wiki**

## Reference

### General

- [[Memory map]]
- [[ROM header]]
- [[CPU vectors]]
- [[SNES Development Manual]]
- [[65C816]] - SNES main CPU, part of the S-CPU
- [[Tools]]
- [[Timing]]
- [[Errata]]
- [[Glossary]]

### Registers

- [[MMIO registers]]
- [[PPU registers]]
- [[DMA registers]]

### Pinouts

- [[S-CPUN Pinout|1CHIP Pinout]]
- [[APU pinout]]
- [[CPU pinout]]
- [[PPU pinout]]
- [[DSP Pinout|DSP Coprocessor Pinout]]
- [[SA-1 Pinout|SA-1 Coprocessor Pinout]]
- [[S-RGB Pinout]]
- [[S-ENC Pinout]]
- [[S-MIX Pinout]]
- [[CIC Pinout]]
- [[MAD-1 Pinout]]
- [[WRAM pinout]]
- [[Cartridge connector]]
- [[Controller connector|Controller Port]]

### Peripherals

- [[Standard controller]]
- [[Mouse]]
- [[Multitap]]
- [[Super Scope]]
- [[NTT Data Keypad]]
- [[Copier]]
- [Turbo File Twin](https://snes.nesdev.org/w/index.php?title=Turbo_File_Twin&action=edit&redlink=1 "Turbo File Twin (page does not exist)")
- [Voyager-kun/Mr. Voice](https://snes.nesdev.org/w/index.php?title=Voyager-kun/Mr._Voice&action=edit&redlink=1 "Voyager-kun/Mr. Voice (page does not exist)")
- [Sufami Turbo](https://snes.nesdev.org/w/index.php?title=Sufami_Turbo&action=edit&redlink=1 "Sufami Turbo (page does not exist)")

### PPU

- [[Backgrounds]]
- [[Tilemaps]]
- [[Tiles]]
- [[Sprites]]
- [[Palettes]]
- [[Windows]]
- [[Offset-per-tile]]
- [[Color math]]

### Sound

- [[S-SMP]] - SNES SHVC-SOUND Board includes the S-SMP / SPC-700 CPU and S-DSP
- [[SPC-700 instruction set]]
- [[S-DSP registers]] - Not to be confused with the DSP-x Coprocessors
- [[DSP envelopes]]
- [[BRR samples]]

### Expansions

- [[DSP Expansion|DSP-x]] (Not to be Confused with S-DSP Audio Chip)
- [SA-1](https://snes.nesdev.org/w/index.php?title=SA-1&action=edit&redlink=1 "SA-1 (page does not exist)")
- [[Super FX]] (GSU)
- [MSU-1](https://snes.nesdev.org/w/index.php?title=MSU-1&action=edit&redlink=1 "MSU-1 (page does not exist)")
- [Super Game Boy](https://snes.nesdev.org/w/index.php?title=Super_Game_Boy&action=edit&redlink=1 "Super Game Boy (page does not exist)")
- [[CX4]] used in *Mega Man X2*/*X3*

### Formats

- [[ROM file formats]]
- [[Save file formats]]
- [SPC File Format](https://snes.nesdev.org/w/index.php?title=SPC_file_format&action=edit&redlink=1 "SPC file format (page does not exist)") - For Music that can run Entirely on the SPC. [reference](https://wiki.superfamicom.org/nintendo-music-format-(n-spc))
- [SNSF File Format](https://snes.nesdev.org/w/index.php?title=SNSF_file_format&action=edit&redlink=1 "SNSF file format (page does not exist)") - For Music that needs the Main CPU as well. [reference](https://www.vgmpf.com/Wiki/index.php/SNSF), [spec](https://snsf.caitsith2.net/snsf%20spec.txt)

## Examples and Guides

### General

- [[Tutorials]]

### SNES hardware

- [[Init code]]
- [[VBlank interrupts]]
- [[Booting the SPC700]]
- [[Controller reading]]
- [[Multiplication]]
- [[Division]]
- [[DMA examples]]
- [[Blargg SPC upload]] - Playing an SPC rip on SNES Hardware

### 65c816 guides

- [[65c816 for 6502 developers|65C816 for 6502 Devs]]
- [[Using X as a pointer]]
- [[MVN and MVP block copy]]
- [[Register sizes in ca65|Reg. Sizes in CA65]]
- [[Signature byte]] - supplying a parameter byte for BRK/COP interrupts or WDM
- [[Struct register tradeoffs]] - explores different ways to implement "this"

### Emulation

- [[Emulator tests]]
- [[Tricky-to-emulate games]]
- [[Uncommon graphics mode games|Games with Uncommon Graphics Modes]]

### Video

- [[VBlank routine]]
- [[SNES PPU for NES developers]]
- [[Scrolling a large map]]
- [[Drawing window shapes]]
- [[HDMA examples]]
- [[Reading and writing PPU memory]]
- [[Mode 7 perspective effects|Mode 7 Effects]]
- [Starting HDMA mid-frame](https://snes.nesdev.org/w/index.php?title=Starting_HDMA_mid-frame&action=edit&redlink=1 "Starting HDMA mid-frame (page does not exist)")
- [[Variable width fonts]]
- [Extending vblank](https://undisbeliever.net/snesdev/registers/inidisp.html#extended-vblank)

## Links

- [SNESdev Forum](https://forums.nesdev.org/viewforum.php?f=12) - NESDev subforum
- [Fullsnes](https://problemkaputt.de/fullsnes.htm) - Nocash's SNES hardware document
- [Superfamicom.org SNES Development Wiki](https://wiki.superfamicom.org/)
- [Super NES Programming Wikibooks](https://en.wikibooks.org/wiki/Super_NES_Programming)
- [Superfamicom.org SNES cartridge database](https://superfamicom.org/)
- [SNES Central](https://snescentral.com/) - game database and PCB images
- [Anomie's SNES documents](https://www.romhacking.net/community/548/) at RHDN

## MediaWiki

- [User's Guide](https://www.mediawiki.org/wiki/Special:MyLanguage/Help:Contents)
- [MediaWiki FAQ](https://www.mediawiki.org/wiki/Special:MyLanguage/Manual:FAQ)
- [[Category:Deletion requests|Deletion requests]]
