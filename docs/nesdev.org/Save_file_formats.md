---
title: "Save file formats"
source_url: "https://snes.nesdev.org/wiki/Save_file_formats"
pageid: 131
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Save files for SNES generally come in two forms: Save RAM, and Savestates.

## Save RAM

Most commonly with the extension **.SRM**, this is a simple dump of the battery backed cartridge save RAM.

The [[ROM header]] will specify the size of save RAM for the cartridge, if it exists, and otherwise it is a linear dump of SRAM starting from its lowest address according to the cartridge [[Memory map]].

This format is universally supported across emulators, and can be transferred to and from the original cartridges with suitable hardware.

Emulators generally save SRM files automatically when they close, if the ROM header specified the existence of save RAM. Many emulators save it periodically during play if data has changed, so that progress is not lost in the case of an emulator crash.

## Savestates

Savestates store the entire state of the SNES at a given moment, allowing instant resume from any point. Many emulators have this feature, and some flash-carts can do it as well.

There is no standard format for this, as in most cases it is really a dump of the emulator's internal state.

Save state formats can sometimes be deduced from the emulator's source code:

- [ZSNES zstate.c](https://github.com/emillon/zsnes/blob/master/src/zstate.c)
