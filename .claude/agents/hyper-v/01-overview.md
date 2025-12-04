# Hyper-V API Overview

## Introduction

The Windows Hypervisor Platform (WHP) provides a set of APIs for creating and managing virtual machines at the hypervisor level. These APIs are lower-level than the Hyper-V management APIs (WMI) and allow fine-grained control over VM execution.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    User Application                         │
│                    (VMM, Emulator)                          │
└─────────────────────────────────────────────────────────────┘
                              │
         ┌────────────────────┼────────────────────┐
         ▼                    ▼                    ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│ winhvplatform   │  │ winhvemulation  │  │  vmdevicehost   │
│     .dll        │  │      .dll       │  │      .dll       │
│                 │  │                 │  │                 │
│ - Partitions    │  │ - I/O Emulation │  │ - Device Host   │
│ - VPs           │  │ - MMIO          │  │ - PCI Devices   │
│ - Memory        │  │ - Instruction   │  │ - Memory        │
│ - Interrupts    │  │   Decode        │  │   Apertures     │
└─────────────────┘  └─────────────────┘  └─────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Windows Hypervisor                        │
│                        (hvix64)                              │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                         Hardware                             │
│              (Intel VT-x / AMD-V, SLAT)                     │
└─────────────────────────────────────────────────────────────┘
```

## Key Concepts

### Partitions

A **partition** is the fundamental unit of isolation in Hyper-V. Each VM runs in its own partition with:
- Isolated address space (Guest Physical Address - GPA)
- One or more virtual processors
- Virtual devices
- Independent security domain

```rust
// Create a partition
let partition: WHV_PARTITION_HANDLE = WHvCreatePartition()?;

// Configure and finalize
WHvSetPartitionProperty(partition, property_code, &value, size)?;
WHvSetupPartition(partition)?;

// Cleanup
WHvDeletePartition(partition)?;
```

### Virtual Processors (VPs)

**Virtual processors** execute guest code. Each VP has:
- Full CPU state (registers, control registers, MSRs)
- Local APIC state
- XSAVE state for extended features

```rust
// Create VP
WHvCreateVirtualProcessor(partition, vp_index, flags)?;

// Run VP (blocks until VM exit)
let mut exit_context: WHV_RUN_VP_EXIT_CONTEXT = std::mem::zeroed();
WHvRunVirtualProcessor(partition, vp_index, &mut exit_context, size)?;

// Handle exit based on exit_context.ExitReason
```

### Guest Physical Address (GPA) Space

The **GPA space** is the VM's view of physical memory:
- Backed by host virtual memory
- Mapped with read/write/execute permissions
- Supports dirty page tracking

```rust
// Map host memory to guest physical address
WHvMapGpaRange(
    partition,
    host_memory_ptr,
    guest_physical_address,
    size_in_bytes,
    WHV_MAP_GPA_RANGE_FLAGS::Read | WHV_MAP_GPA_RANGE_FLAGS::Write,
)?;
```

### VM Exits

When the guest performs certain operations, control returns to the VMM via a **VM exit**:
- Memory access to unmapped regions (MMIO)
- I/O port access
- Privileged instructions (CPUID, MSR access)
- Interrupts and exceptions
- Halts

## Feature Flag

Enable in Cargo.toml:

```toml
[dependencies.windows]
features = ["Win32_System_Hypervisor"]
```

## DLL Dependencies

| DLL | Purpose |
|-----|---------|
| `winhvplatform.dll` | Core hypervisor platform APIs |
| `winhvemulation.dll` | Instruction emulation |
| `vmdevicehost.dll` | Host device virtualization |
| `vmsavedstatedumpprovider.dll` | Saved state debugging |

## Capability Detection

Always check capabilities before use:

```rust
use windows::Win32::System::Hypervisor::*;

unsafe fn check_hypervisor() -> windows::core::Result<bool> {
    let mut capability: WHV_CAPABILITY = std::mem::zeroed();

    WHvGetCapability(
        WHvCapabilityCodeHypervisorPresent,
        &mut capability as *mut _ as *mut _,
        std::mem::size_of::<WHV_CAPABILITY>() as u32,
        None,
    )?;

    Ok(capability.HypervisorPresent.as_bool())
}
```

## Common Capability Codes

| Code | Description |
|------|-------------|
| `WHvCapabilityCodeHypervisorPresent` | Check if hypervisor is available |
| `WHvCapabilityCodeFeatures` | Platform feature flags |
| `WHvCapabilityCodeExtendedVmExits` | Extended VM exit support |
| `WHvCapabilityCodeProcessorVendor` | Intel/AMD |
| `WHvCapabilityCodeProcessorFeatures` | CPU feature flags |
| `WHvCapabilityCodeProcessorXsaveFeatures` | XSAVE capabilities |

## Error Handling

All WHP functions return `HRESULT` values. In windows-rs, these are converted to `Result<T>`:

```rust
match WHvCreatePartition() {
    Ok(handle) => { /* success */ }
    Err(e) => {
        eprintln!("Failed: {:?}", e);
        // Common errors:
        // E_ACCESSDENIED - Need admin rights
        // WHV_E_UNKNOWN_CAPABILITY - Feature not supported
        // WHV_E_INVALID_PARTITION_CONFIG - Bad configuration
    }
}
```

## Typical VMM Flow

```
1. Check capabilities
         │
         ▼
2. Create partition
         │
         ▼
3. Configure partition properties
   - Processor count
   - Extended VM exits
   - CPUID customization
         │
         ▼
4. Setup partition (finalize config)
         │
         ▼
5. Allocate and map guest memory
         │
         ▼
6. Create virtual processor(s)
         │
         ▼
7. Set initial VP state (registers)
         │
         ▼
8. Run VP loop:
   ┌──────────────┐
   │ WHvRunVP     │◄──────────────┐
   └──────┬───────┘               │
          │                       │
          ▼                       │
   ┌──────────────┐               │
   │ Handle Exit  │───────────────┘
   │ - MMIO       │
   │ - I/O        │
   │ - CPUID      │
   │ - etc.       │
   └──────────────┘
          │
          ▼ (on shutdown)
9. Cleanup (delete VP, unmap memory, delete partition)
```

## Platform Requirements

- **Windows Version**: Windows 10 1803+ or Windows Server 2019+
- **Hardware**: Intel VT-x with EPT or AMD-V with NPT
- **Windows Features**: Hyper-V must be enabled
- **Privileges**: Administrator rights required
