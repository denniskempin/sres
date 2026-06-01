---
title: "Errata"
source_url: "https://snes.nesdev.org/wiki/Errata"
pageid: 37
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

This page describes quirks in the SNES hardware that programmers need to be aware of. They could be mistakes in the hardware's implementation, or just unintuitive behavior.

For a list of version-specific bugs, see the [[Version differences]] page.

## Video

- [[Offset-per-tile]] never affects the first (leftmost) tile. This is to compensate for a horizontal scroll with a partial column on each end, allowing all 33 visible tiles to have a unique offset.
- When color math is set to affect sprites, it will only affect sprites using the last four palettes.
- If the program changes the vblank NMI from disabled to enabled through [[MMIO registers#NMITIMEN|NMITIMEN]] bit 7 while the vblank flag ([[MMIO registers#RDNMI|RDNMI]] bit 7) is set, an NMI will trigger immediately. This can cause NMI to occur other than at the start of vblank, or cause more than one NMI in a single vblank, as long as it is still during vertical blanking and the program has not yet read RDNMI. (Workaround: Read RDNMI shortly before enabling NMIs.)
- When there are too many sprite slivers on a scanline, the SNES will drop the *highest priority slivers* instead of the lowest priority ones.
- The SNES programming manual describes a situation where the Time Over flag is erroneously set when the first hardware sprite is 16x16, 32x32, or 64x64, has a horizontal position of 0-255, and other hardware sprites have negative horizontal positions.
- The SNES programming manual says that a hardware sprite should not have its horizontal position set to -256 ($100).
  - Sprites with an X coordinate -256 ($100) will erroneously count towards 32 sprites per scanline limit.
  - When a sprite has an X coordinate -256 ($100), *all tile-slivers in the sprite* count towards the 34 slivers per scanline limit.[[1]](#cite_note-1)
- [[PPU registers#INIDISP|INIDISP]] (register $2100) problems
  - Changing the brightness is not instant. On a 3-chip SNES, it may only take a few pixels to change the brightness, but on a 1-chip SNES it may be a gradual fade that takes 72 pixels or more.
    - This can be a problem for games that extend vblank by disabling rendering and enabling it several scanlines into the frame. For this use-case, it's recommended to disable rendering by writing `$8F` (or $80 ORed with whatever the desired brightness is) to INIDISP instead of `$80`, so that the brightness is not changed as rendering is enabled.
  - INIDISP early read bug: When INIDISP is written to, the PPU doesn't wait for the value to be put on the bus before attempting to read it. This means that the SNES will end up rendering about one pixel where INIDISP has been set to whatever was on the data bus before the correct value. For instructions that don't use indirect addressing, this will likely be the last byte of the instruction.
    - INIDISP writes during the Vertical Blanking Period will not encounter this glitch.
    - Workaround: Use long addressing to write to INIDISP during rendering, and take advantage of how PPU registers are available in many different banks. `STA $8F2100` will put $8f on the bus before the written value, and `STA $0F2100` will put $0f on the bus before the written value, and so on.
- The unofficial 16x32 and 32x64 pixel sprite sizes have quirks.
  - 16x32 sprites do not work correctly with [[PPU registers#SETINI - Screen Mode/Video Select ($2133 write|OBJ interlacing]] "PPU registers")[[2]](#cite_note-2)
    - When OBJ interlacing is on, 16x32 sprites are treated as if they are 16x16 - the bottom 16x16 is ignored, and the top 16x16 is squished into 16x8. 32x64 sprites behave as expected.
  - 16x32 and 32x64 sprites do not handle being vertically flipped correctly.
    - When a 16x32 or 32x64 sprite is vertically flipped, the top half and the bottom half will flip independently, as if the sprite were really two 16x16 sprites or two 32x32 sprites that are vertically adjacent to each other.

## Audio

### S-SMP

- The SNES programming manual warns that writing to the first two SPC700 communication registers ($2140 and $2141) with a 16-bit write can also write to $2143 [[3]](#cite_note-3)[[4]](#cite_note-4)
  - This may be difficult to trigger or perhaps not actually exist[[5]](#cite_note-5)
- Writing to SPC700 communication registers ($2140, $2141, $2142, $2143) at the same time the other processor reads it can result in incorrect data being read.
  - A SPC700 program may want to read twice and only proceed when two subsequent reads have the same value.
- SPC700 writes to the undocumented test register at address $F0 ([[S-SMP#TEST|TEST]]) can easily crash the SPC700.
  - The TEST register can slow down or crash the SPC700 clock.
  - The TEST register can disable Audio-RAM reads and/or writes.
  - The TEST register can disable or halt timers.

### S-DSP

- Consoles with the NEC µPD6376 DAC have a slight audio imbalance. Nintendo tied the 2 Vref pins to a single single 47μF capacitor, while the µPD6376 datasheet shows the 2 Vref pins are supposed to be connected to 2 separate capacitors.[[6]](#cite_note-6)
- The S-DSP release rate is fixed. The four [[DSP envelopes#ADSR Envelope|ADSR parameters]] are Attack Rate, Decay Rate, Sustain Level and Sustain Rate.
  - To implement a custom release rate, the envelope can be changed to a *linear slide down* or *exponential slide down* GAIN mode in the middle of the note to mimic a release envelope.
- There is a race-condition when changing the ADSR/GAIN envelope mode (bit 7 of ADSR1) in the middle of a note. If the S-DSP registers are written in the order ADSR1 followed by ADSR2/GAIN, the S-DSP might read the old ADSR2/GAIN value before the ADSR2/GAIN write, potentially glitching the rest of the envelope (especially if the previous GAIN was a fixed envelope).[[7]](#cite_note-7)
  - Workaround: Write to the ADSR2/GAIN register before the ADSR1 register.
  - Workaround: Only change the ADSR/GAIN envelope mode bit when the channel is in the release state.
- The hardware noise unit interprets the entire 15-bit state of an LFSR as a signed noise sample. Because the LFSR only shifts 1 bit per sample, the contents are strongly correlated from one sample to the next, producing a strong highpass filter effect on the noise.[[8]](#cite_note-8) Especially in the high and low ranges, this will differ from the more typical 1-bit LFSR noise sound seen in other sound chips.
- Setting echo delay ([[S-SMP#Global|EDL]], register $7D) to 0 continuously overwrites 4 bytes of ARAM at the start of the echo buffer page (selected by ESA, $6D). In particular, ESA = $00 and EDL = $00 overwrites zero page locations $0000-$0003. If not using echo, remember to set the echo write protect bit of [[S-SMP#FLG|FLG]] ($6C bit 5) to 1.

S-DSP register writes do not take effect immediately:

- Care must be taken when writing to the EDL and ESA echo buffer registers.
  - EDL (echo delay / echo buffer size) register writes take effect when the S-DSP reaches the end of the echo buffer.[[9]](#cite_note-9)
    - Writing to EDL can take up to 7680 samples (240ms @ 32000Hz) to take effect.
  - The ESA (echo buffer address) register is accessed once per sample and ESA writes can be delayed by a single sample.[[10]](#cite_note-10)
  - The echo buffer wraps around the 16-bit Audio-RAM address boundary, clobbering zeropage.
  - To prevent the echo buffer from clobbering memory when initialing the echo buffer, echo buffer writes should be disabled (FLG bit 5 set) before writing to ESA and EDL. There should be a minimum 7680 sample (240ms @ 32000Hz) delay before echo buffer writes are enabled (FLG bit 5 clear).
- KON and KOFF are polled every second sample.[[11]](#cite_note-11)
  - Clearing the KON bits too early can cause channels to not key-on.
  - Clearing the KOFF bits too early can cause channels to not key-off.
  - Clearing the KOFF bits after writing to KON might key-on, key-off or silence the channel.

There are 3 S-DSP overflow bugs that can cause pops and glitched audio:

- The BRR decoder clamps the output to a signed 16-bit value then clips it to a signed 15-bit value. [[12]](#cite_note-12)
  - BRR encoders that do not implement this behaviour can output corrupt BRR data when a sample has a large amplitude and the delta between two samples is very large.
- The FIR filter uses a clipped-sum for the first 7 tap calculations and a clamped-sum for only the last tap. [[13]](#cite_note-13)
  - This can cause audio clicks if the FIR filter gain exceeds 0dB.
  - To prevent FIR clicks ensure the absolute sum of the FIR taps (S-DSP registers FIR0-FIR7) is <= 128.
- The Gaussian interpolation table contains a bug that can cause an overflow (pop) if the BRR sample contains three 3 maximum-negative values in a row. [[14]](#cite_note-14)

### SPC-700

- The TSET1 (Test and set bits) and TCLR1 (Test and clear bits) instructions does an equality test (z/n flags = ALU(A - old\_value)), not a bit test[[15]](#cite_note-15).
- The flags modified by the MUL (Multiply) instruction are based on the Y register (high-byte) value only.[[16]](#cite_note-16).
- The output of the DIV (Divide) instruction is only valid if the quotient is <= 511 (9 bit result)[[17]](#cite_note-17).
- The z/n flags modified by the DIV (Divide) instruction are based on the A register (bits 0-7 of quotient) value only.

## Mode 7 multiplier

- The Mode 7 multiplier ([[PPU registers#MPY|MPY]]) result can be corrupted if an interrupt or HDMA transfer writes to a BG1 scroll register or Mode7 Matrix register in-between the two M7A writes. (The Mode7 scroll and Mode 7 matrix registers share the same write-twice latch).[[18]](#cite_note-18)

## 65c816

- Setting the index register to 8-bit (via SEP, PLP or XCE) will clear the high byte of X and Y.
  - When saving/restoring registers in an ISR, you should switch to 16-bit Index and Accumulator before pushing or popping the stack.
- The JMP (addr) and JMP [addr] instructions read from Bank 0 (ie, JMP ($1234) will read 2 bytes from $00:1234)
- The JMP (addr,x) and JSR (addr,x) instructions read from the Program Bank (PB) (ie, JSR ($1234,x) will read 2 bytes from PB:{$1234 + X})
- The MVN and MVP instructions will change the Data Bank (DB) to the destination bank.
- The syntax and operand order of the MVN and MVP instructions vary across assemblers.

## S-CPU (5A22)

- Starting a multiplication ($4203 WRMPYB) or division ($4206 WRDIVB) while the 5A22 is still processing a previous multiplication or division can cause the 5A22 to output erroneous values to RDDIV and/or RDMPY.[[19]](#cite_note-19)

### DMA

- Some A-Bus addresses are invalid: [[20]](#cite_note-20)
  - The A-Bus address cannot access a B-Bus address ($21xx)
  - The A-Bus address cannot access the MMIO or DMA registers ($4000-$41ff, $4200-$421f, $4300-$437f)
  - The A-Bus address cannot be a Work-RAM address if the B-Bus address is WMDATA ($2180). This means DMA cannot be used to copy from one section of the SNES's RAM to another.
- On version 1 of the 5A22 chip ("S-CPU"), the chip can crash if [[DMA]] finishes right before [[HDMA]] happens. This is generally only a problem for games that want to use DMA to clear WRAM or copy data from a coprocessor to WRAM, as that's the main reason to use DMA during rendering.
- On version 2 of the 5A22 chip ("S-CPU-A"), a recent HDMA transfer to/from INIDISP (Meaning that BBADn is set to zero $00) can make a DMA transfer fail. Nothing will happen and the DMA size registers (DASnL, DASnH) will be unchanged, instead of zero like they normally are after a DMA has been completed.
  - Workaround: Set BBADn to $ff instead, and set the transfer pattern to 1. This will cause HDMA to write to $21ff (nothing) and then $2100 (INIDISP). Both bytes should be set to the same value to prevent the INIDISP early read bug.
  - S-CPU (the first version), S-CPU-B and the 1-CHIP SNES are not affected by this bug.
- HDMA can fail if a DMA transfer ends when HDMA starts (just after the start of scanline 0) and the previous value read by DMA is 0.[[21]](#cite_note-21)
  - When this glitch occurs the HDMA channel stops at the start of scanline 0 and there are no H-Blank transfers for an entire frame.
- Enabling a HDMA channel (writing a non-zero value to HDMAEN) outside of the Vertical-Blanking period (even when the screen is disabled) can cause unwanted erroneous writes to the PPU.
  - At the start of scanline 0, the DMA controller initialises HDMA state registers (A2An, NLTRn) for only the active HDMA channels.
  - Enabling a HDMA channel outside of VBlank, without setting the HDMA state registers, will cause the DMA controller to read HDMA table entries from an erroneous memory address.

### Input

- Automatic [[Controller reading]] begins between H=32.5 and H=95.5 of the first vblank scanline[[22]](#cite_note-22). This means that checking [[MMIO registers#HVBJOY|HVBJOY]] ($4212) is not quite sufficient to avoid an in-progress auto-read if used immediately after the start of vblank.
- Autoread result may change unexpectedly during a lag frame. Either copy it to a variable or disable autoreading while game logic is running.

## File formats

- LoROM and HiROM memory maps place the [[ROM header]] at different ROM offsets. Emulators and flash-carts use [[ROM header#Header Verification|heuristics to guess]] the memory map of the ROM file.
  - Some Super Everdrive flash-carts require a correct [[ROM header#Checksum|ROM header checksum]].[[23]](#cite_note-23)

- The [SPC file format](https://snes.nesdev.org/w/index.php?title=SPC_file_format&action=edit&redlink=1 "SPC file format (page does not exist)") does not capture the full state of the APU.
  - The SPC file format is intended to store the state of the APU before the first note in a song. It cannot accurately store the state of the APU in the middle of a song or sample.
  - The SPC file format does not store the S-SMP timer positions, echo buffer position, BRR decoder positions, gaussian interpolator positions and voice envelopes.
- The SPC file format does not capture the APUIO communications between the S-CPU and the SPC700. Audio drivers that can transfer samples or song-data while playing a song cannot be captured as a .spc file.
- The SPC file format has 2 different ID666 tag formats (text and binary) and no version field to differentiate between the two.

## Hardware

- A cartridge that pulls the cartridge slot's /RESET pin low will not reset the PPU. It will only reset the S-CPU, APU and S-WRAM.

## References

:   - <https://undisbeliever.net/snesdev/registers/inidisp.html#glitches-and-hardware-bugs>
    - [[SNES Development Manual]] Book 1, section 2-25-1: Documented Problems

1. [↑](#cite_ref-1) [bsnes object.cpp](https://github.com/bsnes-emu/bsnes/blob/4faca659c12ffc81d932cb0d23fea477f227d9d1/bsnes/sfc/ppu/object.cpp#L138): 34 sliver culling behaviour.
2. [↑](#cite_ref-2) [Forum thread](https://forums.nesdev.org/viewtopic.php?t=21389): 16x32 sprites in interlaced mode?
3. [↑](#cite_ref-3) [[SNES Development Manual]] Book 1, section 3-9-6: Sound Programming Cautions
4. [↑](#cite_ref-4) [Super Famicom Development Wiki](https://wiki.superfamicom.org/spc700-reference#communication-ports-74): SPC700 Reference
5. [↑](#cite_ref-5) [Forum post](https://forums.nesdev.org/viewtopic.php?p=279472#p279472): APU crosstalk 16-bit bug
6. [↑](#cite_ref-6) [ConsoleMods Wiki - SNES:Audio Balance Fix](https://consolemods.org/wiki/SNES:Audio_Balance_Fix)
7. [↑](#cite_ref-7) [Terrific Audio Driver - I found a race condition](https://undisbeliever.net/blog/20231231-terrific-audio-driver.html#i-found-a-race-condition)
8. [↑](#cite_ref-8) [Forum post](https://forums.nesdev.org/viewtopic.php?p=282889#p282889): Re: Was the SPC700's noise channel based on the 2a03's noise channel?
9. [↑](#cite_ref-9) Anomie's S-DSP Doc: EDL register
10. [↑](#cite_ref-10) Anomie's S-DSP Doc: ESA register
11. [↑](#cite_ref-11) Anomie's S-DSP Doc: KON and KOFF registers
12. [↑](#cite_ref-12) [apudsp\_jwdonal.txt](https://www.romhacking.net/documents/191/): Anomie's S-DSP document - BRR DECODING
13. [↑](#cite_ref-13) [SnesLab Wiki - FIR Filter - S-DSP Implementation](https://sneslab.net/wiki/FIR_Filter#S-DSP_Implementation)
14. [↑](#cite_ref-14) [Fullsnes](https://problemkaputt.de/fullsnes.htm#snesapudspbrrpitch): SNES APU DSP BRR Pitch - 4-Point Gaussian Interpolation
15. [↑](#cite_ref-15) higan source code, higan::SPC700::instructionTestSetBitsAbsolute(), by Near
16. [↑](#cite_ref-16) higan source code, higan::SPC700::instructionMultiply(), by Near
17. [↑](#cite_ref-17) higan source code, higan::SPC700::instructionDivide(), by Near
18. [↑](#cite_ref-18) [Forum post](https://forums.nesdev.org/viewtopic.php?p=249422#p249422): bsnes-plus and xkas-plus (new debugger and assembler)
19. [↑](#cite_ref-19) [Forum thread](https://forums.nesdev.org/viewtopic.php?p=282493#p282493) - Writing $4203 twice too fast gives erroneous result (not emulated)
20. [↑](#cite_ref-20) higan source code, [sfc/cpu/dma.cpp](https://github.com/higan-emu/higan/blob/master/higan/sfc/cpu/dma.cpp), by Near
21. [↑](#cite_ref-21) [thread](https://forums.nesdev.org/viewtopic.php?t=24822%7CForum): Investigating a HDMA failure
22. [↑](#cite_ref-22) [Form post](https://forums.nesdev.org/viewtopic.php?p=188011#p188011): Stupid problems with autoread on hardware
23. [↑](#cite_ref-23) [forum post](https://forums.nesdev.org/viewtopic.php?p=278748#p278748) Re: Elasticity flicker colour comparison by rainwarrior
