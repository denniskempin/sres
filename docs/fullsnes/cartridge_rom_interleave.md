# SNES Cartridge ROM-Image Interleave

Some ROM images are "interleaved", meaning that their content is ordered differently as how it appears in SNES memory. The interleaved format dates back to old copiers, most modern tools use/prefer normal ROM-images without interleave, but old interleaved files may still show up here and there.

Interleave used by Game Doctor & UFO Copiers These copiers use interleave so that the ROM Header is always stored at file offset 007Fxxh. For HiROM files, interleave is applied as so: store upper 32K of all 64K banks, followed by lower 32K of all 64K banks (which moves the header from 00FFxxh to 007Fxxh). For example, with a 320Kbyte ROM, the ten 32K banks would be ordered as so:

```text
  0,1,2,3,4,5,6,7,8,9 - Original
  1,3,5,7,9,0,2,4,6,8 - Interleaved
```

For LoROM files, there's no interleave applied (since the header is already at 007Fxxh).

Detecting an interleaved file could be done as so:

```text
  Header must be located at file offset 007Fxxh (ie. in LoROM fashion)
  Header must not be a Sufami Turbo header (=title "ADD-ON BASE CASSETE")
  Header must not be a Satellaview header (=different chksum algorithm)
  Header should not contain corrupted entries
  The "Map Mode" byte at "[007FD5h] ANDed with 0Fh" is 01h,05h,0Ah (=HiROM)
```

If so, the file is interleaved (or, possibly, it's having a corrupted header with wrong map mode setting).

#### Interleave used by Human Stupidity

There are interleaving & deinterleaving tools, intended to convert normal ROM-images to/from the format used by above copiers. Using that tools on files that are already in the desired format will result in messed-up garbage. For example, interleaving a 320Kbyte file that was already interleaved:

```text
  1,3,5,7,9,0,2,4,6,8 - Interleaved
  3,7,0,4,8,1,5,9,2,6 - Double-Interleaved
```

Or, trying to deinterleave a 320Kbyte file that wasn't interleaved:

```text
  0,1,2,3,4,5,6,7,8,9 - Original
  5,0,6,1,7,2,8,3,9,4 - Mis-de-interleaved
```

One can eventually repair such files by doing the opposite (de-)interleaving action. Or, in worst case, the user may repeat the wrong action, ending up with a Triple-Interleaved, or Double-mis-de-interleaved file.

Another case of stupidity would be applying interleave to a LoROM file (which would move the header <away from> 007Fxxh towards the middle of the file, ie. the opposite of the intended interleaving effect of moving it <to> 007Fxxh).

#### ExHiROM Files

ExHiROM Files are also having the data ordered differently as in SNES memory.

However, in this special case, the data SHOULD be ordered as so. The ordering is: Fast HiROM (4Mbytes, banks C0h..FFh), followed by Slow HiROM banks (usually 1-2MByte, banks 40h..4Fh/5Fh) (of which, the header and exception vectors are in upper 32K of the first Slow HiROM bank, ie. at file offset 40FFxxh). There are only 2 games using the ExHiROM format:

```text
  Dai Kaiju Monogatari 2 (JP) (5Mbytes) PCB: SHVC-LJ3R-01
  Tales of Phantasia (JP) (6Mbytes)     PCB: SHVC-LJ3M-01
```

The ExHiROM ordering is somewhat "official" as it was defined in Nintendo's developer manuals. Concerning software, the ordering does match-up with the checksum calculation algorithm (8MB chksum across 4MB plus mirror(s) of remaining 1-2MB). Concerning hardware, the ordering may have been 'required' in case Nintendo did (or planned to) use "odd" sized 5Mbyte/6Mbyte-chips (they DID produce cartridges with 3MByte/24Mbit chips).
