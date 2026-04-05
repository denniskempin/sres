---
title: "Init code"
source_url: "https://snes.nesdev.org/wiki/Init_code"
pageid: 24
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

When the SNES is powered on or reset, the 65c816 will jump to the [[CPU vectors|RESET CPU vector]] (in bank 0) in 65C02 emulation mode.

At minimum, the Reset Handler should do the following:

- Preform a long jump into a FastROM bank (if the ROM speed is FastROM)
- Disable the emulation flag (switch to 65816 native mode)
- Setup the status flags. Specifically the register size flags, decimal mode flag (cleared) and irq disable flag (set).
- Setup the *Stack Pointer*
- Setup the *Direct Page* and *Data Bank* registers
- Disable interrupts and rendering

Afterwords, it is recommended you:

- Clear Work-RAM using multiple [[DMA examples#Fixed address DMA transfers|fixed address DMA transfers]] to WMDATA. The code should be inside the Reset Handler, **not** inside a subroutine and before any subroutines are called as the DMA transfers would override the stack. Ensure the DMA transfers do not override any memory you would like to survive a reset (ie, high-scores).
- Reset the PPU registers to a known good state
- Clear VRAM with a fixed address DMA transfer
- Clear CGRAM with a fixed address DMA transfer
- Populate OAM with values that will ensure all sprites are off-screen
- Jump to Main

Sample implementation:

```
ResetHandler:
    jml _FastRomReset       ; Change Program Bank to a FastROM bank
_FastRomReset:

    clc
    xce                     ; Switch to native mode

    ; Reset Status Flags
    ;   n (negative flag) clear
    ;   v (overflow flag) clear
    ;   m (memory size)   set   - 8 bit Accumulator
    ;   x (index size)    clear - 16 bit Index registers
    ;   d (decimal mode)  clear - decimal mode disabled
    ;   i (irq disable)   set   - IRQ interrupts disabled
    ;   z (zero flag)     clear
    ;   c (carry flag)    clear
    rep #$ff
    sep #$24

; 8 bit accumulator
; 16 bit index


    ldx #STACK_BOTTOM
    txs                     ; Set up Stack Pointer

    pea 0
    pld                     ; Reset Direct Page register to 0

    phk
    plb                     ; Set Data Bank to Program Bank
                            ; addr addressing mode can now access PPU/MMIO/DMA registers if PB is $00 - $3F or $80 - $BF


    stz NMITIMEN            ; Disable interrupts
    stz HDMAEN              ; Disable HDMA

    lda #$8f
    sta INIDISP             ; Disable screen


    ; Fill Work-RAM with zeros using two 64KiB fixed address DMA transfers to WMDATA
    stz WMADDL
    stz WMADDM
    stz WMADDH              ; Set VMDATA address

    lda #$08
    sta DMAP0               ; Fixed address transfer to a byte register

    lda #WMDATA & $ff
    sta BBAD0               ; DMA transfer to WMDATA

    ldx #.loword(WorkRamResetByte)
    stx A1T0
    lda #.bankbyte(WorkRamResetByte)
    sta A1B0                ; Set DMA source to `WorkRamResetByte`

    ldx #0
    stx DAS0                ; Transfer size = 64KiB

    lda #1
    sta MDMAEN              ; First DMA Transfer

    ; x = 0
    stx DAS0                ; Transfer size = 64KiB

    ; a = 1
    sta MDMAEN              ; Second DMA Transfer


    ; Reset PPU
    jsr ResetRegisters
    jsr ClearVRAM
    jsr ClearCGRAM
    jsr ResetOAM


    jml Main                ; Jump to Main


; Byte value to use when clearing Work-RAM
; (can be moved to any ROM bank)
WorkRamResetByte:
    db 0
```

## Reset Registers Routine

As the console lacks a boot ROM and most of the PPU registers start in an unknown state, the programmer will need to create a *Reset Registers* routine. This routine should be separate from the *Reset Handler* code to allow the programmer to reset the PPU registers to a known good state at the start of every level or scene.

The first registers to reset should be the NMITIMEN and HDMAEN registers. This will prevent Interrupts or HDMA from erroneously changing the PPU registers in the middle of the routine.

The following is a recommended list of registers to reset along with the values to reset them to.

```
Must reset first
    NMITIMEN    $4200 = 0       (disable Interrupts)
    HDMAEN      $420C = 0       (disable HDMA)

    INIDISP     $2100 = $8F     (enable Force Blank, full brightness)


CPU registers
    MEMSEL      $420D = 0       (set to 1 if FastROM)

    WRIO        $4201 = $FF


Objects
    OBJSEL      $2101 = 0

    OAMADDL     $2102 = 0
    OAMADDH     $2103 = 0       (disable OAM priority rotation)


Backgrounds
    BGMODE      $2105 = 0
    MOSAIC      $2106 = 0

    BG1SC       $2107 = 0
    BG2SC       $2108 = 0
    BG3SC       $2109 = 0
    BG4SC       $210A = 0

    BG12NBA     $210B = 0
    BG34NBA     $210C = 0


Scroll Registers
    BG1HOFS     $210D = 0
    BG1HOFS     $210D = 0       (set horizontal offset to 0)

    BG1VOFS     $210E = $FF
    BG1VOFS     $210E = $FF     (set vertical offset to -1)

    BG2HOFS     $210F = 0
    BG2HOFS     $210F = 0

    BG2VOFS     $2110 = $FF
    BG2VOFS     $2110 = $FF

    BG3HOFS     $2111 = 0
    BG3HOFS     $2111 = 0

    BG3VOFS     $2112 = $FF
    BG3VOFS     $2112 = $FF

    BG4HOFS     $2113 = 0
    BG4HOFS     $2113 = 0

    BG4VOFS     $2114 = $FF
    BG4VOFS     $2114 = $FF


VRAM Registers
    VMAIN       $2115 = $80     (VRAM word access, increment by 1, no remapping)


Mode 7
    M7SEL       $211A = 0       (no flipping or screen repeat)

    M7A         $211B = 0
    M7A         $211B = $01

    M7B         $211C = 0
    M7B         $211C = 0

    M7C         $211D = 0
    M7C         $211D = 0

    M7D         $211E = 0
    M7D         $211E = $01

    M7X         $211F = 0
    M7X         $211F = 0

    M7Y         $2120 = 0
    M7Y         $2120 = 0



Windows
    W12SEL      $2123 = 0
    W34SEL      $2124 = 0
    WOBJSEL     $2125 = 0
    WH0         $2126 = 0
    WH1         $2127 = 0
    WH2         $2128 = 0
    WH3         $2129 = 0
    WBGLOG      $212A = 0
    WOBJLOG     $212B = 0


Layer Enable
    TM          $212C = 0
    TS          $212D = 0
    TMW         $212E = 0
    TSW         $212F = 0


Color Math
    CGWSEL      $2130 = $30     (Color math disable region = everywhere)
    CGADSUB     $2131 = 0
    COLDATA     $2132 = $E0     (set Fixed color data to black)


Misc
    SETINI      $2133 = 0
```

## References

- [[SNES Development Manual]] Book 1, section 2-26-1: Register Clear (Initial Settings)
