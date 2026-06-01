# SNES Cart LoROM Mapping (ROM divided into 32K banks) (around 1500 games)

#### Plain LoROM

```text
  Board Type               ROM Area               ROM Mirrors
  SHVC-1A0N-01,02,10,20,30 00-7D,80-FF:8000-FFFF  40-7D,C0-FF:0000-7FFF
  SHVC-2A0N-01,10,11,20    00-7D,80-FF:8000-FFFF  40-7D,C0-FF:0000-7FFF
  SHVC-BA0N-01,10          00-7D,80-FF:8000-FFFF  40-7D,C0-FF:0000-7FFF
  SHVC-YA0N-01             00-7D,80-FF:8000-FFFF  40-7D,C0-FF:0000-7FFF
```

#### LoROM with SRAM

```text
  Board Type               ROM Area               SRAM Area
  SHVC-1A1B-04,05,06       00-1F,80-9F:8000-FFFF  70-7D,F0-FF:0000-FFFF
  SHVC-1A3B-11,12,13       00-1F,80-9F:8000-FFFF  70-7D,F0-FF:0000-FFFF
  SHVC-1A5B-02,04          00-1F,80-9F:8000-FFFF  70-7D,F0-FF:0000-FFFF
  SHVC-2A3B-01             00-3F,80-BF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-2A3M-01 with MAD-R  00-3F,80-BF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-2A3M-01,11,20       00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-1A3B-20             00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-1A1M-01,11,20       00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-2A1M-01             00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-BA1M-01             00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-1A3M-10,20,21,30    00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-BA3M-01             00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-1A5M-01,11,20       00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-2A5M-01             00-7D,80-FF:8000-FFFF  70-7D,F0-FF:0000-7FFF
  SHVC-1A7M-01             ?                      ?
```

Note that 2A3M-01 exists with/without MAD-R (and have different mappings).

Note that 1A3B-20 differs from earlier 1A3B-xx versions.

The older boards map SRAM to the whole 64K areas at banks 70h-7Dh/F0-FFh.

The newer boards map SRAM to the lower 32K areas at banks 70h-7Dh/F0-FFh (this allows "BigLoROM" games to use the upper 32K of that banks as additional LoROM banks, which is required for games with more than 3MB LoROM).

Most of the existing boards contain 0K, 2K, 8K, or 32K SRAM. A few games contail 64K or 128K SRAM, which is divided into 32K chunks, mapped to bank 70h, 71h, etc.)

Some LoROM games are bigger than 2Mbytes (eg. Super Metroid, Gunple, Wizardry 6, Derby Stallion 3), these have bank 0-3Fh mapped in the 32K LoROM banks as usually, and bank 40h and up each mapped twice in the 64K hirom banks.

Note: There's also a different "SpecialLoROM" mapping scheme for 3MByte ROMs (used by Derby Stallion 96 and Sound Novel Tsukuru; aside from the special ROM mapping, these cartridges have an additional Data Pack Slot).
