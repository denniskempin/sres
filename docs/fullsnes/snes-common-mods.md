# SNES Common Mods

#### CIC Disable

The console contains a F411 (NTSC) or F413 (PAL) chip that verifies if the cartridge contains an identical chip, if it doesn't, then it resets the SNES, preventing to use unlicensed carts, or to use NTSC carts on PAL consoles.

```text
  F411/F413 Pin 4 (GND=Disable/Unlock, VCC=Enable/Lock)
```

Even when disabled, some newer games (eg. Donkey Kong Country) may verify the PAL/NTSC framerate by software and refuse to run if it doesn't match the expected setting, this can be solved by adding a framerate switch (see below), the verification is often done only after power-up, so one can restore the desired setting after power-up.

Some newer games are reportedly also refusing to run if the CIC chip in the console is disabled, as a workaround, one would usually add a switch that allows to re-enables the CIC when needed. Eventually one could also modify the cartridges (they are probably connecting the CIC /RESET output to ROM CE2 pin or so?).

Games with SA-1 or S-DD1 chips won't work.

#### 50Hz/60Hz Switch

```text
  PPU1 Pin 24 (GND=60Hz, VCC=50Hz)
  PPU2 Pin 30 (GND=60Hz, VCC=50Hz)
```

50Hz/60Hz Switch on newer cost-down SNES (those with 160pin S-CPUN A) Basically, the frame rate is selected by a single pin:

```text
  S-CPUN A, Pin 111 - PAL/NTSC  (high=PAL, low=NTSC)
```

An unwanted side effect is that this pin also changes the expected clock input:

```text
  X1 oscillator (21.47727MHz=NTSC, 17.7344750MHz=PAL)
```

as a workaround, buy the missing oscillator, and use a "stereo" switch that simultaneously toggles the oscillator and the PAL/NTSC pin.

Another unwanted side effect is that it does (probably) change the color clock output for the S-RGB A chip, making the composite video signal unusable. As a workaround, one could use a TV set with RGB input (this would also require to connect the R,G,B,SYNC pins, which are left unconnected on the Multi-Out port of the cost-down SNES). Eventually it might be also possible to use composite video by connecting a matching oscillator directly to the S-RGB A chip (NTSC:3.579545MHz, PAL:4.43361875MHz) (not tested).
