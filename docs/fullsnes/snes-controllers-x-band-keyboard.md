# SNES Controllers X-Band Keyboard

The X-Band keyboard is a (rare) optional add-on for the X-Band Modem, intended to allow faster chatting/mailing as with the joypad controlled on-screen keyboard.

#### Keyboard Layout

The keyboard has a black case, 84 keys, an X-Band logo in upper left, and connection cable (to SNES controller port) attached in upper-right.

```text
   ___________________________________________________________________
  |   ><                                       Num  Caps Scroll       |
  |  BAND                                      Lock Lock Lock         |
  |  ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___  |
  | |   |   |   |   |   |   |   |   |   |   |   |   | L |Sel|Sta| R | |
  | |___|___|___|___|___|___|___|___|___|___|___|___|___|___|___|___| |
  | |Can|1! |2@ |3# |4$ |5% |6^ |7& |8* |9( |0) |-_ |=+ | <--   | X | |
  | |___|___|___|___|___|___|___|___|___|___|___|___|___|_______|___| |
  | |Switc| Q | W | E | R | T | Y | U | I | O | P |[{ |]} |     | Y | |
  | |_____|___|___|___|___|___|___|___|___|___|___|___|___|     |___| |
  | |Caps  | A | S | D | F | G | H | J | K | L |;: |'" | Enter  | A | |
  | |______|___|___|___|___|___|___|___|___|___|___|___|________|___| |
  | |Shift   | Z | X | C | V | B | N | M |,< |.> |/? |Shift |UP | B | |
  | |________|___|___|___|___|___|___|___|___|___|___|______|___|___| |
  | |`~ |   |>< |Ctr|                       |Ctr|>< |\| |LT |DN |RT | |
  | |___|___|___|___|_______________________|___|___|___|___|___|___| |
  |___________________________________________________________________|
```

#### Normal Controller Access (via STB and DATA0)

```text
  1st-12th    Unknown/unused
  13th-16th   Unknown/unused (should be usually a 4bit ID)
  17th-24th   Unknown/unused (should be sometimes extended 8bit ID)
  25th and up Unknown/unused
```

Note: If the keyboard data is transferred in sync with STB, then "17th and up" are the LSBs of the 2bit keyboard data pairs (though it might also be in sync with falling IOBIT, rather than with STB).

Keyboard Access (read_scancodes) (via IOBIT and DATA0 & DATA1) Below might be required to be preceeded by reading normal 16bit controller data (eg. via auto-joypad-reading) (ie. unknown if below needs leading STB signal, and preceeding 16xCLK signals).

```text
  [004201]=7Fh                 ;-set IOBIT=0
  id=getbits(8)                ;-read ID byte (must be 78h, aka "x")
  if CAPS=OFF [004201]=FFh     ;-set IOBIT=1 (when CAPS=OFF) (CAPS LED)
  num=getbits(4)               ;-read num scancodes
  if num>0 then for i=1 to num ;\
    [dst]=getbits(8)           ; read that scancodes (if any)
    dst=dst+1                  ;
  next i                       ;/
  [004201]=FFh  ;set IOBIT=1   ;-set IOBIT=1
```

Note: When reading the ID bits, BIOS sets IOBIT=1 after reading the first 2bit group (purpose unknown). When reading ONLY the ID (without reading following scancodes), then the scancodes do remain in the keyboard queue.

#### getbits(n)

```text
  for i=1 to n/2                       ;\
    delay (loop 7 times or so)         ; read 2bits at once, LSB first
    x=(x SHR 2) OR ([004017h] SHL 6)   ;
  next                                 ;/
  x=(x XOR FFh) SHR (8-n)              ;-invert & move result to LSBs
```

#### Scancode Summary

```text
  nn               normal key
  F0h,nn           normal key released
  E0h,nn           special key
  E0h,F0h,nn       special key released
```

#### Normal Scancodes (without prefix)

```text
  ____0xh____1xh___2xh___3xh___4xh___5xh___6xh___7xh_____8xh_____9xh___
  x0h ---    ---   ---   ---   ---   ---   ---   NUM-0   ---     <90h>?
  x1h ---    CTR1? C     N     ,<    ---   ---   NUM-.   ---     <91h>?
  x2h ---    SHF1? X     B     K     '"    ---   NUM-2   ---     <92h>?
  x3h ---    ---   D     H     I     ---   ---   NUM-5   ---     <93h>?
  x4h ---    CTR2? E     G     O     [{    ---   NUM-6   NUM-SUB <94h>?
  x5h ---    Q     4$    Y     0)    =+    ---   NUM-8   ---     <95h>?
  x6h ---    1!    3#    6^    9(    ---   BS    CANCEL  JOY-A   <96h>?
  x7h ---    ---   ---   ---   ---   ---   ---   NUM-DIV JOY-B   <97h>?
  x8h ---    ---   ---   ---   ---   CAPS  ---   ---     JOY-X   <98h>?
  x9h ---    ---   SPACE ---   .>    SHF2? NUM-1 NUM-RET JOY-Y   <99h>?
  xAh ---    Z     V     M     /?    ENTER ---   NUM-3   JOY-L   <9Ah>?
  xBh ---    S     F     J     L     ]}    NUM-4 ---     JOY-R   <9Bh>?
  xCh ---    A     T     U     ;:    \|    NUM-7 NUM-ADD SELECT  <9Ch>?
  xDh SWITCH W     R     7&    P     \|    ---   NUM-9   START   <9Dh>?
  xEh `~     2@    5%    8*    -_    ---   ---   NUM-MUL <8Eh>?  ---
  xFh ---    ---   ---   ---   ---   ---   ---   ---     <8Fh>?  ---
```

#### Special Scancodes (with E0h-prefix)

```text
  E0h,5Ah  JOY-A (alternate to normal scancode 86h)
  E0h,6Bh  LEFT
  E0h,72h  DOWN
  E0h,74h  RIGHT
  E0h,75h  UP
```

#### Notes

The Numeric-Keypad (NUM) isn't present on the existing X-Band keyboard. There is probably only one of the two backslash keys (5Ch/5Dh) and only one of the two Button-A keys (86h/E0h,5Ah) implemented.

There are three keyboard LEDs (Num,Caps,Scroll) visible in upper right on some photos (not visible on other photos; either due to bad photo quality, or maybe some keyboards have no LEDs). The Caps LED is controlled via software (unknown if Num/Scroll LEDs can be controlled, too).

There are several "unused" keys (which aren't used by the BIOS), unknown if/which scancodes are assigned to them (12 noname keys in upper left, 1 noname key in lower left, two control keys, and two xband logo keys). For the two shift keys it's also unknown which one has which scancode.

Unknown if the japanese BIOS includes support for japanese symbols, and unknown if there was a keyboard released in japan. (Note: With an emulated US keyboard, the Japanese BIOS does realize cursor/enter keys, it does also store typed ASCII characters in a ring-buffer at [3BB6+x]; but, for whatever reason, does then ignore those characters).
