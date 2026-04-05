# SNES Memory OAM Access (Sprite Attributes)

```text
2102h/2103h - OAMADDL/OAMADDH - OAM Address and Priority Rotation (W)
  15    OAM Priority Rotation  (0=OBJ #0, 1=OBJ #N) (OBJ with highest priority)
  9-14  Not used
  7-1   OBJ Number #N (for OBJ Priority)   ;\bit7-1 are used for two purposes
  8-0   OAM Address   (for OAM read/write) ;/
```

This register contains of a 9bit Reload value and a 10bit Address register (plus the priority flag). Writing to 2102h or 2103h does change the lower 8bit or upper 1bit of the Reload value, and does additionally copy the (whole) 9bit Reload value to the 10bit Address register (with address Bit0=0 so next access will be an even address).

Caution: During rendering, the PPU is destroying the Address register (using it internally for whatever purposes), after rendering (at begin of Vblank, ie. at begin of line 225/240, but only if not in Forced Blank mode) it reinitializes the Address from the Reload value; the same reload occurs also when deactivating forced blank anytime during the first scanline of vblank (ie. during line 225/240).

```text
2104h - OAMDATA - OAM Data Write (W)
2138h - RDOAM - OAM Data Read (R)
  1st Access: Lower 8bit (even address)
  2nd Access: Upper 8bit (odd address)
```

Reads and Writes to EVEN and ODD byte-addresses work as follows:

```text
  Write to EVEN address      -->  set OAM_Lsb = Data    ;memorize value
  Write to ODD address<200h  -->  set WORD[addr-1] = Data*256 + OAM_Lsb
  Write to ANY address>1FFh  -->  set BYTE[addr] = Data
  Read from ANY address      -->  return BYTE[addr]
```

The address is automatically incremented after every read or write access.

OAM Size is 220h bytes (addresses 220h..3FFh are mirrors of 200h..21Fh).

#### OAM Content

> **See:** [SNES PPU Sprites (OBJs)](snes-ppu-sprites-objs.md)
