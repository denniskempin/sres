# SNES Cart X-Band Rockwell Notes

#### Rockwell Configuration Changes

Various changes (to ASYNC, WDSZ, etc.) seem to be not immediately applied.

Instead, one must apply them by setting NEWC=1 by software (and then wait until hardware sets NEWC=0).

#### Rockwell Dialing

Dialing is done by setting CONF=81h, and then writing the telephone number digits (range 00h..09h) to TBUFFER; before each digit wait for TDBE=1 (TX buffer empty), the BIOS also checks for TONEA=1 before dialing.

The telephone number for the X-Band server is stored as ASCII string in the BIOS ROM:

```text
  "18002071194"  at D819A0h in US-BIOS (leading "800" = Toll-free?)
  "03-55703001"  at CE0AB2h in Japanese BIOS (leading "3" = Tokyo?)
```

Notes: Before dialing the above 'ASCII' numbers, the US-BIOS first dials 0Ah,07h,00h, and the japanese one first dials 01h. The "-" dash in the japanese string isn't dialed.

#### Rockwell Offline

There seems to be no explicit offline mode (in CONF register). Instead, one must probably change the Relay A/B bits (RA/RB) to go online/offline.

#### X-Band Fred Chip Pin-Outs

```text
  1-100 unknown
```

#### X-Band Rockwell Pin-Outs

```text
  Pin Number Signal Name I/O Type
  1 RS2 IA
  2 RS1 IA
  3 RS0 IA
  4 /TEST1
  5 /SLEEP OA
  6 RING
  7 EYEY OB
  8 EYEX OB
  9 EYESYNC OB
  10 RESET ID
  11 XTLI IE
  12 XTLO OB
  13 +5VD
  14 GP18 OA
  15 GP16 OA
  16 XTCLK IA
  17 DGND1
  18 TXD IA
  19 TDCLK OA
  20 TRSTO MI
  21 TSTBO MI
  22 TDACO MI
  23 RADCI MI
  24 RAGCO MI
  25 MODEO MI
  26 RSTBO MI
  27 RRSTO MI
  28 /RDCLK OA
  29 RXD OA
  30 TXA2 O(DD)
  31 TXA1 O(DD)
  32 RXA I(DA)
  33 RFILO MI
  34 AGCIN MI
  35 VC
  36 NC
  37 NC
  38 NC
  39 /RBDVR OD
  40 AGND
  41 /RADRV OD
  42 /SLEEP1 IA
  43 RAGCI MI
  44 NC
  45 RSTBI MI
  46 RRSTI MI
  47 RADCO MI
  48 TDACI MI
  49 TRSTI MI
  50 TSTBI MI
  51 MODE1 MI
  52 +5VA
  53 SPKR O(OF)
  54 DGND2
  55 D7 IA/OB
  56 D6 IA/OB
  57 D5 IA/OB
  58 D4 IA/OB
  59 D3 IA/OB
  60 D2 IA/OB
  61 D1 IA/OB
  62 D0 IA/OB
  63 /IRQ OC
  64 /WRITE IA
  65 /CS IA
  66 /READ IA
  67 RS4 IA
  68 RS3 IA
```

Notes:

(1) MI = Modem Interconnection (2) NC = No connection (may have internal connection; leave pin disconnected (open).

(3) I/O types are described in Table 2-3 (digital signals) and Table 2-4 (analog signals).
