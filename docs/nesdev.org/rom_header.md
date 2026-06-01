---
title: "ROM header"
source_url: "https://snes.nesdev.org/wiki/ROM_header"
pageid: 14
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Nintendo required SNES developers to include a header in the game's data that describes what hardware the game cartridge contains. This is not needed by the SNES hardware, though the game software can access it as ROM data.

However, emulators and flashcarts rely on this header to know how to emulate the game cartridge. Homebrew games should also provide a valid header. Example code: [lorom-template](https://github.com/pinobatch/lorom-template/blob/master/src/snesheader.s)

The header is located at the *CPU* address range $00FFC0-$00FFDF, right before the [[CPU vectors|interrupt vectors]], with an optional second header at $00FFB0-$00FFBF. This means that the location of the header within the actual ROM file will change based on the cartridge's memory map mode - with [[LoROM]] games placing it at $007Fxx, [[HiROM]] games placing it at $00FFxx, and [[ExHiROM]] games placing it at $40FFxx. Therefore, if it's correctly filled out, an emulator will have a higher chance of being able to figure out where the header is. See: [Header Verification](#Header_Verification) below.

This internal ROM header is to be confused with the additional 512-byte headers used by copier devices. See: [[ROM file formats]]

See also: [[Memory map]]

## Cartridge header

Header contents

| First address | Length | Contents |
| --- | --- | --- |
| $FFC0 | 21 | Cartridge title (21 bytes uppercase ASCII. Unused bytes should be spaces.) |
| $FFD5 | 1 | ROM speed and memory map mode (LoROM/HiROM/ExHiROM) |
| $FFD6 | 1 | Chipset (Indicates if a cartridge contains extra RAM, a battery, and/or a coprocessor) |
| $FFD7 | 1 | ROM size: 1<<N kilobytes, rounded up (so 8=256KB, 12=4096KB and so on) |
| $FFD8 | 1 | RAM size: 1<<N kilobytes (so 1=2KB, 5=32KB, and so on) |
| $FFD9 | 1 | Country (Implies NTSC/PAL) |
| $FFDA | 1 | Developer ID |
| $FFDB | 1 | ROM version (0 = first) |
| $FFDC | 2 | Checksum complement (Checksum ^ $FFFF) |
| $FFDE | 2 | Checksum |
| $FFE0 | 32 | [[CPU vectors|Interrupt vectors]] |

### $FFD5

Address $00FFD5 indicates the ROM speed and map mode.

```
001smmmm
   |++++- Map mode
   +----- Speed: 0=Slow, 1=Fast
```

Available modes include:

- 0: [[LoROM]]
- 1: [[HiROM]]
- 5: [[ExHiROM]]

### $FFD6

Address $00FFD6 indicates what extra hardware is in the cartridge, if any.

Possible values include:

- $00 - ROM only
- $01 - ROM + RAM
- $02 - ROM + RAM + battery
- $x3 - ROM + coprocessor
- $x4 - ROM + coprocessor + RAM
- $x5 - ROM + coprocessor + RAM + battery
- $x6 - ROM + coprocessor + battery
- $0x - Coprocessor is DSP ([[DSP-1]], 2, 3 or 4)
- $1x - Coprocessor is [[GSU]] (SuperFX)
- $2x - Coprocessor is [OBC1](https://snes.nesdev.org/w/index.php?title=OBC1&action=edit&redlink=1 "OBC1 (page does not exist)")
- $3x - Coprocessor is [SA-1](https://snes.nesdev.org/w/index.php?title=SA-1&action=edit&redlink=1 "SA-1 (page does not exist)")
- $4x - Coprocessor is [S-DD1](https://snes.nesdev.org/w/index.php?title=S-DD1&action=edit&redlink=1 "S-DD1 (page does not exist)")
- $5x - Coprocessor is [S-RTC](https://snes.nesdev.org/w/index.php?title=S-RTC&action=edit&redlink=1 "S-RTC (page does not exist)")
- $Ex - Coprocessor is Other ([Super Game Boy](https://snes.nesdev.org/w/index.php?title=Super_Game_Boy&action=edit&redlink=1 "Super Game Boy (page does not exist)")/[Satellaview](https://snes.nesdev.org/w/index.php?title=Satellaview&action=edit&redlink=1 "Satellaview (page does not exist)"))
- $Fx - Coprocessor is Custom (specified with $FFBF)

When coprocessor is Custom, $FFBF selects from:

- $00 - [SPC7110](https://snes.nesdev.org/w/index.php?title=SPC7110&action=edit&redlink=1 "SPC7110 (page does not exist)")
- $01 - [ST010](https://snes.nesdev.org/w/index.php?title=ST010&action=edit&redlink=1 "ST010 (page does not exist)")/[ST011](https://snes.nesdev.org/w/index.php?title=ST011&action=edit&redlink=1 "ST011 (page does not exist)")
- $02 - [ST018](https://snes.nesdev.org/w/index.php?title=ST018&action=edit&redlink=1 "ST018 (page does not exist)")
- $03 - [[CX4]]

## Expanded cartridge header

The expanded header's presence is indicated with a "developer ID" in the above table of $33.

Some early games may indicate that $FFBF only is valid when the last byte of cartridge title ($FFD4) is $00.

Expanded header contents

| First address | Length | Contents |
| --- | --- | --- |
| FFB0 | 2 | ASCII maker code |
| FFB2 | 4 | ASCII game code |
| FFB6 | 6 | Reserved, should be zero |
| FFBC | 1 | Expansion flash size: 1 << N kilobytes |
| FFBD | 1 | Expansion RAM size: 1 << N kilobytes - for GSU? |
| FFBE | 1 | Special version (usually zero) |
| FFBF | 1 | Chipset subtype, used if chipset is $F0-$FF |

## Checksum

The checksum is a 16-bit sum of all of the bytes in the ROM, potentially with some portions repeated. It is always computed as if the ROM is a power of 2 in size, as given by the ROM header.

However, some SNES games have a ROM data size that is not a power of 2, e.g. a 3MB game might use a 2MB ROM and a 1MB ROM together. These will use [mirroring](https://snes.nesdev.org/w/index.php?title=Mirroring&action=edit&redlink=1 "Mirroring (page does not exist)") to fill remaining space to reach the next largest power of 2.

### Non Power-of-2 ROM Size

A physical cartridge will mirror its ROM chips to fill the [[Memory map]], but a [[ROM file formats|ROM file]] dump will usually try to omit duplication. To accommodate this, when the file's data doesn't already add to a power of 2, it will be treated as a combination of two regions, each a power of 2 in size. The larger of the two always comes first in the file, and the smaller one will be duplicated until the combined size reaches the next power of 2.[[1]](#cite_note-1) The total size will match what is specified in the ROM header, and when preparing a dump we must ensure the result of this process matches the physical cart's mirrored memory map.

If a ROM file's data size is neither a power of 2 nor the sum of two powers of 2, an emulator will have to first pad the smaller portion to reach the next power of 2. Emulators are inconsistent about what to use for padding (some may fill with 0s), so it is recommended to ensure your ROM file's remainder reaches a power of 2 boundary to avoid this ambiguity.

The general process for preparing the combined ROM data:

1. Find the largest power of 2 less than or equal to the data size.
2. If data remains past this point:

:   - Find the smallest power of 2 greater than or equal to this remainder.
:   - Pad the remainder with 0s to meet this power of 2.
:   - Now that the remainder is a power of 2, repeat this data until the remainder matches the size of the first part of the ROM (the power of 2 in step 1).

When a ROM file's data size is not already a power of 2, most frequently it will be two ROMs in a 2:1 size ratio, which will result in doubling the last third of the data. Cases that need padding are usually homebrew ROMs looking to conserve space by omitting unused memory regions.

### Computing the Checksum

Once we have a ROM prepared with a power of 2 size equal to what the ROM header specified, we may compute its checksum.

Because the ROM header will be part of the computed checksum, before computing the checksum we should first fill the header's checksum and complement values with $0000 and $FFFF. Any value plus its complement will produce the same result, so this ensures the resulting checksum matches the ROM even after the computed checksum is replaced in the header.

Once ready:

1. Start with a 16-bit checksum = 0.
2. Add every byte from the prepared data to the checksum. (Overflow is discarded.)
3. Store the checksum in the ROM header ($FFDE or equivalent).
4. Store checksum ^ $FFFF in the ROM header ($FFDC).

## Header Verification

The primary way to verify a candidate header is to evaluate the [checksum](#Checksum) it contains. Some flash-carts appear to use only the checksum to distinguish LoROM from HiROM.

If no valid checksum can be found (e.g. ROM-hacks or homebrews often omit it), additional heuristics may be used to estimate validity:[[2]](#cite_note-2)[[3]](#cite_note-3)

- ROM checksum matches.
- Checksum and complement sum to $FFFF.
- Map mode matches header location.
- Specified ROM size is not smaller than file size.
- A reset vector < $8000 is invalid because it points outside of ROM.
- The first instruction at a valid reset vector is likely to be: sei, clc, sec, stz, jmp, jml
- The first instruction at a valid reset vector is unlikely to be: brk, cop, stp, wdm, $FF (sbc long)
- ROM and RAM sizes are reasonable.
- Game name field is ASCII characters only.

## Links

- [checksum.py](https://github.com/bbbradsmith/SNES_stuff/blob/main/smalltext/checksum.py) - Python code to compute and apply the checksum for a ROM file.

## References

- [[SNES Development Manual]] Book 1, page 1-2-10: ROM Registration Data - describes the ROM header and checksum process.

1. [↑](#cite_ref-1) [[SNES Development Manual]] Book 1, page 1-2-21: Check Sum - describes non-power-of-2 ROM organization.
2. [↑](#cite_ref-2) [bsnes SuperFamicom::scoreHeader](https://github.com/bsnes-emu/bsnes/blob/f57657f27ddec337b1960c7ddaa1b23894bc00c3/bsnes/heuristics/super-famicom.cpp#L515) - source code for estimating header likelihood
3. [↑](#cite_ref-3) [snes9x CMemory::LoadRomInt](https://github.com/snes9xgit/snes9x/blob/a2e0580992873ec3913fd1ef09f22f368fe44b3b/memmap.cpp#L1421) - source code for estimating header likelihood
