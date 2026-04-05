# SNES DMA and HDMA Notes

#### Starting HDMA midframe

Activating HDMA (via port 420Ch) should be usually done only during VBlank (so that the hardware can reload the HDMA registers automatically at end of Vblank).

Otherwise, when starting HDMA midframe (outside of Vblank), then one must manually reload the HDMA registers. For example, during Vblank, init HDMA registers as so:

```text
  420Ch=00h ;stop all HDMA channels
  43x0h=02h ;transfer two bytes to [bbus+0], and [bbus+0]
  43x1h=88h ;dummy bbus destination address (unused port 2188h)
  42x4h=abus.src.bank       ;with <abus.src> pointing to "02h,77h,55h,00h"
  42x8h=abus.src.offs.lo    ;ie. repeat/pause 2 scanlines (02h), transfer
  42x9h=abus.src.off.hi     ;one data unit (77h,55h), and after the pause,
  42xAh=01h ;remain count   ;finish the transfer (00h).
```

The HDMA starting point seems to depend on whether any HDMA channels were already active in the current frame. The two scenarios (each with above example values) are:

Case 1 - (420Ch was still zero) - First HDMA in current frame Start one (or more) HDMA channel(s) somewhere midframe (eg. in line 128), and watch the src/remain values in 43x8h..43xAh. This will behave as expected (src increases, and remain decreases from 02h downto 00h).

Case 2 - (420Ch was already nonzero) - Further HDMA in current frame Start another HDMA channel (some scanlines later, after the above transfer).

This will behave differently: It's decreasing remain count in 43xAh from 55h downwards, ie. the HDMA does apparently start with "do_transfer=1" (for whatever reason), causing it to transfer 02h,77h as data, and then fetch 55h as repeat count for next scanlines.

Note: One game does start HDMAs midframe is Super Ghouls 'N Ghosts.

[XXX Case 2 may need more research, and isn't yet accurately emulated in no$sns]

#### External DMA

The SNES Cartridge Slot doesn't have any special DRQ/DACK pins for DMA handshaking purposes; DMA from cartridge memory is just implemented as normal memory access (so the cartridge must respond to DMAs within normal memory access time; which is fixed 8 master cycles per byte for DMA).

Unknown if the data decompression chips (S-DD1 and SPC7110) are implemented the same way, or if they do use any "hidden" DMA handshaking mechanisms (they might be too slow to supply bytes within 8 master cycles, and at least one of them seems to decode DMA channel numbers; possibly by sensing writes to 420Bh?).
