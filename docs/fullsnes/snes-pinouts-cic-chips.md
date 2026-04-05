# SNES Pinouts CIC Chips

#### CIC Pinouts

F411/F413: 18pin SMD-chip (used in console and in some carts) D411/D413: 16pin DIP-chip (used in most carts)

```text
  SMD DIP Pin Dir Usage  In Console               In Cartridge
  1   1   P00 Out DTA0   Cart.55 CIC1             Cart.24 CIC0
  2   2   P01 In  DTA1   Cart.24 CIC0             Cart.55 CIC1
  3   3   P02 In  RANDOM Via capacitor to VCC     NC
  4   4   P03 In  MODE   VCC=Console (Lock)       GND=Cartridge (Key)
  5       NC  -   (NC)   NC                       NC
  6   5   CL2 -   (NC)   NC                       SMD:NC or DIP:GND
  7   6   CL1 In  CLK    3.072MHz (from APU)      Cart.56 CIC3 (3.072MHz)
  8   7   RES In  RESET  From Reset button        Cart.25 CIC2 (START)
  9   8   GND -   GND    Supply                   Supply
  10  9   P10 Out /RESET To PPU (and CPU/APU/etc) NC (or to ROM, eg. in SGB)
  11  10  P11 Out START  Cart.25 CIC2             NC
  12  11  P12 -   (NC)   NC                       NC (or SlotID in FamicomBox)
  13  12  P13 -   (NC)   NC                       NC (or SlotID in FamicomBox)
  14      NC  -   (NC)   NC                       NC
  15  13  P20 -   (NC)   NC                       NC
  16  14  P21 -   (NC)   NC                       NC (or SlotID in FamicomBox)
  17  15  P22 -   (NC)   NC                       NC (or SlotID in FamicomBox)
  18  16  VCC -   VCC    Supply                   Supply
```

P00=Out,P01=In are the initial directions (for the Random Seed transfer), later on the directions are randomly swapped (ie. P00=In,P01=Out or P00=Out,P01=In).

START: short HIGH pulse on power-up or when releasing reset button.

/RESET: in console: to PPU, and from there to CPU,APU,Cart,Expansion.
