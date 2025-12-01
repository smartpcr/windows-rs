# Hyper-V API Documentation

This directory contains comprehensive documentation for the Windows Hypervisor Platform (WHP) APIs available through windows-rs.

## Documents

| Document | Description |
|----------|-------------|
| [01-overview.md](01-overview.md) | Introduction to Hyper-V APIs and architecture |
| [02-partition-management.md](02-partition-management.md) | VM partition lifecycle and configuration |
| [03-virtual-processors.md](03-virtual-processors.md) | Virtual processor creation, execution, and state |
| [04-memory-management.md](04-memory-management.md) | Guest physical address space and memory mapping |
| [05-device-virtualization.md](05-device-virtualization.md) | VPCI devices and host device virtualization |
| [06-interrupts-events.md](06-interrupts-events.md) | Interrupt handling and event notifications |
| [07-saved-state.md](07-saved-state.md) | VM saved state debugging and analysis |
| [08-api-reference.md](08-api-reference.md) | Complete API function reference |

## Quick Start

### Cargo.toml

```toml
[dependencies.windows]
version = ">=0.59, <=0.62"
features = [
    "Win32_System_Hypervisor",
    "Win32_Foundation",
]
```

### Basic Example

```rust
use windows::Win32::System::Hypervisor::*;
use windows::core::Result;

fn create_simple_vm() -> Result<()> {
    unsafe {
        // Check hypervisor availability
        let mut present = 0u32;
        WHvGetCapability(
            WHvCapabilityCodeHypervisorPresent,
            &mut present as *mut _ as *mut _,
            std::mem::size_of::<u32>() as u32,
            None,
        )?;

        if present == 0 {
            return Err(windows::core::Error::from_win32());
        }

        // Create partition
        let partition = WHvCreatePartition()?;

        // Configure processor count
        let processor_count = 1u32;
        WHvSetPartitionProperty(
            partition,
            WHvPartitionPropertyCodeProcessorCount,
            &processor_count as *const _ as *const _,
            std::mem::size_of::<u32>() as u32,
        )?;

        // Setup partition
        WHvSetupPartition(partition)?;

        // Create virtual processor
        WHvCreateVirtualProcessor(partition, 0, 0)?;

        // Cleanup
        WHvDeleteVirtualProcessor(partition, 0)?;
        WHvDeletePartition(partition)?;
    }
    Ok(())
}
```

## API Categories

### Core Platform (winhvplatform.dll)
- Partition management
- Virtual processor control
- Memory mapping
- Interrupt handling

### Emulation (winhvemulation.dll)
- I/O port emulation
- MMIO emulation
- Instruction decoding

### Device Host (vmdevicehost.dll)
- Host device virtualization
- PCI device pass-through
- Guest memory access

### Saved State (vmsavedstatedumpprovider.dll)
- VM state debugging
- Memory dump analysis
- Symbol resolution

## Requirements

- Windows 10 version 1803+ or Windows Server 2019+
- Hyper-V enabled in Windows Features
- Administrator privileges for most operations
