# SNES Cart Nintendo Power - FLASH Commands

Before sending write/erase commands, one must initialize the MX15001 chip via port 240xh (particulary: release the /WP pin), selecting the HIROM_ALL mapping mode may be also recommended (for getting the whole 4Mbyte FLASH memory mapped as continous memory block at address C00000h-FFFFFFh).

Observe that the cart contains two FLASH chips. In HIROM_ALL mode, one chip is at C00000h-DFFFFFh, the other one at E00000h-FFFFFFh (ie. commands must be either written to C0AAAAh/C05554h or E0AAAAh/E05554h, depending on which chip is meant to be accessed; when programming large files that occupy both chips, it would be fastest to program both chips simultaneously).

#### FLASH Command Summary

The FLASH chips are using more or less using standard FLASH commands, invoked by writing to low-bytes at word-addresses 05555h and 02AAAh (aka writing bytes to byte-addresses 0AAAAh and 05554h).

```text
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=F0h, data=[addr..] ;Read/Reset
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=90h, ID=[00000h]   ;Get Maker ID
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=90h, ID=[00002h]   ;Get Device ID
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=90h, WP=[x0004h]   ;Get Sector Protect
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=70h, SRD=[00000h]  ;Read Status Reg
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=50h                ;Clear Status Reg
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=A0h, [addr..]=data ;Page/Byte Program
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=80h                ;Prepare Erase...
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=10h                ;...do Chip Erase
  [0AAAAh]=AAh, [05554h]=55h, [x0000h]=30h                ;...do Sector Erase
  [0xxxxh]=B0h                                            ;...Erase suspend
  [0xxxxh]=D0h                                            ;...Erase resume
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=60h                ;Prepare Protect...
  [0AAAAh]=AAh, [05554h]=55h, [addr]=20h                  ;...do Sector Protect
  [0AAAAh]=AAh, [05554h]=55h, [addr]=40h                  ;...do Sector Unprot.
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=C0h                ;Sleep
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=E0h                ;Abort
```

Undocumented commands for hidden sector:

```text
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=77h                ;Prepare Hidden...
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=99h, [addr..]=data ;...do Hidden Write
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=E0h                ;...do Hidden Erase
  [00000h]=38h, [00000h]=D0h, [00000h]=71h, dummy=[00004] ;Prepare Hidden Rd...
  [00000h]=72h, [00000h]=75h, data=[addr...]              ;...do Hidden Read
```

#### FLASH Read/Reset Command (F0h)

Resets the chip to normal Read Data mode; this is required after most commands (in order to resume normal operation; for leaving the Get Status, Get ID, or Sleep states).

FLASH Get Status (70h) and Clear Status (50h) Clear Status resets the error flags in bit4,5 (required because those bits would otherwise stay set forever). Get Status switches to read-status mode (this is usually not required because the erase/program/protect/sleep commands are automatically entering read-status mode). The separate status bits are:

```text
  7   Write/Erase State     (0=Busy, 1=Ready)
  6   Erase Suspend         (0=Normal, 1=Sector Erase Suspended)
  5   Erase Failure         (0=Okay, 1=Fail in erase)
  4   Program Failure       (0=Okay, 1=Fail in program)
  3   Reserved (zero)                           (MX29F1610A/B)
  3   Sector-Protect Status (0=?, 1=?)          (MX29F1611 only)
  2   Sleep Mode            (0=Normal, 1=Sleep) (MX29F1611 only)
  1-0 Reserved (zero)
```

FLASH Get Maker/Device ID and Sector Protect Bytes (90h) Allows to read Maker/Device ID and/or Sector Protect Byte(s) from following address(es):

[00000h]=Manufacturer ID:

```text
  C2h = Macronix
```

[00002h]=Device ID:

```text
  FAh = MX29F1610A  ;\with sector_protect, suspend/resume, without sleep/abort
  FBh = MX29F1610B  ;/
  F7h = MX29F1611   ;-with sector_protect, suspend/resume, sleep/abort
  6Bh = MX29F1615   ;-without sector_protect, suspend/resume, sleep/abort
  F3h = MX29F1601MC ;<-- undocumented, used in SNES nintendo power carts
```

[x0004h]=Sector Protect State:

```text
  00h = normal unprotected 128Kbyte sector (can occur on all sectors)
  C2h = write-protected 128Kbyte sector (can occur on first & last sector only)
```

FLASH Erase: Prepare (80h), and Chip Erase (10h) or Sector Erase (30h) Allows to erase the whole 2Mbyte chip (ie. half of the Nintendo Power cart), or a specific 128Kbyte sector.

Some MX29F16xx chips are also allowing to suspend (B0h) or resume (D0h) sector erase (allowing to access other sectors during erase, if that should be desired).

#### FLASH Page/Byte Program (A0h)

Allows to write one or more bytes (max 80h bytes) to a 128-byte page.

The Page/Byte Program command doesn't auto-erase the written page, so the sector(s) should be manually erased prior to programming (otherwise the new bytes will be ANDed with old data).

Caution: The chips in Nintendo Power carts require the LAST BYTE written TWICE in order to start programming (unlike as in offical MX29F16xx specs, which claim programmig to start automatically after not sending further bytes for about 30..100us).

FLASH Protect: Prepare (60h), and Protect (20h) or Unprotect (40h) Allows to write-protect or unprotect separate 128Kbyte sectors (this works only for the first and last sector of each chip) (/WP=HIGH overrides the protection).

#### FLASH Sleep (C0h)

Switches the chip to sleep state; can be resumed only via Read/Reset command (F0h). Sleep mode is supported on MX29F1611 only.

#### FLASH Abort (E0h)

Aborts something. Supported on MX29F1611 only.

#### Basic MX29F16xx specs

#### JEDEC-standard EEPROM commands

Endurance: 100,000 cycles

#### Fast access time: 70/90/120ns

Sector erase architecture - 16 equal sectors of 128k bytes each - Sector erase time: 1.3s typical Page program operation - Internal address and data latches for 128 bytes/64 words per page - Page programming time: 0.9ms typical - Byte programming time: 7us in average
