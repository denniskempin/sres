# SNES Pinouts RAM Chips

#### 128K WRAM Pinouts

```text
  1 VCC      9  -            17 GND 25 A1  33 GND 41 A8      49 VCC    57 /RD
  2 D4       10 CS,VCC       18 -   26 A10 34 A13 42 ENA,A22 50 PS,PA7 58 /PAWR
  3 D5       11 CS,VCC       19 -   27 A2  35 A5  43 /PS,PA2 51 PS,VCC 59 /WR
  4 D6       12 CS,VCC       20 -   28 A11 36 A14 44 /PS,PA3 52 PS,VCC 60 D0
  5 D7       13 /CS,GND      21 -   29 A3  37 A6  45 /PS,PA4 53 PA0    61 D1
  6 SYSCK    14 /CS,GND      22 -   30 A12 38 A15 46 /PS,PA5 54 PA1    62 D2
  7 REFRESH  15 /CS,/WRAMSEL 23 A0  31 A4  39 A7  47 /PS,PA6 55 G (NC) 63 D3
  8 /RESET   16 VCC          24 A9  32 VCC 40 A16 48 GND     56 /PARD  64 GND
```

Note: The WRAM is Dynamic RAM (DRAM) and does require REFRESH pulses. If REFRESH is disabled (via DRAMMODE pin) then WRAM forgets its content after some minutes. Whereas, refresh occurs also on /RD (with the refresh row output on A0..A8), so, for example, games that are DMA'ing 512 bytes from WRAM to OAM in every frame should work perfectly without REFRESH.

Interestingly, WRAM is kept intact even if the RESET button is held down for about 30 minutes (although the CPU doesn't generate any /RD, REFRESH, nor A0..A8 cycles during that time).

#### SRAM Pinouts

```text
         .-------------__-------------.
      NC |1                         36| VCC
     A20 |2  ..........__.......... 35| A19
  NC,A18 |3  1                   32 34| NC,VCC (NC, ie. not CE2)
     A16 |4  2  .......__....... 31 33| A15
  NC,A14 |5  3  1             28 30 32| A17,CE2,VCC
     A12 |6  4  2  ....__.... 27 29 31| /WE
      A7 |7  5  3  1       24 26 28 30| A13,CE2,VCC
      A6 |8  6  4  2       23 25 27 29| A8
      A5 |9  7  5  3       22 24 26 28| A9
      A4 |10 8  6  4  SRAM 21 23 25 27| A11,/WE
      A3 |11 9  7  5       20 22 24 26| /OE
      A2 |12 10 8  6       19 21 23 25| A10
      A1 |13 11 9  7       18 20 22 24| /CE,/CE1
      A0 |14 12 10 8       17 19 21 23| D7
      D0 |15 13 11 9       16 18 20 22| D6
      D1 |16 14 12 10      15 17 19 21| D5
      D2 |17 15 13 11      14 16 18 20| D4
     GND |18 16 14 12      13 15 17 19| D3
         '----------------------------'
```

28pin 32Kbyte SRAM is used for Video RAM and Sound RAM (on mainboard) Various SRAM sizes are used in game cartridges.
