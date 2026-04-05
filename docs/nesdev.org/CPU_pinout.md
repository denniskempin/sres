---
title: "CPU pinout"
source_url: "https://snes.nesdev.org/wiki/CPU_pinout"
pageid: 3
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

## Pinout

```
                                                ^
                                               / \
                                              /   \
                                             /     \
                                     +5V -- / 1 100 \ -> CPU A7
                                 CPU A8 <- / 2    99 \ -> CPU A6
                                CPU A9 <- / 3   .  98 \ -> CPU A5
                              CPU A10 <- / 4        97 \ -> CPU A4
                             CPU A11 <- / 5          96 \ -> CPU A3
                            CPU A12 <- / 6            95 \ -> CPU A2
                           CPU A13 <- / 7              94 \ -> CPU A1
                          CPU A14 <- / 8                93 \ -> CPU A0
                         CPU A15 <- / 9                  92 \ -> CPU /RD
                        CPU A16 <- / 10                   91 \ -> CPU /WR
                       CPU A17 <- / 11                     90 \ -- GND
                      CPU A18 <- / 12                       89 \ -> /VECTORPULL
                     CPU A19 <- / 13                         88 \ -> UNKNOWN CLK 88
                    CPU A20 <- / 14                           87 \ -> VDA
                   CPU A21 <- / 15                             86 \ -> VPA
                  CPU A22 <- / 16                               85 \ -- +5V
                 CPU A23 <- / 17                                 84 \ -> XFLAG
                    GND -- / 18                                   83 \ -> MFLAG
          joypad IO D0 <> / 19                                     82 \ -> /MEMLOCK
         joypad IO D1 <> / 20                                       81 \ <- RDY
        joypad IO D2 <> / 21                                            \
       joypad IO D3 <> / 22                                     O       /
      joypad IO D4 <> / 23                                          80 / -> R/W
     joypad IO D5 <> / 24                                          79 / -- GND
    joypad IO D6 <> / 25                                          78 / -> /WRAMSEL
   joypad IO D7 <> / 26              Nintendo 5A22               77 / -> /ROMSEL
   joypad 2 D0 -> / 27      Package QFP-100, 0.65mm pitch       76 / <- /ABORT
  joypad 2 D1 -> / 28                                          75 / <- HALT
 joypad 2 D2 -> / 29                  S-CPU                   74 / <- HVCMODE 
joypad 2 D3 -> / 30                                          73 / <- TM
              /       O                                     72 / -> PHI2
              \                                            71 / -> UNKNOWN CLK 71
joypad 2 D4 -> \ 31                                       70 / -> /DMA
 joypad 1 D0 -> \ 32                                     69 / -> /PWR
  joypad 1 D1 -> \ 33                                   68 / -> /PRD
           +5V -- \ 34                                 67 / <> CPU D7
   joypad 1 /OE <- \ 35                               66 / <> CPU D6
    joypad 2 /OE <- \ 36                             65 / <> CPU D5
             OUT0 <- \ 37                           64 / <> CPU D4                 Orientation:
              OUT1 <- \ 38                         63 / <> CPU D3                  --------------------
               OUT2 <- \ 39                       62 / <> CPU D2                       80         51
             REFRESH <- \ 40                     61 / <> CPU D1                         |         |
              TCKSEL0 -> \ 41                   60 / <> CPU D0                         .-----------.
               TCKSEL1 -> \ 42                 59 / -- +5V                          81-|O Nintendo |-50
                 HBLANK -> \ 43               58 / -> PA7                              |   S-CPU   |
                  VBLANK -> \ 44             57 / -> PA6                           100-|.   5A22  O|-31
                     /NMI -> \ 45           56 / -> PA5                                '-----------'
                      /IRQ -> \ 46         55 / -> PA4                                  |         |
                        GND -- \ 47       54 / -> PA3                                  01         30
                  SYSTEM CLK -> \ 48     53 / -> PA2             
              REFRESH /ENABLE -> \ 49   52 / -> PA1                      Legend:
                        /RESET -> \ 50 51 / -> PA0                       ----------------------------
                                   \     /                               --[5A22]-- Power, n/a
                                    \   /                                ->[5A22]<- 5A22 input
                                     \ /                                 <-[5A22]-> 5A22 output
                                      V                                  <>[5A22]<> Bidirectional
```

## Signal descriptions

- **VPA, VDA**: Valid Data Address and Valid Program Address, useful for caching or single-step.
  - VPA = 0, VDA = 0: Address bus may be invalid.
  - VPA = 0, VDA = 1: Valid data address
  - VPA = 1, VDA = 0: Valid program address
  - VPA = 1, VDA = 1: Opcode fetch
