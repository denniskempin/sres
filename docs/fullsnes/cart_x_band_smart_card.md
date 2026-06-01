# SNES Cart X-Band I/O - Smart Card Reader

The X-Band contains a built-in Smart Card reader (credit card shaped chip cards with 8 gold contacts). The X-Band BIOS contains messages that refer to "XBand Cards" and "XBand Rental Cards". There aren't any photos (or other info) of these cards in the internet, maybe X-Band requested customers to return the cards, or the cards got lost for another reason.

#### Purpose

Not much known. Reportedly the card reader was used for Prepaid Cards (for users whom didn't want Xband to charge their credit cards automatically), if that is correct, then only those users would have received cards, and other users didn't need to use the card reader? Note: The Options/Account Info screen show entries "Account" and "Card".

#### Smart Card I/O Ports

The BIOS seems to be accessing the cards via these I/O ports:

```text
  FBC100h Card Data/Control/Whatever (out)
  FBC108h.Bit0 (In) Card Switch (1=card inserted, 0=card missing)
  FBC108h.Bit1 (In) Card Data (input)
```

Related BIOS functions are function 0380h..0386h (on SNES/US).

```text
C100h - SControl (aka sctl) ;smart card control
  0   outputClk:   equ     $01
  1   enOutputData:equ     $02  ;aka data direction?
  2   outputData:  equ     $04
  3   outputReset: equ     $08
  4   outputVcc:   equ     $10
  5-7 unknown/unused

C108h - SStatus (aka sstatus) ;smart card status
  0   detect:      equ     $01 Card Switch (1=card inserted, 0=card missing)
  1   dataIn:      equ     $02 Card Data (input)
  2   outputClk:   equ     $04   ;\
  3   enOutputData:equ     $08   ; Current state of Port C100h.Bit0-4 ?
  4   outputData:  equ     $10   ;
  5   outputReset: equ     $20   ;
  6   outputVcc:   equ     $40   ;/
  7   outputVpp:   equ     $80   ;-Current state of Port ????.Bit0 ?
```

???? - Smart Card control ii parameters for control ii  ;<-- from i\feq.a (uh, "control ii" is what?)

```text
  0   ksoutputvpp: equ     $01
  1-7 unknown/unused

               _______ _______
       VCC C1 |       |       | C5 GND          common smart card pinout
              |____   |   ____|                 (unknown if xband is actually
       RST C2 |    \__|  /    | C6 VPP          using that same pinout)
              |____/     \____|
       CLK C3 |    \_____/    | C7 I/O
              |____/  |  \____|
       NC? C4 |       |       | C8 NC?
              |_______|_______|
```
