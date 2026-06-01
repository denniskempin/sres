# Fuzzing Summary for `sres_emulator`

This document describes the fuzzing setup for the `sres_emulator` crate, located at `sres_emulator/fuzz/`.

## Overview

This is a standard [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz) project using `libfuzzer-sys`. It is configured as a separate workspace crate to avoid interfering with the main project's workspace.

## Project Configuration (`Cargo.toml`)

- **Package name**: `sres_emulator-fuzz`
- **Edition**: 2021
- **Fuzzer**: `libfuzzer-sys = "0.4"`
- **Dependency**: `sres_emulator` (path = `".."`, the parent crate)
- **Workspace**: Isolated (`members = ["."]`)
- **Profile**: `release` profile has `debug = 1` for better crash diagnostics

## Fuzz Targets

There are two fuzz targets defined in `fuzz_targets/`:

### 1. `program`
- **File**: `fuzz_targets/program.rs`
- **What it fuzzes**: The CPU emulator's ability to execute arbitrary byte sequences as 65816 machine code.
- **Behavior**:
  - Takes raw fuzzer bytes and loads them into memory via `SresBus::with_program(data)`.
  - Creates a `Cpu` and executes up to **1000 steps**.
  - The comment in the source explicitly states: *"This can fail in all kinds of ways, but it should never ever panic!"*
- **Goal**: Detect panics, infinite loops, or memory safety issues caused by invalid/emulated CPU instructions.

### 2. `sfc`
- **File**: `fuzz_targets/sfc.rs`
- **What it fuzzes**: The cartridge/SFC ROM loader.
- **Behavior**:
  - Takes raw fuzzer bytes and passes them to `Cartridge::new().load_sfc_data(data)`.
  - The result is discarded (`let _ = ...`).
  - The comment states: *"This will likely fail, but should never panic!"*
- **Goal**: Ensure the ROM parser is robust against malformed or malicious SFC data and does not panic.

## Corpus

The `corpus/` directory contains seed inputs for the fuzzer.

| Target | Location | Count | Notes |
|--------|----------|-------|-------|
| `program` | `corpus/program/` | ~3,957 files | Files are named by SHA1 hash. These are accumulated inputs from previous fuzzing runs. |
| `sfc` | `corpus/sfc/` | 0 files | No seed corpus exists yet for the cartridge loader. |

## Artifacts (Crashes)

The `artifacts/` directory contains inputs that caused crashes.

| Target | File | Size | Notes |
|--------|------|------|-------|
| `program` | `crash-385ca8e41dc9b8d818b1cea37aebc861c8507cd7` | 2 bytes (`0x98 0x9f`) | A minimal 2-byte input triggering a crash. |
| `program` | `crash-da39a3ee5e6b4b0d3255bfef95601890afd80709` | 0 bytes | Empty input triggers a crash. |
| `sfc` | `crash-da39a3ee5e6b4b0d3255bfef95601890afd80709` | 0 bytes | Empty input triggers a crash in cartridge loading. |

> **Note**: The `da39a3ee...` hash is the SHA1 of an empty string, confirming these are empty-file crash reproducers.

## How to Run Fuzzing

Prerequisites: Install `cargo-fuzz`:
```bash
cargo install cargo-fuzz
```

Run from the `sres_emulator/fuzz` directory (or from the crate root with `cargo fuzz --manifest-path fuzz/Cargo.toml ...`):

### Run the `program` fuzzer
```bash
cargo fuzz run program
```

### Run the `sfc` fuzzer
```bash
cargo fuzz run sfc
```

### Run with existing corpus
```bash
cargo fuzz run program -- corpus/program
cargo fuzz run sfc -- corpus/sfc
```

### Reproduce a specific crash
```bash
cargo fuzz run program -- artifacts/program/crash-385ca8e41dc9b8d818b1cea37aebc861c8507cd7
```

## Git Ignore

The following directories are ignored by git (see `.gitignore`):
- `target/`
- `corpus/`
- `artifacts/`
- `coverage/`

This means corpus and crash artifacts are **not** committed to the repository by default.

## Tips for Future Agents

- **Adding a new target**: Create a new `.rs` file in `fuzz_targets/` and add a corresponding `[[bin]]` entry in `Cargo.toml`.
- **Corpus minimization**: Use `cargo fuzz cmin <target>` to reduce the size of the corpus while preserving coverage.
- **Crash triage**: Use `cargo fuzz run <target> -- <artifact_path>` to reproduce crashes. The `debug = 1` release profile helps with backtraces.
- **Empty inputs**: Both targets currently crash on empty input. This may be expected behavior (the fuzzer found a legitimate bug) or may indicate missing input validation.
