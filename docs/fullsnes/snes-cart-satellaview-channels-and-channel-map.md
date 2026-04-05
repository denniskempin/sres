# SNES Cart Satellaview Channels and Channel Map

#### Channels

Transmission is organized in "channels". Each can Channel transmits a single Packet (which contains a File, or other special information like in Directory and Time packets). There can be (theoretically) up to 4 billion logical "Software Channels", but only 65536 physical "Hardware Channels". The latter ones are those being currently transmitted, and which can be received by programming the channel number into Port 2188h/218Eh.

#### Channel Map Packet (Hardware Channel 0124h)

Unlike normal packets (with 10-byte headers), this packet is preceeded by a 5-byte header.

Loaded to 7E9BECh. Unspecified size (size should be max 1485 bytes or less, otherwise it'd overlap the Welcome Message at 7EA1B9h) (with that size limit, the map can contain max 113 channels) (but, caution: Itoi works only with max 1007 bytes (1024-byte buffer, cropped to N*22-bytes, minus 5-byte packet header)).

```text
  00h 2    ID 53h,46h ("SF")
  02h 4    Unknown/unused
  06h 1    Number of entries (must be at least 1)
  07h 1    Checksum (above 7 bytes at [00..06] added together)
  08h ..   Entries (each one is 3+N*13 bytes)
```

#### Each entry is: (Packet Groups)

```text
  00h 2    Software Channel Number (first 2 bytes, of total 4 bytes)
  02h 1    Number of sub-entries (N) (must be at least 1)
  03h N*13 Sub-entries (each one is 13 bytes)
```

#### Each sub-entry is: (Separate Packets)

```text
  00h 1    Unknown/unused
  01h 2    Software Channel Number (last 2 bytes, of total 4 bytes)
  03h 5    Unknown/unused
  08h 2    Fragment Interval (in seconds) (big-endian) (for use as timeout)
  0Ah 1    Type/Target (lower 4bit indicate transfer method or so)
            Bit0-1: Autostart after Download (0=No, 1=Optional, 2=Yes, 3=Crash)
            Bit2-3: Target (0=WRAM, 1=PSRAM, 2=EntireFLASH, 3=FreeFLASH)
            Bit4-7: Unknown/Unused
  0Bh 2    Hardware Channel Number (2 bytes) (for Port 2188h/218Eh)
```

The transmission timeout for the Channel Map itself is 7 seconds.

#### Hardware Channels (2-byte / 16bit) (XXX or only 14bit?!)

```text
  0121h     Used for hardware-connection test (received data is ignored)
  0124h     Channel Map
  AAEEh     Dummy number (often used to indicate an absent Time Channel)
  NNNNh     Other Hardware Channels (as listed in Channel Map)
  [7FFFF7h] Incoming Time Channel value for some games (from separate loader?)
```

#### Software Channels (4-byte pairs / 32bit)

```text
  1.1.0.4    Welcome Message (100 bytes)
  1.1.0.5    Town Status (256 bytes)
  1.1.0.6    Directory (16Kbytes)
  1.1.0.7    SNES Patch (16Kbytes)
  1.1.0.8    Time Channel (used by BS Satella2 1, BS Fire Emblem, and Itoi)
  1.2.0.48   Time Channel (used by Dragon Quest 1, BS Zelda no Densetsu Remix)
  ?.?.?.?    Time Channel (for BS Zelda - Kodai no Sekiban Dai 3 Hanashi)
  1.2.129.0  Special Channel used by Derby Stallion 96 <-- on roof of building
  1.2.129.16 Special Channel used by Derby Stallion 96 <-- 6th main menu option
  1.2.130.N  Special Channel(s) used by Itoi Shigesato no Bass Tsuri No. 1
  N.N.N.N    Other Software Channels (as listed in Directory)
  N.N.0.0    None (for directory entries that have no File or Include File)
```

#### Endianess of Numbers in Satellite Packets

In the satellite packets, all 16bit/24bit values (such like length or address offsets) are in big-endian format (opposite of the SNES CPUs byte-order). For the Hardware/Software Channel Numbers it's hard to say if they are meant to be big-endian, little-endian, or if they are meant to be simple byte-strings without endianess (see below for their ordering in practice).

#### Endianess of Hardware Channels

The endiness of the Hardware Channel numbers in Channel Map is same as in Port

```text
2188h/218Eh. The endianess of the fixed values (0121h and 0124h) is also same
```

as Port 2188h/218Eh. Ie. one can use that values without needing to swap LSBs and MSBs.

#### Endianess of Software Channels

The fixed 4-byte pairs are using same byte-order as how they are ordered in Channel Map (for example, 1.2.0.48 means that 48 (30h) is at highest address).

So far it's simple. The (slightly) confusing part is that SNES software usually encodes them as 2-word pairs (since the SNES uses a little-endian CPU, the above example values would be 0201h.3000h).
