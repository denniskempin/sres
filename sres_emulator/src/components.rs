//! Independent components of the emulator
//!
//! Components are organized around these guidelines to ensure they remain independent qnd provide a clean API:
//! - Components can only depend on code in common/ or util/
//! - Components cannot depend on one another other
//! - All modules inside a component must be private. Re-export types that need to be accessible.
//! - Use self/super to refer to inner modules
//! - Do not use super to refer to outer modules

pub mod apu;
