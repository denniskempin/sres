# SNES Cart Nintendo Power - I/O Ports

Nintendo Power I/O Map  Write registers:

```text
  2400h        - Command
  2401h        - Extra parameter key (used only for wakeup command)
  2402h..2407h - Unknown/unused
```

Read registers (before wakeup):

```text
  2400h..2407h - Fixed 7Dh
```

Read registers (after wakeup):

```text
  2400h        - Fixed 2Ah
  2401h        - Status
  2402h..2403h - Fixed 2Ah
  2404h        - Mapping Info: ROM/RAM Size         ;\these four bytes are
  2405h..2406h - Mapping Info: SRAM Mapping related ; initialized from the
  2407h        - Mapping Info: ROM/RAM Base         ;/hidden flash sector
```

#### Port 2401h = Status (R)

```text
  0-1 zero
  2   release /WP state    (set by CMD_02h, cleared by CMD_03h)
  3   disable ROM reading? (set by CMD_21h, cleared by CMD_20h)
  4-7 Selected Slot (0=Menu/File0, 1..15=File1..15) (via CMD_8xh)
```

#### Port 2404h = Size (R)

```text
  0-1 SRAM Size (0=2K, 1=8K, 2=32K, 3=None) ;ie. 2K SHL (N*2)
  2-4 ROM Size (0=512K, 2=1.5M, 5=3M, 7=4M) ;ie. 512K*(N+1)
  5   Maybe ROM Size MSB for carts with three FLASH chips (set for HIROM:ALL)
  6-7 Mode (0=Lorom, 1=Hirom, 2=Forced HIROM:MENU, 3=Forced HIROM:ALL)
```

#### Port 2407h = Base (R)

```text
  0-3 SRAM Base in 2K units
  4-7 ROM Base in 512K units (bit7 set for HIROM:MENU on skaman's blank cart)
```

Port 2405h,2406h = SRAM Mapping Related (R) The values for port 2405h/2406h are always one of these three sets, apparently related to SRAM mapping:

```text
  29,4A for Lorom with SRAM
  61,A5 for Hirom with SRAM
  AA,AA for Lorom/Hirom without SRAM
  61,A5 (when forcing HIROM:ALL)
  D5,7F (when forcing HIROM:MENU)
  8A,8A (when forcing HIROM:MENU on skaman's blank cart)
```

Probably selecting which bank(s) SRAM is mapped/mirrored in the SNES memory space.

#### Nintendo Power I/O Ports

The I/O ports at 002400h-002401h are used for mapping a selected game. Done as follows:

```text
  mov  [002400h],09h
  cmp  [002400h],7Dh
  jne  $  ;lockup if invalid
  mov  [002401h],28h
  mov  [002401h],84h
  mov  [002400h],06h
  mov  [002400h],39h
  mov  [002400h],80h+(Directory[n*2000h+0] AND 0Fh)
  jmp  $  ;lockup (until reset applies)
```

After the last write, the MX15001 chip maps the desired file, and does then inject a /RESET pulse to the SNES console, which resets the CPU, APU (both SPC and DSP), WRAM (address register), and any Expansion Port hardware (like Satellaview), or piggyback cartridges (like Xband modem). The two PPU chips and the CIC chip aren't affected by the /RESET signal. The overall effect is that it boots the selected file via its Reset vector at [FFFCh].
