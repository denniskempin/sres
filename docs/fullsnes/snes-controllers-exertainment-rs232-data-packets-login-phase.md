# SNES Controllers Exertainment - RS232 Data Packets Login Phase

#### Login Phase

```text
  Bike Packet 08h Login Part 1 (ID string)
  SNES Packet 09h Login Part 2 (random values)
  Bike Packet 0Ah Login Part 3 (reply to random values)
  SNES Packet 0Bh Login Part 4 (based on received data)
  Bike Packet 0Ch Login Part 5 (fixed values 00,FF,00,0C,..)
  SNES Packet 0Dh Login Part 6 (login okay)
  (communication phase...)
  SNES Packet 0Fh Logout (login failed, or want new login)
```

Login should be done on power-up. And, the SNES software does occassionally logout and re-login (eg. when starting a new game from within main menu).

#### PPU Status Request

```text
  Bike Packet 09h PPU Status Request (with ignored content)
  SNES Packet 00h PPU Status Response (with "RAD" string)
```

PPU Status can be transferred during Login or Communication Phase, unknown if/when/why the bike is actually doing that (the data is rather useless, except maybe for use as random seed).

#### From Bike Packet 08h Login Part 1 (ID string)

```text
  ATT      Attention Code (133h, with 9th bit aka parity set = packet start)
  00h      Command (LSB=08h, MSB=Unknown/unused)
  01h..0Bh ID String ("LIFEFITNESS" or "LIFEFITNESs") ;[0Bh].bit5=flag? [1EEEh]
  0Ch..0Dh Unknown/unused
  0Eh      Checksum (00h-[00h..0Dh])
```

#### From SNES Packet 09h Login Part 2 (random values)

```text
  ACK      Acknowledge Code (33h, received packet with good checksum From Bike)
  00h      Command (LSB=09h, MSB=Zero)
  01h..0Ah Random values (RND1,RND2,RND3,RND4,RND5,RND6,RND7,RND8,RND9,RND10)
  0Bh      Checksum (00h-[00h..0Ah])
```

From Bike Packet 0Ah Login Part 3 (reply to random values)

```text
  ATT      Attention Code (133h, with 9th bit aka parity set = packet start)
  00h      Command (LSB=0Ah, MSB=Unknown/unused)
  01h      RND1+RND5                               ;\
  02h      RND2+(RND5*13+9)                        ; RND'values same as in
  03h      RND3+((RND5*13+9)*13+9)                 ; Login Part 2
  04h      RND4+(((RND5*13+9)*13+9)*13+9)          ;
  05h      RND5+((((RND5*13+9)*13+9)*13+9)*13+9)   ;/
  06h..0Ah Unknown/unused <-- these ARE USED for response TO bike
  0Bh..0Dh Unknown/unused <-- these seem to be totally unused
  0Eh      Checksum (00h-[00h..0Dh])
```

From SNES Packet 0Bh Login Part 4 (based on received data)

```text
  ACK      Acknowledge Code (33h, received packet with good checksum From Bike)
  00h      Command (LSB=0Bh, MSB=Zero)
  01h..0Ah Values "[01h]+[01h..0Ah]" from Login Part 3
  0Bh      Checksum (00h-[00h..0Ah])
```

From Bike Packet 0Ch Login Part 5 (fixed values 00,FF,00,0C,..)

```text
  ATT      Attention Code (133h, with 9th bit aka parity set = packet start)
  00h      Command (LSB=0Ch, MSB=Unknown/unused)
  01h..0Ah Constants (00h,FFh,00h,0Ch,0Ah,63h,00h,FAh,32h,C8h)
  0Bh..0Dh Unknown/unused
  0Eh      Checksum (00h-[00h..0Dh])
```

#### From SNES Packet 0Dh Login Part 6 (login okay)

```text
  ACK      Acknowledge Code (33h, received packet with good checksum From Bike)
  00h      Command (LSB=0Dh, MSB=Zero)
  01h..0Ah All zero (00)
  0Bh      Checksum (00h-[00h..0Ah])
```

From SNES Packet 0Fh Logout (login failed, or want new login)

```text
  ACK      Acknowledge Code (33h, received packet with good checksum From Bike)
  00h      Command (LSB=0Fh, MSB=Zero)
  01h..0Ah All zero (00)
  0Bh      Checksum (00h-[00h..0Ah])
```

This is sent upon login mismatch, and also if the game wants to re-enter the login phase (as done after leaving the main menu).

#### From Bike Packet 09h PPU Status Request (with ignored content)

```text
  ATT      Attention Code (133h, with 9th bit aka parity set = packet start)
  00h      Command (LSB=09h, MSB=Unknown/unused)
  01h..0Dh Unknown/unused
  0Eh      Checksum (00h-[00h..0Dh])
```

#### From SNES Packet 00h PPU Status Response (with "RAD" string)

```text
  ACK      Acknowledge Code (33h, received packet with good checksum From Bike)
  00h      Command (LSB=00h, MSB=Zero)
  01h      PPU2 Status   [213Fh] (PPU2 chip version & Interlace/Lightgun/NTSC)
  02h      PPU1 Status   [213Eh] (PPU1 chip version & OBJ overflow flags)
  03h      CPU  Status   [4210h] (CPU chip version & NMI flag)
  04h      Curr Scanline [213Dh] (lower 8bit of current scanline number)
  05h..0Ah Constants (52h,41h,44h,00h,00h,00h) (aka "RAD",0,0,0)
  0Bh      Checksum (00h-[00h..0Ah])
```

Note: This data is send only after PPU Status Request. There are also cases (during menues) where SNES is sending Packet 00h with zerofilled data body.
