---
title: "S-MIX Pinout"
source_url: "https://snes.nesdev.org/wiki/S-MIX_Pinout"
pageid: 190
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Pinout for the S-MIX. It's used in Later Revisions to mix the Audio from the APU, the Cartridge and the Expansion Port.

```
             .---\/---.
      VCC -- | 01  14 | <- PREL
     PRER -> | 02  13 | <- MUTE
    IN_AR -> | 03  12 | <- IN_AL
    IN_ER -> | 04  11 | <- IN_EL
    IN_CR -> | 05  10 | <- IN_CL
      GND -- | 06  09 | -- GND
Right Out <- | 07  08 | -> Left Out
             `--------´
```

- PRER: Right Preset
- PREL: Left Preset
- IN\_AL/R: Audio from APU
- IN\_EL/R: Audio from Expansion Port
- IN\_CL/R: Audio from Cartridge

Full Mixer Schematic (From PAL 1CHIP SNES):

```
                       || ??                                     || ??                                || ??
           +-----------||---|IN_AR                   +-----------||---|IN_ER              +-----------||---|IN_CR
           |  10K      ||                            |  200      ||                       |  200      ||
APU Right|-+-\/\/\--+               Expansion Right|-+-\/\/\--+               Cart Right|-+-\/\/\--+
                    |                                         |                                    |
                    +----|GND                                 +----|GND                            +----|GND
                    |                                         |                                    |
 APU Left|-+-\/\/\--+                Expansion Left|-+-\/\/\--+                Cart Left|-+-\/\/\--+
           |  10K      || ??                         |  200      || ??                    |  200      || ??
           +-----------||---|IN_AL                   +-----------||---|IN_EL              +-----------||---|IN_CL
                       ||                                        ||                                   ||
       +|| 33µF                 +|| 33µF
PRER|---||------|GND     PREL|---||------|GND
        ||                       ||
                 1K
            +---\/\/\------|AV Multiout Right
            |    47K                                   || 330pF
Right Out|--+---\/\/\------------------------------+---||---|GND
            |    10K                               |   ||
            +---\/\/\---+                          |
                        |                          |
                        +---|Expansion Port Mono   +---|RF Mod Mono
                 10K    |                          |
            +---\/\/\---+                          |
            |    47K                               |
 Left Out|--+---\/\/\------------------------------+
            |    1K
            +---\/\/\------|AV Multiout Left
```

### Source

- <https://videogameperfection.com/forums/topic/schematic-for-1chip-pal-snes/>
