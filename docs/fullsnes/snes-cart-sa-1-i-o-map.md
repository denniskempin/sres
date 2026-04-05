# SNES Cart SA-1 I/O Map

#### SA-1 I/O Map (Write Only Registers)

```text
  Port  Side  Name  Reset Expl.
  2200h SNES  CCNT  20h   SA-1 CPU Control (W)
  2201h SNES  SIE   00h   SNES CPU Int Enable (W)
  2202h SNES  SIC   00h   SNES CPU Int Clear  (W)
  2203h SNES  CRV   -     SA-1 CPU Reset Vector Lsb (W)
  2204h SNES  CRV   -     SA-1 CPU Reset Vector Msb (W)
  2205h SNES  CNV   -     SA-1 CPU NMI Vector Lsb (W)
  2206h SNES  CNV   -     SA-1 CPU NMI Vector Msb (W)
  2207h SNES  CIV   -     SA-1 CPU IRQ Vector Lsb (W)
  2208h SNES  CIV   -     SA-1 CPU IRQ Vector Msb (W)
  2209h SA-1  SCNT  00h   SNES CPU Control (W)
  220Ah SA-1  CIE   00h   SA-1 CPU Int Enable (W)
  220Bh SA-1  CIC   00h   SA-1 CPU Int Clear  (W)
  220Ch SA-1  SNV   -     SNES CPU NMI Vector Lsb (W)
  220Dh SA-1  SNV   -     SNES CPU NMI Vector Msb (W)
  220Eh SA-1  SIV   -     SNES CPU IRQ Vector Lsb (W)
  220Fh SA-1  SIV   -     SNES CPU IRQ Vector Msb (W)
  2210h SA-1  TMC   00h   H/V Timer Control (W)
  2211h SA-1  CTR   -     SA-1 CPU Timer Restart (W)
  2212h SA-1  HCNT  -     Set H-Count Lsb (W)
  2213h SA-1  HCNT  -     Set H-Count Msb (W)
  2214h SA-1  VCNT  -     Set V-Count Lsb (W)
  2215h SA-1  VCNT  -     Set V-Count Msb (W)
  2216h -     -     -     -
  2220h SNES  CXB   00h   MMC Bank C - Hirom C0h-CFh / LoRom 00h-1Fh (W)
  2221h SNES  DXB   01h   MMC Bank D - Hirom D0h-DFh / LoRom 20h-3Fh (W)
  2222h SNES  EXB   02h   MMC Bank E - Hirom E0h-EFh / LoRom 80h-9Fh (W)
  2223h SNES  FXB   03h   MMC Bank F - Hirom F0h-FFh / LoRom A0h-BFh (W)
  2224h SNES  BMAPS 00h   SNES CPU BW-RAM Mapping to 6000h-7FFFh (W)
  2225h SA-1  BMAP  00h   SA-1 CPU BW-RAM Mapping to 6000h-7FFFh (W)
  2226h SNES  SBWE  00h   SNES CPU BW-RAM Write Enable (W)
  2227h SA-1  CBWE  00h   SA-1 CPU BW-RAM Write Enable (W)
  2228h SNES  BWPA  FFh   BW-RAM Write-Protected Area (W)
  2229h SNES  SIWP  00h   SNES I-RAM Write-Protection (W)
  222Ah SA-1  CIWP  00h   SA-1 I-RAM Write-Protection (W)
  222Bh -     -     -     -
  2230h SA-1  DCNT  00h   DMA Control (W)
  2231h Both  CDMA  00h   Character Conversion DMA Parameters (W)
  2232h Both  SDA   -     DMA Source Device Start Address Lsb (W)
  2233h Both  SDA   -     DMA Source Device Start Address Mid (W)
  2234h Both  SDA   -     DMA Source Device Start Address Msb (W)
  2235h Both  DDA   -     DMA Dest Device Start Address Lsb (W)
  2236h Both  DDA   -     DMA Dest Device Start Address Mid (Start/I-RAM) (W)
  2237h Both  DDA   -     DMA Dest Device Start Address Msb (Start/BW-RAM)(W)
  2238h SA-1  DTC   -     DMA Terminal Counter Lsb (W)
  2239h SA-1  DTC   -     DMA Terminal Counter Msb (W)
  223Ah -     -     -     -
  223Fh SA-1  BBF   00h   BW-RAM Bit Map Format for 600000h-6FFFFFh (W)
  224xh SA-1  BRF   -     Bit Map Register File (2240h..224Fh) (W)
  2250h SA-1  MCNT  00h   Arithmetic Control (W)
  2251h SA-1  MA    -     Arithmetic Param A Lsb (Multiplicand/Dividend) (W)
  2252h SA-1  MA    -     Arithmetic Param A Msb (Multiplicand/Dividend) (W)
  2253h SA-1  MB    -     Arithmetic Param B Lsb (Multiplier/Divisor) (W)
  2254h SA-1  MB    -     Arithmetic Param B Msb (Multiplier/Divisor)/Start (W)
  2255h -     -     -     -
  2258h SA-1  VBD   -     Variable-Length Bit Processing (W)
  2259h SA-1  VDA   -     Var-Length Bit Game Pak ROM Start Address Lsb (W)
  225Ah SA-1  VDA   -     Var-Length Bit Game Pak ROM Start Address Mid (W)
  225Bh SA-1  VDA   -     Var-Length Bit Game Pak ROM Start Address Msb & Kick
  225Ch -     -     -     -
  2261h -     -     -     Unknown/Undocumented (Jumpin Derby writes 00h)
  2262h -     -     -     Unknown/Undocumented (Super Bomberman writes 00h)
```

#### SA-1 I/O Map (Read Only Registers)

```text
  Port  Side  Name  Reset Expl.
  2300h SNES  SFR   SNES CPU Flag Read (R)
  2301h SA-1  CFR   SA-1 CPU Flag Read (R)
  2302h SA-1  HCR   H-Count Read Lsb / Do Latching (R)
  2303h SA-1  HCR   H-Count Read Msb (R)
  2304h SA-1  VCR   V-Count Read Lsb (R)
  2305h SA-1  VCR   V-Count Read Msb (R)
  2306h SA-1  MR    Arithmetic Result, bit0-7   (Sum/Product/Quotient) (R)
  2307h SA-1  MR    Arithmetic Result, bit8-15  (Sum/Product/Quotient) (R)
  2308h SA-1  MR    Arithmetic Result, bit16-23 (Sum/Product/Remainder) (R)
  2309h SA-1  MR    Arithmetic Result, bit24-31 (Sum/Product/Remainder) (R)
  230Ah SA-1  MR    Arithmetic Result, bit32-39 (Sum) (R)
  230Bh SA-1  OF    Arithmetic Overflow Flag (R)
  230Ch SA-1  VDP   Variable-Length Data Read Port Lsb (R)
  230Dh SA-1  VDP   Variable-Length Data Read Port Msb (R)
  230Eh SNES  VC    Version Code Register (R)
```

#### Reset

Port 2200h = 20h. Port 2228h = FFh. Ports 2220h-2223h = 00h,01h,02h,03h. Ports 2201h-2202h, 2209h-220Bh, 2210h, 2224h-2227h, 2229h-222Ah, 2230h-2231h, 223Fh, 2250h = 00h. Ports 2203h-2208h, 220Ch-220Fh, 2211h-2215h, 2232h-2239h, 2240h-224Fh, 2251h-2254h, 2258h-225Bh = N/A.
