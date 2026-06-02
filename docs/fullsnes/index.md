---
summary: Index/overview of the Fullsnes (no$sns) SNES hardware reference documentation, organized into per-topic markdown files. Links to detailed pages covering the main CPU, memory map, DMA, PPU, APU, controllers, cartridges, coprocessors, and adjacent systems (NSS, SFC-Box, Z80/HD64180, ARM, RTC).
keywords: SNES, hardware reference, PPU, APU, DMA
importance: 5
---

# Fullsnes — SNES hardware reference (split markdown)

This folder contains **no$sns** / **Fullsnes** reference text (Martin Korth, nocash), split from the monolithic spec into one markdown file per section for easier reading and linking. The material covers the SNES and closely related topics (cartridge coprocessors, peripherals, arcade/kiosk variants, and CPU references used by those add-ons).

- **Credits, sources, and upstream:** [About/Credits](about_credits.md) — also links to the [HTML spec](https://problemkaputt.de/fullsnes.htm) and [plain-text snapshot](https://problemkaputt.de/fullsnes.txt).

## SNES Documentation Overview

- **Main CPU (65816 / 65xx family):** [CPU 65XX Microprocessor](cpu_65xx.md) → registers, addressing, opcodes, cycles, glitches ([CPU Registers and Flags](cpu_registers.md), [CPU Memory Addressing](cpu_addressing.md), [CPU Clock Cycles](cpu_cycles.md), …).
- **Memory map and bus behavior:** [SNES Memory](memory.md), [SNES Memory Map](memory_map.md), [SNES Memory Control](memory_control.md), [SNES I/O Map](io_map.md); WRAM/OAM/VRAM/CGRAM access: files under `snes-memory-*`.
- **DMA / HDMA:** [SNES DMA Transfers](dma.md) → [Start/Enable registers](dma_start_enable.md), [Channel 0–7 registers](dma_channel_regs.md), [DMA/HDMA notes](dma_notes.md).
- **PPU:** [SNES Picture Processing Unit (PPU)](ppu.md) → [PPU Control](ppu_control.md), [BG Control](ppu_bg.md), [VRAM](ppu_vram.md), [Sprites (OBJs)](ppu_sprites.md), [CGRAM / direct color](ppu_cgram.md), [Window](ppu_window.md), [Color math](ppu_color_math.md), [Mode 7 / rotation-scaling](ppu_rotation.md), [Resolution](ppu_resolution.md), [Offset-per-tile](ppu_offset_per_tile.md), [Timers & status](ppu_timers.md), [PPU interrupts](ppu_interrupts.md).
- **APU (SPC700 + S-DSP):** [SNES Audio Processing Unit (APU)](snes-audio-processing-unit-apu.md) → [APU memory & I/O map](snes-apu-memory-and-i-o-map.md), [SPC700 overview and opcode groups](snes-apu-spc700-cpu-overview.md), [Main CPU ↔ APU ports](snes-apu-main-cpu-communication-port.md), DSP topics (`snes-apu-dsp-*`), [Low-level timings](snes-apu-low-level-timings.md).
- **Hardware multiply/divide (CPU side):** [SNES Maths Multiply/Divide](maths.md).
- **Scanline / dot timing:** [SNES Timings](timings.md) → [Oscillators](timing_oscillators.md), [H/V counters](timing_counters.md), [H/V events](timing_events.md), [PPU memory access timing](timing_ppu_memory.md).
- **Controllers & automatic joypad read:** [SNES Controllers](controllers.md) → [Automatic reading](snes-controllers-i-o-ports-automatic-reading.md), [Manual reading](snes-controllers-i-o-ports-manual-reading.md); specialty controllers are separate `snes-controllers-*` files (mouse, Super Scope, Multitap, etc.).
- **Cartridges, ROM layout, mappers:** [SNES Cartridges](cartridges.md) → [ROM header](cartridge_rom_header.md), [ROM image headers & extensions](cartridge_rom_headers.md), [Interleave](cartridge_rom_interleave.md), [LoROM](snes-cart-lorom-mapping-rom-divided-into-32k-banks-around-1500-games.md), [HiROM](snes-cart-hirom-mapping-rom-divided-into-64k-banks-around-500-games.md). Coprocessors and odd carts: many `snes-cart-*` pages (SA-1, Super FX/GSU, DSP, CX4, S-RTC, S-DD1, SPC7110, Satellaview, Super Game Boy, flash carts, cheat devices, copiers, etc.) — see [Document index](document-index.md) under the `snes-cart-*` entries.
- **CIC / lockout:** [SNES Cartridge CIC (lockout chip)](cartridge_cic.md) and related `snes-cartridge-cic-*` files.
- **Decompression (general SNES formats):** [SNES Decompression Formats](decompression_formats.md); chip-specific algorithms appear under the relevant `snes-cart-*` coprocessor pages.
- **Pinouts, power, connectors:** [SNES Pinouts](pinouts.md) → CPU/PPU/APU/chip-specific `snes-pinouts-*` files, [Cartridge slot](cartridge_slot.md), [Expansion port](expansion_port.md), [AV connector](snes-audio-video-connector-pinouts.md), [Power supply](power_supply.md), [Chipset overview](chipset.md).
- **Edge cases & undocumented behavior:** [SNES Unpredictable Things](unpredictable.md).

## Other systems documented in the same corpus

These are **not** vanilla retail SNES consoles but appear in the same specification package:

- **Nintendo Super System (NSS — arcade / kiosk):** `nss-*` files (memory maps, I/O, BIOS, tokens, controls).
- **SFC-Box (Japanese store demo / kiosk hardware):** `sfc-box-*` files (HD64180-based coprocessor, maps, OSD, GROM).
- **Z80 CPU reference:** `z80-*` files (instruction set and flags; relevant where a Z80-class core appears in peripherals or docs).
- **HD64180 (Z80 extension):** `hd64180*` files — used with SFC-Box and as a general reference.
- **ARM (e.g. SETA ST018 cartridge):** [SNES Cart Seta ST018 (pre-programmed ARM CPU)](snes-cart-seta-st018-pre-programmed-arm-cpu-1-game.md) plus `arm-*` instruction set pages.
- **RTC S-3520:** [RTC S-3520 (Real-Time Clock)](rtc_s_3520.md) (also referenced from cartridge context).

## Miscellaneous SNES topics in this folder

- Add-ons and accessories: `snes-add-on-*` (e.g. Turbo File, modem, barcode, voice/IR).
- Hotel / arcade variants: [SNES Hotel Boxes and Arcade Machines](hotel_arcade.md).
- Homebrew / dev conveniences: [SNES Xboo Upload (WRAM Boot)](xboo.md), [SNES Common Mods](common_mods.md), [SNES Controller Mods](controller_mods.md).
- 3D glasses: [SNES 3D Glasses](3d_glasses.md).
