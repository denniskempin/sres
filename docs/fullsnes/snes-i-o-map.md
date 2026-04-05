# SNES I/O Map

#### First some bytes

```text
  0000h..1FFFh - WRAM - Mirror of first 8Kbyte of WRAM (at 7E0000h-7E1FFFh)
  2000h..20FFh - N/A  - Unused
```

#### PPU Picture Processing Unit (Write-Only Ports)

```text
  2100h - INIDISP - Display Control 1                                  8xh
  2101h - OBSEL   - Object Size and Object Base                        (?)
  2102h - OAMADDL - OAM Address (lower 8bit)                           (?)
  2103h - OAMADDH - OAM Address (upper 1bit) and Priority Rotation     (?)
  2104h - OAMDATA - OAM Data Write (write-twice)                       (?)
  2105h - BGMODE  - BG Mode and BG Character Size                      (xFh)
  2106h - MOSAIC  - Mosaic Size and Mosaic Enable                      (?)
  2107h - BG1SC   - BG1 Screen Base and Screen Size                    (?)
  2108h - BG2SC   - BG2 Screen Base and Screen Size                    (?)
  2109h - BG3SC   - BG3 Screen Base and Screen Size                    (?)
  210Ah - BG4SC   - BG4 Screen Base and Screen Size                    (?)
  210Bh - BG12NBA - BG Character Data Area Designation                 (?)
  210Ch - BG34NBA - BG Character Data Area Designation                 (?)
  210Dh - BG1HOFS - BG1 Horizontal Scroll (X) (write-twice) / M7HOFS   (?,?)
  210Eh - BG1VOFS - BG1 Vertical Scroll (Y)   (write-twice) / M7VOFS   (?,?)
  210Fh - BG2HOFS - BG2 Horizontal Scroll (X) (write-twice)            (?,?)
  2110h - BG2VOFS - BG2 Vertical Scroll (Y)   (write-twice)            (?,?)
  2111h - BG3HOFS - BG3 Horizontal Scroll (X) (write-twice)            (?,?)
  2112h - BG3VOFS - BG3 Vertical Scroll (Y)   (write-twice)            (?,?)
  2113h - BG4HOFS - BG4 Horizontal Scroll (X) (write-twice)            (?,?)
  2114h - BG4VOFS - BG4 Vertical Scroll (Y)   (write-twice)            (?,?)
  2115h - VMAIN   - VRAM Address Increment Mode                        (?Fh)
  2116h - VMADDL  - VRAM Address (lower 8bit)                          (?)
  2117h - VMADDH  - VRAM Address (upper 8bit)                          (?)
  2118h - VMDATAL - VRAM Data Write (lower 8bit)                       (?)
  2119h - VMDATAH - VRAM Data Write (upper 8bit)                       (?)
  211Ah - M7SEL   - Rotation/Scaling Mode Settings                     (?)
  211Bh - M7A     - Rotation/Scaling Parameter A & Maths 16bit operand(FFh)(w2)
  211Ch - M7B     - Rotation/Scaling Parameter B & Maths 8bit operand (FFh)(w2)
  211Dh - M7C     - Rotation/Scaling Parameter C         (write-twice) (?)
  211Eh - M7D     - Rotation/Scaling Parameter D         (write-twice) (?)
  211Fh - M7X     - Rotation/Scaling Center Coordinate X (write-twice) (?)
  2120h - M7Y     - Rotation/Scaling Center Coordinate Y (write-twice) (?)
  2121h - CGADD   - Palette CGRAM Address                              (?)
  2122h - CGDATA  - Palette CGRAM Data Write             (write-twice) (?)
  2123h - W12SEL  - Window BG1/BG2 Mask Settings                       (?)
  2124h - W34SEL  - Window BG3/BG4 Mask Settings                       (?)
  2125h - WOBJSEL - Window OBJ/MATH Mask Settings                      (?)
  2126h - WH0     - Window 1 Left Position (X1)                        (?)
  2127h - WH1     - Window 1 Right Position (X2)                       (?)
  2128h - WH2     - Window 2 Left Position (X1)                        (?)
  2129h - WH3     - Window 2 Right Position (X2)                       (?)
  212Ah - WBGLOG  - Window 1/2 Mask Logic (BG1-BG4)                    (?)
  212Bh - WOBJLOG - Window 1/2 Mask Logic (OBJ/MATH)                   (?)
  212Ch - TM      - Main Screen Designation                            (?)
  212Dh - TS      - Sub Screen Designation                             (?)
  212Eh - TMW     - Window Area Main Screen Disable                    (?)
  212Fh - TSW     - Window Area Sub Screen Disable                     (?)
  2130h - CGWSEL  - Color Math Control Register A                      (?)
  2131h - CGADSUB - Color Math Control Register B                      (?)
  2132h - COLDATA - Color Math Sub Screen Backdrop Color               (?)
  2133h - SETINI  - Display Control 2                                  00h?
```

#### PPU Picture Processing Unit (Read-Only Ports)

```text
  2134h - MPYL    - PPU1 Signed Multiply Result   (lower 8bit)         (01h)
  2135h - MPYM    - PPU1 Signed Multiply Result   (middle 8bit)        (00h)
  2136h - MPYH    - PPU1 Signed Multiply Result   (upper 8bit)         (00h)
  2137h - SLHV    - PPU1 Latch H/V-Counter by Software (Read=Strobe)
  2138h - RDOAM   - PPU1 OAM Data Read            (read-twice)
  2139h - RDVRAML - PPU1 VRAM Data Read           (lower 8bits)
  213Ah - RDVRAMH - PPU1 VRAM Data Read           (upper 8bits)
  213Bh - RDCGRAM - PPU2 CGRAM Data Read (Palette)(read-twice)
  213Ch - OPHCT   - PPU2 Horizontal Counter Latch (read-twice)         (01FFh)
  213Dh - OPVCT   - PPU2 Vertical Counter Latch   (read-twice)         (01FFh)
  213Eh - STAT77  - PPU1 Status and PPU1 Version Number
  213Fh - STAT78  - PPU2 Status and PPU2 Version Number                Bit7=0
```

#### APU Audio Processing Unit (R/W)

```text
  2140h - APUI00  - Main CPU to Sound CPU Communication Port 0        (00h/00h)
  2141h - APUI01  - Main CPU to Sound CPU Communication Port 1        (00h/00h)
  2142h - APUI02  - Main CPU to Sound CPU Communication Port 2        (00h/00h)
  2143h - APUI03  - Main CPU to Sound CPU Communication Port 3        (00h/00h)
  2144h..217Fh    - APU Ports 2140-2143h mirrored to 2144h..217Fh
```

#### WRAM Access

```text
  2180h - WMDATA  - WRAM Data Read/Write       (R/W)
  2181h - WMADDL  - WRAM Address (lower 8bit)  (W)                        00h
  2182h - WMADDM  - WRAM Address (middle 8bit) (W)                        00h
  2183h - WMADDH  - WRAM Address (upper 1bit)  (W)                        00h
  2184h..21FFh    - Unused region (open bus) / Expansion (B-Bus)          -
  2200h..3FFFh    - Unused region (open bus) / Expansion (A-Bus)          -
```

#### CPU On-Chip I/O Ports

```text
  4000h..4015h        - Unused region (open bus)      ;\These ports have  -
  4016h/Write - JOYWR - Joypad Output (W)             ; long waitstates   00h
  4016h/Read  - JOYA  - Joypad Input Register A (R)   ; (1.78MHz cycles)  -
  4017h/Read  - JOYB  - Joypad Input Register B (R)   ; (all other ports  -
  4018h..41FFh        - Unused region (open bus)      ;/are 3.5MHz fast)  -
```

#### CPU On-Chip I/O Ports (Write-only) (Read=open bus)

```text
  4200h - NMITIMEN- Interrupt Enable and Joypad Request                   00h
  4201h - WRIO    - Joypad Programmable I/O Port (Open-Collector Output)  FFh
  4202h - WRMPYA  - Set unsigned 8bit Multiplicand                        (FFh)
  4203h - WRMPYB  - Set unsigned 8bit Multiplier and Start Multiplication (FFh)
  4204h - WRDIVL  - Set unsigned 16bit Dividend (lower 8bit)              (FFh)
  4205h - WRDIVH  - Set unsigned 16bit Dividend (upper 8bit)              (FFh)
  4206h - WRDIVB  - Set unsigned 8bit Divisor and Start Division          (FFh)
  4207h - HTIMEL  - H-Count Timer Setting (lower 8bits)                   (FFh)
  4208h - HTIMEH  - H-Count Timer Setting (upper 1bit)                    (01h)
  4209h - VTIMEL  - V-Count Timer Setting (lower 8bits)                   (FFh)
  420Ah - VTIMEH  - V-Count Timer Setting (upper 1bit)                    (01h)
  420Bh - MDMAEN  - Select General Purpose DMA Channel(s) and Start Transfer 0
  420Ch - HDMAEN  - Select H-Blank DMA (H-DMA) Channel(s)                    0
  420Dh - MEMSEL  - Memory-2 Waitstate Control                               0
  420Eh..420Fh    - Unused region (open bus)                                 -
```

#### CPU On-Chip I/O Ports (Read-only)

```text
  4210h - RDNMI   - V-Blank NMI Flag and CPU Version Number (Read/Ack)      0xh
  4211h - TIMEUP  - H/V-Timer IRQ Flag (Read/Ack)                           00h
  4212h - HVBJOY  - H/V-Blank flag and Joypad Busy flag (R)                 (?)
  4213h - RDIO    - Joypad Programmable I/O Port (Input)                    -
  4214h - RDDIVL  - Unsigned Division Result (Quotient) (lower 8bit)        (0)
  4215h - RDDIVH  - Unsigned Division Result (Quotient) (upper 8bit)        (0)
  4216h - RDMPYL  - Unsigned Division Remainder / Multiply Product (lower 8bit)
  4217h - RDMPYH  - Unsigned Division Remainder / Multiply Product (upper 8bit)
  4218h - JOY1L   - Joypad 1 (gameport 1, pin 4) (lower 8bit)               00h
  4219h - JOY1H   - Joypad 1 (gameport 1, pin 4) (upper 8bit)               00h
  421Ah - JOY2L   - Joypad 2 (gameport 2, pin 4) (lower 8bit)               00h
  421Bh - JOY2H   - Joypad 2 (gameport 2, pin 4) (upper 8bit)               00h
  421Ch - JOY3L   - Joypad 3 (gameport 1, pin 5) (lower 8bit)               00h
  421Dh - JOY3H   - Joypad 3 (gameport 1, pin 5) (upper 8bit)               00h
  421Eh - JOY4L   - Joypad 4 (gameport 2, pin 5) (lower 8bit)               00h
  421Fh - JOY4H   - Joypad 4 (gameport 2, pin 5) (upper 8bit)               00h
  4220h..42FFh    - Unused region (open bus)                                -
```

CPU DMA, For below ports, x = Channel number 0..7 (R/W)

```text
  (additional DMA control registers are 420Bh and 420Ch, see above)
  43x0h - DMAPx   - DMA/HDMA Parameters                                   (FFh)
  43x1h - BBADx   - DMA/HDMA I/O-Bus Address (PPU-Bus aka B-Bus)          (FFh)
  43x2h - A1TxL   - HDMA Table Start Address (low)  / DMA Curr Addr (low) (FFh)
  43x3h - A1TxH   - HDMA Table Start Address (high) / DMA Curr Addr (high)(FFh)
  43x4h - A1Bx    - HDMA Table Start Address (bank) / DMA Curr Addr (bank)(xxh)
  43x5h - DASxL   - Indirect HDMA Address (low)  / DMA Byte-Counter (low) (FFh)
  43x6h - DASxH   - Indirect HDMA Address (high) / DMA Byte-Counter (high)(FFh)
  43x7h - DASBx   - Indirect HDMA Address (bank)                          (FFh)
  43x8h - A2AxL   - HDMA Table Current Address (low)                      (FFh)
  43x9h - A2AxH   - HDMA Table Current Address (high)                     (FFh)
  43xAh - NTRLx   - HDMA Line-Counter (from current Table entry)          (FFh)
  43xBh - UNUSEDx - Unused byte (read/write-able)                         (FFh)
  43xCh+  -         Unused region (open bus)                                -
  43xFh - MIRRx   - Mirror of 43xBh (R/W)                                 (FFh)
  4380h..5FFFh    - Unused region (open bus)                                -
```

#### Further Memory

```text
  6000h..7FFFh    - Expansion (eg. Battery Backed RAM, in HiROM cartridges)
  8000h..FFFFh    - Cartridge ROM
```

Note: The right column shows the initial value on Reset, values in brackets are left unchanged upon Reset (but do contain the specified value upon initial Power-up).

Audio Registers (controlled by the SPC700 CPU, not by the Main CPU)

> **See:** [SNES APU Memory and I/O Map](snes-apu-memory-and-i-o-map.md)

#### Expansion Overview

```text
  0000h-003Fh  Cheat Device: Pro Action Replay 1-3: I/O Ports; overlapping WRAM
  2000h-2007h  MSU1: Media Streaming Unit (homebrew)
  2184h-218Fh  Copier: Super UFO models Pro-7 and Pro-8
  2188h-2199h  Satellaview Receiver Unit (connected to Expansion Port)
  21C0h-21C3h  More or less "used" by Nintendo's "SNES Test" cartridge
  21C0h-21DFh  Exertainment (exercise bicycle) (connected to Expansion Port)
  21D0h-21E5h  Sony Super Disc prototype: CDROM Controller & BIOS Cart RAM lock
  21FCh-21FFh  Nocash Debug Extension (char_out and 21mhz_timer in no$sns emu)
  2200h-230Eh  SA-1 (programmable 65C816 CPU) I/O Ports
  2400h-2407h  Nintendo Power (flashcard) (bank 00h only, no mirrors)
  2800h-2801h  S-RTC Real Time Clock I/O Ports
  2800h-2810h  Copier: FDC I/O Ports CCL (Supercom Partner & Pro Fighter)
  2C00h-2FFFh  Cheat Device: X-Terminator 2: SRAM (32Kbytes, in banks 00h-1Fh)
  3000h-32FFh  GSU-n (programmable RISC CPU) I/O Ports
  3000h-37FFh  SA-1 (programmable 65C816 CPU) on-chip I-RAM
  3800h-3804h  ST018 (Seta) (pre-programmed ARM CPU) (maybe also FF40h..FF63h)
  4100h        Nintendo Super System (NSS) DIP-Switches (on game cartridge)
  4800h-4807h  S-DD1 Data Decompression chip
  4800h-4842h  SPC7110 Data Decompression chip (optionally with RTC-4513)
  5000h-5FFFh  Satellaview MCC mem ctrl (sixteen 1bit I/O ports banks 00h-0Fh)
  5000h-5FFFh  Satellaview 32Kbyte SRAM (eight 4Kbyte-chunks in banks 10h-17h)
  58xxh        Copier: Venus (Multi Game Hunter)
  5Fxxh        Copier: Gamars Super Disk
  6000h-7FFFh  SRAM Battery Backed Static RAM (in HiROM cartridges) (bank 3xh)
  6000h-7FFFh  SGB Super Gameboy I/O Ports
  6000h-7FFFh  DSP-n on HiROM boards (pre-programmed NEC uPD77C25 CPU)
  7F40h-7FAFh  CX4 I/O Ports (with 3K SRAM at 6000h..6BFFh)
  7FF0h-7FF7h  OBC1 OBJ Controller I/O Ports (with 8K SRAM at 6000h..7FFFh)
  8000h-FFFFh  Cartridge ROM (including header/exception vectors at FFxxh)
  8000h        Pirate X-in-1 Multicart 32K-ROM bank (mapping 20 small games)
  8000h-FFFFh  Copier: Various models map ROM, I/O, SRAM, DRAM in ROM area
  8000h-8101h  Cheat Device: Game Genie I/O Ports (in ROM banks 00h and FFh)
  A000h-A007h  Cheat Device: Pro Action Replay 2: I/O Control (in HiROM area)
  FFE0h-FFFFh  Exception Vectors (variable in SA-1 and CX4) (fixed in GSU)
  FFE8h-FFEAh  Cheat Device: X-Terminator 1-2: I/O Ports (in LoROM area)
  FFF0h-FFF3h  Tri-Star/Super 8: NES Joypad 1-2, BIOS-disable, A/V-select
  xF8000h-3FFFFFh  DSP-n on LoROM boards (pre-programmed NEC uPD77C25 CPU)
  600000h-6F7FFFh  DSP-n on 2Mbyte-LoROM boards (planned/prototype only)
  600000h-67FFFFh  ST010/ST011 Command/Status/Parameters I/O Ports
  680000h-6FFFFFh  ST010/ST011 On-chip Battery-backed RAM
  700000h-7xxxxxh  SRAM Battery Backed Static RAM (in LoROM cartridges)
  808000h-BFFFFFh  Bootleg Copy-Protection I/O Ports
  C00000h-FFFFFFh  Satellaview FLASH Cartridges (Detect/Write/Erase commands)
  C00000h-Cn7FFFh  JRA PAT Backup FLASH Memory  (Detect/Write/Erase commands)
  C08000h-FFFFFFh  Satellaview-like FLASH Data Packs in LoROM Cartridges
  E00000h-FFFFFFh  Satellaview-like FLASH Data Packs in HiROM Cartridges
  E00000h-E0FFFFh  X-Band Modem 64K SRAM (in two 32Kx8 chips)
  FBC000h-FBC1BFh  X-Band Modem I/O Ports (mainly in this area)
  FFFF00h-FFFFFFh  Pirate X-in-1 multicart mapper I/O port
```
