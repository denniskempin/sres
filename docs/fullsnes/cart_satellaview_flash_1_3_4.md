# SNES Cart Satellaview I/O FLASH Access (Type 1,3,4)

The Type 1,3,4 protocol is somewhat compatible to Sharp LH28F032SU/LH28F320SK chips (which has also a same/similar 52pin package). Concerning the commands used by the BIOS, Type 1,3,4 seems to be exactly the same - except that Type 3 doesn't support the Erase-Entire chip command.

Erase Entire Chip (Type 1 and 4 only) (not supported by Type 3)

```text
  [C00000h]=50h                                 ;clear status register
  [C00000h]=71h                                 ;enter status mode
  repeat, X=[C00004h], until (X.bit3=0)         ;wait until VPP voltage okay
  [C00000h]=A7h  ;"erase all unlocked pages"?   ;select erase entire-chip mode
  [C00000h]=D0h                                 ;start erase
  [C00000h]=71h                                 ;enter status mode
  repeat, X=[C00004h], until (X.bit7=1)         ;wait until ready
  if (X.bit5=1) then set erase error flag       ;check if erase error
  [C00000h]=FFh                                 ;terminate status mode
```

#### Unknown Command

Same as Erase Entire (see above), but using 97h instead of A7h, and implemented ONLY for Type 1 and 4, that is: NOT supported (nor simulated by other commands) for neither Type 2 nor Type 3. Maybe A7h erases only unlocked pages, and 97h tries to erase all pages (and fails if some are locked?).

#### Erase 64KByte Sector

```text
  [C00000h]=50h                                 ;clear status register
  [C00000h]=20h                                 ;select erase sector mode
  [nn0000h]=D0h                                 ;start erase 64K bank nn
  [C00000h]=70h                                 ;enter status mode
  repeat, X=[C00000h], until (X.bit7=1)         ;wait until ready
```

;; if (X.bit5=1) then set erase error flag       ;check if erase error

```text
  [C00000h]=FFh                                 ;terminate status mode
```

#### Write Data

```text
  FOR i=first to last
    [C00000h]=10h                               ;write byte command
    [nnnnnnh+i]=data[i]                         ;write one data byte
    [C00000h]=70h                               ;enter status mode
    repeat, X=[C00000h], until (X.bit7=1)       ;wait until ready
  NEXT i
  [C00000h]=70h                                 ;enter status mode
  repeat, X=[C00000h], until (X.bit7=1)         ;hmmm, wait again
  if (X.bit4=1) then set write error flag       ;check if write error
  [C00000h]=FFh                                 ;terminate status mode
```

#### Enter Erase-Status Mode

```text
  [C00000h]=71h                                 ;enter status mode
  X=[C00004h]                                   ;read status byte
  IF (X.bit7=0) THEN busy
  IF (X.bit3=1) THEN not-yet-ready-to-erase (VPP voltage low)
  IF (X.bit7=1) AND (X.bit5=0) THEN ready/okay
  IF (X.bit7=1) AND (X.bit5=1) THEN erase error ?
```

#### Enter Other-Status Mode

```text
  [C00000h]=70h                                 ;enter status mode
```

#### Terminate Command

```text
  [C00000h]=FFh                                 ;terminate
```

Used to leave status or chip-detection mode.

BUGs: On Type 3 chips, the BIOS tries to simulate the "Erase-Entire" command by issuing multiple "Erase-Sector" commands, the bug there is that it tests bit4 of the flash_size in 128Kbyte block units (rather than bit4 of the flash_status) as erase-error flag (see 80BED2h); in practice, that means that the erase-entire will always fail on 2MByte chips (and always pass okay on all other chips); whereas, erase-entire is used when downloading files (except for small files that can be downloaded or relocated to PSRAM).
