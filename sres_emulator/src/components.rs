//! Independent components of the emulator
//!
//! Components are organized around these guidelines to ensure they remain independent qnd provide a clean API:
//! - Components can only depend on code in common/ or util/
//! - Only one-way dependencies between components are allowed, if they are required to work together.
//! - All modules inside a component must be private. Re-export types that need to be accessible.
//! - Use self/super to refer to inner modules
//! - Do not use super to refer to outer modules

pub mod apu;
pub mod spc700;
