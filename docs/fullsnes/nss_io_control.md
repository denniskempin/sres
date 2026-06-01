# NSS I/O Ports - Control Registers

#### Port WHERE.W

Somewhere, following OUTPUT signals should be found:

```text
  SNES Reset Signal (maybe separate CPU/PPU resets, and stop, as on PC10)
  SNES Joypad Disable
  SNES Power Supply Enable (SNES VCC switched via Q1 transistor)
  Maybe support for sending data from Z80 to SNES (eg. to 4016h/4017h/4213h)?
```

#### Port 00h/80h.W - NMI Control and RAM-Protect (IC40/74HC161)

```text
  7-4 Unknown/unused      (should be always 0)
  3     Maybe SNES CPU/PPU reset (usually same as Port 01h.W.Bit1)
  2   RAM at 9000h-9FFFh  (0=Disable/Protect, 1=Enable/Unlock)
  1     Looks like maybe somehow NMI Related ?  ;\or one of these is PC10-style
  0     Looks like NMI Enable                   ;/hardware-watchdog reload?
```

Usually accessed as "Port 80h", sometimes as "Port 00h".

#### Port 01h/81h.W - Unknown and Slot Select (IC39/74HC377)

```text
  7     Maybe SNES Joypad Enable? (0=Disable/Demo, 1=Enable/Game)
  6   Unknown/unused        (should be always 0)
  5   SNES Sound Mute       (0=Normal, 1=Mute) (for optional mute in demo mode)
  4   Player 2 Controls (0=CN4 Connector, 1=Normal/Joypad 2) (INST ROM Flags.0)
  3-2 Slot Select (0..2=1st..3rd Slot, 3=None) (mapping to both SNES and Z80)
  1     Maybe SNES CPU pause?  (cleared on deposit coin to continue) (1=Run)
  0     Maybe SNES CPU/PPU reset?   (0=Reset, 1=Run)
```

Sometimes accessed as "Port 81h", sometimes as "Port 01h".

#### Port 03h/83h.W - Unknown and LED control (IC47/74HC377)

```text
  7     Layer SNES Enable?             (used by token proc, see 7A46h) SNES?
  6     Layer OSD Enable?
  5-4 Unknown/unused (should be always 0)
  3   LED Instructions (0=Off, 1=On)  ;-glows in demo (prompt for INST button)
  2   LED Game 3       (0=Off, 1=On)  ;\
  1   LED Game 2       (0=Off, 1=On)  ; blinked when enough credits inserted
  0   LED Game 1       (0=Off, 1=On)  ;/
```

Usually accessed as "Port 83h", sometimes as "Port 03h".

#### Port 05h.W - Unused/Bug

```text
  7-0 Unknown
```

Accessed only as "Port 05h" (via "outd" opcode executed 5 times; but that seems to be just a bugged attempt to access Port 04h downto 00h).

Port 07h.W - SNES Watchdog: Acknowledge SNES Joypad Read Flag (IC23/74HC109)

```text
  7-0 Unknown/unused (write any dummy value)
```

Accessed only as "Port 07h". Writing any value seems to switch Port 00h.R.Bit7 back to "1". That bit is used for the SNES Watchdog feature; the SNES must read joypads at least once every some frames (the exact limit can be set in INST ROM).

If the watchdog expires more than once, then the game is removed from the cartridge list, and used credits are returned to the user (then allowing to play other games; as long as there are any other games installed).

Note: Judging from hardware tests, there seem to be other ways to acknowledge the flag (probably via Port 07h.R, or maybe even via Port 00h.R itself).

#### NMI

The NMI source is unknown. Maybe Vblank/Vsync, maybe from SNES or OSD, or some other timer signal.

#### Game/Demo-Mode Detection

The original NSS games seem to be unable to detect if a coin is inserted (ie. if they should enter game or demo mode). However, it's possible to do that kind of detection:

Joypad Disable does work much like disconnecting the joypad, so one can check the 17th joypad bit to check if the joypad is connected/enabled (aka if money is inserted). The Magic Floor game is using that trick to switch between game and demo mode (this has been tested by DogP and works on real hardware, ie. the NSS does really disable the whole joypad bitstream, unlike the PC10 which seems to disable only certain buttons).
