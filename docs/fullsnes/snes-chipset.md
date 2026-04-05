# SNES Chipset

Chipset (PAL)  Board:     (C) 1992 Nintendo, SNSP-CPU-01        ;BOARD  U1  100pin Nintendo, S-CPU A, 5A22-02, 2FF 7S    ;CPU  (ID=2 in 4210h)  U2  100pin Nintendo, S-PPU1, 5C77-01, 2EU 64     ;PPU1 (ID=1 in 213Eh)  U3  100pin Nintendo, S-PPU2 B, 5C78-03, 2EV 7G   ;PPU2 (ID=3 in 213Fh)  U4  28pin  SONY JAPAN, CXK58257AM-12L, 227M87EY  ;VRAM1 32Kx8 SRAM  U5  28pin  SONY JAPAN, CXK58257AM-12L, 227M87EY  ;VRAM2 32Kx8 SRAM  U6  64pin  Nintendo, W-WRAM, 9227 T23 F          ;WRAM  U7  24pin  S-ENC, Nintendo, S (for Sony) 9226 B  ;video RGB to composite  U8  18pin  F413, (C) 1992, Nintendo, 9209 A      ;CIC  U9  -      N/A (NTSC version only, type 74HCU04) ;hex inverter (for X1 & CIC)  U10 14pin  (M)224, AN1324S (equivalent to LM324) ;SND Quad Amplifier  U11 3pin   T529D, 267                            ;GND,VCC,/RESET  U12 3pin   17805, 2F2, SV, JAPAN                 ;5V  U13 64pin  Nintendo, S-SMP, SONY, Nintendo'89... ;SND1 (SPC700 CPU)  U14 80pin  Nintendo, S-DSP, SONY'89, WWW149D4X   ;SND2 (sound chip)  U15 28pin  MCM51L832F12, (M) JAPAN RZZZZ9224     ;SND-RAM1 32Kx8 SRAM  U16 28pin  MCM51L832F12, (M) JAPAN RZZZZ9224     ;SND-RAM2 32Kx8 SRAM  U17 16pin  NEC, D6376, 9225CJ (ie. NEC uPD6376)  ;SND-Dual 16bit D/A  U18 14pin  S-CLK, 2FS 4A (for PAL only)          ;X1 to 21.2MHz and 4.43MHz  TC1 2pin   Red Trimmer                           ;X1-ADJUST  X1  2pin   D177F2                                ;CPU/PPU 17.7344750MHz (PAL)  X2  2pin   CSA, 24.57MX, Gm J                    ;SND 24.576MHz  F1  2pin   SOC, 1.5A                             ;FUSE (supply-input)  T1  4pin   TDK, ZJYS-2, t                        ;DUAL-LOOP (supply-input)  DB1 4pin   TOSHIBA, 4B1, 1B 2-E JAPAN            ;AC-DC (PAL/AC-version only)  L1  2pin   220 (22uH)                            ;LOOP (color clock to GND)  VR1 2pin   (M)ZNR, FK220, 26                     ;? (supply)  J1? 2pin   AC Input 9V                           ;AC-IN  J2  4pin   SNSP CCIR-EEC, A E210265, 250142A     ;RF-Unit (modulator)  SW  4pin   Reset Button (on board)  P1  64pin  Cartridge Slot  P2  11pin  To Front Panel (Controllers/Power LED)  P3  2pin   To Power Switch  P4  12pin  Multi Out  P5  28pin  EXT Expansion Port (bottom side)

#### Costdown SNES chipset

```text
  U1 160pin  Nintendo S-CPUN A, RF5A122 (CPU, PPU1, PPU2, S-CLK)
  U2 100pin  Nintendo S-APU             (S-SMP, S-DSP, 64Kx8 Sound RAM)
  U3  64pin  Nintendo S-WRAM B
  U4  28pin  32Kx8 SRAM (video ram)
  U5  28pin  32Kx8 SRAM (video ram)
  U6?  8pin? ?
  U7  24pin  Nintendo S-RGB A
  U8  18pin  Nintendo F411B (CIC)
  U9   3pin  17805 (5V supply)
  U10 14pin  S-MIX A (maybe sound amplifier?)
  U11  3pin  Reset?
  X1   2pin  D21G8N (21.4MHz NTSC, or 17.7MHz PAL)
  X2   2pin  APU clock (probably the usual 24.576MHz?)
```

#### 51832

```text
  Toshiba TC51832FL-12 32k x8 SRAM (SOP28)
```

#### CXK58257AM-12L

```text
  32768-word x 8-bit high speed CMOS static RAM, 120ns,
  standby 2.5uW in 28-pin SOP package.
  Operational temperature range from 0'C to 70'C.
```
