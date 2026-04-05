---
title: "Multitap"
source_url: "https://snes.nesdev.org/wiki/Multitap"
pageid: 62
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The SNES had a standard 5-player multitap accessory.

A [[Standard controller]] is used in port 1, and the multitap device is plugged into port 2. The device has 4 ports of its own to connect 4 more standard controllers.

The standard version of the multitap accessory has a 2-player / 5-player switch which can be used to disable the multitap and become a direct passthrough for the 2nd player controller plugged into its own port 2. With this switch set, the difference between a multitap and a directly connected second controller is not detectable.

The [[SNES Development Manual]] forbids using a [[Mouse]], [SuperScope](https://snes.nesdev.org/w/index.php?title=SuperScope&action=edit&redlink=1 "SuperScope (page does not exist)"), or a second multitap, for reasons of excessive current draw or incompatible interfaces.

## Interface

The high bit of [[MMIO registers#WRIO|WRIO]] ($4201) controls whether to read from multitap ports 2 and 3, or 4 and 5:

- $4201.7 = 1: select ports 2/3
- $4201.7 = 0: select ports 4/5

Normally $4201.7 should be kept at 1 most of the time, so that [[MMIO registers#NMITIMEN|automatic read]] can be used for controllers 2 and 3.

The reported bits for each controller are in the same order as with the standard controller, but over [[MMIO registers#JOYSER1|JOYSER1]] ($4017) on both data lines D0 and D1.

The multitap reports extend the report of each controller with 1 extra bit indicating whether a controller is connected (1) or not (0). This allows you to read the 17th bit to check for the presence of each multitap controller.

Outputs:

- Controller 1: $4106 D0 bits 0-15 (auto: [[MMIO registers#JOY1|JOY1]])
- Controller 2: $4017 D0 bits 0-15 (auto: [[MMIO registers#JOY2|JOY2]])
- Controller 3: $4017 D1 bits 0-15 (auto: [[MMIO registers#JOY4|JOY4]])
- Controller 4: $4017 D0 bits 17-32 (serial)
- Controller 5: $4017 D1 bits 17-32 (serial)

Using the automatic read functionality with $4201.7 set, controllers 1, 2 and 3 will be read to [[MMIO registers#JOY1|JOY1]], [[MMIO registers#JOY1|JOY2]], and [[MMIO registers#JOY1|JOY4]]. (Note that JOY3 is not used, as it corresponds to $4016 D1.)

Some multi-tap devices (especially third party) have been reported[[1]](#cite_note-1) to have an unusually slow transition from $4201.7 = 0 to 1, so this is another reason to read controllers 2/3 first.

## Reading

1. Set $4201.7 to 1 to select ports 2/3, if not already set.
2. Allow automatic-read to read the first 16 bits of controllers 1, 2 and 3.
   1. Without auto-read: strobe $4016 and read the first 16 bits serially.
3. Wait for automatic read to finish. (Poll [[MMIO registers#HBVJOY|HBVJOY]] bit 0.)
4. *Optional:* read $4017 one extra time for the 17th bit, indicating whether controllers 2 and 3 are connected.
5. Set $4201.7 to 0 to select ports 4/5.
6. Read the 16-bit report for controllers 4 and 5.
7. *Optional:* read the 17th bit to detect whether controllers 4 and 5 are connected.
8. Return $4201.7 to 1 to select ports 2/3 for the next frame.

## Detecting the Multitap

The multitap does not directly have a signature in the report, but it can be detected by the following official process:

1. Write 1 to $4016 D0.
2. Read $4017 D1 eight times, check for $FF.
3. Write 0 to $4016 D0.
4. Read $4017 D1 eight times, check that it is not $FF.

It should report 8 1s while the strobe is active, and then after clearing the strobe, it should report any 8-bit value except $FF. Note that this uses D1 rather than D0 to check.

Once the multitap is detected, the 17th bit of each controller report can be used to detect whether they are connected, individually.

## References

- [[SNES Development Manual]] Book II 4-9-1 Multiplayer 5 Specifications
- [[SNES Development Manual]] Book II 4-10-6 m\_check.x65 / multi5.x65 Multiplayer 5 Supplied BIOS Program Listings

1. [↑](#cite_ref-1) [Controllers: Mulitap](https://wiki.superfamicom.org/controllers#multitap-mp5-385) - superfamicom.org wiki article
