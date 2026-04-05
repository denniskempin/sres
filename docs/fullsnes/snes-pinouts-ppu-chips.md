# SNES Pinouts PPU Chips

S-PPU1, 5C77

```text
  1     ?      TST1 (GND)
  2     ?      TST0 (GND)
  3     CPU    /PARD
  4     CPU    /PAWR
  5-12  CPU    PA7-PA0 (main cpu b-bus)
  13    Supply VCC
  14-21 CPU    D7-D0 (main cpu)
  22    Supply GND
  23    ?      HVCMODE (GND)
  24    Mode   PALMODE (VCC=PAL or GND=NTSC)
  25    ?      /MASTER (GND)
  26    ?      /EXTSYNC (VCC)
  27    ?      NC (GND)
  28-35 SRAM   DH0-DH7 (sram data bus high-bytes)
  36    Supply VCC
  37-44 SRAM   DL0-DL7 (sram data bus low-bytes)
  45    Supply GND
  46    SRAM   VA15 (NC)  (would be for 64K-word RAM, SNES has only 32K-words)
  47    SRAM   VA14       (sram address bus for upper/lower 8bit data)
  48-61 SRAM   VAB13-VAB0 (sram address bus for upper 8bit data)
  62    Supply VCC
  63-76 VRAM   VAA13-VAA0 (sram address bus for lower 8bit data)
  77    Supply GND
  78    VRAM   /VAWR (sram write lower 8bit data)
  79    VRAM   /VBWR (sram write upper 8bit data)
  80    VRAM   /VRD  (sram read 16bit data)
  81    Supply VCC
  82-85 PPU    CHR3-CHR0      ;\
  86-87 PPU    PRIO1-PRIO0    ;
  88-90 PPU    COLOR2-COLOR0  ;/
  91           /VCLD (20ms high, 60us low)  (LOW during V=0)
  92           /HCLD (60us high, 0.2us low) (low during 11th dot-cycle
                                             of the 15-cycle color burst)
  93           /5MOUT (shortcut with pin 97, /5MIN) (and to PPU2, /5MIN)
                           8 clks = 7.5*0.2us = 5.333MHz
  94           /OVEP (always high?) (to PPU2 /OVER1 and /OVER2)
  95           FIELD (NTSC: 30Hz, PAL: 25Hz) (signalizes even/odd frame)
  96    Supply GND
  97           /5MIN (shortcut with pin 93, /5MOUT)  (as above 5mout)
  98    PPU    /RESET (from PPU2 /RESOUT0)
  99    ?      TST2 (GND)
  100   System XIN (21MHz)
```

S-PPU2, 5C78

```text
  1     Video  /BURST    (LOW for 15 dot-clocks, thereof 1st dot is LONG)
  2     ?      /PED (NC) (ca. 15kHz, hblank related, 50us high, 10us low)
  3     Video  3.58M (to NTSC encoder)    5 clks = 1.4us
  4     ?      /TOUMEI (NC) (LOW during V-Blank and H-Blank) (or vram access?)
  5     Supply VCC
  6     CPU    /PAWR
  7     CPU    /PARD
  8-15  CPU    D7-D0
  16    Supply GND
  17-24 CPU    PA7-PA0
  25    CPU    HBLANK (for CPU h/v-timers)
                high during last some pixels, right border HSYNC,lead,burst
                low during last some burst clks, left border, and most pixels
  26    CPU    VBLANK (for CPU h/v-timers)
                high during VBLANK (line 225-261)
                low during prepare (line 0) and picture (line 1-224)
  27           /5MOUT (via 100 ohm to DOTCK on Expansion Port Pin22)
  28    System /RESOUT1 (via 1K to CPU, APU, Cartridge, Expansion, etc.)
  29    Joy    EXTLATCH (Lightpen signal)
  30    Mode   PALMODE (VCC=PAL or GND=NTSC)
  31    System XIN (21MHz)
  32    Supply VCC
  33    PPU    /RESOUT0 (to PPU1 /RESET)
  34    CIC    /RESET (from CIC Lockout chip & Reset Button)
  35    Supply GND
  36           FIELD (NTSC: 30Hz, PAL: 25Hz) (signalizes even/odd frame)
  37           /OVER1   ??
  38           /5MIN (from PPU1)
  39           /HCLD (low during 11th dot-cycle of the 15-cycle color burst)
  40           /VCLD (LOW during V=0)
  41-43 PPU    OBJ0-OBJ2 (COLOR0-COLOR2)
  44-45 PPU    OBJ3-OBJ4 (PRIO0-PRIO1)
  46-49 PPU    OBJ5-OBJ8 (CHR0-CHR3)
  50           /OVER2
  51-58 SRAM   VDB0-VDB7 (sram data upper 8bit)
  59    Supply VCC
  60-67 SRAM   VDA0-VDA7 (sram data lower 8bit)
  68    Supply GND
  69-76 SRAM   EXT0-EXT7 (sram data upper 8bit) (shortcut with VDB0-VDB7)
  77-82 ?      TST0-TST5 (NC) (always low?)
  83    Supply VCC
  84-89 ?      TST6-TST11 (NC) (always low?)
  90-93 ?      TST12-TST15 (GND)
  94    Supply AVCC (VCC)
  95-97 Video  R,G,B (Analog RGB Output)
  98    ?      HVCMODE (GND)
  99    Supply GND
  100   Video  /CSYNC (normally LOW during Hsync, but inverted during Vsync)
```

CPUN-A (160pin chip with CPU and PPU1 and PPU2 in one chip) Used in newer cost-down SNES consoles. Pinouts unknown.
