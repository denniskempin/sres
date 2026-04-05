# SNES Cart Super Gameboy

The Super Gameboy (SGB) is some kind of an adaptor for monochrome handheld Gameboy games. The SGB cartridge contains a fully featured gameboy (with CPU, Video & Audio controllers), but without LCD screen and without joypad buttons.

The 4-grayshade 160x144 pixel video signal is forwarded to SNES VRAM and shown on TV Set, and in the other direction, the SNES joypad data is forwarded to SGB CPU.

Some gameboy games include additional SGB features, allowing to display a 256x224 pixel border that surrounds the 160x144 pixel screen, there are also some (rather limited) functions for colorizing the monochrome screen, plus some special Sound, OBJ, Joypad functions. Finally, the gameboy game can upload program code to the SNES and execute it.

#### Chipset

```text
  SGB CPU - 80pin - Super Gameboy CPU/Video/Audio Chip
  ICD2-R (or ICD2-N) - 44pin - Super Gameboy SGB-to-SNES Interface Chip
```

Plus VRAM/WRAM for SGB CPU, plus SNES SGB BIOS, plus CIC chip.

#### SGB I/O Map (ICD2-R)

```text
  6000       R  LCD Character Row and Buffer Write-Row
  6001       W  Character Buffer Read Row Select
  6002       R  16-Byte Packet Available Flag
  6003       W  Reset/Multiplayer/Speed Control
  6004-6007  W  Controller Data for Player 1-4
  6008-600E  -  Unused (Open Bus, or mirror of 600Fh on some chips)
  600F       R  Chip Version (21h or 61h)
  6800-680F  -  Unused (Open Bus)
  7000-700F  R  16-byte command packet (addr 7000..700F)
  7800       R  Character Buffer Data (320 bytes of currently selected row)
  7801-780F  R  Unused (Mirrors of 7800h, not Open Bus)
```

The ICD2 chips decodes only A0-A3,A11-A15,A22 (so above is mirrored to various addresses at xx6xxN/xx7xxN). Reading the Unused registers (and write-only ones) returns garbage. On chips with [600Fh]=61h, that garbage is:

```text
  CPU Open Bus values (though, for some reason, usually with bit3=1).
```

On chips with [600Fh]=21h, that garbage is:

```text
  6001h.R, 6004h-6005h.R --> mirror of 6000h.R
  6003h.R, 6006h-6007h.R --> mirror of 6002h.R
  6008h-600Eh.R          --> mirror of 600Fh.R
```

On ICD2-N chips and/or such with [600Fh]=other, that garbage is: Unknown.

SGB Port 6000h - LCD Character Row and Buffer Write-Row (R)

```text
  7-3  Current Character Row on Gameboy LCD (0..11h) (11h=Last Row, or Vblank)
  2    Seems to be always zero
  1-0  Current Character Row WRITE Buffer Number (0..3)
```

#### SGB Port 6001h - Character Buffer Read Row Select (W)

```text
  7-2  Unknown/unused      (should be zero)
  1-0  Select Character Row READ Buffer Number (0..3)
```

Selects one of the four buffer rows (for reading via Port 7800h). Only the three "old" buffers should be selected, ie. not the currently written row (which is indicated in 6000h.Bit1-0).

#### SGB Port 6002h - 16-Byte Packet Available Flag (R)

```text
  7-1  Seems to be always zero
  0    New 16-byte Packet Available (0=None, 1=Yes)
```

When set, a 16-byte SGB command packet can be read from 7000h-700Fh; of which, reading 7000h does reset the flag in 6002h.

#### SGB Port 6003h - Reset/Multiplayer/Speed Control (W)

```text
  7    Reset Gameboy CPU   (0=Reset, 1=Normal)
  6    Unknown/unused      (should be zero)
  5-4  num_controllers     (0,1,3=One,Two,Four)  (default 0=One Player)
  3-2  Unknown/unused      (should be zero)
  1-0  SGB CPU Speed       (0..3 = 5MHz,4MHz,3MHz,2.3MHz) (default 1=4MHz)
```

The LSBs select the SGB CPU Speed (the SNES 21MHz master clock divided by 4,5,7,9). Unknown if/how/when the SGB BIOS does use this. For the SGB, the exact master clock depends on the console (PAL or NTSC). For the SGB2 it's derived from a separate 20.9MHz oscillator.

#### SGB Port 6004h-6007h - Controller Data for Player 1-4 (W)

```text
  7    Start     (0=Pressed, 1=Released)
  6    Select    (0=Pressed, 1=Released)
  5    Button B  (0=Pressed, 1=Released)
  4    Button A  (0=Pressed, 1=Released)
  3    Down      (0=Pressed, 1=Released)
  2    Up        (0=Pressed, 1=Released)
  1    Left      (0=Pressed, 1=Released)
  0    Right     (0=Pressed, 1=Released)
```

Used to forward SNES controller data to the gameboy Joypad inputs. Ports 6005h-6007h are used only in 2-4 player mode (which can be activated via 6003h; in practice: this can be requested by SGB games via MLT_REQ (command 11h), see SGB section in Pan Docs for details).

#### SGB Port 600Fh - Chip Version (R)

```text
  7-0  ICD2 Chip Version
```

Seems to indicate the ICD2 Chip Version. Known values/versions are:

```text
  21h = ICD2-R (without company logo on chip package)
  61h = ICD2-R (with company logo on chip package)
  ??  = ICD2-N (this one is used in SGB2)
```

The versions differ on reading unused/write-only ports (see notes in SGB I/O map).

#### SGB Port 7000h-700Fh - 16-byte Command Packet (R)

```text
  7-0  Data
```

Reading from 7000h (but not from 7001h-700Fh) does reset the flag in 6002h Aside from regular SGB commands, the SGB BIOS (that in the SGB CPU chip) does transfer six special packets upon Reset; these do contain gameboy cartridge header bytes 104h..14Fh (ie. Nintendo Logo, Title, ROM/RAM Size, SGB-Enable bytes, etc).

#### SGB Port 7800h - Character Buffer Data (R)

```text
  7-0  Data (320 bytes; from Buffer Row number selected in Port 6001h)
```

This port should be used as fixed DMA source address for transferring 320 bytes (one 160x8 pixel character row) to WRAM (and, once when the SNES is in Vblank, the whole 160x144 pixels can be DMAed from WRAM to VRAM).

The ICD2 chip does automatically re-arrange the pixel color signals (LD0/LD1) back to 8x8 pixel tiles with two bit-planes (ie. to the same format as used in Gameboy and SNES VRAM).

The buffer index (0..511) is reset to 0 upon writing to Port 6001h, and is automatically incremented on reading 7800h. When reading more than 320 bytes, indices 320..511 return FFh bytes (black pixels), and, after 512 bytes, it wraps to index 0 within the same buffer row.

#### Gameboy Audio

The stereo Gameboy Audio Output is fed to the External Audio Input on SNES cartridge port, so sound is automatically forwarded to the TV Set, ie. software doesn't need to process sound data (however, mind that the /MUTE signal of the SNES APU must be released).

#### SGB Commands

Above describes only the SNES side of the Super Gameboy. For the Gameboy side (ie. for info on sending SGB packets, etc), see SGB section in Pan Docs:

```text
  http://problemkaputt.de/pandocs.htm
  http://problemkaputt.de/pandocs.txt
```

Some details that aren't described in (current) Pan Docs:

* JUMP does always destroy the NMI vector (even if it's 000000h)  * (The SGB BIOS doesn't seem to use NMIs, so destroying it doesn't harm)  * JUMP can return via 16bit retadr (but needs to force program bank 00h)  * After JUMP, all RAM can be used, except [0000BBh..0000BDh] (=NMI vector)  * The IRQ/COP/BRK vectors/handlers are in ROM, ie. only NMIs can be hooked  * APU Boot-ROM can be executed via MOV [2140h],FEh (but Echo-Write is kept on)  * The TEST_EN command points to a RET opcode (ie. it isn't implemented)  * Upon RESET, six packets with gameboy cart header are sent by gameboy bios  * command 19h does allow to change an undoc flag (maybe palette related?)  * command 1Ah..1Fh point to RET (no function) (except 1Eh = boot info)  * sgb cpu speed can be changed (unknown if/how supported by sgb bios)

#### Note

There is a special controller, the SGB Commander (from Hori), which does reportedly have special buttons for changing the CPU speed - unknown how it is doing that (ie. unknown what data and/or ID bits it is transferring to the SNES controller port).

Probably done by sending button sequences (works also with normal joypad):

Codes for Super GameBoy Hardware  Enter these codes very quickly for the desired effect.

```text
  After choosing a border from 4 - 10, press L + R to exit.
   Press L, L, L, L, R, L, L, L, L, R. - Screen Savers
  At the Super Game Boy,
   press L, L, L, R, R, R, L, L, L, R, R, R, R, R, R, R - Super Gameboy Credits
  Hold UP as you turn on the SNES and then press L, R, R, L, L, R - Toggle
```

#### Speed

```text
  During a game, press L, R, R, L, L, R - Toggle Speed
  During a game, press R, L, L, R, R, L - Toggle Sound
  --
```

Screen Savers --> Choose a border from 4 to 10 and press L + R to exit. Press L(4), R, L(4), R.

Super Gameboy Credits --> When you see the Super Game Boy screen appear, press L, L, L, R, R, R, L, L, L, R, R, R, R, R, R, R Toggle Speed (Fast, Normal, Slow, Very Slow)    Hold Up when powering up the SNES, then press L, R, R, L, L, R very fast.

Toggle Speed (Normal, Slow, Very Slow)  During Gameplay, press L, R, R, L, L, R very fast.

Un/Mute Sound --> During Gameplay, press R, L, L, R, R, L quite fast.
