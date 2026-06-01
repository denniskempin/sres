---
title: "SA-1 Pinout"
source_url: "https://snes.nesdev.org/wiki/SA-1_Pinout"
pageid: 196
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Pinout of the Super Accelerator (SA-1) Coprocessor

```
                                                 _______
                                        /IRQ <- / 1 128 \ <- RD
                                         D7 <> / 2   127 \ <- Region
                                        D3 <> / 3     126 \ <- /WR
                                       D6 <> / 4       125 \ <> CIC Data 1
                                      D2 <> / 5         124 \ <> CIC Data 2
                                     D5 <> / 6           123 \ <- CIC Reset
                                    D1 <> / 7             122 \ <- CIC Clock
                                   D4 <> / 8               121 \ <- SYSCLK
                                  D0 <> / 9                 120 \ <- /RESET
                                VCC -- / 10                  119 \ -- VCC
                               GND -- / 11                    118 \ -- GND
                              A23 -> / 12                      117 \ <> SRAM D7
                              A0 -> / 13                        116 \ <> SRAM D6
                            A22 -> / 14                          115 \ <> SRAM D5
                            A1 -> / 15                            114 \ <> SRAM D4
                          A21 -> / 16                              113 \ <> SRAM D3
                          A2 -> / 17                                112 \ <> SRAM D2
                        A20 -> / 18                                  111 \ <> SRAM D1
                        A3 -> / 19                                    110 \ <> SRAM D0
                      A19 -> / 20                                      109 \ -> SRAM /WR
                      A4 -> / 21                                        108 \ -> SRAM /RD
                    A18 -> / 22                                          107 \ -> SRAM A19?
                    A5 -> / 23                                            106 \ -> SRAM A17?
                  A17 -> / 24                                              105 \ -> SRAM A15
                  A6 -> / 25                                                104 \ -> SRAM A18?
                A16 -> / 26                                                  103 \ -> SRAM A13
                A7 -> / 27                                                   102 / -> SRAM A8
              A15 -> / 28                                                   101 / -- VCC
              A8 -> / 29                                                   100 / -- GND
            A14 -> / 30                                                    99 / -> SRAM A9
            A9 -> / 31                                                    98 / -> SRAM A11
          A13 -> / 32                                                    97 / -> SRAM A10
         A10 -> / 33                                                    96 / -> SRAM A0
        A12 -> / 34                                                    95 / -> SRAM A1
       A11 -> / 35                                                    94 / -> SRAM A2
      VCC -- / 36                                                    93 / -> SRAM A3
     GND -- / 37                                                    92 / -> SRAM A4
REFRESH -> / 38                                                    91 / -> SRAM A5
    GND -- \ 39                                                   90 / -> SRAM A6
     CLK -> \ 40                                                 89 / -> SRAM A7
      CLK -> \ 41                                               88 / -> SRAM A12
       GND -- \ 42                                             87 / -> SRAM A14
    ROM D15 -> \ 43                                           86 / -> SRAM A16?
      ROM D7 -> \ 44                                         85 / -- GND?
      ROM D14 -> \ 45                                       84 / -- GND
        ROM D6 -> \ 46                                     83 / -- VCC
        ROM D11 -> \ 47                                   82 / -- GND?
          ROM D3 -> \ 48                                 81 / -> ROM A23?
          ROM D10 -> \ 49                               80 / -> ROM A22
            ROM D2 -> \ 50                             79 / -> ROM A21
            ROM D13 -> \ 51                           78 / -> ROM A20
              ROM D5 -> \ 52                         77 / -> ROM A18
              ROM D12 -> \ 53                       76 / -> ROM A19
                ROM D4 -> \ 54                     75 / -> ROM A17
                 ROM D9 -> \ 55                   74 / -> ROM A16
                  ROM D1 -> \ 56                 73 / -> ROM A15
                   ROM D8 -> \ 57               72 / -> ROM A14
                    ROM D0 -> \ 58             71 / -> ROM A13
                     ROM A1 <- \ 59           70 / -> ROM A12
                      ROM A2 <- \ 60         69 / -> ROM A11
                       ROM A3 <- \ 61       68 / -> ROM A10
                        ROM A4 <- \ 62     67 / -> ROM A9
                         ROM A5 <- \ 63   66 / -> ROM A8
                          ROM A6 <- \ 64 65 / -> ROM A7
                                     \_____/
```

- Region: GND = NTSC; VCC = PAL
- SRAM A13: Goes to left-solder Pads
- CLK: 21.5MHz Master Clock

Source: <https://forums.nesdev.org/viewtopic.php?t=9592>
