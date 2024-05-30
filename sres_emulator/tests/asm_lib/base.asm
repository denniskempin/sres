arch snes.cpu

macro seek(variable offset) {
  origin ((offset & $7F0000) >> 1) | (offset & $7FFF)
  base offset
}

seek($8000); fill $8000 // Fill Upto $7FFF (Bank 0) With Zero Bytes
include "snes_header.asm" // Include Header & Vector Table
include "snes.inc" // Include Header & Vector Table

seek($8000);

