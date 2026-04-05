# SNES Cart Copiers - Misc

#### Parallel Ports (DB-25)

Parallel Ports are used to upload/download data from PCs. Later copiers seem to be additionally using the Parallel Port for connecting CD-ROM drives.

Some copiers have fully working parallel ports installed (eg. Front Fareast), some have them incompletely installed (eg. some Supercom seem to require additional 74LS245 (8-bit 3-state transceiver), and a specially programmed PAL16V8 chip?).

Other copiers don't have any provisions for parallel ports onboard - but can be eventually upgraded externally by plugging a parallel port cartridge into the SNES cartridge slot: There are at least two such upgrade cartridges (one contains pure logic, the other one additionally contains a BIOS upgrade).

#### DSP Chips

Some copiers include DSP-clones onboard (or do at least have sockets or soldering points for mounting DSP chips), other copiers can be upgraded externally: By plugging a DSP cartridge into the SNES cartridge slot (either a regular game cartridge with DSP chip, or a plain DSP-clone-cartridge without any game in it).

Of course, this will work only with the correct DSP chip (DSP1 for most games; unless there are any DSP clones that support more than one DSP chip at one?).

Another problem may be I/O addresses (different games expect DSP chips at different addresses).

#### Batteries

Some boards contain batteries for the internal SRAM. Either 3V Lithium cells (coin-shaped), or rechargeable 3.6V NiCd batteries (usually with blue coating, which tend to leak acid, and to destroy wires on the PCB). Other boards don't have any batteries at all (they are said to use capacitors instead of batteries, which might be nonsense, or might last only a few minutes?) (there seems to be no way to switch-off the external power-supply, so batteries aren't needed to power SRAM) (eventually some boards might even power the DRAM in standby mode (?), which would require a DRAM refresh generator). And, aside from battery-backup, most or all copiers are allowing to save SRAM to floppy.
