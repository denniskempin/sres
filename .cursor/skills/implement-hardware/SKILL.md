---
name: implement-hardware
description: "Plan or implement an unimplemented SNES hardware Gaps item. Use when the user asks to add, spec, or implement a feature listed under Error Handling & Unimplemented Hardware, or a leaf AGENTS.md Gaps entry."
---

# Implement unimplemented hardware

SOP for turning a Gaps item into a plan (and later code). Hardware bitfields live in `docs/`. Facts about this codebase live in `AGENTS.md`. This file is only the procedure.

## When

Use when the user asks to implement, plan, or spec something listed in:

- Root `AGENTS.md` → Error Handling & Unimplemented Hardware
- A leaf `AGENTS.md` → Gaps

Examples of current Gaps items: HDMA, FastROM, serial `$4016`/`$4017`, open bus, joypad auto-read, PPU windows / MOSAIC / brightness / Mode 7 render. Do not use this skill for refactors, docs-only edits, or frontend work.

After the plan is written, run `.cursor/skills/code-review/SKILL.md` in **plan** mode before implementing. After implementation, run that skill in **diff** mode.

## Procedure

1. **Confirm the gap.** Read root Gaps and the leaf `AGENTS.md` Gaps for the owning directory. Do not repo-search “is this implemented?”
2. **Read the spec.** Keyword-search `docs/index.md`. Open only the listed files. Do not glob `docs/`.
3. **Read the integration owner before designing.** Typically `sres_emulator/src/main_bus/` (MMIO, DMA, `advance_master_clock`), `sres_emulator/src/components/clock.rs` (H/V events), the device `AGENTS.md` (PPU/APU), and the nearest test `AGENTS.md`. Note existing latch / ordering gotchas.
4. **Write the plan** in this order. Omit a section only if it has nothing to say.
   1. Hardware summary with doc citations (no pasted register tables).
   2. Current code and Gaps (file paths, what already warns / returns 0).
   3. Design: registers, sequencing, Clock triggers, wiring into `advance_master_clock` or the device.
   4. Tests by root taxonomy (trace / ROM-outcome / golden-image / golden-WAV / lib unit). Name the System variant.
   5. `AGENTS.md` / `//!` updates (follow `write-agents-docs`).
   6. Explicit remaining Gaps / simplifications.
5. **Clock H/V events.** Do not copy the DRAM-refresh “did `h_counter` cross N in this chunk?” idiom for positions near 0. Read `sres_emulator/src/components/AGENTS.md` (scanline-wrap latch). New triggers: per-line or per-frame latch **after** the wrap in `tick_master_clock`.
6. **Idle cycle cost.** If the feature can run inside `advance_master_clock`, it must charge **zero** master cycles when disabled / inactive. Trace strings include `V:`/`H:` (root Testing Strategy). Name `krom_*` / `ppu_timing` as the regression gate in the plan. `MainBusImpl::reset` already crosses early-scanline positions via `advance_master_clock(186)`.
7. **DMA slot.** Consume timed transfers at the top of `advance_master_clock` (existing DMA → clock tick → device `update_clock` order). Mid-line writes to the PPU apply before `draw_scanline(v+1)` (`components/ppu` AGENTS.md). Call `update_clock` before timestamped batched writes.
8. **Tests without `bass`.** Cloud Agent VMs have no assembler (root Environment Gotchas). Prefer:
   - `MainBusImpl::new` with a recording `BusDeviceU24` stub
   - `Cartridge::with_program` (LoRom; ROM byte 0 = `$00:8000`; empty SRAM — `components` AGENTS.md)
   - Reuse a committed `.sfc` that already exercises the feature
   A golden that captured unimplemented behavior (black frame, missing effect) is a stub: delete it, rerun, visually verify (`tests/ppu_tests` AGENTS.md).
9. **Layer boundaries.** Integration (MMIO routing, clock advance, DMA side effects) stays in `main_bus/` or `lib.rs`. Components import `common/` only. New files stay `mod`, not `pub mod`, unless they are a facade.
10. **Error policy.** Unimplemented leftovers still return `0` + `DebugEvent` / `log::warn` per root. Do not `panic!` or `unimplemented!()` on a reachable game path.

## Plan review gate

Do not start coding until the plan-mode code-review report is `approve` or every blocker is folded into the plan. Then implement, run the named tests, `./check-all.sh`, and `cargo clippy --workspace --all-targets`.

## Hard rules

- Do not paste hardware register tables into `AGENTS.md` or this skill. Point at `docs/index.md`.
- Do not “fix” an unrelated documented divergence (example: GP-DMA `Wrap::NoWrap`) in the same change.
- Do not assemble new `.sfc` in Cloud Agent. If `bass` is required, say so and stop.
- Update leaf `AGENTS.md` Gaps in the same PR as the implementation (`write-agents-docs`).
