# SNES Cart Copiers - CCL (Supercom & Pro Fighter)

Below is for Supercom Partner & Pro Fighter models from CCL (China Coach Limited). See the Front Fareast chapter for their earlier Super Magicom models (which were produced by Front & CCL), and also for Supercom Pro 2 (which was made by CCL alone, but still used the Front-like I/O ports).

Pro Fighter 1993 by H.K. / Supercom Partner A Ports are somewhat based on the Front design (BIOS is expanded from 8K at E000h-FFFFh to 16K at C000h-FFFFh, accordingly FDC Ports C000h-C007h are moved to 2800h-2807h, and Parallel Port Ports C008h-C009h are simply removed. Ports at E00xh are somehow changed, but might be still similar to the Front design (?)

```text
  2800.R   FDC MCS General Purpose Input (bit7,bit6 used)
  2802.W   FDC MCS Motor Control (set to 00h,29h,2Dh)
  2804.R   FDC MCS Main Status
  2805.RW  FDC MCS Command/Data Status
  2807.W   FDC MCS Transfer Rate/Density (set to 0..3)
  Below 2808-2810 only in newer "Pro Fighter Q"
   2808.R   Parallel Port Data (bit0-7)
   2809.W   Parallel Port Data (4bit or 8bit?)
   2810.R   Parallel Port Busy (bit5)
    (there seem to be 4bit & 8bit parallel port modes supported, one of them
    also WRITING to 2808h, and in some cases reading "FDC" register 2800 looks
    also parallel port DATA and/or BUSY related)
  Again changed for Double Pro Fighter
   2803.R   Parallel Port Busy (bit7)
   2808.R   Parallel Port Data (bit0-7)
   2809.W   Parallel Port Data (4bit or 8bit?)
   2804 =FDC DATA   ;\swapped ! (unlike older "non-double" models)
   2805 =FDC STAT   ;/
   280x =other ports in this region may be changed, too ?
   004800    ROM (from offset 8800-9FFF) (contains program code)
   014800    ROM (from offset A800-BFFF) (contains character set)
   E00x
   E800+x
   Note: Having BIOS portions mapped to the fast 3.58MHz region at 4800h-5FFFh
         was probably done unintentionally; this would require 120ns EPROMs,
         whilst some Double Pro Fighter boards are fitted with 200ns EPROMs
         (which are stable at 2.68MHz only, and may cause crashes, or charset
         glitches in this case)
   Double Pro Fighter BIOS is 64Kbytes:
     0000-3FFF  Genesis/Z80 BIOS
     4000-7FFF  Same content as 0000-3FFF
     8000-87FF  Unused (zerofilled)
     8800-9FFF  SNES BIOS (6K mapped to 004800-005FFF)
     A000-A7FF  Unused (zerofilled)
     A800-BFFF  SNES BIOS (6K mapped to 014800-015FFF)
     C000-FFFF  SNES BIOS (16K mapped to 00C000-00FFFF)
  7000.R
  A000.RW               ;7000-related
  C000-FFFF.R BIOS ROM (16Kbytes)
  E002.W   set to 00h   ;7000-related
  E003.W   set to BFh ;then compares BFFD with BFFC,BFFA,BFFB,BFEA,BFEB
  E00C.W   set to 00h   ;7000-related
  E00E.W   set to E0h
  008000.RW   DRAM detection?
  208000.RW   DRAM detection?
  408000.RW   DRAM detection?
  608000.RW   DRAM detection?
```
