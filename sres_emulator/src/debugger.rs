//! Debugger functionality
//!
//! Allows internal components to notify of events during emulation (e.g. memory access),
//! and allows the front end to set and detect breakpoints on those events.

use std::fmt::Display;
use std::fmt::UpperHex;

use std::ops::Range;

use std::str::FromStr;

use std::sync::atomic::Ordering;


use num_traits::PrimInt;



use crate::common::address::AddressU24;
use crate::common::constants::NativeVectorTable;
use crate::common::debug_events::ApuEvent;
use crate::common::debug_events::CpuEvent;
use crate::common::debug_events::DebugEvent;
use crate::common::debug_events::DebugEventCollector;

use crate::common::debug_events::DEBUG_EVENTS_ENABLED;
use crate::common::trace::CpuTraceLine;

use crate::common::util::RingBuffer;

/*
impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Event::*;
        match self {
            CpuStep(cpu) => write!(f, "{}", cpu),
            CpuMemoryRead(addr, value) => write!(f, "R {} = {:X}", addr, value),
            CpuMemoryWrite(addr, data) => write!(f, "W {} = {:X}", addr, data),
            ExecutionError(msg) => write!(f, "Error: {}", msg),
            CpuInterrupt(handler) => write!(f, "Interrupt {}", handler),
            Spc700Step(spc) => write!(f, "{}", spc),
            Spc700MemoryRead(addr, value) => write!(f, "S-R {} = {:X}", addr, value),
            Spc700MemoryWrite(addr, data) => write!(f, "S-W {} = {:X}", addr, data),
        }
    }
}
*/

#[derive(Clone, PartialEq, Debug)]
pub enum EventFilter {
    CpuProgramCounter(Range<u32>),
    CpuInstruction(String),
    CpuMemoryRead(Range<u32>),
    CpuMemoryWrite(Range<u32>),
    ExecutionError,
    Interrupt(Option<NativeVectorTable>),
    Spc700ProgramCounter(Range<u16>),
    Spc700MemoryRead(Range<u16>),
    Spc700MemoryWrite(Range<u16>),
}

impl EventFilter {
    pub fn matches(&self, event: &DebugEvent) -> bool {
        use DebugEvent::*;
        use EventFilter::*;
        match (self, event) {
            (CpuProgramCounter(range), Cpu(CpuEvent::Step(cpu))) => {
                range.contains(&u32::from(cpu.instruction.address))
            }
            (CpuInstruction(instr), Cpu(CpuEvent::Step(cpu))) => {
                instr == &cpu.instruction.operation
            }
            (CpuMemoryRead(range), Cpu(CpuEvent::Read(addr, _))) => {
                range.contains(&u32::from(*addr))
            }
            (CpuMemoryWrite(range), Cpu(CpuEvent::Write(addr, _))) => {
                range.contains(&u32::from(*addr))
            }
            (ExecutionError, Error(_)) => true,
            (Interrupt(expected_handler), Cpu(CpuEvent::Interrupt(handler))) => {
                if let Some(expected_handler) = expected_handler {
                    expected_handler == handler
                } else {
                    true
                }
            }
            (Spc700ProgramCounter(range), Apu(ApuEvent::Step(spc))) => {
                range.contains(&spc.instruction.address.0)
            }
            (Spc700MemoryRead(range), Apu(ApuEvent::Read(addr, _))) => range.contains(&addr.0),
            (Spc700MemoryWrite(range), Apu(ApuEvent::Write(addr, _))) => range.contains(&addr.0),

            _ => false,
        }
    }
}

impl FromStr for EventFilter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EventFilter::*;
        let (key, arg) = s.split_once(' ').unwrap_or((s, ""));
        Ok(match key.to_lowercase().as_str() {
            "pc" => CpuProgramCounter(parse_range(arg)?),
            "irq" => Interrupt(if arg.is_empty() {
                None
            } else {
                Some(NativeVectorTable::from_str(arg)?)
            }),
            "r" => CpuMemoryRead(parse_range(arg)?),
            "w" => CpuMemoryWrite(parse_range(arg)?),
            "s-pc" => Spc700ProgramCounter(parse_range(arg)?),
            &_ => CpuInstruction(s.to_string()),
        })
    }
}

impl Display for EventFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use EventFilter::*;
        match self {
            CpuProgramCounter(range) => write!(f, "pc{}", format_range(range)),
            CpuInstruction(s) => write!(f, "{}", s),
            CpuMemoryRead(range) => write!(f, "r{}", format_range(range)),
            CpuMemoryWrite(range) => write!(f, "w{}", format_range(range)),
            ExecutionError => write!(f, "error"),
            Interrupt(handler) => {
                if let Some(handler) = handler {
                    write!(f, "irq {}", handler)
                } else {
                    write!(f, "irq")
                }
            }
            Spc700ProgramCounter(range) => write!(f, "spc-pc{}", format_range(range)),
            Spc700MemoryRead(range) => write!(f, "spc-r{}", format_range(range)),
            Spc700MemoryWrite(range) => write!(f, "spc-w{}", format_range(range)),
        }
    }
}

pub struct BreakReason {
    pub trigger: EventFilter,
    pub event: DebugEvent,
}

pub enum MemoryAccess {
    Read(AddressU24),
    Write(AddressU24, u8),
}

impl MemoryAccess {
    pub fn addr(&self) -> AddressU24 {
        match self {
            MemoryAccess::Read(addr) => *addr,
            MemoryAccess::Write(addr, _) => *addr,
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq)]
pub enum Trigger {
    ProgramCounter(Range<u32>),
    CpuMemoryRead(Range<u32>),
    CpuMemoryWrite(Range<u32>),
    ExecutionError,
    Interrupt(NativeVectorTable),
}

pub struct Debugger {
    pub log_points: Vec<EventFilter>,
    pub break_points: Vec<EventFilter>,
    pub log: RingBuffer<DebugEvent, 1024>,
    pub break_reason: Option<BreakReason>,
    pub enabled: bool,
}

impl Debugger {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        DEBUG_EVENTS_ENABLED.store(true, Ordering::Relaxed);
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        DEBUG_EVENTS_ENABLED.store(false, Ordering::Relaxed);
    }

    /// Frontend facing API
    pub fn cpu_trace(&self) -> impl Iterator<Item = &CpuTraceLine> {
        self.log.iter().filter_map(|line| match line {
            DebugEvent::Cpu(CpuEvent::Step(cpu)) => Some(cpu),
            _ => None,
        })
    }

    pub fn take_break_reason(&mut self) -> Option<BreakReason> {
        self.break_reason.take()
    }

    pub fn has_break_point(&self, trigger: &EventFilter) -> bool {
        self.break_points.iter().any(|t| t == trigger)
    }

    pub fn add_break_point(&mut self, trigger: EventFilter) {
        if !self.has_break_point(&trigger) {
            self.break_points.push(trigger);
        }
    }

    pub fn remove_break_point(&mut self, trigger: &EventFilter) {
        self.break_points.retain(|t| t != trigger)
    }

    pub fn toggle_break_point(&mut self, trigger: EventFilter) {
        if self.has_break_point(&trigger) {
            self.remove_break_point(&trigger)
        } else {
            self.add_break_point(trigger)
        }
    }

    pub fn has_log_point(&self, trigger: &EventFilter) -> bool {
        self.log_points.iter().any(|t| t == trigger)
    }

    pub fn add_log_point(&mut self, trigger: EventFilter) {
        if !self.has_log_point(&trigger) {
            self.log_points.push(trigger);
        }
    }

    pub fn remove_log_point(&mut self, trigger: &EventFilter) {
        self.log_points.retain(|t| t != trigger)
    }

    pub fn toggle_log_point(&mut self, trigger: EventFilter) {
        if self.has_log_point(&trigger) {
            self.remove_log_point(&trigger)
        } else {
            self.add_log_point(trigger)
        }
    }

    /// Internal API
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            log_points: vec![],
            break_points: vec![],
            break_reason: None,
            log: RingBuffer::default(),
            enabled: false,
        }
    }
}

impl DebugEventCollector for Debugger {
    fn collect_event(&mut self, event: DebugEvent) {
        let break_trigger = self
            .break_points
            .iter()
            .find(|log_filter| log_filter.matches(&event));
        if let Some(break_trigger) = break_trigger {
            self.break_reason = Some(BreakReason {
                event: event.clone(),
                trigger: break_trigger.clone(),
            });
        }

        if self
            .log_points
            .iter()
            .any(|log_filter| log_filter.matches(&event))
        {
            self.log.push(event);
        }
    }
}

/// Parses a hex 1234:5678 range string into a Range<u32>
fn parse_range<T: PrimInt + UpperHex>(s: &str) -> anyhow::Result<Range<T>> {
    let split = s.split_once(':');
    if let Some((left, right)) = split {
        let start = if left.is_empty() {
            T::zero()
        } else {
            T::from_str_radix(left, 16).map_err(|_| anyhow::anyhow!("Invalid range"))?
            // TODO: better error message
        };
        let end = if right.is_empty() {
            T::max_value()
        } else {
            T::one() + T::from_str_radix(right, 16).map_err(|_| anyhow::anyhow!("Invalid range"))?
        };
        Ok(Range { start, end })
    } else if s.is_empty() {
        Ok(Range {
            start: T::zero(),
            end: T::max_value(),
        })
    } else {
        let value = T::from_str_radix(s, 16).map_err(|_| anyhow::anyhow!("Invalid range"))?;
        Ok(Range {
            start: value,
            end: value + T::one(),
        })
    }
}

/// Formats a Range<UInt> into the same format as [parse_range]
fn format_range<T: PrimInt + UpperHex>(range: &Range<T>) -> String {
    if range.start == T::zero() && range.end == T::max_value() {
        "".to_string()
    } else if range.start + T::one() == range.end {
        format!(" {:X}", range.start)
    } else if range.start == T::zero() {
        format!(" :{:X}", range.end - T::one())
    } else if range.end == T::max_value() {
        format!(" {:X}:", range.start)
    } else {
        format!(" {:X}:{:X}", range.start, range.end - T::one())
    }
}

#[cfg(test)]
mod test {
    use crate::common::constants::NativeVectorTable;

    use super::*;

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("2:6").unwrap(), 2..7);
        assert_eq!(parse_range("2:").unwrap(), 2..u32::MAX);
        assert_eq!(parse_range(":6").unwrap(), 0..7);
        assert_eq!(parse_range("6").unwrap(), 6..7);
        assert_eq!(parse_range("").unwrap(), 0..u32::MAX);
    }

    #[test]
    fn test_format_range() {
        assert_eq!(format_range(&(0..0x10)), " :F");
        assert_eq!(format_range(&(0x6..0x7)), " 6");
        assert_eq!(format_range(&(0x6..0x10)), " 6:F");
        assert_eq!(format_range(&(0..u32::MAX)), "");
    }

    #[test]
    fn test_trace_filter_format() {
        let check_format = |filter: &str, expected: EventFilter| {
            assert_eq!(format!("{}", expected), filter);
            assert_eq!(filter.parse::<EventFilter>().unwrap(), expected);
        };

        use EventFilter::*;
        check_format("pc 0", CpuProgramCounter(0..1));
        check_format("jmp", CpuInstruction("jmp".to_string()));
        check_format("irq nmi", Interrupt(Some(NativeVectorTable::Nmi)));
        check_format("r 10:1F", CpuMemoryRead(0x10..0x20));
        check_format("w", CpuMemoryWrite(0..u32::MAX));
    }
}
