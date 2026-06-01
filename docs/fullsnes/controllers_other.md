# SNES Controllers Other Inputs

#### Reset Button (on the console)

Resets various CPU/PPU/APU registers, but leaves WRAM intact, so for example, games could use the button to restart the current level, or to enter the main menu.

Note: The three SPC7110 games are containing a selftest program (executed upon first boot; when battery-backed SRAM is still unitialized), the user is prompted to push the Reset Button after each test screen.

#### Super Famicom Box

This thing has a (external) coin input, two push buttons, and a 6-position switch (all these inputs are accessible only by its HD64180 CPU though, not by the SNES CPU).

The two attached joypads aren't directly wired to the SNES, instead, they are first passed to the HD64180, and then forwarded to SNES via 16bit shift registers. This allows the HD64180 to sense the "L+R+Select+Start" Soft-Reset key combination, and to inject recorded controller data in Demo/Preview mode.

On the restrictive side, the 16bit shift registers are making it incompatible with special controllers like mice (which won't work with most of the SFC-Box games anyways); unless, maybe "automatic-reading" mode bypasses the 16bit shift-register, and forwards the FULL bitstream directly from SFC-Box to SNES?

Moreover, during Game Selection, some normally unused bits of the WRIO/RDIO "controller" port are misused to communicate between KROM1 (on HD64180) and ATROM (on SNES).
