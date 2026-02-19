use std::collections::HashMap;

use egui::Color32;
use egui::CursorIcon;
use egui::RichText;
use egui::Ui;
use lazy_static::lazy_static;
use sres_emulator::apu::ApuBusEvent;
use sres_emulator::common::address::AddressU16;
use sres_emulator::common::address::AddressU24;
use sres_emulator::common::address::InstructionMeta;
use sres_emulator::components::cpu::CpuEvent;
use sres_emulator::components::cpu::CpuState;
use sres_emulator::components::spc700::Spc700Event;
use sres_emulator::components::spc700::Spc700State;
use sres_emulator::debugger::DebugEvent;
use sres_emulator::main_bus::MainBusEvent;

use crate::debug::InternalLink;

pub fn cpu_disassembly_line(
    ui: &mut Ui,
    meta: InstructionMeta<AddressU24>,
    current: bool,
    selected: &mut InternalLink,
) {
    ui.horizontal(|ui| {
        if current {
            ui.label("> ");
        } else {
            ui.label("  ");
        };
        label_cpu_disassembly(ui, &meta, selected);
    });
}

/// Widget to display a single log line with syntax highlighting, hover info and the ability
/// to click addresses.
pub fn log_line(ui: &mut Ui, event: &DebugEvent, selected: &mut InternalLink) {
    use DebugEvent::*;
    ui.horizontal(|ui| {
        match event {
            MainBus(MainBusEvent::Read(addr, value)) => {
                label_cpu(ui);
                label_read(ui, "R");
                label_cpu_effective_addr(ui, *addr, selected);
                label_normal(ui, format!("= {value:02X}"));
            }
            MainBus(MainBusEvent::Write(addr, value)) => {
                label_cpu(ui);
                label_write(ui, "W");
                label_cpu_effective_addr(ui, *addr, selected);
                label_normal(ui, format!("= {value:02X}"));
            }
            Cpu(CpuEvent::Interrupt(interrupt)) => {
                label_cpu(ui);
                label_normal(ui, format!("IRQ: {interrupt:?}"));
            }
            Error(reason) => {
                label_error(ui, format!("Error: {reason:?}"));
            }
            Cpu(CpuEvent::Step(state)) => {
                label_cpu(ui);
                cpu_log_line(ui, state, selected);
            }
            Spc700(Spc700Event::Step(state)) => {
                spc700_log_line(ui, state, selected);
            }
            ApuBus(ApuBusEvent::Read(addr, value)) => {
                label_spc(ui);
                label_read(ui, "R");
                label_addr(ui, addr.to_string());
                label_normal(ui, format!("= {value:02X}"));
            }
            ApuBus(ApuBusEvent::Write(addr, value)) => {
                label_spc(ui);
                label_write(ui, "W");
                label_addr(ui, addr.to_string());
                label_normal(ui, format!("= {value:02X}"));
            }
        };
    });
}

pub fn cpu_log_line(ui: &mut Ui, state: &CpuState, selected: &mut InternalLink) {
    label_cpu_disassembly(ui, &state.instruction, selected);
    label_strong(ui, "A");
    label_normal(ui, format!("{:04X}", state.a));
    label_strong(ui, "X");
    label_normal(ui, format!("{:04X}", state.x));
    label_strong(ui, "Y");
    label_normal(ui, format!("{:04X}", state.y));
    label_strong(ui, "S");
    label_normal(ui, format!("{:04X}", state.s));
    label_strong(ui, "D");
    label_normal(ui, format!("{:04X}", state.d));
    label_strong(ui, "DB");
    label_normal(ui, format!("{:02X}", state.db));
    label_strong(ui, state.status.to_string());
}

pub fn spc700_log_line(ui: &mut Ui, state: &Spc700State, selected: &mut InternalLink) {
    label_spc(ui);
    label_spc700_pc(ui, state.instruction.address, selected);
    label_normal(ui, " ");
    label_strong(ui, format!("{:<5}", state.instruction.operation));
    label_strong(
        ui,
        format!(
            "{:<11}",
            state.instruction.operand_str.as_deref().unwrap_or("")
        ),
    );
    label_strong(ui, "A");
    label_normal(ui, format!("{:02X}", state.a));
    label_strong(ui, "X");
    label_normal(ui, format!("{:02X}", state.x));
    label_strong(ui, "Y");
    label_normal(ui, format!("{:02X}", state.y));
    label_strong(ui, "SP");
    label_normal(ui, format!("{:02X}", state.sp.0));
    label_strong(ui, "P");
    label_normal(ui, state.status.to_string());
}

pub fn label_cpu_disassembly(
    ui: &mut Ui,
    meta: &InstructionMeta<AddressU24>,
    selected: &mut InternalLink,
) {
    label_cpu_pc(ui, meta.address, selected);
    label_strong(ui, meta.operation.clone());
    if let Some(operand_str) = &meta.operand_str {
        label_operand(ui, operand_str);
    } else {
        ui.label("         ");
    }
    if let Some(effective_addr) = meta.effective_addr {
        label_cpu_effective_addr(ui, effective_addr, selected);
    } else {
        ui.label("             ");
    }
}

pub fn label_addr(ui: &mut Ui, text: impl Into<String>) -> egui::Response {
    ui.label(RichText::new(text.into()).color(Color32::LIGHT_BLUE))
        .on_hover_cursor(CursorIcon::PointingHand)
}

pub fn label_cpu_effective_addr(
    ui: &mut Ui,
    addr: AddressU24,
    selected: &mut InternalLink,
) -> egui::Response {
    let addr_u32: u32 = addr.into();
    let response = if ADDR_ANNOTATIONS.contains_key(&addr_u32) {
        let hover_text = addr.to_string();
        let label_text = format!("[{}]", ADDR_ANNOTATIONS[&addr_u32]);
        label_addr(ui, format!("{label_text:<13}")).on_hover_text(hover_text)
    } else {
        let label_text = format!("[{addr}]");
        label_addr(ui, format!("{label_text:<13}"))
    };
    if response.clicked() {
        *selected = InternalLink::CpuMemory(addr);
    }
    response
}

pub fn label_cpu_pc(ui: &mut Ui, addr: AddressU24, selected: &mut InternalLink) {
    if label_addr(ui, addr.to_string()).clicked() {
        *selected = InternalLink::CpuProgramCounter(addr);
    }
}

pub fn label_spc700_pc(ui: &mut Ui, addr: AddressU16, selected: &mut InternalLink) {
    if label_addr(ui, addr.to_string()).clicked() {
        *selected = InternalLink::Spc700ProgramCounter(addr);
    }
}

pub fn label_cpu(ui: &mut Ui) -> egui::Response {
    ui.label(RichText::new("CPU:").color(Color32::LIGHT_GREEN))
}

pub fn label_spc(ui: &mut Ui) -> egui::Response {
    ui.label(RichText::new("SPC:").color(Color32::LIGHT_YELLOW))
}

pub fn label_read(ui: &mut Ui, text: impl Into<String>) -> egui::Response {
    ui.label(RichText::new(text.into()).color(Color32::LIGHT_GRAY))
}

pub fn label_write(ui: &mut Ui, text: impl Into<String>) -> egui::Response {
    ui.label(RichText::new(text.into()).color(Color32::LIGHT_RED))
}

pub fn label_operand(ui: &mut Ui, operand_str: impl Into<String>) -> egui::Response {
    let operand_str = operand_str.into();
    let mut text = RichText::new(format!("{:<9}", operand_str.clone())).strong();
    if operand_str.starts_with('$') || operand_str.starts_with('[') || operand_str.starts_with('(')
    {
        text = text.color(Color32::LIGHT_YELLOW);
    } else if operand_str.starts_with('#') {
        text = text.color(Color32::LIGHT_GREEN);
    } else if operand_str.starts_with('+') | operand_str.starts_with('-') {
        text = text.color(Color32::LIGHT_RED);
    }
    ui.label(text)
}

pub fn label_error(ui: &mut Ui, text: impl Into<String>) -> egui::Response {
    ui.label(RichText::new(text.into()).color(Color32::RED))
}

pub fn label_normal(ui: &mut Ui, text: impl Into<String>) -> egui::Response {
    ui.label(RichText::new(text.into()))
}

pub fn label_strong(ui: &mut Ui, text: impl Into<String>) -> egui::Response {
    ui.label(RichText::new(text.into()).color(Color32::WHITE).strong())
}

lazy_static! {
    pub static ref ADDR_ANNOTATIONS: HashMap<u32, &'static str> = {
        [
            // PPU Display Registers ($2100-$2114)
            (0x2100, "INIDISP"),  // Screen Display
            (0x2101, "OBSEL"),    // Object Size and Character Size
            (0x2102, "OAMADDL"),  // OAM Address Low
            (0x2103, "OAMADDH"),  // OAM Address High
            (0x2104, "OAMDATA"),  // OAM Data Write
            (0x2105, "BGMODE"),   // BG Mode and Character Size
            (0x2106, "MOSAIC"),   // Mosaic Size and Enable
            (0x2107, "BG1SC"),    // BG1 Screen Base and Size
            (0x2108, "BG2SC"),    // BG2 Screen Base and Size
            (0x2109, "BG3SC"),    // BG3 Screen Base and Size
            (0x210A, "BG4SC"),    // BG4 Screen Base and Size
            (0x210B, "BG12NBA"),  // BG1/2 Character Data Area Designation
            (0x210C, "BG34NBA"),  // BG3/4 Character Data Area Designation

            // PPU Scroll Registers ($2110-$2114)
            (0x210D, "BG1HOFS"),  // BG1 Horizontal Scroll
            (0x210E, "BG1VOFS"),  // BG1 Vertical Scroll
            (0x210F, "BG2HOFS"),  // BG2 Horizontal Scroll
            (0x2110, "BG2VOFS"),  // BG2 Vertical Scroll
            (0x2111, "BG3HOFS"),  // BG3 Horizontal Scroll
            (0x2112, "BG3VOFS"),  // BG3 Vertical Scroll
            (0x2113, "BG4HOFS"),  // BG4 Horizontal Scroll
            (0x2114, "BG4VOFS"),  // BG4 Vertical Scroll

            // PPU VRAM Registers ($2115-$2119)
            (0x2115, "VMAIN"),    // VRAM Address Increment
            (0x2116, "VMADDL"),   // VRAM Address Low
            (0x2117, "VMADDH"),   // VRAM Address High
            (0x2118, "VMDATAL"),  // VRAM Data Write Low
            (0x2119, "VMDATAH"),  // VRAM Data Write High

            // PPU Mode 7 Registers ($211A-$2120)
            (0x211A, "M7SEL"),    // Mode 7 Settings
            (0x211B, "M7A"),      // Mode 7 Matrix A
            (0x211C, "M7B"),      // Mode 7 Matrix B
            (0x211D, "M7C"),      // Mode 7 Matrix C
            (0x211E, "M7D"),      // Mode 7 Matrix D
            (0x211F, "M7X"),      // Mode 7 Center X
            (0x2120, "M7Y"),      // Mode 7 Center Y

            // PPU CGRAM Registers ($2121-$2122)
            (0x2121, "CGADD"),    // CGRAM Address
            (0x2122, "CGDATA"),   // CGRAM Data Write

            // PPU Window Registers ($2123-$212F)
            (0x2123, "W12SEL"),   // Window 1/2 Mask Settings for BG1/BG2
            (0x2124, "W34SEL"),   // Window 1/2 Mask Settings for BG3/BG4
            (0x2125, "WOBJSEL"),  // Window 1/2 Mask Settings for OBJ/MATH
            (0x2126, "WH0"),      // Window 1 Left Position
            (0x2127, "WH1"),      // Window 1 Right Position
            (0x2128, "WH2"),      // Window 2 Left Position
            (0x2129, "WH3"),      // Window 2 Right Position
            (0x212A, "WBGLOG"),   // Window Mask Logic for BG
            (0x212B, "WOBJLOG"),  // Window Mask Logic for OBJ
            (0x212C, "TM"),       // Main Screen Designation
            (0x212D, "TS"),       // Sub Screen Designation
            (0x212E, "TMW"),      // Window Mask Designation for Main Screen
            (0x212F, "TSW"),      // Window Mask Designation for Sub Screen

            // PPU Color Math Registers ($2130-$2132)
            (0x2130, "CGWSEL"),   // Color Math Control Register A
            (0x2131, "CGADSUB"),  // Color Math Control Register B
            (0x2132, "COLDATA"),  // Color Math Sub Screen Backdrop Color

            // PPU Status and Mode Registers ($2133-$213F)
            (0x2133, "SETINI"),   // Display Control 2
            (0x2134, "MPYL"),     // Multiplication Result Low
            (0x2135, "MPYM"),     // Multiplication Result Middle
            (0x2136, "MPYH"),     // Multiplication Result High
            (0x2137, "SLHV"),     // Software Latch for H/V Counter
            (0x2138, "OAMDATAREAD"), // OAM Data Read
            (0x2139, "VMDATALREAD"), // VRAM Data Read Low
            (0x213A, "VMDATAHREAD"), // VRAM Data Read High
            (0x213B, "CGDATAREAD"),  // CGRAM Data Read
            (0x213C, "OPHCT"),    // H Counter Read
            (0x213D, "OPVCT"),    // V Counter Read
            (0x213E, "STAT77"),   // PPU Status Flag and Version
            (0x213F, "STAT78"),   // PPU Status Flag and Version

            // APU Communication Registers ($2140-$2143)
            (0x2140, "APUIO0"),   // APU IO Port 0
            (0x2141, "APUIO1"),   // APU IO Port 1
            (0x2142, "APUIO2"),   // APU IO Port 2
            (0x2143, "APUIO3"),   // APU IO Port 3

            // WRAM Access Registers ($2180-$2183)
            (0x2180, "WMDATA"),   // WRAM Data Read/Write
            (0x2181, "WMADDL"),   // WRAM Address Low
            (0x2182, "WMADDM"),   // WRAM Address Middle
            (0x2183, "WMADDH"),   // WRAM Address High

            // CPU Control Registers ($4200-$420D)
            (0x4200, "NMITIMEN"), // Interrupt Enable and Joypad Request
            (0x4201, "WRIO"),     // Programmable I/O Port (Out)
            (0x4202, "WRMPYA"),   // Multiplicand
            (0x4203, "WRMPYB"),   // Multiplier
            (0x4204, "WRDIVL"),   // Dividend Low
            (0x4205, "WRDIVH"),   // Dividend High
            (0x4206, "WRDIVB"),   // Divisor
            (0x4207, "HTIMEL"),   // IRQ Timer Horizontal Counter Low
            (0x4208, "HTIMEH"),   // IRQ Timer Horizontal Counter High
            (0x4209, "VTIMEL"),   // IRQ Timer Vertical Counter Low
            (0x420A, "VTIMEH"),   // IRQ Timer Vertical Counter High
            (0x420B, "MDMAEN"),   // DMA Enable
            (0x420C, "HDMAEN"),   // HDMA Enable
            (0x420D, "MEMSEL"),   // ROM Access Speed

            // CPU Status Registers ($4210-$421F)
            (0x4210, "RDNMI"),    // NMI Flag and 5A22 Version
            (0x4211, "TIMEUP"),   // IRQ Flag
            (0x4212, "HVBJOY"),   // PPU Status
            (0x4213, "RDIO"),     // Programmable I/O Port (In)
            (0x4214, "RDDIVL"),   // Divide Result Low
            (0x4215, "RDDIVH"),   // Divide Result High
            (0x4216, "RDMPYL"),   // Multiply Result Low
            (0x4217, "RDMPYH"),   // Multiply Result High
            (0x4218, "JOY1L"),    // Controller Port 1 Data Low
            (0x4219, "JOY1H"),    // Controller Port 1 Data High
            (0x421A, "JOY2L"),    // Controller Port 2 Data Low
            (0x421B, "JOY2H"),    // Controller Port 2 Data High
            (0x421C, "JOY3L"),    // Controller Port 3 Data Low
            (0x421D, "JOY3H"),    // Controller Port 3 Data High
            (0x421E, "JOY4L"),    // Controller Port 4 Data Low
            (0x421F, "JOY4H"),    // Controller Port 4 Data High

            // DMA Control Registers ($43x0-$43xF, x=0-7)
            (0x4300, "DMAP0"),    // DMA0 Control
            (0x4301, "BBAD0"),    // DMA0 Destination
            (0x4302, "A1T0L"),    // DMA0 Source Address Low
            (0x4303, "A1T0H"),    // DMA0 Source Address High
            (0x4304, "A1B0"),     // DMA0 Source Bank
            (0x4305, "DAS0L"),    // DMA0 Size Low
            (0x4306, "DAS0H"),    // DMA0 Size High
            (0x4307, "DASB0"),    // DMA0 Bank for HDMA
            (0x4308, "A2A0L"),    // DMA0 HDMA Table Address Low
            (0x4309, "A2A0H"),    // DMA0 HDMA Table Address High
            (0x430A, "NTLR0"),    // DMA0 HDMA Line Counter
        ]
        .iter()
        .cloned()
        .collect()
    };
}
