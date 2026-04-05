# SNES Cart CDROM - CDROM Bootsector and Volume Descriptor

SNES CD can be in MODE1 or MODE2/FORM1 format. The disc requires an 28h-byte ID in sector 16, and a 800h-byte bootsector in sector 0, which may then loaded further data via BIOS functions, or via direct access to the cdrom I/O ports.

The BIOS doesn't contain any filesystem support, however, the games may implement a standard ISO filesystem (or some custom format), if desired.

Aside from data sectors, the drive controller does also support CD-DA audio tracks and playing compressed ADPCM audio sectors.

#### SNES CD Bootsector (sector 0)

Located on Sector 0 (address 00:02:00), loaded to 00:1000h..17FFh, and then started by jumping to 00:1080h.

#### Primary Volume Descriptor (sector 16)

Located on Sector 16 (address 00:02:16), the first 28h bytes must have following values for boot-able SNES CDs.

```text
  000h 1    Volume Descriptor Type        (01h=Primary Volume Descriptor)
  001h 5    Standard Identifier           ("CD001")
  006h 1    Volume Descriptor Version     (01h=Standard)
  007h 1    Reserved                      (00h)
  008h 32   System Identifier             (a-characters) ("SUPERDISC")
  028h ...  (further ISO primary volume descriptor entries may follow here)
```

#### Note

Aside from booting executable software, the CD BIOS does also contain code for some "ELECTRONIC BOOK" format, but the volume descriptor detection lacks support for detecting that disc type.
