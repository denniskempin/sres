# SFC-Box Coprocessor (HD64180) (extended Z80)

This is the "heart" of the SFC-Box. The two central parts are a HD64180 CPU (with extended Z80 instruction set), and a 64Kbyte EPROM labelled "KROM 1" (HD64180 BIOS). Plus, a frightening amount of about 50 small logic chips on the mainboard & daughterboard.

Overall Features are (probably)...

```text
  - Injecting Controller Data (for Demo/Preview mode)
  - Sniffing Controller Data (for L+R+Select+Start Soft-Reset feature)
  - Send/Receive Data to the SNES Menu Program (via WRIO/RDIO ports)
  - Mapping the selected Game ROM (or Menu Program) into SNES memory
  - Maybe also mapping GAME-SRAM bank(s) and/or the DSP-1 chip
  - Resetting the SNES, for starting the Game ROM (or Menu Program)
  - Reading "GROM" data from EPROMs in the cartridges
  - Reportedly drawing an extra "OSD" video layer on top of the SNES picture
  - Accessing the RTC Real-Time-Clock (unknown purpose)
  - Somehow logging/counting or restricting the "pay-per-play" time
  - Maybe handling the GAME/TV button in whatever fashion
  - Maybe handling the RESET button by software
  - Maybe controlling the two GAME/TV LEDs
```

#### Pay-per-play

Not much known there. Some people say the SFC-Box was coin-operated... but, it doesn't contain any coin-slot, and there seem to be no external connectors for external coin-slot hardware. And, there seem to be no external connectors for a "network-cable" for automatically charging the room-bill.

Usage of [4201]=RDIO / [4213]=WRIO on SNES Side (used by Menu Program) Default WRIO output value is 00E6h.

```text
  bit0 Out (usually Output=LOW, from SNES) (maybe indicate ready)
  bit1 In  (data in, to SNES)
  bit2 In  (status/ready/malfunction or so, to SNES)
  bit3 Out (clock/ack out, from SNES)
  bit4 Out (data out, from SNES)
  bit5 In  (clock in, to SNES)
  bit6 -   (probably normal joy1 io-line)
  bit7 -   (probably normal joy2 io-line & lightgun latch)
```

After booting, the SNES menu program checks the initial "1bit" status, and does then repeatedly receive 32bit packets (one command byte, and three parameter bytes) (and, in response to certain commands, it does additionally send or receive further bytes; in some cases these extra transfers are done via joy1/joy2 shift-registers instead of via WRIO, that probably because the HLL-coded KROM is so incredibly inefficient that it needs "hardware-accelerated" serial shifts).
