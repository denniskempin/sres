// SNES SPC700 Play Noise Demo (CPU Code) by krom (Peter Lemon):
arch snes.cpu
output "play_noise.sfc", create

macro seek(variable offset) {
  origin ((offset & $7F0000) >> 1) | (offset & $7FFF)
  base offset
}

seek($8000); fill $8000 // Fill Upto $7FFF (Bank 0) With Zero Bytes
include "lib/snes.inc"        // Include SNES Definitions
include "lib/snes_header.asm" // Include Header & Vector Table
include "lib/snes_spc700.inc" // Include SPC700 Definitions & Macros

seek($8000); Start:
  SNES_INIT(SLOWROM) // Run SNES Initialisation Routine

  SPCWaitBoot() // Wait For SPC To Boot
  TransferBlockSPC(SPCROM, SPCRAM, SPCROM.size) // Load SPC File To SMP/DSP
  SPCExecute(SPCRAM) // Execute SPC At $0200

Loop:
  jmp Loop

// SPC Code
// BANK 0
insert SPCROM, "play_noise.spc"
