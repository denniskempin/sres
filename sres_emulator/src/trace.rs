//! Implements the trace log format used by BSNES to compare the emulator to BSNES.
//!
//! Also a useful, compact format for debugging emulator execution.
use std::fmt::Display;
use std::str::FromStr;

use anyhow::Result;

use crate::components::cpu::CpuTraceLine;
use crate::components::spc700::Spc700TraceLine;

#[allow(dead_code)]
pub enum TraceLine {
    Cpu(CpuTraceLine),
    Spc700(Spc700TraceLine),
}

impl FromStr for TraceLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with("..") {
            Ok(TraceLine::Spc700(s.parse()?))
        } else {
            Ok(TraceLine::Cpu(s.parse()?))
        }
    }
}

impl Display for TraceLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TraceLine::Cpu(line) => write!(f, "{}", line),
            TraceLine::Spc700(line) => write!(f, "{}", line),
        }
    }
}
