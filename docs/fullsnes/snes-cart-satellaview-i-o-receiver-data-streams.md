# SNES Cart Satellaview I/O Receiver Data Streams

The receiver can be programmed to watch (and receive) two different Hardware Channels simultaneously. In practice, Stream 1 is used only by the BIOS, and Stream 2 is used only by a few BS FLASH games (Dragon Quest 1, Satella2 1, BS Fire Emblem Akaneia Senki 1, and maybe some others) (which do use it for receiving Time Channel Packets).

```text
2188h/2189h Stream 1 Hardware Channel Number, Lsb/Msb (R/W)
218Eh/218Fh Stream 2 Hardware Channel Number, Lsb/Msb (R/W)
  0-15  Hardware Channel Number (16bit)
             XXX reportedly only 14bit !?
```

Values written to these registers should be taken from the Channel Map packet (or for receiving the Channel Map itself, use fixed value 0124h). Be sure to reset the Queues after changing the channel number (so you won't receive old data from old channel).

218Ah Stream 1 Queue Size (number of received 1+22 byte Units) (R) 2190h Stream 2 Queue Size (number of received 1+22 byte Units) (R)

```text
  0-6  Number of received Units contained in the Queue (0..127)
  7    Overrun Error Flag (set when received more than 127 units)
```

Indicates how many frames are in the queues. One doesn't need to process all frames at once; when reading only a few frames, the Queue Size is decremented accordingly, and the remaining frames stay in the Queue so they can be processed at a later time. The decrement occurs either after reading 1 byte from the Status Queue, or after reading 22 bytes from the Data Queue (anyways, to keep the queues in sync, one should always read the same amount of 1/22-byte Units from both Queues, so it doesn't matter when the decrement occurs).

218Bh Stream 1 Queue 1-byte Status Units (Read=Data, Write=Reset) 2191h Stream 2 Queue 1-byte Status Unit(s?) (Read=Data, Write=Reset) Contains Header/Data Start/End flags for the received Data Frames, the format seems to be same as for Port 218Dh/2193h (see there for details) (if it's really same, then the two Error bits should be also contained in the Status Queue, though the BIOS doesn't use them in that place).

218Ch Stream 1 Queue 22-byte Data Units (Read=Data, Write=Reset/Ack) 2192h Stream 2 Queue 22-byte? Data Unit(s?) (Read=Data, Write=Reset/Ack) Contains the received Data Frames, or in case of Header Frames: The 5/10-byte Frame Header, followed by the Packet/Fragment Header, followed by the actual Data.

218Dh Stream 1 Status Summary (R) 2193h Stream 2 Status Summary (R) These registers seem to contain a summary of the Status bytes being most recently removed from the Queue. Ie. status bits are probably getting set by ORing all values being read from Port 218Ah/2190h. The bits are probably cleared after reading 218Dh/2193h.

```text
  0-1  Unknown/unused
  2-3  Error Flags (probably set on checksum errors or lost data/timeouts)
  4    Packet Start Flag (0=Normal, 1=First Frame of Packet) (with Header)
  5-6  Unknown/unused
  7    Packet End Flag   (0=Normal, 1=Last Frame of Packet)
```

Bit 2-3 are more or less self-explaining: Don't use the queued data, and discard any already (but still incompletely) received packet fragments. Bit 4,7 are a bit more complicated. See Notes in next chapter for details.

> **See:** [SNES Cart Satellaview I/O Receiver Data Streams (Notes)](snes-cart-satellaview-i-o-receiver-data-streams-notes.md)
