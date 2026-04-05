# SNES Pinouts RTC Chips

#### Sharp S-RTC Pin-Outs (used by Dai Kaiju Monogatari 2)

```text
  1-24 Unknown (should have an address decoder and 4bit data bus or so)
```

24pin chip. Still unknown which & how many address/data lines are connected, and if there are "specials" like /IRQs (?)

Epson/Seiko RTC-4513 Pin-Outs (for Far East of Eden Zero) (via SPC7110 chip)

```text
  1 NC
  2 DATA
  3 STD.P
  4 NC
  5 NC
  6 VCC
  7 NC
  8 NC
  9 GND
  10 NC
  11 NC
  12 CE
  13 CLK
  14 NC
```

#### Seiko/Epson S-3520CF Pin-Outs (used in SFC-Box and NSS)

```text
  1 Xin
  2 NC
  3 Xout
  4 /CLK
  5 DataIn
  6 /WR
  7 GND
  8 /TPOUT
  9 DataOut
  10 PDW
  11 /CS
  12 Capacitor
  13 NC
  14 VCC
```

#### Crystal = 32.768kHz (see datasheet page 13)
