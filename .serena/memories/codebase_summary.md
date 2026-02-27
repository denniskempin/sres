# SRES Codebase Summary

## Project Identity
SRES is a **WIP SNES (Super Nintendo Entertainment System) emulator written in Rust**. It is a learning-focused side project by denniskempin that aims to emulate the complete SNES hardware.

**Repository**: https://github.com/denniskempin/sres  
**Language**: Rust (edition 2021, min version 1.72)  
**CI**: GitHub Actions (`.github/workflows/postsubmit.yml`) + codecov for coverage

---

## Workspace Structure

```
.
├── sres_emulator/      # Core emulation library crate
├── sres_egui/          # GUI frontend crate (native + WASM)
├── check-all.sh        # Run all checks (tests, WASM, clippy, format)
├── fix-all.sh          # Auto-fix clippy and formatting issues
├── rustfmt.toml        # Custom rustfmt config
├── rust-toolchain.toml # Rust toolchain pinning
├── Cargo.toml          # Workspace manifest
├── CLAUDE.md           # AI coding assistant guidance
└── README.md           # Project overview
```

---

## sres_emulator — Core Library

### Top-level API (`src/lib.rs`)

- **`System`**: Main entry point for emulation. Owns `Cpu<MainBusImpl>`, a `Debugger`, and framebuffer state.
  - `System::new()` / `System::with_cartridge(cartridge)`
  - `execute_one_instruction()`, `execute_until_halt()`, `execute_frames(n)`, `execute_scanlines(n)`, `execute_cycles(n)`, `execute_for_audio_samples(n)`, `execute_for_duration(seconds)`
  - `swap_video_frame()`, `swap_audio_buffer()` for extracting output
  - `debugger()` → `RefMut<Debugger>` for breakpoints/logging
  - `debug()` → `SystemDebug` for read-only inspection of PPU/APU state
- **`ExecutionResult`**: `Normal`, `Halt`, `Break(BreakReason)`
- **`CpuT`**: Type alias for `Cpu<MainBusImpl<BatchedBusDeviceU24<Ppu>, BatchedBusDeviceU24<Apu>>>`

### Component Modules

#### CPU — `src/components/cpu/`
65816 (WDC 65C816) processor implementation.
- **`Cpu<BusT>`**: Full CPU state + execution engine
- **`MainBus`** trait: Interface the CPU uses to access the rest of the system
- **`VariableLengthRegister`**: Handles 8/16-bit accumulator and index registers
- **`NativeVectorTable`** / **`EmuVectorTable`**: Interrupt vector addresses
- Submodules: `instructions`, `opcode_table`, `operands`, `status`, `debug`, `test`
- Per-opcode test data in `test/[0-f]x.json.xz` (compressed JSON fixtures)

#### PPU — `src/components/ppu/`
Picture Processing Unit — graphics rendering engine.
- **`Ppu`**: Core PPU with register handling and frame rendering
- **`Framebuffer`**: 256×224 pixel output buffer, indexable as `framebuffer[(x, y)]`
- **`Background`**: BG layer configuration (mode, tilemap, tile data)
- **`BgMode`**: Modes 0–7 (SNES background rendering modes)
- **`BitDepth`** / `Bpp2Decoder`, `Bpp4Decoder`, `Bpp8Decoder`: Tile decoding
- Sub-memories: **`vram`** (64KB), **`oam`** (object attribute memory), **`cgram`** (color palette)
- Supports BG1–BG4, sprite layers, colour math, HDMA effects, interlace

#### APU — `src/apu/`
Audio Processing Unit integration layer.
- **`Apu`**: Contains SPC700 processor + S-DSP, runs on APU bus
- **`AudioBuffer`**: Ring-buffer-like structure for PCM audio samples
- **`ApuDebug`**: Read-only debug view of APU state
- Constants: `APU_SAMPLE_RATE` (32000 Hz), `MASTER_CLOCK_FREQUENCY`, `CYCLES_PER_SAMPLE`
- Submodules: `apu_bus` (APU memory map), `timers` (3 hardware timers), `test`

#### SPC700 — `src/components/spc700/`
Sony SPC700 audio CPU (runs audio program code uploaded from the main CPU).
- **`Spc700<BusT>`**: SPC700 processor state and execution
- **`Spc700Bus`** trait: Interface to APU memory
- Submodules: `instructions`, `opcode_table`, `operands`, `status`, `debug`, `test`
- Per-opcode test fixtures in `test/[0-f]x.json.xz`

#### S-DSP — `src/components/s_dsp/`
Sony S-DSP sound chip — 8-voice DSP with BRR sample decompression.
- **`SDsp`**: Core DSP state, processes 8 simultaneous voices
- **`NoiseGenerator`**: Hardware noise channel
- **`Flg`**: DSP flags register (soft reset, mute, echo, noise rate)
- Submodules: `brr` (BRR sample decompression), `voice` (per-voice state + ADSR), `pitch`, `test`
- `NOISE_RATE_DIVIDERS`: Lookup table for noise frequency

#### Cartridge — `src/components/cartridge.rs`
ROM loading and cartridge emulation.
- **`Cartridge`**: Holds ROM data and parsed header
- **`SnesHeader`** / **`RawSnesHeader`**: SNES ROM header parsing
- **`MappingMode`**: `LoROM` or `HiROM` — determines memory banking

#### Clock — `src/components/clock.rs` + `src/common/clock.rs`
Timing and synchronization. `ClockInfo` tracks master clock cycles, scanlines (`v`), and frames (`f`).

### Common / Utilities — `src/common/`

- **`bus.rs`**: Core traits `Bus<Addr>` and `BusDeviceU24`. Wrappers: `BatchedBusDeviceU24` (sync calls in batches for performance), `AsyncBusDeviceU24` (lazy sync). `BusAction` enum.
- **`address.rs`**: `AddressU24` (SNES 24-bit address), `AddressU16` types with banking logic
- **`uint.rs`**: Utility integer types (U24, etc.)
- **`debug_events.rs`**: `DebugEventCollector` trait + `DebugEventCollectorRef` for event collection
- **`image.rs`**: Image utilities for PNG comparison in tests
- **`test_bus.rs`**: `TestBus<AddressU24>` for unit testing components in isolation
- **`test_util.rs`**: Test helper utilities
- **`logging.rs`**: Logging setup helpers
- **`util.rs`**: `EdgeDetector` (detects rising/falling edges on boolean signals)

### Main Bus — `src/main_bus/`
SNES memory mapping and bus arbitration.
- **`MainBusImpl<PpuT, ApuT>`**: Concrete bus implementation connecting CPU to all devices
- **`MainBusEvent`**: Debug event type for bus transactions
- **`MemoryBlock`**: Memory region classification
- Functions: `lorom_memory_map()`, `hirom_memory_map()`, `memory_access_speed()`
- Submodules: `dma` (DMA / HDMA transfers), `multiplication` (hardware multiply/divide)

### Debugger — `src/debugger.rs`
Comprehensive debugging and event tracing system.
- **`Debugger`**: Core debugger state; shared via `DebuggerRef` (`Rc<RefCell<Debugger>>`)
- **`EventFilter`**: Parseable filter for which events to break/log on
- **`BreakReason`**: What caused a break (with trigger info)
- **`Trigger`** / **`DebugEvent`**: Event types (memory access, instruction, etc.)
- **`MemoryAccess`**: Read/write event on a memory address
- Implements `DebugEventCollector` for `CpuEvent`, `MainBusEvent`, `ApuBusEvent`, `Spc700Event`

### Controller — `src/controller.rs`
SNES controller input handling. Joypad state accessible via `MainBusImpl::update_joypads(joy1, joy2)`.

---

## sres_egui — GUI Frontend

Built with **eframe/egui**. Supports both **native desktop** (via Cargo) and **WebAssembly** (via Trunk).

### Main Files
- **`app.rs`**: `EmulatorApp` — main eframe application struct implementing `eframe::App`
- **`home.rs`**: Home screen / ROM picker UI
- **`audio.rs`**: Audio output (CPAL for native, Web Audio API for WASM)
- **`embedded_roms.rs`**: ROMs embedded at compile time for the WASM build
- **`util.rs`**: GUI utility helpers
- **`test_utils.rs`**: Test helpers for egui tests
- **`main.rs`**: Entry point for native builds

### Debug Views — `src/debug/`
- **`cpu.rs`**: CPU state inspector, disassembly view
- **`ppu.rs`**: PPU state, VRAM viewer, palette viewer
- **`apu.rs`**: APU state, voice envelope/waveform visualization
- **`memory.rs`**: Hex memory viewer
- **`event.rs`**: Debug event log viewer
- **`log_viewer.rs`**: Application log viewer
- **`syntax.rs`**: Syntax highlighting for disassembly

---

## Testing

### Test Organization (`sres_emulator/tests/`)
- **`rom_tests.rs`** + `rom_tests/`: ROM-based CPU integration tests
  - SNES `.sfc` binaries + reference trace logs (Mesen format, `.log.xz`)
  - Covers krom test suite: `asl`, `bit`, `bra`, `cmp`, `dec`, `eor`, `inc`, `jmp`, `ldr`, `lsr`, `msc`, `mov`, `ora`, `phl`, `psr`, `ret`, `rol`, `ror`, `sbc`, `str`, `trn`, plus DMA tests, PPU timing
  - `process.py`: Helper to process/generate trace files
- **`ppu_tests.rs`** + `ppu_tests/`: Graphics rendering validation
  - Renders frames and compares against reference PNG snapshots
  - Test ROMs: krom hello world, rings, bgmap modes, interlace, HDMA, colour math, SMW, TLoZ
- **`apu_tests.rs`** + `apu_tests/`: Audio output validation
  - Runs audio for N samples and compares against reference WAV files
  - Test cases: play_noise, play_brr_sample, ffvii_prelude

### Per-Component Unit Tests
- **CPU** (`src/components/cpu/test.rs`): JSON fixture tests for every opcode group
- **SPC700** (`src/components/spc700/test.rs`): JSON fixture tests for every SPC700 opcode
- **S-DSP** (`src/components/s_dsp/test.rs`): DSP unit tests
- **APU** (`src/apu/test.rs`): APU integration tests
- **Main Bus** (`src/main_bus/` tests): Bus and DMA tests
- **Debugger** (`src/debugger.rs` tests): Event filter and breakpoint tests

### Assembly Test Library (`tests/asm_lib/`)
Helper assembly files for building custom test ROMs:
- `base.asm`, `snes_header.asm`, `snes_header_ret.asm`: ROM boilerplate
- `snes.inc`, `snes_gfx.inc`, `snes_spc700.inc`: SNES hardware register definitions
- `font8x8.asm`: 8×8 pixel font for text rendering in test ROMs

---

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `anyhow` | Error handling (`Result<T>` everywhere) |
| `egui` / `eframe` | GUI framework |
| `intbits`, `packed_struct`, `bilge` | Bit manipulation and packed register types |
| `log`, `env_logger`, `colored` | Logging |
| `puffin` | Performance profiling |
| `bitcode` | Serialization for save states |
| `hound` | WAV audio file I/O |
| `image` | PNG image comparison in tests |
| `criterion` | Benchmarking |
| `xz2` | Decompress `.xz` trace/test files |
| `pretty_assertions` | Enhanced test assertion output |
| `itertools`, `strum` | Utility libraries |

---

## Common Patterns

### Bus Trait Pattern
All memory access goes through `Bus<Addr>` trait. Components implement `BusDeviceU24` and are wrapped in `BatchedBusDeviceU24` for performance (defers sync until needed).

### Generic CPU
`Cpu<BusT>` is generic over the bus type, enabling unit testing with `TestBus` without needing the full system.

### Debug Event System
Components emit events via `DebugEventCollector` trait. The `Debugger` collects these events, evaluates breakpoints, and can log/halt execution.

### Save States
`bitcode` crate is used for compact binary serialization of emulator state (all major structs implement `bitcode::Encode`/`Decode`).

### Feature Flags
- `debug_log`: Enables verbose per-instruction logging (significant performance cost)

---

## Build & Test Commands

```bash
# Run all tests
cargo nextest run

# Run specific test suites
cargo nextest run rom_tests
cargo nextest run ppu_tests
cargo nextest run apu_tests

# Build
cargo build
cargo build --release

# WASM build
cd sres_egui && trunk build
cd sres_egui && trunk serve

# Code quality
cargo clippy --workspace
cargo fmt
./check-all.sh   # all checks: tests + WASM + clippy + fmt
./fix-all.sh     # auto-fix clippy + fmt
```
