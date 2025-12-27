# Code Style and Conventions

## Formatting
The project uses `rustfmt` with a custom configuration defined in `rustfmt.toml`:
```toml
unstable_features = true
imports_granularity = "Item"
group_imports = "StdExternalCrate"
```

### Import Organization
- Imports are grouped into: std library, external crates, and local crates
- Each import item is on its own line (imports_granularity = "Item")
- Example:
  ```rust
  use std::collections::HashMap;
  use std::sync::Arc;
  
  use anyhow::Result;
  use log::info;
  
  use crate::components::cpu::Cpu;
  ```

## Linting
- Uses `cargo clippy` for linting
- All clippy warnings should be addressed before committing

## Rust Edition and Version
- Edition: 2021
- Minimum Rust version: 1.72

## Naming Conventions
- Follow standard Rust naming conventions:
  - `snake_case` for functions, variables, modules
  - `PascalCase` for types, structs, enums
  - `SCREAMING_SNAKE_CASE` for constants
  - `'lowercase` for lifetimes

## Code Organization
- Each hardware component is implemented as a separate module
- Use clear interfaces for component communication
- Support debug tracing for all components
- Implement serialization/deserialization for save state support

## Error Handling
- Use `anyhow::Result` for error handling throughout the codebase
- Provide context to errors where appropriate

## Logging
- Use the `log` crate for logging (info!, debug!, warn!, error!)
- The `env_logger` is used for log output with colored support
- Debug logging can be enabled with the `debug_log` feature flag

## Testing
- ROM-based integration testing is preferred
- PPU tests compare rendered output against reference images
- APU tests verify audio sample generation
- CPU tests validate instruction execution with trace comparison
