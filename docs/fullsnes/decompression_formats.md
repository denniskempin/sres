# SNES Decompression Formats

Nintendo-specific Compression Overall Format (BSX and SFC-Box) Compressed data consists of "code/length" pairs encoded in 1 or 2 bytes:

```text
  cccNnnnn           --> code (ccc=0..6) len 5bit (Nnnnn+1)
  111cccNn.nnnnnnnn  --> code (ccc=0..7) len 10bit (Nnnnnnnnnn+1)
  11111111           --> end code (FFh)
```

The "code/length" pairs are then follwed by "src" data, or "disp" offsets (depending on the "ccc" codes). The meaning of the "ccc" codes varies from program to program (see below for how they are used by BSX and SFC-Box).

Note: As seen above, ccc=7 works only with 10bit len (not 5bit len) and only with max len=2FFh+1 (not 3FFh+1).

#### Nintendo-specific Compression Codes (BSX) (Satellaview)

Used to decompress various data (including custom person OBJs in the Directory packet). The decompression functions are at 80939Fh (to RAM) and 80951Eh (to VRAM) in the BSX-BIOS. The meaning of the "ccc" codes is:

```text
  0  Copy_bytes_from_src
  1  Fill_byte_from_src
  2  Fill_word_from_src
  3  Fill_incrementing_byte_from_src
  4  Copy_bytes_from_dest_base_plus_16bit_disp
  5  Copy_bytes_from_dest_base_plus_16bit_disp_with_invert
  6  Copy_bytes_from_current_dest_addr_minus_8bit_disp
  7  Copy_bytes_from_current_dest_addr_minus_8bit_disp_with_invert
```

For all codes (including ccc=2), len is the number of BYTEs to be copied/filled. For ccc=4..6, the code is followed by a 16bit offset in LITTLE-ENDIAN format. For ccc=5/7, copied data is inverted (XORed with FFh).

Nintendo-specific Compression Codes (SFC-Box) (Super Famicom Box) Used (among others) to decompress the Title-Bitmaps in "GROM" EPROMs, the decompression function is at 0088A2h in the "ATROM" menu program. The meaning of the "ccc" codes is:

```text
  0  Copy_bytes_from_src
  1  Fill_byte_from_src
  2  Fill_word_from_src
  3  Fill_incrementing_byte_from_src
  4  Copy_bytes_from_dest_base_plus_16bit_disp
  5  Copy_bytes_from_dest_base_plus_16bit_disp_with_xflip
  6  Copy_bytes_from_dest_base_plus_16bit_disp_with_yflip
  7  Unused (same as ccc=4)
```

For ccc=2, len is the number of WORDs to be filled, for all other codes, it's the number of BYTEs to be copied/filled. For ccc=4..7, the code is followed by a 16bit offset in BIG-ENDIAN format. For ccc=5 (xflip), bit-order of all bytes is reversed (bit0/1/2/3 <--> bit7/6/5/4). For ccc=6 (yflip), reading starts at dest_base+disp (as usually), but the read-address is then decremented after each byte-transfer (instead of incremented).

#### SNES Decompression Hardware

The APU automatically decompresses BRR-encoded audio samples (4bit to 15bit ADPCM, roughly similar to CD-XA format). Cartridges with SPC7110 or S-DD1 chips can decompress (roughly JPEG-style) video data, and convert it to SNES bit-plane format. Cartridges with SA-1 chips include a "Variable-Length Bit Processing" feature for reading "N" bits from a compressed bit-stream.
