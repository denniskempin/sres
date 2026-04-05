# SNES Cart SA-1 Character Conversion

#### Character Conversion Types

```text
  Conversion  DMA-Transfer     Source / Pixel-Format
  Type 1      Automatic        BW-RAM, Packed Pixels, Bitmap Pixel Array
  Type 2      Semi-Automatic   CPU, Unpacked Pixels, 8x8 Pixel Tiles
```

Both Conversion types are writing data to a temporary buffer in I-RAM:

```text
  I-RAM buffer 32/64/128 bytes (two 8x8 tiles at 2bit/4bit/8bit color depth)
```

From that buffer, data is forwarded to SNES (via a simultanously executed SNES DMA, ie. via ports 43xxh).

Character Conversion 1 - Automatically Convert Packed BW-RAM Pixels Can be used only if the cartridge DOES contain BW-RAM (most or all do so).

First, do this on SA-1 side:

```text
  Set DCNT (Port 2230h) set to Char Conversion Type 1   (...and no DMA-enable?)
```

Then do following on SNES side:

```text
  Set SDA (Port 2232h-2234h)=BW-RAM offset, align by (bytes/char)*(chars/line)
  Set CDMA (Port 2231h) = store bits/pixel and chars/line
  Set DDA (Port 2235h-2236h)=I-RAM offset, align (bytes/char)*2 (2237h=unused)
  Wait for SFR.Bit5 (Port 2300h) Char_DMA_IRQ (=first character available)
  Launch SNES-DMA via Port 43xxh from "Virtual BW-RAM?" to PPU-VRAM
    (this can transfer the WHOLE bitmap in one pass)
```

Finally, after the SNES-DMA has finished, do this on SA-1 side:

```text
  Set CDMA.Bit7=1 (Port 2231h) - terminate SA-1 DMA
    (that stops writing to I-RAM on SA-1 side)
    (and stops tile-data to be mapped to 400000h-43FFFFh on SNES-side)
```

During conversion, the SA-1 can execute other program code (but waits may occur on BW-RAM and I-RAM accesses). The SNES CPU is paused (by the DMA) for most of the time, except for the time slots shortly before/after the DMA; in that time slots, the SNES may access I-RAM, but may not access BW-RAM.

Conversion 1 is used by Haruka Naru Augusta 3 and Pebble Beach no Hotou.

Character Conversion 2 - Semi-Automatic Convert Unpacked CPU Pixels First, do this on SA-1 side:

```text
  Set DCNT (Port 2230h) set to Char Conversion Type 2 and set DMA-enable
  Set CDMA (Port 2231h) = store bits/pixel (chars/line is not used)
  Set DDA (Port 2235h-2236h)=I-RAM offset, align (bytes/char)*2 (2237h=unused)
```

Then repeat for each character:

```text
  for y=0 to 7, for x=0 to 7, [2240h+x+(y and 1)]=pixel(x,y), next x,y
  On SNES side: Transfer DMA from 1st/2nd I-RAM buffer half to VRAM or WRAM
```

Finally,

```text
  Set DCNT.Bit7=0 (Port 2230h) - disable DMA
```

Conversion 2 is used by Haruka Naru Augusta 3 and SD Gundam G NEXT.
