# SNES Cart Cheat Devices - Pro Action Replay I/O Ports

#### PAR1 I/O Ports (W)

```text
  008000h                  ;Code 0-3 (MID) (shared for code 0..3)
  010000h,010001h,010002h  ;Code 0 (DTA,LSB,MSB) (not used by BIOS)
  010003h                  ;Control (set to FFh)
  010004h,010005h,010006h  ;Code 1 (DTA,LSB,MSB) (not used by BIOS)
  010007h,010008h,010009h  ;Code 2 (DTA,LSB,MSB) (used as NMI vector.LSB)
  01000Ah,01000Bh,01000Ch  ;Code 3 (DTA,LSB,MSB) (used as NMI vector.MSB)
```

Most I/O ports are overlapping WRAM in bank 01h (unlike PAR2-PAR3 which use bank 10h). The four code registers would allow to apply 4 hardware patches, the PAR1 BIOS actually has provisions for doing that, but, before applying those patches it erases code 0-3 (and does then use code 2-3 for patching the NMI vector at 00FFEAh for applying the codes as WRAM software patches).

The PAR1 supports only LoROM addresses (address bit15 removed, and bit23-16 shifted down). The PAR1 does not (maybe cannot) disable unused codes; instead, it directs them to an usually unused ROM location at 00FFF6h (aka 007FF6h after removing bit15). Applying the codes is done in order DTA,MID,LSB,MSB, whereof, the "shared" MID value is probably applied on the following LSB port write.

The control register is set to FFh before starting the game, purpose is unknown (maybe enable the codes, or write-protect them, or disable the BIOS ROM; in case that can be done by software).

#### PAR2 I/O Ports (W)

```text
  100000h,100001h,100002h,100003h  ;Code 0 (DTA,LSB,MID,MSB) (code 0)
  100004h,100005h,100006h,100007h  ;Code 1 (DTA,LSB,MID,MSB) (code 1)
  100008h,100009h,10000Ah,10000Bh  ;Code 2 (DTA,LSB,MID,MSB) (code 2/NMI.LSB)
  10000Ch,10000Dh,10000Eh,10000Fh  ;Code 3 (DTA,LSB,MID,MSB) (code 3/NMI.MSB)
  100010h      ;Control A (set to 00h or FFh)
  C0A00nh      ;Control B (address LSBs n=0..7) (written data=don't care)
```

The registers overlapping the WRAM area are similar as for PAR1. The PAR2 BIOS allows to use the code registers for ROM patches (and/or hooking the NMI handler for WRAM patches).

Similar as in PAR1, address bit23-16 are shifted down, but with bit15 being moved to bit23 (still a bit messy, but HiROM is now supported). Unused codes are redirected to 00FFF6h (aka 807FF6h after moving bit15).

The control register is set to FFh before starting the game, purpose is unknown (maybe enable the codes, or write-protect them, or disable the BIOS ROM; in case that can be done by software).

The lower address bits of the newly added C0A00nh Register are:

```text
  Address bit0 - set if one or more codes use bank 7Fh..FEh
  Address bit1 - set/cleared for PAL/NTSC selection (or vice-versa NTSC/PAL?)
  Address bit2 - set to... maybe, forcing the selection in bit1 (?)
```

The exact purpose of that three bits is unknown (and their implemention in PAR2a/b BIOSes looks bugged). Doing anything special on bank 7Fh-FEh doesn't make any sense, maybe the programmer wanted to use banks 80h-FFh, but that wouldn't make much more sense either; it might be something for enabling/disabling memory mirrors or so. The default PAL/NTSC flag is auto-detected by reading 213Fh.Bit4 (the BIOS is doing that detection twice, with opposite results on each detection, which seems to be a bug), the flag can be also manually changed in the BIOS menu; purpose of the PAL/NTSC thing is unknown... maybe directing a transistor to shortcut D4 to GND/VCC when games are reading 213Fh.

#### PAR3 I/O Ports

```text
  100000h,100001h,100002h,100003h  ;Code 0 (DTA,LSB,MID,MSB) (code 0)
  100004h,100005h,100006h,100007h  ;Code 1 (DTA,LSB,MID,MSB) (code 1)
  100008h,100009h,10000Ah,10000Bh  ;Code 2 (DTA,LSB,MID,MSB) (code 2)
  10000Ch,10000Dh,10000Eh,10000Fh  ;Code 3 (DTA,LSB,MID,MSB) (code 3)
  100010h,100011h,100012h,100013h  ;Code 4 (DTA,LSB,MID,MSB) (code 4)
  100014h,100015h,100016h,100017h  ;Code 5 (DTA,LSB,MID,MSB) (always NMI.LSB)
  100018h,100019h,10001Ah,10001Bh  ;Code 6 (DTA,LSB,MID,MSB) (always NMI.MSB)
  10001Ch         ;Control A    (bit4,6,7)
  10001Dh-10001Fh ;Set to zero  (maybe accidently, trying to init "code 7")
  10003Ch         ;Control B    (set to 01h upon game start)
  086000h         ;Control LEDs (bit0,1)
  206000h         ;Control C    (bit0)
  008000h         ;Control D    (set to 00h upon PAR-NMI entry)
```

Control A (10001Ch):

```text
  Bit0-3 Should be 0
  Bit4   ROM Mapping (0=Normal, 1=Temporarily disable BIOS & enable GAME ROM)
  Bit5   Should be 0
  Bit6-7 Select/force Video Type (0=Normal, 1=NTSC, 2=PAL, 3=Reserved)
```

Control LEDs (086000h) (LEDs are in sticker area on front of cartridge):

```text
  Bit0   Control left or right LED? (0=on or off?, 1=off or on?)
  Bit1   Control other LED          ("")
  Bit2-7 Should be 0
```

Control C (206000h):

```text
  Bit0   Whatever (0=BIOS or PAR-NMI Execution, 1=GAME Execution)
  Bit1-7 Should be 0
```

Code0-6:

Unused codes are set to 00000000h (unlike PAR1/PAR2), and, codes use linear 24bit addresses (without moving/removing bit15). Of the seven codes, code 5-6 are always used for hooking the NMI handler (even when not using any WRAM software patches), so one can use max 5 hardware ROM patches.
