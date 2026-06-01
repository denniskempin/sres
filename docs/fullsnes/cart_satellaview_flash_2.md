# SNES Cart Satellaview I/O FLASH Access (Type 2)

Type 2 protocol is completely different as for Type 1,3,4 (aside from the Chip Detection sequence, which is same for all types).

#### Erase Entire Chip

```text
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=80h   ;unlock erase
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=10h   ;do erase entire chip
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=70h   ;enter status mode
  repeat, X=[C05555h], until (X.bit7=1)         ;wait until ready
  if (X.bit5=1) then set erase error flag       ;check if erase error
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=F0h   ;terminate status mode
```

#### Erase 64KByte Sector

```text
  [C00000h]=50h                                 ;huh? (maybe a BIOS bug)
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=80h   ;unlock erase
  [C05555h]=AAh, [C02AAAh]=55h, [nn0000h]=30h   ;do erase bank nn
  repeat, X=[C05555h], until (X.bit7=1)         ;wait until ready
  if (X.bit5=1) then set erase error flag       ;check if erase error
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=F0h   ;terminate status mode
```

#### Write 1..128 Bytes (within a 128-byte boundary)

```text
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=A0h   ;enter write mode
  FOR i=first to last, [nnnnnn+i]=data[i]       ;write 1..128 byte(s)
  [nnnnnn+last]=DATA[last]   ;write LAST AGAIN  ;start write operation
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=70h   ;enter status mode
  repeat, X=[C05555h], until (X.bit7=1)         ;wait until ready
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=F0h   ;terminate status mode
```

#### Enter Status Mode

```text
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=70h   ;enter status mode
  X=[C05555h]                                   ;read status byte
  IF (X.bit7=0) THEN busy
  IF (X.bit7=1) AND (X.bit5=0) THEN ready/okay
  IF (X.bit7=1) AND (X.bit5=1) THEN ready/erase error ?
```

#### Terminate Command

```text
  [C05555h]=AAh, [C02AAAh]=55h, [C05555h]=F0h   ;terminate
```

Used to leave status or chip-detection mode.

#### Bugged Commands

The are some cases (BIOS addresses 80BF88h, 80DBA4h, 80E0D5h) where Type 2 programming is mixed up with some non-Type-2 commands (setting [C00000h]=50h and [C00000h]=FFh), these commands are probably ignored by chip (or switching it back default mode).
