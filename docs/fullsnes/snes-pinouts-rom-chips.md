# SNES Pinouts ROM Chips

```text
             Standard SNES ROMs                        EPROM-style ROMs
           __________   __________
      GND | 01       \_/       40 | VCC
      GND | 02 ......   .......39 | VCC               ________   ________
  --> A20 | 03 01    \./    36 38 | VCC            ? | 01     \_/     36 | VCC
  GND,A21 | 04 02 ...   ... 35 37 | A22,GND <--    ? | 02 ....   .... 35 | ?
  --> A17 | 05 03 01 \./ 32 34 36 | NC,VCC  <--   NC | 03 01  \./  32 34 | VCC
  --> A18 | 06 04 02     31 33 35 | /CS <--      A16 | 04 02       31 33 | NC
      A15 | 07 05 03     30 32 34 | A19 <--      A15 | 05 03       30 32 | A17
      A12 | 08 06 04     29 31 33 | A14          A12 | 06 04       29 31 | A14
       A7 | 09 07 05 ROM 28 30 32 | A13           A7 | 07 05 EPROM 28 30 | A13
       A6 | 10 08 06     27 29 31 | A8            A6 | 08 06 style 27 29 | A8
       A5 | 11 09 07     26 28 30 | A9            A5 | 09 07 (eg.  26 28 | A9
       A4 | 12 10 08     25 27 29 | A11           A4 | 10 08 in    25 27 | A11
       A3 | 13 11 09     24 26 28 | A16 <--       A3 | 11 09 SGB)  24 26 | /OE
       A2 | 14 12 10     23 25 27 | A10           A2 | 12 10       23 25 | A10
       A1 | 15 13 11     22 24 26 | /RD           A1 | 13 11       22 24 | /CS
       A0 | 16 14 12     21 23 25 | D7            A0 | 14 12       21 23 | D7
       D0 | 17 15 13     20 22 24 | D6            D0 | 15 13       20 22 | D6
       D1 | 18 16 14     19 21 23 | D5            D1 | 16 14       19 21 | D5
       D2 | 19 17 15     18 20 22 | D4            D2 | 17 15       18 20 | D4
      GND | 20 18 16     17 19 21 | D3           GND | 18 16       17 19 | D3
          |_______________________|                  |___________________|
```

Note that Standard SNES ROMs have /CS and A16..A22 located elsewhere as on normal EPROMs. Most common SNES ROMs are 32pin or 36pin (the 40pin ROMs are used by some SPC7110 games; these chips are using a bigger package, though without actually having more address lines). Most SNES carts are using DIP chips (smaller SMD ROMs are used only in carts that contain SMD coprocessors).

Mind that SNES "LoROM" cartridges are leaving SNES.A15 unused (and do instead connect "ROM.A15 and up" to "SNES.A16 and up").

#### 44pin ROMs

```text
          _____   _____
```

#### /WE A22 |  1  \_/  44 | A21 /WP

```text
     A19 |  2       43 | A20
     A18 |  3       42 | A9
     A8  |  4       41 | A10
     A7  |  5       40 | A11
     A6  |  6       39 | A12
     A5  |  7       38 | A13
     A4  |  8       37 | A14
     A3  |  9       36 | A15
     A2  | 10       35 | A16
     A1  | 11       34 | A17
     /CE | 12       33 | BHE (HI)
     GND | 13       32 | GND
     /OE | 14       31 | D15,A0
     D0  | 15       30 | D7
     D8  | 16       29 | D14
     D1  | 17       28 | D6
     D9  | 18       27 | D13
     D2  | 19       26 | D5
     D10 | 20       25 | D12
     D3  | 21       24 | D4
     D11 | 22       23 | VCC
         |_____________|
```

44pin ROMs are used by (some) SPC7110 boards (with 8bit databus), and by (all) S-DD1 and SA-1 boards (existing photos look as if: with 16bit databus).

44pin FLASH is used in Nintendo Power carts (with 8bit databus) (with /WE and /WP instead of A22 and A21).
