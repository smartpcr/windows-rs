# windows-rs Documentation

This directory contains comprehensive documentation for the windows-rs project.

## Documents

| Document | Description |
|----------|-------------|
| [01-overview.md](01-overview.md) | Project introduction and quick start |
| [02-architecture.md](02-architecture.md) | System architecture and component design |
| [03-modules.md](03-modules.md) | Detailed capability documentation for each crate |
| [04-usage-guide.md](04-usage-guide.md) | Practical usage patterns and examples |
| [05-design-principles.md](05-design-principles.md) | Design philosophy and architectural decisions |
| [06-code-generation.md](06-code-generation.md) | Metadata parsing and code generation system |

## Quick Navigation

### For New Users
Start with [Overview](01-overview.md) then [Usage Guide](04-usage-guide.md).

### For Contributors
Read [Architecture](02-architecture.md) and [Design Principles](05-design-principles.md).

### For Understanding Internals
See [Code Generation](06-code-generation.md) and [Modules](03-modules.md).

## Crate Summary

```
User-Facing Crates:
├── windows          - Safe Windows API bindings
├── windows-sys      - Raw/unsafe bindings (fast compile)
├── windows-registry - Registry access
└── windows-version  - Version detection

Core Infrastructure:
├── windows-core     - COM types, interfaces, errors
├── windows-strings  - String types (HSTRING, BSTR)
├── windows-result   - Error handling
├── windows-future   - Async support
└── windows-link     - Linking infrastructure

Code Generation:
├── windows-metadata - ECMA-335 parser
├── windows-bindgen  - Code generator
├── windows-implement - #[implement] macro
└── windows-interface - #[interface] macro

Platform Support:
├── windows-targets  - Import library selection
└── targets/*        - Per-architecture import libs
```

## External Resources

- [Getting Started](https://kennykerr.ca/rust-getting-started/)
- [Feature Search](https://microsoft.github.io/windows-rs/features)
- [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
- [Releases](https://github.com/microsoft/windows-rs/releases)
- [GitHub Issues](https://github.com/microsoft/windows-rs/issues)
