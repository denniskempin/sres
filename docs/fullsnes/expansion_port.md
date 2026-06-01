# SNES Expansion Port (EXT) Pinouts

EXT (at bottom of console) (used by Satellaview and Exertainment) Most pins are exactly the same as on the cartridge slot, the only special pins, which aren't found on Cart slot, are EXT pins 21, 22, and 25: SMPCK is 8.192MHz (24.576MHz/3) from APU, DOTCK is the PPU dot clock (around 5.3MHz, with some stuttering during hblank period), MONO is a mono audio output.

```text
             .--------.--- SHIELD=GND
         PA0 |1      2| PA1          Bottom View of console:
         PA2 |3      4| PA3        .-------------------------------------.
         PA4 |5      6| PA5        |              (rear side)            |
         PA6 |7      8| PA7        | +----+---------------+              |
       /PAWR |9     10| /PARD      | |snap|  28 ...... 2  |              |
          D0 |11    12| D1         | | in |  27 ...... 1  |              |
          D2 |13    14| D3         | +----+---------------+              |
          D4 |15    16| D5         |           EXT                       |
          D6 |17    18| D7         |                                     |
      /RESET |19    20| +5VDC      |                                     |
      SMPCLK |21    22| DOTCK      |                                     |
         GND |23    24| EXPAND     |                                     |
  MONO-AUDIO |25    26| /IRQ       |                                     |
     L-AUDIO |27    28| R-AUDIO    |              (front side)           |
             '--------'            '-------------------------------------'
```

The L/R-AUDIO inputs are essentially same as on cart slot, although they are passed through separate capacitors, so there is no 0 Ohm connection between EXT and Cart audio pins. EXT Pin 24 connects to Cart pin 2 (aside from a 10K pull-up it isn't connected anywhere inside of the console) so this pin is reserved for communication between cartridge hardware and ext hardware (used by Satellaview).
