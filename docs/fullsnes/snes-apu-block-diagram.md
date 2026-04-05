# SNES APU Block Diagram

DSP Voice Block Diagram (n=voice, 0..7)

```text
                    OUTx(n-1)   PITCHn
                     PMON         |    ADSRn/ENV
                       |____MUL___|     |
  DIR*256                    |          +-------------------------------> ENVxn
  +SRCn*4                    |          |             .-----------------> OUTxn
   _____      _______      __V___       |             |        _____
  |     |    |  BRR  |    | BRR  |      |    _____    |       |VOLnL|
  | RAM |--->|Decoder|--->| Time |---o  '-->|     |   |   .-->| MUL |---> Ln
  |_____|    |_______|    |______|    \     | MUL |   |   |   |_____|
                           ______ NONn o--->|     |---+---+    _____
                          | Noise|          |_____|       |   |VOLnR|
                          | Time |---o                    '-->| MUL |---> Rn
                          |______|                            |_____|
                             ^
                             |
                            FLG
```

DSP Mixer/Reverb Block Diagram (c=channel, L/R)

```text
          ________                        _____                   _____
  c0 --->| ADD    |                      |MVOLc| Master Volume   |     |
  c1 --->| Output |--------------------->| MUL |---------------->|     |
  c2 --->| Mixing |                      |_____|                 |     |
  c3 --->|        |                       _____                  | ADD |--> c
  c4 --->|        |                      |EVOLc| Echo Volume     |     |
  c5 --->|        |   Feedback   .------>| MUL |---------------->|     |
  c6 --->|        |   Volume     |       |_____|                 |_____|
  c7 --->|________|    _____     |                      _________________
                      | EFB |    |                     |                 |
     EON  ________    | MUL |<---+---------------------|   Add FIR Sum   |
  c0 -:->|        |   |_____|                          |_________________|
  c1 -:->|        |      |                              _|_|_|_|_|_|_|_|_
  c2 -:->|        |      |                             |   MUL FIR7..0   |
  c3 -:->|        |      |         ESA=Addr, EDL=Len   |_7_6_5_4_3_2_1_0_|
  c4 -:->| ADD    |    __V__  FLG   _______________     _|_|_|_|_|_|_|_|_
  c5 -:->| Echo   |   |     | ECEN | Echo Buffer c |   | FIR Buffer c    |
  c6 -:->| Mixing |-->| ADD |--:-->|   RAM         |-->| (Hardware regs) |
  c7 -:->|________|   |_____|      |_______________|   |_________________|
                                   Newest --> Oldest    Newest --> Oldest
```

#### External Sound Output & Input

```text
   _____     _____     ___________     _______     _______
  |     |   |     |   | Pre-      |   |       |   | Post- |
  | DSP |-->| D/A |-->| Amplifier |-->| Analog|-->| Ampl. |--> Multi-Out
  |_____|   |_____|   |___________|   | Mixer |   |       |    (Stereo Out)
     |                                |       |   | (with |
     |   Cartridge Slot Stereo In --->|       |   | phase |--> TV Modulator
     |                                |       |   | inver-|    (Mono Out)
     |   Expansion Port Stereo In --->|       |   | sion) |
     |                                |_______|   |       |--> Expansion Port
     |   /MUTE signal                     |       |       |    (Mono Out)
     '------------------------------->----'       |_______|
```

Note: The Cartridge Audio input is used only by SGB and MSU1. Unknown if it is also used by X-Band modem (for injecting dial/connection sounds)? The Expansion port input might be used by the Satellaview (?) and SNES CD prototype.

The Nintendo Super System (NSS) also allows to mute the SNES sound via its Z80 CPU.
