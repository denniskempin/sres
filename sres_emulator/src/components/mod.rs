//! Independent components of the emulator
//!
//! These are components of the emulator that are largely independent of each other and could be
//! re-used separately.
//!
//! To ensure they remain independent and provide a clean API, the following rules are applied:
//! - Components cannot depend on one another
//! - Components can only import code from common/ or util/
//! - Keep exported types and functionality to a minimum
//! - All modules inside a component must be private
//! - Use self/super to refer to inner modules
//! - Do not use super to refer to outer modules

pub mod ppu;
pub mod s_dsp;
pub mod spc700;
