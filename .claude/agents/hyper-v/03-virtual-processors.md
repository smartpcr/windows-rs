# Virtual Processors

## Overview

Virtual Processors (VPs) are the execution units within a partition. Each VP represents a virtualized CPU core with its own register state, APIC, and execution context.

## VP Lifecycle

```
┌─────────────────┐
│ Create VP       │  WHvCreateVirtualProcessor()
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Set Initial     │  WHvSetVirtualProcessorRegisters()
│ State           │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Run Loop        │◄───────────────┐
│ WHvRunVP()      │                │
└────────┬────────┘                │
         │                         │
         ▼                         │
┌─────────────────┐                │
│ Handle VM Exit  │────────────────┘
└────────┬────────┘
         │ (shutdown)
         ▼
┌─────────────────┐
│ Delete VP       │  WHvDeleteVirtualProcessor()
└─────────────────┘
```

## Creating Virtual Processors

### Basic Creation

```rust
use windows::Win32::System::Hypervisor::*;

unsafe fn create_vp(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
) -> windows::core::Result<()> {
    // Create VP with default flags
    WHvCreateVirtualProcessor(partition, vp_index, 0)
}
```

### Creation with Properties

```rust
unsafe fn create_vp_with_properties(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
    numa_node: u16,
) -> windows::core::Result<()> {
    let properties = [
        WHV_VIRTUAL_PROCESSOR_PROPERTY {
            PropertyCode: WHvVirtualProcessorPropertyCodeNumaNode,
            Anonymous: WHV_VIRTUAL_PROCESSOR_PROPERTY_0 {
                NumaNode: numa_node,
            },
        },
    ];

    WHvCreateVirtualProcessor2(partition, vp_index, &properties)
}
```

## Running Virtual Processors

### Basic Run Loop

```rust
unsafe fn run_vp(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
) -> windows::core::Result<()> {
    let mut exit_context: WHV_RUN_VP_EXIT_CONTEXT = std::mem::zeroed();

    loop {
        // Run until VM exit
        WHvRunVirtualProcessor(
            partition,
            vp_index,
            &mut exit_context as *mut _ as *mut _,
            std::mem::size_of::<WHV_RUN_VP_EXIT_CONTEXT>() as u32,
        )?;

        // Handle exit
        match exit_context.ExitReason {
            WHvRunVpExitReasonNone => {
                // Should not happen
            }
            WHvRunVpExitReasonMemoryAccess => {
                // Handle MMIO
                let ctx = &exit_context.Anonymous.MemoryAccess;
                handle_mmio(ctx)?;
            }
            WHvRunVpExitReasonX64IoPortAccess => {
                // Handle I/O port
                let ctx = &exit_context.Anonymous.IoPortAccess;
                handle_io_port(ctx)?;
            }
            WHvRunVpExitReasonX64Halt => {
                // Guest executed HLT
                break;
            }
            WHvRunVpExitReasonCanceled => {
                // WHvCancelRunVirtualProcessor was called
                break;
            }
            _ => {
                println!("Unhandled exit: {:?}", exit_context.ExitReason);
                break;
            }
        }
    }

    Ok(())
}
```

### Canceling VP Execution

```rust
// From another thread:
unsafe fn cancel_vp(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
) -> windows::core::Result<()> {
    WHvCancelRunVirtualProcessor(partition, vp_index, 0)
}
```

## VM Exit Reasons

| Exit Reason | Description |
|-------------|-------------|
| `WHvRunVpExitReasonNone` | No exit (shouldn't occur) |
| `WHvRunVpExitReasonMemoryAccess` | Access to unmapped GPA (MMIO) |
| `WHvRunVpExitReasonX64IoPortAccess` | I/O port access |
| `WHvRunVpExitReasonUnrecoverableException` | Triple fault or other fatal error |
| `WHvRunVpExitReasonInvalidVpRegisterValue` | Invalid VP register state |
| `WHvRunVpExitReasonUnsupportedFeature` | Unsupported feature used |
| `WHvRunVpExitReasonX64InterruptWindow` | Interrupt window notification |
| `WHvRunVpExitReasonX64Halt` | HLT instruction |
| `WHvRunVpExitReasonX64ApicEoi` | APIC EOI |
| `WHvRunVpExitReasonSynicSintDeliverable` | SYNIC message deliverable |
| `WHvRunVpExitReasonCanceled` | Run canceled |
| `WHvRunVpExitReasonException` | Exception (if enabled) |
| `WHvRunVpExitReasonX64Cpuid` | CPUID (if enabled) |
| `WHvRunVpExitReasonX64MsrAccess` | MSR access (if enabled) |
| `WHvRunVpExitReasonX64Rdtsc` | RDTSC (if enabled) |
| `WHvRunVpExitReasonHypercall` | Hypercall |

## Register Management

### Common Register Names

**General Purpose Registers:**
- `WHvX64RegisterRax`, `WHvX64RegisterRbx`, `WHvX64RegisterRcx`, `WHvX64RegisterRdx`
- `WHvX64RegisterRsi`, `WHvX64RegisterRdi`, `WHvX64RegisterRsp`, `WHvX64RegisterRbp`
- `WHvX64RegisterR8` - `WHvX64RegisterR15`
- `WHvX64RegisterRip`, `WHvX64RegisterRflags`

**Control Registers:**
- `WHvX64RegisterCr0`, `WHvX64RegisterCr2`, `WHvX64RegisterCr3`, `WHvX64RegisterCr4`, `WHvX64RegisterCr8`

**Segment Registers:**
- `WHvX64RegisterCs`, `WHvX64RegisterDs`, `WHvX64RegisterEs`
- `WHvX64RegisterFs`, `WHvX64RegisterGs`, `WHvX64RegisterSs`

**Descriptor Tables:**
- `WHvX64RegisterGdtr`, `WHvX64RegisterIdtr`, `WHvX64RegisterLdtr`, `WHvX64RegisterTr`

**System Registers:**
- `WHvX64RegisterEfer`
- `WHvX64RegisterTsc`

### Reading Registers

```rust
unsafe fn read_registers(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
) -> windows::core::Result<()> {
    let register_names = [
        WHvX64RegisterRip,
        WHvX64RegisterRsp,
        WHvX64RegisterRflags,
        WHvX64RegisterCr0,
        WHvX64RegisterCr3,
    ];

    let mut register_values: [WHV_REGISTER_VALUE; 5] = std::mem::zeroed();

    WHvGetVirtualProcessorRegisters(
        partition,
        vp_index,
        register_names.as_ptr(),
        register_names.len() as u32,
        register_values.as_mut_ptr(),
    )?;

    // Access values
    let rip = register_values[0].Reg64;
    let rsp = register_values[1].Reg64;
    let rflags = register_values[2].Reg64;
    let cr0 = register_values[3].Reg64;
    let cr3 = register_values[4].Reg64;

    println!("RIP: {:#x}, RSP: {:#x}", rip, rsp);
    Ok(())
}
```

### Writing Registers

```rust
unsafe fn write_registers(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
    rip: u64,
    rsp: u64,
) -> windows::core::Result<()> {
    let register_names = [
        WHvX64RegisterRip,
        WHvX64RegisterRsp,
    ];

    let register_values = [
        WHV_REGISTER_VALUE { Reg64: rip },
        WHV_REGISTER_VALUE { Reg64: rsp },
    ];

    WHvSetVirtualProcessorRegisters(
        partition,
        vp_index,
        register_names.as_ptr(),
        register_names.len() as u32,
        register_values.as_ptr(),
    )
}
```

### Setting Initial State (Real Mode)

```rust
unsafe fn set_real_mode_state(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
) -> windows::core::Result<()> {
    // Set up for real mode boot (like x86 reset state)
    let register_names = [
        WHvX64RegisterRip,
        WHvX64RegisterCs,
        WHvX64RegisterCr0,
        WHvX64RegisterRflags,
    ];

    let mut cs_segment = WHV_X64_SEGMENT_REGISTER::default();
    cs_segment.Base = 0xFFFF0000;
    cs_segment.Limit = 0xFFFF;
    cs_segment.Selector = 0xF000;
    cs_segment.Attributes = 0x9B;  // Present, code, readable

    let register_values = [
        WHV_REGISTER_VALUE { Reg64: 0xFFF0 },  // RIP = reset vector
        WHV_REGISTER_VALUE { Segment: cs_segment },
        WHV_REGISTER_VALUE { Reg64: 0x60000010 },  // CR0: PE=0, ET=1, NW=0, CD=0
        WHV_REGISTER_VALUE { Reg64: 0x2 },  // RFLAGS: reserved bit 1 set
    ];

    WHvSetVirtualProcessorRegisters(
        partition,
        vp_index,
        register_names.as_ptr(),
        register_names.len() as u32,
        register_values.as_ptr(),
    )
}
```

## VP State Management

### Get VP State

```rust
unsafe fn get_vp_state(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
) -> windows::core::Result<Vec<u8>> {
    let mut buffer = vec![0u8; 4096];
    let mut bytes_written = 0u32;

    WHvGetVirtualProcessorState(
        partition,
        vp_index,
        WHvVirtualProcessorStateTypeXsaveState,
        buffer.as_mut_ptr() as *mut _,
        buffer.len() as u32,
        Some(&mut bytes_written),
    )?;

    buffer.truncate(bytes_written as usize);
    Ok(buffer)
}
```

### Set VP State

```rust
unsafe fn set_vp_state(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
    state: &[u8],
) -> windows::core::Result<()> {
    WHvSetVirtualProcessorState(
        partition,
        vp_index,
        WHvVirtualProcessorStateTypeXsaveState,
        state.as_ptr() as *const _,
        state.len() as u32,
    )
}
```

## Interrupt Controller State

### Get APIC State

```rust
unsafe fn get_apic_state(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
) -> windows::core::Result<Vec<u8>> {
    let mut buffer = vec![0u8; 1024];
    let mut bytes_written = 0u32;

    WHvGetVirtualProcessorInterruptControllerState2(
        partition,
        vp_index,
        buffer.as_mut_ptr() as *mut _,
        buffer.len() as u32,
        Some(&mut bytes_written),
    )?;

    buffer.truncate(bytes_written as usize);
    Ok(buffer)
}
```

### Set APIC State

```rust
unsafe fn set_apic_state(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
    state: &[u8],
) -> windows::core::Result<()> {
    WHvSetVirtualProcessorInterruptControllerState2(
        partition,
        vp_index,
        state.as_ptr() as *const _,
        state.len() as u32,
    )
}
```

## XSAVE State

### Get XSAVE State

```rust
unsafe fn get_xsave_state(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
) -> windows::core::Result<Vec<u8>> {
    let mut buffer = vec![0u8; 8192];
    let mut bytes_written = 0u32;

    WHvGetVirtualProcessorXsaveState(
        partition,
        vp_index,
        buffer.as_mut_ptr() as *mut _,
        buffer.len() as u32,
        &mut bytes_written,
    )?;

    buffer.truncate(bytes_written as usize);
    Ok(buffer)
}
```

## CPUID Customization

```rust
unsafe fn get_cpuid_output(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
    eax: u32,
    ecx: u32,
) -> windows::core::Result<WHV_CPUID_OUTPUT> {
    WHvGetVirtualProcessorCpuidOutput(partition, vp_index, eax, ecx)
}
```

## VP Counters

```rust
unsafe fn get_vp_counters(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
) -> windows::core::Result<WHV_PROCESSOR_RUNTIME_COUNTERS> {
    let mut counters: WHV_PROCESSOR_RUNTIME_COUNTERS = std::mem::zeroed();
    let mut bytes_written = 0u32;

    WHvGetVirtualProcessorCounters(
        partition,
        vp_index,
        WHvProcessorCounterSetRuntime,
        &mut counters as *mut _ as *mut _,
        std::mem::size_of::<WHV_PROCESSOR_RUNTIME_COUNTERS>() as u32,
        Some(&mut bytes_written),
    )?;

    Ok(counters)
}
```

## Deleting Virtual Processors

```rust
unsafe fn delete_vp(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
) -> windows::core::Result<()> {
    // VP must not be running
    WHvDeleteVirtualProcessor(partition, vp_index)
}
```

## Complete Example: Boot a Simple Guest

```rust
use windows::Win32::System::Hypervisor::*;

unsafe fn boot_guest(
    partition: WHV_PARTITION_HANDLE,
    code: &[u8],
    entry_point: u64,
) -> windows::core::Result<()> {
    // 1. Create VP
    WHvCreateVirtualProcessor(partition, 0, 0)?;

    // 2. Set initial state (protected mode)
    let register_names = [
        WHvX64RegisterRip,
        WHvX64RegisterRsp,
        WHvX64RegisterRflags,
        WHvX64RegisterCr0,
    ];

    let register_values = [
        WHV_REGISTER_VALUE { Reg64: entry_point },
        WHV_REGISTER_VALUE { Reg64: 0x8000 },  // Stack at 32KB
        WHV_REGISTER_VALUE { Reg64: 0x2 },     // RFLAGS
        WHV_REGISTER_VALUE { Reg64: 0x1 },     // CR0.PE = 1
    ];

    WHvSetVirtualProcessorRegisters(
        partition,
        0,
        register_names.as_ptr(),
        register_names.len() as u32,
        register_values.as_ptr(),
    )?;

    // 3. Run loop
    let mut exit_context: WHV_RUN_VP_EXIT_CONTEXT = std::mem::zeroed();

    loop {
        WHvRunVirtualProcessor(
            partition,
            0,
            &mut exit_context as *mut _ as *mut _,
            std::mem::size_of::<WHV_RUN_VP_EXIT_CONTEXT>() as u32,
        )?;

        match exit_context.ExitReason {
            WHvRunVpExitReasonX64Halt => {
                println!("Guest halted");
                break;
            }
            WHvRunVpExitReasonX64IoPortAccess => {
                let ctx = &exit_context.Anonymous.IoPortAccess;
                if ctx.PortNumber == 0x3F8 {
                    // Serial port output
                    print!("{}", ctx.Rax as u8 as char);
                }
            }
            _ => {
                println!("Exit: {:?}", exit_context.ExitReason);
                break;
            }
        }
    }

    // 4. Cleanup
    WHvDeleteVirtualProcessor(partition, 0)?;

    Ok(())
}
```
