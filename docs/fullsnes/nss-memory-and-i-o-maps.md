# NSS Memory and I/O Maps

#### Z80 Memory Map

```text
  0000h-7FFFh : 32K BIOS
  8000h-9FFFh : 8K RAM (upper 4K with write-protect)
  A000h       : EEPROM Input (R)
  C000h-DFFFh : Upper 8K of 32K Instruction EPROM (in Cartridge) (INST-ROM)
  E000h       : EEPROM Output (W)
  Exxxh       : PROM Input AND Output AND Program Code (RST opcodes) (R/W/EXEC)
```

Note: For some reason, Nintendo has stored the 8K INST-ROM in 32K EPROMs - the first 24K of that EPROMs are unused (usually 00h-filled or FFh-filled, and EPROM pins A13 and A14 are wired to VCC, so there is no way to access the unused 24K area).

#### Z80 IN-Ports

```text
  Port 00h.R - IC46/74LS540 - Joypad Buttons and Vsync Flag
  Port 01h.R - IC38/74LS540 - Front-Panel Buttons & Game Over Flag
  Port 02h.R - IC32/74LS540 - Coin and Service Buttons Inputs
  Port 03h.R - IC31/74HC367 - Real-Time Clock (RTC) Input
  Port 04h.R - Returns FFh (unused)
  Port 05h.R - Returns FFh (unused)
  Port 06h.R - Returns FFh (unused)
  Port 07h.R - Returns FFh (same effect as write-any-value to Port 07h.W)
```

Port 0008h..FFFFh are mirrors of above ports (whereof, mirrors at xx00h..xx03h are often used).

#### Z80 OUT-Ports

```text
  Port 00h/80h.W         - IC40/74HC161 - NMI Control and RAM-Protect
  Port 01h/81h.W         - IC39/74HC377 - Unknown and Slot Select
  Port 02h/82h/72h/EAh.W - IC45/74HC377 - RTC and OSD
  Port 03h/83h.W         - IC47/74HC377 - Unknown and LED control
  Port 84h.W             - IC25/74HC161 - Coin Counter Outputs
  Port 05h.W             - Unused (bug: written by mistake)
  Port 06h.W             - Unused
  Port 07h.W - IC23/74HC109 - SNES Watchdog: Acknowledge SNES Joypad Read Flag
```

These ports seem to be decoded by A0..A2 only (upper address bits are sometimes set to this or that value, but seem to have no meaning).

#### SNES Memory Map

Normal SNES memory map, plus some special registers:

```text
  4100h/Read.Bit0-7  - DIP-Switches (contained in some NSS cartridges)
  4016h/Write.Bit0   - Joypad Strobe (probably clears the SNES Watchdog flag?)
                          (OR, maybe that occurs not on 4016h-writes,
                          but rather on 4016h/4017h-reads, OR elsewhere?)
  4016h/Write.Bit2   - Joypad OUT2 indicates Game Over (in Skill Mode games)
  4016h/4017h/4218h..421Bh - Joypad Inputs (can be disabled)
```
