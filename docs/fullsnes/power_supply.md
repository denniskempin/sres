# SNES Power Supply

#### AC 9V (PAL version)

Power Supply input. The 9V AC input is internally converted to 9V DC, and is then converted to 5V DC by a 7805 IC which turns the additional volts into heat. The CPU/PPU/APU are solely operated at 5V DC (ie. you can feed 5V to the 7805 output). However, 9V DC are used by the Audio Amplifier, the console works without the 9V supply, but you won't hear any sounds.

The Amplifier does work more less okay with 5V supply (ie. you can shortcut the 7805 input and output pins, and feed 5V to both of them) (when doing that disconnect the original 9V input to make sure that 9V aren't accidently passed to the CPU/PPU). However, at 5V, the middle amplitude levels are generated linearily intact, but the MIN/MAX levels are chopped off, for example a sawtooth wave looks like so:

```text
  Amplifier Output at 9V:                 Amplifier Output at 5V:
     /|   /|   /|   /|   /| -8000h +2.5V     _    _    _    _    _
    / |  / |  / |  / |  / |                 / |  / |  / |  / |  / | -3XXXh +1V
   /  | /  | /  | /  | /  |               _/  |_/  |_/  |_/  |_/  | +3XXXh -1V
  /   |/   |/   |/   |/   | +7FFFh -2.5V
```

The effect applies only if a game does use MIN/MAX levels (ie. there'd be no problem at Master Volume of 40h or less; unless additional output comes from Echo Unit).

#### 10V DC 850mA (NTSC version)

Same as above, but using a DC supply (not AC), passed through a diode (so internally the voltage may drop from 10V to around 9V).

#### Power Switch / Anti-Eject

In older SNES consoles the Power switch comes with an anti-eject lever, which goes into a notch in the cartridge. Some carts have notches with square edges; cart cannot be removed while power is on. Other carts have notches with diagonal edge; the lever gets pushed (while sliding along the diagonal edge), and power is switched off automatically when removing the cartridge.

The anti-eject type is probably (?) used by carts with battery-backed SRAM, in order to prevent garbage writes (which might occur when simultaneously ejecting and powering-down).
