# SNES Controllers Multiplayer 5 (MP5) (Five Player Adaptor)

The MP5 plugs into one Controller Port on the SNES (typically Port 2), and has 4 ports for controllers to be plugged into it (labeled 2 through 5). It also has an override switch which makes it pass through Pad 2 and ignore everything else.

#### Reading Controller Data

```text
  [4016h].Bit0=1                                 ;-strobe on (to player 1-5)
  [4016h].Bit0=0                                 ;-strobe off (to player 1-5)
  read any number of bits from [4016h].Bit0      ;-read Player 1 data
  read any number of bits from [4017h].Bit0/Bit1 ;-read Player 2/3 data
  [4201h].Bit7=0                                 ;-select Player 4/5
  read any number of bits from [4017h].Bit0/Bit1 ;-read Player 4/5 data
  [4201h].Bit7=1  ;(prepare here for next frame) ;-select Player 2/3
  do no further access until next frame (allow [4201h].Bit7=1 to stabilize)
```

The strobe on/off part, and reading first 16bits for player 1-3 is usually done via automatic reading (whereas, Player 3 data will obviously show up in "JOY4" register, not in "JOY3" register). Whilst reading further player 1-3 bits, and all player 4-5 bits is done via manual reading.

As shown above, player 2-3 should be always read before 4-5, for two reasons:

At least some MP5 devices may respond slowly on 0-to-1 transitions of [4201h].Bit7 (unless the device contains a pull-up resistor). Some MP5's (namely the Tribal Tap) are always passing CLK to player 2 (in that case player 2 data would be shifted-out when accessing player 4-5 data).

#### Detecting the MP5 Hardware

Below can be used to detect MP5 in ports 1 (n=0) and 2 (n=1). Games do usually check both ports (and show an error messages when sensing a MP5 in port 1).

```text
  [4016h].Bit0=1                              ;-strobe on (force MP5 Bit1=1)
  read 8 bits from [4016h+n].Bit1 to byte A   ;-read byte A
  [4016h].Bit0=0                              ;-strobe off (normal data mode)
  read 8 bits from [4016h+n].Bit1 to byte B   ;-read byte B
  if A=FFh and B<>FFh then MP5=present        ;-verify result
```

If there's no MP5 connected, then A and B will be typically 00h (since most controllers don't use [4017h].Bit1, exceptions are Turbo File, SFC Modem, Voice-Kun, and X-Band Keyboard).

If a MP5 is connected, then A will be FFh, and B will be first 8bit of data from joypad 3 or 5 (which can't be FFh since one can't push all four DPAD directions at once).

Also note that there is nothing preventing the MP5 from functioning perfectly when plugged in to Port 1, except that the game must use bit 6 of $4201 instead of bit 7 to set IOBit and must use the Port 1 registers instead of the Port 2 registers. With 2 MP5 units, one could actually create an 8-player game.

#### Supported/Unsupported Games/Hardware

The Multiplayer is supported by more than 100 games, but incompatible with almost everything except normal joypads.

> **See:** [SNES Controllers Multiplayer 5 - Unsupported Hardware](snes-controllers-multiplayer-5-unsupported-hardware.md)
> **See:** [SNES Controllers Multiplayer 5 - Supported Games](snes-controllers-multiplayer-5-supported-games.md)

#### Multitap/Multiplayer Adaptor Versions

```text
  2or3? Way Multiplay Adaptor (Gamester LMP) (with only 2 (or 3?) sockets)
  5 Player Game Plug (Laing) (same polygonal case as SN-5)
  HORI Multitap HSM-07 (HORI) (4 "top-loading" connectors)
  HORI Super Tetris 3 (HORI) (red case, otherwise same as HORI HSM-07)
  Multi Adaptor Auto (Partyroom21)
  Multi Player Adaptor (unknown manufacturer) (roughly PS1 shaped)
  Multi-Player Adaptor (Super Power) (same case as Multiplay Adaptor from LMP)
  Multiplay Adaptor (Gamester LMP) (square gray case, "crown" shaped LMP logo)
  SN-5 Multitap (Phase 9) (same polygonal case as Super 5 QJ/Super 5-Play)
  SNES MultiPlayer 5 Schematic Diagram (1st May 1992) (Nintendo) (book2.pdf)
  Super 5 QJ (same polygonal case as SN-5)
  Super 5 Multi-Player Adapter by Innovation (same polygonal case as SN-5)
  Super 5-Play (Performance) (same polygonal case as SN-5)
  Super Link by BPS (Bullet Proof Software) (same case as HORI HSM-07)
  Super Multitap (noname) (polyshaped, but different than the SN-5 case)
  Super Multitap (Hudson) (long slim device with 4 connectors on front panel)
  Super Multitap 2 (Hudson) (square device with yellow Bomberman face)
  Super Multitap Honest (same polygonal case as SN-5)
  Tribal-Tap 5 (Nakitek) (same case as Multiplay Adaptor from LMP)
  Tribal Tap, 6 Player Adaptor (Naki)
  Tribal Tap, 6 Player Adaptor (Fire) (same as Naki, but without Naki logo)
```

SNES MultiPlayer 5 - Schematic Diagram (Rev 2.3) 1st May 1992

```text
              _________                            _________
             |74HCT4053|                          | 74HC241 |
             |         |               /MODE5P--->|/OE      |
     ??? --->|VEE   /EN|<------STB--------------->|IN    OUT|---> STB'BCD
             |  _ _ _  |                 STB'A--->|IN    OUT|---> DETECT
  IO'SEL --->|SELX   X1|------>CLK1-------------->|IN    OUT|---> CLK'B
     CLK --->|X _ _ _X0|------>CLK0-------------->|IN _ _OUT|---> CLK'CD
  IO'SEL --->|SELY   Y1|<-------------------------|OUT    IN|<--- IN0'A
     IN0 <---|Y _ _ _Y0|<-------------------------|OUT    IN|<--- IN0'C
  IO'SEL --->|SELZ   Z1|<-------------------------|OUT    IN|<--- IN01
     IN1 <---|Z      Z0|<-------------------------|OUT    IN|<--- IN0'D
             |         |                  ??? --->|OE       |
             |_________|                          |_________|
              _________                            _________
             | 74HC126 |                          |4-Channel|
     CLK --->|IN    OUT|---> CLK1         VCC --->|2P Switch|---> /MODE5P
   STB'A --->|OE _ _   |                  GND --->|5P _ _ _ |
    CLK1 --->|IN    OUT|---> CLK'A        GND --->|2P       |---> VCC'BCD
     ??? --->|OE _ _   |                  VCC --->|5P _ _ _ |
     STB --->|IN    OUT|---> STB'A      IN1'A --->|2P       |---> IN01
     ??? --->|OE _ _   |                IN0'B --->|5P _ _ _ |
     GND --->|IN    OUT|---> IN1         IO'A <---|2P       |<--- IO
  DETECT --->|OE       |               IO'SEL <---|5P       |
             |_________|                          |_________|
   __________________________________________
  |   (Female)(............Male.............)|     GND ------[10K]----- DETECT
  |Pin SNES   PORT2   PORT3   PORT4   PORT5  |     VCC ------[10K]----- IO'SEL
  |1   VCC    VCC     VCC'BCD VCC'BCD VCC'BCD|     VCC ------[10K]----- CLK1
  |2   CLK    CLK'A   CLK'B   CLK'CD  CLK'CD |     VCC ------[10K]----- CLK0
  |3   STB    STB'A   STB'BCD STB'BCD STB'BCD|     VCC ------[10K]----- IN0'A
  |4   IN0    IN0'A   IN0'B   IN0'C   IN0'D  |     VCC ------[10K]----- IN01
  |5   IN1    IN1'A   -       -       -      | VCC'BCD ------[10K]----- IN0'C
  |6   IO     IO'A    -       -       -      | VCC'BCD ------[10K]----- IN0'D
  |7   GND    GND     GND     GND     GND    | VCC'BCD --<LED|--[220]-- VCC
  |__________________________________________| VCC'BCD --------||------ GND
```

The schematic was released by Nintendo (included in book2.pdf), components are:

```text
  74HCT4053 (triple 2-to-1 line analog multiplexer/demultiplexer)
  74HC126 (quad 3-state noninverting buffer with active high enables)
  74HC241 (dual 4-bit 3-state noninverting buffer/line driver)
  4-channel 2-position switch (2P/5P-mode selection)
  LED (glows in 2P-mode)
  1 female joypad connector, 4 male joypad connectors
  plus some resistors
```

Connection of the four "???" pins is unclear (maybe just wired to VCC or GND).

Unknown if any of the existing adaptors do actually use the above schematic (Hudson's Multitap and Multitap 2 are both using a single custom 20pin "HuC6205B" instead of the above schematic).

#### Tribal Tap (Naki)

This adaptor is supposed to support up to 6 players (one more than the normal multitaps). The 6-player feature isn't supported by any games, and it's unknown how to access the 6th port by software - some people do believe that it isn't possible at all, and the the 6th port is just a fake - but, that theory is based on the (incorrect) assumption that PALs cannot be programmed to act as flipflops. However, if the schematic shown below is correct (the "IN0'E" signal from Port6 being really & solely <input> to the OUT0 <output> pin; not verified), then it's probably really a fake.

The Tribal Tap schematic should be reportedly looking somehow like so:

```text
                           .-----. .-----.
  VCC--[RP]-------CLK---> 1|IN0  '-'  VCC|20 <---VCC
  VCC--[RP]--------IO---> 2|IN1 16L8 OUT7|19 --->IN0    (out-only)
  VCC--[RP]-----IN0'A---> 3|IN2 PAL  I/O6|18 --->IN1
  VCC--[RP]-----IN1'A---> 4|IN3      I/O5|17 --->STB'A
  VCC--[RP]---/MODE6P---> 5|IN4      I/O4|16 --->IO'A
  VCC--[RP]-----IN0'B---> 6|IN5      I/O3|15 --->CLK'B
  VCC--[RP]---/MODE2P---> 7|IN6      I/O2|14 --->CLK'CDE
  VCC--[RP]-----IN0'D---> 8|IN7      I/O1|13 --->STB'BCDE
  VCC--[RP]-----IN0'C---> 9|IN8      OUT0|12 <---IN0'E  (out-only???)
                  GND -->10|GND       IN9|11 <---/STB----[R]---VCC
                           '-------------'
       .----------.           .-------.             .-------.
       |3-position|           | S9013 |             | S9013 |        .-[R]-STB
       |switch  2P|--/MODE2P  |      E|-->VCC'BCDE  |      E|---GND  |
  GND--|        5P|--NC       | NPN  B|<--/MODE2P   | NPN  B|<-------+
       |        6P|--/MODE6P  |      C|---SNES.VCC  |      C|-->/STB |
       '----------'           '-------'             '-------'        '-[R]-GND
  .------------------------------------------------------.
  |   (Female)(.............Male........................)|    ???--|LED>--???
  |Pin SNES   PORT2   PORT3    PORT4    PORT5    PORT6   |
  |1   VCC    VCC     VCC'BCDE VCC'BCDE VCC'BCDE VCC'BCDE|    further resistors
  |2   CLK    CLK     CLK'B    CLK'CDE  CLK'CDE  CLK'CDE |    ???
  |3   STB    STB'A   STB'BCDE STB'BCDE STB'BCDE STB'BCDE|
  |4   IN0    IN0'A   IN0'B    IN0'C    IN0'D    IN0'E   |    (not installed)
  |5   IN1    (IN1'A) -        -        -        -       |    diodes ???
  |6   IO     (IO'A)  -        -        -        -       |
  |7   GND    GND     GND      GND      GND      GND     |
  '------------------------------------------------------'
     Note: The PCB has wires to all 7 pins of PORT2,
     but the installed connector has only 5 pins, so,
     in practice, IN1'A and IO'A are not connected.
```
