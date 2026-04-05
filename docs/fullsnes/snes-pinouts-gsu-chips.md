# SNES Pinouts GSU Chips

#### GSU Chip Packages

```text
     100       81           112       85             111       86
      .----------.         .------------.           .------------.
   1 /O          |80      1| O          |84      112|          O |85
    |   MC1      |         |            |          1|            |
    |   GSU1     |         |   GSU2     |           |  GSU2-SP1  |
    |   GSU1A    |         |            |           |            |
  30|            |51     28|            |57       29|            |56
    '------------'         '------------'           '------------'
     31        50           29        56             30        55
```

GSU2-SP1 is having odd pin numbering (with pin1 being the SECOND pin; which was apparently done to maintain same pin numbers as for GSU2).

MC1, GSU1, and GSU1A

```text
  1 GND
  2 ROM.A18
  3 ROM.A17
  4 ROM.A16
  5 ROM.A15
  6 ROM.A14
  7 ROM.A13
  8 ROM.A12
  9 ROM.A11
```

10 ROM.A10  11 ROM.A9  12 ROM.A8  13 ROM.A7  14 ROM.A6  15 ROM.A5  16 ROM.A4  17 ROM.A3  18 ROM.A2  19 ROM.A1  20 ROM.A0  21 ROM.D7  22 ROM.D6  23 ROM.D5  24 ROM.D4  25 ROM.D3  26 ROM.D2  27 GND  28 ROM.D1  29 ROM.D0  30 VCC  -- 31 ?

32 /WR  33 /RD  34 /RESET  35 D7  36 D6  37 D5  38 D4  39 D3  40 GND    ;\swapped on GSU2  41 VCC    ;/  42 D2  43 D1  44 D0  45 A22  46 A21  47 A20  48 A19  49 A18  50 A17  -- 51 A16  52 A15  53 A14  54 A13  55 A12  56 /IRQ  57 A0  58 A1  59 A2  60 A3  61 A4  62 A5  63 A6  64 A7  65 A8  66 A9  67 A10  68 A11  69 GND  70 X1 (21.44MHz ?)  71 SRAM.D0  72 SRAM.D1  73 SRAM.D2  74 SRAM.D3  75 SRAM.D4  76 SRAM.D5  77 SRAM.D6  78 SRAM.D7  79 SRAM.A0  80 SRAM.A1  -- 81 SRAM.A2  82 SRAM.A3  83 SRAM.A4  84 SRAM.A5  85 SRAM.A6  86 SRAM.A7  87 SRAM.A8  88 SRAM.A9  89 VCC  90 GND  91 SRAM.A10  92 SRAM.A11  93 SRAM.A12  94 SRAM.A13  95 SRAM.A14  96 GND  97 SRAM.A15  98 SRAM./OE  99 SRAM./WE  100 ROM.A19

GSU2, and GSU2-SP1

```text
  1 ROM.A17
  2 ROM.A16
  3 ROM.A15
  4 ROM.A14
  5 ROM.A13
  6 ROM.A12
  7 ROM.A11
  8 ROM.A10
  9 ROM.A9
```

10 ROM.A8  11 ROM.A7  12 ROM.A6  13 ROM.A5  14 VCC  15 ROM.A4  16 ROM.A3  17 ROM.A2  18 ROM.A1  19 ROM.A0  20 ROM./CE  21 ?       (NC, probably /CE for 2nd ROM chip)  22 ROM.D7  23 ROM.D6  24 ROM.D5  25 ROM.D4  26 ROM.D3  27 ROM.D2  28 GND?

-- 29 ROM.D1  30 ROM.D0  31 ?

32 ?

33 /WR  34 /RD  35 /RESET  36 GND?

37 D7  38 D6  39 D5  40 D4  41 D3  42 VCC  43 GND  44 D2  45 D1  46 D0  47 A23  48 A22  49 A21  50 A20  51 A19  52 A18  53 A17  54 A16  55 A15  56 A14  -- 57 A13  58 A12  59 /IRQ  60 A0  61 A1  62 A2  63 A3  64 A4  65 GND?

66 A5  67 A6  68 A7  69 A8  70 VCC  71 A9  72 A10  73 A11  74 GND  75 X1 (21.44MHz)  76 VCC  77 SRAM.D0  78 SRAM.D1  79 SRAM.D2  80 SRAM.D3  81 SRAM.D4  82 SRAM.D5  83 SRAM.D6  84 SRAM.D7  -- 85 SRAM.A0  86 SRAM.A1  87 SRAM.A2  88 SRAM.A3  89 SRAM.A4  90 SRAM.A5  91 SRAM.A6  92 SRAM.A7  93 SRAM.A8  94 SRAM.A9  95 NC?

96 NC?

97 VCC  98 VCC  99 GND  100 SRAM.A10  101 SRAM.A11  102 SRAM.A12  103 SRAM.A13  104 SRAM.A14  105 NC/SRAM.A15  106 NC/SRAM.A16  107 SRAM./OE  108 SRAM./WE  109 ROM.A20  110 ROM.A19  111 GND  112 ROM.A18
