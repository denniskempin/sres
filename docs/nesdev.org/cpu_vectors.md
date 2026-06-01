---
title: "CPU vectors"
source_url: "https://snes.nesdev.org/wiki/CPU_vectors"
pageid: 49
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

When an interrupt occurs, the address of the interrupt handler is read from the vector table in bank $00. The vector used is determined by the type of interrupt and the current CPU mode. Vectors are all 16-bit and the target bank is forced to $00.

## 65C816 native mode vectors

| Vector | Address | Examples |
| --- | --- | --- |
| COP | $FFE4-FFE5 | COP instruction |
| BRK | $FFE6-FFE7 | BRK instruction |
| (ABORT) | $FFE8-FFE9 | (Unused on 5A22 S-CPU) |
| NMI | $FFEA-FFEB | [[MMIO registers#NMITIMEN|NMITIMEN]] vblank interrupt, or 5A22 /NMI input |
| (none) | $FFEC-FFED |  |
| IRQ | $FFEE-FFEF | [[MMIO registers#NMITIMEN|NMITIMEN]] H/V timer interrupt, or external interrupt (5A22 /IRQ input) |

## 6502 emulation mode vectors

| Vector | Address | Examples |
| --- | --- | --- |
| COP | $FFF4-FFF5 | COP instruction |
| (none) | $FFF6-FFF7 |  |
| (ABORT) | $FFF8-FFF9 | (Unused on 5A22 S-CPU) |
| NMI | $FFFA-FFFB | 5A22 /NMI input |
| RESET | $FFFC-FFFD | 5A22 /RESET (CPU always resets into 6502 mode) |
| IRQ/BRK | $FFFE-FFFF | BRK instruction, or external interrupt (5A22 /IRQ input) |
