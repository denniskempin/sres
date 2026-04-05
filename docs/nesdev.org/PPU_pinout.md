---
title: "PPU pinout"
source_url: "https://snes.nesdev.org/wiki/PPU_pinout"
pageid: 5
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

## S-PPU1

### Pinout

```
                                         ^
                                        / \
                                       /   \
                                      /     \
                             TST1 -> / 1 100 \ <- SYSTEM CLK
                            TST0 -> / 2    99 \ <- TST2
                           /PRD -> / 3  (*) 98 \ <- /RESET
                          /PWR -> / 4        97 \ <- /PIXEL CLK IN
                          PA7 -> / 5          96 \ -- GND
                         PA6 -> / 6            95 \ <> FIELD
                        PA5 -> / 7              94 \ -> /OVER
                       PA4 -> / 8                93 \ -> /PIXEL CLK OUT
                      PA3 -> / 9                  92 \ <> /HCLD
                     PA2 -> / 10                   91 \ <> /VCLD
                    PA1 -> / 11                     90 \ -> COLOR0
                   PA0 -> / 12                       89 \ -> COLOR1
                  +5V -- / 13                         88 \ -> COLOR2
              CPU D7 <> / 14                           87 \ -> PRIO0
             CPU D6 <> / 15                             86 \ -> PRIO1
            CPU D5 <> / 16                               85 \ -> CHR0
           CPU D4 <> / 17                                 84 \ -> CHR1
          CPU D3 <> / 18                                   83 \ -> CHR2
         CPU D2 <> / 19                                     82 \ -> CHR3
        CPU D1 <> / 20                                       81 \ -- +5V
       CPU D0 <> / 21                                            \
         GND -- / 22                                             /
    HVCMODE -> / 23                                          80 / -> /VRD
   PALMODE -> / 24                                          79 / -> /VBWR
  /MASTER -> / 25                                          78 / -> /VAWR
/EXTSYNC -> / 26              Nintendo 5C77               77 / -- GND
    GND -- / 27      Package QFP-100, 0.65mm pitch       76 / -> VAA0
  VDB0 <> / 28                                          75 / -> VAA1
 VDB1 <> / 29                  S-PPU1                  74 / -> VAA2 
VDB2 <> / 30                                          73 / -> VAA3
       /                                             72 / -> VAA4
       \                                            71 / -> VAA5
VDB3 <> \ 31                                       70 / -> VAA6
 VDB4 <> \ 32                                     69 / -> VAA7 
  VDB5 <> \ 33                                   68 / -> VAA8 
   VDB6 <> \ 34                                 67 / -> VAA9
    VDB7 <> \ 35                               66 / -> VAA10
      +5V -- \ 36                             65 / -> VAA11
      VDA0 <> \ 37                           64 / -> VAA12                    Orientation:
       VDA1 <> \ 38                         63 / -> VAA13                     --------------------
        VDA2 <> \ 39                       62 / -- +5V                          80         51
         VDA3 <> \ 40                     61 / -> VAB0                           |         |
          VDA4 <> \ 41                   60 / -> VAB1                           .-----------.
           VDA5 <> \ 42                 59 / -> VAB2                         81-| Nintendo O|-50
            VDA6 <> \ 43               58 / -> VAB3                             |  S-PPU1   |
             VDA7 <> \ 44             57 / -> VAB4                          100-|@  5C77    |-31
               GND -- \ 45           56 / -> VAB5                               '-----------'
               VA15 <- \ 46         55 / -> VAB6                                 |         |
                VA14 <- \ 47   O   54 / -> VAB7                                 01         30
                VAB13 <- \ 48     53 / -> VAB8            
                 VAB12 <- \ 49   52 / -> VAB9                     Legend:
                  VAB11 <- \ 50 51 / -> VAB10                     ----------------------------
                            \     /                               --[5C77]-- Power, n/a
                             \   /                                ->[5C77]<- 5C77 input
                              \ /                                 <-[5C77]-> 5C77 output
                               V                                  <>[5C77]<> Bidirectional
                                                                  ??[5C77]?? Unknown
```

## S-PPU2

### Pinout

```
                                              _____
                                             /     \
                                  /BURST <- / 1 100 \ -> /CSYNC
                                   /PED <- / 2    99 \ -- GND
                        COLORBURST CLK <- / 3      98 \ <- HVCMODE
                         /TRANSPARENT <> / 4        97 \ -> B
                                 +5V -- / 5          96 \ -> G
                               /PWR -> / 6            95 \ -> R
                              /PRD -> / 7              94 \ -- +5VA
                           CPU D7 <> / 8                93 \ <- DIGITAL VIDEO ENABLE
                          CPU D6 <> / 9                  92 \ <> TST14
                         CPU D5 <> / 10                   91 \ <> TST13
                        CPU D4 <> / 11                     90 \ <> TST12
                       CPU D3 <> / 12                       89 \ <> TST11
                      CPU D2 <> / 13                         88 \ <> TST10
                     CPU D1 <> / 14                           87 \ <> TST9
                    CPU D0 <> / 15                             86 \ <> TST8
                      GND -- / 16                               85 \ <> TST7
                     PA7 -> / 17                                 84 \ <> TST6
                    PA6 -> / 18                                   83 \ -- +5V 
                   PA5 -> / 19                                     82 \ <> TST5
                  PA4 -> / 20                                       81 \ <> TST4
                 PA3 -> / 21                                            \
                PA2 -> / 22                                     O       /
               PA1 -> / 23                                          80 / <> TST3
              PA0 -> / 24                                          79 / <> TST2
          HBLANK <- / 25                                          78 / <> TST1
         VBLANK <- / 26              Nintendo 5C78               77 / <> TST0
/PIXEL CLK OUT <- / 27      Package QFP-100, 0.65mm pitch       76 / <- EXT7
     /RESOUT1 <- / 28                                          75 / <- EXT6
   /EXTLATCH -> / 29                  S-PPU2                  74 / <- EXT5 
    PALMODE -> / 30                                          73 / <- EXT4
              /       O                                     72 / <- EXT3
              \                                            71 / <- EXT2
 SYSTEM CLK -> \ 31                                       70 / <- EXT1
         +5V -- \ 32                                     69 / <- EXT0 
     /RESOUT0 <- \ 33                                   68 / -- GND  
        /RESET -> \ 34                                 67 / <- VDA7 
            GND -- \ 35                               66 / <- VDA6 
           FIELD -> \ 36                             65 / <- VDA5 
           /OVER1 -> \ 37                           64 / <- VDA4                     Orientation:
     /PIXEL CLK IN -> \ 38                         63 / <- VDA3                      --------------------
              /HCLD -> \ 39                       62 / <- VDA2                         80         51
               /VCLD -> \ 40                     61 / <- VDA1                           |         |
               COLOR0 -> \ 41                   60 / <- VDA0                           .-----------.
                COLOR1 -> \ 42                 59 / -- +5V                          81-|O Nintendo |-50
                 COLOR2 -> \ 43               58 / <- VDB7                             |   S-PPU2  |
                   PRIO0 -> \ 44             57 / <- VDB6                          100-|    5C78  O|-31
                    PRIO1 -> \ 45           56 / <- VDB5                               \-----------'
                      CHR0 -> \ 46         55 / <- VDB4                                 |         |
                       CHR1 -> \ 47       54 / <- VDB3                                 01         30
                        CHR2 -> \ 48     53 / <- VDB2            
                         CHR3 -> \ 49   52 / <- VDB1                     Legend:
                        /OVER2 -> \ 50 51 / <- VDB0                      ----------------------------
                                   \     /                               --[5C78]-- Power, n/a
                                    \   /                                ->[5C78]<- 5C78 input
                                     \ /                                 <-[5C78]-> 5C78 output
                                      V                                  <>[5C78]<> Bidirectional
                                                                         ??[5C78]?? Unknown
```

### Signal descriptions

- **COLORBURST CLK**: 3.58 MHz clock.
- **/PIXEL CLK IN, /PIXEL CLK OUT**: 5.37 MHz dot clock. In comes from S-PPU1. Out goes to expansion port pin 22.
- **SYSTEM CLK**: 21.47727 MHz clock.
- **/RESOUT0**: S-PPU1 reset.
- **/RESOUT1**: The main reset signal, connected to the CPU, APU, cartridge, and expansion port.
- **/RESET**: Reset from CIC.
- **/EXTLATCH**: Controls H/V counter latching. Normally connected to joypad IO D7, but connected instead on the Sharp SF-1 TV to joypad 2 D1. Can be used with a light pen.
- **/OVER1, /OVER2**: /OVER from S-PPU1.
- **/TRANSPARENT**: This is believed to be high whenever an opaque (sprite or tilemap) pixel is drawn.
- **EXT7..0**: Video input, connected to VDB7..0.
- **DIGITAL VIDEO ENABLE**: When high, TST4..0, TST9..5, and TST14..10 are digital R4..0, G4..0, and B4..0 output. For correct digital video output, this should be connected to /OVER.[[1]](#cite_note-1) As sold, this is connected to ground.
- **TST14..12**: When DIGITAL VIDEO ENABLE is low, these control other kinds of data that can be outputted over the TST pins. Output correlated with VRAM accesses has been observed.[[2]](#cite_note-2) As sold, these are connected to ground.

## References

1. [↑](#cite_ref-1) [Shmups forum thread](https://shmups.system11.org/viewtopic.php?f=6&t=66597): Sharp analog RGB for the 3-Chip SNES using digital signals
2. [↑](#cite_ref-2) [circuit-board forum thread](https://circuit-board.de/forum/index.php/Thread/25396-SNES-Chips-decapped-2PPU-1CHIP-APU-DSP/?postID=702636#post702636): SNES-Chips decapped (2PPU, 1CHIP, APU, DSP)
