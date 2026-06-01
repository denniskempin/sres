# SFC-Box I/O Ports (Custom Ports)

[80h].R - Keyswitch and Button Inputs

```text
  0   Switch Pin0 Position ("OFF") Play Mode? (2nd from left) (0=Yes, 1=No)
  1   Switch Pin1 Position ("ON")  Play Mode? (3rd from left) (0=Yes, 1=No)
  2   Switch Pin2 Position ("2")   Play Mode? (4th from left) (0=Yes, 1=No)
  3   Switch Pin3 Position ("3")   Self-Test  (5th from left) (0=Yes, 1=No)
  4   Switch Pin9 Position ("1")   Options    (1st from left) (0=Yes, 1=No)
  5   Switch Pin4 Position (N/A)   Relay Off? (6th from left) (0=Yes, 1=No)
  6   TV/GAME Button (0=On, 1=Off)
  7   RESET Button   (0=On, 1=Off)
```

[80h].W - SNES Transfer and Misc Output

```text
  0   SNES Transfer STAT to SNES  (Bit2 of WRIO/RDIO on SNES side)
  1   SNES Transfer CLOCK to SNES (Bit5 of WRIO/RDIO on SNES side)
  2   SNES Transfer DATA to SNES  (Bit1 of WRIO/RDIO on SNES side)
  3   Unknown/unused
  4     ?? pulsed while [C094] is nonzero (0370h timer0 steps)
  5     ??         PLENTY used (often same as bit7)
  6   Unknown/unused
  7     ??                     (often same as bit5)
```

[81h].R - SNES Transfer and Misc Input

```text
  0   Int0 Request (Coin-Input, Low for 44ms..80ms) (0=IRQ, 1=No)
  1   SNES Transfer ACK from SNES  (Bit3 of WRIO/RDIO on SNES side)
  2   SNES Transfer DATA from SNES (Bit4 of WRIO/RDIO on SNES side)
  3   Boot mode or so (maybe a jumper, or watchdog-flag, or Bit0 of WRIO/RDIO?)
  4   Unknown/unused (0) ;\joy1/slot0 or so, used by an UNUSED function (08A0h)
  5   Unknown/unused (0)  ;/(for "joy2/slot1" or so, use [A0].4-5)
  6   Int1 Request (Joypad is/was accessed by SNES or so?) (0=IRQ, 1=No)
  7   Vblank, Vsync, or Whatever flag (seems to toggle at 100..200Hz or so?)
```

[81h].W - Misc Output

```text
  0   SNES Reset CPU/PPU/APU/GSU/DSP1 or so (0=Reset, 1=Normal)
  1     ??         PLENTY used
  2     ??  something basic, ATROM related (or maybe HALT snes CPU?)
  3   Int1 Acknowledge (Joypad related) (0=Ack, 1=Normal)
  4     ??         PLENTY used
  5     ?? set to 1-then-0 upon init (maybe ACK/RESET something?)
  6   Watchdog Reload (must be pulsed during mem tests/waits/transfers/etc)
  7   OSD Chip Select (for CSI/O) (0=No, 1=Select)
```

[83h].R - Joypad Input/Status

```text
  0   Joy2 Port [86h..87h] ready for reading (0=No, 1=Yes)     ;\Automatic
  1   Joy1 Port [84h..85h] ready for reading (0=No, 1=Yes)     ;/Reading
  2   Unknown/unused (usually/always 0)
  3   Unknown/unused (usually/always 1) (maybe joy4 Data?)
  4   Unknown/unused (usually/always 1) (maybe joy3 Data?)
  5   Joy2 Data   (0=Low, 1=High) ;\that is inverse as on SNES ;\Manual
  6   Joy1 Data   (0=Low, 1=High) ;/(where it'd be 1=Low)      ;/Reading
  7   Unknown/unused (usually/always 0)
```

[83h].W - Joypad Output/Control

```text
  0   Joypad Strobe  (0=No, 1=Yes)                             ;\Manual
  1   Joypad2? Clock (0=Yes, 1=No)                             ; Reading
  2   Joypad1? Clock (0=Yes, 1=No)                             ;/
  3   Joypad Reading (0=Automatic, 1=Manual)
  4   Joypad Swap    (0=Normal, 1=Swap Joy1/Joy2)
  5-7 Unknown/unused (should be 0)
```

Not quite clear if the "Swap" feature affects... software/hardware? upon reading/writing? upon manual/automatic access?

[84h].R/W - Joypad 1, MSB (1st 8 bits) (eg. Bit7=ButtonB, 0=Low=Pressed) [85h].R/W - Joypad 1, LSB (2nd 8 bits) (eg. Bit0=LSB of ID, 0=Low=One) [86h].R/W - Joypad 2, MSB (1st 8 bits) (eg. Bit7=ButtonB, 0=Low=Pressed) [87h].R/W - Joypad 2, LSB (2nd 8 bits) (eg. Bit0=LSB of ID, 0=Low=One) 2x16bit Joypad data from Controller / to SNES. In Automatic Reading mode, data is automatically forwarded to SNES, and if desired, it can be read from [84h..87h] (when [83h].0-1 are zero). The clock source for reading is unknown (maybe it comes from the SNES, so it'd work only IF the SNES is reading).

In Manual Reading mode, joypad can be read via [83h], and data can be then forwarded to SNES by writing to [84h..87h].

Notes: Observe that the bits are inverse as on SNES (where it'd be 1=Low).

Aside from [83h..87h], joypad seems to also somehow wired to INT1 interrupt (and [81h].R.Bit6 and [81h.W.Bit3). Also observe that [84h/86h] are containing the MSBs (not LSBs). The KROM1/ATROM are also mis-using [84h..87h] for general-purpose "high-speed" data transfers (that is, faster than the crude HLL coded software transfers in KROM1).

[A0h].R - Real Time Clock Input (S-3520)

```text
  0   RTC Data In      (0=Low=Zero, 1=High=One)
  1   Unknown/unused (usually/always 0)
  2   Unknown/unused (usually/always 0)
  3   Unknown/unused (usually/always 1)
  4   Unknown/unused (0) ;\joy2/slot1 or so, used by an UNUSED function (08A0h)
  5   Unknown/unused (0) ;/(for "joy1/slot0" or so, use [81].4-5)
  6   Unknown/used?! (usually/always 1)   used/flag ?   extra BUTTON ?
  7   Unknown/used?! (usually/always 1)   used/flag ?   extra BUTTON ?
```

[A0h].W - Real Time Clock Output (S-3520)

```text
  0   RTC Chip Select  (0=High=No,   1=Low=Select)
  1   RTC Direction    (0=Low=Write, 1=High=Read)
  2   RTC Data Out     (0=Low=Zero,  1=High=One)
  3   RTC Serial Clock (0=Low=Clk,   1=High=Idle)
  4     ??     cleared after "C632" offhold (5 timer1 steps)
  5   Unknown/Set to 0 (can be changed via 0A2Dh)
  6   Unknown/Unused   (can be changed via 0A26h)
  7   Unlock access to lower 16K of WRAM (0=Lock, 1=Unlock) (save area)
```

[C0h].W - SNES Mapping Register 0

```text
  0-1 ROM Socket  (0=ROM5, 1=ROM1/7/12, 2=ROM3/9, 3=IC20)
  2   ROM Slot    (0=Slot0, 1=Slot1)
  3   SRAM Enable (0=Disable, 1=Enable)
  4   SRAM Slot   (0=Slot0, 1=Slot1)
  5   DSP Enable  (0=Disable, 1=Enable)
  6   DSP Slot    (0=Slot0, 1=Slot1)
  7   ROM, DSP, and/or SRAM Mapping (0=LoROM, 1=HiROM)
```

[C1h].W - SNES Mapping Register 1

```text
  0-1 ROM, DSP, and/or SRAM Mapping (0=Reserved, 1=GSU, 2=LoROM, 3=HiROM)
  2-3 SRAM Base   (in 32Kbyte units) (range 0..3)
  4   GSU Slot    (0=Slot0, 1=Slot1)
  5   Zero/Unused?
  6-7 SRAM Size   (0=2K, 1=8K, 2=Reserved, 3=32K)
```

[82h].R/W - Unknown/unused [C0h].R - Unknown/unused [C1h].R - Unknown/unused Not used by KROM1 (nor GROMs). Reading from these ports usually/always returns FFh.
