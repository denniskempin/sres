---
title: "APU register table/SMP"
source_url: "https://snes.nesdev.org/wiki/APU_register_table/SMP"
pageid: 182
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

See: [[APU register table]]

This table lists the 2 common names for the S-SMP registers.

| Name | | Address | Bits | Type | Notes |
| --- | --- | --- | --- | --- | --- |
| [[S-SMP#TEST|TEST]] | Test | $F0 | IIEE TRWH | W8 | Undocumented test register. |
| [[S-SMP#CONTROL|CONTROL]] | Control | $F1 | I.CC .210 | W8 | Enable IPL ROM (I), Clear data ports (C), timer enable (2,1,0). |
| [[S-SMP#DSPADDR|DSPADDR]] | Register Address | $F2 | RAAA AAAA | RW8 | Selects a DSP register address. |
| [[S-SMP#DSPDATA|DSPDATA]] | Register Data | $F3 | DDDD DDDD | RW8 | Reads or writes data to the selected DSP address. |
| [[S-SMP#CPUIO|CPUIO0]] | Port 0 | $F4 | DDDD DDDD | RW8 | Reads or writes data to [[MMIO registers#APUIO0|APUIO0]]. |
| [[S-SMP#CPUIO|CPUIO1]] | Port 1 | $F5 | DDDD DDDD | RW8 | Reads or writes data to [[MMIO registers#APUIO1|APUIO1]]. |
| [[S-SMP#CPUIO|CPUIO2]] | Port 2 | $F6 | DDDD DDDD | RW8 | Reads or writes data to [[MMIO registers#APUIO2|APUIO2]]. |
| [[S-SMP#CPUIO|CPUIO3]] | Port 3 | $F7 | DDDD DDDD | RW8 | Reads or writes data to [[MMIO registers#APUIO3|APUIO3]]. |
|  | --- | $F8 | .... .... | RW8 | Unused (normal RAM). |
|  | --- | $F9 | .... .... | RW8 | Unused (normal RAM). |
| [[S-SMP#TxTARGET|T0TARGET]] | Timer 0 | $FA | TTTT TTTT | W8 | 8KHz timer 0 interval. |
| [[S-SMP#TxTARGET|T1TARGET]] | Timer 1 | $FB | TTTT TTTT | W8 | 8KHz timer 1 interval. |
| [[S-SMP#TxTARGET|T2TARGET]] | Timer 2 | $FC | TTTT TTTT | W8 | 64KHz timer 2 interval. |
| [[S-SMP#TxOUT|T0OUT]] | Counter 0 | $FD | 0000 CCCC | R8 | Timer 0 count-up. |
| [[S-SMP#TxOUT|T1OUT]] | Counter 1 | $FE | 0000 CCCC | R8 | Timer 1 count-up. |
| [[S-SMP#TxOUT|T2OUT]] | Counter 2 | $FF | 0000 CCCC | R8 | Timer 2 count-up. |
