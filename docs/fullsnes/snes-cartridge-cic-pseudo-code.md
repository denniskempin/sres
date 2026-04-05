# SNES Cartridge CIC Pseudo Code

#### CicMain

```text
  CicInitFirst, CicInitTiming, CicRandomSeed, CicInitStreams
  time=data_start, a=1, noswap=1, if snes then noswap=0
```

mainloop:

```text
  for x=a to 0Fh
    if nes then Wait(time-5), else if snes then (time-7)     ;\verify idle
    if (nes_6113=0) and (P0.0=1 or P0.1=1) then Shutdown     ;/
    Wait(time+0)                                             ;\
    if (console xor snes) then a=[00h+x].0, else a=[10h+x].0 ; output data
    if noswap then P0.0=a, else P0.1=a                       ;/
    Wait(time+2-data_rx_error)                               ;\
    if (console xor snes) then a=[10h+x].0, else a=[00h+x].0 ; verify input
    if noswap then a=(a xor P0.1), else a=(a xor P0.0)       ;
    if a=1 then Shutdown                                     ;/
    Wait(time+3)                                             ;\output idle
    if noswap then P0.0=0, else P0.1=0                       ;/
    if snes then time=time+92, else if nes then time=time+79
  next x
  CicMangle(00h), CicMangle(10h)                        ;\mangle
  if snes then CicMangle(00h), CicMangle(10h)           ; (thrice on SNES)
  if snes then CicMangle(00h), CicMangle(10h)           ;/
  if snes then noswap=[17h].0   ;eventually swap input/output pins (SNES only)
  a=[17h]
  if a=0 then a=1, time=time+2
  if snes then time=time+44, else if nes then time=time+29
  goto mainloop
```

#### CicMangle(buf)

```text
  for i=[buf+0Fh]+1 downto 1
    a=[buf+2]+[buf+3h]+1
    if a<10h then x=[buf+3], [buf+3]=a, a=x, x=1, else x=0
    [buf+3+x]=[buf+3+x]+a
    for a=x+6 to 0Fh, [buf+a]=[buf+a]+[buf+a-1]+1, next a
    a=[buf+4+x]+8, if a<10h then [buf+5+x]=[buf+5+x]+a, else [buf+5+x]=a
    [buf+4+x]=[buf+4+x]+[buf+3+x]
    [buf+1]=[buf+1]+i
    [buf+2]=NOT([buf+2]+[buf+1]+1)
    time=time+84-(x*6)
  next i
```

Note: All values in [buf] are 4bit wide (aka ANDed with 0Fh).

#### CicInitFirst

```text
  timer=0                       ;reset timer (since reset released)
  P0=00h
  console=P0.3                  ;get console/cartridge flag
  if console
    while P0.2=1, r=r+1         ;get 4bit random seed (capacitor charge time)
    P1.1=1, P1.1=0              ;issue reset to CIC in cartridge
    timer=0                     ;reset timer (since reset released)
  if nes_6113 and (console=1)
    Wait(3), nes_6113_in_console=1, P0.0=1      ;request special 6113 mode
  if nes_6113 and (console=0)
    Wait(6), nes_6113_in_console=P0.1           ;check if 6113 mode requested
```

#### CicRandomSeed

```text
  time=seed_start
  for i=0 to 3                  ;send/receive 4bit random seed (r)
    bit=((i+3) and 3)           ;bit order is 3,0,1,2 (!)
    if console=1 Wait(time+0+i*15), P0.0=r.bit, Wait(time+3+i*15), P0.0=0 ;send
    if console=0 Wait(time+2+i*15), r.bit=P0.1                            ;recv
  next i
```

#### CicInitStreams

```text
  if snes
    if ntsc then x=9, else if pal then x=6
    [01h..0Fh]=B,1,4,F,4,B,5,7,F,D,6,1,E,9,8   ;init stream from cartridge (!)
    [11h..1Fh]=r,x,A,1,8,5,F,1,1,E,1,0,D,E,C   ;init stream from console   (!)
  if nes_usa                ;3193A
    [01h..0Fh]=1,9,5,2,F,8,2,7,1,9,8,1,1,1,5   ;init stream from console
    [11h..1Fh]=r,9,5,2,1,2,1,7,1,9,8,5,7,1,5   ;init stream from cartridge
    if nes_6113_in_console then overwrite [01h]=5 or so ???   ;special-case
  if nes_europe             ;3195A
    [01h..0Fh]=F,7,B,E,F,8,2,7,D,7,8,E,E,1,5   ;init stream from console
    [11h..1Fh]=r,7,B,D,1,2,1,7,E,6,7,A,7,1,5   ;init stream from cartridge
  if nes_hongkong_asia      ;3196A
    [01h..0Fh]=E,6,A,D,F,8,2,7,E,6,7,E,E,E,A   ;init stream from console
    [11h..1Fh]=r,6,A,D,E,D,E,8,E,6,7,A,7,1,5   ;init stream from cartridge
  if nes_uk_italy_australia ;3197A
    [01h..0Fh]=3,5,8,9,3,7,2,8,8,6,8,5,E,E,B   ;init stream from console
    [11h..1Fh]=r,7,9,A,A,1,6,8,5,8,9,1,5,1,7   ;init stream from cartridge
  if_nes_famicombox         ;3198A
    (unknown)
```

Note: In most cases, the PAL region changes are simply inverted or negated NTSC values (not/neg), except, one NES-EUR value, and most of the NES-UK values are somehow different. The rev-engineered NES-UK values may not match the exact original NES-UK values (but they should be working anyways).

#### CicInitTiming

```text
  if snes_d411           -> seed_start=630, data_start=817   ;snes/ntsc
  if snes_d413           -> (unknown?) (same as d411?)       ;snes/pal
  if nes_3193            -> (seems to be same as nes_3195?)  ;nes/usa (v1)
  if nes_3195            -> seed_start=32, data_start=200    ;nes/europe
  if nes_3196            -> (unknown?)                       ;nes/asia
  if nes_3197            -> (unknown?) ("burns five")        ;nes/uk
  if nes_6113            -> seed_start=32, data_start=201    ;nes/usa (v2)
  if nes_6113_in_console -> seed_start=33, data_start=216    ;nes/special
  if nes_tengen          -> seed_start=32, data_start=201    ;nes/cic-clone
  ;now timing errors...
  data_rx_error=0  ;default
  if console=0 and nes_3193a -> randomly add 0 or 0.25 to seed_start/data_start
  if console=0 and snes_d413 -> always add 1.33 to seed_start/data_start (bug)
  if console=0 and nes_6113  -> data_rx_error=1 (and maybe +1.25 on seed/data?)
  if other_chips & chip_revisions -> (unknown?)
```

Note: 3197 reportedly "burns five extra cycles before initialization", but unknown if that is relative to 3193 <or> 3195 timings, and unknown if it applies to <both> seed_start and data_start, and unknown if it means 1MHz <or> 4MHz cycles.

Note: The "data_rx_error" looks totally wrong, but it is somewhat done intentionally, so there might be a purpose (maybe some rounding, in case 6113 and 3193 are off-sync by a half clock cycle, or maybe an improper bugfix in case they are off-sync by 1 or more cycles).

#### Wait(time)

Wait until "timer=time", whereas "timer" runs at 1MHz (NES) or 1.024MHz (SNES).

The "time" values are showing the <completion> of the I/O opcodes (ie. the I/O opcodes <begin> at "time-1").

Shutdown (should never happen, unless cartridge is missing or wrong region)

```text
  a=0, if nes then time=830142, else if snes then time=1037682
```

#### endless_loop:          ;timings here aren't 100.000% accurate

```text
  if nes_3195 then time=xlat[P1/4]*174785  ;whereas, xlat[0..3]=(3,2,4,5)
  if (console=0) and (snes or nes_6113) then P0=03h, P1=01h
  if (console=1) then P1=a, Wait(timer+time), a=a xor 4  ;toggle reset on/off
  goto endless_loop
```
