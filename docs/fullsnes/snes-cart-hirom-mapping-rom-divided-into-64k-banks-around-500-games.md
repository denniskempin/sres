# SNES Cart HiROM Mapping (ROM divided into 64K banks) (around 500 games)

#### Plain HiROM

```text
  Board               ROM Area      ROM Mirrors   SRAM Area
  Type                at 0000-FFFF  at 8000-FFFF  (none such)
  SHVC-BJ0N-01,20     40-7d,c0-ff   00-3f,80-bf   N/A
  SHVC-YJ0N-01        40-7d,c0-ff   00-3f,80-bf   N/A
  SHVC-1J0N-01,10,20  40-7d,c0-ff   00-3f,80-bf   N/A
  SHVC-2J0N-01,10,11  40-7d,c0-ff   00-3f,80-bf   N/A
  SHVC-3J0N-01        40-6f,c0-ef   00-2f,80-af   N/A
```

The SHVC-3J0N-01 board contains 3 ROM chips (memory is divided into chunks of 16 banks, with one ROM per chunk, and with each 4th chunk being left empty, ie. bank 30-3F,70-7D,B0-BF,F0-FF are open-bus).

#### HiROM with SRAM

```text
  Board               ROM Area      ROM Mirrors   SRAM Area
  Type                at 0000-FFFF  at 8000-FFFF  at 6000-7FFF
  SHVC-1J3B-01        40-7d,c0-ff   00-3f,80-bf   20-3f,a0-bf
  SHVC-1J1M-11,20     40-7d,c0-ff   00-3f,80-bf   20-3f,a0-bf
  SHVC-1J3M-01,11,20  40-7d,c0-ff   00-3f,80-bf   20-3f,a0-bf
  SHVC-BJ3M-10        40-7d,c0-ff   00-3f,80-bf   20-3f,a0-bf
  SHVC-1J5M-11,20     40-7d,c0-ff   00-3f,80-bf   20-3f,a0-bf
  SHVC-2J3M-01,11,20  40-7d,c0-ff   00-3f,80-bf   10-1f,30-3f,90-9f,b0-bf
  SHVC-2J5M-01        40-7d,c0-ff   00-3f,80-bf   10-1f,90-9f,30-3f,b0-bf
  SHVC-LJ3M-01        40-7d,c0-ff   00-3f,80-bf   80-bf
```

The SHVC-LJ3M-01 board uses ExHiROM mapping (meaning that bank 00h-7Dh contain different ROM banks than 80h-FFh).
