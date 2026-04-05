# SNES Unpredictable Things

#### Open Bus

Garbage appears on reads from unused addresses (see below), on reads from registers that contain less than 8 bits (see more below), and on reads from (most) write-only registers (see even more below). The received data is the value being previously output on the data bus. In most cases, that is the last byte of the opcode (direct reads), or the upper byte of indirect address (indirect reads), ie.

```text
  LDA IIJJ,Y   aka MOV A,[IIJJ+Y]  --> garbage = II
  LDA (NN),Y   aka MOV A,[[NN]+Y]  --> garbage = [NN+1]
```

When using General Purpose DMA, it'd be probably rely on the store opcode that has started the DMA (in similar fashion as above loads). In case of HDMA things would be most unpredictable, garbage would be whatever current-opcode related value. And, if two DMA transfers follow directly after each other, garbage would probably come from the previous DMA.

Also, in 6502-mode (and maybe also in 65816-mode), the CPU may insert dummy-fetches from unintended addresses on page-wraps?

#### Unused Addresses in the System Region

```text
  Address       Size
  2000h..20FFh  100h    ;unused addresses
  2181h..2183h  3       ;write-only WRAM Address registers
  2184h..21FFh  7Ch     ;unused addresses in the B-BUS region
  2200h..3FFFh  1E00h   ;unused addresses
  4000h..4015h  16h     ;unused slow-CPU Ports
  4018h..41FFh  1E8h    ;unused slow-CPU Ports
  4200h..420Dh  0Eh     ;write-only CPU Ports
  420Eh..420Fh  2       ;unused CPU Ports
  4220h..42FFh  E0h     ;unused CPU Ports
  43xCh..43xEh  3*8     ;unused DMA Ports
  4380h..7FFFh  3C80h   ;unused/expansion area
```

So, of the total of 8000h bytes, a large number of 5EF4h is left unused.

Ports 2144h..217Fh are APU mirrors, NOT open bus.

#### Unused bits (in Ports with less than 8 used bits)

```text
  Addr  Mask Name    Unused Bits
  4016h FCh  JOYA    Bit7-2 are open bus
  4017h E0h  JOYB    Bit7-5 are open bus
  4210h 70h  RDNMI   Bit6-4 are open bus
  4211h 7Fh  TIMEUP  Bit6-0 are open bus
  4212h 3Eh  HVBJOY  Bit5-1 are open bus
```

#### PPU1 Open Bus

PPU1 Open Bus is relies on the value most recently read from Ports 2134h-2136h, 2138h-213Ah, 213Eh. This memorized value shows up on later reads from read-only Ports 21x4h..21x6h and 21x8h..21xAh (with x=0,1,2) (in all 8bits), as well as in Port 213Eh.Bit4.

#### PPU2 Open Bus

PPU2 Open Bus is relies on the value most recently read from Ports 213Bh-213Dh, 213Fh. This memorized value shows up on later reads from Port 213Bh.2nd_read.Bit7, 213Ch/213Dh.2nd_read.Bits7-1, and Port 213Fh.Bit5.

#### PPU Normal Open Bus

Other write-only PPU registers & the HV-latch "strobe" register are acting like "normal" CPU Open Bus (ie. usually returning the most recent opcode byte).

These are 21x0h..21x3h, 21x7h (with x=0,1,2,3), and 21xBh..21xFh (with x=0,1,2).

#### Open Bus for DMA

DMA cannot read from most I/O ports, giving it some additional open bus areas:

```text
  2100h-21FFh  Open Bus (when used as A-Bus) (of course they work as B-Bus)
  4000h-41FFh  Open Bus (name 4016h/4017h cannot be read)
          actually, 4017h <does> return bit4-2 set (1=GNDed joypad input)
  4210h-421Fh  These do work (the only I/O ports that are not open bus)
  4300h-437Fh  Special Open Bus (DMA registers, may return [PC] instead of FFh)
```

For DMA reads, one may expect the same garbage as for CPU reads (ie. when starting a DMA via "MOV [420Bh],A", one would expect 42h as open bus value).

However, it takes a few cycles before the DMA transfer does actually start, during that time the hardware "forgets" the 42h value, and instread, DMA does always read FFh (High-Z) as open bus value. With two exceptions: If DMA wraps from an used to unused address (eg. from 1FFFh/WRAM to 2000h/unused) then the first "unused" byte will be same as the last "used" byte, thereafter, it forgets that value (and returns FFh on further unused addresses).

The other exception is if DMA <starts> at 4300h..437Fh: In this case it will read [PC] for that region (and will also "memorize" it when reaching the first unused address at 4380h, and then returns FFh for 4381h and up). For example:

"MOV [420Bh],A" followed by "ADC A,33h" would return 69h (the first byte of the ADC opcode). This effect occurs only if the transfer <starts> in that region, ie. if starts below that area, and does then wrap from 42FFh to 4300h, then it returns FFh instead of 69h. (The reason of this special effect is probably that the DMA somehow "ignores the databus", so external HIGH-Z levels (like from XBOO cable or Cartridge) cannot drag the "memorized" value to HIGH.) EDIT: The above "memorize for next ONE unused address" applies only when XBOO cable is connected (which pulls the databus to HIGH rather quickly). Without XBOO cable (and without cartridge connected) the "memorized" value may last for the next 2000h (!) unused addresses, then it may slowly get corrupted (some bits going to HIGH state, until, after some more time, all bits are HIGH).

Actually, it seems to last even longer than 2000h -- possibly forever (until DMA ends, or until it reaches a used address).

SPC700 Division Overflow/Result (DIV YA,X opcode)

The overall division mechanism (with and without overflows) is:

```text
  H = (X AND 0Fh)<=(Y AND 0Fh)   ;half carry flag (odd dirt effect)
  Temp = YA
  FOR i=1 TO 9
    Temp=Temp*2                                     ;\rotate within 17bits
    IF Temp AND 20000h THEN Temp=(Temp XOR 20001h)  ;/
    IF Temp>=(X*200h)  THEN Temp=(Temp XOR 1)
    IF Temp AND 1      THEN Temp=(Temp-(X*200h)) AND 1FFFFh
  NEXT i
  A = (Temp AND FFh) ;result.bit7-0
  V = (Temp.Bit8=1)  ;result.bit8
  Y = (Temp/200h)    ;remainder (aka temp.bit9-16)
  N = (A.Bit7=1)     ;sign-flag (on result.bit7) (though division is unsigned)
  Z = (A=00h)        ;zero-flag (on result.bit7-0)
```

That is, normally (when result = 0000h..00FFh):

```text
  A=YA/X, Y=YA MOD X, N=ResultingA.Bit7, Z=(ResultingA=00h), V=0, H=(see above)
```

An intact 9bit result can be read from V:A (when result = 0000h..01FFh).

Otherwise return values are useless garbage (when result = 0200h..Infinite).
