# SNES Cart Satellaview Directory Packet

#### Directory (Software Channel 1.1.0.6)

Loaded to 7FC000h and then copied to 7EC000h. Size (max) 16Kbytes.

```text
  00h   1   Directory ID (compared to Directory ID in Town Status packet)
  01h   1   Number of Folders (Buildings, People, or hidden folders)
  02h   3   Unknown/unused
  05h   ..  Folders (and File) entries (see below)       ;\together
  ..    1   Number of Expansion Data Entries (00h=none)  ; max 3FFBh bytes
  ..    ..  Expansion Data Entries (see next chapter)    ;/
```

#### Folder Entry Format

```text
  00h   1   Flags (bit0=1=Invalid) (bit1-7=unknown/unused)
  01h   1   Number of File Entries (if zero: buildings are usually closed)
  02h   15h Folder Name (max 20 chars, terminated by 00h) (shown in building)
  17h   1   Length of Folder Message (X) (in bytes, including ending 00h)
  18h   X   Folder Message/Description (terminated by 00h)
  18h+X 1   More Flags (Folder Type)
             Bit0   : Folder Content (0=Download/Files, 1=Shop/Items)
             Bit1-3 : Folder Purpose (0=Building, 1=Person, 2=Include-Files)
               000b = Indoors (Building ID at [19h+X])     (Bit3-1=000b)   (0)
               x01b = Outdoors (Person ID at [19h+X])      (Bit2-1=01b)  (1,5)
               x1xb = Folder contains hidden Include Files (Bit2=1b) (2,3,6,7)
               100b = Unknown/unused (may be useable for "Files at Home"?) (4)
             Bit4-7 : Unknown/unused
  19h+X 1   Folder 6bit ID (eg. for Building:01h=News, for People:01h=Hiroshi)
  1Ah+X 1   Unknown/Unused
  1Bh+X 1   Unknown/Unused
  1Ch+X 1   Clerk/Avatar (00h..10h) (eg. 0Eh=Robot, 10h=BS-X) (11h..FFh=Crash)
  1Dh+X 1   Unknown/Unused
  1Eh+X 1   Unknown/Unused
  1Fh+X 1   Unknown/Unused
  20h+X ..  File/Item Entries (each one is 32h+X bytes; for items: fixed X=79h)
```

#### File/Item Entry Format

For Both Files and Items:

```text
  00h   1   File ID (compared to File IDs in Town Status Packet)
  01h   1   Flag (bit0=used by "Town Status" check (1=Not Available))
  02h   15h File Name (max 20 chars, terminated by 00h) (shown in building)
```

#### For Files: (in File Folders)

```text
  17h   1   Length of File Message (X) (in bytes, including ending 00h)
  18h   X   File Message/Description (terminated by 00h)
```

#### For Items: (in Item Folders)

```text
  17h   1   Length of Item Description+Activation+Price+Flag (X) (fixed=79h)
  18h   25h Item Description (max 36 chars, plus ending 00h)
  3Dh   47h Item Activation Message (min 1, max 70 chars, plus ending 00h)
  84h   12  Item Price (12-Digit ASCII String, eg. "000000001200" for 1200G)
  90h   1   Item Drop/Keep Flag (00h=Drop after Activation, 01h=Keep Item)
```

For Both Files and Items:

```text
  18h+X 4   Software Channel Number of Current File (for Items: N.N.0.0=None)
  1Ch+X 3   Big-Endian Filesize
  1Fh+X 3   Unknown/unused (except, first 2 bytes used by Derby Stallion 96)
  22h+X 1   Flags
              Bit0: used by "Town Status" check (1=Not Available)
              Bit1: Unknown/unused
              Bit2: Building Only (0=Also available at Home, 1=Building only)
              Bit3: Low Download Accuracy / Streaming or so
               0=High-Download-Accuracy (for programs or other important data)
               1=Low-Download-Accuracy (for audio/video data streaming)
              Bit4: Unused (except, must be 0 for Derby Stallion 96 files)
              Bit5-7: Unknown/unused
  23h+X 1   Unknown/unused
  24h+X 1   Flags/Target (seems to be same/similar as in Channel Map)
             Bit2-3:
              0=Download to WRAM (not really implemented, will crash badly)
              1=Download to PSRAM (without saving in FLASH)
              2=Download to Continous FLASH banks (erases entire chip!)
              3=Download to FREE-FLASH banks (relocate to PSRAM upon execution)
  25h+X 2   Unknown/unused
  27h+X 1   Date (Bit7-4=Month, Bit3-0=0)             ;\copied to Satellaview
  28h+X 1   Date (Bit7-3=Day, Bit2=0, Bit1-0=Unknown) ;/FLASH File Header FFD6h
  29h+X 1   Timeslot (Bit7-3=StartHours.Bit4-0, Bit2-0=Start Minutes.Bit5-3)
  2Ah+X 1   Timeslot (Bit4-0=EndHours.Bit4-0,   Bit7-5=Start Minutes.Bit2-0)
  2Bh+X 1   Timeslot (Bit7-2=EndMinutes.Bit5-0, Bit1-0=Unused)
  2Ch+X 4   Software Channel Number for additional Include File (N.N.0.0=None)
  30h+X 2   Unknown/unused
```

The directory may contain files that aren't currently transmitted; check the Town Status for list of currently available File IDs. Also check the Directory ID in the Town Status, if it doesn't match up with the ID of the Directory packet, then one must download the Directory again (otherwise one needs to download it only once after power-up).

#### Include Files

Each File in the directory has an Include File entry. Before downloading a file, the BIOS checks if the Include File's Software Channel Number is listed in the Directory (in any folder(s) that are marked as containing Include Files). If it isn't listed (or if it's N.N.0.0), then there is no include file.

If it is listed, then the BIOS does first download the include file, and thereafter download the original file. Whereas, the include file itself may also have an include file, and so on (the download order starts with the LAST include file, and ends with the original file).

There are some incomplete dumps of some games, which have 1Mbyte FLASH dumped, but do require additional data in WRAM/PSRAM:

```text
  BS Dragon Quest (copy of channel_map in PSRAM, episode_number in WRAM)
  BS Zelda no Densetsu Kodai no Sekiban Dai 3 Hanashi (hw_channel in WRAM)
```

The missing data was probably transferred in form of Include files.

#### Streaming Files

There seems to be some streaming support:

Files flagged as FILE[22h+X].Bit3=1 are repeatedly downloaded-and-executed again and again (possibly intended to produce movies/slideshows) (details: max 256K are loaded to upper half of PSRAM, and, if successfully received: copied to lower 256K of PSRAM and executed in that place; whereas, the execution starts once when receiving the next streaming block, that is: with a different 4bit transmission ID in the 10-byte packet header).

Moreover, Packets marked as having 00h fragments, are treated as having infinite fragments (this would allow to overwrite received data by newer data; though none of the higher-level BIOS functions seems to be using that feature).

#### Files Available at Home

Some files can be downloaded at the "Home" building (via the 3rd of the 4 options in that building), these "Home" files may be located in any folders, namely, including following folders:

```text
  FOLDER[00h].Bit0=Don't care (folder may be marked as hidden)
  FOLDER[18h+X]=Don't care    (folder may be also used as building/person/etc.)
```

For downloading them at "Home", the files must be defined as so:

```text
  FILE[1Ah+X]<>0000h   (software channel isn't N.N.0.0)
  FILE[22h+X].Bit2=0   (flagged as available at home)
  FILE[18h]=Don't care (file description isn't shown at home)
```

BUG: If there are more than 32 of such "Home" files, then the BIOS tries to skip the additional files, but destroys the stack alongside that attempt.

Note: Unlike other downloads, Home ones are always having a wooden-plank background, and are done without transmission Interval timeouts. And, Include Files are ignored. And, Autostart is disabled (ie. downloads to PSRAM are totally useless, downloads to FLASH can/must be manually started).
