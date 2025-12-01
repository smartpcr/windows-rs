# Code Generation System

## Overview

windows-rs uses a sophisticated code generation pipeline to create Rust bindings from Windows metadata (.winmd) files. This document describes how the system works.

## Metadata Format

### ECMA-335

Windows metadata uses the ECMA-335 (CLI) format, the same binary format used by .NET assemblies:

```
┌─────────────────────────────────────┐
│ DOS Header                          │
├─────────────────────────────────────┤
│ PE Header                           │
├─────────────────────────────────────┤
│ CLR Header                          │
├─────────────────────────────────────┤
│ Metadata Header                     │
├─────────────────────────────────────┤
│ Streams:                            │
│   #~ (Tables)                       │
│   #Strings (String heap)            │
│   #Blob (Binary data)               │
│   #GUID (GUID heap)                 │
└─────────────────────────────────────┘
```

### Metadata Tables

Key tables used by windows-rs:

| Table | Index | Purpose |
|-------|-------|---------|
| TypeDef | 0x02 | Type definitions |
| TypeRef | 0x01 | External type references |
| MethodDef | 0x06 | Method definitions |
| Field | 0x04 | Field definitions |
| Param | 0x08 | Method parameters |
| Attribute | 0x0C | Custom attributes |
| InterfaceImpl | 0x09 | Interface implementations |
| Constant | 0x0B | Constant values |

### Type Categories

Types are categorized by their base class:

```rust
pub enum TypeCategory {
    Interface,   // No base class
    Class,       // Generic class
    Enum,        // Extends System.Enum
    Struct,      // Extends System.ValueType
    Delegate,    // Extends System.MulticastDelegate
    Attribute,   // Extends System.Attribute (skipped)
}
```

---

## Generation Pipeline

### Stage 1: Metadata Reading

```rust
// windows-metadata crate
let index = TypeIndex::read("Windows.winmd")?;
```

The `TypeIndex` provides fast lookups by namespace and name:

```rust
// Lookup structure
HashMap<String, HashMap<String, Vec<(usize, usize)>>>
//      namespace      name        (file, row)
```

### Stage 2: Filtering

Filter types by namespace and name patterns:

```rust
let filter = Filter::new([
    "Windows.Foundation.*",           // Include namespace
    "!Windows.Foundation.Diagnostics", // Exclude
    "Windows.Win32.Foundation.HANDLE", // Specific type
]);
```

Filter matching uses longest-prefix-first ordering.

### Stage 3: Dependency Resolution

Build transitive closure of required types:

```rust
// TypeMap collects all dependencies
let types = TypeMap::filter(&index, &filter, &references);
```

Each type implements `Dependencies` to declare what it needs:

```rust
impl Dependencies for TypeDef {
    fn dependencies(&self) -> Vec<TypeName> {
        let mut deps = vec![];
        deps.extend(self.fields().flat_map(|f| f.ty().dependencies()));
        deps.extend(self.methods().flat_map(|m| m.signature().dependencies()));
        deps
    }
}
```

### Stage 4: Code Generation

Generate Rust code for each type category:

```rust
impl Config {
    fn write_type(&self, def: TypeDef) -> TokenStream {
        match def.category() {
            TypeCategory::Struct => self.write_struct(def),
            TypeCategory::Enum => self.write_enum(def),
            TypeCategory::Interface => self.write_interface(def),
            TypeCategory::Class => self.write_class(def),
            TypeCategory::Delegate => self.write_delegate(def),
            TypeCategory::Attribute => quote!{}, // Skip
        }
    }
}
```

---

## Generated Code Patterns

### Structs

Input (metadata):
```
TypeDef: Windows.Foundation.Point
  Fields: X (f32), Y (f32)
  Attributes: System.ValueType
```

Output (Rust):
```rust
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub X: f32,
    pub Y: f32,
}

impl windows_core::TypeKind for Point {
    type TypeKind = windows_core::CopyType;
}
```

### Enums

Input (metadata):
```
TypeDef: Windows.Win32.Foundation.WIN32_ERROR
  Fields: ERROR_SUCCESS (0), ERROR_FILE_NOT_FOUND (2), ...
  BaseType: System.Enum (u32)
```

Output (Rust):
```rust
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIN32_ERROR(pub u32);

impl WIN32_ERROR {
    pub const ERROR_SUCCESS: Self = Self(0);
    pub const ERROR_FILE_NOT_FOUND: Self = Self(2);
}

impl core::ops::BitOr for WIN32_ERROR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}
// ... BitAnd, BitOrAssign, etc.
```

### Win32 Functions

Input (metadata):
```
MethodDef: GetTickCount
  DllImport: kernel32.dll
  Returns: u32
```

Output (Rust):
```rust
#[inline]
pub unsafe fn GetTickCount() -> u32 {
    windows_link::link!("kernel32.dll" "system" fn GetTickCount() -> u32);
    unsafe { GetTickCount() }
}
```

Sys-style output:
```rust
windows_link::link!("kernel32.dll" "system" fn GetTickCount() -> u32);
```

### COM Interfaces

Input (metadata):
```
TypeDef: IUnknown
  GUID: 00000000-0000-0000-C000-000000000046
  Methods:
    QueryInterface(riid: *const GUID, ppvObject: *mut *mut c_void) -> HRESULT
    AddRef() -> u32
    Release() -> u32
```

Output (Rust):
```rust
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IUnknown(core::ptr::NonNull<core::ffi::c_void>);

impl IUnknown {
    pub unsafe fn QueryInterface<T: Interface>(&self) -> windows_core::Result<T> {
        let mut result = None;
        (self.vtable().QueryInterface)(
            self.as_raw(),
            &T::IID,
            &mut result as *mut _ as *mut _,
        ).ok()?;
        result.ok_or(windows_core::Error::empty())
    }
}

#[repr(C)]
pub struct IUnknown_Vtbl {
    pub QueryInterface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}

unsafe impl Interface for IUnknown {
    type Vtable = IUnknown_Vtbl;
    const IID: windows_core::GUID = windows_core::GUID::from_u128(
        0x00000000_0000_0000_c000_000000000046
    );
}
```

### WinRT Classes

Input (metadata):
```
TypeDef: Windows.Data.Xml.Dom.XmlDocument
  Implements: IXmlDocument, IXmlDocumentIO
  ActivationType: Default
```

Output (Rust):
```rust
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XmlDocument(windows_core::IUnknown);

impl XmlDocument {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|this| unsafe { this.ActivateInstance() })
    }

    fn IActivationFactory<R, F: FnOnce(&IActivationFactory) -> R>(callback: F) -> R {
        static SHARED: windows_core::imp::FactoryCache<XmlDocument, IActivationFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}

impl windows_core::RuntimeName for XmlDocument {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocument";
}
```

### Delegates

Input (metadata):
```
TypeDef: Windows.Foundation.AsyncActionCompletedHandler
  Invoke(asyncInfo: IAsyncAction, asyncStatus: AsyncStatus)
```

Output (Rust):
```rust
#[repr(transparent)]
pub struct AsyncActionCompletedHandler(windows_core::IUnknown);

impl AsyncActionCompletedHandler {
    pub fn new<F: FnMut(Option<&IAsyncAction>, AsyncStatus) -> windows_core::Result<()> + Send + 'static>(
        invoke: F
    ) -> Self {
        // Create closure-wrapping COM object
    }

    pub fn Invoke(&self, asyncInfo: Option<&IAsyncAction>, asyncStatus: AsyncStatus)
        -> windows_core::Result<()>
    {
        let this = self.vtable();
        unsafe { (this.Invoke)(self.as_raw(), ...).ok() }
    }
}
```

---

## Configuration Options

### Output Modes

| Mode | Description |
|------|-------------|
| Nested (default) | Module hierarchy matching namespaces |
| Flat (`--flat`) | All types in single module |
| Package (`--package`) | Multiple files with Cargo.toml |

### Binding Styles

| Style | Description |
|-------|-------------|
| Normal (default) | Safe wrappers with Result types |
| Sys (`--sys`) | Raw link! macros only |
| Sys + fn ptrs | Function pointer types |

### Additional Options

```
--implement      Include *_Impl traits for implementing interfaces
--no-deps        Skip windows-* crate dependencies
--no-comment     Skip generation comment
--derive X=A,B   Add derives to specific types
--reference      Link to external crate definitions
```

---

## Type Mapping

### Primitive Types

| Metadata | Rust |
|----------|------|
| System.Boolean | bool |
| System.Byte | u8 |
| System.Int16 | i16 |
| System.Int32 | i32 |
| System.Int64 | i64 |
| System.UInt16 | u16 |
| System.UInt32 | u32 |
| System.UInt64 | u64 |
| System.Single | f32 |
| System.Double | f64 |
| System.Char | u16 |
| System.IntPtr | isize |
| System.UIntPtr | usize |
| System.Void | () |

### Special Types

| Metadata | Rust |
|----------|------|
| System.Guid | GUID |
| System.String | HSTRING |
| Windows.Win32.Foundation.BOOL | BOOL |
| Windows.Win32.Foundation.HANDLE | HANDLE |
| Windows.Win32.Foundation.HRESULT | HRESULT |

### Pointer Types

| Metadata | Rust |
|----------|------|
| T* (mutable) | *mut T |
| T* (const, via IsConst) | *const T |
| T& (byref) | &T or &mut T |
| T[] (szarray) | *mut T with length param |

---

## Architecture-Specific Code

Some types vary by architecture:

```rust
// Generated with cfg attributes
#[cfg(target_arch = "x86_64")]
pub struct CONTEXT { /* 64-bit layout */ }

#[cfg(target_arch = "x86")]
pub struct CONTEXT { /* 32-bit layout */ }

#[cfg(target_arch = "aarch64")]
pub struct CONTEXT { /* ARM64 layout */ }
```

Detected via `SupportedArchitectureAttribute` in metadata.

---

## Extending Bindings

### Custom Derives

```rust
windows_bindgen::bindgen([
    "--out", "src/bindings.rs",
    "--filter", "Windows.Foundation.Numerics.Vector2",
    "--derive", "Windows.Foundation.Numerics.Vector2=Hash,Ord",
]);
```

Generates:
```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Hash, Ord)]
pub struct Vector2 { /* ... */ }
```

### External References

Link to types defined elsewhere:

```rust
windows_bindgen::bindgen([
    "--reference", "my_crate,full,Windows.Foundation.Point",
    // Will generate: use my_crate::Windows::Foundation::Point;
]);
```
