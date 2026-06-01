# SNES Cart Copiers - Floppy Disc FAT12 Format

The SNES Copier floppy format is compatible to that used under DOS on PCs.

Typical formats are 3.5", Double Density, 80 Tracks/9 Sectors, Double Sided (720KB). The Sectors are logically numbered 01h..09h, and each sized 200h bytes.

#### XXX HD-disks have more sectors

XXX snes copiers are usually HD (or maybe some are DD?) XXX snes copiers support 1.44MB and 1.6MB (FDFORMAT-like)

#### Boot-Record

The first sector is always used as bootsector, giving information about the usage of the following sectors, and including the boot procedure (for loading MSDOS etc).

```text
  00-02       80x86 boot procedure (jmp opcode) (not used for SNES)
  03-0A       ascii disk name
  0B-0C       bytes / sector
  0D          sectors / cluster
  0E-0F       sectors / boot-record
  10          number of FAT-copys
  11-12       entrys / root-directory
  13-14       sectors / disk
  15          ID: F8=hdd, F9=3.5", FC=SS/9sec, FD=DS9, FE=SS8,FF=DS8
  16-17       sectors / FAT
  18-19       sectors / track
  1A-1B       heads / disk
  1C-1D       number of reserved sectors
  1E-1FF      MSX boot procedure (Z80 code) (not used for SNES)
```

#### FAT and FAT copy(s)

The following sectors are occupied by the File Allocation Table (FAT), which contains 12- or 16-bit entries for each cluster:

```text
  (0)000      unused, free
  (0)001      ???
  (0)002...   pointer to next cluster in chain (0)002..(F)FEF
  (F)FF0-6    reserved (no part of chain, not free)
  (F)FF7      defect cluster, don't use
  (F)FF8-F    last cluster of chain
```

Number and size of FATs can be calculated by the information in the boot sector.

#### Root directory

The following sectors are the Root directory, again, size depends on the info in bootsector. Each entry consists of 32 bytes:

```text
  00-07       Filename (first byte: 00=free entry,2E=dir, E5=deleted entry)
  08-0A       Filename extension
  0B          Fileattribute
  0C-15       reserved
  16-17       Timestamp: HHHHHMMM, MMMSSSSS
  18-19       Datestamp: YYYYYYYM, MMMDDDDD
  1A-1B       Pointer to first cluster of file
  1C-1F       Filesize in bytes
```

The 'cluster' entry points to the first used cluster of the file. The FAT entry for that cluster points to the next used cluster (if any), the FAT entry for that cluster points to the next cluster, and so on.

#### Reserved Sectors (if any)

Usually the number of reserved sectors is zero. If it is non-zero, then the following sector(s) are reserved (and could be used by the boot procedure for whatever purposes).

#### Data Clusters 0002..nnnn

Finally all following sectors are data clusters. The first cluster is called cluster number (0)002, followed by number (0)003, (0)004, and so on.

#### Special Features

Unknown if any copiers support sub-directories.

Unknown if any copiers support long file names.

Unknown if any copiers support compressed files (ZIP or such).
