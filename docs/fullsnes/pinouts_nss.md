# SNES Pinouts NSS Connectors

NSS - CN11/12/13 - Cartridge Slots (3 slots, 2x50pin each)

```text
            Solder side    Component side
                      A    B
  WRAM.64         GND - 1  - VCC2        INST.28                ;\
  WRAM.64         GND - 2  - VCC2        INST.28                ; PROM
  PROM.7-R3  PROM.RES - 3  - PROM.CLK    PROM.6                 ; (and SNES
  PROM.5-R2  PROM.TST - 4  - PROM.CNT    PROM.8                 ; select)
              /SNES_# - 5  - PROM.DTA    PROM.1                 ;/
  INST.15          D3 - 6  - D4          INST.16                ;\
  INST.13          D2 - 7  - D5          INST.17                ;
  INST.12          D1 - 8  - D6          INST.18                ;
  INST.11          D0 - 9  - D7          INST.19                ;
  INST.10          A0 - 10 - /CE_#       INST.20                ;
  INST.9           A1 - 11 - A10         INST.21                ; INST ROM
  INST.8           A2 - 12 - /OE         INST.22                ;
  INST.7           A3 - 13 - A11         INST.23                ;
  INST.6           A4 - 14 - A9          INST.24                ;
  INST.5           A5 - 15 - A8          INST.25                ;
  INST.4           A6 - 16 - A7          INST.3                 ;
  INST.2          A12 - 17 - GND         WRAM.64                ;
  WRAM.64         GND - 18 - VCC2        INST.28                ;
  WRAM.64 _______ GND - 19 - VCC2 ______ INST.28                ;/
  WRAM.64         GND - 20 - VCC         WRAM.1                 ;\
  WRAM.64         GND - 21 - VCC         WRAM.1                 ;
  WRAM.56       /PARD - 22 - /PAWR       WRAM.58                ;
  WRAM.47         PA6 - 23 - PA7         WRAM.50                ;
  WRAM.45         PA4 - 24 - PA5         WRAM.46                ;
  WRAM.43         PA2 - 25 - PA3         WRAM.44                ; SNES Bus
  WRAM.53         PA0 - 26 - PA1         WRAM.54                ; (and PROM
  WRAM.57         /RD - 27 - /WR         WRAM.59                ; select)
  WRAM.63          D3 - 28 - D4          WRAM.2   ;\D4..D7 in   ;
  WRAM.62          D2 - 29 - D5          WRAM.3   ; opposite    ;
  WRAM.61          D1 - 30 - D6          WRAM.4   ; order as    ;
  WRAM.60          D0 - 31 - D7          WRAM.5   ;/on SNES     ;
  CPU.46         /IRQ - 32 - /ROMSEL     CPU.77                 ;
  CPU.93           A0 - 33 - A23         CPU.17                 ;
  CPU.94           A1 - 34 - A22         CPU.16                 ;
  CPU.95           A2 - 35 - A21         CPU.15                 ;
  CPU.96           A3 - 36 - A20         CPU.14                 ;
  CPU.97           A4 - 37 - A19         CPU.13                 ;
  CPU.98           A5 - 38 - A18         CPU.12                 ;
  CPU.99           A6 - 39 - A17         CPU.11                 ;
  CPU.100          A7 - 40 - A16         CPU.10                 ;
  CPU.2            A8 - 41 - A15         CPU.9                  ;
  CPU.3            A9 - 42 - A14         CPU.8                  ;
  CPU.4           A10 - 43 - A13         CPU.7                  ;
  CPU.5           A11 - 44 - A12         CPU.6                  ;
  WRAM.7      REFRESH - 45 - /WRAMSEL    WRAM.15                ;
              AUDIO_L - 46 - AUDIO_R                            ;
  PROM.2   PROM./CE_# - 47 - SYSCLK      WRAM.6                 ;
  CPU.48      MCK 21M - 48 - /RESET      WRAM.8                 ;
  WRAM.64         GND - 49 - VCC         WRAM.1                 ;
  WRAM.64         GND - 50 - VCC         WRAM.1                 ;/
```

The NSS motherboard uses female Matsushita AXD100271 connectors, and the NSS cartridges have male Matsushita AXD200251 connectors. Both are obsolete as of a few years ago, but they're just shrouded 0.1" headers.

#### RICOH RP5H01 PROM Pinout (Decryption Key PROM on NSS Cartridges)

```text
  1 DATA.OUT
  2 /CE (VPP)
  3 VCC
  4 GND
  5 TEST
  6 DATA.CLK
  7 RESET
  8 COUNTER.OUT
```

NSS - CN1 - Big Edge Connector "JAMMA" - 2x28 pin

```text
  1  GND (from Power Supply)
  A  GND (from Power Supply)
  2  GND (NC)
  B  GND (from Power Supply)
  3  +5V (from Power Supply)
  C  +5V (to joypads; and NC there)
  4  +5V (from Power Supply)
  D  +5V (from Power Supply)
  5  NC  (NC)
  E  -5V (from Power Supply)
  6  +12V (to Coin Lamps and Coin Counter)
  F  +12V (from Power Supply)
  7       KEY
  H       KEY
  8  Coin Counter 1
  J  Coin Counter 2
  9  NC
  K  NC
  10 SPEAKER (Right)
  L  SPEAKER (Left)
  11 AUDIO (+) (NC)
  M  AUDIO GND
  12 VIDEO RED
  N  VIDEO GREEN
  13 VIDEO BLUE
  P  VIDEO SYNC
  14 VIDEO GND
  R  SERVICE SW
  15 TEST SW
  S  NC
  16 COIN SW 1
  T  COIN SW 2
  17 1P START
  U  2P START
  18 1P UP
  V  2P UP
  19 1P DOWN
  W  2P DOWN
  20 1P LEFT
     2P LEFT
  21 1P RIGHT
     2P RIGHT
  22 1P A
     2P A
  23 1P B
     2P B
  24 1P SELECT
     2P SELECT
  25 VOLUME ?   (POT Center Pin)
     VOLUME ?   (POT Outer Pin)
  26 VOLUME GND (POT Outer Pin)
     NC
  27 GND
     GND
  28 GND
     GND
```

#### NSS - CN2 - 10P Connector (Extra Joypad Buttons)

```text
  1  GND
  2  2P TR
  3  2P TL
  4  2P Y
  5  2P X
  6  1P TR
  7  1P TL
  8  1P Y
  9  1P X
  10 GND
```

#### NSS - CN3 - 13P Connector (Front Panel LEDs/Buttons)

```text
  1  GND (for Buttons)
  2  Button Restart
  3  Button Page Down
  4  Button Page Up
  5  Button Instructions
  6  Button Game #3
  7  Button Game #2
  8  Button Game #1
  9  LED Instructions
  10 LED Game #3
  11 LED Game #2
  12 LED Game #1
  13 +5V or so (for LEDs)
```

#### NSS - CN4

```text
  1 GND      (to SNES Controller pin 7)
  2 /EXT_CTRL2 (Low=External CN4 controller, High=Internal Joypad2 selected)
  3 JPIO7    (to SNES Controller pin 6)  ;\
  4 JPSTR    (to SNES Controller pin 3)  ; always connected
  5 JPCLK2   (to SNES Controller pin 2)  ;/
  6 4017.D1  (to SNES Controller pin 5)  ;\only when CN4 selected
  7 4017.D0  (to SNES Controller pin 4)  ;/
  8 SNES +5V (to SNES Controller pin 1)
```

The external input is enabled when setting INST ROM Flags Bit0=0 (that bit is copied to Port 01h.W bit4).

#### NSS - CN5

```text
  1 GND
  2 IC32/74LS540 pin 9 (Port 02h.R bit 7)
  3 IC32/74LS540 pin 8 (Port 02h.R bit 6)
  4 IC32/74LS540 pin 7 (Port 02h.R bit 5)
  5 IC32/74LS540 pin 6 (Port 02h.R bit 4)
  6 IC32/74LS540 pin 5 (Port 02h.R bit 3)
  7 +5V
```

NSS Repair (Blank Screen / Washed out colors) There seems to be a fairly common hardware problem that causes the NSS to show a picture with washed out colors or a completely blank screen; in some cases the problem appears or disappears when the unit has warmed up.

The problem is related to the power supply of the IR3P32A chip: The supply should be around 9V, and video glitches appear when it drops below 8V. For deriving the "9V", Nintendo has strapped the IR3P32A to the 12V line via a 100 ohm resistor; which is rather crude and unreliable.

As workaround one could add a second resistor in parallel with the 100 ohms (which is equally crude, though it should help temporarily), a more reliable solution should be to replace the 100 ohms by a 7809 voltage regulator (and eventually some capacitors as far as needed).

The actual reason for the problem is unknown - apparently some odd aging effect on the IR3P32A chip and/or other components connected to it. No info if the problem occurs both with original monitor and power supply as well as with third-party hardware.

#### NSS-to-SNES-cartridge adaptor (signal quality)

Using SNES cartridges with coprocessors (eg. DSP1 carts) on NSS requires some fine tuning:

DogP's older solution: The /RD and /WR pins seem to have high slew rates and overshoot badly (by around 3V, for just a few ns)... a regular Mario Kart cartridge works perfectly with LPFs added to those pins. The PowerPak seems to still have some issues though.

DogP's newer solution: I actually ended up just adding small resistors in series with the data bus, which helped reduce the overshoot/ringing. This also fixed the PowerPak issues.

#### NSS-to-SNES-cartridge adaptor (CIC)

A fully functional NSS-to-SNES-cartridge adaptor would also require a CIC chip (as a few SNES cartridges with special protections won't work if the 'console' doesn't output the correct CIC signals).

Accordingly, the adaptor would also need something that generates the 3.072MHz CIC clock signal (on a real SNES that would be 24.576MHz/8 coming from APU) (on the NSS adaptor it would require a separate oscillator, or if accuracy doesn't matter, then one might get away with 21.xxxMHz PAL/NTSC master clock divided by 7 (or dirtier: divided by 8)).

Unless there should be another way to get those protected cartridges to work (maybe by simply wiring CIC clock to VCC or GND, or by feeding it only a few dozen of CIC clks after reset, so it could initialize itself, but would never reach the point where the protection could do something harmful).
