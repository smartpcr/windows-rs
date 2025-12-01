# windows-rs Architecture

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         User Application                                 │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                    ┌───────────────┴───────────────┐
                    ▼                               ▼
        ┌───────────────────┐           ┌───────────────────┐
        │     windows       │           │   windows-sys     │
        │  (safe bindings)  │           │  (raw bindings)   │
        └───────────────────┘           └───────────────────┘
                    │                               │
                    └───────────────┬───────────────┘
                                    ▼
        ┌───────────────────────────────────────────────────┐
        │                  windows-core                      │
        │  (COM types, interfaces, error handling, strings) │
        └───────────────────────────────────────────────────┘
                                    │
        ┌───────────────┬───────────┼───────────┬───────────┐
        ▼               ▼           ▼           ▼           ▼
   ┌─────────┐   ┌───────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐
   │strings  │   │  result   │ │ future  │ │implement│ │interface│
   └─────────┘   └───────────┘ └─────────┘ └─────────┘ └─────────┘
                                    │
                                    ▼
        ┌───────────────────────────────────────────────────┐
        │                 windows-targets                    │
        │  (platform-specific import libraries)              │
        └───────────────────────────────────────────────────┘
```

## Code Generation Pipeline

```
┌─────────────────┐
│ .winmd Files    │  Windows metadata (ECMA-335 format)
│ - Windows.winmd │  ~31MB bundled with windows-bindgen
│ - Win32.winmd   │
│ - Wdk.winmd     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│windows-metadata │  Low-level ECMA-335 parser
│                 │  - PE header parsing
│                 │  - Metadata tables
│                 │  - Type resolution
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│windows-bindgen  │  Code generator
│                 │  - Type filtering
│                 │  - Dependency resolution
│                 │  - Rust code emission
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Generated .rs   │  Final Rust bindings
│ - Structs       │  - Type definitions
│ - Enums         │  - Function declarations
│ - Interfaces    │  - Constants
│ - Functions     │
└─────────────────┘
```

## Crate Dependency Graph

```
                        windows / windows-sys
                               │
                               ▼
                         windows-core
                               │
        ┌──────────────────────┼──────────────────────┐
        │          │           │           │          │
        ▼          ▼           ▼           ▼          ▼
   windows-   windows-   windows-   windows-   windows-
   strings    result     future     implement  interface
        │          │           │
        └──────────┴───────────┘
                   │
                   ▼
            windows-link
                   │
                   ▼
            windows-targets
                   │
    ┌──────────────┼──────────────┐
    ▼              ▼              ▼
 x86_64_msvc   i686_gnu    aarch64_msvc
   (etc.)       (etc.)       (etc.)
```

## Component Responsibilities

### Core Layer

| Component | Responsibility |
|-----------|---------------|
| `windows-core` | COM interface traits, IUnknown, GUID, reference counting |
| `windows-strings` | HSTRING, BSTR, PCWSTR, string macros (s!, w!) |
| `windows-result` | HRESULT, Error, Result<T> types |
| `windows-link` | Platform linking infrastructure |
| `windows-targets` | Pre-compiled import libraries per architecture |

### Abstraction Layer

| Component | Responsibility |
|-----------|---------------|
| `windows` | Safe, ergonomic API wrappers |
| `windows-sys` | Raw function declarations, structs, constants |
| `windows-future` | Async/await integration for WinRT async |
| `windows-registry` | High-level registry access |
| `windows-version` | Windows version detection |

### Code Generation Layer

| Component | Responsibility |
|-----------|---------------|
| `windows-metadata` | Parse .winmd files (ECMA-335 format) |
| `windows-bindgen` | Generate Rust code from metadata |
| `windows-implement` | #[implement] macro for COM classes |
| `windows-interface` | #[interface] macro for COM interfaces |

## Memory Model

### COM Reference Counting

```
┌─────────────────────────────────────────┐
│              ComObject<T>               │
│  ┌─────────────────────────────────┐   │
│  │  T_Impl (generated wrapper)     │   │
│  │  ┌────────────────────────────┐ │   │
│  │  │  RefCount (atomic i32)     │ │   │
│  │  │  identity: IUnknown vtbl   │ │   │
│  │  │  interface_vtbls: ...      │ │   │
│  │  │  inner: T (user type)      │ │   │
│  │  └────────────────────────────┘ │   │
│  └─────────────────────────────────┘   │
└─────────────────────────────────────────┘

Clone → AddRef (increment)
Drop  → Release (decrement, free when zero)
```

### String Types

```
HSTRING (immutable, reference-counted UTF-16)
┌──────────────────────────────────────┐
│ HStringHeader                        │
│ ┌──────────────────────────────────┐ │
│ │ flags: u32                       │ │
│ │ len: u32                         │ │
│ │ padding: u32                     │ │
│ │ ref_count: AtomicI32             │ │
│ │ data: [u16; len]                 │ │
│ └──────────────────────────────────┘ │
└──────────────────────────────────────┘

BSTR (length-prefixed, OLE Automation)
┌───────────┬────────────────────────┐
│ len (u32) │ UTF-16 data + null     │
└───────────┴────────────────────────┘
    ↑
  pointer points here (after length prefix)
```

## Error Handling Architecture

```
Windows API call returns HRESULT (i32)
            │
            ▼
    ┌───────────────┐
    │ Success check │ HRESULT >= 0
    │   (SUCCEEDED) │
    └───────┬───────┘
            │ Failure
            ▼
    ┌───────────────┐
    │ Create Error  │ Contains:
    │               │ - HRESULT code
    │               │ - IErrorInfo (optional)
    │               │ - Debug info
    └───────────────┘
            │
            ▼
    Result<T, Error> → Rust ? operator
```

## Interface Hierarchy

```
IUnknown (base of all COM interfaces)
├── QueryInterface(iid) → *mut c_void
├── AddRef() → u32
└── Release() → u32
    │
    ▼
IInspectable (base of WinRT interfaces)
├── GetIids() → [GUID]
├── GetRuntimeClassName() → HSTRING
└── GetTrustLevel() → i32
    │
    ▼
User Interfaces (IAsyncAction, IVector<T>, etc.)
```

## Build-Time vs Runtime

### Build-Time (bindgen)
- Metadata parsing
- Type filtering
- Rust code generation
- Feature gate generation

### Runtime
- Dynamic linking to Windows DLLs
- COM reference counting
- Interface casting (QueryInterface)
- Error propagation

## Feature System

Features are hierarchically organized:

```
Windows.Foundation           → Foundation types
Windows.Foundation.Collections → Collection interfaces
Win32_Foundation            → Win32 base types
Win32_System_Threading      → Threading APIs
Win32_UI_WindowsAndMessaging → Window/message APIs
```

Each feature:
- Declares API subset dependency
- Triggers conditional compilation
- Controls code generation scope
