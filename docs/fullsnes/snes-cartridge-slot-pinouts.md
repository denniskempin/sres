# SNES Cartridge Slot Pinouts

#### Cartridge Slot 62 pins (31x2 pins)

Most cartridges are using only the middle 46 pins.

```text
  Front/Round    Rear/Flat
  Solder side    Component side
  MCK 21M - 01   32 - /WRAMSEL
  EXPAND  - 02   33 - REFRESH
  PA6     - 03   34 - PA7
  /PARD   - 04   35 - /PAWR
             <key>
  GND     - 05   36 - GND
  A11     - 06   37 - A12
  A10     - 07   38 - A13
  A9      - 08   39 - A14
  A8      - 09   40 - A15
  A7      - 10   41 - A16
  A6      - 11   42 - A17
  A5      - 12   43 - A18
  A4      - 13   44 - A19
  A3      - 14   45 - A20
  A2      - 15   46 - A21
  A1      - 16   47 - A22
  A0      - 17   48 - A23
  /IRQ    - 18   49 - /ROMSEL
  D0      - 19   50 - D4
  D1      - 20   51 - D5
  D2      - 21   52 - D6
  D3      - 22   53 - D7
  /RD     - 23   54 - /WR
  CIC0    - 24   55 - CIC1
  CIC2    - 25   56 - CIC3 3.072MHz (or 4.00MHz on older SNES)
  /RESET  - 26   57 - SYSCK
  +5V     - 27   58 - +5V
             <key>
  PA0     - 28   59 - PA1
  PA2     - 29   60 - PA3
  PA4     - 30   61 - PA5
  SOUND-L - 31   62 - SOUND-R
  GND     - SHIELD  - GND
```

Caution: The connector uses a nonstandard 2.5mm pitch (not 2.54mm). And, the PCB is only 1.2mm thick (not 1.5mm).

The width of the key gaps equals to 2 pins each (ie. the overall connector size is 35x2 pins, with 31x2 used pins, and two unused 2x2 pin clusters).

Pin assignments  A23-0, D7-0, /WR, /RD - CPU address/data bus, read/write signals  /IRQ      - Interrupt Request (used by SA-1 and GSU)  /RESET    - When the system is reset (power-up or hard reset) this goes low  /WRAMSEL  - Work RAM select (00-3F,80-BF:0000-1FFF, 7E-7F:0000-FFFF)  /ROMSEL   - Cart ROM select (00-3F,80-BF:8000-FFFF, 40-7D,C0-FF:0000-FFFF)  PA7-0     - Address bus for $2100-$21FF range in banks $00-$3F/$80-$BF (B-Bus)  /PAWR     - Write strobe for B-Bus  /PARD     - Read strobe for B-Bus  MCK       - 21.47727 MHz master clock (used by SGB1 and MarioChip1)  SYSCK     - Unknown, is an output from the CPU.

SOUND-L/R - Left/Right Analog Audio Input, mixed with APU output (SGB, MSU1)  EXPAND    - Connected to pin 24 of the EXT expansion port (for Satellaview)  REFRESH   - DRAM refresh (connects to WRAM, also used by SGB and SA-1)

```text
              four HIGH pulses every 60us (every scanline)
              Used by SGB (maybe to sense SNES hblanks?)
```

CIC0      - Lockout Data to CIC chip in console    ;\from/to=initial direction  CIC1      - Lockout Data from CIC chip in console  ;/(on random-seed transfer)  CIC2      - Lockout Start (short HIGH pulse when releasing reset button)  CIC3      - Lockout Clock (3.072MHz) (24.576MHz/8 from APU) (or 4.00MHz)  SHIELD    - GND (connected in SA-1 carts, SGB-also has provisions)

#### Physical Cartridge Shape

```text
                            Front Side
        _________________                _________________
   .--''                 ''--.    .-----'                 '-----.
  /    Japan NTSC and PAL     \   |           US NTSC           |
  |___________________________|   |_:":_____________________:":_|

                             Rear Side
```
