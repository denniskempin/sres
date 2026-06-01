---
title: "VBlank interrupts"
source_url: "https://snes.nesdev.org/wiki/VBlank_interrupts"
pageid: 82
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

As the majority of the PPU only be accessed during the Vertical Blanking Period while the display is active, careful consideration is required to ensure all PPU writes occur at the correct time.

Enter the VBlank routine. The main-loop would process the display-frame, writing the data that is to be transferred to the PPU into a queue, buffer or shadow variable. Later a VBlank interrupt would interrupt the main-loop and execute the VBlank routine, transferring data from the queue/buffer/shadow-variable to the PPU.

VBlank interrupts are NMI (Non-Maskable Interrupt) interrupts that are activated when the Vertical Blanking period begins and the *vblank NMI enable* bit of [[MMIO registers#NMITIMEN|NMITIMEN]] is set. When the interrupt is activated, the CPU will execute the NMI Interrupt Service Routine (ISR) located in the [[CPU vectors|NMI Interrupt Vector]].

It is recommended that the NMI ISR:

- Jumps to a FastROM address (if the game uses FastROM)
- Pushes the CPU registers to stack
  - The NMI ISR should save and restore all CPU registers
  - The A, X and Y registers must be pushed and popped in 16 bit mode
  - There is no need to push the P register (status flags) to stack. The 65c816 will automatically push and pop the P register for us.
- Resets the DB and DP registers
- Invokes the VBlank routine
- Pops the CPU registers from stack
- Exits the ISR with the rti instruction

It is vitally important that the VBlank routine is **not** executed in the middle of PPU setup or level setup, as it could clobber the PPU registers (most commonly the CGRAM/OAM/VRAM addresses). A simple way to do this is to use a byte flag to indicate if the VBlank routine is to be executed or not. When the main-loop has finished processing the display-frame, the flag would be set and the NMI ISR knows to execute the VBlank routine. One advantage of this design is that VBlank routine will not be executed during lag-frames so there is no need to litter the code with mutexes.

Alternatively, VBlank interrupts could be disabled during level setup but careful consideration for lag-frames will be required in the VBlank routine.

VBlank interrupts are also useful for timers and clocks. A frame counter can be created by incrementing a variable on every VBlank interrupt. Dividing this counter by 50 or 60, depending on the state of the *PAL/NTSC* bit of [[PPU registers#STAT78|STAT78]], will convert the frame counter into seconds clock.

It is highly recommended that the main-loop maintains exclusive access to the Work-RAM registers (WMADD, WMDATA), Multiplication registers and Division registers. Unexpected behaviour can occur if an IRQ or NMI ISR modifies them in the middle of the main-loop.

TODO: Mention the amount of DMA time available.

TODO: VBlank overruns and how to detect them. Mention Mesen-S's event viewer.

TODO: Should I include examples of buffers, queues and shadow variables? (separate from the Sample implementation below)

Sample implementation:

```
; The following uses two variables, both of which MUST be in Low-RAM:
;
;   vBlankFlag       u8 - byte flag.  If zero, the VBlank routine will not be executed.
;                         Must only be accessed by `EnableVBlankInterrupts`, `WaitForVBlank` or `NmiIsr`.
;   frameCounter    u32 - Counts NMI interrupts, useful for clocks and timers.
;



; Enable NMI Interrupts.
;
; This routine should only be called during setup.
;
; This routine will disable IRQ interrupts.
;
; DB access registers
.a8
.i16
.proc EnableVBlankInterrupts
    ; Clear `vBlankFlag`, prevent VBlank routine from executing until the next `WaitForVBlank` call
    stz vBlankFlag

    ; Clear the RDNMI VBlank flag
    lda RDNMI

    ; Enable vblank NMI interrupts and Joypad auto-read
    lda #$81
    sta NMITIMEN

    rts
.endproc



; Waits until the VBlank routine has been processed.
;
; REQUIRES: NMI enabled
;
; DB access Low RAM ($00-$3f, $7e, $80-$bf)
; a unknown
; i unknown
.proc WaitForVBlank
    php

    sep #$20
.a8

    ; Set `vBlankFlag`, VBlank routine will be executed on the next NMI interrupt
    lda #1
    sta vBlankFlag


    ; Loop until `vBlankFlag` is clear
    Loop:
        wai

        lda vBlankFlag
        bne Loop


    plp
    rts
.endproc



; NMI Interrupt Service Routine
;
;
; This ISR will preform the necessary steps to setup the CPU in preparation for the `VBlankMacro`.
;
; The `VBlankMacro` will only be executed if the Main Loop is inside the `WaitForVBlank` subroutine.
;
; The `VBlankMacro` will be invoked with the following CPU state:
;   * 8 bit Accumulator
;   * 16 bit Index
;   * DB = $80 or $00
;   * DP = 0
;
; This ISR will also increment the `frameCounter` on every NMI interrupt.
;
;
; a unknown
; i unknown
; DB unknown
; DP unknown
.proc NmiIsr
    ; Interrupts start in Program Bank 0.
    ; Switch to a FastROM bank
    ; (Assumes NmiIsr is in bank 0x80. Should be removed if code is SlowROM)
    jml FastNmiIsr
FastNmiIsr:


    ; Push CPU registers to stack
    ;
    ; There is no need to push P to the stack, the 65c816 does that automatically on interrupt.
    ;
    ; Accumulator and Index registers must be saved in 16 bit mode.
    ;   * Register sizes are unknown
    ;   * Using a 8 bit Index will clobbers the high byte of the index
    ;   * Using a 16 bit Accumulator will clobber the high byte of A (which may be used by the program, even in 8 bit A mode)
    ;
    ; The DB and DP registers are in an unknown state. Push then to stack, so they can be changed later.
    ;
    rep #$38            ; 16 bit A, 16 bit I, decimal mode disabled
.a16
.i16
    pha
    phx
    phy
    phb
    phd


    ; Reset DB and DP registers

    .assert(.bankbyte(*) & 0x7f = 0), lderror, "NmiIsr is not in bank $00 or $80"
    phk
    plb
; DB = 0x80 or 0x00 (can access Low-RAM, PPU registers, DMA registers and MMIO registers)

    lda #0
    tcd
; DP = 0


    sep #$20
.a8

    ; Skip VBlank routine if `vBlankFlag` is zero.
    ; (prevents the VBlank routine from executing during a lag frame or while the MainLoop is loading data to PPU)
    lda vBlankFlag
    bne ExecuteVBlankRoutine
        jmp SkipVBlankRoutine
    ExecuteVBlankRoutine:

        ; Execute VBlank routine
        VBlankMacro


        .assert .asize = 8, error, "VBlank macro must exit with 8 bit A"

        ; clear `vBlankFlag`
        stz vBlankFlag

    SkipVBlankRoutine:



    rep #$30
.a16
.i16

    ; Increment 32 bit frameCounter
    ; (Always increment frameCounter on an NMI interrupt)
    inc frameCounter
    bne :+
        inc frameCounter + 2
    :


    ; Restore CPU registers
    .assert .asize = 16 && .isize = 16, error, "Invalid register sizes"
    pld
    plb
    ply
    plx
    pla


    ; Return from interrupt
    rti
.endproc
```

## See Also

- [[VBlank routine]]
