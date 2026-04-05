---
title: "S-CPUN Pinout"
source_url: "https://snes.nesdev.org/wiki/S-CPUN_Pinout"
pageid: 187
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Pinout of S-CPUN (Ricoh 5A122) used in 1CHIP Consoles. This Chip combines the CPU and both PPUs into a single IC.

```
                                                _______
                                          ? ?? / 1 160 \ -- GND
                                         ? ?? / 2   159 \ ?? TST16?
                                      GND -- / 3     158 \ -> Blue Channel
                                    XOUT <- / 4       157 \ -> Green Channel
                                  TST0? ?? / 5         156 \ -> Red Channel
                                 HSYNC <- / 6           155 \ -- VCC
                                  GND -- / 7             154 \ ?? ?
                               XTAL0 <> / 8               153 \ ?? ?
                              XTAL1 <> / 9                 152 \ -> BFP
                               VCC -- / 10                  151 \ -> CSYNC
                            TST1? ?? / 11                    150 \ -> SC
                           TST2? ?? / 12                      149 \ -- GND
                          TST3? ?? / 13                        148 \ -> A12
                         TST4? ?? / 14                          147 \ -> A11
                        TST5? ?? / 15                            146 \ -> A13
                       TST6? ?? / 16                              145 \ -> A10
                      TST7? ?? / 17                                144 \ -> A14
                     TST8? ?? / 18                                  143 \ -> A9
                    TST9? ?? / 19                                    142 \ -> A15
                  TST10? ?? / 20                                      141 \ -> A8
                 TST11? ?? / 21                                        140 \ -> A16
                TST12? ?? / 22                                          139 \ -> A7
                  VCC -- / 23                                            138 \ -> A17
                VDB3 <> / 24                                              137 \ -> A6
               VDB4 <> / 25                                                136 \ -> A18
              VDB5 <> / 26                                                  135 \ -> A5
             VDB6 <> / 27                                                    134 \ -> A19
            VDB7 <> / 28                                                      133 \ -> A4
          VAB10 <- / 29                                                        132 \ -> A20
         VAB11 <- / 30                                                          131 \ -> A3
         VAB9 <- / 31                                                            130 \ -> A21
        VAB8 <- / 32                                                              129 \ -> A2
      VAB13 <- / 33                                                                128 \ -> A22
     /VBWR <- / 34                                                                  127 \ -> A1
     VA14 <- / 35                                                                    126 \ -> A23
   VAB12 <- / 36                                                                      125 \ -> A0
   VAB7 <- / 37                                                                        124 \ -> /ROMSEL
  VAB6 <- / 38                                                                          123 \ <- /IRQ
 VAB5 <- / 39                                                                            122 \ -> Lock CIC CLK
VAB4 <- / 40                                                                              121 \ -- VCC
VAB3 <- \ 41                                                                              120 / -> System Clock
 VAB2 <- \ 42                                                                            119 / -- GND
  VAB1 <- \ 43                                                                          118 / -> Key CIC CLK
   VAB0 <- \ 44                                                                        117 / ?? TST17?
    VDB0 <> \ 45                                                                      116 / -> CPU /WR
     VDB1 <> \ 46                                                                    115 / -> CPU /RD
      VDB2 <> \ 47                                                                  114 / - GND
        GND -- \ 48                                                                113 / ?? TST14?
         VCC -- \ 49                                                              112 / ?? TST13?
         VDA3 <> \ 50                                                            111 / <- REGION
          VDA4 <> \ 51                                                          110 / -- VCC
           VDA5 <> \ 52                                                        109 / -> PA7
            VDA6 <> \ 53                                                      108 / -> PA6
             VDA7 <> \ 54                                                    107 / -> PA5
             VAA10 <- \ 55                                                  106 / -> PA4
               /VRD <- \ 56                                                105 / -> PA3
               VAA11 <- \ 57                                              104 / -> PA2
                 VAA9 <- \ 58                                            103 / -> PA1
                  VAA8 <- \ 59                                          102 / -> PA0
                  VAA13 <- \ 60                                        101 / -- VCC
                   /VAWR <- \ 61                                      100 / -> PAWR
                    VAA12 <- \ 62                                     99 / -> /PARD
                      VAA7 <- \ 63                                   98 / -- GND
                       VAA6 <- \ 64                                 97 / -> REFRESH
                        VAA5 <- \ 65                               96 / -> /RAMSEL
                         VAA4 <- \ 66                             95 / <> D4
                          VAA3 <- \ 67                           94 / <> D0
                           VAA2 <- \ 68                         93 / <> D5
                            VAA1 <- \ 69                       92 / <> D1
                             VAA0 <- \ 70                     91 / <> D6
                              VDA0 <> \ 71                   90 / <> D2
                               VDA1 <> \ 72                 89 / <> D7
                                VDA2 <> \ 73               88 / <> D3
                                  GND -- \ 74             87 / <- ? (Connects to /RESOUT0 on S-APU, Pulled High, Most Likely Input)
                                 JPIO6 <> \ 75           86 / <- /RESET
                                $4016.1 <- \ 76         85 / -> JPIO7
                                 $4016.0 <- \ 77       84 / -> JPIO7
                                    JPSTR <- \ 78     83 / -> $4017.1
                                    JPCLK1 <- \ 79   82 / -> $4017.0
                                        VCC -- \ 80 81 / -> JPCLK2
                                                -------
```

- TSTx?: Maybe PPU TST Pins? Unsure. TST13 and 14 are tied Together; TST16 is tied to GND
- VABx: PPU Address Bus for Second RAM Chip
- VDBx: PPU Data Bus for Second RAM Chih
- XOUT: System Clock
- /VRD: /RD for Both RAM Chips
- /VBWR: /WR for Second RAM Chip
- /VAWR: /WR for First RAM Chip
- VAAx: PPU Address Bus for First RAM Chip
- JPIOx: Joypad IO Bit x
- JPSTR: Joypad Strobe
- JPCLKx: Joypad Clock x
- Dx: CPU Data
- Ax: CPU Address
- REFRESH: Refresh Signal for DRAM
- REGION: Region Select (GND = NTSC; VCC = PAL)
- SC: Goes to Pin 10 of S-RGB Encoder
- BFP: Goes to Pin 8 of S-RGB

Based on [this Post](https://videogameperfection.com/forums/topic/schematic-for-1chip-pal-snes/)
