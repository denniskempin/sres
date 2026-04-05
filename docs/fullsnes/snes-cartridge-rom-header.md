# SNES Cartridge ROM Header

The Cartridge header is mapped to 00FFxxh in SNES memory (near the exception vectors). In ROM-images it is found at offset 007Fxxh (LoROM), 00FFxxh (HiROM), or 40FFxxh (ExHiROM); add +200h to that offsets if "(imagesize AND 3FFh)=200h", ie. if there's an extra header from SWC/UFO/etc. copiers.

#### Cartridge Header (Area FFC0h..FFCFh)

```text
  FFC0h  Cartridge title (21 bytes, uppercase ascii, padded with spaces)
  FFC0h  First byte of title (or 5Ch far-jump-opcode in Pirate X-in-1 Carts)
  FFD4h  Last byte of title (or 00h indicating Early Extended Header)
  FFD5h  Rom Makeup / ROM Speed and Map Mode (see below)
  FFD6h  Chipset (ROM/RAM information on cart) (see below)
  FFD7h  ROM size (1 SHL n) Kbytes (usually 8=256KByte .. 0Ch=4MByte)
          Values are rounded-up for carts with 10,12,20,24 Mbits
  FFD8h  RAM size (1 SHL n) Kbytes (usually 1=2Kbyte .. 5=32Kbyte) (0=None)
  FFD9h  Country (also implies PAL/NTSC) (see below)
  FFDAh  Developer ID code  (00h=None/Homebrew, 01h=Nintendo, etc.) (33h=New)
  FFDBh  ROM Version number (00h=First)
  FFDCh  Checksum complement (same as below, XORed with FFFFh)
  FFDEh  Checksum (all bytes in ROM added together; assume [FFDC-F]=FF,FF,0,0)
```

Extended Header (Area FFB0h..FFBFh) (newer carts only) Early Extended Header (1993) (when [FFD4h]=00h; Last byte of Title=00h):

```text
  FFB0h  Reserved   (15 zero bytes)
```

Later Extended Header (1994) (when [FFDAh]=33h; Old Maker Code=33h):

```text
  FFB0h  Maker Code (2-letter ASCII, eg. "01"=Nintendo)
  FFB2h  Game Code  (4-letter ASCII) (or old 2-letter padded with 20h,20h)
  FFB6h  Reserved   (6 zero bytes)
  FFBCh  Expansion FLASH Size (1 SHL n) Kbytes (used in JRA PAT)
  FFBDh  Expansion RAM Size (1 SHL n) Kbytes (in GSUn games) (without battery?)
  FFBEh  Special Version      (usually zero) (eg. promotional version)
```

Both Early and Later Extended Headers:

```text
  FFBFh  Chipset Sub-type (usually zero) (used when [FFD6h]=Fxh)
```

Note: The early-extension is used only with ST010/11 games.

If the first letter of the 4-letter Game Code is "Z", then the cartridge does have Satellaview-like Data Pack/FLASH cartridge slot (this applies ONLY to "Zxxx" 4-letter codes, not to old "Zx  " space padded 2-letter codes).

#### Cartridge Header Variants

The BS-X Satellaview FLASH Card Files, and Sufami Turbo Mini-Cartridges are using similar looking (but not fully identical) headers (and usually the same .SMC file extension) than normal ROM cartridges. Detecting the content of .SMC files can be done by examining ID Strings (Sufami Turbo), or differently calculated checksum values (Satellaview). For details, see:

> **See:** [SNES Cart Satellaview (satellite receiver & mini flashcard)](snes-cart-satellaview-satellite-receiver-mini-flashcard.md)
> **See:** [SNES Cart Sufami Turbo (Mini Cartridge Adaptor)](snes-cart-sufami-turbo-mini-cartridge-adaptor.md)

Homebrew games (and copiers & cheat devices) are usually having several errors in the cartridge header (usually no checksum, zero-padded title, etc), they should (hopefully) contain valid entryoints in range 8000h..FFFEh. Many Copiers are using 8Kbyte ROM bank(s) - in that special case the exception vectors are located at offset 1Fxxh within the ROM-image.

#### CPU Exception Vectors (Area FFE0h..FFFFh)

```text
  FFE0h  Zerofilled (or ID "XBOO" for WRAM-Boot compatible files)
  FFE4h  COP vector     (65C816 mode) (COP opcode)
  FFE6h  BRK vector     (65C816 mode) (BRK opcode)
  FFE8h  ABORT vector   (65C816 mode) (not used in SNES)
  FFEAh  NMI vector     (65C816 mode) (SNES V-Blank Interrupt)
  FFECh  ...
  FFEEh  IRQ vector     (65C816 mode) (SNES H/V-Timer or External Interrupt)
  FFF0h  ...
  FFF4h  COP vector     (6502 mode)
  FFF6h  ...
  FFF8h  ABORT vector   (6502 mode) (not used in SNES)
  FFFAh  NMI vector     (6502 mode)
  FFFCh  RESET vector   (6502 mode) (CPU is always in 6502 mode on RESET)
  FFFEh  IRQ/BRK vector (6502 mode)
```

Note: Exception Vectors are variable in SA-1 and CX4, and fixed in GSU.

#### Text Fields

The ASCII fields can use chr(20h..7Eh), actually they are JIS (with Yen instead backslash).

#### ROM Size / Checksum Notes

The ROM size is specified as "(1 SHL n) Kbytes", however, some cartridges contain "odd" sizes:

```text
  * Game uses 2-3 ROM chips (eg. one 8MBit plus one 2MBit chip)
  * Game originally designed for 2 ROMs, but later manufactured as 1 ROM (?)
  * Game uses a single 24MBit chip (23C2401)
```

In all three cases the ROM Size entry in [FFD7h] is rounded-up. In memory, the "bigger half" is mapped to address 0, followed by the "smaller half", then followed by mirror(s) of the smaller half. Eg. a 10MBit game would be rounded to 16MBit, and mapped (and checksummed) as "8Mbit + 4x2Mbit". In practice:

```text
  Title                       Hardware          Size   Checksum
  Dai Kaiju Monogatari 2 (J)  ExHiROM+S-RTC     5MB    4MB + 4 x Last 1MB
  Tales of Phantasia (J)      ExHiROM           6MB    <???>
  Star Ocean (J)              LoROM+S-DD1       6MB    4MB + 2 x Last 2MB
  Far East of Eden Zero (J)   HiROM+SPC7110+RTC 5MB    5MB
  Momotaro Dentetsu Happy (J) HiROM+SPC7110     3MB    2 x 3MB
  Sufami Turbo BIOS           LoROM in Minicart xx     without checksum
  Sufami Turbo Games          LoROM in Minicart xx     without checksum
  Dragon Ball Z - Hyper Dimension    LoROM+SA-1 3MB       Overdump 4MB
  SD Gundam GNext (J)                LoROM+SA-1 1.5MB     Overdump 2MB
  Megaman X2                         LoROM+CX4  1.5MB     Overdump 2MB
  BS Super Mahjong Taikai (J)        BS           Overdump/Mirr+Empty
  Demon's Crest... reportedly 12MBit ? but, that's bullshit ?

  SPC7110 Title               ROM Size (Header value)    Checksum
  Super Power League 4        2MB      (rounded to 2MB)  1x(All 2MB)
  Momotaro Dentetsu Happy (J) 3MB      (rounded to 4MB)  2x(All 3MB)
  Far East of Eden Zero (J)   5MB      (rounded to 8MB)  1x(All 5MB)
```

On-chip ROM contained in external CPUs (DSPn,ST01n,CX4) is NOT counted in the ROM size entry, and not included in the checksum.

Homebrew files often contain 0000h,0000h or FFFFh,0000h as checksum value.

#### ROM Speed and Map Mode (FFD5h)

```text
  Bit7-6 Always 0
  Bit5   Always 1 (maybe meant to be MSB of bit4, for "2" and "3" MHz)
  Bit4   Speed (0=Slow, 1=Fast)              (Slow 200ns, Fast 120ns)
  Bit3-0 Map Mode
```

Map Mode can be:

```text
  0=LoROM/32K Banks             Mode 20 (LoROM)
  1=HiROM/64K Banks             Mode 21 (HiROM)
  2=LoROM/32K Banks + S-DD1     Mode 22 (mappable) "Super MMC"
  3=LoROM/32K Banks + SA-1      Mode 23 (mappable) "Emulates Super MMC"
  5=HiROM/64K Banks             Mode 25 (ExHiROM)
  A=HiROM/64K Banks + SPC7110   Mode 25? (mappable)
```

Note: ExHiROM is used only by "Dai Kaiju Monogatari 2 (JP)" and "Tales of Phantasia (JP)".

Chipset (ROM/RAM information on cart) (FFD6h) (and some subclassed via FFBFh)

```text
  00h     ROM
  01h     ROM+RAM
  02h     ROM+RAM+Battery
  x3h     ROM+Co-processor
  x4h     ROM+Co-processor+RAM
  x5h     ROM+Co-processor+RAM+Battery
  x6h     ROM+Co-processor+Battery
  x9h     ROM+Co-processor+RAM+Battery+RTC-4513
  xAh     ROM+Co-processor+RAM+Battery+overclocked GSU1 ? (Stunt Race)
  x2h     Same as x5h, used in "F1 Grand Prix Sample (J)" (?)
  0xh     Co-processor is DSP    (DSP1,DSP1A,DSP1B,DSP2,DSP3,DSP4)
  1xh     Co-processor is GSU    (MarioChip1,GSU1,GSU2,GSU2-SP1)
  2xh     Co-processor is OBC1
  3xh     Co-processor is SA-1
  4xh     Co-processor is S-DD1
  5xh     Co-processor is S-RTC
  Exh     Co-processor is Other  (Super Gameboy/Satellaview)
  Fxh.xxh Co-processor is Custom (subclassed via [FFBFh]=xxh)
  Fxh.00h Co-processor is Custom (SPC7110)
  Fxh.01h Co-processor is Custom (ST010/ST011)
  Fxh.02h Co-processor is Custom (ST018)
  Fxh.10h Co-processor is Custom (CX4)
```

In practice, following values are used:

```text
  00h     ROM             ;if gamecode="042J" --> ROM+SGB2
  01h     ROM+RAM (if any such produced?)
  02h     ROM+RAM+Battery ;if gamecode="XBND" --> ROM+RAM+Batt+XBandModem
                          ;if gamecode="MENU" --> ROM+RAM+Batt+Nintendo Power
  03h     ROM+DSP
  04h     ROM+DSP+RAM (no such produced)
  05h     ROM+DSP+RAM+Battery
  13h     ROM+MarioChip1/ExpansionRAM (and "hacked version of OBC1")
  14h     ROM+GSU+RAM                    ;\ROM size up to 1MByte -> GSU1
  15h     ROM+GSU+RAM+Battery            ;/ROM size above 1MByte -> GSU2
  1Ah     ROM+GSU1+RAM+Battery+Fast Mode? (Stunt Race)
  25h     ROM+OBC1+RAM+Battery
  32h     ROM+SA1+RAM+Battery (?) "F1 Grand Prix Sample (J)"
  34h     ROM+SA1+RAM (?) "Dragon Ball Z - Hyper Dimension"
  35h     ROM+SA1+RAM+Battery
  43h     ROM+S-DD1
  45h     ROM+S-DD1+RAM+Battery
  55h     ROM+S-RTC+RAM+Battery
  E3h     ROM+Super Gameboy      (SGB)
  E5h     ROM+Satellaview BIOS   (BS-X)
  F5h.00h ROM+Custom+RAM+Battery     (SPC7110)
  F9h.00h ROM+Custom+RAM+Battery+RTC (SPC7110+RTC)
  F6h.01h ROM+Custom+Battery         (ST010/ST011)
  F5h.02h ROM+Custom+RAM+Battery     (ST018)
  F3h.10h ROM+Custom                 (CX4)
```

#### Country (also implies PAL/NTSC) (FFD9h)

```text
  00h -  International (eg. SGB)  (any)
  00h J  Japan                    (NTSC)
  01h E  USA and Canada           (NTSC)
  02h P  Europe, Oceania, Asia    (PAL)
  03h W  Sweden/Scandinavia       (PAL)
  04h -  Finland                  (PAL)
  05h -  Denmark                  (PAL)
  06h F  France                   (SECAM, PAL-like 50Hz)
  07h H  Holland                  (PAL)
  08h S  Spain                    (PAL)
  09h D  Germany, Austria, Switz  (PAL)
  0Ah I  Italy                    (PAL)
  0Bh C  China, Hong Kong         (PAL)
  0Ch -  Indonesia                (PAL)
  0Dh K  South Korea              (NTSC) (North Korea would be PAL)
  0Eh A  Common (?)               (?)
  0Fh N  Canada                   (NTSC)
  10h B  Brazil                   (PAL-M, NTSC-like 60Hz)
  11h U  Australia                (PAL)
  12h X  Other variation          (?)
  13h Y  Other variation          (?)
  14h Z  Other variation          (?)
```

Above shows the [FFD9h] value, and the last letter of 4-character game codes.

Game Codes (FFB2h, exists only when [FFDAh]=33h)

```text
  "xxxx"  Normal 4-letter code (usually "Axxx") (or "Bxxx" for newer codes)
  "xx  "  Old 2-letter code (space padded)
  "042J"  Super Gameboy 2
  "MENU"  Nintendo Power FLASH Cartridge Menu
  "Txxx"  NTT JRA-PAT and SPAT4 (SFC Modem BIOSes)
  "XBND"  X-Band Modem BIOS
  "Zxxx"  Special Cartridge with satellaview-like Data Pack Slot
```

The last letter indicates the region (see Country/FFD9h description) (except in 2-letter codes and "MENU"/"XBND" codes).
