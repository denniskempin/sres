# SNES Cartridge CIC Lockout Chip

SNES cartridges are required to contain a CIC chip (security chip aka lockout chip). The CIC is a small 4bit CPU with built-in ROM. An identical CIC is located in the SNES console. The same 4bit CPU (but with slightly different code in ROM) is also used in NES consoles/cartridges.

The CIC in the console is acting as "lock", and that in the cartridge is acting as "key". The two chips are sending random-like bitstreams to each other, if the data (or transmission timing) doesn't match the expected values, then the "lock" issues a RESET signal to the console. Thereby rejecting cartridges without CIC chip (or such with CICs for wrong regions).

#### CIC Details

> **See:** [SNES Cartridge CIC Pseudo Code](snes-cartridge-cic-pseudo-code.md)
> **See:** [SNES Cartridge CIC Instruction Set](snes-cartridge-cic-instruction-set.md)
> **See:** [SNES Cartridge CIC Notes](snes-cartridge-cic-notes.md)
> **See:** [SNES Cartridge CIC Versions](snes-cartridge-cic-versions.md)
> **See:** [SNES Pinouts CIC Chips](snes-pinouts-cic-chips.md)

#### CIC Disable

> **See:** [SNES Common Mods](snes-common-mods.md)
