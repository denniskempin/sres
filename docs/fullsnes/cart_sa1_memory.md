# SNES Cart SA-1 Memory Control

2220h SNES CXB - Set Super MMC Bank C - Hirom C0h-CFh / LoRom 00h-1Fh (W) 2221h SNES DXB - Set Super MMC Bank D - Hirom D0h-DFh / LoRom 20h-3Fh (W) 2222h SNES EXB - Set Super MMC Bank E - Hirom E0h-EFh / LoRom 80h-9Fh (W) 2223h SNES FXB - Set Super MMC Bank F - Hirom F0h-FFh / LoRom A0h-BFh (W)

```text
  0-2  Select 1Mbyte ROM-Bank (0..7)
  3-6  Not used (should be 0)
  7    Map 1Mbyte ROM-Bank (0=To HiRom, 1=To LoRom and HiRom)
```

If LoRom mapping is disabled (bit7=0), then first 2 MByte of ROM are mapped to 00h-3Fh, and next 2 MByte to 80h-BFh. The registers do affect both SNES and SA-1 mapping.

2224h SNES BMAPS - SNES CPU BW-RAM Mapping to 6000h-7FFFh (W)

```text
  0-4  Select 8Kbyte BW-RAM Block for mapping to 6000h-7FFFh (0..31)
  5-7  Not used (should be 0)
```

BW-RAM is always mapped to bank 40h-43h (max 256 Kbytes).

This register allows to map an 8Kbyte chunk to offset 6000h-7FFFh in bank 0-3Fh and 80h-BFh.

2225h SA-1 BMAP - SA-1 CPU BW-RAM Mapping to 6000h-7FFFh (W)

```text
  0-6  Select 8Kbyte BW-RAM Block for mapping to 6000h-7FFFh (0..31 or 0..127)
  7    Select source (0=Normal/Bank 40h..43h, 1=Bitmap/Bank 60h..6Fh)
```

223Fh SA-1 BBF - BW-RAM Bit Map Format for 600000h-6FFFFFh (W)

```text
  0-6 Not used (should be "..") (whatever ".." means, maybe "0"?)
  7   Format (0=4bit, 1=2bit)
```

"BW-RAM bitmap logical space format setting from perspective of the SA-1 CPU"

```text
  600000h.Bit0-1 or Bit0-3 mirrors to 400000h.Bit0-1 or 400000h.Bit0-3
  600001h.Bit0-1 or Bit0-3 mirrors to 400000h.Bit2-3 or 400000h.Bit4-7
  600002h.Bit0-1 or Bit0-3 mirrors to 400000h.Bit4-5 or 400001h.Bit0-3
  600003h.Bit0-1 or Bit0-3 mirrors to 400000h.Bit6-7 or 400001h.Bit4-7
  etc.
```

Note that the LSBs in the packed-area contain the left-most pixel (not the right-most one). The MSBs in the unpacked area are "ignored" (this is obvious in case of writing; for reading it's unknown what it means - are reads supported at all, and if so, do they return zero's or garbage in MSBs?)

2226h SNES SBWE - SNES CPU BW-RAM Write Enable (W) 2227h SA-1 SBWE - SA-1 CPU BW-RAM Write Enable (W)

```text
  0-6  Not used (should be 0)
  7    Write Enable BW-RAM (0=Protect, 1=Write Enable)
```

#### 2228h SNES BWPA - BW-RAM Write-Protected Area (W)

```text
  0-3  Select size of Write-Protected Area ("256 SHL N" bytes)
  4-7  Not used (should be 0)
```

Selects how many bytes (originated at 400000h) shall be write protected.

It isn't possible to set the size to "none" (min is 256 bytes), though, one can probably completely disable the protection via ports 2226h/2227h?

2229h SNES SIWP - SNES I-RAM Write-Protection (W) 222Ah SA-1 CIWP - SA-1 I-RAM Write-Protection (W)

```text
  0-7  Write enable flags for eight 256-byte chunks (0=Protect, 1=Write Enable)
```

Bit0 for I-RAM 3000h..30FFh, bit1 for 3100h..31FFh, etc. bit7 for 3700h..37FFh.
