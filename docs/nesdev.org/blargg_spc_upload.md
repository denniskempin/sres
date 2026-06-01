---
title: "Blargg SPC upload"
source_url: "https://snes.nesdev.org/wiki/Blargg_SPC_upload"
pageid: 110
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

This is an article about uploading SPC music rips for playback on real SNES hardware. It was written by Blargg[[1]](#cite_note-1) and contributed to the NESDev Wiki, migrated here now that a SNES Wiki exists.

## Introduction

This covers the important points of uploading an SPC music file into the SNES SPC-700 sound module. See [spc\_loader.c](http://blargg.8bitalley.com/misc/spc_loader.c) for tested code in C. Thanks to Anti Resonance for much of the original algorithm. I've mostly streamlined and tweaked it.

The main tasks to play an SPC file are

- Restore DSP registers
- Restore 64K RAM
- Restore CPU registers and other final values

## DSP registers

We can't just restore all the registers directly, because some have effects that depend on the others, or cause RAM to be overwritten. Those must be restored at the end. We make a copy of the DSP registers and modify as follows:

| Register | Value | Purpose |
| --- | --- | --- |
| KOFF | $00 | There are no notes to stop, and if any bits were set, it would stop the first notes. |
| KON | $00 | We don't want notes to start before we've restored DSP registers and RAM. |
| FLG | $60 | Enables mute, and disables echo write. We can't have echo writing over memory while restoring. |
| EDL | $00 | Sets length of echo buffer, but not until *after* the current echo pointer reaches the end of the buffer. Since EDL might have been $0F previously, we must assume that this new EDL of $00 might take as long as 1/4 second to take effect. We write DSP registers first, then the 64K RAM, so during the time taken to write RAM, the echo pointer will have looped and taken on a new loop size of zero. To take less than 1/4 second uploading the 64K RAM, you'd have to upload each byte in less than 4 S-SMP cycles, an impossibility. After all this, with EDL *and* the internal echo position reliably at 0, we then restore the proper EDL from the SPC file, knowing exactly where it will first start writing echo samples. |

With these modifications, we upload them to the DSP via a short SPC program:

```
       .org $0002
       mov x,#dsp_regs     ; pointer to table
       mov y,#0
next:  mov a,(x)+          ; copy to DSP
       mov $F2,y
       mov $F3,a
       inc y
       bpl next            ; stop when y > 127
       jmp !$FFC0          ; rerun bootrom
dsp_regs:
       .res 128            ; modified values to load
```

## 64K RAM

Again, we can't restore every byte of RAM directly, because some of the I/O locations have effects that we can't have just yet. We make a copy of the 64K RAM and modify it as follows:

| Address | Value | Purpose |
| --- | --- | --- |
| $0000 | $00 | The bootloader uses these locations to store a pointer to the current destination page, so when we overwrite them, we must overwrite with the same values. |
| $0001 | $00 |
| $00F0 | $0A | Test register |
| $00F1 | $80 | Enables IPL ROM and stops timers. The IPL ROM must be enabled, because the bootloader is there. Note that having it enabled doesn't prevent storing values into RAM at $FFC0-$FFFF, so we can load those just fine. |
| $00F2 | $6C | DSP address |
| $00F3 | $60 | We just rewrite FLG with $60 again here, because we have to write something to the DSP. |
| $00F4 | $F4 | This is critical. This is the value the loader would have written here normally. If we write a different value, it will disrupt communication and hang the loader, but only very rarely, because the window of opportunity is small. |
| $00FD | $00 | Not really necessary to be zero, since the timer out registers are only readable, but why not. |
| $00FE | $00 |
| $00FF | $00 |

Some final patching is necessary before sending the RAM.

## Final restoration

With those values patched, we still need to insert some code to restore the final registers. We need to find some free space in RAM. Many have long stretches of $FF bytes, a good candidate. If the echo buffer is enabled, it could be used, though this will introduce a slight click. Some other SPC files have runs of the repeating pattern of 32 $FF bytes followed by 32 $00 bytes, aligned to a 32-byte boundary.

Once we've found space, we can patch in the following code. The notation spc.ram [n] refers to the RAM in the unpatched SPC file, spc.dsp [reg] to the unpatched DSP value, and spc.<register> to the processor register value in the SPC file.

```
   ; Restore first two bytes of RAM
   mov $00,#spc.ram [0]
   mov $01,#spc.ram [1]
   
   ; Restore CPU registers
   mov x,#spc.sp - 1   ; See below for why the -1
   mov sp,X
   mov y,#spc.y
   mov x,#spc.x
   mov a,#spc.a
   
   ; Restore SMP/DSP registers
   mov $F1,#spc.ram [$F1] AND $CF    ; Control
```

Note that we clear bits 4 and 5, because we don't want the input ports being cleared.

```
   mov $F3,#spc.dsp [FLG]
```

Now we restore the original FLG value, which might enable echo writing. Note that we don't need to set the DSP address, as it's already been set during RAM restore. Since we know the echo pointer will now be at the beginning of the echo buffer, writing is safe. If we put this final loader in the echo buffer, we should NOT place it at the very beginning, otherwise we'll be chased by the echo overwriting as we execute here, and might lose the race. So we should be at a small offset like 1024 (small enough to fit in case EDL is only $01).

```
   mov $F2,#$7D    ; EDL
   mov $F3,#spd.dsp [EDL]
   mov $F2,#$4C    ; KON
   mov $F3,#spd.dsp [KON]
```

No need to restore the original KOFF.

```
   mov $F2,#spc.ram [$F2]   ; DSP Address
```

Don't forget this, as the code might be expecting the address to already be set to something in particular.

```
   ; Restore PSW and PC
   pop psw
   mov spc.sp,#saved byte from stack
```

We pop the saved PSW we push on the stack before executing this final restore code. That saved PSW has the P flag set. Then we restore that byte on the stack to whatever it originally was. The SPC file could have been depending on that byte, even though it's outside the current stack. The P flag is set, so we can use direct-page addressing here.

```
   setp or clrp    ; depending on what's set in PSW in SPC file
```

Now we restore the P flag to what it should be.

```
   jmp !spc.pc
```

And finally jump to wherever the SPC was executing when the SPC file was captured.

The above code relies on the PSW being on the stack, so we must push that ourselves, saving the old byte so we can restore that too.

With the 64K RAM patched, we upload that to the SPC and execute our patch inside it, and away it goes. Immediately after that, we must do one final restoration: write the final values to the input ports that were there when the SPC was captured.

## References

1. [↑](#cite_ref-1) [NESDev Wiki Blargg SPC Upload History](https://www.nesdev.org/w/index.php?title=Blargg_SPC_Upload&action=history)
