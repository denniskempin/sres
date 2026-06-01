# SNES Add-On Turbo File - TFII Mode Filesystem

#### Turbo File Memory

The first byte (at offset 0000h) is unused (possibly because that there is a risk that other games with other controller access functions may destroy it); after resetting the address, one should read one dummy byte to skip the unused byte. The used portion is 8191 bytes (offset 0001h..1FFFh). The "filesystem" is very simple: Each file is attached after the previous file, an invalid file ID indicates begin of free memory.

Turbo File Fileformat (newer files) (1987 and up) Normal files are formatted like so:

```text
  2   ID "AB" (41h,42h)
  2   Filesize (16+N+2) (including title and checksum)
  16  Title in ASCII (terminated by 00h or 01h)
  N   Data Portion
  2   Checksum (all N bytes in Data Portion added together)
```

#### Turbo File Fileformat (old version) (1986)

The oldest Turbo File game (NES Castle Excellent from 1986) doesn't use the above format. Instead, it uses the following format, without filename, and with hardcoded memory offset 0001h..01FFh (511 bytes):

```text
  1   Don't care (should be 00h)    ;fixed, at offset 0001h
  2   ID AAh,55h                    ;fixed, at offset 0002h..0003h
  508 Data Portion (Data, end code "BEDEUTUN", followed by some unused bytes)
```

CAUTION:

```text
  The early version has transferred all bytes in reversed bit-order,
  so above ID bytes AAh,55h will be seen as 55h,AAh in newer versions!
```

Since the address is hardcoded, Castle Excellent will forcefully destroy any other/newer files that are located at the same address. Most newer NES/SNES games (like NES Fleet Commander from 1988, and SNES Wizardry 5 from 1992) do include support for handling the Castle Excellent file. One exception that doesn't support the file is NES Derby Stallion - Zenkoku Ban from 1992.
