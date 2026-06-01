# `sres_emulator/src/components` — Summary

This directory contains **independent, reusable emulator components**. Each component models a specific piece of SNES hardware and is designed to be usable in isolation.

---

## Independence Rules (from `mod.rs`)

The `components/` directory enforces strict architectural boundaries:

1. **Components cannot depend on one another.**
2. **Components can only import from `common/`.**
3. **Exported types and functionality are kept to a minimum.**
4. **All inner modules are private.**
5. **Use `self`/`super` to refer to inner modules; do not use `super` to reach outer modules.**

This means, for example, the `cpu/` component knows nothing about the `ppu/` or `s_dsp/` components. Integration happens at the `main_bus/` and `lib.rs` levels.

---

## Top-Level Files

| File | Purpose |
|------|---------|
| `mod.rs` | Module declarations and the independence rules above. |
| `cartridge.rs` | SFC ROM file parsing, SNES header extraction, LoRom/HiRom detection. |
| `clock.rs` | PPU event/timing tracker: master clock, scanline (V), horizontal position (H), frame (F), NMI, H/V timer IRQs. |

---

## Subdirectories

| Directory | Component | Hardware |
|-----------|-----------|----------|
| `cpu/` | W65C816 CPU emulator | Main SNES CPU (65816) |
| `ppu/` | Picture Processing Unit | Ricoh 5C77 (graphics rendering) |
| `s_dsp/` | Sony Digital Signal Processor | S-DSP (audio synthesis) |
| `spc700/` | Sony SPC700 | Audio co-processor |

Each subdirectory has its own `SUMMARY.md` with full details.

---

## `cartridge.rs` — ROM Loading

- **`Cartridge`** — Holds `SnesHeader`, ROM bytes, and SRAM bytes.
- **`SnesHeader`** — Parsed from offset `0x7FC0` (LoRom) or `0xFFC0` (HiRom). Contains name, mapping mode, ROM size, SRAM size, fast-ROM flag.
- **`MappingMode`** — `LoRom` or `HiRom`.
- **`RawSnesHeader`** — Packed struct for binary header parsing.

Factory methods:
- `Cartridge::with_sfc_file(path)` — Loads `.sfc` + optional `.srm` save file.
- `Cartridge::with_sfc_data(data, srm_data)` — From raw bytes.
- `Cartridge::with_program(program)` — Minimal cartridge for test ROMs (no header).

---

## `clock.rs` — PPU Timing & Interrupts

The `Clock` component tracks the SNES master clock and generates NMI and H/V timer interrupts. It is driven by `main_bus` (not the PPU itself).

- **`Clock`** — Tracks `master_clock`, `v` (scanline), `h_counter` (horizontal master cycles), `f` (frame).
- **`HVTimerMode`** — `Off`, `TriggerH`, `TriggerV`, `TriggerHV` for IRQ configuration.

### Key Behaviors

- **Short scanline**: Line 240 on odd frames is 1360 cycles (vs 1364).
- **DRAM refresh**: ~40-cycle pause at ~536 cycles into each scanline.
- **NMI**: Triggered on vblank rise (V >= 225) if enabled via `$4200`.
- **H/V timers**: Triggered when dot/scanline matches target registers (`$4207-$420A`).
- **NMI read quirk**: Reading `$4210` in the first 2 cycles of V=225 does not clear the NMI flag.

### Registers

| Address | Register |
|---------|----------|
| `$4200` | NMITIMEN (NMI/IRQ enable) |
| `$4207/$4208` | HTIMEL/HTIMEH |
| `$4209/$420A` | VTIMEL/VTIMEH |
| `$4210` | RDNMI (read clears flag) |
| `$4211` | TIMEUP (timer flag, read clears) |
| `$4212` | HVBJOY (vblank/hblank status) |

---

## How Components Fit Together

```
┌─────────────────────────────────────────┐
│              SystemImpl                 │
│  (src/lib.rs — orchestration layer)     │
└─────────────────────────────────────────┘
                   │
    ┌──────────────┼──────────────┐
    ▼              ▼              ▼
┌───────┐    ┌─────────┐    ┌─────────┐
│  CPU  │◄──►│Main Bus │◄──►│  PPU    │
│ (cpu) │    │(main_bus│    │ (ppu)   │
└───────┘    │/devices)│    └─────────┘
             └────┬────┘
                  │
             ┌────┴────┐
             ▼         ▼
        ┌────────┐  ┌────────┐
        │  APU   │  │Cartridge│
        │ (apu)  │  │(cartridge)
        └───┬────┘  └────────┘
            │
       ┌────┴────┐
       ▼         ▼
   ┌────────┐ ┌────────┐
   │ SPC700 │ │ S-DSP  │
   │(spc700)│ │(s_dsp) │
   └────────┘ └────────┘
```

- **`cpu/`, `spc700/`** — Execute instructions; interact with memory through generic `Bus` traits.
- **`ppu/`, `s_dsp/`** — Produce video/audio output; accessed via memory-mapped registers.
- **`cartridge/`** — Provides ROM/SRAM data to the main bus.
- **`clock/`** — Generates timing signals and interrupts for the CPU.
- **`apu/`** — Glues SPC700 + S-DSP together and bridges to the main CPU via APUIO.
- **`main_bus/`** — Routes memory accesses to the correct device (WRAM, cartridge, PPU, APU, clock).

---

## For AI Agents

- **When modifying a component**, ensure you only import from `common/` and do not add cross-component dependencies.
- **The `cartridge.rs` header parser** uses heuristics (non-empty name, matching mapping mode) to choose between LoRom and HiRom headers.
- **Clock timing is extremely sensitive.** The `advance_master_clock` method chunks advances into ≤64-cycle ticks to avoid missing events. See `clock.rs` tests for exact reference behavior.
- **All components derive `bitcode::Encode/Decode`** where applicable for save-state serialization.
