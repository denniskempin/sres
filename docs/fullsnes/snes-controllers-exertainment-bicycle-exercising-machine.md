# SNES Controllers Exertainment (bicycle exercising machine)

The Exertainment is an exercising machine made by Life Fitness. It consists of a stationary bicycle, a monitor with TV tuner, a SNES game cartridge, and a SNES console with some extra hardware plugged into its Expansion Port.

#### Technical Info

> **See:** [SNES Controllers Exertainment - I/O Ports](snes-controllers-exertainment-i-o-ports.md)
> **See:** [SNES Controllers Exertainment - RS232 Controller](snes-controllers-exertainment-rs232-controller.md)
> **See:** [SNES Controllers Exertainment - RS232 Data Packets & Configuration](snes-controllers-exertainment-rs232-data-packets-configuration.md)
> **See:** [SNES Controllers Exertainment - RS232 Data Packets Login Phase](snes-controllers-exertainment-rs232-data-packets-login-phase.md)
> **See:** [SNES Controllers Exertainment - RS232 Data Packets Biking Phase](snes-controllers-exertainment-rs232-data-packets-biking-phase.md)

#### Drawings

> **See:** [SNES Controllers Exertainment - Drawings](snes-controllers-exertainment-drawings.md)

#### Supported Games

```text
  Cannondale Cup (1993) CEG/American Softworks/RadicalEntertainment (US)
  Exertainment Mountain Bike Rally (1994) LifeFitness/RadicalEntertainment (US)
  Exertainment Mountain Bike Rally & Speed Racer (combo cart) (1995) (USA)
  Exertainment Mountain Bike Rally & Speed Racer (combo cart) (prototype) (EU)
```

Aside from the games, all three Exertainment cartridges are including a "Program Manager", allowing to view/edit user profiles, and, in the old cart from 1994 only - also including some "mini games" called Workout and Fit Test).

Cannondale Cup is essentially same as Mountain Bike Rally, it is sending Exertainment packets (including for checking for the "LIFEFITNES(s)" ID), but it lacks the Program Manager, and even the actual game doesn't seem to react to actions on the exertainment hardware(?).

Playing normal SNES games during exercising isn't supported (the SNES cartridge slot isn't externally accessible, and, selecting the pedal resistance requires special program code in the game cartridge).

The Mountain Bike game works with/without the Exertainment hardware (with the Exertainment features being shown only if the hardware is present).

#### Joypad Controls

Joypad like controls are attached to the handlebars, featuring the same 12bit button/direction signals as normal joypads. In fact, there should be two such joypads, both mapped as "player 1" input (ie. both wired to Port 4218h).

Turning the handlebars isn't possible, instead, steering is done via DPAD Left/Right buttons. Whereas, for the Mountain Bike game, steering is needed only for gaining optional bonus points.

#### Other Controls

Pedaling speed/force info is probably sent via Port 21C0h Data Packets. The front panel has six buttons - unknown if the buttons states are sent to the SNES - Volume & Program Up/Down and Picture-in-picture are possibly wired directly to the TV set unit, the Menu button is possibly wired to SNES Reset signal(?)

Exertainment Expansion Port Unit - PCB Component List info from byuu

```text
  U1 40pin TL16C550AN CF62055 N9304 2342265 TI  ;-RS232 controller
  U2 20pin PEEL18CV8P CTM22065 333FB    ;-some PAL (sticker "K41A-12802-0000")
  U3 20pin 74HC374N (addr.msb & serial) ;\two 8bit latches (13bit SRAM address,
  U4 20pin 74HC374N (addr.lsb)          ;/and the 3bit serial-port outputs)
  U5 28pin LH5268A-10LL 9348 SHARP      ;-8Kx8 SRAM
  U6 16pin ADM232LJN 9403 OF31824       ;-RS232 voltage converter
  BATT1    CR2032                       ;-battery (for SRAM)
  P2 4pin  short cable to rear 623K-6P4C;-SIO (1=LED?, 2=CLK?, 3=DTA?, 4=/SEL?)
  P3 3pin  long cable to front 616M-4P4C;-RS232 (1=GND, 2=N/A, 3=TX, 4=RX)
  Px 28pin Connector to SNES expansion port (at bottom of SNES console)
```
