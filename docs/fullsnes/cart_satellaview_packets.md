# SNES Cart Satellaview Packet Headers and Frames

#### Packet Fragment Format (10-byte header)

```text
  00h 1    Transmission ID (in upper 4bit) (must stay same for all fragments)
  01h 1    Current Fragment Number (in lower 7bit)
  02h 3    Fragment Size (N) (big-endian) (excluding first 5 bytes at [00..04])
  05h 1    Fixed, Must be 01h
  06h 1    Total Number of Fragments (00h=Infinite Streaming?)
  07h 3    Target Offset (big-endian) (location of fragment within whole file)
  0Ah N-5  Data Body (first 12 bytes located directly in header frame)
  ... ...  Unused/Padding (until begin of next 22-byte Frame)
```

This is the normal format used by all packets (except Channel Map packet).

#### Channel Map Packet Format (5-byte header)

```text
  00h 1    Unknown/unused (would be 4bit Transmission ID for normal packets)
  01h 1    Unknown/unused (would be 7bit Fragment Number for normal packets)
  02h 3    Packet Size (N) (big-endian) (excluding first 5 bytes at [00..04])
  05h N    Data Body (first 17 bytes located directly in header frame)
  ... ...  Unused/Padding (until begin of next 22-byte Frame)
```

#### Frames (22-bytes)

Packets are divided into one or more 22-byte frames.

For each frame, a 1-byte status info can be read from Port 218Bh/2191h, the status contains error flags (indicating bad checksums or lost-frames or so), and header flags (indicating begin/end of header/data or so).

The 22-byte frame data can be read from Port 218Ch/2192h. If it is header frame, then its first 10 (or 5) bytes contain the header (as described above), and the remaining 12 (or 17) bytes are data. If it is a data frame, then all 22 bytes are plain data.

#### Fragmented Packets

A packet can consist of 1..128 fragments. The packet transmission is repeated several times (say, for one hour). If it is consisting of several fragments, one can start downloading anywhere, eg. with the middle fragment, and one can keep downloading even if some fragments had transmission errors (in both cases, one must download the missing fragments in the next pass(es) when the transmission is repeated).

Software should maintain a list of fragments that are already received (if a fragment is already received: just remove it from the queue without writing to target area; both for saving CPU load, and for avoiding to destroy previously received packets in case of transmission errors).

At some time (say, after an hour), transmission of the packet will end, and a different packet may be transferred on the same hardware channel. Verify the 4bit Transmission ID to avoid mixing up fragments from the old (desired) file with the new file. Ideally, that ID <should> be stored in the Channel Map or Directory (this may actually be so), however, the Satellaview BIOS simply takes the ID from the first received Fragment, and then compares it against following Fragments (if the ID changed shortly after checking the Directory, and before receiving the first fragment, then the BIOS will download a "wrong" file).

#### Fragment Size Bug

The Satellaview BIOS supports only 16bit fragment sizes, it does try to handle 24bit sizes, but if the fragment size exceeds 65535 then it does receive only the first few bytes, and then treats the whole fragment as "fully" received.

#### Text Strings

Text strings (file/folder names and descriptions) can contain ASCII (and presumably JIS and SHIFT-JIS), and following specials:

```text
  00h        End of Line (or return from a "\s" Sub-String)
  0Dh        Carriage Return Line Feed (in descriptions)
  20h..7Eh   ASCII 6pix characters (unlike SHIFT-JIS 12pix ones)
  80h..9Fh   Prefixes for double-byte characters (SHIFT-JIS)
  A0h..DFh   Japanese single-byte characters (JIS or so)
  E0h..EAh   Prefixes for double-byte characters (SHIFT-JIS)
  F0h        Prefix for Symbols (40h..51h:Music-Note,Heart,Dots,Faces,MaKenji)
  "\\"           Yen symbol (unlike ASCII, not a backslash)
  "\b0".."\b3"   Insert Username/Money/Gender/NumItems (12pix SHIFT-JIS)
  "\c0".."\c5"   Changes color or palette or so
  "\d#",p24bit   Insert 16bit Decimal at [p24bit] using 6pix-font
  "\D#",p24bit   Insert 16bit Decimal at [p24bit] using 12pix-font
  "\du#",v24bit  Insert 16bit Decimal Interpreter-Variable using 6pix-font
  "\Du#",v24bit  Insert 16bit Decimal Interpreter-Variable using 12pix-font
                    # = 00     Variable width (no leading spaces/zeroes)
                    # = 1..6   Width 1..6 chars (with leading spaces)
                    # = 01..06 Width 1..6 chars (with leading zeroes)
  "\s",ptr24bit  Insert Sub-string (don't nest with further "\s,\d,\D")
  "\g",ptr24bit  Insert Custom Graphics/Symbol (ptr to xsiz,ysiz,bitmap)
  "\i"           Carriage Return (set x=0, keep y=unchanged) (not so useful)
  "\m0".."\m3"   Flags (bit0=ForceHorizontal16pixGrid, bit1=DonNotUpdateBg3Yet)
  "\n"           Carriage Return Line Feed (same as 0Dh)
  "\p00".."\p07" Palette
  "\w00".."\w99" Character Delay in Frames (00=None)
  "\x00".."\xNN" Set Xloc
  "\y00".."\yNN" Set Yloc
```

Note: "\m","\p","\w","\x","\y" are slightly bugged (causing stack overflows when using them too often within a single string; the text output thread quits at the string-end, which somewhat 'fixes' the stack-problem).

CRLF can be used in Item Activation Messages, Folder or Download-File Descriptions.

Multi-line messages are wrapped to the next line when reaching the end of a line (the wrapping can occur anywhere within words, to avoid that effect one must manually insert CRLF's (0Dh) at suitable locations). Some message boxes are clipped to the visible number of lines, other messages boxes prompt the user to push Button-A to read further lines.

Caution: CRLF will hang in Item-Descriptions (they do work in item shops, but will hang in the Inventory menu; the only way to implement longer descriptions here is to space-pad them so that the wrapping occurs at a suitable location).
