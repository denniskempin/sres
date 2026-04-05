# SNES Cart FLASH Backup

Most SNES games are using battery-backed SRAM for storing data, the only exception - which do use FLASH memory - are the JRA PAT BIOS cartridges for the SFC Modem:

> **See:** [SNES Add-On SFC Modem (for JRA PAT)](snes-add-on-sfc-modem-for-jra-pat.md)

There are two JRA PAT versions, the older one (1997) supports only AMD FLASH, the newer one (1999) supports AMD/Atmel/Sharp FLASH chips.

```text
  ID=2001h - AM29F010 AMD (128Kbyte)      ;supported by BOTH bios versions
  ID=D51Fh - AT29C010A Atmel (128Kbyte)   ;supported only by newer bios version
  ID=32B0h - LH28F020SUT Sharp (256Kbyte?);supported only by newer bios version
```

The FLASH Size size defined in entry [FFBCh] of the Cartridge Header (this is set to 07h in JRA PAT, ie. "(1K SHL 7)=128Kbytes").

There don't seem to be any data sheets for the Sharp LH28F020SUT-N80 chip (ID B0h,32h) (so not 100% sure if it's really 256Kbytes), anyways, it does somehow resemble LH28F020SU-N (5V, ID B0h,30h) and LH28F020SU-L (5V/3.3V, ID B0h,31h).

#### JRA PAT Memory Map

```text
  80h-9Fh:8000h-FFFFh  ;1Mbyte LoROM (broken into 32 chunks of 32Kbytes)
  C0h-C3h:0000h-7FFFh  ;128Kbyte FLASH (broken into 4 chunks of 32Kbytes)
```

#### AMD FLASH

#### Get Device ID (Type 1 - AMD)

```text
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=90h           ;enter ID mode
  manufacturer=01h=[C00000h], device_type=20h=[C00001h] ;read ID (AM29F010)
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=F0h           ;terminate command
```

#### Erase Entire Chip (Type 1 - AMD)

```text
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=80h           ;prepare erase
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=10h           ;erase entire chip
  repeat, stat=[C00000h], until stat.bit7=1=okay, or stat.bit5=1=timeout
```

#### Erase 16Kbyte Sector (Type 1 - AMD)

```text
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=80h           ;prepare erase
  [C05555h]=AAh, [C02AAAh]=55h, [Cxx000h]=30h           ;erase 16kbyte sector
  repeat, stat=[Cxx000h], until stat.bit7=1=okay, or stat.bit5=1=timeout
```

#### Write Single Data Byte (Type 1 - AMD)

```text
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=A0h           ;write 1 byte command
  [Cxxxxxh]=dta                                         ;write the data byte
  repeat, stat=[Cxxxxxh], until stat.bit7=dta.bit7=okay, or stat.bit5=1=timeout
```

#### Notes

After AMD timeout errors, one should issue one dummy/status read from [C00000h] to switch the device back into normal data mode (at least, JRA PAT is doing it like so, not too sure if that is really required/correct).

#### ATMEL FLASH

#### Get Device ID (Type 2 - Atmel)

```text
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=90h           ;enter ID mode
  manufacturer=1Fh=[C00000h], device_type=D5h=[C00001h] ;read ID (AT29C010A)
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=F0h           ;terminate command
```

#### Erase Entire Chip (Type 2 - Atmel)

```text
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=80h           ;prepare erase
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=10h           ;erase entire chip
  wait two frames (or check if bit6 toggles on each read from [C00000h])
```

#### Erase 16Kbyte Sector (Type 2 - Atmel)

```text
  No such command (one can write data without erasing)
  (to simulate a 16K-erase: write 128 all FFh-filled 128-byte blocks)
```

Write 1..128 Data Bytes (within 128-byte boundary) (Type 2 - Atmel)

```text
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=A0h           ;write 1..128 byte(s)
  [Cxxxxxh+0..n]=dta[0..n]                              ;write the data byte(s)
  repeat, stat=[Cxxxxxh+n], until stat=dta[n]           ;wait last written byte
```

#### Notes

The JRA PAD functions do include a number of wait-vblank delays between various ATMEL commands, that delays aren't shown in above flowcharts.

#### SHARP FLASH

#### Get Device ID (Type 3 - Sharp)

```text
  [C00000h]=90h                                         ;enter ID mode
  manufacturer=B0h=[C00000h], device_type=32h=[C00001h] ;read ID (LH28F020SUT)
  [C00000h]=FFh                                         ;terminate command
```

#### Set/Reset Protection (Type 3 - Sharp)

```text
  [C00000h]=57h/47h, [C000FFh]=D0h    ;<-- C000FFh (!)  ;set/reset protection
  repeat, stat=[C00000h], until stat.bit7=1             ;wait busy
  if stat.bit4=1 or stat.bit5=1 then [C00000h]=50h      ;error --> clear status
  [C00000h]=FFh                                         ;terminate command
```

#### Erase Entire Chip (Type 3 - Sharp)

```text
  [C00000h]=A7h, [C00000h]=D0h                          ;erase entire chip
  repeat, stat=[C00000h], until stat.bit7=1             ;wait busy
  if stat.bit4=1 or stat.bit5=1 then [C00000h]=50h      ;error --> clear status
  [C00000h]=FFh                                         ;terminate command
```

#### Erase 16Kbyte Sector (Type 3 - Sharp)

```text
  [C00000h]=20h, [Cxx000h]=D0h                          ;erase 16kbyte sector
  repeat, stat=[C00000h], until stat.bit7=1             ;wait busy
  if stat.bit4=1 or stat.bit5=1 then [C00000h]=50h      ;error --> clear status
  [C00000h]=FFh                                         ;terminate command
  if failed, issue "Reset Protection", and retry
```

#### Write Single Data Byte (Type 3 - Sharp)

```text
  [C00000h]=40h                                         ;write 1 byte command
  [Cxxxxxh]=dta                                         ;write the data byte
  repeat, stat=[C00000h], until stat.bit7=1             ;wait busy
  ;below error-check & terminate are needed only after writing LAST byte
  if stat.bit4=1 or stat.bit5=1 then [C00000h]=50h      ;error --> clear status
  [C00000h]=FFh                                         ;terminate command
```

#### PCB VERSIONS

#### Older PCB "SHVC-1A9F-01" (1996) (DIP) (for JRA-PAT and SPAT4)

```text
  U1 32pin ROM
  U2 32pin AMD AM29F010-90PC (FLASH)
  U3 16pin SN74LS139AN
  U4 16pin D411B (CIC)
```

#### Newer PCB "SHVC-1A8F-01" (1999) (SMD) (for JRA-PAT-Wide)

```text
  U1 32pin ROM
  U2 32pin Sharp LH28F020SUT-N80 (FLASH)
  U3 16pin 74AC139
  U4 18pin F411B (CIC)
  U5 14pin 74AC08
```

See Also

Another approach for using FLASH backup is used in carts with Data Pack slots:

> **See:** [SNES Cart Data Pack Slots (satellaview-like mini-cartridge slot)](snes-cart-data-pack-slots-satellaview-like-mini-cartridge-slot.md)
