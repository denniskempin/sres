# SNES Cart X-Band I/O Map

Below I/O Map is based on source code of the Sega Genesis X-Band version (files i\harddef.a and i\feq.a). The I/O Map of the SNES version might differ in some places.

#### default base addresses

```text
  kDefaultInternal:   equ     ($1de000*2)     ;;=3BC000h   ;aka SNES: FBC000h
  kDefaultControl:    equ     ($1dff00*2)     ;;=3BFE00h   ;aka SNES: FBFE00h
```

#### X-Band I/O Map

```text
  Addr  $nn*2 i\harddef.a      i\feq.a     ;Comment
  ----------------------------------------------------------------------------
  C000h $00*2 kPatch_0_Byte0   -   (lo?)   ;Translation (Patch Addr) regs ...
  C002h $01*2 kPatch_0_Byte1   -   (mid?)  ;(aka "Vectors 0..10"?)
  C004h $02*2 kPatch_0_Byte2   -   (hi?)
  C006h       N/A              -
  C008h $04*2 kPatch_1_Byte0   -
  C00Ah $05*2 kPatch_1_Byte1   -
  C00Ch $06*2 kPatch_1_Byte2   -
  C00Eh       N/A              -
  C010h $08*2 kPatch_2_Byte0   -
  C012h $09*2 kPatch_2_Byte1   -
  C014h $0A*2 kPatch_2_Byte2   -
  C016h       N/A              -
  C018h $0C*2 kPatch_3_Byte0   -
  C01Ah $0D*2 kPatch_3_Byte1   -
  C01Ch $0E*2 kPatch_3_Byte2   -
  C01Eh       N/A              -
  C020h $10*2 kPatch_4_Byte0   -
  C022h $11*2 kPatch_4_Byte1   -
  C024h $12*2 kPatch_4_Byte2   -
  C026h       N/A              -
  C028h $14*2 kPatch_5_Byte0   -
  C02Ah $15*2 kPatch_5_Byte1   -
  C02Ch $16*2 kPatch_5_Byte2   -
  C02Eh       N/A              -
  C030h $18*2 kPatch_6_Byte0   -
  C032h $19*2 kPatch_6_Byte1   -
  C034h $1A*2 kPatch_6_Byte2   -
  C036h       N/A              -
  C038h $1C*2 kPatch_7_Byte0   -
  C03Ah $1D*2 kPatch_7_Byte1   -
  C03Ch $1E*2 kPatch_7_Byte2   -
  C03Eh       N/A              -
  C040h $20*2 kPatch_8_Byte0   -
  C042h $21*2 kPatch_8_Byte1   -
  C044h $22*2 kPatch_8_Byte2   -
  C046h       N/A              -
  C048h $24*2 kPatch_9_Byte0   -
  C04Ah $25*2 kPatch_9_Byte1   -
  C04Ch $26*2 kPatch_9_Byte2   -
  C04Eh       N/A              -
  C050h $28*2 kPatch_10_Byte0  -
  C052h $29*2 kPatch_10_Byte1  -
  C054h $2A*2 kPatch_10_Byte2  -
  C056h       N/A              -
  C058h $2C*2 kRange0Start     -
  C05Ah         ""-mid?        -
  C05Ch         ""-hi?         -
  C05Eh       N/A              -
  C060h $30*2 kRange1Start     -
  C062h         ""-mid?        -
  C064h         ""-hi?         -
  C066h       N/A              -
  C068h       N/A              -
  C06Ah       N/A              -
  C06Ch       N/A              -
  C06Eh       N/A              -
  C070h $38*2 kMagicAddrByte0  kmagicl
  C072h $39*2 kMagicAddrByte1  kmagicm
  C074h $3A*2 kMagicAddrByte2  kmagich
  C076h       N/A              -
  C078h       N/A              -
  C07Ah       N/A              -
  C07Ch       N/A              -
  C07Eh       N/A              -
  C080h $40*2 kRange0End       krangel
  C082h         ""-mid?        krangem
  C084h         ""-hi?         krangeh
  C086h       N/A              -
  C088h $44*2 kRange1End       -
  C08Ah         ""-mid?        -
  C08Ch         ""-hi?         -
  C08Eh       N/A              -
  C090h       N/A              -
  C092h       N/A              -
  C094h       N/A              -
  C096h       N/A              -
  C098h       N/A              -
  C09Ah       N/A              -
  C09Ch       N/A              -
  C09Eh       N/A              -
  C0A0h $50*2 kRange0Dest      ktrbl
  C0A2h         ""-hi?         ktrbh
  C0A4h $52*2 kRange0Mask      ktrm
  C0A6h       N/A              -
  C0A8h $54*2 kRange1Dest      -
  C0AAh         ""-hi?         -
  C0ACh $56*2 kRange1Mask      -
  C0AEh       N/A              -
  C0B0h       N/A              -
  C0B2h       N/A              -
  C0B4h       N/A              -
  C0B6h       N/A              -
  C0B8h       N/A              -
  C0BAh       N/A              -
  C0BCh       N/A              -
  C0BEh       N/A              -
  C0C0h $60*2 kRAMBaseByte0    ksaferambasel
  C0C2h $61*2 kRAMBaseByte1    ksaferambaseh
  C0C4h       N/A              -
  C0C6h       N/A              -
  C0C8h $64*2 kRAMBoundByte0   ksaferambndl
  C0CAh $65*2 kRAMBoundByte1   ksaferambndh
  C0CCh       N/A              -
  C0CEh       N/A              -
  C0D0h $68*2 kVTableBaseByte0 kvtablel ;\vector table base address?
  C0D2h $69*2 kVTableBaseByte1 kvtableh ;/ (in 32-byte, or 32-word steps
```

#### maybe?)

```text
  C0D4h       N/A              -
  C0D6h       N/A              -
  C0D8h $6c*2 kEnableByte0     kenbll
  C0DAh $6d*2 kEnableByte1     kenblh
  C0DCh       N/A              -
  C0DEh       N/A              -
  C0E0h $70*2 kROMBound        ksaferombnd
  C0E2h       N/A              -
  C0E4h       N/A              -
  C0E6h       N/A              -
  C0E8h $74*2 kROMBase         ksaferombase
  C0EAh       N/A              -
  C0ECh       N/A              -
  C0EEh       N/A              -
  C0F0h       N/A              -              ;<-- but this is used on SNES !?!
  C0F2h       N/A              -              ;<-- but this is used on SNES !?!
  C0F4h       N/A              -
  C0F6h       N/A              -
  C0F8h $7c*2 kAddrStatus      kaddrstatusl
  C0FAh         ""-hi?         kaddrstatush
  C0FCh       N/A              -
  C0FEh       N/A              -
  C100h $80*2 kSControl        ksctl                  ;smart card control
  C102h       N/A              -
  C104h       N/A              -
  C106h       N/A              -
  C108h $84*2 kSStatus         ksstatus               ;smart card status
  C10Ah       N/A              -
  C10Ch       N/A              -
  C10Eh       N/A              -
  C110h $88*2 kReadMVSyncLow   kreadmvsync    ;<--Low? ;\Range of 0 to $61.
  C112h $89*2 kReadMVSyncHigh  kreadmvsynclow ;<--low? ;/Equal to
  C114h       N/A              -                       ; ReadSerialVCnt/2.
  C116h       N/A              -                       ; Value is $5c at start
  C118h $8c*2 kMStatus1        kmstatus1               ; of VBlank.
  C11Ah       N/A              -
  C11Ch       N/A              -
  C11Eh       N/A              -
  C120h $90*2 kTxBuff          ktxbuff            ; modem (and serial) bits ...
  C122h       N/A              -
  C124h       N/A              -
  C126h       N/A              -
  C128h $94*2 kRxBuff          krxbuff
  C12Ah       N/A              -
  C12Ch       N/A              -
  C12Eh       N/A              -
  C130h $98*2 kReadMStatus2    kreadmstatus2
  C132h       N/A              -
  C134h       N/A              -
  C136h       N/A              -
  C138h $9c*2 kReadSerialVCnt  kreadserialvcnt
  C13Ah       N/A              -
  C13Ch       N/A              -
  C13Eh       N/A              -
  C140h $a0*2 kReadMStatus1    kreadmstatus1
  C142h       N/A              -
  C144h       N/A              -
  C146h       N/A              -
  C148h $a4*2 kGuard           kguard
  C14Ah       N/A              -
  C14Ch       N/A              -
  C14Eh       N/A              -
  C150h $a8*2 kBCnt            kbcnt
  C152h       N/A              -
  C154h       N/A              -
  C156h       N/A              -
  C158h $ac*2 kMStatus2        kmstatus2
  C15Ah       N/A              -
  C15Ch       N/A              -
  C15Eh       N/A              -
  C160h $b0*2 kVSyncWrite      kvsyncwrite
  C162h       N/A              -
  C164h       N/A              -
  C166h       N/A              -
  C168h $b4*2 kLEDData         kleddata
  C16Ah $b5*2 kLEDEnable       kledenable
  C16Ch       N/A              -
  C16Eh       N/A              -
  C170h       N/A              -
  C172h       N/A              -
  C174h       N/A              -
  C176h       N/A              -
  C178h       N/A              -
  C17Ah       N/A              -
  C17Ch       N/A              -
  C17Eh       N/A              -
  C180h $c0*2 kModem           - ;<-- base for rockwell registers (C180h-C1BEh)

  FC02h       N/A              -      ;<-- unknown, but this is used by SNES
  FE00h $00*2 kKillReg         kkillhere ;same as killheresoft...trans register
  FE02h $01*2 kControlReg      kreghere
  FF80h $c0*2 kKillHereSoft    kkillheresoft  ;\maybe some sort of mirrors of
  FF82h $c1*2 kCtlRegSoft      kctlregsoft    ;/FE00h and FE02h ?

  617000h ? weirdness kSNESKillHereSoft       ;\maybe some sort of mirrors of
  617001h ? weirdness kSNESCtlRegSoft         ;/FE00h and FE02h ?

  FFC000h          I/O Port  (unknown functions?) ;-bank FFh ;\
  004F02h          I/O Port  (unknown functions?) ;\         ; whatever, used
  00F000h          Dummy/strobe read?             ; bank 00h ; by SNES version
  00FFE0h          Dummy/strobe read?             ;/         ;/
```
