// PPU Timing test
//
// This test just executes a series of nop's and is used to compare PPU cycles of This
// emulator against

output "ppu_timing.sfc", create

include "lib/base.asm"

Start:
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
    jmp Start
