# windows-rs Usage Guide

## Getting Started

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies.windows]
version = ">=0.59, <=0.62"
features = [
    "Win32_Foundation",
    "Win32_System_Threading",
]
```

Using a version range improves dependency resolution in larger projects.

### Finding Features

Use the [Feature Search](https://microsoft.github.io/windows-rs/features) tool to find which feature enables the API you need.

Alternatively, search locally:
```bash
# In the windows-rs repository
grep -r "GetTickCount" crates/libs/windows/src/
```

---

## Common Patterns

### Calling Win32 Functions

Most Win32 functions are unsafe:

```rust
use windows::Win32::System::Threading::*;
use windows::Win32::Foundation::*;

fn main() -> windows::core::Result<()> {
    unsafe {
        // Create an event
        let event = CreateEventW(None, true, false, None)?;

        // Signal the event
        SetEvent(event)?;

        // Wait for the event
        WaitForSingleObject(event, INFINITE);

        // Clean up
        CloseHandle(event)?;
    }
    Ok(())
}
```

### Working with Strings

```rust
use windows::core::{h, s, w, HSTRING, PCWSTR};

// String literals (compile-time)
let ansi = s!("ANSI string");        // PCSTR
let wide = w!("Wide string");         // PCWSTR
let hstr = h!("HSTRING literal");     // &HSTRING

// Runtime string creation
let dynamic = HSTRING::from("Dynamic string");

// Converting to Rust string
let rust_string = dynamic.to_string_lossy();

// Passing to Windows APIs
unsafe {
    MessageBoxW(None, w!("Message"), w!("Title"), MB_OK);
}
```

### Error Handling

```rust
use windows::core::{Error, Result, HRESULT};
use windows::Win32::Foundation::*;

fn example() -> Result<()> {
    unsafe {
        // Method 1: Use ? operator (functions returning Result)
        let handle = CreateEventW(None, true, false, None)?;

        // Method 2: Check HRESULT manually
        let hr = SomeFunction();
        if hr.is_err() {
            return Err(hr.into());
        }

        // Method 3: Get last error
        let result = SomeBoolFunction();
        if !result.as_bool() {
            return Err(Error::from_win32());
        }
    }
    Ok(())
}
```

### COM Interfaces

```rust
use windows::Win32::System::Com::*;
use windows::core::*;

fn use_com() -> Result<()> {
    unsafe {
        // Initialize COM
        CoInitializeEx(None, COINIT_MULTITHREADED)?;

        // Create COM object
        let obj: IUnknown = CoCreateInstance(
            &CLSID_SomeClass,
            None,
            CLSCTX_ALL,
        )?;

        // Cast to specific interface
        let specific: ISpecificInterface = obj.cast()?;

        // Use the interface
        specific.DoSomething()?;

        // COM cleanup
        CoUninitialize();
    }
    Ok(())
}
```

### WinRT APIs

WinRT APIs use modern patterns with Result types:

```rust
use windows::Data::Xml::Dom::*;
use windows::core::*;

fn use_winrt() -> Result<()> {
    // Create WinRT object (no unsafe needed for activation)
    let doc = XmlDocument::new()?;

    // Load XML
    doc.LoadXml(h!("<root><item>Hello</item></root>"))?;

    // Navigate DOM
    let root = doc.DocumentElement()?;
    let name = root.NodeName()?;

    println!("Root element: {}", name);
    Ok(())
}
```

### Async Operations

```rust
use windows::Storage::*;
use windows::core::*;

async fn async_file_ops() -> Result<()> {
    // Async file operations
    let file = StorageFile::GetFileFromPathAsync(h!("C:\\test.txt"))?.await?;

    let content = FileIO::ReadTextAsync(&file)?.await?;
    println!("Content: {}", content);

    Ok(())
}

// Blocking alternative
fn sync_file_ops() -> Result<()> {
    let file = StorageFile::GetFileFromPathAsync(h!("C:\\test.txt"))?.get()?;
    Ok(())
}
```

---

## Implementing COM Interfaces

### Define a Custom Interface

```rust
use windows::core::*;

#[interface("12345678-1234-1234-1234-123456789abc")]
unsafe trait IMyInterface: IUnknown {
    fn Add(&self, a: i32, b: i32) -> i32;
    fn GetName(&self, name: *mut HSTRING) -> HRESULT;
}
```

### Implement the Interface

```rust
#[implement(IMyInterface)]
struct MyObject {
    name: String,
}

impl IMyInterface_Impl for MyObject {
    unsafe fn Add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    unsafe fn GetName(&self, name: *mut HSTRING) -> HRESULT {
        if name.is_null() {
            return E_POINTER;
        }
        *name = HSTRING::from(&self.name);
        S_OK
    }
}

// Usage
fn create_object() -> IMyInterface {
    let obj = MyObject { name: "Test".into() };
    obj.into()  // Converts to COM interface
}
```

---

## Custom Bindings Generation

### Basic bindgen Usage

Create a `build.rs`:

```rust
fn main() {
    windows_bindgen::bindgen([
        "--out", "src/bindings.rs",
        "--filter", "Windows.Win32.Foundation.GetTickCount",
        "--filter", "Windows.Win32.System.Threading.Sleep",
    ]).unwrap();
}
```

### Advanced Configuration

```rust
fn main() {
    windows_bindgen::bindgen([
        // Input/Output
        "--in", "custom.winmd",           // Custom metadata
        "--out", "src/bindings.rs",

        // Filtering
        "--filter", "MyNamespace.*",       // Include namespace
        "--filter", "!MyNamespace.Internal", // Exclude

        // Output style
        "--flat",                          // No module hierarchy
        "--sys",                           // Raw sys-style

        // Features
        "--implement",                     // Include impl traits
        "--no-deps",                       // Skip windows-* deps

        // Customization
        "--derive", "MyType=Hash,Eq",      // Extra derives
    ]).unwrap();
}
```

### Sys-Style Bindings

For zero-overhead raw bindings:

```rust
windows_bindgen::bindgen([
    "--out", "src/bindings.rs",
    "--filter", "Windows.Win32.Foundation.*",
    "--sys",
    "--sys-fn-ptrs",  // Function pointers instead of link!
]).unwrap();
```

---

## Registry Operations

```rust
use windows_registry::*;

fn registry_example() -> Result<()> {
    // Open existing key
    let key = CURRENT_USER.open("Software\\MyApp")?;

    // Create new key
    let new_key = CURRENT_USER.create("Software\\MyApp\\Settings")?;

    // Read values
    let name: String = key.get_string("Name")?;
    let count: u32 = key.get_u32("Count")?;
    let data: Vec<u8> = key.get_value("BinaryData")?;

    // Write values
    new_key.set_string("Version", "1.0.0")?;
    new_key.set_u32("Flags", 0x1234)?;

    // Enumerate
    for subkey in key.keys()? {
        println!("Subkey: {}", subkey?);
    }

    for (name, value) in key.values()? {
        println!("{}: {:?}", name?, value);
    }

    // Transactional operations
    let tx = Transaction::new()?;
    let key = CURRENT_USER
        .options()
        .transaction(&tx)
        .create("Software\\MyApp\\Atomic")?;
    key.set_string("Key1", "Value1")?;
    key.set_string("Key2", "Value2")?;
    tx.commit()?;  // Atomic commit

    Ok(())
}
```

---

## Windows Services

```rust
use windows::Win32::System::Services::*;
use windows::core::*;

fn service_example() -> Result<()> {
    unsafe {
        // Open Service Control Manager
        let scm = OpenSCManagerW(None, None, SC_MANAGER_ALL_ACCESS)?;

        // Open a service
        let service = OpenServiceW(scm, w!("MyService"), SERVICE_ALL_ACCESS)?;

        // Query status
        let mut status = SERVICE_STATUS::default();
        QueryServiceStatus(service, &mut status)?;

        println!("Service state: {}", status.dwCurrentState);

        // Clean up
        CloseServiceHandle(service)?;
        CloseServiceHandle(scm)?;
    }
    Ok(())
}
```

---

## Best Practices

### 1. Use Appropriate Crate

- **Application code**: Use `windows` crate
- **Library code**: Consider `windows-sys` for smaller dependency
- **Performance critical**: Use `windows-sys`

### 2. Minimize Feature Surface

```toml
# Good: Specific features
features = ["Win32_Foundation", "Win32_System_Threading"]

# Avoid: Broad features when not needed
features = ["Win32"]
```

### 3. Handle Errors Properly

```rust
// Good: Propagate with context
fn do_work() -> Result<()> {
    let handle = CreateEventW(None, true, false, None)
        .map_err(|e| {
            eprintln!("Failed to create event: {e}");
            e
        })?;
    Ok(())
}
```

### 4. Clean Up Resources

```rust
// Use RAII patterns where possible
struct SafeHandle(HANDLE);

impl Drop for SafeHandle {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0); }
    }
}
```

### 5. Avoid Unnecessary Unsafe

```rust
// WinRT APIs are often safe
let doc = XmlDocument::new()?;  // No unsafe

// Win32 requires unsafe
unsafe {
    MessageBoxW(None, w!("Hi"), w!("Title"), MB_OK);
}
```

---

## Troubleshooting

### "unresolved import" Error

Ensure you have the correct feature enabled:
```toml
features = ["Win32_UI_WindowsAndMessaging"]  # For MessageBoxW
```

### Linker Errors

Check target architecture matches:
```bash
rustup target add x86_64-pc-windows-msvc
cargo build --target x86_64-pc-windows-msvc
```

### COM Initialization

Many COM APIs require initialization:
```rust
unsafe {
    CoInitializeEx(None, COINIT_MULTITHREADED)?;
    // ... use COM ...
    CoUninitialize();
}
```

### HRESULT Errors

Decode HRESULT values:
```rust
let hr = HRESULT(0x80070005);  // Access denied
let error = Error::from(hr);
println!("Error: {}", error.message());
```
