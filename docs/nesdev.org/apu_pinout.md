---
title: "APU pinout"
source_url: "https://snes.nesdev.org/wiki/APU_pinout"
pageid: 7
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

## S-SMP

### Pinout

```
                                    / \
                                   /   \
                                  /     \
                       SMP A4 <- / 1  64 \ -> SMP A5
                      SMP A3 <- / 2    63 \ -> SMP A6
                     SMP A2 <- / 3   .  62 \ -> SMP A7
                    SMP A1 <- / 4        61 \ -> SMP A8
                   SMP A0 <- / 5          60 \ -> SMP A9
                  SMP D7 <> / 6            59 \ -> SMP A10
                 SMP D6 <> / 7              58 \ -- GND
                SMP D5 <> / 8                57 \ -- +5V
               SMP D4 <> / 9                  56 \ -> SMP A11
              SMP D3 <> / 10                   55 \ -> SMP A12
             SMP D2 <> / 11                     54 \ -> SMP A13
            SMP D1 <> / 12                       53 \ -> SMP A14
           SMP D0 <> / 13                         52 \ -> SMP A15
 "PD3" SMP CLKEN -> / 14                              \
  "PD2" SMP R/W <- / 15                        O      /
"CPUK" SMP CLK -> / 16      Nintendo S-SMP        51 / <- PA7 (/CS)
        /P5RD <- / 17      Package QFP-64        50 / <- PA6 (CS)
         P57 <> / 18   1mm pitch (20mm × 14mm)  49 / <- PA0
        P56 <> / 19                            48 / <- PA1
              /      O                        47 / <- /PWR
              \                              46 / <- /PRD                 Orientation:
        P55 <> \ 20                         45 / <> CPU D0                --------------------
         P54 <> \ 21                       44 / <> CPU D1                    51         33
          P53 <> \ 22                     43 / <> CPU D2                      |         |
           P52 <> \ 23                   42 / <> CPU D3                      .-----------.
            P51 <> \ 24                 41 / <> CPU D4                    52-|O Nintendo |-32
             P50 <> \ 25               40 / <> CPU D5                        |   S-SMP   |
              GND -- \ 26             39 / <> CPU D6                      64-|.         O|-20
               P47 <> \ 27           38 / <> CPU D7                          '-----------'
                P46 <> \ 28         37 / <- /RESET                            |         |
                 P45 <> \ 29       36 / <- T0                                01         19
                  P44 <> \ 30     35 / <- T1                              
                   P43 <> \ 31   34 / <> P40                              Legend: 
                    P42 <> \ 32 33 / <> P41                               ---------------------------- 
                            \     /                                       --[S-SMP]-- Power, n/a 
                             \   /                                        ->[S-SMP]<- S-SMP input 
                              \ /                                         <-[S-SMP]-> S-SMP output 
                               V                                          <>[S-SMP]<> Bidirectional
```

## S-DSP

### Pinout

```
                                         ^
                                        / \
                                       /   \
                                      /     \
                                  <- / 1  80 \ ->
                 BUS ARBITER CLK <- / 2    79 \ ->
            BUS ARBITER STATUS1 <- / 3   .  78 \ -> DCK
           BUS ARBITER STATUS2 <- / 4    O   77 \ ->
          BUS ARBITER STATUS3 <- / 5          76 \ <- SMP A15
                  DSP RAM D2 <> / 6            75 \ <- SMP A14
                 DSP RAM D1 <> / 7              74 \ <- SMP A13
                DSP RAM D0 <> / 8                73 \ -- +5V
               DSP RAM A0 <- / 9                  72 \ <- SMP A12
              DSP RAM A1 <- / 10                   71 \ <- SMP A11
             DSP RAM A2 <- / 11                     70 \ <- SMP A10
                   GND -- / 12                       69 \ <- SMP A9
           DSP RAM A3 <- / 13                         68 \ <- SMP A8
          DSP RAM A4 <- / 14                           67 \ <- SMP A7
         DSP RAM A5 <- / 15                             66 \ <- SMP A6
        DSP RAM A6 <- / 16                               65 \ <- SMP A5
       DSP RAM A7 <- / 17                                    \
     DSP RAM A12 <- / 18                                     /
    DSP RAM A14 <- / 19                                  64 / <- SMP A4
   DSP RAM A15 <- / 20                                  63 / <- SMP A3
          DIP ?? / 21          Nintendo S-DSP          62 / <- SMP A2
  DSP RAM D3 <> / 22          Package QFP-80          61 / <- SMP A1
 DSP RAM D4 <> / 23     0.8mm pitch (20mm × 14mm)    60 / <- SMP A0
DSP RAM D5 <> / 24                                  59 / <- SMP D7
             /                                     58 / <- SMP D6
             \                                    57 / <- SMP D5
DSP RAM D6 <> \ 25                               56 / <- SMP D4
 DSP RAM D7 <> \ 26                             55 / <- SMP D3          Orientation:
DSP RAM /CE1 <- \ 27                           54 / <- SMP D2           --------------------
 DSP RAM /CE0 <- \ 28                         53 / <- SMP D1               64         41
   DSP RAM A10 <- \ 29                       52 / -- GND                    |         |
    DSP RAM /OE <- \ 30                     51 / <- SMP D0                 .-----------.
     DSP RAM A11 <- \ 31                   50 / -> SMP CLKEN "PD3"      65-| Nintendo O|-40
       DSP RAM A9 <- \ 32                 49 / <- SMP R/W "PD2"            |   S-DSP   |
               +5V -- \ 33               48 / -> SMP CLK "CPUK"         80-|.O         |-25
         DSP RAM A8 <- \ 34             47 / <- /RESET                     '-----------'
         DSP RAM A13 <- \ 35           46 / <- CRYSTAL IN                   |         |
          DSP RAM /WE <- \ 36    O    45 / -> CRYSTAL OUT                  01         24
                    TF -> \ 37       44 / -> DAC DATA                   
                     TK -> \ 38     43 / -> DAC LEFT/RIGHT CLK          Legend: 
                   /MUTE <- \ 39   42 / -> DAC BIT CLK                  ----------------------------
                          <- \ 40 41 / -> CIC CLK                       --[S-DSP]-- Power, n/a  
                              \     /                                   ->[S-DSP]<- S-DSP input 
                               \   /                                    <-[S-DSP]-> S-DSP output 
                                \ /                                     <>[S-DSP]<> Bidirectional
                                 V                                      ??[S-DSP]?? Unknown

Note: The mold ejection pins are near pins 32/33 and pins 72/73 on some chips.
```

### Signal descriptions

- **CIC CLK**: 3.072 MHz clock, used for the CIC on some consoles.
- **CPU CLK** / **CPUK**: 2.048 MHz
- **CPU CLKEN** / **PD3**: 1.024 MHz, 75% duty cycle
- **CRYSTAL IN**: 24.576 MHz clock. In practice, this clock varies by console and temperature and has been observed in the range 24.584218 MHz to 24.667392 MHz.[[1]](#cite_note-1) *An American Tail: Fievel Goes West* does not work properly with the advertised 24.576 MHz clock.

## SHVC-SOUND Module

### Pinout

```
       SNES |Module| SNES
             ______
            |      |
     PA6 -> | 02   |
            |   01 | <- PA7
     PA1 -> | 04   |
            |   03 | <- PA0
    /PRD -> | 06   |
            |   05 | <- /PWR
  CPU D1 <> | 08   |
            |   07 | <> CPU D0
  CPU D3 <> | 10   |
            |   09 | <> CPU D2
  CPU D5 <> | 12   |
            |   11 | <> CPU D4
  CPU D7 <> | 14   |                     Orientation:
            |   13 | <> CPU D6           ------------------------
   (n/c) -- | 16   |                      ________________________
            |   15 | <- /RESET           |  1________23       | O |
     +5V -- | 18   |                     |  |________|        \___|
            |   17 | -> DCK              | 2       24             |
   /MUTE <- | 20   |                     |             ____       |
            |   19 | -- GND              |             ____>      |
 Audio-R <- | 22   |                     |             ____       |
            |   21 | -> Audio-L          |             ____>      |
AudioVcc -- | 24   |                     |__     SHVC-SOUND     __|
            |   23 | -- GND              | O\                  /O |
            |______|                     |___|________________|___|
```

Note that the numbering of this connector zig-zags in a somewhat counter-intuitive way.

SHVC-SOUND Module pinout source[[2]](#cite_note-2)

## S-APU

The S-APU was used in Later Revisions and combines both the S-SMP and the S-DSP into a single Chip.

### Pinout

```
                                     _______
                             /CS -> / 1 100 \ <- /RESET
                            +CS -> / 2    99 \ -> /RESOUT0
                           VCC -- / 3      98 \ ?? ?
                          GND -- / 4        97 \ ?? ?
                         /WR -> / 5          96 \ ?? ?
                        /RD -> / 6            95 \ ?? ?
                        D4 <> / 7              94 \ -> DAC Channel
                       D0 <> / 8                93 \ -> DAC Data
                      D5 <> / 9                  92 \ -> DAC Clock
                     D1 <> / 10                   91 \ <- MUTE?
                    D6 <> / 11                     90 \ -- GND
                   D2 <> / 12                       89 \ ?? ?
                  D7 <> / 13                         88 \ ?? ?
                 D3 <> / 14                           87 \ ?? ?
               GND -- / 15                             86 \ ?? ?
        /RESOUT1? <- / 16                               85 \ ?? ?
              A1 -> / 17                                 84 \ ?? ?
             A0 -> / 18                                   83 \ ?? ?
             ? ?? / 19                                     82 \ ?? ?
            ? ?? / 20                                       81 \ ?? ?
           ? ?? / 21                                        80 / ?? ?
          ? ?? / 22                                        79 / -- GND
         ? ?? / 23                                        78 / -- VCC
        ? ?? / 24                                        77 / ?? ?
       ? ?? / 25                                        76 / ?? ?
      ? ?? / 26                                        75 / ?? ?
     ? ?? / 27                                        74 / ?? ?
  VCC -- / 28                                        73 / ?? ?
   ? ?? / 29                                        72 / ?? ?
TST -> / 30                                        71 / ?? ?
  ? ?? \ 31                                       70 / ?? ?
   ? ?? \ 32                                     69 / ?? ?
    ? ?? \ 33                                   68 / ?? ?
     ? ?? \ 34                                 67 / ?? ?
      ? ?? \ 35                               66 / ?? ?
       ? ?? \ 36                             65 / -- GND
        ? ?? \ 37                           64 / ?? ?
         ? ?? \ 38                         63 / ?? ?
          ? ?? \ 39                       62 / ?? ?
           ? ?? \ 40                     61 / ?? ?
            ? ?? \ 41                   60 / ?? ?
             ? ?? \ 42                 59 / ?? ?
              ? ?? \ 43               58 / ?? ?
               ? ?? \ 44             57 / ?? ?
                ? ?? \ 45           56 / ?? ?
                 ? ?? \ 46         55 / ?? ?
                  ? ?? \ 47       54 / -- GND
                   ? ?? \ 48     53 / -- VCC
                    ? ?? \ 49   52 / ?? ?
                     ? ?? \ 50 51 / ?? ?
                           -------
```

### Signal Descriptions

- **?**: Unknown. All these pins are Connected to GND. They are Most Likely there because the size of the Die.

## References

<https://videogameperfection.com/forums/topic/schematic-for-1chip-pal-snes/>

1. [↑](#cite_ref-1) [plgDavid spreadsheet](https://docs.google.com/spreadsheets/d/1WIrmYrPJXcihIIJofMeVcLIY3g0jXLSMxvC9SgJz42Q/edit#gid=0): Real world S-DSP clock rates
2. [↑](#cite_ref-2) [SNES SHVC-SOUND Connector Pinout by Jonathon W. Donaldson (jwdonal) (special thanks to caitsith2)](https://gamesx.com/wiki/lib/exe/fetch.php?media=schematics:pinout_shvc-sound_connector.pdf)
