# SNES Controllers SuperScope (Lightgun)

#### SUPER SCOPE

```text
            Front Sight              Sight Tube            Receiver (connect to
             /                 _______ /               ___ / controller slot 2)
            =--               |__.-.__|(       _______|___|_______
             \ \_______________/  __/         |  _______________  |
             /___.___________.___/            | |               | |
                 :           :                | |               | |
                 : Slot 1    : Slot 2         | |    TV SET     | |
                 :           :                | |               | |
                 :           :                | |               | |
                 V           V                | |_______________| |
                                              |___________________|
                                               |__|_|_|_|_|_|_|__|
       Transmitter
           /            Release            Pause
          /     Slot 1     / Slot 2  Fire   / Power Switch (Off/On/Turbo)
       _____      /       /   /        /   /  /
      \\____\____._______,___._______,,__,,_.._________
     ||             ^               \\\\\/////         \
     ||                                          _____/
     ||___      ________________________        /
          \     \\ <-Cursor           _/      /
           \     \                 __/      /
            \     \               \       /______
             \     \               \    _________\
              \_____\              /___/           <-- Shoulder Rest
```

#### Batteries

Takes six "AA" batteries. Which do reportedly last only for a few hours.

#### Super Scope Bits

```text
  1st          Fire Button   (0=High=Released, 1=Low=Pressed/Newly Pressed)
  2nd          Cursor Button (0=High=Released, 1=Low=Pressed)
  3rd          Turbo Switch  (0=Normal/PowerOn, 1=Turbo/PowerOn)
  4th          Pause Button  (0=High=Released, 1=Low=Newly Pressed)
  5th..6th     Extra ID Bits (always 0=High)
  7th          Offscreen     (0=High=Okay, 1=Low=CRT-Transmission Error)
  8th          Noise         (0=High=Okay, 1=Low=IR-Transmission Error)
  9th..12th    Extra ID Bits (always 1=Low)
  13th..16th   ID Bits       (always 1=Low=One) (0Fh)
  17th and up  Unused        (always 1=Low)
```

For obtaining the H/V position & latch flag (Ports 213Ch,213Dh,213Fh), see:

> **See:** [SNES PPU Timers and Status](snes-ppu-timers-and-status.md)

#### Games compatible with the Super Scope

```text
  Battle Clash (US) (EU) / Space Bazooka (JP) (1992) Nintendo
  Bazooka Blitzkrieg (US) (1992) Bandai
  Hunt for Red October (used for bonus games) (US) (EU) (JP)
  Lamborghini American Challenge (used in special game mode) (US) (EU) (1993)
  Lemmings 2 (US) (EU) (JP) (1994) Psygnosis (at game start: aim at crosshair)
  Metal Combat: Falcon's Revenge (includes OBC1 chip) (US) (1993)
  Operation Thunderbolt (US) (1994) Taito
  Super Scope 6 (bundled with the hardware) (Blastris & LazerBlazer) (1992)
  Terminator 2 - T2: The Arcade Game (US) (EU) (JP) (1993) Carolco/LJN
  Tin Star (US) (1994) Nintendo
  X-Zone (US) (EU) (1992) Kemco
  Yoshi's Safari (US) (EU) (JP) (1993) Nintendo
```

Moreover, the "SNES Test Program" (1991 by Nintendo) includes a Super Scope test, and, reportedly, there's also a special Super Scope test cartridge (?)

#### Notes

The SuperScope has two modes of operation: normal mode and turbo mode. The current mode is controlled by a switch on the unit, and is indicated by the 3rd bit. Note however that the 3rd bit is only updated when the Fire button is pressed (ie. the 1st bit is set). Thus, when you turn turbo on the 3rd bit remains clear until you shoot, and similarly when turbo is deactivated the bit remains set until you fire.

In either mode, the Pause bit will be set for the first strobe after the pause button is pressed, and then will be clear for subsequent strobes until the button is pressed again. However, the pause button is ignored if either cursor or fire are down(?).

In either mode, the Cursor bit will be set while the Cursor button is pressed.

In normal mode, the Fire bit operates like Pause: it is on for only one strobe.

In turbo mode, it remains set as long as the button is held down.

When Fire/Cursor are set, Offscreen will be set if the gun did not latch during the previous strobe and cleared otherwise (Offscreen is not altered when Fire/Cursor are both clear).

If the Fire button is being held when turbo mode is activated, the gun sets the Fire bit and begins latching. If the Fire button is being held when turbo mode is deactivated, the next poll will have Fire clear but the Turbo bit will stay set (because it isn't be updated until pressing fire the next time).

The PPU latch operates as follows: When Fire or Cursor is set, IOBit is set to 0 when the gun sees the TV's electron gun, and left a 1 otherwise. Thus, if the SNES also leaves it one (bit7 of 4201h), the PPU Counters will be latched at that point. This would also imply that bit7 of 4213h will be 0 at the moment the SuperScope sees the electron gun.

Since the gun depends on the latching behaviour of IOBit, it will only function properly when plugged into Port 2. If plugged into Port 1 instead, everything will work except that there will be no way to tell where on the screen the gun is pointing.

When creating graphics for the SuperScope, note that the color red is not detected. For best results, use colors with the blue component over 75% and/or the green component over 50%.

Data2 is presumably not connected, but this is not known for sure.
