# SNES Cart Data Pack Slots (satellaview-like mini-cartridge slot)

#### Data Packs

Data Packs are Satellaview 8M Memory Packs which have data meant to be used as expansion for a Data Pack-compatible game. Data Pack-compatible game cartridges have a resemblence to the BS-X Cartridge itself.

#### Usage

For most of these games, Data was distributed via St.GIGA's Satellaview services. Same Game and SD Gundam G-Next had some Data Packs sold as retail in stores. RPG Tsukuru 2, Sound Novel Tsukuru and Ongaku Tsukuru Kanaderu could save user-created data to 8M Memory Packs.

#### Cartridges with Data Pack Slot

```text
  Derby Stallion 96                  (SpecialLoROM, 3MB ROM, 32K RAM)
  Itoi Shigesato no Bass Tsuri No. 1 (SA-1, map-able 4MB ROM, 8K RAM)
  Joushou Mahjong Tenpai             (HiROM, 1MB)
  Ongaku Tukool/Tsukuru Kanaderu     (HiROM, 1MB)
  RPG Tukool/Tsukuru 2               (LoROM, 2MB)
  Same Game Tsume Game               (HiROM, 1MB)
  Satellaview BS-X BIOS              (MCC, 1MB ROM) (FLASH at C00000h)
  SD Gundam G-NEXT                   (SA-1, map-able 1.5MB ROM, 32K RAM)
  Sound Novel Tukool/Tsukuru         (SpecialLoROM, 3MB ROM, 64K RAM)
```

Aside from the BS-X BIOS, two of the above games are also accessing BS-X hardware via I/O Ports 2188h/2194h/etc (Derby Stallion 96, Itoi Shigesato no Bass Tsuri No. 1). For doing that, the cartridges do probably require the EXPAND pin to be wired via 100 ohm to SYSCK.

Cartridge Header of cartridges with Data Pack Slot The presence of Data Pack Slots is indicated by a "Z" as first letter of Game Code:

```text
  [FFB2h]="Z"   ;first letter of game code
  [FFB5h]<>20h  ;game code must be 4-letters (not space padded 2-letters)
  [FFDAh]=33h   ;game code must exist (ie. extended header must be present)
```

#### Data Pack Mapping

```text
  MCC (BSX-BIOS)        FLASH at C00000h (continous) (mappable via MCC chip)
  SA-1                  FLASH at <unknown address> (probably mappable via SA1)
  HiROM                 FLASH at E00000h (probably continous)
  LoROM/SpecialLoROM    FLASH at C00000h (looks like 32K chunks)
```

#### LoROM/SpecialLoROM Mapping Notes

The FLASH memory seems to be divided into 32K chunks (mirrored to Cn0000h and Cn8000h) (of which, Derby Stallion 96 uses Cn8000h, RPG Tukool uses Cn0000h, and Ongaku Tukool uses both Cn0000h and Cn8000h).

The two 3MB SpecialLoROM games also have the ROM mapped in an unconventional fashion:

```text
  1st 1MB of ROM mapped to banks 00-1F
  2nd 1MB of ROM mapped to banks 20-3F and A0-BF
  3rd 1MB of ROM mapped to banks 80-9F
  1MB of Data Pack FLASH mapped to banks C0-DF
  32K..64K SRAM mapped to banks 70-71
```

Despite of memory-mirroring of 2nd MB, the checksum-mirroring goes on 3rd MB?

Note: Above mapping differs from "normal" 3MB LoROM games like Wizardry 6 (which have 3rd 1MB in banks 40h-5Fh).
