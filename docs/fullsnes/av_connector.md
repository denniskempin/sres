# SNES Audio/Video Connector Pinouts

#### RF Out (TV Modulator)

Cinch with channel switch. Modulated video signal with mono-audio.

#### Multi Out

```text
  1   RGB - Red analog video out       ________________---________________
  2   RGB - Green analog video out    /  11    9     7     5     3     1  \
  3   RGB - H/V sync out             |                                     |
  4   RGB - Blue analog video out     \__12____10____8_____6_____4_____2__/
  5   Ground (used for Video)
  6   Ground (used for Audio)
  7   S-Video Y (luminance) out
  8   S-Video C (chroma) out
  9   Video Composite out (Yellow Cinch)
  10  +5V DC
  11  Audio Left out      (White Cinch)
  12  Audio Right out     (Red Cinch)
```

Pin 1,2,4: Red/Green/Blue (1V DC offset, 1V pp video into 75 ohms) Pin 3,7,8,9: (1V pp into 75 ohms) Pin 11,12: Left/Right (5V pp) In cost-down SNES models, pin 1-4 and 7-8 are reportedly not connected (though one can upgrade them with some small modifications on the mainboard).
