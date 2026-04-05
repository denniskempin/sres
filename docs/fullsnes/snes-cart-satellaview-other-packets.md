# SNES Cart Satellaview Other Packets

SNES Patch Packet (by 105BBC) (Software Channel 1.1.0.7) Loaded to 7FC000h, data portions then copied to specified addresses. Size (max) 16Kbytes.

```text
  00h  1   Number of entries (01h..FFh) (MUST be min 01h)
  01h  ..  Entries (max 3FFFh bytes)
```

Each entry is:

```text
  00h  1   Flags (bit0=1=Invalid) (bit1-7=unknown/unused)
  01h  1   Unknown/unused
  02h  2   Length (N) (16bit, big-endian) (MUST be min 0001h)
  04h  3   SNES-specific Memory Address (24bit, big-endian)
  07h  N   Data (N bytes)
```

The data portions are copied directly to the specified SNES addresses (the BIOS doesn't add any base-offset to the addresses; ie. if the data is to be copied to WRAM, then the satellite would transmit 7E0000h..7FFFFFh as address; there is no address checking, ie. the packet can overwrite stack or I/O ports).

#### Welcome Message Packet (Software Channel 1.1.0.4)

Loaded to 7EA1B9. Size max 64h bytes (100 decimal). Displayed in text window with 37x4 ASCII cells.

```text
  00h  100 Custom Message (max 99 characters, plus ending 00h)
```

If this packet is included in the Channel Map (and if it's successfully received), then the Custom Message is displayed right before entering the town.

The japanese Default Message is still displayed, too (so one gets two messages - which is causing some annoying additional slowdown).

Time Channel Packet (Software channel 1.1.0.8) (BS Fire Emblem) Time Channel Packet (Software channel 1.1.0.8) (BS Satella2 1) Time Channel Packet (Software channel 1.1.0.8) (BS Parlor Parlor 2) Time Channel Packet (Software channel 1.1.0.8) (BS Shin Onigashima 1) Time Channel Packet (Software channel 1.1.0.8) (BS Tantei Club) Time Channel Packet (Software channel 1.1.0.8) (BS Kodomo Tyosadan..) Time Channel Packet (Software channel 1.1.0.8) (BS Super Mario USA 3) Time Channel Packet (Software channel 1.1.0.8) (BS Super Mario Collection 3) Time Channel Packet (Software channel 1.1.0.8) (BS Excitebike - Mario .. 4) Time Channel Packet (Software channel 1.1.0.8) (Itoi Shigesato no Bass Tsuri) Time Channel Packet (Software channel 1.2.0.48) (BS Dragon Quest 1) Time Channel Packet (Software channel 1.2.0.48) (BS Zelda .. Remix) Time Channel Packet (HW channel [7FFFF7h]) (BS Zelda - Kodai .. Dai 3 ..) Time Channel Packet (HW channel [7FFFF7h]) (BS Marvelous Camp Arnold 1) Time Channel Packet (HW channel [7FFFF7h]) (BS Marvelous Time Athletic 4) Preceeded by a 10-byte packet header. Of which, first 5 bytes are ignored (Body is 8-bytes, so packet size should be probably 5+8). Fixed 01h must be 01h.

Number of Fragments must be 01h. Target Offset must be 000000h. The 8-byte Data Body is then:

```text
  00h  1  Unknown/unused (probably NOT seconds) ;(un-)used by Itoi only
  01h  1  Minutes     (0..3Bh for 0..59)
  02h  1  Hours       (0..17h for 0..23) (or maybe 24..26 after midnight)
  03h  1  Day of Week (01h..07h) (rather not 00h..06h) (?=Monday)
  04h  1  Day         (01h..1Fh for 1..31)
  05h  1  Month       (01h..0Ch for 1..12)
  06h  1  Unknown/unused (maybe year)    ;\could be 2x8bit (00:00 to 99:99)
  07h  1  Unknown/unused (maybe century) ;/or maybe 16bit (0..65535) or so
```

Caution: The BS satellite program specified hours from 11..26 (ie. it didn't wrap from 23 to 0), the Time Channel(s) might have been following that notation; if so, then Date values might have also been stuck on the previous day. Unknown if the Time Channels have been online 24 hours a day, or if their broadcast ended at some time late-night.

Time Channel are often used by Soundlink games, in so far, it's also quite possible that the "hours:minutes" are referring to the time within the broadcast (eg. from 0:00 to 0:59 for a 1-hour broadcast duration), rather than to a real-time-clock-style time-of-the-day.

Differences between 1.1.0.8 and 1.2.0.48 are unknown. Some games require incoming Hardware channel number at [7FFFF7h] (from a separate loader or so).

One would expect TIME[0] to contain seconds - however, there aren't any games using that entry as seconds (instead, they wait for minutes to change, and reset seconds to zero; due to that wait-for-next-minute mechanism, many BSX games seem to "hang" for up to 60 seconds after booting them).

TIME[4..7] aren't really used as "date" by any games (a few games seem to be using TIME[4] or TIME[5] to determine how often the user has joined the game).

Itoi uses (and displays) TIME[4..5] as date.

Some games are treating TIME[6..7] as a 16bit little-endian value, others are using only either TIME[6] or TIME[7] (ie. half of the games that use the "year" values are apparently bugged).

#### File Packet (Software Channel N.N.N.N as taken from Directory)

```text
  00h  N   Data, N bytes (N=Filesize as specified in Directory)
```

The file must contain a Satellaview Transmit Header (at file offset 7Fxxh or FFxxh), that header is similar to the FLASH File Header. For details & differences, see:

> **See:** [SNES Cart Satellaview FLASH File Header](snes-cart-satellaview-flash-file-header.md)

The filesize should be usually a multiple of 128Kbytes (because the checksum in File Header is computed accross that size). Transmitting smaller files is possible with some trickery: For FLASH download, assume FLASH to be erased (ie. expand checksum to FFh-filled 128Kbyte boundary). For PSRAM download, the checksum isn't verified (so no problem there). Moreover, filesize should be usually at least 32Kbytes (for LoROM header at 7Fxxh), however, one could manipulate the fragment-offset in packet header (eg. allowing a 4Kbyte file to be loaded to offset 7000h..7FFFh).

Special Channel(s) used by Itoi Shigesato no Bass Tsuri No. 1 (1.2.130.N) Itoi supports four special channel numbers to unlock special contests. The game doesn't try to receive any data on that channels (it only checks if one of them is listed in the Channel Map).

```text
  1.2.130.0      ;\Special Contests 1..4 (or so)
  1.2.130.16     ; The 4 contests are looking more or less the same, possibly
  1.2.130.32     ; with different parameters, different japanese descriptions,
  1.2.130.48     ;/contest 2-3 have "No fishing" regions in some lake-areas.
  1.2.130.other  ;-Invalid (don't use; shows error with TV-style "test screen")
```

For Itoi, the Channel Map may be max 1019 bytes (or even LESS?!) (unlike usually, where it could be 1485 bytes). There should be only one channel with value 1.2.130.N in the Channel Map. The game additionally requires the 1.1.0.8 Time Channel. And, uses the "APU" byte in the Town Status packet.

1.2.129.0  Special Channel used by Derby Stallion 96 (Dish on Building/Roof) 1.2.129.16 Special Channel used by Derby Stallion 96 (6th Main Menu Option) Differences between the two channels are unknown; both are processed by the same callback function (and thus seem to have the same data-format). The packet(s) are both loaded to 7F0000h, size could be max 8000h (although, actual size might be 7E00h, as indicated by the checksum calculation). The overall format is:

```text
  0000h 3    Unknown/unused (3 bytes)
  0003h 8    ID "SHVCZDBJ" (compared against [B289D6])
  000Bh 2    Number of Chunks at 0010h and up (16bit) (little-endian) (min 1)
  000Dh 1    Unknown/unused? (8bit)
  000Eh 1    Checksum (bytes at [0000h..7DFFh] added together) (8bit)
  000Fh 1    Checksum complement (8bit)
  0010h DF0h Chunks
  7E00h 200h Begin of non-checksummed area (IF ANY) (if it DOES exist,
             then it MIGHT contain a file-style header at 7FB0h..7FFFh ?)
```

Note: The Chunks are processed via function at B28A10h. Despite of the hardcoded channel numbers, the packets must be also listed (with the same channel numbers) in the Directory Packet (as hidden Include File entries or so).

#### Hardware Channel 0121h - Test Channel

Used for hardware-connection test (received data is ignored). Used by BIOS function 105B6Ch; which is used only when pressing X+L+R buttons while the Welcome Message is displayed. Transmission Timeout for the Test Channel is 10 seconds.
