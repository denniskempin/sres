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
- CI: `.github/workflows/postsubmit.yml` (`health`/`test`/`coverage` on PRs and `main`; Pages `deploy` on `main` only)

## Architecture Overview

### Layer Structure (top to bottom)

**`sres_egui`** — Native egui or WASM frontend. Playback uses the public `System` API (see Entry Point Call Chain). Cartridge load and debugger panels also import `components/`, `apu/`, `main_bus/`, `debugger`, and `common/`.

**`sres_emulator` (`lib.rs`)** — System orchestration. `SystemImpl<PpuT, ApuT>` has three variants: `BatchedSystem` (default, batched PPU/APU), `SyncSystem` (cycle-accurate), `AsyncSystem` (threaded PPU and APU). Owns the CPU, `MainBusImpl`, `Apu`, debugger, and framebuffer.

**`main_bus/`** — 65816 memory map, LoRom/HiRom address decoding, 8-channel DMA and HDMA, hardware multiply/divide, and NMI/IRQ delegation to `Clock`. Connects CPU to all devices.

**`apu/`** — APU integration layer. Orchestrates `Spc700` + `S-DSP`. `ApuBus` provides APU RAM, IPL ROM, timers, and APUIO ports. Outputs 32 kHz `AudioBuffer`.

**`components/`** — Independent hardware components. They import only from `common/`; never from each other. All integration happens above in `main_bus/`, `apu/`, and `lib.rs`.

| Component | Hardware | Notes |
|---|---|---|
| `cpu/` | W65C816 | 256-op table, generic `BusT`, 8/16-bit `UInt` dispatch |
| `ppu/` | Ricoh 5C77 | Scanline renderer, VRAM/CGRAM/OAM |
| `spc700/` | Sony SPC700 | Audio CPU |
| `s_dsp/` | Sony S-DSP | 8-voice BRR sample playback, 32 kHz output |
| `cartridge` | ROM/SRAM | LoRom/HiRom header parsing |
| `clock` | Timer/IRQ | NMI, H/V timer IRQs, HDMA trigger latches |

**`common/`** — Foundational types used by all layers: `AddressU24/U16/U15`, `Bus` trait, `UInt` (u8/u16 generic), `ClockInfo`, `DebugEventCollector`, `Rgb15/Rgba32/Image`.

### Key Design Patterns

- **Lazy APU catch-up**: SPC700 is not stepped every master cycle. `Apu::update_clock` catch-up-steps to the current master clock (`SyncSystem`: every CPU bus cycle; `BatchedSystem`: on device read, vblank/`sync()`, cache overflow).
- **Generic CPU bus**: `Cpu<BusT: MainBus>` and `Spc700<BusT: Spc700Bus>` — bus injected at compile time.
- **8/16-bit dispatch**: CPU instructions generic over `T: UInt`; dispatch by M/X status flags at runtime.
- **PPU scanline renderer**: Draws one scanline at a time; new frame available only on vblank rise.
- **Zero-cost debug**: `DebugEventCollector` guarded by `DEBUG_EVENTS_ENABLED` atomic; `#[cold]` dispatch.
- **Save states**: `PpuState` encoded with `bitcode`. APU and CPU are not snapshotted.

## System Variants

`System` (alias for `BatchedSystem`) is used everywhere except where noted:

| Type | PPU/APU update | When to use |
|---|---|---|
| `BatchedSystem` | Buffered; flushed on device read, vblank/`sync()`, cache overflow | Default; UI, general tests |
| `SyncSystem` | Cycle-accurate, every CPU step | Trace-comparison tests (`rom_tests`), cycle-timing bugs |
| `AsyncSystem` | PPU and APU on background threads | Performance exploration / benchmarks only |

`SyncSystem` is required when comparing against BSNES traces because batched updates introduce observable timing differences at register boundaries.

## Entry Point Call Chain

```
EmulatorApp::ui()                        // eframe::App
  → system.update_joypads(joy1, 0)       // joy2 is always 0
  → system.execute_for_audio_samples(n)  // normal play; debugger uses execute_for_duration
  → AudioOutput::update()                // UI thread: system.swap_audio_buffer()
  → system.swap_video_frame()            // true on vblank rise
cpal callback (sres_egui/src/audio.rs)
  → drain AudioBufferQueue               // does not touch System
```

`execute_*` → `execute_until` → `step()` → `cpu.step()` → `MainBusImpl::bus_read/write` → PPU/APU/DMA/Clock.

## Testing Strategy

| Test type | Location | System variant | Use for |
|---|---|---|---|
| Trace-comparison | `tests/rom_tests/` | `SyncSystem` | CPU instruction correctness vs BSNES |
| ROM-outcome | `tests/rom_tests/` | `System` | DMA, memory behavior; inspect memory at `stp` |
| Golden-image | `tests/ppu_tests/` | `System` (ROM) / `Ppu` (snapshots) | PPU rendering correctness; diff against `.png` |
| Golden-WAV | `tests/apu_tests/` | `System` | SPC700/S-DSP vs `.wav` (`play_noise` is RAM/DSP, not WAV) |

Golden files are auto-created on first run; verify them before committing. Mismatches write `.actual.png` / `.actual.wav`.

## Error Handling & Unimplemented Hardware

- **Unimplemented registers**: reads return `0`, writes are silently ignored. Both emit a `DebugEvent` error (visible in debugger; no panic). Exception: PPU unhandled I/O uses `log::warn`, not a `DebugEvent` (see `sres_emulator/src/components/ppu`).
- **Unmapped memory**: same — return `0` + emit error.
- **Open bus**: not emulated; unmapped reads return `0` (known divergence from hardware, noted in test comments).
- **FastROM**: not implemented; banks `$80+` still use SLOW access (`TODO` in `main_bus/mod.rs`).
- **Panics** are reserved for internal logic errors (wrong operand type, CPU halt in wrong context) — never for unimplemented hardware. Exception: PPU `decode_bgmode` panics on BG modes 4/6/7 (see `sres_emulator/src/components/ppu`).
- **Fuzz targets** in `sres_emulator/fuzz/` are intended to test that arbitrary input never panics. The bins are stale and do not compile.

## Reference

- `docs/index.md` — indexed hardware reference docs (fullsnes.txt extracts and nesdev.org articles). Covers PPU, APU, DMA, memory maps, CPU opcodes, timing, and controllers. Use keyword search within the index to find the relevant file.
- `.cursor/skills/code-review/SKILL.md` — independent code-owner review via a readonly subagent. Checks: `.cursor/skills/code-review/references/review-guide.md`.
- `.cursor/skills/write-agents-docs/SKILL.md` — follow it when editing any `AGENTS.md` or `//!` file header.

## Subdirectory AGENTS.md Files

Each module and test directory under `sres_emulator/` and `sres_egui/src` has its own `AGENTS.md` (`find . -name AGENTS.md`). Read the nearest one before editing. Directory files hold local facts and never restate this file.

## Environment Gotchas

- **Nightly Rust**: Required. `rust-toolchain.toml` specifies channel; `rust-src` component needed.
- **libxkbcommon-x11-0**: Runtime dependency for native egui. Install via `apt` if missing.
- **Binary test assets**: `.sfc`, `.xz`, `.png`, `.wav` are committed directly to git (LFS was removed in `309c47b`). Missing files fail the test; Cargo does not reassemble.
- **bass**: Assembler for committed test ROM sources (`arch snes.cpu` / `arch snes.smp`). Test drivers load `.sfc` only.
- **cargo-nextest**: Preferred runner. `curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-$HOME/.cargo}/bin`

## Important Agent Rules
- **Concise**: Speak concisely, drop conversational fillers, pleasantries, rambling explanations. Use simple and direct language.
- **Push back**: Do not blindly agree with inefficient, illogical or requests that lead to bad outcomes. Push back by stating the technical blocker in direct language.
