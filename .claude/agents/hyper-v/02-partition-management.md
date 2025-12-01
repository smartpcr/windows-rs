# Partition Management

## Overview

A partition is the fundamental isolation unit in Hyper-V. Each virtual machine runs within its own partition, providing complete isolation of memory, CPU state, and devices.

## Partition Lifecycle

```
┌──────────────┐
│    Create    │  WHvCreatePartition()
└──────┬───────┘
       │
       ▼
┌──────────────┐
│  Configure   │  WHvSetPartitionProperty() (multiple calls)
└──────┬───────┘
       │
       ▼
┌──────────────┐
│    Setup     │  WHvSetupPartition()
└──────┬───────┘
       │
       ▼
┌──────────────┐
│   Running    │  Add VPs, map memory, run
└──────┬───────┘
       │
       ▼
┌──────────────┐
│   Delete     │  WHvDeletePartition()
└──────────────┘
```

## Creating a Partition

```rust
use windows::Win32::System::Hypervisor::*;

unsafe fn create_partition() -> windows::core::Result<WHV_PARTITION_HANDLE> {
    // Create returns a handle
    let partition = WHvCreatePartition()?;

    // IMPORTANT: Must configure before setup
    // Partition is not usable until WHvSetupPartition is called

    Ok(partition)
}
```

## Configuring Partition Properties

Properties must be set **before** calling `WHvSetupPartition()`.

### Processor Count (Required)

```rust
unsafe fn set_processor_count(
    partition: WHV_PARTITION_HANDLE,
    count: u32,
) -> windows::core::Result<()> {
    WHvSetPartitionProperty(
        partition,
        WHvPartitionPropertyCodeProcessorCount,
        &count as *const _ as *const _,
        std::mem::size_of::<u32>() as u32,
    )
}
```

### Extended VM Exits

Enable additional exit reasons:

```rust
unsafe fn enable_extended_exits(
    partition: WHV_PARTITION_HANDLE,
) -> windows::core::Result<()> {
    let mut exits = WHV_EXTENDED_VM_EXITS::default();

    // Enable CPUID exits
    exits._bitfield |= 1 << 0;  // X64CpuidExit

    // Enable MSR exits
    exits._bitfield |= 1 << 1;  // X64MsrExit

    // Enable exception exits
    exits._bitfield |= 1 << 2;  // ExceptionExit

    WHvSetPartitionProperty(
        partition,
        WHvPartitionPropertyCodeExtendedVmExits,
        &exits as *const _ as *const _,
        std::mem::size_of::<WHV_EXTENDED_VM_EXITS>() as u32,
    )
}
```

### Exception Exit Bitmap

Control which exceptions cause VM exits:

```rust
unsafe fn set_exception_bitmap(
    partition: WHV_PARTITION_HANDLE,
    bitmap: u64,
) -> windows::core::Result<()> {
    // Bitmap: bit N = exception vector N
    // E.g., bit 14 = #PF (Page Fault)
    WHvSetPartitionProperty(
        partition,
        WHvPartitionPropertyCodeExceptionExitBitmap,
        &bitmap as *const _ as *const _,
        std::mem::size_of::<u64>() as u32,
    )
}
```

### Nested Virtualization

```rust
unsafe fn enable_nested_virtualization(
    partition: WHV_PARTITION_HANDLE,
) -> windows::core::Result<()> {
    let enabled = 1u32;
    WHvSetPartitionProperty(
        partition,
        WHvPartitionPropertyCodeNestedVirtualization,
        &enabled as *const _ as *const _,
        std::mem::size_of::<u32>() as u32,
    )
}
```

### CPU Features

Configure which CPU features are exposed to guest:

```rust
unsafe fn configure_processor_features(
    partition: WHV_PARTITION_HANDLE,
) -> windows::core::Result<()> {
    // Get default features first
    let mut features: WHV_PROCESSOR_FEATURES = std::mem::zeroed();
    let mut size = 0u32;

    WHvGetPartitionProperty(
        partition,
        WHvPartitionPropertyCodeProcessorFeatures,
        &mut features as *mut _ as *mut _,
        std::mem::size_of::<WHV_PROCESSOR_FEATURES>() as u32,
        Some(&mut size),
    )?;

    // Modify features as needed
    // features._bitfield &= !some_feature_bit;

    WHvSetPartitionProperty(
        partition,
        WHvPartitionPropertyCodeProcessorFeatures,
        &features as *const _ as *const _,
        std::mem::size_of::<WHV_PROCESSOR_FEATURES>() as u32,
    )
}
```

## Property Codes Reference

| Property Code | Description | Type |
|--------------|-------------|------|
| `WHvPartitionPropertyCodeProcessorCount` | Number of VPs | u32 |
| `WHvPartitionPropertyCodeExtendedVmExits` | Extended exit reasons | WHV_EXTENDED_VM_EXITS |
| `WHvPartitionPropertyCodeExceptionExitBitmap` | Exception vectors to exit on | u64 |
| `WHvPartitionPropertyCodeNestedVirtualization` | Enable nested virtualization | u32 (bool) |
| `WHvPartitionPropertyCodeProcessorFeatures` | CPU feature flags | WHV_PROCESSOR_FEATURES |
| `WHvPartitionPropertyCodeProcessorXsaveFeatures` | XSAVE features | WHV_PROCESSOR_XSAVE_FEATURES |
| `WHvPartitionPropertyCodeSeparateSecurityDomain` | Separate security domain | u32 (bool) |
| `WHvPartitionPropertyCodeX64MsrExitBitmap` | MSRs that cause exits | u64 |
| `WHvPartitionPropertyCodePrimaryNumaNode` | NUMA node assignment | u32 |
| `WHvPartitionPropertyCodeCpuReserve` | CPU reservation (%) | u32 |
| `WHvPartitionPropertyCodeCpuCap` | CPU cap (%) | u32 |
| `WHvPartitionPropertyCodeCpuWeight` | Scheduling weight | u32 |
| `WHvPartitionPropertyCodeCpuGroupId` | CPU group ID | u64 |
| `WHvPartitionPropertyCodeProcessorFrequencyCap` | Max processor frequency | u32 |
| `WHvPartitionPropertyCodeAllowDeviceAssignment` | Enable device assignment | u32 (bool) |
| `WHvPartitionPropertyCodeDisableSmt` | Disable SMT | u32 (bool) |
| `WHvPartitionPropertyCodeLocalApicEmulationMode` | APIC emulation mode | WHV_X64_LOCAL_APIC_EMULATION_MODE |
| `WHvPartitionPropertyCodeReferenceTime` | Reference time counter | u64 |

## Setting Up the Partition

After all properties are configured:

```rust
unsafe fn setup_partition(
    partition: WHV_PARTITION_HANDLE,
) -> windows::core::Result<()> {
    // Finalizes configuration
    // No property changes allowed after this
    WHvSetupPartition(partition)
}
```

## Resetting a Partition

Reset partition to initial state (retains configuration):

```rust
unsafe fn reset_partition(
    partition: WHV_PARTITION_HANDLE,
) -> windows::core::Result<()> {
    WHvResetPartition(partition)
}
```

## Deleting a Partition

```rust
unsafe fn delete_partition(
    partition: WHV_PARTITION_HANDLE,
) -> windows::core::Result<()> {
    // All VPs must be deleted first
    // All memory mappings are automatically cleaned up
    WHvDeletePartition(partition)
}
```

## Partition Migration

Move a partition to another process:

```rust
unsafe fn migrate_partition(
    partition: WHV_PARTITION_HANDLE,
) -> windows::core::Result<()> {
    // Start migration - returns a handle
    let migration_handle = WHvStartPartitionMigration(partition)?;

    // Transfer migration_handle to target process...

    // In target process:
    // let new_partition = WHvAcceptPartitionMigration(migration_handle)?;

    // Complete migration in original process
    WHvCompletePartitionMigration(partition)?;

    Ok(())
}

// Cancel if needed
unsafe fn cancel_migration(
    partition: WHV_PARTITION_HANDLE,
) -> windows::core::Result<()> {
    WHvCancelPartitionMigration(partition)
}
```

## Time Management

Suspend and resume partition time:

```rust
unsafe fn manage_partition_time(
    partition: WHV_PARTITION_HANDLE,
) -> windows::core::Result<()> {
    // Suspend time (useful for save/restore)
    WHvSuspendPartitionTime(partition)?;

    // ... perform operations ...

    // Resume time
    WHvResumePartitionTime(partition)?;

    Ok(())
}
```

## Partition Counters

Get performance counters:

```rust
unsafe fn get_partition_counters(
    partition: WHV_PARTITION_HANDLE,
) -> windows::core::Result<WHV_PARTITION_MEMORY_COUNTERS> {
    let mut counters: WHV_PARTITION_MEMORY_COUNTERS = std::mem::zeroed();
    let mut bytes_written = 0u32;

    WHvGetPartitionCounters(
        partition,
        WHvPartitionCounterSetMemory,
        &mut counters as *mut _ as *mut _,
        std::mem::size_of::<WHV_PARTITION_MEMORY_COUNTERS>() as u32,
        Some(&mut bytes_written),
    )?;

    Ok(counters)
}
```

## Complete Example

```rust
use windows::Win32::System::Hypervisor::*;

unsafe fn create_configured_partition() -> windows::core::Result<WHV_PARTITION_HANDLE> {
    // 1. Create partition
    let partition = WHvCreatePartition()?;

    // 2. Set processor count
    let processor_count = 2u32;
    WHvSetPartitionProperty(
        partition,
        WHvPartitionPropertyCodeProcessorCount,
        &processor_count as *const _ as *const _,
        std::mem::size_of::<u32>() as u32,
    )?;

    // 3. Enable extended exits for CPUID
    let mut exits = WHV_EXTENDED_VM_EXITS::default();
    exits._bitfield |= 1;  // X64CpuidExit
    WHvSetPartitionProperty(
        partition,
        WHvPartitionPropertyCodeExtendedVmExits,
        &exits as *const _ as *const _,
        std::mem::size_of::<WHV_EXTENDED_VM_EXITS>() as u32,
    )?;

    // 4. Setup partition (finalize)
    WHvSetupPartition(partition)?;

    Ok(partition)
}
```
