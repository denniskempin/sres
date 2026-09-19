// WAI stalls until H-IRQ even with I=1, then continues without taking the vector.
arch snes.cpu
output "wai_irq.sfc", create

macro seek(variable offset) {
  origin ((offset & $7F0000) >> 1) | (offset & $7FFF)
  base offset
}

seek($8000); fill $8000 // Fill Upto $7FFF (Bank 0) With Zero Bytes
include "lib/snes.inc"        // Include SNES Definitions
include "lib/snes_header_irq.asm" // Include Header & Vector Table

seek($8000)
  sei // Keep I=1; WAI still wakes on IRQ
  clc // Clear Carry To Switch To Native Mode
  xce // Xchange Carry & Emulation Bit (Native Mode)
  sep #$20  // 8-bit A

  lda.b #$40
  sta.w REG_HTIMEL
  stz.w REG_HTIMEH
  lda.b #$10
  sta.w REG_NMITIMEN // Enable H-IRQ
  wai
  lda.w REG_TIMEUP
  sta $0001
  stp

IrqHandler:
  lda.b #$FF
  sta $0000
  rti
