# `sres_emulator/src/common`

Foundational types shared by all emulator layers.

## Files

| File | Owns |
|------|------|
| `mod.rs` | Module declarations. |
| `address.rs` | `Address`, `Wrap`, `AddressU24`, `AddressU16`, `AddressU15`, `InstructionMeta`. |
| `bus.rs` | `Bus<AddressT>`, `BusDeviceU24`. |
| `clock.rs` | `ClockInfo`. |
| `debug_events.rs` | `DebugEventCollector`, `DebugEventCollectorRef`, `DEBUG_EVENTS_ENABLED`, `noop_collector`. |
| `unimplemented.rs` | `UnimplementedBehavior`. Lives here so components can emit without importing `debugger`. |
| `image.rs` | `Rgb15`, `Rgba32`, `ColorIdx`, `Image`. |
| `logging.rs` | `SresLogger`, `init()`, `test_init()`. |
| `test_bus.rs` | `TestBus`, `Cycle`, `SparseMemory`. File is `#![cfg(test)]`. |
| `test_util.rs` | `compare_wav_against_golden`, 32 kHz mono i16 WAV. |
| `uint.rs` | `UInt`, `UIntTruncate`, `VariableLengthUInt`, `U8Ext`/`U16Ext`/`U32Ext`. |
| `util.rs` | `RingBuffer`, `EdgeDetector`, `format_memory`. |

## Behaviors & Gotchas

1. `AddressU24`/`AddressU16` have no `Add`/`Sub`. Use `add`/`sub`/`add_signed` with `Wrap` (`WrapPage`, `WrapBank`, `NoWrap`). `AddressU16` `WrapBank` is `unimplemented!()`. `AddressU15` (PPU VRAM) does not implement `Address`; `+`/`-` wrap with `& 0x7FFF`.
2. `ClockInfo::from_mesen_vhf` maps Mesen traces, which increment `f` at vblank (`v = 225`), not `v = 0`. Use `from_master_clock` elsewhere. `vblank()` is `v >= 225`. Short scanline and 6-cycle dots: `components/clock.rs`.
3. `init()` reads `SRES_LOG` (default `error`) and sets `trace_as_context_only`. Traces buffer (20 lines) and dump on the next non-`Trace` record, not only warnings. `test_init(verbose)` sets `trace_as_context_only = !verbose`. `init` and `test_init` share one `Once`; the first caller wins.
4. `TestBus` records `Cycle::Read`/`Write`/`Internal` on every bus cycle. Unmapped reads store `None` and return `0`.

## Integration

- `MainBus: Bus<AddressU24>` (CPU) and `Spc700Bus: Bus<AddressU16>` (SPC700). `MainBusImpl` implements `Bus<AddressU24>`; devices implement `BusDeviceU24`.
- PPU VRAM uses `AddressU15`. CGRAM and `Framebuffer` use `Rgb15`. OAM RAM uses `OamAddr`; sprite nametables are `AddressU15`. `Image` is implemented in `sres_egui` and `tests/ppu_tests.rs`.
- `components/clock.rs` uses `EdgeDetector`. `SystemImpl` consumes `ClockInfo`.
- Components emit through `DebugEventCollectorRef` (`on_event` / `on_error` / `on_unimplemented`). `Debugger` stores events in `RingBuffer` and unimplemented hits in a `HashMap`.
- Native frontend calls `logging::init()`. Tests call `logging::test_init`. CPU tests use `TestBus` and `debug_events::test::mock_collector`. APU tests use `compare_wav_against_golden`.

## Gaps

- No SNES MMIO in this directory. Unknown-register policy is root `on_error`. `AddressU16` `WrapBank` panics (`unimplemented!()`), which matches root (panics are internal logic errors).

## Tests

- Unit tests: `clock.rs` (`from_mesen_vhf`), `uint.rs` (BCD add). Other files have none. `debug_events::test::mock_collector` aliases `noop_collector`.
- `cargo nextest run -p sres_emulator --lib -E 'test(common::)'`

