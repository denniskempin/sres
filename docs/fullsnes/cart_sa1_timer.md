# SNES Cart SA-1 Timer

#### 2210h SA-1 TMC - H/V Timer Control (W)

```text
  0   HEN             ;\Enables Interrupt or so ?
  1   VEN             ;/
  2-6 Not used (should be 0)
  7   Timer Mode (0=HV Timer, 1=Linear Timer)
```

#### 2211h SA-1 CTR - SA-1 CPU Timer Restart (W)

```text
  0-7 Don't care (writing any value restarts the timer at 0)
```

2212h SA-1 HCNT - Set H-Count Lsb (W) 2213h SA-1 HCNT - Set H-Count Msb (W)

```text
  0-8  H-Counter (9bit)
  9-15 Not used (should be 0)
```

Ranges from 0-340 (in HV mode), or 0-511 (in Linear mode).

2214h SA-1 VCNT - Set V-Count Lsb (W) 2215h SA-1 VCNT - Set V-Count Msb (W)

```text
  0-8  V-Counter (9bit)
  9-15 Not used (should be 0)
```

Ranges from 0-261 (in HV/NTSC mode), 0-311 (in HV/PAL mode), or 0-511 (in Linear mode). The PAL/NTSC selection is probably done by a soldering point on the PCB (which is probably also used for switching the built-in CIC to PAL/NTSC mode).

2302h SA-1 HCR - H-Count Read Lsb / Do Latching (R) 2303h SA-1 HCR - H-Count Read Msb (R) 2304h SA-1 VCR - V-Count Read Lsb (R) 2305h SA-1 VCR - V-Count Read Msb (R) Reading from 2302h automatically latches the other HV-Counter bits to 2303h-2305h.

#### Notes

In HV-mode, the timer clock is obviously equivalent to the dotclock (four 21MHz master cycles per dot). The time clock in linear mode is unknown (probably same as in HV-mode).

H-counter has 341 dots (one more as in SNES, but without long dots). Unknown if the short-scanline (in each 2nd NTSC non-interlaced frame) is reproduced (if it isn't, then one must periodically reset the timer in order to keep it in sync with the PPU). There is no provision for interlaced video timings.

The meaning of Port 2212h-2215h is totally unknown (according to existing specs it <sounds> as if they do set the <current> counter value - though alltogether it'd be more likely that they do contain <compare> values).

Unknown what happens when setting both HEN and VEN (probably IRQ triggers only if <both> H+V do match, ie. similar as for the normal SNES timers).
