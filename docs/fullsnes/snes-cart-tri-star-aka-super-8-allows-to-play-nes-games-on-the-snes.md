# SNES Cart Tri-Star (aka Super 8) (allows to play NES games on the SNES)

The Tri-Star is an adaptor for playing NES games on the SNES (similar to the Super Gameboy which allows to play Gameboy games on SNES). The thing have three cartridge slots (two for western/japanese NES/Famicom cartridges, and one for SNES cartridges).

NES or SNES mode can be selected in BIOS boot menu. SNES mode does simply disable the BIOS and jump to game entrypoint at [FFFCh]. NES mode executes the games via a NOAC (NES-on-a-Chip, a black blob, which is also used in various other NES clones), in this mode, the SNES video signal is disabled, and, aside from the BIOS passing joypad data to the NES, the SNES does merely serve as power-supply for the NES.

#### Memory and I/O Map

```text
  00E000h-00FFFFh.R - BIOS ROM (8Kbytes)
  00FFF0h.W - NES Joypad 1 (8bit data, transferred MSB first, 1=released)
  00FFF1h.W - NES Joypad 2 (bit4-5: might be NES reset and/or whatever?)
  00FFF2h.W - Enter NES Mode (switch to NES video signal or so)
  00FFF3h.W - Disable BIOS and map SNES cartridge
```

#### Joypad I/O

In NES mode, the BIOS is reading SNES joypads once per frame (via automatic reading), and forwards the first 8bit of the SNES joypad data to the NES (accordingly, it will work only with normal joypads, not with special hardware like multitaps or lightguns). Like on japanese Famicoms, there are no Start/Select buttons transferred to joypad 2. Instead, FFF1h.Bit5/4 are set to Bit5=0/Bit4=1 in SNES mode, and to Bit5=1/Bit4=0 in NES mode (purpose is unknown, one of the bits might control NES reset signal, the other might select NES/SNES video signal, unless that part is controlled via FFF3h).

#### Mode Selection I/O

When starting a NES/SNES game, ports FFF2h or FFF3h are triggered by writing twice to them (probably writing any value will work, and possibly writing only once might work, too).

BIOS Versions (and chksum, shown when pressing A+X on power-up)

```text
  Tri-Star (C) 1993                        ;ROM CHKSUM: 187C
  Tri-Star Super 8 by Innovation (C) 1995  ;ROM CHKSUM: F61E
```

Both BIOSes are 8Kbytes in size (although ROM-images are often overdumped). The versions seem to differ only by the changed copyright message. The GUI does resemble that of the X-Terminator and Super UFO (which were probably made by the same anonymous company).

A third version would have been the (unreleased) Superdeck (a similar device that has been announced by Innovation and some other companies).

#### Component List (Board: SFFTP_C/SFFTP_S; component/solder side)

```text
  82pin NOAC chip (black blob on 82pin daughterboard) (on PCB bottom side)
  28pin EPROM 27C64 (8Kx8) (socketed)
  16pin SNES-CIC clone (NTSC: ST10198S) (PAL: probably ST10198P) (socketed)
  20pin sanded-chip (probably 8bit latch for joypad 1)
  20pin sanded-chip (probably 8bit latch for joypad 2)
  16pin sanded-chip (probably 8bit parallel-in shift-register for joypad 1)
  16pin sanded-chip (probably 8bit parallel-in shift-register for joypad 2)
  16pin sanded-chip (probably analog switch for SNES/NES audio or video)
  20pin sanded-chip (probably PAL for address decoding or so) (socketed)
  2pin  oscillator (? MHz) (for NES cpu-clock and/or NES color-clock or so)
  62pin cartridge edge (SNES) (on PCB bottom side)
  12pin cartridge edge (A/V MultiOut) (to TV set) (on PCB rear side)
  62pin cartridge slot (SNES)
  60pin cartridge slot (Famicom) (japanense NES)
  72pin cartridge slot (NES) (non-japanense NES)
  6pin  socket for three shielded wires (Composite & Stereo Audio in from SNES)
  TV Modulator (not installed on all boards)
  four transistors, plus some resistors & capacitors
```
