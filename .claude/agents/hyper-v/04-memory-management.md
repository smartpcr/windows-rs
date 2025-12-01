# Memory Management

## Overview

The Windows Hypervisor Platform provides APIs to manage Guest Physical Address (GPA) space. Host virtual memory is mapped into the GPA space, allowing the guest to access it as physical memory.

## GPA Space Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Guest View (GPA)                         │
│                                                              │
│  0x00000000 ┌────────────────────┐                          │
│             │      RAM           │ ◄─── WHvMapGpaRange      │
│  0x000A0000 ├────────────────────┤                          │
│             │    VGA Memory      │ ◄─── MMIO (exits)        │
│  0x000C0000 ├────────────────────┤                          │
│             │    ROM Area        │ ◄─── Read-only mapping   │
│  0x00100000 ├────────────────────┤                          │
│             │   Extended RAM     │ ◄─── WHvMapGpaRange      │
│             │                    │                          │
│  0xFEC00000 ├────────────────────┤                          │
│             │    IOAPIC          │ ◄─── MMIO                │
│  0xFEE00000 ├────────────────────┤                          │
│             │    Local APIC      │ ◄─── MMIO                │
│             └────────────────────┘                          │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼ (EPT/NPT)
┌─────────────────────────────────────────────────────────────┐
│                    Host View (HVA)                           │
│                                                              │
│  VirtualAlloc'd memory regions                               │
└─────────────────────────────────────────────────────────────┘
```

## Mapping Memory

### Basic Mapping

```rust
use windows::Win32::System::Hypervisor::*;
use windows::Win32::System::Memory::*;

unsafe fn map_guest_memory(
    partition: WHV_PARTITION_HANDLE,
    guest_address: u64,
    size: u64,
) -> windows::core::Result<*mut core::ffi::c_void> {
    // 1. Allocate host memory
    let host_memory = VirtualAlloc(
        None,
        size as usize,
        MEM_RESERVE | MEM_COMMIT,
        PAGE_READWRITE,
    );

    if host_memory.is_null() {
        return Err(windows::core::Error::from_win32());
    }

    // 2. Map into guest GPA space
    WHvMapGpaRange(
        partition,
        host_memory,
        guest_address,
        size,
        WHV_MAP_GPA_RANGE_FLAGS(
            WHvMapGpaRangeFlagRead.0 |
            WHvMapGpaRangeFlagWrite.0 |
            WHvMapGpaRangeFlagExecute.0
        ),
    )?;

    Ok(host_memory)
}
```

### Map Flags

| Flag | Description |
|------|-------------|
| `WHvMapGpaRangeFlagRead` | Guest can read |
| `WHvMapGpaRangeFlagWrite` | Guest can write |
| `WHvMapGpaRangeFlagExecute` | Guest can execute |
| `WHvMapGpaRangeFlagTrackDirtyPages` | Track dirty pages |

### Mapping with Process Handle

Map memory from another process:

```rust
unsafe fn map_from_process(
    partition: WHV_PARTITION_HANDLE,
    process: HANDLE,
    source_address: *const core::ffi::c_void,
    guest_address: u64,
    size: u64,
) -> windows::core::Result<()> {
    WHvMapGpaRange2(
        partition,
        process,
        source_address,
        guest_address,
        size,
        WHV_MAP_GPA_RANGE_FLAGS(
            WHvMapGpaRangeFlagRead.0 | WHvMapGpaRangeFlagWrite.0
        ),
    )
}
```

## Unmapping Memory

```rust
unsafe fn unmap_guest_memory(
    partition: WHV_PARTITION_HANDLE,
    guest_address: u64,
    size: u64,
) -> windows::core::Result<()> {
    WHvUnmapGpaRange(partition, guest_address, size)
}
```

## Reading and Writing GPA

### Direct GPA Access

```rust
unsafe fn read_gpa(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
    guest_address: u64,
    buffer: &mut [u8],
) -> windows::core::Result<()> {
    let controls = WHV_ACCESS_GPA_CONTROLS {
        CacheType: WHvCacheTypeWriteBack.0 as u64,
        Reserved: 0,
    };

    WHvReadGpaRange(
        partition,
        vp_index,
        guest_address,
        controls,
        buffer.as_mut_ptr() as *mut _,
        buffer.len() as u32,
    )
}

unsafe fn write_gpa(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
    guest_address: u64,
    buffer: &[u8],
) -> windows::core::Result<()> {
    let controls = WHV_ACCESS_GPA_CONTROLS {
        CacheType: WHvCacheTypeWriteBack.0 as u64,
        Reserved: 0,
    };

    WHvWriteGpaRange(
        partition,
        vp_index,
        guest_address,
        controls,
        buffer.as_ptr() as *const _,
        buffer.len() as u32,
    )
}
```

## Address Translation

Translate Guest Virtual Address (GVA) to Guest Physical Address (GPA):

```rust
unsafe fn translate_gva_to_gpa(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
    gva: u64,
) -> windows::core::Result<u64> {
    let mut result: WHV_TRANSLATE_GVA_RESULT = std::mem::zeroed();
    let mut gpa: u64 = 0;

    WHvTranslateGva(
        partition,
        vp_index,
        gva,
        WHV_TRANSLATE_GVA_FLAGS(WHvTranslateGvaFlagValidateRead.0),
        &mut result,
        &mut gpa,
    )?;

    if result.ResultCode == WHvTranslateGvaResultSuccess {
        Ok(gpa)
    } else {
        // Translation failed - handle error
        Err(windows::core::Error::from_win32())
    }
}
```

### Translation Flags

| Flag | Description |
|------|-------------|
| `WHvTranslateGvaFlagValidateRead` | Validate read access |
| `WHvTranslateGvaFlagValidateWrite` | Validate write access |
| `WHvTranslateGvaFlagValidateExecute` | Validate execute access |
| `WHvTranslateGvaFlagPrivilegeExempt` | Skip privilege check |
| `WHvTranslateGvaFlagSetPageTableBits` | Update accessed/dirty bits |

### Translation Result Codes

| Code | Description |
|------|-------------|
| `WHvTranslateGvaResultSuccess` | Translation succeeded |
| `WHvTranslateGvaResultPageNotPresent` | Page not mapped |
| `WHvTranslateGvaResultPrivilegeViolation` | Access not allowed |
| `WHvTranslateGvaResultInvalidPageTableFlags` | Invalid page table entry |
| `WHvTranslateGvaResultGpaUnmapped` | GPA not mapped |
| `WHvTranslateGvaResultGpaNoReadAccess` | No read permission |
| `WHvTranslateGvaResultGpaNoWriteAccess` | No write permission |
| `WHvTranslateGvaResultGpaIllegalOverlayAccess` | Overlay access error |
| `WHvTranslateGvaResultIntercept` | Access intercepted |

## Dirty Page Tracking

### Enable Tracking

```rust
unsafe fn map_with_dirty_tracking(
    partition: WHV_PARTITION_HANDLE,
    host_memory: *const core::ffi::c_void,
    guest_address: u64,
    size: u64,
) -> windows::core::Result<()> {
    WHvMapGpaRange(
        partition,
        host_memory,
        guest_address,
        size,
        WHV_MAP_GPA_RANGE_FLAGS(
            WHvMapGpaRangeFlagRead.0 |
            WHvMapGpaRangeFlagWrite.0 |
            WHvMapGpaRangeFlagTrackDirtyPages.0
        ),
    )
}
```

### Query Dirty Bitmap

```rust
unsafe fn get_dirty_pages(
    partition: WHV_PARTITION_HANDLE,
    guest_address: u64,
    size: u64,
) -> windows::core::Result<Vec<u64>> {
    // Calculate bitmap size (1 bit per page, 4KB pages)
    let page_count = (size + 0xFFF) / 0x1000;
    let bitmap_size = ((page_count + 63) / 64) as usize;
    let mut bitmap = vec![0u64; bitmap_size];

    WHvQueryGpaRangeDirtyBitmap(
        partition,
        guest_address,
        size,
        Some(bitmap.as_mut_ptr()),
        (bitmap_size * 8) as u32,
    )?;

    Ok(bitmap)
}
```

## GPA Range Advice

Provide hints about memory usage:

### Populate Memory

```rust
unsafe fn populate_memory(
    partition: WHV_PARTITION_HANDLE,
    ranges: &[WHV_MEMORY_RANGE_ENTRY],
) -> windows::core::Result<()> {
    let advice = WHV_ADVISE_GPA_RANGE_POPULATE {
        Flags: WHvAdviseGpaRangePopulateFlagPrefetch,
    };

    WHvAdviseGpaRange(
        partition,
        ranges,
        WHvAdviseGpaRangeCodePopulate,
        &advice as *const _ as *const _,
        std::mem::size_of::<WHV_ADVISE_GPA_RANGE_POPULATE>() as u32,
    )
}
```

### Pin Memory

```rust
unsafe fn pin_memory(
    partition: WHV_PARTITION_HANDLE,
    ranges: &[WHV_MEMORY_RANGE_ENTRY],
) -> windows::core::Result<()> {
    WHvAdviseGpaRange(
        partition,
        ranges,
        WHvAdviseGpaRangeCodePin,
        std::ptr::null(),
        0,
    )
}
```

### Unpin Memory

```rust
unsafe fn unpin_memory(
    partition: WHV_PARTITION_HANDLE,
    ranges: &[WHV_MEMORY_RANGE_ENTRY],
) -> windows::core::Result<()> {
    WHvAdviseGpaRange(
        partition,
        ranges,
        WHvAdviseGpaRangeCodeUnpin,
        std::ptr::null(),
        0,
    )
}
```

## Handling MMIO Exits

When guest accesses unmapped GPA, a memory access exit occurs:

```rust
unsafe fn handle_mmio_exit(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
    ctx: &WHV_MEMORY_ACCESS_CONTEXT,
) -> windows::core::Result<()> {
    let gpa = ctx.Gpa;
    let access_type = ctx.AccessInfo.AccessType();
    let access_size = ctx.AccessInfo.AccessSize();
    let instruction_bytes = &ctx.InstructionBytes[..ctx.InstructionByteCount as usize];

    println!(
        "MMIO: GPA={:#x}, type={:?}, size={}",
        gpa, access_type, access_size
    );

    if access_type == WHvMemoryAccessWrite.0 as u32 {
        // Write operation - data is in ctx.Data
        let data = match access_size {
            1 => ctx.Anonymous.Data8 as u64,
            2 => ctx.Anonymous.Data16 as u64,
            4 => ctx.Anonymous.Data32 as u64,
            8 => ctx.Anonymous.Data64,
            _ => return Err(windows::core::Error::from_win32()),
        };
        handle_device_write(gpa, data, access_size)?;
    } else {
        // Read operation - need to provide data
        let data = handle_device_read(gpa, access_size)?;

        // Use emulator to complete the instruction
        // Or manually update RAX and advance RIP
    }

    Ok(())
}
```

## Complete Memory Setup Example

```rust
use windows::Win32::System::Hypervisor::*;
use windows::Win32::System::Memory::*;

const MB: u64 = 1024 * 1024;
const GB: u64 = 1024 * MB;

struct GuestMemory {
    host_ptr: *mut core::ffi::c_void,
    guest_base: u64,
    size: u64,
}

impl Drop for GuestMemory {
    fn drop(&mut self) {
        unsafe {
            VirtualFree(self.host_ptr, 0, MEM_RELEASE);
        }
    }
}

unsafe fn setup_guest_memory(
    partition: WHV_PARTITION_HANDLE,
    ram_size: u64,
) -> windows::core::Result<GuestMemory> {
    // Allocate low memory (0 - 640KB)
    let low_mem_size = 640 * 1024;
    let low_mem = VirtualAlloc(
        None,
        low_mem_size as usize,
        MEM_RESERVE | MEM_COMMIT,
        PAGE_READWRITE,
    );

    WHvMapGpaRange(
        partition,
        low_mem,
        0,
        low_mem_size,
        WHV_MAP_GPA_RANGE_FLAGS(
            WHvMapGpaRangeFlagRead.0 |
            WHvMapGpaRangeFlagWrite.0 |
            WHvMapGpaRangeFlagExecute.0
        ),
    )?;

    // Allocate extended memory (1MB+)
    let extended_size = ram_size - MB;
    let extended_mem = VirtualAlloc(
        None,
        extended_size as usize,
        MEM_RESERVE | MEM_COMMIT,
        PAGE_READWRITE,
    );

    WHvMapGpaRange(
        partition,
        extended_mem,
        MB,
        extended_size,
        WHV_MAP_GPA_RANGE_FLAGS(
            WHvMapGpaRangeFlagRead.0 |
            WHvMapGpaRangeFlagWrite.0 |
            WHvMapGpaRangeFlagExecute.0
        ),
    )?;

    Ok(GuestMemory {
        host_ptr: extended_mem,
        guest_base: MB,
        size: extended_size,
    })
}

unsafe fn load_image(
    memory: &GuestMemory,
    image: &[u8],
    guest_offset: u64,
) {
    let dest = (memory.host_ptr as *mut u8)
        .add((guest_offset - memory.guest_base) as usize);
    std::ptr::copy_nonoverlapping(image.as_ptr(), dest, image.len());
}
```
