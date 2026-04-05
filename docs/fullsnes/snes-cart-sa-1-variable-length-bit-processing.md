# SNES Cart SA-1 Variable-Length Bit Processing

#### 2258h SA-1 VBD - Variable-Length Bit Processing (W)

```text
  0-3  Data Length (1..15=1..15 bits, or 0=16 bits)
  4-6  Not used (should be "..") (whatever ".." means, maybe "0"?)
  7    Data Read Mode (0=Fixed Mode, 1=Auto-increment)
```

Manual/Fixed Mode is used by Jumpin Derby. Auto-increment isn't used by any known games.

2259h SA-1 VDA - Variable-Length Bit Game Pak ROM Start Address Lsb (W) 225Ah SA-1 VDA - Variable-Length Bit Game Pak ROM Start Address Mid (W) 225Bh SA-1 VDA - Variable-Length Bit Game Pak ROM Start Address Msb & Kick

```text
  0-23  Game Pak ROM Address
```

Reading starts on writing to 225Bh.

The ROM address is probably originated at 000000h (rather than using LoROM/HiROM like CPU addresses)?

230Ch SA-1 VDP - Variable-Length Data Read Port Lsb (R) 230Dh SA-1 VDP - Variable-Length Data Read Port Msb (R)

```text
  0-15  Data
```

Unknown what happens on data length less than 16bits:

```text
  Are the selected bits located in MSBs or LSBs?
  Are the other bits set to zero? To next/prev values? Sign-expanded??
```

There is an "auto-increment" feature, which may trigger on reading 230Ch? or on reading or 230Dh?

;*******PRELOAD:

;Preload occurs after writing VDA ;        bitpos = [2259h]*8 ;        [230Ch] = WORD[bitpos/8]

;*******INCREMENT:

;Increment occurs AFTER reading VDP (when auto-increment enabled), ;and after writing VDB (reportedly always, but SHOULD be ONLY when inc=off)?

;        bitpos=bitpos+(([2258h]-1) AND 0Fh)+1 ;        [230Ch] = dword[bitpos/16*2] shr (bitpos and 15) AND FFFFh
