//! `SDsp` register read/write sanity.
#![cfg(test)]

use super::*;
use crate::common::debug_events::DebugEventCollectorRef;
use crate::common::unimplemented::UnimplementedBehavior;
use crate::debugger::DebugEvent;
use crate::debugger::Debugger;
use crate::debugger::EventFilter;

#[test]
fn test_read_write_register() {
    let mut s_dsp = SDsp::default();
    s_dsp.write_register(0x0, 0x12);
    assert_eq!(s_dsp.read_register(0x0), 0x12);
}

#[test]
fn unused_registers_store_in_raw() {
    let mut s_dsp = SDsp::default();
    for reg in [
        0x1D, 0x0A, 0x1A, 0x2A, 0x3A, 0x4A, 0x5A, 0x6A, 0x7A, 0x0B, 0x1B, 0x2B, 0x3B, 0x4B, 0x5B,
        0x6B, 0x7B, 0x0E, 0x1E, 0x2E, 0x3E, 0x4E, 0x5E, 0x6E, 0x7E,
    ] {
        s_dsp.write_register(reg, 0xAB);
        assert_eq!(s_dsp.read_register(reg), 0xAB, "unused ${reg:02X}");
    }
}

#[test]
fn kon_is_only_4c() {
    let mut s_dsp = SDsp::default();
    s_dsp.write_register(0x4C, 0x01);
    assert!(s_dsp.voices[0].trigger_on);
    assert!(!s_dsp.voices[1].trigger_on);

    let mut s_dsp = SDsp::default();
    s_dsp.write_register(0x5C, 0xFF);
    assert!(s_dsp.voices.iter().all(|v| !v.trigger_on));
}

#[test]
fn unknown_register_does_not_hit_named_map() {
    let mut s_dsp = SDsp::default();
    s_dsp.write_register(0x00, 0x12);
    s_dsp.write_register(0x80, 0x34);
    assert_eq!(s_dsp.read_register(0x00), 0x12);
}

#[test]
fn unused_is_not_unknown_or_unimplemented() {
    let debugger = Debugger::new();
    debugger.lock().unwrap().enable();
    debugger
        .lock()
        .unwrap()
        .add_log_point(EventFilter::ExecutionError);
    let mut s_dsp = SDsp::new(DebugEventCollectorRef(debugger.clone()));

    s_dsp.write_register(0x1D, 0xAB);
    s_dsp.write_register(0x0A, 0xCD);
    s_dsp.write_register(0x0B, 0xEF);
    s_dsp.write_register(0x0E, 0x11);
    {
        let mut d = debugger.lock().unwrap();
        assert!(d.unimplemented_hits().is_empty());
        assert!(d
            .drain_events(|e| match e {
                DebugEvent::Error(_) => Some(()),
                _ => None,
            })
            .is_empty());
    }

    s_dsp.write_register(0x0C, 0x00);
    {
        let mut d = debugger.lock().unwrap();
        assert_eq!(
            d.unimplemented_hits(),
            vec![(UnimplementedBehavior::DspMvol, 1)]
        );
        assert!(d
            .drain_events(|e| match e {
                DebugEvent::Error(_) => Some(()),
                _ => None,
            })
            .is_empty());
    }

    s_dsp.write_register(0x80, 0x34);
    {
        let mut d = debugger.lock().unwrap();
        assert_eq!(
            d.unimplemented_hits(),
            vec![(UnimplementedBehavior::DspMvol, 1)]
        );
        let errors = d.drain_events(|e| match e {
            DebugEvent::Error(msg) => Some(msg.clone()),
            _ => None,
        });
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert!(
            errors[0].contains("unknown S-DSP register $80"),
            "{errors:?}"
        );
    }

    debugger.lock().unwrap().disable();
}
