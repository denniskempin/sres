---
summary: Describes the SNES cartridge CIC (lockout) chip, a 4-bit security CPU that performs lock-and-key authentication with a matching CIC in the console, resetting the system when the exchange fails. References cover the pseudo code, instruction set, version differences, pinouts, and disable mods.
keywords: CIC, lockout chip, authentication, security, region lock
importance: 2
---

# SNES Cartridge CIC Lockout Chip

SNES cartridges are required to contain a CIC chip (security chip aka lockout chip). The CIC is a small 4bit CPU with built-in ROM. An identical CIC is located in the SNES console. The same 4bit CPU (but with slightly different code in ROM) is also used in NES consoles/cartridges.

The CIC in the console is acting as "lock", and that in the cartridge is acting as "key". The two chips are sending random-like bitstreams to each other, if the data (or transmission timing) doesn't match the expected values, then the "lock" issues a RESET signal to the console. Thereby rejecting cartridges without CIC chip (or such with CICs for wrong regions).

#### CIC Details

> **See:** [SNES Cartridge CIC Pseudo Code](cartridge_cic_pseudo.md)
> **See:** [SNES Cartridge CIC Instruction Set](cartridge_cic_isa.md)
> **See:** [SNES Cartridge CIC Notes](cartridge_cic_notes.md)
> **See:** [SNES Cartridge CIC Versions](cartridge_cic_versions.md)
> **See:** [SNES Pinouts CIC Chips](pinouts_cic.md)

#### CIC Disable

> **See:** [SNES Common Mods](common_mods.md)
