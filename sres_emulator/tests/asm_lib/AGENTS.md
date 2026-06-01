# `asm_lib` — SNES Test ROM Assembly Utilities

## What is this directory?

`asm_lib` is a collection of reusable 65816/SNES assembly utilities used by the **SRES emulator** integration tests. Its purpose is to make it easy to write small, self-contained test ROMs (`.sfc`) and SPC700 programs (`.spc`) that run inside the emulator and verify correctness.

Test ROMs validate CPU instructions, DMA behavior, PPU timing, APU/SPC700 operation, and more. They are assembled with the **bass** SNES assembler, and some SPC700 programs are assembled with **xa65**.

---

## File Listing

| File | Description |
|------|-------------|
| **`base.asm`** | Minimal boilerplate for simple LoROM test ROMs. Sets up the `seek` macro, fills Bank 0, and includes the standard header and register definitions. |
| **`snes_header.asm`** | Standard SNES cartridge header (LoROM, SlowROM, no RAM). Reset vector points to `$8000`. Used by most tests. |
| **`snes_header_ret.asm`** | Same header as above, but the **BRK vector** points to an external `RTIBreak` label. Used by tests that exercise software interrupts (`brk`) and need a custom RTI handler. |
| **`snes.inc`** | Massive register definition file. Defines constants for **all** SNES MMIO registers (PPU, APU, DMA, joypads, timers, etc.) and provides macros for initialization and 8/16-bit read/write helpers. |
| **`snes_gfx.inc`** | Graphics and DMA helper macros: palette loading, VRAM loading/clearing, screen fades, background scrolling, and Mode 7 matrix calculations. |
| **`snes_spc700.inc`** | SPC700/APU definitions and macros. Includes SPC700 memory map, DSP register constants, SPC upload/execution helpers, and pitch-table generation macros. |
| **`font8x8.asm`** | A 1BPP 8×8 tile font (ASCII `$20`–`$7E`). Used by CPU instruction tests (e.g., `krom_*.asm`) to print "PASS"/"FAIL" results to the screen. |

---

## How test ROMs are built

### Assembler

The project uses the **bass** assembler (v14+ syntax). A typical build command looks like:

```bash
bass dma_vram.asm
```

This produces `dma_vram.sfc` in the same directory.

### SPC700 code

Some tests (e.g., `play_noise`) assemble SPC700 programs with **xa65**:

```bash
xa65 play_noise.spc.asm
```

The resulting `.spc` binary is then embedded into a `.sfc` ROM via bass’s `insert` directive.

---

## Key macros & helpers

### `seek(variable offset)` (defined in `base.asm`)

Translates a **SNES bus address** into a **file offset** for LoROM mapping, then sets the assembly base address.

```asm
macro seek(variable offset) {
  origin ((offset & $7F0000) >> 1) | (offset & $7FFF)
  base offset
}
```

All test ROMs use this to place code at `$8000` while keeping the file layout correct for a LoROM cartridge.

### `SNES_INIT(ROMSPEED)` (defined in `snes.inc`)

A comprehensive initialization macro that:

1. Switches the CPU to **native mode** (`clc`, `xce`).
2. Sets up the stack pointer (`$1FFF`).
3. Resets the direct-page register.
4. Configures the PPU (force blank, clear BG/OBJ registers, zero scroll positions, reset Mode 7 matrix).
5. Clears **OAM**, **WRAM** (via DMA from a zero word in ROM), **VRAM**, and **CGRAM** (via DMA).
6. Disables interrupts and sets I/O ports to safe defaults.

Usage:

```asm
SNES_INIT(SLOWROM)   // or SNES_INIT(FASTROM)
```

### Register constants (`snes.inc`)

Every SNES MMIO register has a named `REG_*` constant. Examples:

| Constant | Address | Description |
|----------|---------|-------------|
| `REG_INIDISP` | `$2100` | Display control / brightness |
| `REG_BGMODE` | `$2105` | Background mode |
| `REG_VMADDL` | `$2116` | VRAM address (low) |
| `REG_VMDATAL` | `$2118` | VRAM data write (low) |
| `REG_CGADD` | `$2121` | CGRAM address |
| `REG_APUIO0` | `$2140` | APU communication port 0 |
| `REG_DMAP0` | `$4300` | DMA channel 0 parameters |
| `REG_MDMAEN` | `$420B` | Start general-purpose DMA |

> **Convention:** Registers that must be written twice for 16-bit values (e.g., scroll registers, Mode 7 matrix) are documented inline with comments like `1st Write = 0 (Lower 8-Bit)` and `2nd Write = 0 (Upper 3-Bit)`.

### Graphics macros (`snes_gfx.inc`)

| Macro | Purpose |
|-------|---------|
| `WaitNMI()` | Spin until V-Blank NMI flag (`REG_RDNMI`) is set. |
| `WaitHV()` | Spin until H/V-timer IRQ flag (`REG_TIMEUP`) is set. |
| `WaitHVB()` | Spin until inside V-Blank (`REG_HVBJOY` bit 7). |
| `FadeIN()` / `FadeOUT()` | Fade screen brightness from 0→15 or 15→0, one step per V-Blank. |
| `LoadPAL(SRC, DEST, SIZE, CHAN)` | DMA a palette into CGRAM. |
| `LoadVRAM(SRC, DEST, SIZE, CHAN)` | DMA word-sized graphics data into VRAM. |
| `ClearVRAM(SRC, DEST, SIZE, CHAN)` | Clear VRAM to a fixed word using two DMA transfers (lo/hi byte). |
| `BGScroll8(BGSCR, BGPOS, DIR)` | Scroll a background by an 8-bit amount. |
| `Mode7CALC(...)` | Compute and upload a Mode 7 transformation matrix using the PPU multiplier. |

### SPC700 macros (`snes_spc700.inc`)

| Macro | Purpose |
|-------|---------|
| `WDSP(REG, DATA)` | Write a byte to an SPC700 DSP register. |
| `SPC_INIT()` | Reset DSP state (key off, disable echo, clear flags). |
| `SPCWaitBoot()` | Wait for the SPC700 to finish its IPL boot ROM. |
| `SPCExecute(ADDR)` | Start executing uploaded code at the given SPC RAM address. |
| `TransferBlockSPC(SRC, DEST, SIZE)` | Upload a block of data from ROM into SPC RAM. |
| `WriteDSP(REG, BYTE)` | Upload a two-byte sequence to set a DSP register + data. |
| `SetPitch(voice, note, octave, C9Pitch)` | Calculate and set a voice pitch from a musical note. |
| `WritePitchTable(C9Pitch)` | Generate a 108-word pitch table (C1–B9) for sampled instruments. |

---

## Memory layout conventions

### Simple tests (`base.asm`)

For tests that only need CPU or DMA verification (no graphics), include `lib/base.asm`:

```asm
output "my_test.sfc", create
include "lib/base.asm"

// Your code starts here at $8000
```

`base.asm` does the following automatically:

1. `arch snes.cpu`
2. Defines `seek` macro.
3. Fills `$0000`–`$7FFF` (Bank 0) with zeroes.
4. Includes `snes_header.asm` and `snes.inc`.
5. Positions code at `$8000`.

### Visual / instruction tests (manual setup)

Tests that render text (e.g., `krom_*.asm`) set up memory manually:

```asm
seek($8000); fill $8000       // Zero-fill up to $7FFF
include "lib/snes.inc"
include "lib/snes_header.asm" // or snes_header_ret.asm
include "lib/snes_gfx.inc"

seek($8000); Start:
  SNES_INIT(SLOWROM)
  // ... load font, palette, print text ...
```

### WRAM usage

- **`$0000`–`$1FFF`** is the 8 KB WRAM mirror. Tests often use `$0000`–`$00FF` for temporary data and `$0100`+ for DMA read-back verification.
- `snes.inc` defines `constant WRAM = $0000`.

---

## Two header variants

| Header | Reset Vector | BRK Vector | When to use |
|--------|--------------|------------|-------------|
| `snes_header.asm` | `$8000` | `$0000` | Most tests |
| `snes_header_ret.asm` | `$8000` | `RTIBreak` | Tests that trigger `brk` and need a custom RTI handler (e.g., `krom_ret.asm`) |

If a test uses `brk`, it **must** include `snes_header_ret.asm` and define the `RTIBreak` label, or the CPU will crash.

---

## Integration with Rust tests

Assembled `.sfc` ROMs are loaded by the Rust test suite (see `tests/rom_tests.rs`, `tests/apu_tests.rs`, `tests/ppu_tests.rs`).

Example flow:

1. A developer writes `my_test.asm` and includes `asm_lib` files.
2. `bass my_test.asm` produces `my_test.sfc`.
3. A Rust test loads the ROM:

   ```rust
   let rom_path = root_dir.join("tests/rom_tests/my_test.sfc");
   let mut system = SyncSystem::with_cartridge(&Cartridge::with_sfc_file(&rom_path).unwrap());
   ```

4. The test either:
   - Steps the CPU and compares against a **BSNES trace log** (cycle-accurate verification), or
   - Runs until `stp` / infinite loop and inspects memory state.

Some tests also use `.spc` files embedded inside `.sfc` ROMs to verify APU/SPC700 behavior.

---

## Tips for future AI agents

- **Always check which header is needed.** If the test uses `brk`, use `snes_header_ret.asm`.
- **Use `base.asm` for quick CPU/DMA tests** that do not need graphics.
- **Use `SNES_INIT` before any PPU access.** It force-blanks the screen and clears all video state safely.
- **Remember double-writes.** Many PPU registers (scroll, Mode 7, etc.) require writing the low byte followed by the high byte to the **same address**.
- **DMA channel numbering.** Macros in `snes_gfx.inc` accept a channel number `0..7`. Make sure not to collide with channels used by `SNES_INIT` (channel 0 is used internally for WRAM/VRAM/CGRAM clearing).
- **SPC700 upload protocol.** Use `SPCWaitBoot()`, then `TransferBlockSPC()`, then `SPCExecute()` to run code on the SPC700. The handshake is timing-sensitive; follow the macros exactly.
