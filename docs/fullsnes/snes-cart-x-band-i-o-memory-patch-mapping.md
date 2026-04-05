# SNES Cart X-Band I/O - Memory Patch/Mapping

```text
FE00h - KillReg (aka killhere) ;same as killheresoft...trans register
```

;kill register bits  ;aka "kKillReg" and/or "kKillHereSoft"?

```text
  0   HereAssert:      equ     $01 ; "Here" = cannot see cart
  1   Unknown/unused
  2   DecExcept:       equ     $04 ;
  3   Force:           equ     $08 ;
  4-7 Unknown/unused

FE02h - ControlReg (aka reghere)
```

;control bits for control register ;aka "kControlReg"? and/or "kCtlRegSoft"?

```text
  0   EnTwoRam:        equ     $01 ;<-- maybe disable one of the two SRAMs?
  1   EnSafeRom:       equ     $02 ;<-- maybe SRAM read-only? or FlashROM?
  2   RomHi:           equ     $04 ;
  3   EnInternal:      equ     $08 ;<-- maybe disable ports C000h..C1FFh?
  4   EnFixedInternal: equ     $10 ;<-- maybe whatever related to above?
  5   EnSNESExcept:    equ     $20 ;
  6-7 Unknown/unused

FF80h - KillHereSoft (aka kkillheresoft)
FF82h - CtlRegSoft (aka kctlregsoft)
```

Unknown, maybe some sort of mirrors of FE00h and FE02h ?

617000h ? weirdness kSNESKillHereSoft 617001h ? weirdness kSNESCtlRegSoft Unknown, maybe some sort of mirrors of FE00h and FE02h ?

Maybe non-Sega, SNES only stuff? Or maybe weird/ancient prototype stuff?

```text
C000h/C002h/C004h - Patch 0, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)
C008h/C00Ah/C00Ch - Patch 1, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)
C010h/C012h/C014h - Patch 2, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)
C018h/C01Ah/C01Ch - Patch 3, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)
C020h/C022h/C024h - Patch 4, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)
C028h/C02Ah/C02Ch - Patch 5, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)
C030h/C032h/C034h - Patch 6, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)
C038h/C03Ah/C03Ch - Patch 7, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)
C040h/C042h/C044h - Patch 8, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)
C048h/C04Ah/C04Ch - Patch 9, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)
C050h/C052h/C054h - Patch 10, Byte0/Byte1/Byte2 (Lo/Mid/Hi?)
```

aka "Vectors 0..10"?

```text
C070h/C072h/C074h - MagicAddr Byte0/1/2 (Lo/mid/hi) (aka magicl/m/h)
  0-23 Unknown (also referred to as "transition address"?)

C058h/C05Ah/C05Ch - Range0Start (Lo/mid/hi?)
C060h/C062h/C064h - Range1Start (Lo/mid/hi?)
C080h/C082h/C084h - Range0End (Lo/mid/hi) (aka rangel/m/h)
C088h/C08Ah/C08Ch - Range1End (Lo/mid/hi?)
  0-23 Unknown (maybe ROM start/end addresses for BIGGER patch regions?)

C0A0h/C0A2h - Range0Dest (Lo/hi) (aka trbl/h)
C0A8h/C0AAh - Range1Dest (Lo/hi?)
  0-15 Unknown (maybe SRAM mapping target for above ROM ranges?)

C0A4h - Range0Mask (aka trm)
C0ACh - Range1Mask
  0-7  Unknown

C0D0h/C0D2h - VTableBase Byte0/1 (Lo/hi) (aka kvtablel/h)
  0-15 Unknown (maybe SRAM mapping target for ROM patch vectors?)
```

vector table base address? (in 32-byte, or 32-word steps maybe?)

```text
C0D8h/C0DAh - Enable Byte0/1 (Lo/hi) (aka enbll/h)
  0-10 Vector 0-10 Enable (aka enable "kPatch_0..10"?) (?=off, ?=on)
  11   range0ena
  12   range1ena
  13   unknown/unused
  14   transAddrEnable aka magicena   ;enable transition address
  15   zeroPageEnable                 ;enable zero page  <-- game cart access?

C0C0h/C0C2h - RAMBase, Byte0/1 (Lo/hi) (aka saferambasel/h)
  0-15 Unknown

C0C8h/C0CAh - RAMBound Byte0/1 (Lo/hi) (aka saferambndl/h)
  0-15 Unknown

C0E0h - ROMBound (aka saferombnd)
  0-7  Unknown

C0E8h - ROMBase (aka saferombase)
  0-7  Unknown

C0F8h/C0FAh - AddrStatus (Lo/hi?) (aka addrstatusl/h)
  0-15 Unknown
```
