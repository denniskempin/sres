# SNES Pinouts MAD Chips

#### MAD-1 (and MAD-1 A) Pinouts (Memory Address Decoder 1)

```text
  1  OUT1 /ROM.CS1 ;Chipselect to Upper ROM (NC if single ROM)
  2  OUT2 /SRAM.CS ;Chipselect to SRAM
  3  OUT3 /AUX.CS  ;Chipselect to Expansion I/O or so (usually NC)
  4  OUT4 /ROM.CS  ;Chipselect to Single ROM (NC if two ROMs)
  5  Vout          ;Supply to SRAM (+3V when VCC=off, +5V when VCC=on)
  6  VCC           ;Supply from SNES (+5V)
  7  Vbat          ;Supply from Battery via resistor (+3V)
  8  GND           ;Supply Ground
  9  IN6  /RESET   ;From cart.26
  10 IN5  MODE     ;HiROM: VCC                  | LoROM: GND
  11 IN4  /ROMSEL  ;From cart.49
  12 IN3  Addr3    ;HiROM: A22 (400000h) or A15 | LoROM: A22 (400000h) or VCC
  13 IN2  Addr2    ;HiROM: A21 (200000h)        | LoROM: A21 (200000h)
  14 IN1  Addr1    ;HiROM: A14 (4000h)          | LoROM: A20 (100000h)
  15 IN0  Addr0    ;HiROM: A13 (2000h)          | LoROM: A15 (8000h)
  16 OUT0 /ROM.CS0 ;Chipselect to Lower ROM (NC if single ROM)
```

Note that Addr3 is sometimes wired this or that way. And, when using two ROMs, Addr2 is used as upper ROM address line (eg. Addr2=A20 for a cart with two 1Mbyte ROM chips) (so the ROM-size may also affect the SRAM/AUX mapping; according to the Addr2 wiring).

#### MAD-1 (and MAD-1 A) Logic Table

```text
  IN0   IN1   IN2   IN3   IN4   IN5   IN6   --> Output being
  Addr0 Addr1 Addr2 Addr3 /ROM  MODE  /RES      dragged LOW
  -----------------------------------------------------------
  HIGH  x     x     x     LOW   LOW   HIGH  --> /ROM.CS=LOW   ;\
  HIGH  x     LOW   x     LOW   LOW   HIGH  --> /ROM.CS0=LOW  ;
  HIGH  x     HIGH  x     LOW   LOW   HIGH  --> /ROM.CS1=LOW  ; LoROM
  LOW   LOW   HIGH  HIGH  LOW   LOW   HIGH  --> /AUX.CS=LOW   ;
  LOW   HIGH  HIGH  HIGH  LOW   LOW   HIGH  --> /SRAM.CS=LOW  ;/
  x     x     x     x     LOW   HIGH  HIGH  --> /ROM.CS=LOW   ;\
  x     x     LOW   x     LOW   HIGH  HIGH  --> /ROM.CS0=LOW  ;
  x     x     HIGH  x     LOW   HIGH  HIGH  --> /ROM.CS1=LOW  ; HiROM
  HIGH  HIGH  LOW   LOW   HIGH  HIGH  HIGH  --> /AUX.CS=LOW   ;
  HIGH  HIGH  HIGH  LOW   HIGH  HIGH  HIGH  --> /SRAM.CS=LOW  ;/
```

#### MAD-2 Pinouts (Memory Address Decoder 2)

Unknown. Used in some newer DSPn cartridges (mainly/only DSP1B ones?) (probably similar to MAD-1, but maybe with some pins replaced by a clock amplifier for the DSPn oscillator... and/or maybe it supplies an inverted RESET signal to the DSPn chip).

MAD-R Pinouts (Memory Address Decoder with Reset-Inverter) Pinouts are same as MAD-1, except for one pin: Pin4 outputs RESET (inverse of /RESET input) (there aren't any known cartridges using this feature).

The SHVC-2A3M-01, SHVC-2A3M-10, SHVC-2J3M-01 boards can be fitted with either MAD-1 or MAD-R (later SHVC-2A3M-11, SHVC-2J3M-11, SHVC-2J3M-20 boards are used with MAD-1 only, so MAD-R appears to have been discontinued after soon).

```text
  XXX The following boards do (also) accept either MAD-1 or MAD-R:
  2A1M-01
  2A5M-01
  2J5M-01
```

#### MAD-R Logic Table

```text
  IN0   IN1   IN2   IN3   IN4   IN5   IN6   --> Output being
  Addr0 Addr1 Addr2 Addr3 /ROM  MODE  /RES      dragged LOW
  -----------------------------------------------------------
  x     x     x     x     x     x     HIGH  --> RESET=LOW    * ;-Reset
  x     x     LOW   LOW   LOW   LOW   HIGH  --> /ROM.CS0=LOW * ;\
  x     x     HIGH  LOW   LOW   LOW   HIGH  --> /ROM.CS1=LOW * ; LoROM
  x     x     LOW   HIGH  LOW   LOW   HIGH  --> /AUX.CS=LOW  * ;
  LOW   HIGH  HIGH  HIGH  LOW   LOW   HIGH  --> /SRAM.CS=LOW   ;/
  x     x     LOW   x     LOW   HIGH  HIGH  --> /ROM.CS0=LOW   ;\
  x     x     HIGH  x     LOW   HIGH  HIGH  --> /ROM.CS1=LOW   ; HiROM
  HIGH  HIGH  LOW   LOW   HIGH  HIGH  HIGH  --> /AUX.CS=LOW    ;
  HIGH  HIGH  HIGH  LOW   HIGH  HIGH  HIGH  --> /SRAM.CS=LOW   ;/
```

The four "*" marked lines are different as MAD-1.

#### Other Battery Controllers

Battery Controllers are used to write-protect the SRAM during power-up/power-down, and to switch from Vbat to VCC supply when available.

Early carts used a transistor/diode circuit, later carts are using MAD-1/MAD-2/MAD-R or MM1026/MM1134 chips.

#### MM1026/MM1134 Pinouts

```text
  1 GND
  2 /RESET (output)
  3 CS (to SRAM) (usually NC when using /CS)
  4 Vbat (from battery)
  5 /CS (to SRAM)
  6 Vout (to SRAM)
  7 NC (MM1026) or /Y (MM1134) (aka /CS input)
  8 VCC (from snes)
```

On MM1026, /CS is simply inverse of CS (both true when VCC=good). On MM1134, the additional /Y input allows to force /CS false (but doesn't affect CS).

On SA-1 "SNSP-1L3B-20" boards, the PCB text layer says MM1026AF, but the actual chip is labelled "6129A, 6C33"; which refers to a "BA6129A" chip.

#### Batteries

```text
  CR2032 (3 volt lithium cells, 20mm diameter, 3.2mm width) <-- most SNES carts
  CR2430 (3 volt lithium cells, 24.5mm diameter, 3mm width) <-- X-Band Modem
  NiCd (3.6V, rechargeable, but acid-leaking) <-- many Copiers
```

#### MAD Versions/Revisions & Power Down Notes

```text
  BU2230           MAD-1
  XLU2230          MAD-1
  BU2230A          MAD-1A
  BU2231A          MAD-2A
  BU2220           MAD-R
  TexasInstruments MAD-1
```

At VCC>Vbat, the chips do operate normally (as shown in above Logic Tables).

At VCC=Vbat, the BU2230,BU2230A,XLU2230 (MAD1/MAD1A) force /SRAM.CS=high, whilst TexasInstrument (MAD1) and BU2220 (MAD-R) keep /SRAM.CS operating as normally.

At VCC=0, the BU2230,BU2230A,XLU2230 (MAD1/MAD1A) chips and BU2220 (MAD-R) switch the other 4 outputs to LOW, whilst TexasInstrument (MAD1) switch them HIGH (or maybe they are left floating and get pulled high by the test circuit).
