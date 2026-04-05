# SNES Cart Copiers - Front Fareast (Super Magicom & Super Wild Card)

#### Front/CCL/Clones

The Front Fareast I/O addresses are used by Front's own models, by early CCL models, and by some third-party clones:

```text
  Super Magicom (Front/CCL)
  Super Wild Card (Front)
  Supercom Pro (CCL) (later CCL models use other I/O ports)
  Super Drive Pro-3 UFO (noname) (later UFO models use other I/O ports)
```

#### I/O Ports (in banks 00h..7Dh and 80h..FFh)

```text
  C000.R    FDC Flags (Bit7: MCS3201 IRQ Signal, Bit6: Drive 'Index' Signal)
              Note: Index signal is (mis-)used for Disk Insert Check
  C002.W    FDC MCS3201 Drive Control Register (motor on, etc.)
  C004.R    FDC MCS3201 Main Status Register
  C005.RW   FDC MCS3201 Command/Data Register
  C007.R    FDC MCS3201 Diagnostics Register (bit7=disk change; MCS-chip only)
  C007.W    FDC MCS3201 Density Select Register (bit0-1=Transfer rate)
  C008.R    Parallel Data Input (Reading this register reverses busy flag)
  C008.W    Parallel Data Output (bit0-3) and DRAM/SRAM mapping (bit0-1)
              Bit 0: 0=LoROM/Mode 20, 1=HiROM/Mode 21 (DRAM Mapping)
              Bit 1: 0=LoROM/Mode  1, 1=HiROM/Mode  2 (SRAM Mapping)
  C009.R    Parallel Port Busy Flag, Bit 7 (older EP1810 Version) (Altera chip)
  C000.R    Parallel Port Busy Flag, Bit 5 (newer FC9203 Version) (FRONT chip)
  C00A-C00F Unused (mirrors of C008h-C009h)
  C010-DFFF Unused (mirrors of C000h-C00Fh)
```

#### Below E000h-E00Dh are triggered by writing any value

```text
  E000.W    Memory Page 0  ;\Select an 8Kbyte page, CART/DRAM/SRAM address is:
  E001.W    Memory Page 1  ;    SNES address AND 1FFFh      ;lower bits
  E002.W    Memory Page 2  ;    +Selected Page * 2000h      ;upper bits
  E003.W    Memory Page 3  ;/   +SNES address AND FF0000h   ;bank number
  E004.W    Set System Mode 0 (BIOS Mode)             (with all I/O enabled)
  E005.W    Set System Mode 1 (Play Cartridge)        (with all I/O disabled)
  E006.W    Set System Mode 2 (Cartridge Emulation 1) (with E004-E007 kept on)
  E007.W    Set System Mode 3 (Cartridge Emulation 2) (with all I/O disabled)
  E008.W    Select 44256 DRAM Type  (for 2,4,6,8 Mega DRAM Card)
  E009.W    Select 441000 DRAM Type (for 8,16,24,32 Mega DRAM Card)
  E00C.W    BIOS Mode:CART at A000-BFFF, DRAM Mode:DRAM in bank 20-5F/A0-DF
  E00D.W    BIOS Mode:SRAM at A000-BFFF, DRAM Mode:CART in bank 20-5F/A0-DF
```

Later Wild Card DX models have various extra ports, eg. E0FDh, F083h, C108h.

Ports C00xh seem to be used by models up to DX and DX96.

In DX2, Ports C00xh seem to be moved to CF8xh/DF8xh.

#### System Mode 0 (BIOS Mode) (selected via E004h)

```text
  bb2000-bb3FFF RW: SRAM or CART (E00C/E00D)  bb-40-7D,C0-FF ;\8K page via
  bb8000-bb9FFF RW: DRAM                      bb-00-7D,80-FF ; E000-E003
  bbA000-bbBFFF RW: SRAM or CART (E00C/E00D)  bb=00-7D,80-FF ;/
  bbC000-bbC00x RW: I/O Ports                 bb=00-7D,80-FF
  bbE000-bbE00x W : I/O Ports                 bb=00-7D,80-FF
  bbE000-bbFFFF R : BIOS ROM (8/16/256Kbytes) bb=00-1F
```

#### System Mode 1 (CART Mode) (selected via E005h)

```text
  bb0000-bbFFFF RW: CART
```

#### System Mode 2/3 (DRAM Modes) (selected via E006h/E007h)

```text
  bb0000-bb7FFF R : DRAM Mapping, bb=40-6F, C0-DF. (HiROM/Mode 21)
  bb8000-bbFFFF R : DRAM Mapping, bb=00-6F, 80-DF. (AnyROM/Mode 20,21)
  708000-70FFFF RW: SRAM Mode 1 Mapping.         ;<-- typically for LoROM
  306000-307FFF RW: SRAM Mode 2 Mapping, Page 0. ;<-- typically for HiROM
  316000-317FFF RW: SRAM Mode 2 Mapping, Page 1. ;\extra banks for HiROM
  326000-327FFF RW: SRAM Mode 2 Mapping, Page 2. ; (do any 'real' cartridges
  336000-337FFF RW: SRAM Mode 2 Mapping, Page 3. ;/do actually have that?)
```

DRAM mapping (LoROM/HiROM), and corresponding SRAM mapping are selected via (sharing) Bit0-1 of the Parallel Data Output (Port C008h.W) HiROM/Mode 21:

```text
  Even DRAM Bank is mapped to bb0000-bb7FFF.
  Odd DRAM Bank is mapped to  bb8000-bbFFFF.
```

Optionally, banks 20-5F and A0-DF can be mapped to CART instead of DRAM (via E00Dh), probably intended to allow ROM-images in DRAM to access DSP chips in CART.

#### BIOS Notes

Observe that the BIOS is divided into 8Kbyte banks (so, the exception vectors are at offset 1Fxxh in the ROM-image) (however, there are some overdumped ROM-images that contain 24K padding prior to each 8K ROM-bank, ie. with LoROM mapping style exception vectors at offset 7Fxxh). Aside from the exception vectors, there isn't any title, nor other valid cartridge header entries.

Note: Unlike most SNES programs, Magicom & Wild Card (until v1.8) BIOSes are running in 6502 emulation mode (with E=1), rather than 65C816 mode (with E=0).

#### Parallel Port Protocol (on SNES side)

Data is received via 8bit data register, and sent via 4bit "status" register (which is seen as status on PC side). Strobe/busy aren't clearly documented in official Front specs; probably, Busy gets set automatically when sensing Strobe (from PC side), and gets cleared automatically when reading Data from Port C008h (on SNES side).

#### Parallel Port Protocol (on PC side)

Byte Output Procedure:

```text
  Wait Busy Bit = 1           ;Status  PC Port 379h/279h/3BDh.Bit7
  Write One Byte              ;Data    PC Port 378h/278h/3BCh.Bit0-7
  Reverse Strobe Bit          ;Control PC Port 37Ah/27Ah/3BEh.Bit0
```

Byte Input Procedure:

```text
  Wait Busy Bit = 0           ;Status  PC Port 379h/279h/3BDh.Bit7
  Read Low 4 Bits of Byte     ;Status  PC Port 379h/279h/3BDh.Bit3-6
  Reverse Strobe Bit          ;Control PC Port 37Ah/27Ah/3BEh.Bit0
  Wait Busy Bit = 0           ;Status  PC Port 379h/279h/3BDh.Bit7
  Read High 4 Bits of Byte    ;Status  PC Port 379h/279h/3BDh.Bit3-6
  Reverse Strobe Bit          ;Control PC Port 37Ah/27Ah/3BEh.Bit0
```

Receiving 4bit units via status line is done for compatibility with old one-directional PC parallel (printer) ports.

Unknown if "Wait Busy Bit = X" means to wait while-or-until Bit=X?

#### Parallel Port Command Format

Commands are 9-bytes in length, sent from PC side.

```text
  00h 3   ID (D5h,AAh,96h)
  03h 1   Command Code (00h-01h, or 04h-06h)
  04h 2   Address (LSB,MSB)
  06h 2   Length (LSB,MSB)
  08h 1   Checksum (81h XORed by Bytes 03h..07h)
  Followed by <Length> bytes of data (upload/download commands only)
```

Commands can be:

```text
  Command 00h : Download Data (using page:address,length)   ;to-or-from PC?
  Command 01h : Upload Data (using page:address,length)     ;from-or-to PC?
  Command 04h : Force SFC Program to JMP (to address... plus page/bank?)
  Command 05h : Select 8Kbyte Memory Page Number (using address)
  Command 06h : Sub Function (address: 0=InitialDevice: 1=ExecDRAM, 2=ExecCART)
```

InitialDevice does probably reset the BIOS? ExecDRAM allows to run the uploaded ROM-image, but unknow how to select LoROM/HiROM mode, or is it automatically done by examining the uploaded cartridge header. The usage of the 16bit address isn't quite clear: The lower 13bit are somehow combined with the 8Kbyte page number, the upper 3bit might be used to select DRAM/SRAM?

#### Super Magicom V3H - BIOS upgrade

This ROM-image is a Magicom BIOS upgrade, and it's a pain in the ass:

The upgrade works ONLY as ROM-image (ie. must be loaded to DRAM), and does NOT work as real ROM (ie. cannot be burned to EPROM), the reason is that it doesn't include a character set (and uses that from the original Magicom BIOS at E000h).

The upgrade isn't compatible with the parallel port (the original BIOS relocates parallel port code from ROM to WRAM, and the upgrade relocates itself from DRAM to WRAM - but still expects the parallel port code in the same WRAM location and crashes when Busy-bit gets set).

The upgrade exists as 32Kbyte or 32.5Kbyte ROM-image (an 8K upgrade, 24K garbage with entrypoint at end of garbage, plus 0.5K extra garbage), emulators and other tools will be typically interprete the 32.5K file as 32Kbyte ROM with 512-byte header (which is NOT correct in that special case).

The upgrade exists in two variants: One using the standard Front Fareast I/O addresses, one using the I/O addresses at C000h-C00Fh re-ordered as so:

```text
  8000h-FFFFh RW DRAM-mode: DRAM (containing the Magicom V3H upgrade)
  E000h-FFFFh R  BIOS-mode: BIOS (containing the Character set)
  C000h       W  DRAM bank mapped to 8000h? (set to 00,20,40 upon DRAM detect)
  C001h       W  Memory Control?
  C002h       -  Unused
  C003h       R  Parallel Port Busy (bit7) (when set: crashes the V3H upgrade)
  C004h-C008h -  Unused
  C009h       R  Status (bit7=ready?,bit5=busy/timeout?)
  C00Ah       -  FDC Unused
  C00Bh       W  FDC Motor Control (set to 00h,29h,2Dh)
  C00Ch       RW FDC Command/Data
  C00Dh       R  FDC Main Status
  C00Eh       W  FDC Transfer Rate? (set to 00h,01h,02h or so)
  C00Fh       -  FDC Unused
  E004h       W  Map BIOS ROM (instead V3H upgrade) ;\
  E006h-E007h W  Something on/off                   ; seems to be same/similar
  E008h-E009h W  Something on/off                   ; as Front-like I/O ports
  E00Ch-E00Dh W  Something on/off                   ;/
```

Unknown which hardware uses that re-ordered addresses. Note: The V3H version with re-ordered I/O addresses does also contain different 24K garbage (sorts of as if it were created from original source code, rather than just patched?).

#### Component List - Super Magicom Plus

```text
  U1  24pin  DRAM (onboard)
  U2  20pin  SN74LS245N (8-bit 3-state transceiver)
  U3  24pin  DRAM (onboard)
  U4  24pin  DRAM (onboard)
  U5  24pin  DRAM (onboard)
  U6  28pin  27C128-25 EPROM (16Kx8)
  U7  68pin  MCCS3201FN (=MCS3201FN without double-C) (disc controller)
  U8  100pin ?
  U9  28pin  HM62256 (SRAM 32Kx8)
  U10 20pin  ?
  U11 14pin  ?  (does not exist in later versions?)
  U12 20pin  AMI 16CVB8PC-25
  U13 20pin  AMI 16CVB8PC-25
  U14 16pin  ST10198S (newer version only) (mounted on top of U10 in old ver)
  BT1 2pin   ?
  J1  25pin  DB-25 parallel port
  J2? 25pin  DB-25 external floppy (not installed)
  J3  40pin  DRAM expansion board
  J4  34pin  internal floppy (flat cable)
  J5  26pin  (not installed)
  J6  4pin   floppy power supply
  J7  62pin  cartridge edge
  J8  62pin  cartridge slot
  J9  12pin  jumpers (I:I:I: or :I:I:I) (enable internal or external CIC)
  Y1  2pin   24.000 MHz
```

Component List - Super Wild Card DX (AH-558001-02 Made in Japan 94.8.23)

```text
  U1  100pin CPU      FRONT FC9203 HG62E22926F9 (or so) (SMD)
  U2   28pin S-RAM    NEC D43256AC-10L (uPD43256AC) (SRAM, 32Kx8)
  U3   20pin          SN74LS245 (to parallel port) (8-bit 3-state transceiver)
  U4    ?pin PAL-2    L GAL20V84 25LP
  U5   20pin          SN74LS...? (or so)
  U6   32pin BIOS-ROM BIOS
  U7   16pin U7       SN74LS139AN (decoder/demultiplexer)
  U8   14pin U8       SN74LS125AN (quad 3-state buffer)
  U9   44pin          GoldStar GM82C765B (SMD) (floppy disc controller)
  U10  16pin DECODER  SNC4011 (or so)
  U11  20pin PAL-3    iCT PEEL17CV8P CTN24053
  U12?  3pin 7805H    voltage regulator
  U13  20pin PAL-1    AMI
  X1    2pin 16MHZ    16.000 MHz
  CN?   2pin AC/DC-IN power supply input
  CN?   4pin ..POW    power supply to internal disc drive (only 2pin connected)
  CN2  62pin          female cartridge slot
  CN3? 46pin RAM-SLOT to DRAM daughterboard (only 40pin used on remote side)
  CN5  25pin PC-I/F   DB-25 parallel port
  CN6  34pin FDD-I/F  cable to internal disc drive
  CN01 34pin          goes to one 1st of male 62pin cartridge edge
  CN02 34pin          goes to one 2nd of male 62pin cartridge edge
  SW1   3pin RESET-SW reset switch/button or so, for whatever purpose
  DB1   4pin          AC-DC converter
  BT1   2pin          3V battery
  J1   12pin          jumpers (near cartridge slot)
  J2   20pin          jumpers (near cartridge slot)
  J3    2pin          jumper (near power-input)
  J4    2pin          jumper (near power-input)
  J5    2pin          jumper (near power-input)
  DRAM Daughterboard:
  U1,U2,U7,U8   16pin  ST T74LS139B (decoder/demultiplexer) (four pieces)
  U3-U6,U9-U10  28pin  NEC D424900G5 (or so) (six pieces) (SMD)
  U11-U12       28pin  M5M44800ATP           (two pieces) (SMD)
```

Component List - Supercom Pro (SP3200) (dated around 1992) (probably uses Front-like I/O)

```text
  U1  20pin  SN74LS245N (to parallel port) (8-bit 3-state transceiver)
  U2  16pin  HD74LS174 (to parallel port?)
  U4  68pin  MCCS3201FN (=MCS3201FN without double-C) (floppy disc controller)
  U?  68pin  Altera EP1810LC-45 D9219
  U?  28pin  EPROM
  U?  28pin  SRAM Winbond W24256-10L 9149
  U7  20pin  SN74LS245N (8-bit 3-state transceiver)
  U8  16pin  not installed
  U?  20pin  modded (?) chip (soldered on cart-edge connector at bottom side)
  J1  25pin  DB-25 parallel port
  J2  25pin  DB-25 external floppy disc connector
  J3  40pin  to DRAM daughterboard
  J4  34pin  not installed (internal floppy disc connector)
  J5  62pin  cartridge edge
  J6  62pin  cartridge slot
  Y1  2pin   24.000 MHz
  BT1 2pin   VARTA Ni/Cd, 3.6V 60mA, 14h 6mA (recharge-able & acid-leaking)
```

#### Component List - SMD800 Super Magic Drive (requires SNES-to-Genesis adaptor)

```text
  U1  20pin  SN74LS245N (to parallel port?) (8-bit 3-state transceiver)
  U2  16pin  SN74LS174 (to parallel port?)
  U3  20pin  SN74HC245P (8-bit 3-state transceiver)
  U4  20pin  SN74HC245P (8-bit 3-state transceiver)
  U5? 68pin  MCS3201FN (floppy disk controller)
  U6  20pin  SN74HC245P (8-bit 3-state transceiver)
  U   28pin  27C64A-15 (EPROM, 8Kx8) (with Genesis Z80 code, non-SNES code)
  U   28pin  HY62256ALP-10 (SRAM, 32Kx8)
  U     pin  Altera EP1810LC-45
  U10 16pin  MC74HC157 (decoder/demultiplexer)
  U11 16pin  MC74HC157 (decoder/demultiplexer)
  U12 14pin  xxxx
  J   25pin  DB-25 parallel port
  J2  25pin  DB-25 external floppy
  J3  40pin  internal floppy (not installed) (likely only 34pins of 40pin used)
  J   64pin  cartridge edge (genesis)
  J   64pin  cartridge slot (genesis)
  J   40pin  to DRAM daughterboard
  Y   2pin   oscillator
  BT  2pin   VARTA Ni/Cd, 3.6V 60mA, 14h 6mA (recharge-able & acid-leaking)
```

DRAM Daughterboard:

```text
  U1  20pin  HY514400J-70 (DRAM)
  U2  20pin  HY514400J-70 (DRAM)
  U3  20pin  HY514400J-70 (DRAM)
  U4  20pin  HY514400J-70 (DRAM)
  U5  14pin  74LS08 (quad 2-input AND gates)
  U6  16pin  HD74LS157P (decoder/demultiplexer)
  U7  14pin  74LS08 (quad 2-input AND gates)
  CN1 40pin  connector to mainboard
```

Super Magicom-Drive (SNES-to-Genesis adaptor for above):

```text
  xxx        components unknown
```
