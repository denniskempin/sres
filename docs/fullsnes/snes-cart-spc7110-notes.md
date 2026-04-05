# SNES Cart SPC7110 Notes

#### Compression/Decompression Example

Uncompressed Data (64-byte ASCII string):

```text
  Test123.ABCDABCDAAAAAAAAaaaabbbbccccdddd7654321076543210.Test123
```

Compressed in Mode0:

```text
  68 91 36 15 F8 BF 42 35 2F 67 3D B7 AA 05 B4 F7 70 7A 26 20 EA 58 2C 09 61 00
  C5 00 8C 6F FF D1 42 9D EE 7F 72 87 DF D6 5F 92 65 00 00
```

Compressed in Mode1:

```text
  4B F6 80 1E 3A 4C 42 6C DA 16 0F C6 44 ED 64 10 77 AF 50 00 05 C0 01 27 22 B0
  83 51 05 32 4A 1E 74 93 08 76 07 E5 32 12 B4 99 9E 55 A3 F8 00
```

Compressed in Mode2:

```text
  13 B3 27 A6 F4 5C D8 ED 6C 6D F8 76 80 A7 87 20 39 4B 37 1A CC 3F E4 3D BE 65
  2D 89 7E 0B 0A D3 46 D5 0C 1F D3 81 F3 AD DD E8 5C C0 BD 62 AA CB F8 B5 38 00
```

#### Selftest Program

All three SPC7110 games include a selftest function (which executes on initial power-up, ie. when the battery-backed SRAM is still uninitialized). Press Button A/B to start 1st/2nd test, and push Reset Button after each test.

#### PCBs

```text
  SHVC-BDH3B-01 (without RTC)
  SHVC-LDH3C-01 (with RTC)
```
