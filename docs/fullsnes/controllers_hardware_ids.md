# SNES Controllers Hardware ID Codes

Controller ID Bits (13th-16th bit, or extended: 13th-24th bit)

```text
  13th ... 24th Hex  Type
  0000.00000000 0.00 No controller connected
  0000.11111111 0.FF Normal Joypad (probably also 3rd-party joypads/joysticks)
  0001          1    Mouse
  0010          2 ?  Unknown (if any)
  0011          3    SFC Modem (used by JRA PAT)
  0100          4    NTT Data Controller Pad (used by JRA PAT)
  ....          5-C  Unknown (if any)
  1101          D    Voice-Kun (IR-transmitter/receiver, for CD Players)
  1110.xxxxxxxx E.xx Third-Party Devices (see below)
  1110.000000xx E.0x Epoch Barcode Battler II (detection requires DELAYS?!)
  1110.01010101 E.55 Konami Justifier
  1110.01110111 E.77 Sunsoft Pachinko Controller
  1110.11111110 E.FE ASCII Turbo File Twin in STF mode
  1110.11111111 E.FF ASCII Turbo File Twin in TFII mode (or Turbo File Adapter)
  1111          F    Nintendo Super Scope
  N/A           N/A  M.A.C.S. (no ID, returns all bits = trigger button)
```

Note: The Multiplayer 5 can be also detected (using a different mechansim than reading bits 13th-16th).

Devices with unknown IDs (if they do have any special controller IDs at all)

```text
  Lasabirdie            ;\
  Twin Tap              ; these should have custom IDs?
  Miracle Piano         ;
  X-Band Keyboard       ;/
  Exertainment          ;-connects to expansion port (thus no controller id)
  BatterUP                                              ;\
  TeeV Golf                                             ; these might return
  StuntMaster                                           ; normal "joypad" ID?
  Nordic Quest                                          ;
  Hori SGB Commander (in normal mode / in SGB mode)     ;
  Nintendo Joysticks                                    ;/
```
