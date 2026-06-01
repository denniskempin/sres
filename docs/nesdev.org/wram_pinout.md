---
title: "WRAM pinout"
source_url: "https://snes.nesdev.org/wiki/WRAM_pinout"
pageid: 48
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

## Pinout

```
                    ____________________
                   |                    |
            +5V -- | 01  .           64 | -- GND
         CPU D4 <> | 02              63 | <> CPU D3
         CPU D5 <> | 03              62 | <> CPU D2
         CPU D6 <> | 04              61 | <> CPU D1
         CPU D7 <> | 05              60 | <> CPU D0
     SYSTEM CLK -> | 06              59 | <- CPU /WR
        REFRESH -> | 07              58 | <- /PWR
         /RESET -> | 08              57 | <- CPU /RD
             NC -- | 09              56 | <- /PRD
      CS1 (+5V) -> | 10              55 | ?? G (NC)
      CS2 (+5V) -> | 11              54 | <- PA1
      CS3 (+5V) -> | 12              53 | <- PA0
     /CS1 (GND) -> | 13              52 | <- PS3 (+5V)
     /CS2 (GND) -> | 14              51 | <- PS2 (+5V)
/CS3 (/WRAMSEL) -> | 15   Nintendo   50 | <- PS1 (PA7)                    Orientation:
            +5V -- | 16    S-WRAM    49 | -- +5V                     --------------------
            GND -- | 17              48 | -- GND                       64         33
             NC -- | 18              47 | <- /PS5 (PA6)                 |         |
             NC -- | 19              46 | <- /PS4 (PA5)                .-----------.
             NC -- | 20              45 | <- /PS3 (PA4)                | Nintendo  |
             NC -- | 21              44 | <- /PS2 (PA3)                |  S-WRAM  O|
             NC -- | 22              43 | <- /PS1 (PA2)                |.          |
         CPU A0 -> | 23              42 | <- ENABLE (A22)              '-----------'
         CPU A9 -> | 24              41 | <- CPU A8                     |         |
         CPU A1 -> | 25              40 | <- CPU A16                   01         32
        CPU A10 -> | 26              39 | <- CPU A7
         CPU A2 -> | 27              38 | <- CPU A15               Legend:
        CPU A11 -> | 28              37 | <- CPU A6                ----------------------------
         CPU A3 -> | 29              36 | <- CPU A14               --[S-WRAM]-- Power, n/a
        CPU A12 -> | 30      O       35 | <- CPU A5                ->[S-WRAM]<- S-WRAM input
         CPU A4 -> | 31              34 | <- CPU A13               <-[S-WRAM]-> S-WRAM output
            +5V -- | 32              33 | -- GND                   <>[S-WRAM]<> Bidirectional
                   |____________________|                          ??[S-WRAM]?? Unknown
```

## Signal descriptions

- **ENABLE**: This enables WRAM A16 (and possibly also A15..13) and is connected to CPU A22, allowing CPU A16 to address different RAM in banks $7E and $7F, but not elsewhere (resulting in the same 8 KiB in the other banks).
- **/CS3**: CPU select, connected to the CPU's /WRAMSEL output to map S-WRAM to CPU bus addresses $00-3F,80-BF:0000-1FFF and $7E-7F:0000-FFFF.
- **PS1**, **/PS5..1**: Peripheral select, connected to PA7..2 to map S-WRAM to peripheral bus addresses $80-83.
- **PA1..0**: These select the S-WRAM register being accessed on the peripheral bus (WRAM data or address).
- **G**: Suspected to be an output indicating whether S-WRAM is selected on either bus. This has been seen outputting low while the console is held in reset and spiking high while running.
