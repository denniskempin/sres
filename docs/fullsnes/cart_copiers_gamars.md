---
summary: Documents the Gamars Super Disk FC-301 SNES cart copier, listing its known hardware variants (ALMA Super Disk F-16, FR-402 Super Disk) along with the required SRAM mirroring and FDC register mappings. Also clarifies that the commonly labeled "Gamars (Copier BIOS)" ROM image is actually a puzzle game.
keywords: Gamars, cart copier, FDC, SRAM, ALMA
importance: 1
---

# SNES Cart Copiers - Gamars Copier

Known as:

```text
  ALMA Super Disk F-16
  Gamars Super Disk FC-301
  FR-402 Super Disk (bundled with "FR-402 Super 16bit" SNES clone)

  2K SRAM at 005000 with REQUIRED mirror at 005800
            3F5Fxx.W  set to FFh,FFh,FFh...
            3F5FC0.R  FDC stat  (bit7,bit5)
            3F5FD2.W  FDC motor? (set to 0Ch,1Ch,08h,0Ch)
            3F5FE4.R  FDC Main Status
            3F5FED.RW FDC Command/Data (emit 03,DF,03)
```

#### Gamars Puzzle

Aside from the Gamars BIOSes, there's a mis-named ROM-image in the internet:

"Gamars (Copier BIOS)", this file is made by the same company, but it's a Puzzle game, not a copier BIOS.
