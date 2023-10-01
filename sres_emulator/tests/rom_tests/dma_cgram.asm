// DMA copy from/to CGRAM test
//
// 1. Generates test data in WRAM at $0000 (a sequence of 0x00..0xFF)
// 2. DMA transfer the data to cgram at $0000
// 3. DMA transfer back from cgram into WRAM at $0100

output "dma_cgram.sfc", create

include "lib/base.asm"

sei // Disable Interrupts
clc // Clear Carry To Switch To Native Mode
xce // Xchange Carry & Emulation Bit (Native Mode)
rep #$10  // Enable 16-Bit index registers

////////////////////////////////////////////////////////////////////////////////////////////////////
// Load 0x00..0xFF into $0000
lda.b #$00
ldx.w #$0000
-
    sta.w $0000,x
    inc
    inx
    cpx.w #$0100
    bne -

////////////////////////////////////////////////////////////////////////////////////////////////////
// DMA transfer from WRAM $0000 into CGRAM $00

// Set CGADD: Destination address in CGRAM = $00
lda.b #$00
sta.w REG_CGADD

// Set DMA mode: 0000 0010 = A -> B, Increment A, "+0 +0" pattern
lda.b #$02
sta.w REG_DMAP0

// Set B-bus target to $2122 = CGDATA
lda.b #$22
sta.w REG_BBAD0

// Set A-bus address to $000000
lda.b #$00
sta.w REG_A1B0
ldx.w #$0000
stx.w REG_A1T0L

// Set transfer size: 0x0100
ldx.w #$0100
stx.w REG_DAS0L

// Enable DMA transfer
lda.b #$01
sta.w REG_MDMAEN
nop

////////////////////////////////////////////////////////////////////////////////////////////////////
// DMA transfer from CGRAM $00 into WRAM $0100

// Set DMA mode: 0000 0001 = A <- B, Increment A, "+0 +0" pattern
lda.b #$82
sta.w REG_DMAP0

// Set CGADD: Destination address in CGRAM = $0000
lda.b #$00
sta.w REG_CGADD

// Set A-bus address to $000100
lda.b #$00
sta.w REG_A1B0
ldx.w #$0100
stx.w REG_A1T0L

// Set transfer size: 0x0100
ldx.w #$0100
stx.w REG_DAS0L

// Set B-bus target to $213B = VMDATALREAD
lda.b #$3B
sta.w REG_BBAD0

// Enable DMA transfer
lda.b #$01
sta.w REG_MDMAEN
nop

stp
