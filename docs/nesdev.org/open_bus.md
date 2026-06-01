---
title: "Open bus"
source_url: "https://snes.nesdev.org/wiki/Open_bus"
pageid: 162
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

When no device responds on a data bus, the result is known as open bus. This can happen at memory regions where no ROM or other device is mapped, or when reading from a register for which only some bits are driven, leaving the other bits floating.

In general, open bus will retain the last value present on it for some amount of time before it decays, and so reads will return this stale value. The timing and decay behavior are analog effects and can vary. Driving a bit effectively refreshes its decay timer, largely independent of the other bits.

## CPU open bus

For the CPU bus, the last value driven on each data line will be retained, and reading from open bus will simply repeat each line's value. CPU writes always drive all 8 bits, while CPU reads may drive all, some, or none of the bits. CPU open bus has been found to decay in approximately 2 frames.[[1]](#cite_note-1) In most cases, the open bus value is the last byte of the instruction data fetched before the read (often the high byte of the read address), or for an indirect instruction it may be the high byte of the fetched indirect address. Note that a cartridge can potentially influence the behavior of CPU open bus, such as maintaining its value indefinitely or forcing undriven bits to 0 or 1.

The 5A22 S-CPU chip also has an internal data bus, and it is suspected there are actually separate read and write buses, each only updated by their respective access type. When reading registers internal to the 5A22 (readable registers in the range $4016-$437F), the value only exists on the internal read bus, ignoring and separate from any value on the external bus. In this case, any bits that are open bus would return the value from the last read, even if a write occurred in between because that write would not have updated the read bus. The read bus state would also be lost as soon as an external read occurs, overridden by the external bus even if the external bus is open.

## PPU open bus

The two PPU units each have an internal output bus with open bus behavior. When the CPU reads a readable PPU register, that PPU drives all 8 bits of its output bus onto the CPU bus, but the output bus may itself have open bus bits retaining for some time the last value driven on them. A PPU register with open bus bits means those bits are not driven onto the PPU's output bus, and so the CPU will read the stale or decayed open bus value persisting on those bits. Writes to PPU registers go to the PPU's internal input bus and do not affect the output bus.

- PPU1 read addresses: $2134-$2136, $2138-$213A, $213E
  - Some of PPU1's write-only registers also return PPU1 open bus when read: $21x4-$21x6, $21x8-$21xA (x=0-2)
- PPU2 read addresses: $213B-$213D, $213F

## References

:   - [Fullsnes: SNES Unpredictable Things](https://problemkaputt.de/fullsnes.htm#snesunpredictablethings)
    - [Anomie's SNES Documents](https://www.romhacking.net/community/548/) - ob-wrap.txt
    - [Open Bus](https://wiki.superfamicom.org/open-bus) - superfamicom.org wiki (Anomie)

1. [↑](#cite_ref-1) [Forum post](https://forums.nesdev.org/viewtopic.php?p=287063#p287063): lidnariq's open bus test ROM and results
