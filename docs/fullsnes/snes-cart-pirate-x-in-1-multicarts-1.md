# SNES Cart Pirate X-in-1 Multicarts (1)

There are several X-in-1 Multicarts, all containing the same type of text based GUI, and thus probably all made by the same company.

#### Cartridge Header

The first 4 bytes of the title string at FFC0h do usually (or always) contain values 5C,xx,xx,80 (a "JMP FAR 80xxxxh" opcode, which jumps to the GAME entrypoint). The next 4 title bytes are sometimes containing another JMP FAR opcode, the rest of the header is unmodified header of the first game; except that [FFFCh] contains the MENU entrypoint.

#### ROM-images

ROM-images found in the internet are usually incomplete dumps, containing only the first 4MBytes (clipped to the maximum size for normal unmapped LoROM games), or only the first game (clipped to the ROM size entry of the 1st game header). Whilst, the actual multicarts are usually 8MBytes in size (there's one 4Mbyte cartridge, which is actually fully dumped).

#### ROM Size

Most cartridges seem to contain 8Mbyte ROMs. There is one 4MByte cartridge. And there's one cartridge that contains an 8Kbyte EPROM (plus unknown amount of ROM).

#### LoROM/HiROM

Most games seem to be LoROM. Eventually "Donkey Kong Land 3" is HiROM? Unknown if HiROM banks can be also accessed (dumped) in LoROM mode, and if so, unknown how they are ordered; with/without 32Kbyte interleave...?

#### SRAM

According to photos, most or all X-in-1 carts do not contain any SRAM. Though some might do so?

#### DSPn

According to photos, most or all X-in-1 carts do not contain any DSP chips.

Though the "Super 11 in 1" cartridge with "Top Gear 3000" seems to require a DSP4 clone?

#### Port FFFFxxh

```text
  A0-A3 Bank Number bit0-3 (base offset in 256Kbyte units)
  A4    Bank Number bit4 (or always one in "1997 New 7 in 1")
  A5    Always 0         (or Bank bit4 in "1997 New 7 in 1")
  A6    Varies (always 0, or always 1, or HiROM-flag in "Super 7 in 1")
  A7    Always 1 (maybe locks further access to the I/O port)
```

The bank number is somehow merged with the SNES address. As for somehow: This may be ORed, XORed, or even ADDed - in most cases OR/XOR/ADD should give the same result; in case of "1997 New 7 in 1" it looks as if it's XORed(?) The special meaning of A4-A5 can be detected by sensing MOV [FFFFnn],A opcodes (rather than normal MOV [FFFF00+x],A opcodes).

The special meaning of A6 can be detected by checking if the selected bank contains a HiROM-header.

#### Port 6FFFxxh

Unknown. Some games write to both FFFFxxh and 6FFFxxh (using same data & address LSBs for both ports). Maybe the ROM bank address changed to 6FFFxxh on newer boards, and FFFFxxh was kept in there for backwards compatibility. Or maybe 6FFFxxh controls SRAM mapping instead ROM mapping?

#### X-in-1 Cartridges

```text
  Title               FFFFxx      6FFFxx    Size/Notes
  8 in 1 and 10 in 1  C0-DF       N/A       8MB (8 big games + 10 mini games?)
  1997 New 7 in 1     D0-DF,F0-FF N/A       ? MB
  Super 5 in 1        80-9F       80-9F     8MB
  Super 6 in 1        80-8F       N/A       4MB
  Super 7 in 1        80-8F,D0    80-8F,D0  8MB? (mario all stars + 3 games)
  Super 11 in 1       80-9F       N/A       8MB+DSP4 ?
```

Chipset 7-in-1 (Board: SSF-07, REV.1)

```text
  U1 16pin CIVIC CT6911 (CIC clone)
  U2 16pin 74LS13x or so (not legible on photo)
  U3 16pin whatever      (not legible on photo)
  U4 14pin 74LS02 or so  (not legible on photo)
  U5 black blob
  U6 black blob
```

#### Chipset 8-in-1 (Board: MM32-2)

```text
  U  20pin iCT PEEL18CV8P-25
  U  16pin 93C26 A60841.1 9312 (CIC clone)
  U  42pin 56C001 12533A-A 89315
  U  42pin 56C005-4X 12534A-A 89317
```

Chipset 8-in-1 (Board: NES40M, 20045)

```text
  U  16pin CIVIC 74LS13 (CIC clone)
  U  16pin not installed
  U  28pin 27C64Q EPROM (8Kx8)
  U  20pin iCT PEEL18CV8P-25
  U  42pin JM62301
  U  42pin JM62305
```
