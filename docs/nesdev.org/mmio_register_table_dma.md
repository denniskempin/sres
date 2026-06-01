---
title: "MMIO register table/DMA"
source_url: "https://snes.nesdev.org/wiki/MMIO_register_table/DMA"
pageid: 45
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

See: [[MMIO register table]]

| Name | Address | Bits | Type | Notes |
| --- | --- | --- | --- | --- |
| [[DMA registers#DMAPn|DMAPn]] | $43n0 | DI.A APPP | RW8 | Direction (D), indirect HDMA (I), address increment mode (A), transfer pattern (P). |
| [[DMA registers#BBADn|BBADn]] | $43n1 | AAAA AAAA | RW8 | B-bus address. |
| [[DMA registers#A1TnL|A1TnL]] [[DMA registers#A1TnH|A1TnH]] [[DMA registers#A1Bn|A1Bn]] | $43n2 $43n3 $43n4 | LLLL LLLL HHHH HHHH BBBB BBBB | RW24 | DMA source address / HDMA table start address. |
| [[DMA registers#DASnL|DASnL]] [[DMA registers#DASnH|DASnH]] [[DMA registers#DASBn|DASBn]] | $43n5 $43n6 $43n7 | LLLL LLLL HHHH HHHH BBBB BBBB | RW24 | DMA byte count (H:L) / HDMA indirect table address (B:H:L). |
| [[DMA registers#A2AnL|A2AnL]] [[DMA registers#A2AnH|A2AnH]] | $43n8 $43n9 | LLLL LLLL HHHH HHHH | RW16 | HDMA table current address within bank (H:L). |
| [[DMA registers#NLTRn|NLTRn]] | $43nA | RLLL LLLL | RW8 | HDMA reload flag (R) and scanline counter (L). |
| [[DMA registers#UNUSEDn|UNUSEDn]] | $43nB $43nF | DDDD DDDD | RW8 | Unused shared data byte (D). |
