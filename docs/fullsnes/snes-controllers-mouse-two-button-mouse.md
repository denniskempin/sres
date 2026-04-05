# SNES Controllers Mouse (Two-button Mouse)

#### Mouse Connection

The mouse can be connected to Controller Port 1 or 2. Default seems to be Port 1 for most games. Exception: Satellaview FLASH games should default to Port 2 (the joypad controlled BS-X BIOS doesn't work with mouse plugged into Port 1).

Mario Paint accepts it ONLY in Port 1, other games may accept either port (maybe there are also some that accept only Port 2?). Two-player games (eg.

Operation Thunderbolt) may accept two mice to be connected. Some games (eg.

Super Bomberman Panic Bomber World) refuse to run if a mouse is connected. The mouse should not be connected to Multiplayer 5 adaptors (which allow only 17mA per controller, whilst the mouse requires 50mA).

#### Supported Games

> **See:** [SNES Controllers Mouse Games](snes-controllers-mouse-games.md)

#### Mouse Bits

```text
  1st..8th     Unused       (always 0=High)
  9th          Right Button (0=High=Released, 1=Low=Pressed)
  10th         Left Button  (0=High=Released, 1=Low=Pressed)
  11th         Sensitivity Bit1   (0=High=Zero)     ;\0=slow, 1=normal, 2=fast
  12th         Sensitivity Bit0   (0=High=Zero)     ;/
  13th         ID Bit3      (always 0=High)
  14th         ID Bit2      (always 0=High)
  15th         ID Bit1      (always 0=High)
  16th         ID Bit0      (always 1=Low)
  17th         Vertical Direction     (0=High=Down, 1=Low=Up)
  18th         Vertical Offset Bit6   (0=High=Zero)    ;\
  19th         Vertical Offset Bit5   (0=High=Zero)    ;
  20th         Vertical Offset Bit4   (0=High=Zero)    ; this is a 7bit
  21th         Vertical Offset Bit3   (0=High=Zero)    ; UNSIGNED value
  22th         Vertical Offset Bit2   (0=High=Zero)    ; (00h=No motion)
  23th         Vertical Offset Bit1   (0=High=Zero)    ;
  24th         Vertical Offset Bit0   (0=High=Zero)    ;/
  25th         Horizontal Direction   (0=High=Right, 1=Low=Left)
  26th         Horizontal Offset Bit6 (0=High=Zero)    ;\
  27th         Horizontal Offset Bit5 (0=High=Zero)    ;
  28th         Horizontal Offset Bit4 (0=High=Zero)    ; this is a 7bit
  29th         Horizontal Offset Bit3 (0=High=Zero)    ; UNSIGNED value
  30th         Horizontal Offset Bit2 (0=High=Zero)    ; (00h=No motion)
  31th         Horizontal Offset Bit1 (0=High=Zero)    ;
  32th         Horizontal Offset Bit0 (0=High=Zero)    ;/
  33th and up  Padding      (always 1=Low)
```

Note that the motion values consist of a Direction Bit and an UNSIGNED 7bit offset (ie. not a signed 8bit value). After reading, the 7bit offsets are automatically reset to zero (whilst the direction bits do reportedly stay unchanged unless/until the mouse is moved in opposite direction).

#### Mouse Support ID-String

Games that support the mouse should contain the string "START OF MOUSE BIOS" somewhere in the ROM-image.

#### Mouse Sensitivity

The Mouse Resolution is specified as "50 counts/inch (+/-10%)". There are three selectable Sensitivity (Threshold) settings:

```text
  0 - slow   - linear fixed level (1:1)
  1 - normal - exponential -?- levels (1:1 to ?:1)  (?:1=smaller than 6:1)
  2 - fast   - exponential six levels (1:1 to 6:1)
```

Setting 0 returns raw mickeys (so one must implement effects like double-speed threshold by software). Settings 1-2 can be used directly as screen-pixel offsets. To change the sensitivity (for port n=0 or n=1):

```text
  [4016h]=01h           ;set STB=1
  dummy=[4016h+n]       ;issue CLK pulse while STB=1 <-- increments the value,
  [4016h]=00h           ;set STB=0                       or wraps from 2 to 0
  ;Thereafter, one should read the Sensitivity bits, typically like so:
  [4016h]=01h           ;set STB=1  ;\another STB on/off, for invoking reading
  [4016h]=00h           ;set STB=0  ;/(not sure if this part is required)
  for i=11 to 0, dummy=[4016h+n], next i              ;skip first 12 bits
  for i=1 to 0, sensitivity.bit(i)=[4016h+n], next i  ;read 2 sensitivity bits
  ;Repeat the above procedure until the desired sensitivity value is reached.
```

Caution: According to Nintendo, the internal threshold factors aren't initialized until the change-sensitivty procedure is executed at least once (ie. after power-up, or after sensing a newly connected mouse, one MUST execute the change-sensitivity procedure, EVEN if the mouse does return the desired 2bit sensitivity code).
