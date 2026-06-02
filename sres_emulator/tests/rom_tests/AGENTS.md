# rom_tests

CPU trace-comparison and ROM-outcome tests. Driven by `sres_emulator/tests/rom_tests.rs`.

## Files

| Pattern | Purpose |
|---------|---------|
| `*.sfc` | Pre-assembled SNES ROMs (Git LFS) |
| `*.asm` | Source assembly (bass for krom tests, xa65 for hand-written) |
| `*-trace.log.xz` | XZ-compressed BSNES execution traces |
| `process.py` | Trims infinite loops from raw traces and compresses to `.xz` |
| `lib` | Symlink → `../asm_lib`. Shared assembly includes |

## Test Types

**Trace-comparison** (`run_rom_test`): Load `.sfc`, stream `-trace.log.xz` line-by-line, advance one CPU step per line, compare `CpuState` strings. Mismatch fails immediately.

- 23 `krom_*` tests: Peter Lemon's 65816 CPU opcode tests (ADC, AND, ASL, etc.)
- `ppu_timing`: NOP loop for PPU cycle alignment
- `play_noise`: Mixed CPU+SPC700 trace. Uses `run_rom_test_with_spc700_trace`. Buffers out-of-order steps, asserts sync at APUIO accesses (`$2140-$217F`)

**ROM-outcome** (`run_test_rom`): Load `.sfc` into `System`, run until CPU halt (`stp`), inspect memory with `cpu.bus.peek_range(...)`.

- `dma_vram`, `dma_cgram`, `dma_oam`: Verify DMA transfer to/from PPU memory

## Adding Tests

**Trace-comparison:**
1. Create `.sfc` ROM
2. Generate BSNES trace in Mesen format
3. Run `process.py` to trim infinite loops and compress to `.xz`
4. Place files with matching basenames
5. Add `#[test]` calling `run_rom_test("name")` in `rom_tests.rs`

**ROM-outcome:**
1. Write assembly that halts with `stp` and leaves verifiable state
2. Assemble to `.sfc`
3. Add `#[test]` calling `run_test_rom("name")`, assert on `cpu.bus.peek_range(...)`

## Quirks

- **Open bus**: Not implemented. `effective_addr` is cleared during trace comparison.
- **CPUMSC initial read**: All trace tests manually write `0x93` to `$000000` before reset. Reason unknown.
- **Git LFS**: `.sfc` and `.xz` files stored in LFS. Missing objects cause test failures.
- **Toolchains**: krom tests use **bass**; hand-written tests use **xa65**. Assembled outside Rust build.
