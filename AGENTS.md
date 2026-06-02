# AGENTS.md

SRES is a SNES emulator in Rust.

## Services

| Component | Run Command |
|---|---|
| `sres_emulator` | Library crate |
| `sres_egui` (native) | `cargo run` or `cargo run -- rom.sfc` |
| `sres_egui` (WASM) | `cd sres_egui && trunk serve` |

## Commands

- Build: `cargo build`
- Run (headless): `DISPLAY=:1 cargo run`
- Lint: `cargo clippy --workspace`
- Format: `cargo fmt --check`
- Test: `cargo nextest run --workspace` (or `cargo test`)
- Full check: `./check-all.sh`
- Fix: `./fix-all.sh`

## Architecture Overview

### Layer Structure (top to bottom)

**`sres_egui`** — Native egui or WASM frontend. Calls `execute_frames()`, `swap_video_frame()`, `swap_audio_buffer()`, `update_joypads()` on the emulator.

**`sres_emulator` (`lib.rs`)** — System orchestration. `SystemImpl<PpuT, ApuT>` has three variants: `BatchedSystem` (default, batched PPU/APU), `SyncSystem` (cycle-accurate), `AsyncSystem` (threaded APU). Owns the CPU, `MainBusImpl`, `Apu`, debugger, and framebuffer.

**`main_bus/`** — 65816 memory map, LoRom/HiRom address decoding, 8-channel DMA, hardware multiply/divide, and NMI/IRQ delegation to `Clock`. Connects CPU to all devices.

**`apu/`** — APU integration layer. Orchestrates `Spc700` + `S-DSP` with lazy catch-up clocking. `ApuBus` provides APU RAM, IPL ROM, timers, and APUIO ports. Outputs 32 kHz `AudioBuffer`.

**`components/`** — Independent hardware components with no cross-component dependencies. All integration happens above in `main_bus/` and `lib.rs`.

| Component | Hardware | Notes |
|---|---|---|
| `cpu/` | W65C816 | 256-op table, generic `BusT`, 8/16-bit `UInt` dispatch |
| `ppu/` | Ricoh 5C77 | Scanline renderer, VRAM/CGRAM/OAM |
| `spc700/` | Sony SPC700 | Audio CPU, lazy catch-up |
| `s_dsp/` | Sony S-DSP | 8-voice BRR sample playback, 32 kHz output |
| `cartridge` | ROM/SRAM | LoRom/HiRom header parsing |
| `clock` | Timer/IRQ | NMI, H/V timer IRQs, scanline timing |

**`common/`** — Foundational types used by all layers: `AddressU24/U16/U15`, `Bus` trait, `UInt` (u8/u16 generic), `ClockInfo`, `DebugEventCollector`, `Rgb15/Rgba32/Image`.

### Key Design Patterns

- **Lazy APU catch-up**: SPC700 only advances at APUIO access or audio sample boundaries — not every CPU cycle.
- **Component isolation**: `components/` have zero cross-component deps; integration lives in `main_bus/` and `lib.rs`.
- **Generic CPU bus**: `Cpu<BusT: MainBus>` and `Spc700<BusT: Spc700Bus>` — bus injected at compile time.
- **8/16-bit dispatch**: CPU instructions generic over `T: UInt`; dispatch by M/X status flags at runtime.
- **PPU scanline renderer**: Draws one scanline at a time; new frame available only on vblank rise.
- **Zero-cost debug**: `DebugEventCollector` guarded by `DEBUG_EVENTS_ENABLED` atomic; `#[cold]` dispatch.
- **Save states**: All serializable state in `PpuState`/`ApuBus` etc., encoded with `bitcode`.

## System Variants

`System` (alias for `BatchedSystem`) is used everywhere except where noted:

| Type | PPU/APU update | When to use |
|---|---|---|
| `BatchedSystem` | Buffered, flushed at vblank/sync points | Default; UI, general tests |
| `SyncSystem` | Cycle-accurate, every CPU step | Trace-comparison tests (`rom_tests`), cycle-timing bugs |
| `AsyncSystem` | APU on background thread | Performance exploration / benchmarks only |

`SyncSystem` is required when comparing against BSNES traces because batched updates introduce observable timing differences at register boundaries.

## Entry Point Call Chain

```
sres_egui::App::update()
  → system.execute_frames(1)       // advance emulation
  → system.swap_video_frame()      // true on vblank rise
  → system.swap_audio_buffer()     // exchange AudioBuffer
  → system.update_joypads(joy1, joy2)
```

`execute_frames` → `execute_until` → `step()` → `cpu.step()` → `MainBusImpl::bus_read/write` → PPU/APU/DMA/Clock.

## Testing Strategy

| Test type | Location | System variant | Use for |
|---|---|---|---|
| Trace-comparison | `tests/rom_tests/` | `SyncSystem` | CPU instruction correctness vs BSNES |
| ROM-outcome | `tests/rom_tests/` | `System` | DMA, memory behavior; inspect memory at `stp` |
| Golden-image | `tests/ppu_tests/` | `System` | PPU rendering correctness; diff against `.png` |
| Golden-WAV | `tests/apu_tests/` | `System` | SPC700/S-DSP audio correctness; diff against `.wav` |

Golden files are auto-created on first run and committed to Git LFS. Mismatches write `.actual.png` / `.actual.wav`.

## Error Handling & Unimplemented Hardware

- **Unimplemented registers**: reads return `0`, writes are silently ignored. Both emit a `DebugEvent` error (visible in debugger; no panic).
- **Unmapped memory**: same — return `0` + emit error.
- **Open bus**: not emulated; unmapped reads return `0` (known divergence from hardware, noted in test comments).
- **HDMA**: not implemented; `$420C` write logs a warning.
- **Panics** are reserved for internal logic errors (wrong operand type, CPU halt in wrong context) — never for unimplemented hardware.
- **Fuzz targets** explicitly test that arbitrary input never panics.

## Reference

`docs/index.md` — indexed hardware reference docs (fullsnes.txt extracts and nesdev.org articles). Covers PPU, APU, DMA, memory maps, CPU opcodes, timing, and controllers. Use keyword search within the index to find the relevant file.

## Subdirectory AGENTS.md Files

| Path | Coverage |
|---|---|
| `sres_emulator/src/AGENTS.md` | System orchestration, controller, debugger |
| `sres_emulator/src/common/AGENTS.md` | Shared types, traits, utilities |
| `sres_emulator/src/components/AGENTS.md` | Component rules, cartridge, clock |
| `sres_emulator/src/components/cpu/AGENTS.md` | W65C816 CPU |
| `sres_emulator/src/components/ppu/AGENTS.md` | Picture Processing Unit |
| `sres_emulator/src/components/s_dsp/AGENTS.md` | Sony S-DSP (audio) |
| `sres_emulator/src/components/spc700/AGENTS.md` | Sony SPC700 audio CPU |
| `sres_emulator/src/apu/AGENTS.md` | APU integration |
| `sres_emulator/src/main_bus/AGENTS.md` | System bus, DMA, memory mapping |
| `sres_emulator/tests/AGENTS.md` | Integration tests |
| `sres_emulator/tests/rom_tests/AGENTS.md` | CPU trace & ROM tests |
| `sres_emulator/tests/ppu_tests/AGENTS.md` | Golden-image rendering tests |
| `sres_emulator/tests/apu_tests/AGENTS.md` | Golden-WAV audio tests |
| `sres_emulator/tests/asm_lib/AGENTS.md` | Test ROM assembly library |
| `sres_emulator/benches/AGENTS.md` | Criterion benchmarks |
| `sres_emulator/fuzz/AGENTS.md` | Fuzzing setup |

## Environment Gotchas

- **Nightly Rust**: Required. `rust-toolchain.toml` specifies channel; `rust-src` component needed.
- **Headless X11**: `DISPLAY=:1` required in headless environments.
- **libxkbcommon-x11-0**: Runtime dependency for native egui. Install via `apt` if missing.
- **Git LFS**: Test ROMs (`.sfc`), traces (`.xz`), images (`.png`) stored in LFS. If LFS 404s, tests fall back to assembled ROMs (`xa65`).
- **xa65 assembler**: `sudo apt-get install -y xa65`
- **cargo-nextest**: Preferred runner. `curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-$HOME/.cargo}/bin`

## Important Agent Rules
- **Concise**: Speak concisely, drop conversational fillers, pleasantries, rambling explanations. Use simple and direct language. 
- **Push back**: Do not blindly agree with inefficient, illogical or requests that lead to bad outcomes. Push back by stating the technical blocker in direct language.
