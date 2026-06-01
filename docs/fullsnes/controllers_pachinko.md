# SNES Controllers Pachinko

#### Pachinko Controller (Sunsoft)

The Pachinko controller should be connected to controller Port 2 (plus a normal joypad in Port 1 for menu selections). After the usual joypad strobing, Pachinko data can be read serially from [4017h].Bit0:

```text
  1st..8th    Unknown/unused                (would be Buttons/DPAD on joypads)
  9th         Used... probably a button?    (would be Button-A on joypads)
  10th..12th  Unknown/unused                (would be Buttons on joypads)
  13th..16th  ID Bit3-0       (must be 0Eh) (MSB first)
  17th..24th  Extra ID Bit7-0 (must be 77h) (MSB first)
  25th        Unknown/unused
  26th..32th  Analog Dial Position (7bit, MSB first, inverted, 1=Low=Zero)
  33th..      Unknown/padding
```

Average analog 7bit range returned on real hardware is unknown. In software, the used 7bit range is around 18h=Stopped through 7Fh=Fastest.

The controller looks somewhat like a blue egg, plus a yellow dial with zagged finger-grips, and probably with a button somewhere (maybe the orange window in the middle of the head or so?):

```text
            _
  Top-View  \_\___   .             __________   Side-View
          ..'     '..|\          .'  ''''''  '.     <-- Blue Head
      ...'           '.|       _|______________|
    .'.'      ___      '.     <__<_|__<_|_______)   <-- Yello dial
   / .'     .'   '.     '.      |              |
  /  |     |       |     |__    |              |    <-- Blue Base
  |/\|     |SUNSOFT|     | /    |              |
     '.     '.___.'     .'/      |            |
      '.               .'         ''._ __ _.'' \
        '.           .'          _____|__|_____ ''----- cable
          ''._____.''           (______________)
```

Known supported games are:

```text
  Hissatsu Pachinko Collection 1 (J) 1994 Sunsoft/Fuji
  Hissatsu Pachinko Collection 2 (J) 1995 Sunsoft/Fuji
  Hissatsu Pachinko Collection 3 (J) 1995 Sunsoft/Daiichi/Nifty-Serve
  Hissatsu Pachinko Collection 4 (J) 1996 Sunsoft/Kyoraku/Nifty-Serve
```

Note: Pachinko is a japanese gambling game; its appearance is resembling pinball, but concerning stupidity it's more resembling one-armed-bandit-style slot machines.
