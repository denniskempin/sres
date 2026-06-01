---
title: "ROM file formats"
source_url: "https://snes.nesdev.org/wiki/ROM_file_formats"
pageid: 122
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Super NES ROM files are usually found in one of 2 variations of the same format.

The most common filename extension is **.SFC**, followed by **.SMC**. Less common extensions include: **.FIG**, **.SWC**.

The data contained in the file may be **unheadered** or **headered**. The only difference between these two is that the *headered* version has 512 extra bytes at the start of the file.

The 512 byte header is not the same as the [[ROM header]], which is a part of the on-cartridge ROM. Instead it is 512 bytes of metadata gathered by a [[Copier]] device used to dump the ROM. This extra data is generally considered useless, except to the specific copier device that it was originally used with. Modern common practice prefers *unheadered* ROM files.

## ROM file layout

**See: [[Memory map]]**

Ignoring the optional 512-byte header, the files contain the contents of the cartridge ROM in linear order. Depending on how the cartridge is mapped, there are 3 orderings:

- LoROM: 32k banks starting from $800000
- HiROM: 64k banks starting from $C00000
- ExHiROM: 64k banks starting from $C00000, then continuing from $400000 after 4MB.

This means that the [[ROM header]] could be at one of 3 locations which maps to memory at $00FFC0:

- LoROM: $007FC0
- HiROM: $00FFC0
- ExHiROM: $40FFC0

ROM sizes are not always a power of two, but should at least be the sum of two powers of two. For example: this allows a cartridge that needs 3MB to have one ROM of 2MB and a second ROM of 1MB.

Unpacking a ROM to fill the memory space should use the same mirroring rules as the [[ROM header#Checksum|ROM header's checksum]].

## Detecting Headered ROM

Because ROM files are generally expected to include complete 32 or 64 kb banks, a simple way of detecting a header is by checking if the file size modulo 1024 is equal to 512.

Alternatively, if a [[ROM header]] is not detected at one of the 3 normal locations, looking for them again at a 512 byte offset may also identify the headered ROM. (Note that the ROM header's [[ROM header#Checksum|checksum]] calculation would not include the 512 bytes of this file's header.)
