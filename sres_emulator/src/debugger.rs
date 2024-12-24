//! Debugger functionality
//!
//! Allows internal components to notify of events during emulation (e.g. memory access),
//! and allows the front end to set and detect breakpoints on those events.

use std::cell::RefCell;
use std::fmt::Display;
use std::fmt::UpperHex;
use std::ops::Range;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::atomic::Ordering;

use num_traits::PrimInt;

use crate::apu::ApuBusEvent;
use crate::apu::ApuBusEventFilter;
use crate::common::address::AddressU24;
use crate::common::debug_events::DebugErrorCollector;
use crate::common::debug_events::DebugEventCollector;
use crate::common::debug_events::DebugEventLogger;
use crate::common::debug_events::DebuggerConfig;
use crate::common::debug_events::EventFilter;
use crate::common::debug_events::DEBUG_EVENTS_ENABLED;
use crate::common::util::RingBuffer;
use crate::components::cpu::CpuEvent;
use crate::components::cpu::CpuEventFilter;
use crate::components::cpu::CpuState;
use crate::components::cpu::NativeVectorTable;
use crate::components::spc700::Spc700Event;
use crate::components::spc700::Spc700EventFilter;
use crate::main_bus::MainBusEvent;
use crate::main_bus::MainBusEventFilter;

#[derive(Clone, Debug, PartialEq)]
pub enum DebugEvent {
    Cpu(CpuEvent),
    MainBus(MainBusEvent),
    ApuBus(ApuBusEvent),
    Spc700(Spc700Event),
    Error(String),
}

impl From<CpuEvent> for DebugEvent {
    fn from(event: CpuEvent) -> Self {
        DebugEvent::Cpu(event)
    }
}

impl From<MainBusEvent> for DebugEvent {
    fn from(event: MainBusEvent) -> Self {
        DebugEvent::MainBus(event)
    }
}

impl From<ApuBusEvent> for DebugEvent { 
    fn from(event: ApuBusEvent) -> Self {
        DebugEvent::ApuBus(event)
    }
}

impl From<Spc700Event> for DebugEvent {
    fn from(event: Spc700Event) -> Self {
        DebugEvent::Spc700(event)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AnyEventFilter {
    Cpu(CpuEventFilter),
    MainBus(MainBusEventFilter),
    ApuBus(ApuBusEventFilter),
    Spc700(Spc700EventFilter),
    ExecutionError,
}

impl AnyEventFilter {
    pub fn matches(&self, event: &DebugEvent) -> bool {
        match (self, event) {
            (AnyEventFilter::Cpu(filter), DebugEvent::Cpu(event)) => filter.matches(event),
            (AnyEventFilter::MainBus(filter), DebugEvent::MainBus(event)) => filter.matches(event),
            (AnyEventFilter::ApuBus(filter), DebugEvent::ApuBus(event)) => filter.matches(event),
            (AnyEventFilter::Spc700(filter), DebugEvent::Spc700(event)) => filter.matches(event),
            (AnyEventFilter::ExecutionError, DebugEvent::Error(_)) => true,
            _ => false,
        }
    }
}

impl From<CpuEventFilter> for AnyEventFilter {
    fn from(filter: CpuEventFilter) -> Self {
        AnyEventFilter::Cpu(filter)
    }
}

impl From<MainBusEventFilter> for AnyEventFilter {
    fn from(filter: MainBusEventFilter) -> Self {
        AnyEventFilter::MainBus(filter)
    }
}

impl From<ApuBusEventFilter> for AnyEventFilter {
    fn from(filter: ApuBusEventFilter) -> Self {
        AnyEventFilter::ApuBus(filter)
    }
}

impl From<Spc700EventFilter> for AnyEventFilter {
    fn from(filter: Spc700EventFilter) -> Self {
        AnyEventFilter::Spc700(filter)
    }
}

impl FromStr for AnyEventFilter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key, arg) = s.split_once(' ').unwrap_or((s, ""));
        Ok(match key.to_lowercase().as_str() {
            "pc" => AnyEventFilter::Cpu(CpuEventFilter::ProgramCounter(parse_range(arg)?)),
            "irq" => AnyEventFilter::Cpu(CpuEventFilter::Interrupt(if arg.is_empty() {
                None
            } else {
                Some(NativeVectorTable::from_str(arg)?)
            })),
            "r" => AnyEventFilter::MainBus(MainBusEventFilter::MemoryRead(parse_range(arg)?)),
            "w" => AnyEventFilter::MainBus(MainBusEventFilter::MemoryWrite(parse_range(arg)?)),
            "s-pc" => AnyEventFilter::Spc700(Spc700EventFilter::ProgramCounter(parse_range(arg)?)),
            "error" => AnyEventFilter::ExecutionError,
            &_ => AnyEventFilter::Cpu(CpuEventFilter::Instruction(s.to_string())),
        })
    }
}

impl Display for AnyEventFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyEventFilter::Cpu(filter) => write!(f, "{}", filter),
            AnyEventFilter::MainBus(filter) => write!(f, "{}", filter),
            AnyEventFilter::ApuBus(filter) => write!(f, "{}", filter),
            AnyEventFilter::Spc700(filter) => write!(f, "{}", filter),
            AnyEventFilter::ExecutionError => write!(f, "error"),
        }
    }
}

impl Display for CpuEventFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuEventFilter::ProgramCounter(range) => write!(f, "pc{}", format_range(range)),
            CpuEventFilter::Instruction(s) => write!(f, "{}", s),
            CpuEventFilter::Interrupt(handler) => {
                if let Some(handler) = handler {
                    write!(f, "irq {}", handler)
                } else {
                    write!(f, "irq")
                }
            }
        }
    }
}

impl Display for MainBusEventFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainBusEventFilter::MemoryRead(range) => write!(f, "r{}", format_range(range)),
            MainBusEventFilter::MemoryWrite(range) => write!(f, "w{}", format_range(range)),
        }
    }
}

impl Display for ApuBusEventFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApuBusEventFilter::MemoryRead(range) => write!(f, "spc-r{}", format_range(range)),
            ApuBusEventFilter::MemoryWrite(range) => write!(f, "spc-w{}", format_range(range)),
        }
    }
}

impl Display for Spc700EventFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spc700EventFilter::ProgramCounter(range) => write!(f, "spc-pc{}", format_range(range)),
        }
    }
}

pub struct BreakReason {
    pub trigger: AnyEventFilter,
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

pub type DebuggerRef = Rc<RefCell<Debugger>>;

pub struct Debugger {
    pub log_points: Vec<AnyEventFilter>,
    pub break_points: Vec<AnyEventFilter>,
    pub log: RingBuffer<DebugEvent, 1024>,
    pub break_reason: Option<BreakReason>,
    pub enabled: bool,
}

impl Debugger {
    pub fn new() -> DebuggerRef {
        Rc::new(RefCell::new(Debugger {
            log_points: Vec::new(),
            break_points: Vec::new(),
            log: RingBuffer::default(),
            break_reason: None,
            enabled: false,
        }))
    }

    pub fn collect_events<EventT: Into<DebugEvent>, EventFilterT: EventFilter<EventT>>(&mut self, logger: &mut DebugEventLogger<EventT, EventFilterT>) {
        for event in logger.log.drain() {
            self.collect_debug_event(event.into());
        }
    } 

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
    pub fn cpu_trace(&self) -> impl Iterator<Item = &CpuState> {
        self.log.iter().filter_map(|line| match line {
            DebugEvent::Cpu(CpuEvent::Step(cpu)) => Some(cpu),
            _ => None,
        })
    }

    pub fn take_break_reason(&mut self) -> Option<BreakReason> {
        self.break_reason.take()
    }

    pub fn has_break_point(&self, trigger: &AnyEventFilter) -> bool {
        self.break_points.iter().any(|t| t == trigger)
    }

    pub fn add_break_point(&mut self, trigger: AnyEventFilter) {
        if !self.has_break_point(&trigger) {
            self.break_points.push(trigger);
        }
    }

    pub fn remove_break_point(&mut self, trigger: &AnyEventFilter) {
        self.break_points.retain(|t| t != trigger)
    }

    pub fn toggle_break_point(&mut self, trigger: AnyEventFilter) {
        if self.has_break_point(&trigger) {
            self.remove_break_point(&trigger)
        } else {
            self.add_break_point(trigger)
        }
    }

    pub fn has_log_point(&self, trigger: &AnyEventFilter) -> bool {
        self.log_points.iter().any(|t| t == trigger)
    }

    pub fn add_log_point(&mut self, trigger: AnyEventFilter) {
        let trigger = trigger.into();
        if !self.has_log_point(&trigger) {
            self.log_points.push(trigger);
        }
    }

    pub fn remove_log_point(&mut self, trigger: &AnyEventFilter) {
        self.log_points.retain(|t| t != trigger)
    }

    pub fn toggle_log_point(&mut self, trigger: AnyEventFilter) {
        if self.has_log_point(&trigger) {
            self.remove_log_point(&trigger)
        } else {
            self.add_log_point(trigger)
        }
    }

    fn collect_debug_event(&mut self, event: DebugEvent) {
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

impl DebugErrorCollector for Debugger {
    fn on_error(&mut self, message: String) {
        self.collect_debug_event(DebugEvent::Error(message));
    }
}

impl DebugEventCollector<ApuBusEvent> for Debugger {
    fn on_event(&mut self, event: ApuBusEvent) {
        self.collect_debug_event(DebugEvent::ApuBus(event));
    }
}

impl DebugEventCollector<Spc700Event> for Debugger {
    fn on_event(&mut self, event: Spc700Event) {
        self.collect_debug_event(DebugEvent::Spc700(event));
    }
}

impl DebugEventCollector<CpuEvent> for Debugger {
    fn on_event(&mut self, event: CpuEvent) {
        self.collect_debug_event(DebugEvent::Cpu(event));
    }
}

impl DebugEventCollector<MainBusEvent> for Debugger {
    fn on_event(&mut self, event: MainBusEvent) {
        self.collect_debug_event(DebugEvent::MainBus(event));
    }
}

impl DebugEventCollector<()> for Debugger {
    fn on_event(&mut self, _event: ()) {}
}

pub trait DebugEventLoggerConfigBuilder<EventT, EventFilterT: EventFilter<EventT>> {
    fn build_debug_event_logger_config(&self) -> DebuggerConfig<EventT, EventFilterT>;
}

impl DebugEventLoggerConfigBuilder<CpuEvent, CpuEventFilter> for Debugger {
    fn build_debug_event_logger_config(&self) -> DebuggerConfig<CpuEvent, CpuEventFilter> {
        DebuggerConfig::new(
            self.enabled,
            self.log_points.iter().filter_map(|filter| {
                match filter {
                    AnyEventFilter::Cpu(filter) => Some(filter),
                    _ => None,
                }
            }).cloned().collect()
        )
    }
}

impl DebugEventLoggerConfigBuilder<MainBusEvent, MainBusEventFilter> for Debugger {
    fn build_debug_event_logger_config(&self) -> DebuggerConfig<MainBusEvent, MainBusEventFilter> {
        DebuggerConfig::new(
            self.enabled,
            self.log_points.iter().filter_map(|filter| {
                match filter {
                    AnyEventFilter::MainBus(filter) => Some(filter),
                    _ => None,
                }
            }).cloned().collect()
        )
    }
}

impl DebugEventLoggerConfigBuilder<ApuBusEvent, ApuBusEventFilter> for Debugger {
    fn build_debug_event_logger_config(&self) -> DebuggerConfig<ApuBusEvent, ApuBusEventFilter> {
        DebuggerConfig::new(
            self.enabled,
            self.log_points.iter().filter_map(|filter| {
                match filter {
                    AnyEventFilter::ApuBus(filter) => Some(filter),
                    _ => None,
                }
            }).cloned().collect()
        )
    }
}   

impl DebugEventLoggerConfigBuilder<Spc700Event, Spc700EventFilter> for Debugger {
    fn build_debug_event_logger_config(&self) -> DebuggerConfig<Spc700Event, Spc700EventFilter> {
        DebuggerConfig::new(
            self.enabled,
            self.log_points.iter().filter_map(|filter| {
                match filter {
                    AnyEventFilter::Spc700(filter) => Some(filter),
                    _ => None,
                }
            }).cloned().collect()
        )
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
        let check_format = |filter: &str, expected: AnyEventFilter| {
            assert_eq!(format!("{}", expected), filter);
            assert_eq!(filter.parse::<AnyEventFilter>().unwrap(), expected);
        };

        check_format(
            "pc 0",
            AnyEventFilter::Cpu(CpuEventFilter::ProgramCounter(0..1)),
        );
        check_format(
            "jmp",
            AnyEventFilter::Cpu(CpuEventFilter::Instruction("jmp".to_string())),
        );
        check_format(
            "irq nmi",
            AnyEventFilter::Cpu(CpuEventFilter::Interrupt(Some(NativeVectorTable::Nmi))),
        );
        check_format(
            "r 10:1F",
            AnyEventFilter::MainBus(MainBusEventFilter::MemoryRead(0x10..0x20)),
        );
        check_format(
            "w",
            AnyEventFilter::MainBus(MainBusEventFilter::MemoryWrite(0..u32::MAX)),
        );
    }
}
