# SNES Add-On Turbo File - STF Mode Filesystem

#### FileTwinCapacityCodes (last 4bit of FileTwinRecvStatusAndID)

```text
  00h  Single Drive with 128Kbytes (normal) (plus extra 32Kbyte for TFII mode)
  01h  Single Drive with 256Kbytes
  02h  Single Drive with 384Kbytes
  03h  Single Drive with 640Kbytes (really, this is NOT 512Kbytes)
  04h  Multi-Drive with 1 normal 128K Drive  (128K total) ;\allows to READ from
  05h  Multi-Drive with 2 normal 128K Drives (256K total) ; all drives, but
  06h  Multi-Drive with 3 normal 128K Drives (384K total) ; can WRITE only
  07h  Multi-Drive with 5 normal 128K Drives (640K total) ;/to first drive
  08h..0Fh  Reserved (treated same as 00h; Single Drive with 128Kbytes)
```

#### FileTwinAddresses (?)

#### XXX multiply below by 400h

```text
  000h..00Fh  FAT (4Kbytes)
  010h..1FFh  Entries for 1st 124Kbytes
  200h..7FFh  Unused
  800h..9FFh  Entries for 2nd 128Kbytes (if any)
  A00h..BFFh  Entries for 3rd 128Kbytes (if any)
  C00h..DFFh  Entries for 4th 128Kbytes (if any) (seems to be bugged)
  E00h..FFFh  Unused
  xxxh..xxxh  Partition-Read FAT (though WRITES are to address zero ?)
```

#### FileTwinFAT

The FAT is 4096 bytes in size:

```text
  000h..1EFh  Entries for 1st 124Kbytes
  1F0h..3EFh  Entries for 2nd 128Kbytes (if any)
  3F0h..5EFh  Entries for 3rd 128Kbytes (if any)
  5F0h..7EFh  Entries for 4th 128Kbytes (if any) (seems to be bugged)
  7F0h..FFBh  Unused
  FFCh..FFFh  ID "FAT0"
```

Each FAT Entry is 32bit (4 bytes) wide:

```text
  0-11   filesize in kbyte (1st blk), 000h (2nd..Nth blk), FFFh (free blk)
  12-23  NNNh (next blk), FFFh (last blk), or also FFFh (free blk)
  24-31  8bit sector chksum (all 1024 bytes added together), or FFh (free blk)
```

Note: The above FFFh values should be as so (though older games are checking only the upper 4bit, thereby treating any Fxxh values as free/last block).

Unused FAT entries (that exceed memory capacity) are also marked as "free".

#### FileTwinFileHeaders

The first 24 bytes (in the first sector) of a file contain File ID & Name:

```text
  000h..003h ID1 (must be "SHVC")
  004h..007h ID2 (should be same as Game Code at [FFB2h] in ROM-Header)
  008h..017h Filename (padded with ...spaces ?)
  018h..     File Data (Filesize from FAT, multiplied by 1024, minus 24 bytes)
```

ID2 and Name may contain ASCII characters 20h..3Fh and 41h..5Ah.
