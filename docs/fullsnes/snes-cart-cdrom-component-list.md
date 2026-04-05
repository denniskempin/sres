# SNES Cart CDROM - Component List

"based on the on photos that have been posted, the main board has the same parts as a Super Famicom, but with 7 additional chips:

```text
  1) CXD2500 CD-DSP
  2) CXD1800 CD-ROM decoder/interface
  3) 32K SRAM (presuambly the CD-ROM sector buffer)
  4) some 20 pin SOP device that looks like a bus buffer
  5) a QFP with no markings (mechacon MCU?)
  6) A Sanyo 16 bit stereo DAC
  7) an 8 pin SOP - probably a dual opamp (it's next to the DAC outputs,
     so probably a buffer)
  The top board has a 4-bit MCU and a liquid crystal display.
```

There are also 5 visible ICs on the back of the CD-ROM control board - one of them is a Rohm BTL driver another looks like a Sony CXA1272 (old CD drive focus / tracking servo) - the other chips are small SOP devices with numbers I can't read. No sign of an RF amp chip, but on a lot of those older drives it was built into the optical pickup. Basically, it has all the chips you would expect for a basic data/audio CD drive of that vintage and nothing else."

Sony Playstation SFX-100 Console Component List  Mainboard (MA-115, 0-396-987-04)

```text
  IC101 100pin Nintendo S-CPU, 5A22-01  (65816 CPU with joypad I/O ports)
  IC102 100pin Nintendo S-PPU1, 5C77-01 (Video Chip 1)
  IC103 100pin Nintendo S-PPU2, 5C78-01 (Video Chip 2)
  IC104 28pin  NEC uPD43256A6U-10L? (32Kx8 SRAM, Video RAM 1)
  IC105 28pin  NEC uPD43256A6U-10L? (32Kx8 SRAM, Video RAM 2)
  IC106        ... whatever, maybe S-ENC or similar (Video RGB to composite)
  IC107 64pin  Nintendo S-WRAM (128Kx8 DRAM with B-bus)
  IC108 28pin  65256BLFP-12T   (32Kx8 SRAM, Sound RAM 1)
  IC109 18pin  Nintendo F411   (NTSC CIC)
  IC110 28pin  65256BLFP-12T   (32Kx8 SRAM, Sound RAM 2)
  IC111 64pin  SONY CXP1100Q-1 (APU, some newer S-SMP revision, SPC700 CPU)
  IC112 80pin  SONY CXD1222Q-1 (APU, some newer S-DSP revision, Sound Chip)
  IC113 20pin  LC78815M        (Two-channel 16bit D/A converter 1)
  IC201 80pin  SONY CXD2500AQ  (CDROM Signal Processor)
  IC202 20pin  LC78815M        (Two-channel 16bit D/A converter 2)
  IC203 48pin  Noname   ...  maybe Servo Amplifier (like CXA1782BR on PSX?)
  IC204 80pin  SONY CXD1800Q    (CDROM Decoder/FIFO, equivalent to CXD1196AR)
  IC205 18pin  74xxxx? (PCB has 20pin solderpoints, but chip is only 18pin)
  IC206 28pin  SONY CXK58257AM-70L (32Kx8 SRAM, CDROM Sector Buffer)
  IC301 8pin   Texas Instruments RC4558, "R4558 TI 25" (Dual Op-Amp 1)
  IC302 ...    ... whatever, maybe one of the 8pin IC???'s
  IC303 8pin   Texas Instruments RC4558, "R4558 TI 25" (Dual Op-Amp 2)
  ICxxx ...    if any... ?
  IC??? 8pin   whatever (front board edge, near headphone socket)
  IC??? 8pin   whatever (front board edge, near headphone socket)
  IC??? 24pin  whatever (front/mid board edge) (probably S-ENC or so)
  IC??? 3pin   voltage regulator (7805 or similar)
  IC??? ??     address decoder for I/O ports, 21E4h latch, NEXT port...?
               (maybe IC203 is doing that? but then where's Servo Amplifier?)
  CN201 29pin  To LCD Board
  CN..  ..     To CDROM Drive
  CN..  ..     To Controller Ports
  CN..  62pin  SNES Cartridge Slot
  CN..  ..     Rear panel
```

#### Daughterboard with LCD

```text
  IC701  80pin NEC uPD75P308GF  (CDROM Mechacon?)
  IC7xx ...    if any... ?
  X701         oscillator
  CN701  28pin LCD and six Buttons    (28pin, or maybe 2x28 pins?)
  CN702   4pin to somewhere  (2 LEDs ?, left of drive tray)
  CN703  29pin to Mainboard           (29pin, or maybe 2x29 pins?)
  CN704   3pin to front panel (disc eject button?, right of drive tray)
  ICxxx ...    if any... ?
  N/A?   28pin Something like BA6297AFP,BA6398FP,BA6397FP,AN8732SB,etc ?
```

#### Daughterboard with controller ports

```text
  ???   ...    whatever
```

#### Daughterboard with Eject button

```text
  ???   ...    whatever, a button, and maybe more stuff for the 3pin wire
```

#### Daughterboard with LEDs

```text
  ???   ...    whatever, two LEDs, and maybe more stuff for the 4pin wire
```

#### Components in actual CD Drive unit

```text
  ???   ...    whatever
```

#### External Connectors

```text
  1x snes cartridge slot (top)
  2x controller ports (front)
  1x 3.5mm headphone socket with "voltage level" regulator (front)
  1x "NEXT" port (serial link like PSX maybe?)
  1x Audio R        (red) (apparently with mono-switch when not connected)
  1x Audio L (MONO) (white)
  1x Video          (yellow)
  1x S VIDEO
  1x RF DC OUT
  1x MULTI OUT
  1x DC IN 7.6V
```

Note: Some other/similar model has three RCA jacks instead headphone on front

#### BIOS Cartridge - Case Sticker "'92.10.6." (plus some japanese symbols)

```text
  PCB "RB-01, K-PE1-945-01"
  IC1  64pin  Nintendo S-WRAM (128Kx8 DRAM)
  IC2  64pin  Nintendo S-WRAM (128Kx8 DRAM)
  IC3  32pin  HN2xxxxxx? (Sticker 0.95 SX) (ROM/EPROM)
  IC4  28pin  SONY CXK5864BM-12LL (8Kx8 SRAM)
  IC5  16pin  Noname?
  IC6  14pin  74F32 (Quad 2-input OR gates)
  IC7  16pin  74F138? (1-of-8 inverting decoder/demultiplexer?)
  IC8  14pin  Noname?
  IC9  14pin  Noname?
  IC10 16pin  Noname?
  IC11 16pin  Nintendo D411 (NTSC CIC)
  ?    ?      white space (in upper left)
  ?    2pin   something with 2 pins is apparently on PCB back side (battery?)
```

#### LCD/Button Panel

```text
           PlayStation
                       SFX-100
  .---------------------------.
  |    TRACK  STEP/MIN SEC    |
  |  .---------------------.  |
  |  |        (LCD)        |  |
  |  '---------------------'  |
  |   PLAY MODE     REMAIN    |
  |  =========== ===========  |
  |      |<<         >>|      |
  |  =========== ===========  |
  |     |> ||         []      |
  |  =========== ===========  |
  '---------------------------'
```

Note: The date codes on the three S-WRAM's, D411, and uPD75P308GF seem to be from 1991. Sticker on case of BIOS cart seems to be from 1992.
