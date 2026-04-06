# Fullsnes — SNES hardware reference (split markdown)

This folder contains **no$sns** / **Fullsnes** reference text (Martin Korth, nocash), split from the monolithic spec into one markdown file per section for easier reading and linking. The material covers the SNES and closely related topics (cartridge coprocessors, peripherals, arcade/kiosk variants, and CPU references used by those add-ons).

- **Credits, sources, and upstream:** [About/Credits](about-credits.md) — also links to the [HTML spec](https://problemkaputt.de/fullsnes.htm) and [plain-text snapshot](https://problemkaputt.de/fullsnes.txt).

## SNES Documentation Overview

- **Main CPU (65816 / 65xx family):** [CPU 65XX Microprocessor](cpu-65xx-microprocessor.md) → registers, addressing, opcodes, cycles, glitches ([CPU Registers and Flags](cpu-registers-and-flags.md), [CPU Memory Addressing](cpu-memory-addressing.md), [CPU Clock Cycles](cpu-clock-cycles.md), …).
- **Memory map and bus behavior:** [SNES Memory](snes-memory.md), [SNES Memory Map](snes-memory-map.md), [SNES Memory Control](snes-memory-control.md), [SNES I/O Map](snes-i-o-map.md); WRAM/OAM/VRAM/CGRAM access: files under `snes-memory-*`.
- **DMA / HDMA:** [SNES DMA Transfers](snes-dma-transfers.md) → [Start/Enable registers](snes-dma-and-hdma-start-enable-registers.md), [Channel 0–7 registers](snes-dma-and-hdma-channel-0-7-registers.md), [DMA/HDMA notes](snes-dma-and-hdma-notes.md).
- **PPU:** [SNES Picture Processing Unit (PPU)](snes-picture-processing-unit-ppu.md) → [PPU Control](snes-ppu-control.md), [BG Control](snes-ppu-bg-control.md), [VRAM](snes-ppu-video-memory-vram.md), [Sprites (OBJs)](snes-ppu-sprites-objs.md), [CGRAM / direct color](snes-ppu-color-palette-memory-cgram-and-direct-colors.md), [Window](snes-ppu-window.md), [Color math](snes-ppu-color-math.md), [Mode 7 / rotation-scaling](snes-ppu-rotation-scaling.md), [Resolution](snes-ppu-resolution.md), [Offset-per-tile](snes-ppu-offset-per-tile-mode.md), [Timers & status](snes-ppu-timers-and-status.md), [PPU interrupts](snes-ppu-interrupts.md).
- **APU (SPC700 + S-DSP):** [SNES Audio Processing Unit (APU)](snes-audio-processing-unit-apu.md) → [APU memory & I/O map](snes-apu-memory-and-i-o-map.md), [SPC700 overview and opcode groups](snes-apu-spc700-cpu-overview.md), [Main CPU ↔ APU ports](snes-apu-main-cpu-communication-port.md), DSP topics (`snes-apu-dsp-*`), [Low-level timings](snes-apu-low-level-timings.md).
- **Hardware multiply/divide (CPU side):** [SNES Maths Multiply/Divide](snes-maths-multiply-divide.md).
- **Scanline / dot timing:** [SNES Timings](snes-timings.md) → [Oscillators](snes-timing-oscillators.md), [H/V counters](snes-timing-h-v-counters.md), [H/V events](snes-timing-h-v-events.md), [PPU memory access timing](snes-timing-ppu-memory-accesses.md).
- **Controllers & automatic joypad read:** [SNES Controllers](snes-controllers.md) → [Automatic reading](snes-controllers-i-o-ports-automatic-reading.md), [Manual reading](snes-controllers-i-o-ports-manual-reading.md); specialty controllers are separate `snes-controllers-*` files (mouse, Super Scope, Multitap, etc.).
- **Cartridges, ROM layout, mappers:** [SNES Cartridges](snes-cartridges.md) → [ROM header](snes-cartridge-rom-header.md), [ROM image headers & extensions](snes-cartridge-rom-image-headers-and-file-extensions.md), [Interleave](snes-cartridge-rom-image-interleave.md), [LoROM](snes-cart-lorom-mapping-rom-divided-into-32k-banks-around-1500-games.md), [HiROM](snes-cart-hirom-mapping-rom-divided-into-64k-banks-around-500-games.md). Coprocessors and odd carts: many `snes-cart-*` pages (SA-1, Super FX/GSU, DSP, CX4, S-RTC, S-DD1, SPC7110, Satellaview, Super Game Boy, flash carts, cheat devices, copiers, etc.) — see [Document index](document-index.md) under the `snes-cart-*` entries.
- **CIC / lockout:** [SNES Cartridge CIC (lockout chip)](snes-cartridge-cic-lockout-chip.md) and related `snes-cartridge-cic-*` files.
- **Decompression (general SNES formats):** [SNES Decompression Formats](snes-decompression-formats.md); chip-specific algorithms appear under the relevant `snes-cart-*` coprocessor pages.
- **Pinouts, power, connectors:** [SNES Pinouts](snes-pinouts.md) → CPU/PPU/APU/chip-specific `snes-pinouts-*` files, [Cartridge slot](snes-cartridge-slot-pinouts.md), [Expansion port](snes-expansion-port-ext-pinouts.md), [AV connector](snes-audio-video-connector-pinouts.md), [Power supply](snes-power-supply.md), [Chipset overview](snes-chipset.md).
- **Edge cases & undocumented behavior:** [SNES Unpredictable Things](snes-unpredictable-things.md).

## Other systems documented in the same corpus

These are **not** vanilla retail SNES consoles but appear in the same specification package:

- **Nintendo Super System (NSS — arcade / kiosk):** `nss-*` files (memory maps, I/O, BIOS, tokens, controls).
- **SFC-Box (Japanese store demo / kiosk hardware):** `sfc-box-*` files (HD64180-based coprocessor, maps, OSD, GROM).
- **Z80 CPU reference:** `z80-*` files (instruction set and flags; relevant where a Z80-class core appears in peripherals or docs).
- **HD64180 (Z80 extension):** `hd64180*` files — used with SFC-Box and as a general reference.
- **ARM (e.g. SETA ST018 cartridge):** [SNES Cart Seta ST018 (pre-programmed ARM CPU)](snes-cart-seta-st018-pre-programmed-arm-cpu-1-game.md) plus `arm-*` instruction set pages.
- **RTC S-3520:** [RTC S-3520 (Real-Time Clock)](rtc-s-3520-real-time-clock.md) (also referenced from cartridge context).

## Miscellaneous SNES topics in this folder

- Add-ons and accessories: `snes-add-on-*` (e.g. Turbo File, modem, barcode, voice/IR).
- Hotel / arcade variants: [SNES Hotel Boxes and Arcade Machines](snes-hotel-boxes-and-arcade-machines.md).
- Homebrew / dev conveniences: [SNES Xboo Upload (WRAM Boot)](snes-xboo-upload-wram-boot.md), [SNES Common Mods](snes-common-mods.md), [SNES Controller Mods](snes-controller-mods.md).
- 3D glasses: [SNES 3D Glasses](snes-3d-glasses.md).
