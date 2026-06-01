# SNES Add-On SFC Modem - Misc

"The Modem as far as I know only had one function and that was to allow you to do online betting via the official JRA (Japanese Horse Racing) online service.

The modem ran on the NTT lines which probably means that NTT (Nippon Telecommunications) also had something to make out of this service or at least they thought they did :-D"

```text
  JRA = Japan Racing Association (japanese horse racing)
  PAT = Personal Access Terminal (for telephone/online betting)
  NTT = Nippon Telegraph and Telephone (japanese telecommunications)
```

#### NTT JRA PAT (1997) (2400 Baud version) (J)

```text
  baud rates seem to be 2400,1200 (see ROM 03:8910) (and "AT%B" strings)
  uses standard AT-commands (ATI0, ATS0?, ATD, ATX1, etc.)
  supports AMD FLASH only
  there are two ROM versions (SHVC-TJAJ-0 and SHVC-TJBJ-0)
```

NTT JRA PAT - Wide Baken Taiyou (1999) (9600 Baud version) (J)

```text
  baud rates seem to be 9600,2400,1200 (see ROM 03:87F0) (and "AT%B" strings)
  uses standard AT-commands (ATI0, ATS0?, ATD, ATX1, etc.)
  supports AMD/ATMEL/SHARP FLASH
  there are two ROM versions (SHVC-TJDJ-0 and SHVC-TJEJ-0)
```

#### Zaitaku Touhyou System - SPAT4-Wide (1999 or so)

```text
  unknown, reportedly also horse betting with NTT modem
  there is one ROM version (SHVC-TOBJ-0)
```

There is no special "modem-hardware" entry in cartridge header, but: note that all NTT/SFC modem BIOS have special "SHVC-Txxx-x" game codes.
