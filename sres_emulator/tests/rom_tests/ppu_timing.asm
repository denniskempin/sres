// PPU Timing test
//
// This test just executes a series of nop's and is used to compare PPU cycles of This
// emulator against


arch snes.cpu
output "ppu_timing.sfc", create

macro seek(variable offset) {
  origin ((offset & $7F0000) >> 1) | (offset & $7FFF)
  base offset
}

seek($8000); fill $8000 // Fill Upto $7FFF (Bank 0) With Zero Bytes
include "lib/snes_header.asm" // Include Header & Vector Table

seek($8000);
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
jmp $8000
