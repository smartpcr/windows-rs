# windows-rs Module Capabilities

## Core Crates

### windows-core

**Purpose**: Foundation for all Windows-rs functionality, providing COM infrastructure and core types.

**Key Types**:

| Type | Description |
|------|-------------|
| `Interface` | Trait for all COM interfaces |
| `IUnknown` | Base COM interface with ref counting |
| `IInspectable` | Base WinRT interface |
| `GUID` | Globally unique identifier |
| `ComObject<T>` | Heap-allocated COM object wrapper |
| `InterfaceRef<'a, T>` | Borrowed interface (no AddRef) |
| `Weak<T>` | Weak reference to COM objects |
| `Ref<T>` | Immutable borrowed reference |
| `OutRef<T>` | Mutable output parameter |

**Capabilities**:
- COM interface definition and implementation
- Reference counting (AddRef/Release)
- Interface casting (QueryInterface)
- Dynamic type checking
- Weak reference support
- Agile reference for cross-apartment calls
- WinRT array support

**Example**:
```rust
use windows_core::{Interface, IUnknown, GUID};

// Cast between interfaces
let unknown: IUnknown = some_interface.cast()?;

// Get interface GUID
let iid: GUID = IMyInterface::IID;

// Downgrade to weak reference
let weak = strong_ref.downgrade()?;
```

---

### windows-strings

**Purpose**: Windows string types and conversions.

**Key Types**:

| Type | Description |
|------|-------------|
| `HSTRING` | Reference-counted UTF-16 string |
| `BSTR` | OLE Automation string |
| `PCWSTR` | Pointer to const wide string |
| `PWSTR` | Pointer to mutable wide string |
| `PCSTR` | Pointer to const ANSI string |
| `PSTR` | Pointer to mutable ANSI string |

**Macros**:
- `s!("text")` - Create ANSI string literal
- `w!("text")` - Create wide string literal
- `h!("text")` - Create HSTRING literal

**Capabilities**:
- UTF-16 encoding/decoding
- Conversion to/from Rust `String`
- Conversion to/from `OsString`
- Zero-copy string references
- Compile-time string literals

**Example**:
```rust
use windows::core::{HSTRING, w, s};

// Create strings
let h = HSTRING::from("Hello");
let wide = w!("Wide string");
let ansi = s!("ANSI string");

// Convert to Rust string
let rust_str = h.to_string_lossy();
```

---

### windows-result

**Purpose**: Windows error handling and result types.

**Key Types**:

| Type | Description |
|------|-------------|
| `Error` | Comprehensive error with optional IErrorInfo |
| `HRESULT` | 32-bit COM status code |
| `Result<T>` | `std::result::Result<T, Error>` |
| `WIN32_ERROR` | Win32-specific error code |
| `BOOL` | Win32 boolean type |
| `NTSTATUS` | NT status code |

**Capabilities**:
- HRESULT to Error conversion
- Win32 error code support
- Extended error info (IErrorInfo)
- Stack trace capture (debug)
- Niche optimization for Result

**Example**:
```rust
use windows::core::{Result, Error, HRESULT};

fn do_something() -> Result<()> {
    let hr = unsafe { SomeWindowsApi() };
    hr.ok()?;  // Convert HRESULT to Result
    Ok(())
}

// Get last Win32 error
let err = Error::from_win32();
```

---

### windows-registry

**Purpose**: Safe Windows registry access.

**Key Types**:

| Type | Description |
|------|-------------|
| `Key` | Registry key handle |
| `Value` | Registry value with type info |
| `Type` | Registry value type enum |
| `OpenOptions` | Builder for key access |
| `Transaction` | Atomic registry operations |

**Predefined Keys**:
- `CURRENT_USER`
- `LOCAL_MACHINE`
- `CLASSES_ROOT`
- `USERS`
- `CURRENT_CONFIG`

**Capabilities**:
- Create/open/delete keys
- Read/write typed values (u32, u64, string, binary)
- Enumerate keys and values
- Transactional operations
- Type-safe value access

**Example**:
```rust
use windows_registry::*;

// Read a value
let key = CURRENT_USER.open("Software\\MyApp")?;
let version: u32 = key.get_u32("Version")?;

// Write a value
key.set_string("Name", "MyApplication")?;

// Enumerate subkeys
for name in key.keys()? {
    println!("Subkey: {}", name?);
}
```

---

### windows-future

**Purpose**: Async/await support for WinRT async operations.

**Key Types**:

| Type | Description |
|------|-------------|
| `IAsyncAction` | Async operation returning unit |
| `IAsyncOperation<T>` | Async operation returning T |
| `IAsyncActionWithProgress<P>` | Async with progress |
| `IAsyncOperationWithProgress<T,P>` | Async with output and progress |

**Capabilities**:
- Bridge WinRT async to Rust futures
- Blocking wait with `join()`
- Callback completion with `when()`
- Progress reporting
- Cancellation support

**Example**:
```rust
use windows::Storage::StorageFile;

async fn read_file() -> Result<()> {
    let file = StorageFile::GetFileFromPathAsync(w!("C:\\file.txt"))?.await?;
    let content = FileIO::ReadTextAsync(&file)?.await?;
    println!("{}", content);
    Ok(())
}
```

---

### windows-version

**Purpose**: Windows version detection.

**Capabilities**:
- Query Windows version at runtime
- Check for specific feature availability
- OS capability detection

**Example**:
```rust
use windows_version::OsVersion;

let version = OsVersion::current();
if version >= OsVersion::new(10, 0, 19041, 0) {
    // Windows 10 2004 or later
}
```

---

## Code Generation Crates

### windows-metadata

**Purpose**: Low-level ECMA-335 metadata parsing.

**Key Types**:

| Type | Description |
|------|-------------|
| `TypeIndex` | Main lookup structure |
| `TypeDef` | Type definition |
| `MethodDef` | Method definition |
| `Field` | Field definition |
| `Signature` | Method signature |
| `Type` | Type representation |

**Capabilities**:
- Parse .winmd files (PE format)
- Read metadata tables
- Resolve type references
- Parse method signatures
- Handle generic types

**Example**:
```rust
use windows_metadata::reader::TypeIndex;

let index = TypeIndex::read("Windows.winmd")?;
let typedef = index.expect("Windows.Foundation", "Point");
for field in typedef.fields() {
    println!("{}: {:?}", field.name(), field.ty());
}
```

---

### windows-bindgen

**Purpose**: Generate Rust code from Windows metadata.

**Arguments**:

| Argument | Description |
|----------|-------------|
| `--in` | Input .winmd files |
| `--out` | Output .rs file |
| `--filter` | Type/namespace filters |
| `--flat` | Flat output (no modules) |
| `--sys` | Generate sys-style bindings |
| `--implement` | Include impl traits |

**Capabilities**:
- Generate structs, enums, functions
- Generate COM interfaces
- Generate WinRT classes
- Handle generics
- Resolve dependencies
- Apply custom derives

**Example** (build.rs):
```rust
fn main() {
    windows_bindgen::bindgen([
        "--out", "src/bindings.rs",
        "--filter", "Windows.Win32.Foundation.GetTickCount",
        "--filter", "Windows.Win32.System.Threading.Sleep",
    ]).unwrap();
}
```

---

### windows-implement

**Purpose**: Procedural macro for implementing COM interfaces.

**Macro**: `#[implement(Interface1, Interface2, ...)]`

**Capabilities**:
- Generate COM wrapper type
- Set up vtables automatically
- Handle multiple interfaces
- Reference counting
- Agile object support

**Example**:
```rust
#[implement(IValue)]
struct MyValue(i32);

impl IValue_Impl for MyValue {
    unsafe fn GetValue(&self, value: *mut i32) -> HRESULT {
        *value = self.0;
        S_OK
    }
}

let obj: IValue = MyValue(42).into();
```

---

### windows-interface

**Purpose**: Procedural macro for defining COM interfaces.

**Macro**: `#[interface("GUID")]`

**Capabilities**:
- Define interface trait
- Generate vtable structure
- Set up GUID association
- Handle inheritance

**Example**:
```rust
#[interface("094d70d6-5202-44b8-abb8-43860da5aca2")]
unsafe trait IMyInterface: IUnknown {
    fn DoSomething(&self) -> HRESULT;
    fn GetValue(&self, value: *mut i32) -> HRESULT;
}
```

---

## Utility Crates

### windows-targets

**Purpose**: Platform-specific import libraries.

**Supported Targets**:
- `x86_64-pc-windows-msvc`
- `x86_64-pc-windows-gnu`
- `x86_64-pc-windows-gnullvm`
- `i686-pc-windows-msvc`
- `i686-pc-windows-gnu`
- `aarch64-pc-windows-msvc`
- `aarch64-pc-windows-gnullvm`

Contains pre-compiled `.lib` files for linking Windows APIs.

---

### windows-link

**Purpose**: Linking infrastructure.

**Macro**: `link!("dll" "conv" fn name(args) -> ret)`

Generates extern function declarations with proper linking attributes.

---

### cppwinrt

**Purpose**: C++/WinRT compiler bundle.

Contains the C++/WinRT compiler for generating C++ projections, used for interop scenarios.
