# SNES Cart X-Band Misc

Info...

It was used for networked gaming via phone lines.

The Xband worked by sending controller instructions, by intercepting code from the game, and patching it with its own instructions, much like the Game Genie works. (that are, probably, two separate features messed into one sentence?) The system worked by dialing up the main server, which was located in Cupertino, California (USA), and somewhere else (Japan). The server then sent the Xband newsletters (called Bandwidth and Xband News). It also sent any patches that were needed. You could then search for opponents.

#### Unknown Features

There seems to be no CIC chip, so the BIOS does likewise work only with another SNES cart connected.

There is switch, for whatever on/off/mode selection. There are three LEDs for whatever purpose. And, there is some kind of a credit-card (or so) reader.

#### Memory Map

```text
  D00000h-DFFFFFh  1MB ROM (executed here, not at C00000h-CFFFFFh)
  E00000h-E0FFFFh  64K SRAM (in two 32Kx8 chips) (unknown if BOTH have battery)
  FBC000h-FBC17Fh  I/O Ports (unknown functions?)
  FBC180h-FBC1BFh  I/O Ports (Rockwell Modem Chip)
  FBFC02h          I/O Port  (unknown functions?)
  FBFE00h          I/O Port  (unknown functions?)
  FFC000h          I/O Port  (unknown functions?)
  004F02h          I/O Port  (unknown functions?)
  00F000h          Dummy/strobe read?
  00FFE0h          Dummy/strobe read?
```

I/O Ports seem to be 8bit-wide / word-aligned (ie. one can use 8bit or 16bit writes, with the MSB ignored in the latter case). Normally ONLY the even addresses are used (some exceptions are: 8bit write 00h to FBC153h, 16bit write 0000h to FBC160h).

Some of the I/O ports outside of the FBCxxxh region might belong to other hardware? (eg. the X-Band might automatically disable any Game Genie BIOS in order to access the Game ROM).

#### Unknown 100pin Chip

Unknown. Probably controls the cart reader, the cheat/patching feature, and maybe also memory & I/O mapping of the other chips.

#### Games supported by the X-Band modem

```text
  Doom                           +
  Ken Griffey Jr. Baseball       ? (not listed in stats)
  Killer Instinct                +
  Madden NFL '95                 +
  Madden NFL '96                 +
  Mortal Kombat II               +
  Mortal Kombat 3                +
  NBA Jam TE                     +
  NHL '95                        ? (not listed in stats)
  NHL '96                        ? (not listed in stats)
  Super Mario Kart               +
  Weaponlord                     + (listed in sf2dxb stats only)
```

and,

```text
  Kirby's Avalanche              +
  Super Street Fighter II        +
  The Legend of Zelda: A Link to the Past (secret maze game)   +
  Super Mario World (chat function)
```

"First of all, the Legend of Zelda wasn't the only cartridge that would activate the hidden maze game -- basically, any unsupported SNES cart would do it. I usually used Super Mario World." CZroe: "Zelda triggered the XBAND's built-in maze game (someone reported that their copy didn't work... Zelda 1.1?!). Mario World triggered the Chat function." CZroe: "This is how I identified that there was a second version of Killer Instinct long before it debuted on this site (all US Killer Instinct bundle SNES consoles would not work with the XBAND)." gainesvillefrank: "I remember XBAND tried this experimental use of Mario World after a while. If you dialed in to XBAND with Mario World in your SNES then it would treat the cartridge as a chat room." "The black switch on the side needs to be in the down position. Otherwise it passes through." Most of the above games, don't include any built-in Xband support, instead, Catapult reverse-engineered how they work, and patched them to work with the modem. Exceptions are Weaponlord (and Doom?), which were released with "modem support" (unknown what that means exactly... do they control modem I/O ports... interact with the modem BIOS... or are they patched the same way as other games, and the only difference is that the developers created the patches before releasing the game?) Note: The japanese BIOS does read the Game cartridge header several times (unlike the US version which reads it only once), basically there is no good reason for those multiple reads, but it might indicate the japanese version includes multiple patches built-in in ROM?)

CODES/SECRETS (still working, even when offline)

#### Maze mini-game

Press Down(2), Left(2), Right, B at the main menu.

#### Blockade mini-game (tron clone)

Press Up(2), Left, Right, Left(2), Right, L at the main menu.

#### Fish Pong mini-game

Genesis only?

#### Change Font

To change the text font, enter these codes at the Player Select screen.

Green and yellow font - Up, Up, Right, Right, Down, Down, Left Rainbow font - Left, Left, Up, Up, Right, Right, Down Searchlight font - Down, Down, Left, Left, Up, Up, Right

#### Alternate screen

Press Up, Up, Left, Right on the title screen.

#### Screen Saver

Press Left, Right, Down, Down, R at the "X-Mail" and "Newsletters" screens.

#### SNES X-Band SRAM Dumps

```text
  benner  3.26.97 (main character with most stats is lower-right)
  sf2dxb  4.30.97
  luke2   3.1.97
```

contains stats (for played game titles; separately for each of the 4 player accounts), and the most recent bandwidth/newletter magazines, and x-mails.

PCB "123-0002-16, Cyclone Rev 9, Catapult (C) 1995" Component List

```text
  U1   28pin Winbond W24257S-70L                    (32Kx8 SRAM)
  U2   36pin X X, X BAND, X X, SNES US ROM 1.0.1    (BIOS ROM)
  U3  100pin FredIIH, H3A4D1049, 9511 Korea (with Hyundai logo)
  U4   68pin RC2324DPL, R6642-14, Rockwell 91, 9439 A49172-2, Mexico
  U5    6pin LITEON 4N25 (optocoupler) (near TN0) (back side)
  U6   28pin Winbond W24257S-70L                    (32Kx8 SRAM)
  U7    6pin AT&T LF1504 (solid state relay) (near TN0) (back side)
  BT0   2pin Battery (not installed) (component side)
  BT200 2pin Battery (3V Lithium Penata CR2430) (back side)
  SW1   3pin Two-position switch (purpose unknown... battery off ??)
  J0   10pin Card-reader (for credit cards or so?) 8 contacts, plus 2pin switch
  J1   62pin SNES Cartridge Edge (to be plugged into the SNES console)
  J2   62pin SNES Cartridge Slot (for game-cart plugged on top of the modem)
  J3  4/6pin RJ socket (to phone line)
  Y1    2pin Oscillator (R24AKBB4, =24MHz or so?) (back side)
  TN0   4pin Transformator (671-8001 MIDCOM C439)
  LEDs       Three red LEDs (purpose/usage unknown?)
```

PCB "123-0002-17, Catapult (C) 1995" Component List

```text
  MODEM is  "RC2424DPL, R6642-25, Rockwell 91, 9507 A61877.2, Hong Kong"
```

PCB "123-0003-04, Tornado, Catapult (C) 1995" (Japan)

```text
  SRAMs are "SEC KOREA, 550A, KM62256CLG-7L"
  BIOS  is  "X X 9549, X BAND, X X, SUPER FAMICOM, ROM1.0"
  FRED  is  "Catapult, FRED5S, 549D" (100pin)
  MODEM is  "RC2424DPL, R6642-25, Rockwell 91, 9609 A62975-2, Mexico"
  Y1    is  "A24.000"
  BT201 is  "C?2032" (installed instead of bigger BT200)
```
