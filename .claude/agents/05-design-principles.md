# windows-rs Design Principles

## Core Philosophy

windows-rs follows the design philosophy established by C++/WinRT: use standard language features and compilers to create natural, idiomatic bindings from Windows metadata.

## Design Principles

### 1. Zero-Cost Abstractions

The library provides high-level abstractions without runtime overhead:

```rust
// This safe wrapper...
let result = CreateEventW(None, true, false, None)?;

// ...compiles to the same code as:
extern "system" { fn CreateEventW(...) -> HANDLE; }
let result = CreateEventW(ptr::null(), 1, 0, ptr::null());
```

**Implementation**:
- `#[repr(transparent)]` wrappers for all handle types
- Inline function wrappers that optimize away
- Direct vtable calls for COM interfaces
- No runtime reflection or boxing

### 2. Type Safety

Windows APIs are strongly typed in Rust:

```rust
// Handles are distinct types
fn process_event(event: HANDLE) { }  // Won't accept HWND by mistake

// Enums prevent invalid values
let flags = FILE_ATTRIBUTE_READONLY | FILE_ATTRIBUTE_HIDDEN;

// COM interfaces are type-checked
let unknown: IUnknown = obj.cast()?;  // Compile-time interface verification
```

**Implementation**:
- Newtype wrappers for handles (HANDLE, HWND, HDC, etc.)
- `#[repr(transparent)]` enums for flags
- Trait bounds for interface casting

### 3. Memory Safety

Reference counting and lifetimes prevent use-after-free:

```rust
// COM objects are reference counted
let obj: IMyInterface = create_object();
let clone = obj.clone();  // AddRef called
drop(obj);               // Release called
// clone still valid

// Borrowed references track lifetimes
let borrowed: InterfaceRef<'_, IUnknown> = obj.to_ref();
// borrowed cannot outlive obj
```

**Implementation**:
- `Clone` calls `AddRef`
- `Drop` calls `Release`
- `InterfaceRef<'a, T>` for borrowed pointers
- `Weak<T>` for weak references

### 4. Minimal Dependencies

Each crate is focused and independent:

```
windows-result: 0 dependencies (no-std)
windows-strings: 0 dependencies (no-std)
windows-core: windows-result + windows-strings
windows: windows-core + generated bindings
```

**Benefits**:
- Faster compilation
- Smaller binaries
- No-std support where possible

### 5. Feature-Based Compilation

Only compile what you use:

```toml
[dependencies.windows]
features = [
    "Win32_Foundation",      # ~100 types
    "Win32_System_Threading", # ~50 functions
]
# Not "Win32" which would include everything
```

**Implementation**:
- Cargo features map to Windows namespaces
- Hierarchical feature dependencies
- Conditional compilation via `#[cfg(feature = "...")]`

### 6. Idiomatic Rust

APIs feel natural to Rust developers:

```rust
// Result-based error handling
let file = OpenFile(path)?;

// Iterator support
for key in registry_key.keys()? {
    process(key?);
}

// Async/await integration
let data = ReadFileAsync(path)?.await?;

// Standard traits
let guid = GUID::from_u128(0x12345678...);
println!("{:?}", guid);  // Debug
```

### 7. Dual-Crate Strategy

Two crates serve different needs:

| Aspect | `windows` | `windows-sys` |
|--------|-----------|---------------|
| Safety | Safe wrappers | Raw/unsafe |
| Overhead | Minimal | Zero |
| Compile time | Moderate | Fast |
| Ergonomics | High | Low |
| Use case | Applications | Libraries/FFI |

---

## Architectural Decisions

### Metadata-Driven Generation

**Decision**: Generate code from .winmd files at build time.

**Rationale**:
- Single source of truth (Microsoft's metadata)
- Automatic updates when APIs change
- Complete API coverage guaranteed
- Consistent naming and structure

**Trade-offs**:
- Larger initial build time
- Complex build tooling
- Version synchronization requirements

### COM as Foundation

**Decision**: Use COM patterns as the base abstraction.

**Rationale**:
- Windows APIs are fundamentally COM-based
- WinRT is COM with additional conventions
- Enables interface composition
- Supports binary compatibility

**Implementation**:
```rust
// All interfaces derive from IUnknown
unsafe trait Interface: Sized {
    const IID: GUID;
    fn vtable(&self) -> &Self::Vtable;
}
```

### Reference Counting Strategy

**Decision**: Atomic reference counting with ownership semantics.

**Rationale**:
- Matches Windows COM model
- Thread-safe by default
- Compatible with Rust ownership

**Implementation**:
```rust
pub struct RefCount(AtomicI32);

impl RefCount {
    pub fn add_ref(&self) -> u32 {
        (self.0.fetch_add(1, Ordering::Relaxed) + 1) as u32
    }

    pub fn release(&self) -> u32 {
        let remaining = self.0.fetch_sub(1, Ordering::Release) - 1;
        if remaining == 0 {
            fence(Ordering::Acquire);
        }
        remaining as u32
    }
}
```

### Error Model

**Decision**: Unified `Error` type wrapping HRESULT with optional extended info.

**Rationale**:
- Single error type for all Windows errors
- Preserves original error codes
- Supports rich error information
- Compatible with `?` operator

**Trade-off**: Optional slim mode sacrifices details for size:
```rust
// Normal: 24+ bytes with IErrorInfo
// Slim: 4 bytes (just HRESULT)
#[cfg(windows_slim_errors)]
```

### String Handling

**Decision**: Multiple string types matching Windows conventions.

**Rationale**:
- HSTRING for WinRT (reference counted, immutable)
- BSTR for OLE Automation (length-prefixed)
- PCWSTR/PWSTR for Win32 (null-terminated)
- Zero-copy where possible

**Implementation**:
```rust
// Compile-time literals avoid allocation
let s = w!("Hello");  // &'static PCWSTR

// Runtime strings use appropriate type
let h = HSTRING::from("Dynamic");
```

---

## Evolution Guidelines

### Adding New Features

1. **Maintain backward compatibility** where possible
2. **Use feature flags** for optional functionality
3. **Prefer composition** over inheritance
4. **Document breaking changes** clearly

### Version Policy

- Major: Breaking API changes
- Minor: New features, deprecations
- Patch: Bug fixes only

### Crate Versioning

Related crates maintain compatible versions:
```toml
windows-core = "0.62"
windows-strings = "0.5"
windows-result = "0.4"
# All compatible with windows = "0.62"
```

---

## Code Style

### Naming Conventions

| Windows | Rust |
|---------|------|
| `IFoo` | `IFoo` (interfaces kept as-is) |
| `FOO_BAR` | `FOO_BAR` (constants kept as-is) |
| `FooBar` | `FooBar` (types kept as-is) |
| `lpszFoo` | `foo` (parameter names simplified) |

### Documentation

- All public items documented
- Examples in doc comments
- Links to Microsoft documentation

### Safety

- Mark functions `unsafe` when they can cause UB
- Document safety requirements
- Prefer safe abstractions where possible

```rust
/// Creates an event object.
///
/// # Safety
/// The caller must ensure the returned handle is closed with `CloseHandle`.
pub unsafe fn CreateEventW(...) -> Result<HANDLE>
```
