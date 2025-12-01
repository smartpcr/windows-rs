# windows-rs Overview

## Introduction

**windows-rs** is Microsoft's official Rust language projection for Windows APIs. It provides Rust bindings to the complete Windows API surface, including Win32, WinRT, COM, and WDK (Windows Driver Kit) APIs.

The project follows the tradition established by [C++/WinRT](https://github.com/microsoft/cppwinrt), using automatic code generation from official Windows metadata to create idiomatic Rust bindings.

## Key Features

- **Complete API Coverage**: Access to all Windows APIs (past, present, and future)
- **Zero-Cost Abstractions**: Minimal runtime overhead with safe Rust wrappers
- **Code Generation**: Bindings generated on-the-fly from Windows metadata
- **Dual Crate Approach**: `windows` (safe) and `windows-sys` (raw) options
- **Feature-Based Selection**: Only include APIs you need via Cargo features
- **Cross-Architecture Support**: x86, x86_64, and ARM64 targets

## Repository Structure

```
windows-rs/
├── crates/
│   ├── libs/           # Core library crates (20 crates)
│   ├── samples/        # Example projects
│   ├── targets/        # Platform-specific import libraries
│   ├── tests/          # Test suites
│   └── tools/          # Build and generation tools
├── docs/               # Documentation
└── web/                # Web resources (feature search, book)
```

## Main Crates

| Crate | Purpose | Version |
|-------|---------|---------|
| `windows` | Safe Windows API bindings | 0.62.x |
| `windows-sys` | Raw/unsafe Windows API bindings | 0.61.x |
| `windows-core` | Core types and COM infrastructure | 0.62.x |
| `windows-bindgen` | Code generator from metadata | 0.65.x |
| `windows-metadata` | ECMA-335 metadata parser | 0.59.x |

## Quick Start

Add to `Cargo.toml`:

```toml
[dependencies.windows]
version = ">=0.59, <=0.62"
features = [
    "Win32_Foundation",
    "Win32_System_Threading",
    "Win32_UI_WindowsAndMessaging",
]
```

Basic usage:

```rust
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    unsafe {
        let event = CreateEventW(None, true, false, None)?;
        SetEvent(event)?;
        WaitForSingleObject(event, 0);
        CloseHandle(event)?;

        MessageBoxW(None, w!("Hello"), w!("Title"), MB_OK);
    }
    Ok(())
}
```

## Choosing Between `windows` and `windows-sys`

### Use `windows` when:
- Building applications with complex Windows API usage
- You want safe, ergonomic Rust abstractions
- Working with COM/WinRT interfaces
- Error handling with proper Rust `Result<T>`

### Use `windows-sys` when:
- Compile time is critical (zero-overhead)
- You only need function declarations, structs, and constants
- Building low-level system components
- You're comfortable with raw/unsafe code

## Resources

- [Getting Started Guide](https://kennykerr.ca/rust-getting-started/)
- [Feature Search](https://microsoft.github.io/windows-rs/features)
- [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
- [Releases](https://github.com/microsoft/windows-rs/releases)
