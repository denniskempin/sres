# SNES Controllers Tilt/Motion Sensors

There are a few SNES controllers with Tilt/Motion Sensors, most or all of them are emulating normal SNES joypad button/direction signals, which is making them compatible with existing games, but also means that the SNES receives only digital data (pressed/released) rather than anlogue (slow/fast) data.

Alltogether, the controllers appear to be more uncomfortable than useful.

#### BatterUP (1994) (Sports Sciences Inc.)

The BatterUP is a 24-inch foam-covered plastic baseball bat for Sega Genesis and Super Nintendo. Reportedly, it "doesn't sense swing speed or location, only timing" (whatever that means, probably simulating a button-pressed signal at the time when swinging the bat). Aside from the swing/motion sensor, the middle of the bat contains joypad buttons.

```text
    ___
   /   \     BatterUP       _________________
  |     |                  |             []  |  Some versions (probably for
  |     |                  |        [] START |  SEGA or so) have a "C" button,
  |     | <-- blue foam   .|        UP       |  and no X/Y/SELECT buttons.
  |     |               .' |  []         []  |
  |     |             .'   | LEFT      RIGHT |  Purpose of the 4 DIP switches
  |_____|   .........'     | ::::   []       |  is unknown (maybe sensitivity,
  |  .' |                  | DIPs  DOWN  []  |  or assigning the swing-sensor
  | '.' |     DPAD         |           SELECT|  to a specific joypad-button?)
  |  .' | <-- buttons -->  |       A[]       |
  |  :  |                  |                 |
   \_'_/    .........      |       B[]       |
   |   |             '.     \               /
   |   |               '.    \     X[]     /
   |   |  <-- handle     '.   \           /
   |   |                       \   Y[]   /
   |   |                        \_______/
   |   |
    \_/
     '.______ cable (to console's controller port)
```

Games compatible with the SNES version (according to instruction manual?):

```text
  Cal Ripken Jr. Baseball, 1992 Mindscape
  ESPN Baseball Tonight, 1994 Sony Imagesoft
  Hardball III, 1994 Accolade
  Ken Griffey Jr. Presents Major League Baseball, 1994 Nintendo
  Ken Griffey, Jr.'s Winning Run, 1996 Nintendo
  MLBPA Baseball, 1994 EA Sports
  Sports Illustrated Championship Football and Baseball, 1993 Malibu Games
  Super Baseball, 1994 EA Techmo
  Super Batter Up, 1993 Namco
```

#### TeeV Golf (1993/1995) (Sports Sciences Inc.)

The TeeV Golf hardware consists of a wireless (battery-powered) golf club, and a rectangular box which is supposed to be set on the floor. There's a mimmicked (half) golf ball in the middle of the box. According to photos, there are two rows of six "red dots" on the box (these might be nonfunctional decorative elements, or important LEDs/sensors for motion tracking?), some kind of a BIOS cartridge or so (which seems to contain something customized for specific games), and two connection cables (one to the consoles controller port, and one forwarded to a joypad).

```text
       .......................................
                 cables           ___________:___________
                 (to joypad      | O   O   O   O   O   O |__
                 and to          |        .'''''.  TeeV  :  | <-- BIOS
                 console)        |       :   _   : Golf  :  |     cartridge
   Golf Club                     |       :  (_)  :       :__|     or so
   (with 2 AA batteries)         |       :.......:       |        (with "PGA
   ___________                   | O   O :.O   O.: O   O |        Tour Golf"
  |          :\ __               |Mode1  : ''''' :  Mode2|        text on it)
  |          : :  |              |_______:_______:_______|
  |          : :  |                        _____________________
  |          : :__|=======================|_____________________|
  |__________:/
```

According to the box: The TeeV SNES version is compatible with PGA Tour Golf (unknown if that refers to the whole PGA series, and unknown if there are other games supported; other games might possibly require other "BIOS" cartridges).

The PGA Tour Golf BIOS cartridge does probably translate motion data to a specially timed sequence of joypad button pressed/released signals.

There are TeeV versions for SNES, Sega Genesis, and PC. The US Patent number for the TeeV hardware is 4971325 (with the addition of "other patents pending").

#### StuntMaster (VictorMaxx)

Advertised as "3-D Virtual Reality Headset".

"Despite what the box says, the StuntMaster VR is not a 3D display. It contains one extremely grainy low resolution LCD screen in the center of the goggles. If you put it on, it hurts your face. The display singes your retinas with an intensely fuzzy, hard-to-focus-on image. The head tracking mechanism is nothing more than a stick you clip to your shoulder (see picture above) which slides through a loop on the side of the headset. When you turn your head, the StuntMaster detects the stick sliding in the loop and translates this into a left or right button press on a control pad, assuming you've actually hooked it up to the controller port of your SNES or Genesis. Remember the "point-of-view instantly scrolls or rotates with the turn of your head" quote? I'd love to see that happen in Super Mario World. Obviously, it couldn't actually work unless the game were programmed for that functionality in advance. Unless, of course, you're playing Doom and you want to turn left or right by moving your head."

```text
  1  +6V
  2  GND
  3  Joypad (SNES:DTA, SEGA:Right In)
  4  Joypad (SNES:CLK, SEGA:N/A)
  5  Joypad (SNES:STB, SEGA:Left In)
  6  GND
  7  VCC (SNES:+5V, SEGA:N/A?) (for Joypad?)
  8  N/A
  9  GND
  10 Video in (NTSC composite)
  11 Joypad (SNES:DTA, SEGA:Right Out)
  12 Joypad (SNES:STB, SEGA:Left Out)
  13 GND
  14 Audio in (Left)
  15 Audio in (Right)
```

Resolution:     240x86 color triads  Field of View:  17 degrees  Weight:         circa 2.5 pounds

#### Nordic Quest (interactive ski-exerciser) (Nordic Track)

The Nordic Quest is an add-on for treadmills (walking-exercising machines) from Nordic Track. Unlike normal treadmills, the Nordic Track one features two handles attached to a string, which the user pulls back and forth during exercising (similar to nordic walking/skiing sticks).

The Nordic Quest includes replacement handles with DPAD (left handle) and buttons (right handle), allowing to play "virtually any" joypad controlled games during exercising; there aren't any special "nordic" games for the controller, instead it can be used for games like golf, car-racing, and flight simulations (as illustrated on the box).

The exercising intensity is claimed to affect the game speed - unknown how this works - maybe by toggling the DPAD on/off, or maybe by toggling the Start (pause) button on/off?

JS-306 Power Pad Tilt (Champ) (joypad with autofire, slowmotion, tilt-mode) A regular joypad with normal DPAD, the tilt-sensors can be optionally used instead of the DPAD.

Nigal Mouncefill Fly Wheel (Logic 3) (wheel-shaped, tilt-sensor instead dpad) An odd wheel-shaped controller with tilt-sensors instead of DPAD.
