// WAI stalls until vblank NMI, then the handler runs before the next instruction.
arch snes.cpu
output "wai_nmi.sfc", create

macro seek(variable offset) {
  origin ((offset & $7F0000) >> 1) | (offset & $7FFF)
  base offset
}

seek($8000); fill $8000 // Fill Upto $7FFF (Bank 0) With Zero Bytes
include "lib/snes.inc"        // Include SNES Definitions
include "lib/snes_header_nmi.asm" // Include Header & Vector Table

seek($8000)
  sei // Disable IRQs (NMI ignores I)
  clc // Clear Carry To Switch To Native Mode
  xce // Xchange Carry & Emulation Bit (Native Mode)
  sep #$20  // 8-bit A

  lda.b #$80
  sta.w REG_NMITIMEN // Enable vblank NMI
  wai
  lda $0000
  sta $0001
  stp

NmiHandler:
  lda.b #$A5
  sta $0000
  rti
