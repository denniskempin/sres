# sres_emulator/src/common

Foundational types and utilities shared across the emulator.

## Files

| File | Purpose |
|------|---------|
| `address.rs` | Typed addresses (`AddressU24`, `AddressU16`, `AddressU15`) with explicit wrapping. |
| `bus.rs` | `Bus` trait (generic over address width), `BusDeviceU24` for memory-mapped I/O. |
| `clock.rs` | `ClockInfo`: master-clock to scanline/frame conversion. |
| `uint.rs` | `UInt` trait for generic 8/16-bit ops, `VariableLengthUInt`, bit manipulation. |
| `util.rs` | `RingBuffer`, `EdgeDetector`, hex formatting. |
| `logging.rs` | `SresLogger` with trace ring buffer and colored output. |
| `debug_events.rs` | `DebugEventCollector`/`DebugEventCollectorRef` with zero-cost disabled path. |
| `test_util.rs` | WAV golden-file comparison for audio tests. |
| `test_bus.rs` | `TestBus` with cycle recording and sparse memory (test-only). |
| `image.rs` | SNES color types (`Rgb15`, `Rgba32`) and `Image` trait. |

## Key Details

- **Addresses**: All arithmetic uses explicit `Wrap` parameter (`WrapPage`, `WrapBank`, `NoWrap`). Never use raw `+`/`-`.
- **Bus**: Generic over `AddressT`, reused for both U24 (CPU) and U16 (SPC700) buses.
- **Clock**: `master_clock` is the single source of truth. Components receive `ClockInfo`, don't track their own timing.
- **UInt**: `UInt` trait lets CPU instructions work generically for 8-bit and 16-bit modes.
- **Logging**: `trace_as_context_only` buffers trace logs; dumped only when warning/error occurs.
- **Debug Events**: `DEBUG_EVENTS_ENABLED` atomic flag + `#[cold]` dispatch = zero-cost when disabled.
- **Image**: PPU renders `Rgb15`, converted to `Rgba32` at presentation time.
