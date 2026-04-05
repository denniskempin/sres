# SNES Cart OBC1 (OBJ Controller) (1 game)

The OBC1 is a 80pin OBJ Controller chip from Nintendo, used by only one game:

```text
  Metal Combat: Falcon's Revenge (1993) Intelligent Systems/Nintendo
  (Note: the game also requires a Super Scope lightgun)
```

#### OBC1 I/O Ports

```text
  7FF0h OAM Xloc = [Base+Index*4+0]  (R/W)
  7FF1h OAM Yloc = [Base+Index*4+1]  (R/W)
  7FF2h OAM Tile = [Base+Index*4+2]  (R/W)
  7FF3h OAM Attr = [Base+Index*4+3]  (R/W)
  7FF4h OAM Bits = [Base+Index/4+200h].Bit((Index AND 3)*2+0..1) (R?/W)
  7FF5h Base for 220h-byte region (bit0: 0=7C00h, 1=7800h)
  7FF6h Index (OBJ Number) (0..127)
  7FF7h Unknown (set to 00h or 0Ah) (maybe SRAM vs I/O mode select)
```

Other bytes at 6000h..7FFFh contain 8Kbyte battery-backed SRAM (of which, 7800h..7A1Fh and 7C00h..7E1Fh can be used as OBJ workspace).

#### Notes

Port 7FF0h-7FF3h/7FF5h are totally useless. Port 7FF4h/7FF6h are eventually making it slightly easier to combine the 2bit OAM fragments, though putting a huge 80pin chip into the cartridge for merging 2bit fragments is definetly overcomplicated.

As far as known, the Index isn't automatically incremented. Port 7FF4h does read-modify-write operations which may involve timing restrictions (?), or, modify-write (when prefetching data on 7FF6h writes) which may come up with out-dated-prefetch effects.

Reading from 7FF4h does reportedly return the desired BYTE, but WITHOUT isolating & shifting the desired BITS into place?

Setting Index bits7+5 does reportedly enable SRAM mapping at 6000h..77FFh?

ROM is reportedly mapped to bank 00h..3Fh, and also to bank 70h..71h? Maybe that info just refers to SRAM not being mapped to that region (as it'd be in some other LoROM cartridges).

#### PCB "SHVC-2E3M-01"

Contains six chips and a battery. The chips are: Two 1MB ROMs, MAD-1, OBC1, CIC, 8K SRAM. All chips (except MAD-1) are SMD chips.
