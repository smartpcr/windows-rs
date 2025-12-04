# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is **windows-rs**, Microsoft's official Rust bindings for the Windows API. It provides two main crates:
- `windows` - Safer bindings for C-style, COM, and WinRT APIs
- `windows-sys` - Raw bindings for C-style Windows APIs (lighter weight, lower MSRV)

The bindings are generated from Windows metadata (`.winmd` files) using the `windows-bindgen` code generator.

## Build Commands

```bash
# Build the entire workspace
cargo build

# Run all tests for a specific package
cargo test -p windows-core

# Run clippy on the workspace (uses nightly)
cargo clippy -p <package_name> --tests

# Format code (newline_style = Unix)
cargo fmt --all
```

## Code Generation

The crates contain generated code that must be kept in sync. After modifying metadata or bindgen logic:

```bash
# Regenerate bindings for all crates (windows, windows-sys, etc.)
cargo run -p tool_bindings --release

# Regenerate GitHub workflow YAML files
cargo run -p tool_yml

# Regenerate workspace configuration
cargo run -p tool_workspace

# Regenerate license headers
cargo run -p tool_license

# Generate standalone bindings
cargo run -p tool_standalone
```

CI verifies generated code is up-to-date by checking for git diff after running these tools.

## Architecture

### Core Library Crates (`crates/libs/`)

- **windows-core** - Core COM/WinRT type support (IUnknown, GUID, HRESULT, etc.)
- **windows-result** - Error handling (HRESULT, Error type)
- **windows-strings** - String types (HSTRING, PCWSTR, BSTR)
- **windows-metadata** - Low-level ECMA-335 metadata parser
- **windows-bindgen** - Code generator that reads metadata and emits Rust
- **windows-implement** - `#[implement]` proc-macro for COM interface implementation
- **windows-interface** - `#[interface]` proc-macro for COM interface definition
- **windows** - Full Windows API bindings (uses feature flags for namespaces)
- **windows-sys** - Raw FFI bindings (lighter weight alternative)

### Tool Crates (`crates/tools/`)

- **tool_bindings** - Runs bindgen to regenerate all crate bindings
- **tool_yml** - Generates CI workflow YAML from workspace info
- **tool_gnu** - Builds import libraries for GNU targets
- **tool_msvc** - Builds import libraries for MSVC targets

### Target Crates (`crates/targets/`)

Import library bundles for each target triple (aarch64_msvc, x86_64_gnu, etc.).

### Test Crates (`crates/tests/`)

- `crates/tests/libs/` - Tests for library crates
- `crates/tests/misc/` - Miscellaneous integration tests
- `crates/tests/winrt/` - WinRT-specific tests

## Feature Flag Hierarchy

The `windows` and `windows-sys` crates use hierarchical feature flags matching the Windows API namespace structure:

```toml
# Example: enabling Win32 UI controls
windows = { features = ["Win32_UI_Controls"] }
```

Features automatically enable their parent namespaces (e.g., `Win32_UI_Controls` enables `Win32_UI` and `Win32`).

## Key Patterns

### COM Interface Implementation
```rust
#[implement(IMyInterface)]
struct MyStruct { ... }
```

### COM Interface Definition
```rust
#[interface("GUID-HERE")]
unsafe trait IMyInterface: IUnknown { ... }
```

### Windows API Calls
The `windows` crate provides safe wrappers. The `windows-sys` crate provides raw FFI signatures requiring `unsafe`.

## Testing Notes

- Tests require Windows to run (most are skipped on other platforms)
- Some tests require specific Windows features or elevated privileges
- CI tests across x86_64, i686, and aarch64 with both MSVC and GNU toolchains
