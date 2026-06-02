# Index of Documentation

Below is a table of the most important documentation files. Try `grep`ing to find
additional files covering more niche topics.

## FullSNES Docs

`docs/fullsnes` contains SNES architecture details extracted from [fullsnes.txt](https://problemkaputt.de/fullsnes.htm).

| File                                 | Keywords                                                                         |
| ------------------------------------ | -------------------------------------------------------------------------------- |
| fullsnes/timing_ppu_memory.md        | PPU, VRAM, CGRAM, OAM, scanline timing                                           |
| fullsnes/ppu.md                      | PPU, video memory, sprites, backgrounds, priority                                |
| fullsnes/ppu_sprites.md              | PPU, sprites, OAM, OBSEL                                                         |
| fullsnes/ppu_resolution.md           | PPU, resolution, high-resolution, interlace, BG modes                            |
| fullsnes/ppu_control.md              | PPU, INIDISP, SETINI, screen designation, forced blanking                        |
| fullsnes/ppu_bg.md                   | BG modes, BGMODE, mosaic, BG scroll, tilemap                                     |
| fullsnes/pinouts_ppu.md              | PPU, S-PPU1, S-PPU2, pinouts, VRAM                                               |
| fullsnes/pinouts_cpu.md              | SNES, CPU, 5A22, pinout, W65C816                                                 |
| fullsnes/memory.md                   | SNES memory, WRAM, VRAM, OAM, DMA                                                |
| fullsnes/memory_wram.md              | WRAM, Work RAM, Port 2180h, DMA, 2.6MHz                                          |
| fullsnes/memory_vram.md              | VRAM, VMAIN, VMADDR, address translation, prefetch                               |
| fullsnes/memory_oam.md               | OAM, sprites, OBJ, SNES PPU, sprite attributes                                   |
| fullsnes/memory_map.md               | memory map, LoROM, HiROM, WRAM, bank switching                                   |
| fullsnes/memory_cgram.md             | CGRAM, palette, CGADD, CGDATA, RDCGRAM                                           |
| fullsnes/index.md                    | SNES, hardware reference, PPU, APU, DMA                                          |
| fullsnes/dma.md                      | DMA, H-DMA, GP-DMA, PPU, H-Blank                                                 |
| fullsnes/dma_start_enable.md         | DMA, HDMA, MDMAEN, HDMAEN, transfer ordering                                     |
| fullsnes/dma_notes.md                | DMA, HDMA, midframe, cartridge                                                   |
| fullsnes/dma_channel_regs.md         | DMA, HDMA, registers, PPU, channels                                              |
| fullsnes/cpu_transfers.md            | W65C816, CPU opcodes, load/store, stack, block transfer                          |
| fullsnes/cpu_registers.md            | W65C816, CPU registers, processor status flags, emulation mode, 6502             |
| fullsnes/cpu_jump.md                 | 65C816, jump, branch, interrupt, opcode                                          |
| fullsnes/cpu_cycles.md               | W65C816, CPU cycles, memory speed, addressing modes, timing                      |
| fullsnes/cpu_65xx.md                 | 65XX, CPU, 65816, instruction set, registers                                     |
| fullsnes/chipset.md                  | SNES chipset, S-CPU, S-PPU, S-SMP, S-DSP                                         |
| fullsnes/cart_lorom.md               | SNES, LoROM, cartridge mapping, SRAM, bank layout                                |
| fullsnes/apu.md                      | APU, SPC700, S-DSP, audio                                                        |
| fullsnes/apu_memory_map.md           | APU, SPC700, DSP, memory map                                                     |
| fullsnes/apu_dsp_pitch.md            | SNES, APU, DSP, BRR, pitch, PMON, gaussian interpolation                         |
| fullsnes/apu_dsp_control.md          | S-DSP, APU, audio, voice control, registers                                      |
| fullsnes/apu_dsp_brr.md              | BRR, DSP, APU, sample format                                                     |
| fullsnes/apu_dsp_adsr.md             | ADSR, Gain, Envelope, S-DSP, Voice                                               |
| fullsnes/apu_cpu_port.md             | APUIO, SPC700 boot ROM, 2140h-2143h, IPL uploader                                |
| fullsnes/unpredictable.md            | open bus, DMA, PPU, SPC700 division, unused addresses                            |
| fullsnes/timing_oscillators.md       | NTSC, PAL, oscillators, APU, cartridge chips                                     |
| fullsnes/timing_events.md            | PPU timing, H/V events, HDMA, IRQ, scanline                                      |
| fullsnes/timing_counters.md          | SNES timing, H/V counter, scanline, NTSC, PAL color clock                        |
| fullsnes/ppu_window.md               | PPU, window, W12SEL, W34SEL, WOBJSEL, mask logic                                 |
| fullsnes/ppu_vram.md                 | VRAM, BG Map, Tiles, PPU, Mode 7                                                 |
| fullsnes/ppu_timers.md               | PPU, H/V counter, OPHCT/OPVCT, STAT78, lightgun                                  |
| fullsnes/ppu_rotation.md             | PPU, Mode 7, rotation, scaling, affine transformation                            |
| fullsnes/ppu_interrupts.md           | NMI, IRQ, VBlank, H/V timer, PPU registers                                       |
| fullsnes/ppu_color_math.md           | PPU, Color Math, Main Screen, Sub Screen, CGWSEL                                 |
| fullsnes/ppu_cgram.md                | CGRAM, palette, direct color, PPU, backdrop                                      |
| fullsnes/pinouts_apu.md              | S-DSP, S-SMP, SPC700, S-APU, pinouts                                             |
| fullsnes/io_map.md                   | PPU, APU, DMA, WRAM, joypad                                                      |
| fullsnes/cpu_rotate_shift.md         | W65C816, CPU, shift, rotate, ASL, LSR, ROL, ROR                                  |
| fullsnes/cpu_glitches.md             | 65C816, CPU glitches, read-modify-write, page-wraps, dummy cycles                |
| fullsnes/cpu_alu.md                  | W65C816, ALU, opcodes, addressing modes, flags                                   |
| fullsnes/cpu_addressing.md           | CPU, addressing modes, zero page, absolute, indirect                             |
| fullsnes/controllers_pinouts.md      | joypad, controller port, pinout, strobe, clock                                   |
| fullsnes/controllers_joypad.md       | joypad, controller, button bits, input, NTT Data Pad                             |
| fullsnes/controllers_auto_read.md    | SNES controllers, automatic reading, JOY registers, button state, V-Blank timing |
| fullsnes/cartridge_slot.md           | pinout, cartridge, connector, CIC, address bus                                   |
| fullsnes/cartridge_rom_header.md     | SNES cartridge header, LoROM, HiROM, checksum, map mode                          |
| fullsnes/cart_hirom.md               | HiROM, SNES, cartridge mapping, SRAM, ExHiROM                                    |
| fullsnes/apu_spc700.md               | SPC700, APU, S-SMP, PSW, addressing modes                                        |
| fullsnes/apu_spc700_load_store.md    | SPC700, APU, load/store, opcodes, addressing modes                               |
| fullsnes/apu_spc700_jump.md          | SPC700, APU, jump instructions, control commands, opcodes                        |
| fullsnes/apu_spc700_io.md            | SPC700, APU, I/O ports, Timers, Waitstates                                       |
| fullsnes/apu_spc700_alu.md           | SPC700, APU, ALU, opcode, SNES                                                   |
| fullsnes/apu_dsp_volume.md           | SNES, APU, DSP, volume, output mixer                                             |
| fullsnes/apu_dsp_echo.md             | S-DSP, echo, FIR filter, ring buffer, ESA, EDL                                   |
| fullsnes/apu_block_diagram.md        | APU, DSP, BRR, echo/reverb, audio mixing                                         |
| fullsnes/maths.md                    | multiplication, division, PPU math, Mode 7, hardware registers                   |
| fullsnes/controllers.md              | controllers, joypad, I/O ports, light guns, peripherals                          |
| fullsnes/controllers_manual_read.md  | JOYWR, JOYA/JOYB, manual joypad read, WRIO/RDIO, controller ports                |
| fullsnes/controllers_hardware_ids.md | SNES, controller, hardware ID, controller detection, peripherals                 |
| fullsnes/cartridge_pcbs.md           | SNES cartridge PCB, SHVC naming convention, LoRom, HiRom, coprocessors           |
| fullsnes/cart_sa1_char_conv.md       | SA-1, Character Conversion, BW-RAM, I-RAM, DMA                                   |
| fullsnes/apu_timings.md              | APU timing, DSP cycles, SPC700, audio sample, RAM access                         |

## NESDev.org

`docs/nesdev.org` contains SNES architecture details extracted from [nesdev.org](https://nesdev.org).

| File                                         | Keywords                                                                              |
| -------------------------------------------- | ------------------------------------------------------------------------------------- |
| nesdev.org/timing.md                         | "master clock, CPU cycles, scanline, vblank, DRAM refresh"                            |
| nesdev.org/tiles.md                          | "2bpp, 4bpp, 8bpp, Mode 7, direct color"                                              |
| nesdev.org/subroutine_call_tradeoffs.md      | "W65C816, calling convention, register size, tail call optimization, stack arguments" |
| nesdev.org/sprites.md                        | "OAM, OBJ, sprites, PPU, rendering"                                                   |
| nesdev.org/snes_ppu_for_nes_developers.md    | "PPU, VRAM, CGRAM, OAM, background modes"                                             |
| nesdev.org/s_smp.md                          | S-SMP, SPC-700, APUIO, IPL Boot ROM, timers                                           |
| nesdev.org/s_dsp_registers.md                | "S-DSP registers, ADSR envelope, echo/FIR filter, BRR voices, DSPADDR/DSPDATA"        |
| nesdev.org/rom_file_formats.md               | "LoROM, HiROM, ExHiROM, headered ROM, SFC"                                            |
| nesdev.org/reading_and_writing_ppu_memory.md | "VRAM, CGRAM, OAM, PPU memory, DMA"                                                   |
| nesdev.org/ppu_registers.md                  | "PPU registers, MMIO, VRAM, Mode 7, color math"                                       |
| nesdev.org/palettes.md                       | "CGRAM, palette, PPU registers, 15-bit RGB, CGADD/CGDATA"                             |
| nesdev.org/oam_layout.md                     | "OAM, sprites, low table, high table, sprite attributes"                              |
| nesdev.org/mvn_and_mvp_block_copy.md         | "MVN, MVP, block copy, 65C816, memory transfer"                                       |
| nesdev.org/mode_7_transform.md               | "Mode 7, affine transformation, PPU, HDMA, texture mapping"                           |
| nesdev.org/mmio_register_table.md            | "MMIO registers, PPU, DMA, APU, WRAM"                                                 |
| nesdev.org/mmio_register_table_ppu.md        | PPU, MMIO registers, SNES, register table                                             |
| nesdev.org/mmio_register_table_mmio.md       | "MMIO, SNES, registers, DMA, joypad"                                                  |
| nesdev.org/mmio_register_table_dma.md        | "DMA, HDMA, DMAPn, A-bus, B-bus"                                                      |
| nesdev.org/memory_map.md                     | "LoROM, HiROM, ExHiROM, ROM header, memory map"                                       |
| nesdev.org/errata.md                         | "PPU, S-DSP, SPC700, 5A22, DMA"                                                       |
| nesdev.org/dma_registers.md                  | "DMA, HDMA, registers, channels, MDMAEN"                                              |
| nesdev.org/dma_examples.md                   | "SNES, DMA, VRAM, 65816, assembly"                                                    |
| nesdev.org/cpu_vectors.md                    | "CPU vectors, 65C816, interrupts, native mode, emulation mode"                        |
| nesdev.org/cpu_pinout.md                     | "S-CPU, 5A22, pinout, W65C816, QFP-100"                                               |
| nesdev.org/booting_the_spc700.md             | "SPC700, APUIO, 65c816, boot ROM, DSP registers"                                      |
| nesdev.org/backgrounds.md                    | "background modes, tilemaps, PPU, Mode 7, priority, high resolution"                  |
| nesdev.org/apu_register_table.md             | S-SMP, S-DSP, APU registers, audio                                                    |
| nesdev.org/apu_register_table_dsp_voice.md   | "S-DSP, voice registers, APU, ADSR envelope, sample playback"                         |
| nesdev.org/65c816.md                         | "65C816, 65816, Ricoh 5A22, S-CPU, instruction set"                                   |
| nesdev.org/65c816_for_6502_developers.md     | "65c816, 6502, W65C816, CPU, SNES"                                                    |
| nesdev.org/windows.md                        | "PPU, Windows, Backgrounds, Color math, HDMA"                                         |
| nesdev.org/version_differences.md            | "S-CPU, S-PPU2, 1-CHIP, S-APU, DMA"                                                   |
| nesdev.org/v_blank_interrupts.md             | "VBlank, NMI, NMITIMEN, PPU, ISR"                                                     |
| nesdev.org/tilemaps.md                       | "tilemap, nametable, VRAM, Mode 7, BGMODE"                                            |
| nesdev.org/spc_700_instruction_set.md        | "SPC-700, S-SMP, SNES, audio CPU, opcodes"                                            |
| nesdev.org/scrolling_a_large_map.md          | "scrolling, tilemap, VMAIN, VRAM, PPU"                                                |
| nesdev.org/rom_header.md                     | "ROM header, map mode, checksum, chipset, cartridge"                                  |
| nesdev.org/ppu_pinout.md                     | "PPU, S-PPU1, S-PPU2, 5C77, 5C78, pinout, video"                                      |
| nesdev.org/mode_7_perspective_effects.md     | "Mode 7, PPU, perspective, background, transform"                                     |
| nesdev.org/mmio_registers.md                 | SNES, MMIO, registers, 5A22, APU, WRAM, joypad                                        |
| nesdev.org/hdma_examples.md                  | "HDMA, DMA, PPU, scanline effects, CGRAM"                                             |
| nesdev.org/glossary.md                       | "glossary, terminology, CPU, PPU, audio"                                              |
| nesdev.org/dsp_envelopes.md                  | "S-DSP, ADSR, GAIN, envelope, period table"                                           |
| nesdev.org/controller_connector.md           | "controller connector, pinout, joypad, JOYSER, JOYOUT"                                |
| nesdev.org/color_math.md                     | "color math, PPU blending, transparency, CGADSUB, sub screen"                         |
| nesdev.org/brr_samples.md                    | "BRR, S-DSP, ADPCM, sample format, gaussian interpolation"                            |
| nesdev.org/blargg_spc_upload.md              | "SPC, SPC-700, DSP registers, SPC upload, SNES audio"                                 |
| nesdev.org/apu_register_table_smp.md         | S-SMP, APU, registers, timers, S-DSP                                                  |
| nesdev.org/apu_register_table_dsp_global.md  | "S-DSP, APU, DSP registers, echo, audio"                                              |
| nesdev.org/tutorials.md                      | SNES development, assembly programming, graphics, game engine, tutorials              |
| nesdev.org/tricky_to_emulate_games.md        | "emulation bugs, PPU, DMA, HDMA, Super FX"                                            |
| nesdev.org/struct_register_tradeoffs.md      | "65c816, X and Y registers, this pointer, addressing modes, direct page"              |
| nesdev.org/standard_controller.md            | standard controller, JOY1, JOYSER0, button report, peripheral signature               |
| nesdev.org/signature_byte.md                 | "signature byte, BRK, COP, WDM, 65C816"                                               |
| nesdev.org/open_bus.md                       | "open bus, CPU, PPU, 5A22, data bus"                                                  |
| nesdev.org/offset_per_tile.md                | "offset-per-tile, PPU, scroll, background modes, BG3"                                 |
| nesdev.org/drawing_window_shapes.md          | "SNES PPU windows, HDMA, window mask logic, trapezium"                                |
| nesdev.org/controller_reading.md             | "controller reading, auto-read, manual reading, NMITIMEN, HVBJOY"                     |
| nesdev.org/apu_pinout.md                     | "APU, S-SMP, S-DSP, pinout, SHVC-SOUND"                                               |
