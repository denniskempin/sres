# SNES Cart Satellaview Town Status Packet

#### Town Status (Software Channel 1.1.0.5)

Loaded to 7EA31Dh, then copied to 7EA21Dh. Size (max) 256 bytes.

Uses a normal 10-byte fragment header, but with "4bit Transmission ID" and "7bit Fragment Number" ignored, but still does use "Target Offset" for fragment(s)?

```text
  00h   1   Flags (bit0=1=Invalid) (bit1-7=unknown/unused)
  01h   1   Town Status ID (packet must be processed only if this ID changes)
  02h   1   Directory ID   (compared to Directory ID in Directory packet)
  03h   4   Unknown/unused
  07h   1   APU Sound Effects/Music & BSX Receiver Power-Down
             Bit0-3 Unknown/unused
             Bit4-5 APU (0=Mute, 1=Effects, 2=Effects/MusicA, 3=Effects/MusicB)
             Bit6   BSX (0=Normal, 1=Power-down with Port 2199h Reg[0]=88h)
             Bit7   BSX (0=Normal, 1=Power-down with Port 2199h Reg[0]=00h)
             (Or, maybe, the "Power-down" stuff enables satellite radio,
             being injected to audio-inputs on expansion port...?)
  08h   1   Unknown/unused
  09h   8   People Present Flags (Bit0-63) (max 5)        (LITTLE-ENDIAN)
  11h   2   Fountain Replacement & Season Flags (Bit0-15) (LITTLE-ENDIAN)
  13h   4   Unknown/unused
  17h   1   Number of File IDs (X) (may be 00h=none) (max=E8h)
  18h   X   File IDs (one byte each) (compared against File ID in Directory)
```

This packet should be (re-)downloaded frequently. The File IDs indicate which Directory entries are valid (whereas, it seems to be possible to share the same ID for ALL files), the Directory ID indicates if the Directory itself is still valid.

#### Fountain/Replacement and Season

The animated Fountain (near beach stairs) has only decorative purposes (unlike buildings/people that can contain folders). Optionally, the fountain can be replaced by other (non-animated) decorative elements via Fountain Replacement Flags in the Town Status packet: Bit0-11 are selecting element 1-12 (if more than one bit is set, then the lowest bit is used) (if no bits are set, then the fountain is used).

```text
  None  00h Default Fountain (default when no bits set) (animated)
  Bit0  01h Jan     Altar with Apple or so
  Bit1  02h Feb     Red Roses arranged as a Heart
  Bit2  03h Mar     Mohican-Samurai & Batman-Joker
  Bit3  04h Apr     Pink Tree
  Bit4  05h May     Origami with Fish-flag
  Bit5  06h Jun     Decorative Mushrooms
  Bit6  07h Jul     Palmtree Christmas with Plastic-Blowjob-Ghost?
  Bit7  08h Aug     Melons & Sunshade
  Bit8  09h Sep     White Rabbit with Joss Sticks & Billiard Balls
  Bit9  0Ah Oct     National Boule-Basketball
  Bit10 0Bh Nov     Red Hemp Leaf (cannabis/autum)
  Bit11 0Ch Dec     Christmas Tree (with special Gimmick upon accessing it)
```

As shown above, the 12 replacements are eventually associated with months (Christmas in December makes sense... and Fish in May dunno why).

Bit12-15 of the above flags are selecting Season:

```text
  None  00h Default (when no bits set)
  Bit12 01h Spring  (pale green grass) (seems to be same as default)
  Bit13 02h Summery (poppy colors with high contrast)
  Bit14 03h Autumn  (yellow/brown grass)
  Bit15 04h Winter  (snow covered)
```

As for the Fountain, default is used when no bits set, and lowest bit is used if more than one bit is set.
