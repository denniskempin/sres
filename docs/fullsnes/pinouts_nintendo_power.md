# SNES Pinouts Nintendo Power Flashcarts

#### MX15001TFC

```text
  1 GND         21 SNES_A6     41 VCC         61 FLASH_A13   81 SNES_D7
  2 VCC         22 SNES_A5     42 SRAM_A11    62 FLASH_A14   82 SNES_D6
  3 SNES_A23    23 SNES_A4     43 SRAM_A12    63 FLASH_A15   83 SNES_D5
  4 SNES_A22    24 SNES_A3     44 SRAM_A13    64 FLASH_A16   84 SNES_D4
  5 SNES_A21    25 SNES_A2     45 SRAM_A14    65 FLASH_A17   85 SNES_D3
  6 SNES_A20    26 SNES_A1     46 MEM_A0      66 GND         86 SNES_D2
  7 SNES_A19    27 SNES_A0     47 MEM_A1      67 FLASH_A18   87 SNES_D1
  8 SNES_A18    28 GND         48 MEM_A2      68 FLASH_A19   88 SNES_D0
  9 SNES_A17    29 VCC         49 MEM_A3      69 FLASH_A20   89 FLASH_OE
  10 SNES_A16   30 SNES_21MHZ  50 MEM_A4      70 GND         90 GND
  11 SNES_A15   31 SNES_21MHZ  51 MEM_A5      71 FLASH_CS3   91 VCC
  12 SNES_A14   32 FLASH_WP    52 VCC         72 NC          92 GND
  13 SNES_A13   33 GND         53 GND         73 NC          93 CIC_ERROR
  14 SNES_A12   34 GND         54 MEM_A6      74 FLASH_CS2   94 GND
  15 GND        35 VCC_GOOD    55 MEM_A7      75 FLASH_CS1   95 GND
  16 SNES_A11   36 SRAM_CS     56 MEM_A8      76 FLASH_WE1   96 SNES_RESET1
  17 SNES_A10   37 VCC         57 MEM_A9      77 FLASH_WE2   97 MODESEL2?
  18 SNES_A9    38 MODESEL1?   58 MEM_A10     78 MEM_WE3     98 SNES_WR
  19 SNES_A8    39 SRAM_OE     59 FLASH_A11   79 VCC         99 SNES_RD
  20 SNES_A7    40 GND         60 FLASH_A12   80 GND         100 SNES_RESET2
```

The "MEM_xxx" signals are wired to both FLASH and SRAM.

The CIC_ERROR pin should be held LOW. For PAL (with CIC disabled in the console), the cart does somewhat work when rebooting several times, but it works more reliable when cutting CIC_ERROR (and preferably GNDing pin93, though pin93 seems to be floating/low anyways).
