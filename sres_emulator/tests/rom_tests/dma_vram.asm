// DMA copy from/to VRAM test
//
// 1. Generates test data in WRAM at $0000 (a sequence of 0x00..0xFF)
// 2. DMA transfer the data to VRAM at $0000
// 3. DMA transfer back from VRAM into WRAM at $0100

output "dma_vram.sfc", create

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
// DMA transfer from WRAM $0000 into VRAM $0000

// Set VMAIN: Increment by 1 word after writing $2119
lda.b #$80
sta.w REG_VMAIN

// Set VMADD: Destination address in VRAM = $0000
ldx.w #$0000
stx.w REG_VMADDL

// Set DMA mode: 0000 0001 = A -> B, Increment A, "+0 +1" pattern
lda.b #$01
sta.w REG_DMAP0

// Set B-bus target to $2018 = VMDATAL
lda.b #$18
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
// DMA transfer from VRAM $0000 into WRAM $0100
// This time, transfer in two batches of 0x80 bytes.

// Set DMA mode: 0000 0001 = A <- B, Increment A, "+0 +1" pattern
lda.b #$81
sta.w REG_DMAP0

// Set VMADD: Destination address in VRAM = $0000
ldx.w #$0000
stx.w REG_VMADDL
ldx.w REG_RDVRAML // Pre-load latch after setting address

// Set A-bus address to $000100
lda.b #$00
sta.w REG_A1B0
ldx.w #$0100
stx.w REG_A1T0L

// Set transfer size: 0x0100
ldx.w #$0080
stx.w REG_DAS0L

// Set B-bus target to $2039 = VMDATALREAD
lda.b #$39
sta.w REG_BBAD0

// Execute two transfers 0f 0x80 bytes to transfer a total of 0x100.
lda.b #$01
sta.w REG_MDMAEN
nop
sta.w REG_MDMAEN
nop

stp
