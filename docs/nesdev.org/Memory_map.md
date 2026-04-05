---
title: "Memory map"
source_url: "https://snes.nesdev.org/wiki/Memory_map"
pageid: 18
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

## Overall

[![](https://snes.nesdev.org/w/images/snes/d/db/Snes_overall_map.png)](https://snes.nesdev.org/wiki/File:Snes_overall_map.png)

The SNES natively provides access to RAM and I/O at specific locations. The cartridge is free to provide whatever it wants in the remaining space, and there are specific address ranges that are conventionally used to add access to additional hardware such as extra RAM or a coprocessor. Different address ranges are accessed at different speeds, and the speed of the ROM at banks $80-$FF may be changed with [[MMIO registers#MEMSEL|register $420D]].

The first 8 KiB of RAM is mirrored into many banks for convenient access, and banks $7E-$7F provide continuous access to the entire 128 KiB in one continuous address range.

A [[ROM header]] is always present in the memory map at $00FFC0, though LoROM and HiROM will place these at a different location within the ROM itself.

## LoROM

[![](https://snes.nesdev.org/w/images/snes/0/0c/Snes_lorom_map.png)](https://snes.nesdev.org/wiki/File:Snes_lorom_map.png)

The LoROM mapping mode uses 32 KiB banks. The first 15 address pins are connected normally, but the 16th address pin on the SNES cartridge port is not connected to anything.

The benefit of LoROM is that it is simpler to understand, and is closer to what NES developers are used to. LoROM can go up to 4 MiB, but past the 2 MiB mark the RAM and I/O are no longer mirrored into the banks. For a LoROM game over 2 MiB, it's recommended to put code toward the beginning of the ROM, and data toward the end.

The [[ROM header]] resides at the end of the first 32 KiB bank at $007FC0 in the ROM, mapped to $00FFC0 in memory.

Connections:

```
A0-A14 --> A0-A14
A15 (Not connected)
A16 --> A15
A17 --> A16
A18 --> A17
A19 --> A18
A20 --> A19
A21 --> A20
A22 --> A21
A23 (Not connected)
```

The unused lower half of banks $40-7D, $C0-FF are normally mirrors of the upper half if no SRAM is present ($0000-7FFF = $8000-FFFF).

When SRAM is present there are many variations on what appears in these unused regions. Sometimes SRAM is mirrored in other locations, often at $F0-FF[[1]](#cite_note-1).

## HiROM

[![](https://snes.nesdev.org/w/images/snes/6/64/Snes_hirom_map.png)](https://snes.nesdev.org/wiki/File:Snes_hirom_map.png)

The HiROM mapping mode uses 64 KiB banks. It is created by connecting the SNES's address pins to the ROM's address pins 1-to-1, without skipping any pins. This allows access to a linear view of the entire ROM at $C0-$FF. This simplifies programming for data that can cross bank boundaries. Additionally it helps fit more data into the ROM, because data that cannot cross bank boundaries has fewer bank boundaries to avoid.

HiROM can be viewed as a superset of LoROM because it still provides access to the last 32 KiB of every bank in the LoROM area. Therefore a HiROM game can decide to put code in those areas and be programmed as if it were a LoROM game, while still having the benefits of 64 KiB data banks.

The [[ROM header]] resides at the end of the first 64 KiB bank at $00FFC0 in the ROM, mapped to $00FFC0 in memory.

Connections:

```
A0-A21 --> A0-A21
A22 (Not connected)
A23 (Not connected)
```

The unused region in banks $40-7D will normally be a mirror of $C0-FD, though there are minor variations[[2]](#cite_note-2).

## ExHiROM

[![](https://snes.nesdev.org/w/images/snes/7/70/Snes_exhirom_map.png)](https://snes.nesdev.org/wiki/File:Snes_exhirom_map.png)

ExHiROM is a map meant for exceeding the 4 MiB limit HiROM normally has. Banks $80-$FF point to the first 4 MiB of the ROM file as normal, but banks $00-$7D can point up to an additional 4 MiB (minus 64 KiB due to the RAM banks).

Connections:

```
A0-A21 --> A0-21
A22 (Not connected)
A23 --> A22 (inverted)
```

The [[ROM header]] resides at the end of the first 32 KiB bank past 4MB at $40FFC0 in the ROM, mapped to $00FFC0 in memory.

## References

1. [↑](#cite_ref-1) [fullsnes](https://problemkaputt.de/fullsnes.htm#snescartlorommappingromdividedinto32kbanksaround1500games): SNES Cart LoROM Mapping (ROM divided into 32K banks) (around 1500 games)
2. [↑](#cite_ref-2) [fullsnes](https://problemkaputt.de/fullsnes.htm#snescarthirommappingromdividedinto64kbanksaround500games): SNES Cart HiROM Mapping (ROM divided into 64K banks) (around 500 games)
