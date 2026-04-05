# SFC-Box Component List (Cartridges)

#### SFC-Box Cartridge PCB (GS 0871-102)

```text
  IC1  28pin  DIP 27C256 EPROM "GROMn-1" (usually 28pin; 28pin/32pin possible)
  IC2  20pin  SMD Philips 74HC273D
  IC3  20pin  SMD Philips 74HC541D
  IC4  20pin  SMD Philips 74HC273D
  IC5  14pin  SMD <unknown>
  IC6  14pin  SMD <unknown>
  IC7  14pin  SMD <unknown>
  IC8  16pin  SMD <unknown> 74HC138                     (semi-optional)
  IC9  16pin  SMD <unknown> 74HC138                     (semi-optional)
  IC10 16pin  SMD <unknown> HC138 or HC130 or so?       (semi-optional)
  IC11 16pin  SMD <unknown> 74HC138 or 74HC130 or so ?  (semi-optional)
  IC12 16pin  SMD <unknown> (near IC16)                 (semi-optional)
  IC13 16pin  SMD Philips 74HC153D                      (semi-optional)
  IC14 20pin  DIP GAL16V8B
  IC15 14pin  SMD 74AC125 (near IC17)
  IC16 32pin  DIP Sony CXK581000P-12L (SRAM 128Kx8)          (optional)
  IC17 28pin  DIP Nintendo DSP1 A/B (for Mario Kart)         (optional)
  IC18 14pin  SMD <unknown> 74HC04 (below X1)           (semi-optional)
  IC19 100pin SMD Mario Chip 1 (Star Fox GSU)                (optional)
  IC20 32pin  SMD SHVC-FO-1    (Star Fox ROM)                (optional)
  IC21 28pin  SMD HY62256A     (Star Fox RAM, 32Kx8)         (optional)
  IC22 14pin  SMD <unknown> (below IC4)
  IC23 14pin  SMD <unknown> HC08                        (semi-optional)
  ROM1 36pin  DIP -or- ROM7  36pin DIP ;\solder pads for up to six ROMs
  ROM2 36pin  DIP -or- ROM8  36pin DIP ; (each with two alternate pin-outs,
  ROM3 36pin  DIP -or- ROM9  36pin DIP ; eg. ROM1=LoROM or ROM7=HiROM)
  ROM4 36pin  DIP -or- ROM10 36pin DIP ; (can be fitted with 32pin/36pin chips
  ROM5 36pin  DIP -or- ROM11 36pin DIP ; except, ROM6 can be 32pin only)
  ROM6 32pin  DIP -or- ROM12 36pin DIP ;/(see IC1 & IC20 for further (EP)ROMs)
  X1   ?pin   DIP <unknown>, oscillator for DSP1, probably 2-3 pins?(optional)
  CN1  100pin DIP OMRON XC5F-0122 Cartridge Connector (female 2x50pin)
```

IC16 is 128K SRAM, this chip is installed in the PSS61 cartridge only, but, it's shared for multiple games (including games in PSS62-PSS64 carts).

The SRAM isn't battery-backed, however, the SFC-Box cannot be switched off (unless when unplugging supply cables), so SRAM should be always receiving a standby-voltage from the console.

The hardware might allow to share the DSP1 chip in similar fashion (?), in the existing carts, it's used only for Mario Kart.

The "optional" components are installed in PSS61 only. The "semi-optional" ones are installed in PSS61 and (for unknown reason) also in PSS62 (whilst, PSS63/PSS64 don't have them, although they should be functionally same as PSS62).

Unknown how many different programs are possible (there are pads for max 6 DIP ROMs, plus 1 SMD ROM, but maybe the SMD is alternate to one DIP, and maybe some pads are reserved for games with 2 ROMs; and unknown if the GUI menu supports more than 8 games).
