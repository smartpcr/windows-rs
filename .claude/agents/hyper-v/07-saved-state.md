# Saved State and Debugging

## Overview

The `vmsavedstatedumpprovider.dll` provides APIs for loading and analyzing VM saved state files (.vmrs, .bin, .vsv). These APIs are useful for debugging VMs, forensic analysis, and building diagnostic tools.

## Loading Saved State Files

### Load Single File

```rust
use windows::Win32::System::Hypervisor::*;

unsafe fn load_saved_state(
    vmrs_path: &str,
) -> windows::core::Result<*mut core::ffi::c_void> {
    let path = windows::core::HSTRING::from(vmrs_path);
    let mut handle = std::ptr::null_mut();

    LoadSavedStateFile(&path, &mut handle)?;

    Ok(handle)
}
```

### Load Binary + VSV Files

```rust
unsafe fn load_saved_state_files(
    bin_path: &str,
    vsv_path: &str,
) -> windows::core::Result<*mut core::ffi::c_void> {
    let bin = windows::core::HSTRING::from(bin_path);
    let vsv = windows::core::HSTRING::from(vsv_path);
    let mut handle = std::ptr::null_mut();

    LoadSavedStateFiles(&bin, &vsv, &mut handle)?;

    Ok(handle)
}
```

### Locate Saved State Files

```rust
unsafe fn locate_saved_state(
    vm_name: &str,
    snapshot_name: &str,
) -> windows::core::Result<(String, String, String)> {
    let vm = windows::core::HSTRING::from(vm_name);
    let snapshot = windows::core::HSTRING::from(snapshot_name);

    let mut bin_path = windows::core::PWSTR::null();
    let mut vsv_path = windows::core::PWSTR::null();
    let mut vmrs_path = windows::core::PWSTR::null();

    LocateSavedStateFiles(&vm, &snapshot, &mut bin_path, &mut vsv_path, &mut vmrs_path)?;

    Ok((
        bin_path.to_string().unwrap_or_default(),
        vsv_path.to_string().unwrap_or_default(),
        vmrs_path.to_string().unwrap_or_default(),
    ))
}
```

### Release Saved State

```rust
unsafe fn release_saved_state(
    handle: *mut core::ffi::c_void,
) -> windows::core::Result<()> {
    ReleaseSavedStateFiles(handle)
}
```

## Virtual Processor Information

### Get VP Count

```rust
unsafe fn get_vp_count(
    handle: *mut core::ffi::c_void,
) -> windows::core::Result<u32> {
    let mut count = 0u32;
    GetVpCount(handle, &mut count)?;
    Ok(count)
}
```

### Get Architecture

```rust
unsafe fn get_architecture(
    handle: *mut core::ffi::c_void,
    vp_id: u32,
) -> windows::core::Result<VIRTUAL_PROCESSOR_ARCH> {
    let mut arch = VIRTUAL_PROCESSOR_ARCH::default();
    GetArchitecture(handle, vp_id, &mut arch)?;
    Ok(arch)
}
```

Architecture values:
- `Arch_Unknown` (0)
- `Arch_x86` (1)
- `Arch_x64` (2)
- `Arch_Armv8` (3)

### Get Register Value

```rust
unsafe fn get_register(
    handle: *mut core::ffi::c_void,
    vp_id: u32,
    register_id: u32,
) -> windows::core::Result<VIRTUAL_PROCESSOR_REGISTER> {
    let mut value = VIRTUAL_PROCESSOR_REGISTER::default();
    GetRegisterValue(handle, vp_id, register_id, &mut value)?;
    Ok(value)
}
```

### Get Paging Mode

```rust
unsafe fn get_paging_mode(
    handle: *mut core::ffi::c_void,
    vp_id: u32,
) -> windows::core::Result<PAGING_MODE> {
    let mut mode = PAGING_MODE::default();
    GetPagingMode(handle, vp_id, &mut mode)?;
    Ok(mode)
}
```

## Memory Access

### Read Guest Physical Address

```rust
unsafe fn read_physical_memory(
    handle: *mut core::ffi::c_void,
    physical_address: u64,
    buffer: &mut [u8],
) -> windows::core::Result<u32> {
    let mut bytes_read = 0u32;

    ReadGuestPhysicalAddress(
        handle,
        physical_address,
        buffer.as_mut_ptr() as *mut _,
        buffer.len() as u32,
        Some(&mut bytes_read),
    )?;

    Ok(bytes_read)
}
```

### Translate Guest Virtual Address

```rust
unsafe fn translate_virtual_to_physical(
    handle: *mut core::ffi::c_void,
    vp_id: u32,
    virtual_address: u64,
) -> windows::core::Result<u64> {
    let mut physical_address = 0u64;

    GuestVirtualAddressToPhysicalAddress(
        handle,
        vp_id,
        virtual_address,
        &mut physical_address,
        None,
    )?;

    Ok(physical_address)
}
```

### Get Guest Memory Chunks

```rust
unsafe fn get_memory_chunks(
    handle: *mut core::ffi::c_void,
) -> windows::core::Result<(u64, Vec<GPA_MEMORY_CHUNK>)> {
    let mut page_size = 0u64;
    let mut chunk_count = 0u64;

    // First call to get count
    GetGuestPhysicalMemoryChunks(
        handle,
        &mut page_size,
        std::ptr::null_mut(),
        &mut chunk_count,
    )?;

    // Allocate and get chunks
    let mut chunks = vec![GPA_MEMORY_CHUNK::default(); chunk_count as usize];

    GetGuestPhysicalMemoryChunks(
        handle,
        &mut page_size,
        chunks.as_mut_ptr(),
        &mut chunk_count,
    )?;

    Ok((page_size, chunks))
}
```

### Get Raw Saved Memory Size

```rust
unsafe fn get_raw_memory_size(
    handle: *mut core::ffi::c_void,
) -> windows::core::Result<u64> {
    let mut size = 0u64;
    GetGuestRawSavedMemorySize(handle, &mut size)?;
    Ok(size)
}
```

### Read Raw Saved Memory

```rust
unsafe fn read_raw_memory(
    handle: *mut core::ffi::c_void,
    offset: u64,
    buffer: &mut [u8],
) -> windows::core::Result<u32> {
    let mut bytes_read = 0u32;

    ReadGuestRawSavedMemory(
        handle,
        offset,
        buffer.as_mut_ptr() as *mut _,
        buffer.len() as u32,
        Some(&mut bytes_read),
    )?;

    Ok(bytes_read)
}
```

## Nested Virtualization

### Check Nested Virtualization

```rust
unsafe fn is_nested_virtualization_enabled(
    handle: *mut core::ffi::c_void,
) -> windows::core::Result<bool> {
    let mut enabled = windows::core::BOOL::default();
    IsNestedVirtualizationEnabled(handle, &mut enabled)?;
    Ok(enabled.as_bool())
}
```

### Force Nested Host Mode

```rust
unsafe fn force_nested_host_mode(
    handle: *mut core::ffi::c_void,
    vp_id: u32,
    host_mode: bool,
) -> windows::core::Result<bool> {
    let mut old_mode = windows::core::BOOL::default();

    ForceNestedHostMode(
        handle,
        vp_id,
        host_mode,
        Some(&mut old_mode),
    )?;

    Ok(old_mode.as_bool())
}
```

## Guest OS Information

### Get Guest OS Info

```rust
unsafe fn get_guest_os_info(
    handle: *mut core::ffi::c_void,
    vtl: u8,
) -> windows::core::Result<GUEST_OS_INFO> {
    let mut info = GUEST_OS_INFO::default();
    GetGuestOsInfo(handle, vtl, &mut info)?;
    Ok(info)
}
```

Guest OS vendor values:
- `GuestOsVendorUndefined` (0)
- `GuestOsVendorMicrosoft` (1)
- `GuestOsVendorHPE` (2)
- `GuestOsVendorLANCOM` (512)

Microsoft OS IDs:
- `GuestOsMicrosoftUndefined` (0)
- `GuestOsMicrosoftMSDOS` (1)
- `GuestOsMicrosoftWindows3x` (2)
- `GuestOsMicrosoftWindows9x` (3)
- `GuestOsMicrosoftWindowsNT` (4)
- `GuestOsMicrosoftWindowsCE` (5)

### Check Kernel Space

```rust
unsafe fn is_in_kernel_space(
    handle: *mut core::ffi::c_void,
    vp_id: u32,
) -> windows::core::Result<bool> {
    let mut in_kernel = windows::core::BOOL::default();
    InKernelSpace(handle, vp_id, &mut in_kernel)?;
    Ok(in_kernel.as_bool())
}
```

## Symbol Support

### Load Symbol Provider

```rust
unsafe fn load_symbols(
    handle: *mut core::ffi::c_void,
    symbol_path: Option<&str>,
    force: bool,
) -> windows::core::Result<()> {
    let path = symbol_path.map(|s| windows::core::HSTRING::from(s));

    LoadSavedStateSymbolProvider(
        handle,
        path.as_ref().map(|h| h.as_wide().as_ptr()).unwrap_or(std::ptr::null()),
        force,
    )
}
```

### Load Module Symbols

```rust
unsafe fn load_module_symbols(
    handle: *mut core::ffi::c_void,
    image_name: &str,
    module_name: &str,
    base_address: u64,
    size: u32,
) -> windows::core::Result<()> {
    LoadSavedStateModuleSymbols(
        handle,
        windows::core::PCSTR::from_raw(image_name.as_ptr()),
        windows::core::PCSTR::from_raw(module_name.as_ptr()),
        base_address,
        size,
    )
}
```

### Get Symbol Type Size

```rust
unsafe fn get_type_size(
    handle: *mut core::ffi::c_void,
    vp_id: u32,
    type_name: &str,
) -> windows::core::Result<u32> {
    let mut size = 0u32;

    GetSavedStateSymbolTypeSize(
        handle,
        vp_id,
        windows::core::PCSTR::from_raw(type_name.as_ptr()),
        &mut size,
    )?;

    Ok(size)
}
```

### Resolve Global Variable

```rust
unsafe fn resolve_global_variable(
    handle: *mut core::ffi::c_void,
    vp_id: u32,
    global_name: &str,
) -> windows::core::Result<(u64, u32)> {
    let mut address = 0u64;
    let mut size = 0u32;

    ResolveSavedStateGlobalVariableAddress(
        handle,
        vp_id,
        windows::core::PCSTR::from_raw(global_name.as_ptr()),
        &mut address,
        Some(&mut size),
    )?;

    Ok((address, size))
}
```

### Read Global Variable

```rust
unsafe fn read_global_variable(
    handle: *mut core::ffi::c_void,
    vp_id: u32,
    global_name: &str,
    buffer: &mut [u8],
) -> windows::core::Result<()> {
    ReadSavedStateGlobalVariable(
        handle,
        vp_id,
        windows::core::PCSTR::from_raw(global_name.as_ptr()),
        buffer.as_mut_ptr() as *mut _,
        buffer.len() as u32,
    )
}
```

## Stack Unwinding

```rust
unsafe fn unwind_call_stack(
    handle: *mut core::ffi::c_void,
    vp_id: u32,
    module_info: &[MODULE_INFO],
    frame_count: u32,
) -> windows::core::Result<String> {
    let mut callstack = windows::core::PWSTR::null();

    CallStackUnwind(
        handle,
        vp_id,
        module_info.as_ptr(),
        module_info.len() as u32,
        frame_count,
        &mut callstack,
    )?;

    Ok(callstack.to_string().unwrap_or_default())
}
```

## Memory Scanning

### Scan for DOS Images

```rust
unsafe extern "system" fn image_callback(
    context: *const core::ffi::c_void,
    image_info: *const DOS_IMAGE_INFO,
) -> windows::core::BOOL {
    let info = &*image_info;
    println!(
        "Found image: base={:#x}, size={}, timestamp={}",
        info.ImageBaseAddress,
        info.ImageSize,
        info.Timestamp
    );
    windows::core::BOOL(1)  // Continue scanning
}

unsafe fn scan_for_images(
    handle: *mut core::ffi::c_void,
    vp_id: u32,
    start: u64,
    end: u64,
) -> windows::core::Result<()> {
    ScanMemoryForDosImages(
        handle,
        vp_id,
        start,
        end,
        std::ptr::null_mut(),
        Some(image_callback),
        std::ptr::null(),
        0,
    )
}
```

## Virtual Trust Levels (VTL)

### Get Active VTL

```rust
unsafe fn get_active_vtl(
    handle: *mut core::ffi::c_void,
    vp_id: u32,
) -> windows::core::Result<u8> {
    let mut vtl = 0u8;
    GetActiveVirtualTrustLevel(handle, vp_id, &mut vtl)?;
    Ok(vtl)
}
```

### Get Enabled VTLs

```rust
unsafe fn get_enabled_vtls(
    handle: *mut core::ffi::c_void,
    vp_id: u32,
) -> windows::core::Result<u32> {
    let mut vtls = 0u32;
    GetEnabledVirtualTrustLevels(handle, vp_id, &mut vtls)?;
    Ok(vtls)
}
```

### Force Active VTL

```rust
unsafe fn force_active_vtl(
    handle: *mut core::ffi::c_void,
    vp_id: u32,
    vtl: u8,
) -> windows::core::Result<()> {
    ForceActiveVirtualTrustLevel(handle, vp_id, vtl)
}
```

## Complete Analysis Example

```rust
use windows::Win32::System::Hypervisor::*;

unsafe fn analyze_vm_state(vmrs_path: &str) -> windows::core::Result<()> {
    // Load saved state
    let handle = {
        let path = windows::core::HSTRING::from(vmrs_path);
        let mut h = std::ptr::null_mut();
        LoadSavedStateFile(&path, &mut h)?;
        h
    };

    // Get basic info
    let mut vp_count = 0u32;
    GetVpCount(handle, &mut vp_count)?;
    println!("VP Count: {}", vp_count);

    for vp_id in 0..vp_count {
        println!("\n=== VP {} ===", vp_id);

        // Get architecture
        let mut arch = VIRTUAL_PROCESSOR_ARCH::default();
        GetArchitecture(handle, vp_id, &mut arch)?;
        println!("Architecture: {:?}", arch);

        // Get paging mode
        let mut paging_mode = PAGING_MODE::default();
        GetPagingMode(handle, vp_id, &mut paging_mode)?;
        println!("Paging Mode: {:?}", paging_mode);

        // Check kernel space
        let mut in_kernel = windows::core::BOOL::default();
        InKernelSpace(handle, vp_id, &mut in_kernel)?;
        println!("In Kernel: {}", in_kernel.as_bool());

        // Get key registers (x64)
        if arch == Arch_x64 {
            let mut rip = VIRTUAL_PROCESSOR_REGISTER::default();
            GetRegisterValue(handle, vp_id, 16, &mut rip)?;  // RIP
            println!("RIP: {:#x}", rip.Reg64);

            let mut rsp = VIRTUAL_PROCESSOR_REGISTER::default();
            GetRegisterValue(handle, vp_id, 20, &mut rsp)?;  // RSP
            println!("RSP: {:#x}", rsp.Reg64);

            let mut cr3 = VIRTUAL_PROCESSOR_REGISTER::default();
            GetRegisterValue(handle, vp_id, 2, &mut cr3)?;  // CR3
            println!("CR3: {:#x}", cr3.Reg64);
        }
    }

    // Get memory layout
    let mut page_size = 0u64;
    let mut chunk_count = 0u64;
    GetGuestPhysicalMemoryChunks(
        handle,
        &mut page_size,
        std::ptr::null_mut(),
        &mut chunk_count,
    )?;
    println!("\nMemory: {} chunks, {} byte pages", chunk_count, page_size);

    // Cleanup
    ReleaseSavedStateFiles(handle)?;

    Ok(())
}
```
