# `rom_tests` Integration Tests Summary

This directory contains the primary integration-test ROMs for the SRES emulator.  All tests are driven from the Rust source file **`sres_emulator/tests/rom_tests.rs`** (28 `#[test]` functions, 364 lines).

Tests fall into two broad categories:

1. **Trace-comparison tests** – execute a ROM cycle-for-cycle against a BSNES-generated execution trace.
2. **ROM-outcome tests** – run a small hand-written ROM to completion and inspect final memory / bus state.

---

## Directory Layout

| Pattern | Purpose | Count |
|---------|---------|-------|
| `*.sfc` | Pre-assembled SNES ROM images (some tracked in Git LFS). | 28 |
| `*.asm` | Source assembly for the ROMs (hand-written and krom tests). | 25+ |
| `*-trace.log.xz` | XZ-compressed BSNES execution traces for cycle-accurate comparison. | 25 |
| `process.py` | One-off helper that trims infinite-loop tails from raw `.txt` traces and compresses them to `.xz`. |
| `lib` | **Symlink** → `../asm_lib`. Shared assembly includes (SNES register definitions, header macros, font data, SPC700 macros). |

### Complete File Inventory

**Trace-comparison ROMs** (each has a matching `.asm`, `.sfc`, and `-trace.log.xz`):
- `krom_adc`, `krom_and`, `krom_asl`, `krom_bit`, `krom_bra`, `krom_cmp`, `krom_dec`, `krom_eor`, `krom_inc`, `krom_jmp`, `krom_ldr`, `krom_lsr`, `krom_mov`, `krom_msc`, `krom_ora`, `krom_phl`, `krom_psr`, `krom_ret`, `krom_rol`, `krom_ror`, `krom_sbc`, `krom_str`, `krom_trn` (23 tests)
- `ppu_timing` (PPU cycle-alignment NOP loop)
- `play_noise` (mixed CPU + SPC700 trace)

**ROM-outcome ROMs** (only `.asm` + `.sfc`):
- `dma_cgram`, `dma_oam`, `dma_vram` (3 DMA tests)

**Extra source files for `play_noise`:**
- `play_noise.spc.asm` – SPC700 / DSP program (noise generation)
- `play_noise.spc` – Assembled SPC binary

---

## 1. Trace-Comparison Tests (`run_rom_test`)

### How they work (rom_tests.rs:151-172)

```rust
fn run_rom_test(test_name: &str) {
    let mut system = SyncSystem::with_cartridge(&Cartridge::with_sfc_file(&rom_path).unwrap());
    // Quirk: some krom tests read 0x93 from $000000 on first instruction
    system.cpu.bus.cycle_write_u8(0x000000.into(), 0x93);
    system.cpu.reset();

    let cpu_steps = SystemDebug::cpu_step_iter(&mut system);
    for (line_num, (expected_line, actual_line)) in trace_log_from_xz_file(&trace_path)
        .unwrap()
        .zip(cpu_steps)
        .enumerate()
    {
        assert_cpu_trace_eq(line_num, expected_line.unwrap(), actual_line);
    }
}
```

1. Load a `.sfc` ROM into `SyncSystem`.
2. Manually write `0x93` to `$000000` (see **Known Quirks** below).
3. Reset the CPU.
4. Stream the corresponding `-trace.log.xz` line-by-line.
5. For every trace line, advance the emulator by exactly **one CPU step** and compare the full `CpuState` string representation.
6. Any mismatch fails the test immediately with a `pretty_assertions` diff.

### Trace format

Traces are plain text, one line per CPU step, in **Mesen trace format** (parsed by `CpuState::parse_mesen_trace`). The Python helper `process.py` was used to:

- Rename raw `.txt` traces to `-trace.log`
- Trim trailing infinite loops (stops at a `JMP` whose operand effective address equals its own PC)
- Compress with `xz`

### Covered ROMs

| Test name | What it exercises | Notes |
|-----------|-------------------|-------|
| `krom_adc` | `ADC` in all addressing modes, 8/16-bit, binary/decimal, with/without carry | |
| `krom_and` | `AND` | |
| `krom_asl` | `ASL` | |
| `krom_bit` | `BIT` | |
| `krom_bra` | Branch instructions (`BRA`, `BCC`, `BCS`, `BEQ`, `BMI`, `BNE`, `BPL`, `BVC`, `BVS`) | |
| `krom_cmp` | `CMP` / `CPX` / `CPY` | |
| `krom_dec` | `DEC` / `DEX` / `DEY` | |
| `krom_eor` | `EOR` | |
| `krom_inc` | `INC` / `INX` / `INY` | |
| `krom_jmp` | `JMP` / `JSR` / `RTS` | |
| `krom_ldr` | Load instructions (`LDA`, `LDX`, `LDY`) | |
| `krom_lsr` | `LSR` | |
| `krom_mov` | Move / transfer instructions (`MVN`, `MVP`, `TAX`, etc.) | |
| `krom_msc` | Miscellaneous (`NOP`, `WDM`, `BRK`, `COP`, `WAI`, `STP`) | **Ignored** – instructions not implemented yet |
| `krom_ora` | `ORA` | |
| `krom_phl` | Push / pull (`PHA`, `PHP`, `PLA`, `PLP`, etc.) | |
| `krom_psr` | Flag instructions (`SEC`, `CLC`, `SEI`, `CLI`, etc.) | |
| `krom_ret` | Return instructions (`RTS`, `RTL`, `RTI`) | Uses `lib/snes_header_ret.asm` (needs custom BRK/RTI handler) |
| `krom_rol` | `ROL` | |
| `krom_ror` | `ROR` | |
| `krom_sbc` | `SBC` | |
| `krom_str` | Store instructions (`STA`, `STX`, `STY`) | |
| `krom_trn` | Transfer instructions (`TAX`, `TXA`, `TAY`, `TYA`, etc.) | |
| `ppu_timing` | Simple `NOP` loop | Verifies PPU cycle alignment against BSNES trace |

All `krom_*` tests are derived from **Peter Lemon’s (krom) SNES 65816 CPU test suite**. They are comprehensive opcode tests that:
- Set up specific flag / register states
- Execute the instruction under test
- Print the result and PSR flags to VRAM using `PrintText`, `PrintValue`, `PrintPSR` macros
- Compare against hard-coded expected values embedded in the ROM
- **Hang in an infinite loop on failure** (`FailN: PrintText(Fail, ...); bra FailN`), or continue on success

Because the tests hang on failure, the trace-comparison approach naturally catches any deviation: the emulator will diverge from the BSNES trace at the first wrong result (the PC jumps to the `Fail` infinite loop instead of the expected path).

### Special case: `play_noise` (mixed CPU + SPC700 trace)

`play_noise` tests APU boot and SPC700 execution. It uses **`run_rom_test_with_spc700_trace`** (rom_tests.rs:174-240), which reads a **mixed** trace file containing both CPU and SPC700 steps.

- The trace parser distinguishes CPU vs SPC700 lines by line length (< 100 chars = SPC700, longer = CPU).
- The test matcher buffers out-of-order steps from the two processors and pairs them by type via two `VecDeque` buffers (`pending_cpu` and `pending_spc`).
- At every **APUIO access** (`$2140-$217F`), it asserts that both buffers are empty (CPU and SPC700 are strictly in sync at I/O boundaries).

This is the only test in this directory that validates SPC700 / DSP behavior via trace comparison.

---

## 2. ROM-Outcome Tests (`run_test_rom`)

### How they work (rom_tests.rs:326-339)

```rust
fn run_test_rom(test_name: &str) -> CpuT {
    let mut system = System::with_cartridge(&Cartridge::with_sfc_file(&rom_path).unwrap());
    system.cpu.reset();
    while !system.cpu.halted() {
        system.cpu.step();
    }
    system.cpu
}
```

1. Load a `.sfc` ROM into `System` (not `SyncSystem` – simpler synchronous execution).
2. Run `cpu.step()` in a loop until the CPU halts (`stp` instruction or similar).
3. Inspect memory / bus state with `cpu.bus.peek_range(...)`.

### Covered ROMs

| Test name | What it exercises | Verification |
|-----------|-------------------|--------------|
| `dma_vram` | DMA transfer from WRAM → VRAM and back | Checks that `$0000-$00FF` and `$0100-$01FF` both contain `0x00..0xFF` |
| `dma_cgram` | DMA transfer from WRAM → CGRAM and back | Same memory check |
| `dma_oam` | DMA transfer from WRAM → OAM and back | Same memory check |

These are small, hand-written assembly programs (see `dma_*.asm`) that:
1. Generate a test pattern (`0x00..0xFF`) in WRAM at `$0000`.
2. Use the SNES DMA engine to copy it to PPU memory (VRAM, CGRAM, or OAM).
3. DMA-copy it back to WRAM at `$0100`.
4. Execute `stp` to halt the CPU.

The test then asserts that both source and destination WRAM regions contain the expected sequence.

---

## Assembly Conventions

### krom tests (`krom_*.asm`)

- Use **bass** assembler syntax (`arch snes.cpu`, `seek(...)`, macro definitions with `{}`).
- Include shared libraries:
  - `lib/snes.inc` – full SNES register map constants (`REG_INIDISP`, `REG_BGMODE`, etc.)
  - `lib/snes_header.asm` – ROM header / vector table (LoROM, SlowROM, reset → `$8000`)
  - `lib/snes_gfx.inc` – graphics setup macros (`SNES_INIT`, `LoadPAL`, `LoadLOVRAM`, `WaitNMI`, `ClearVRAM`, etc.)
- Define per-ROM macros such as `PrintText`, `PrintValue`, `PrintPSR` to render results to VRAM.
- Each test covers **many pages / sub-cases** across the screen:
  - Different addressing modes (immediate, absolute, long, direct-page, indirect, etc.)
  - 8-bit vs 16-bit accumulator width (`sep #$20` / `rep #$20`)
  - Binary vs Decimal mode (`sep #$08` / `rep #$08`)
  - Carry flag set vs clear (`sec` / `clc`)
- On a given page, the test clears the screen between pages (`ClearVRAM`) and prints pass/fail for each sub-case.
- Most include `lib/snes_header.asm`. `krom_ret.asm` uniquely includes `lib/snes_header_ret.asm` because it exercises `brk` → `RTI` and needs a custom `RTIBreak` vector handler.

### Hand-written tests (`dma_*.asm`, `ppu_timing.asm`)

- Use **xa65** assembler syntax (`output "name.sfc", create`, `include "lib/base.asm"`, `sei`, `clc`, `xce`, etc.).
- Include `lib/base.asm` for minimal setup (sets `seek($8000)`, fills bank, includes standard header).
- Very small, focused; often just set up registers and run `stp` or an infinite loop.

### `play_noise` (CPU + SPC700)

- `play_noise.sfc.asm` – SNES CPU code that boots the SPC700, uploads `play_noise.spc`, and jumps to an infinite loop.
- `play_noise.spc.asm` – SPC700 / DSP code that configures echo, noise, and volume, then loops playing different noise patterns (kick, hi-hat, snare).
- Uses `lib/snes_spc700.inc` for SPC700 register constants and macros (`SPCWaitBoot`, `TransferBlockSPC`, `SPCExecute`, `WDSP`).

### Shared library (`lib/` → `../asm_lib/`)

| File | Purpose |
|------|---------|
| `base.asm` | Minimal startup for xa65-based tests (`arch snes.cpu`, `seek($8000)`, fill, include header) |
| `snes.inc` | Full SNES register map (~500 lines of `constant REG_XXX = $YYYY`) |
| `snes_gfx.inc` | Graphics initialization macros and font loading routines (`SNES_INIT`, `LoadPAL`, `WaitNMI`, etc.) |
| `snes_header.asm` | Standard LoROM/SlowROM header, reset vector → `$8000` (used by most tests) |
| `snes_header_ret.asm` | Same header, but BRK vector points to external `RTIBreak` label (used by `krom_ret.asm`) |
| `snes_spc700.inc` | SPC700 register definitions and transfer macros (`SPCWaitBoot`, `TransferBlockSPC`, `WDSP`, etc.) |
| `font8x8.asm` | 1BPP 8×8 font tile data (ASCII `$20`–`$7E`) used by CPU tests to print "PASS"/"FAIL" |

---

## Adding a New Test

### Trace-comparison test

1. Obtain or create a `.sfc` ROM.
2. Generate a BSNES execution trace in Mesen format.
3. Run `process.py` (or manually) to trim infinite loops and compress to `.xz`.
4. Place both files in this directory with matching basenames.
5. Add a `#[test]` function in `rom_tests.rs` calling `run_rom_test("your_name")`.

### ROM-outcome test

1. Write an assembly program that halts the CPU (`stp`) and leaves verifiable state in memory.
2. Assemble to `.sfc` (xa65 or bass).
3. Add a `#[test]` function in `rom_tests.rs` calling `run_test_rom("your_name")`, then assert on `cpu.bus.peek_range(...)`.

---

## Key Rust Types & Functions (from `rom_tests.rs`)

| Item | Role |
|------|------|
| `SyncSystem` | Cycle-accurate system used for trace comparison (`run_rom_test`) |
| `System` | Simpler synchronous system used for ROM-outcome tests (`run_test_rom`) |
| `SystemDebug::cpu_step_iter` | Iterator yielding `CpuState` per CPU step |
| `SystemDebug::trace_step_iter` | Iterator yielding `TraceStep::Cpu` or `TraceStep::Spc700` interleaved |
| `CpuState::parse_mesen_trace` | Parses a Mesen-format trace log line into a `CpuState` |
| `Spc700State::parse_mesen_trace` | Parses SPC700 trace lines |
| `trace_log_from_xz_file` | Opens and iterates a compressed CPU-only trace (`Iterator<Item = Result<CpuState>>`) |
| `mixed_trace_log_from_xz_file` | Opens and iterates a compressed mixed CPU+SPC700 trace (line length < 100 = SPC700) |
| `assert_cpu_trace_eq` | Compares two `CpuState`s by string; **clears `effective_addr`** on both before comparison (open-bus quirk) |
| `assert_spc_trace_eq` | Compares two `Spc700State`s; **clears `operand_str` and `master_cycle`** before comparison |
| `is_cpu_apuio_access` | Checks if a CPU trace step accesses `$2140-$217F` (APU I/O ports) |

---

## Known Quirks

- **Open bus reads**: The emulator does not implement open-bus behavior. Therefore `effective_addr` is cleared during trace comparison, because memory values shown for write-only MMIO registers would otherwise mismatch.
- **CPUMSC initial read**: All trace-comparison tests manually write `0x93` to `$000000` before reset because `krom_msc` (and some other krom tests) read `0x93` from that address on the first instruction. The reason is not fully understood – `$000000` should map to WRAM and should read `0x00` on boot.
- **Git LFS**: `.sfc`, `.xz`, and `.png` files are stored in Git LFS. If LFS objects are missing, ROM-based tests may be skipped or fail to load.
- **Assembly toolchains**: krom tests use **bass** syntax; hand-written `dma_*` and `ppu_timing` tests use **xa65** syntax. Both are assembled outside the Rust build process (the `.sfc` binaries are checked in).
