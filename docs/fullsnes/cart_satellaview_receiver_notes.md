# SNES Cart Satellaview I/O Receiver Data Streams (Notes)

#### Resetting the Queues

Clearing the Status & Data Queues is needed on power-up, after Overrun, or after changing the Hardware Channel number. The procedure is:

```text
  MOV A,01h     ;\
  MOV [218Bh],A ; must be executed in FAST memory (at 3.58MHz) (otherwise the
  NOP           ; the Status Queue may be not in sync with the Data Queue)
  NOP           ; (for Stream 2 do the same with Port 2192h/2193h accordingly,
  NOP           ; though the existing games that do use Stream 2 are including
  NOP           ; several near-excessive timing bugs in that section)
  MOV [218Ch],A ;/
```

Thereafter, Status & Data queue are empty, and the Queue Size register is 00h (both 7bit counter, and Overrun flag cleared).

#### Reading the Queues

```text
  N=[218Ah]                                      ;-get queue size
  if N=0 then exit                               ;-exit if no data in queues
  if N.Bit7=1 then reset_queue/abort_packet/exit ;-handle overrun error
  N=max(20,N)                                    ;-limit to max 20 (if desired)
  for i=0 to (N-1), stat[i]=[219Bh], next        ;-read status units
  stat_summary=[219Dh]                           ;-get status summary
  for i=0 to (N*22-1), data[i]=[219Ch], next     ;-read data units
```

#### Channel Disable

After receiving a full packet, the BIOS issues a "MOV [218Ch],00h", this might acknowledge something, or (more probably) disable the Channel so that no new data is added to the Queue. The mechanism for re-enabling the channel is unknown (prossibly resetting the Queue, or writing the Channel register). For Stream 2, "MOV [2192h],00h" should do the same thing.

#### Overrun Notes

Overrun means that one hasn't processed the queues fast enough. If so, one should Reset the queues and discard any already-received incomplete packet fragments. There seems to be no problem if an overrun occurs WHILE reading from the queue (ie. overrun seems to stop adding data to the queue, rather than overwriting the old queued data) (of course AFTER reading the queue, one will need to handle the overrun, ie. discard all newer data).

Note: Stream 1 can queue 127 frames (presumably plus 1 incomplete frame, being currently received). As far as known, Stream 2 is used only for Time Channels (with single-frame packets), so it's unknown if Stream 2 is having the same queue size.

#### Packet Start/End Flags

The status queue values (with start/end bits isolated) would be:

```text
  90h               ;packet is 1 frame  (10-byte header + 12-byte data)
  10h,80h           ;packet is 2 frames (10-byte header + 34-byte data)
  10h,00h,80h       ;packet is 3 frames (10-byte header + 56-byte data)
  10h,00h,00h,80h   ;packet is 4 frames (10-byte header + 78-byte data)
```

and so on. For Channel Map, header is only 5-byte, and data is 5 bigger.

Caution: After having received the header (ie. at time when receiving the data), the BIOS treats the Header-Start Flag in the Status Summary register (!) as Error-Flag, to some point that makes sense in the Data-phase, but it will cause an error if a new header frame is received shortly AFTER the Data-phase.

As a workaround, the transmitter should not send new Packet Fragments (on the same Hardware Channel) for at least 1/60 seconds (preferably longer, say 1/20 seconds) after the end of the last Data Frame.

#### Transfer Rate

The transfer rate is unknown. Aside from the actual transmission speed, the effective download rate will also depend on how often data is transmitted on a specific channel (there are probably pauses between packet fragments, and maybe also between 22-byte frames), this may vary depening on how many other packets are transmitted, and how much priority is given to individual packets.

The download rate is slow enough for allowing the BIOS to write incoming data directly to FLASH memory. Moreover, the BIOS Vblank NMI Handler processes only max twenty 22-byte frames per 60Hz PPU frame. Knowing that, the download rate must be definetly below 26400 bytes/second (20*22*60).

As far as known, the Satellaview broadcasts replaced former St.GIGA radio broadcasts, assuming that the radio used uncompressed CD quality (2x16bit at 44.1kHz), and assuming that Satellaview used the same amount of data, the transfer rate may have been 176400 bytes/second (which could have been divided to transfer 8 different packets at 22050 bytes/second, for example).
