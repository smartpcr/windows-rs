# Interrupts and Events

## Overview

The Windows Hypervisor Platform provides APIs for managing virtual interrupts, event notifications, and inter-VM communication.

## Interrupt Delivery

### Request Interrupt

Inject an interrupt into a virtual processor:

```rust
use windows::Win32::System::Hypervisor::*;

unsafe fn request_interrupt(
    partition: WHV_PARTITION_HANDLE,
    interrupt_type: WHV_INTERRUPT_TYPE,
    destination_mode: WHV_INTERRUPT_DESTINATION_MODE,
    trigger_mode: WHV_INTERRUPT_TRIGGER_MODE,
    destination: u32,
    vector: u32,
) -> windows::core::Result<()> {
    let mut interrupt = WHV_INTERRUPT_CONTROL::default();

    // Set interrupt type (Fixed, LowestPriority, NMI, etc.)
    interrupt.Type = interrupt_type;

    // Set destination mode
    interrupt.DestinationMode = destination_mode;

    // Set trigger mode
    interrupt.TriggerMode = trigger_mode;

    // Set destination APIC ID
    interrupt.Destination = destination;

    // Set interrupt vector
    interrupt.Vector = vector;

    WHvRequestInterrupt(
        partition,
        &interrupt,
        std::mem::size_of::<WHV_INTERRUPT_CONTROL>() as u32,
    )
}
```

### Interrupt Types

| Type | Description |
|------|-------------|
| `WHvX64InterruptTypeFixed` | Fixed delivery to specific APIC |
| `WHvX64InterruptTypeLowestPriority` | Lowest priority delivery |
| `WHvX64InterruptTypeSmi` | System Management Interrupt |
| `WHvX64InterruptTypeRemoteRead` | Remote read |
| `WHvX64InterruptTypeNmi` | Non-Maskable Interrupt |
| `WHvX64InterruptTypeInit` | INIT |
| `WHvX64InterruptTypeSipi` | Startup IPI |
| `WHvX64InterruptTypeExtInt` | External interrupt |
| `WHvX64InterruptTypeLocalInt0` | Local INT0 |
| `WHvX64InterruptTypeLocalInt1` | Local INT1 |

### Get Interrupt Target VPs

```rust
unsafe fn get_interrupt_target_vps(
    partition: WHV_PARTITION_HANDLE,
    destination: u64,
    destination_mode: WHV_INTERRUPT_DESTINATION_MODE,
) -> windows::core::Result<Vec<u32>> {
    let mut target_vps = vec![0u32; 64];
    let mut target_vp_count = 0u32;

    WHvGetInterruptTargetVpSet(
        partition,
        destination,
        destination_mode,
        &mut target_vps,
        &mut target_vp_count,
    )?;

    target_vps.truncate(target_vp_count as usize);
    Ok(target_vps)
}
```

## Synthetic Interrupts (SYNIC)

The Synthetic Interrupt Controller provides Hyper-V specific features.

### Post SYNIC Message

```rust
unsafe fn post_synic_message(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
    sint_index: u32,
    message: &[u8],
) -> windows::core::Result<()> {
    assert!(message.len() <= WHV_SYNIC_MESSAGE_SIZE as usize);

    WHvPostVirtualProcessorSynicMessage(
        partition,
        vp_index,
        sint_index,
        message.as_ptr() as *const _,
        message.len() as u32,
    )
}
```

### Signal SYNIC Event

```rust
unsafe fn signal_synic_event(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
    sint_index: u8,
    flag_number: u16,
) -> windows::core::Result<bool> {
    let event_params = WHV_SYNIC_EVENT_PARAMETERS {
        VpIndex: vp_index,
        TargetSint: sint_index,
        Reserved: 0,
        FlagNumber: flag_number,
    };

    let mut newly_signaled = windows::core::BOOL::default();

    WHvSignalVirtualProcessorSynicEvent(
        partition,
        event_params,
        Some(&mut newly_signaled),
    )?;

    Ok(newly_signaled.as_bool())
}
```

## Notification Ports

Notification ports allow receiving events from the hypervisor.

### Create Notification Port

```rust
unsafe fn create_notification_port(
    partition: WHV_PARTITION_HANDLE,
    port_type: WHV_NOTIFICATION_PORT_TYPE,
    event: HANDLE,
) -> windows::core::Result<*mut core::ffi::c_void> {
    let params = WHV_NOTIFICATION_PORT_PARAMETERS {
        NotificationPortType: port_type,
        Reserved: 0,
        Anonymous: WHV_NOTIFICATION_PORT_PARAMETERS_0 {
            Doorbell: WHV_DOORBELL_MATCH_DATA::default(),
        },
    };

    let mut port_handle = std::ptr::null_mut();

    WHvCreateNotificationPort(
        partition,
        &params,
        event,
        &mut port_handle,
    )?;

    Ok(port_handle)
}
```

### Delete Notification Port

```rust
unsafe fn delete_notification_port(
    partition: WHV_PARTITION_HANDLE,
    port_handle: *const core::ffi::c_void,
) -> windows::core::Result<()> {
    WHvDeleteNotificationPort(partition, port_handle)
}
```

### Set Notification Port Property

```rust
unsafe fn set_notification_port_property(
    partition: WHV_PARTITION_HANDLE,
    port_handle: *const core::ffi::c_void,
    property_code: WHV_NOTIFICATION_PORT_PROPERTY_CODE,
    value: u64,
) -> windows::core::Result<()> {
    WHvSetNotificationPortProperty(
        partition,
        port_handle,
        property_code,
        value,
    )
}
```

## Doorbell Events

Doorbells notify when guest writes to specific GPA regions.

### Register Doorbell Event

```rust
unsafe fn register_doorbell(
    partition: WHV_PARTITION_HANDLE,
    guest_address: u64,
    value: u64,
    length: u32,
    event: HANDLE,
) -> windows::core::Result<()> {
    let match_data = WHV_DOORBELL_MATCH_DATA {
        GuestAddress: guest_address,
        Value: value,
        Length: length,
        _bitfield: 0,
    };

    WHvRegisterPartitionDoorbellEvent(partition, &match_data, event)
}
```

### Unregister Doorbell Event

```rust
unsafe fn unregister_doorbell(
    partition: WHV_PARTITION_HANDLE,
    guest_address: u64,
    value: u64,
    length: u32,
) -> windows::core::Result<()> {
    let match_data = WHV_DOORBELL_MATCH_DATA {
        GuestAddress: guest_address,
        Value: value,
        Length: length,
        _bitfield: 0,
    };

    WHvUnregisterPartitionDoorbellEvent(partition, &match_data)
}
```

## Triggers

Triggers provide flexible event notification mechanisms.

### Create Trigger

```rust
unsafe fn create_trigger(
    partition: WHV_PARTITION_HANDLE,
    trigger_type: WHV_TRIGGER_TYPE,
) -> windows::core::Result<(*mut core::ffi::c_void, HANDLE)> {
    let params = WHV_TRIGGER_PARAMETERS {
        TriggerType: trigger_type,
        Reserved: 0,
        Anonymous: WHV_TRIGGER_PARAMETERS_0 {
            InterruptTrigger: WHV_INTERRUPT_CONTROL::default(),
        },
    };

    let mut trigger_handle = std::ptr::null_mut();
    let mut event_handle = HANDLE::default();

    WHvCreateTrigger(
        partition,
        &params,
        &mut trigger_handle,
        &mut event_handle,
    )?;

    Ok((trigger_handle, event_handle))
}
```

### Update Trigger Parameters

```rust
unsafe fn update_trigger(
    partition: WHV_PARTITION_HANDLE,
    trigger_handle: *const core::ffi::c_void,
    params: &WHV_TRIGGER_PARAMETERS,
) -> windows::core::Result<()> {
    WHvUpdateTriggerParameters(partition, params, trigger_handle)
}
```

### Delete Trigger

```rust
unsafe fn delete_trigger(
    partition: WHV_PARTITION_HANDLE,
    trigger_handle: *const core::ffi::c_void,
) -> windows::core::Result<()> {
    WHvDeleteTrigger(partition, trigger_handle)
}
```

## Handling Interrupt Window Exits

When configured via extended VM exits, you can receive notifications when the guest is ready to accept interrupts:

```rust
unsafe fn handle_interrupt_window_exit(
    partition: WHV_PARTITION_HANDLE,
    vp_index: u32,
    exit_context: &WHV_RUN_VP_EXIT_CONTEXT,
) -> windows::core::Result<()> {
    // Guest is now ready for interrupt injection
    // Inject pending interrupt

    let interrupt = WHV_INTERRUPT_CONTROL {
        Type: WHvX64InterruptTypeFixed,
        DestinationMode: WHvX64InterruptDestinationModePhysical,
        TriggerMode: WHvX64InterruptTriggerModeEdge,
        Reserved: 0,
        Destination: 0,  // BSP
        Vector: 0x30,    // Example vector
        Reserved2: 0,
    };

    WHvRequestInterrupt(
        partition,
        &interrupt,
        std::mem::size_of::<WHV_INTERRUPT_CONTROL>() as u32,
    )
}
```

## APIC EOI Exit Handling

Handle APIC End-Of-Interrupt notifications:

```rust
unsafe fn handle_apic_eoi_exit(
    exit_context: &WHV_RUN_VP_EXIT_CONTEXT,
) {
    let vector = exit_context.Anonymous.ApicEoi.InterruptVector;
    println!("APIC EOI for vector {}", vector);

    // Notify device that interrupt was acknowledged
}
```

## Hyper-V Socket (HVSOCKET)

For VM-to-host communication via sockets:

```rust
// Socket address structure
pub struct HVSOCKET_ADDRESS_INFO {
    pub SystemId: GUID,
    pub VirtualMachineId: GUID,
    pub SiloId: GUID,
    pub Flags: u32,
}

// Constants
pub const HVSOCKET_ADDRESS_FLAG_PASSTHRU: u32 = 1;
pub const HVSOCKET_CONNECTED_SUSPEND: u32 = 4;
pub const HVSOCKET_CONNECT_TIMEOUT: u32 = 1;
pub const HVSOCKET_CONTAINER_PASSTHRU: u32 = 2;
pub const HVSOCKET_HIGH_VTL: u32 = 8;
```

## Complete Interrupt Handling Example

```rust
use windows::Win32::System::Hypervisor::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;

struct VirtualInterruptController {
    partition: WHV_PARTITION_HANDLE,
    pending_interrupts: Vec<(u32, u8)>,  // (APIC ID, vector)
}

impl VirtualInterruptController {
    unsafe fn inject_pending_interrupts(&mut self) -> windows::core::Result<()> {
        for (apic_id, vector) in self.pending_interrupts.drain(..) {
            let interrupt = WHV_INTERRUPT_CONTROL {
                Type: WHvX64InterruptTypeFixed,
                DestinationMode: WHvX64InterruptDestinationModePhysical,
                TriggerMode: WHvX64InterruptTriggerModeEdge,
                Reserved: 0,
                Destination: apic_id,
                Vector: vector as u32,
                Reserved2: 0,
            };

            WHvRequestInterrupt(
                self.partition,
                &interrupt,
                std::mem::size_of::<WHV_INTERRUPT_CONTROL>() as u32,
            )?;
        }
        Ok(())
    }

    fn queue_interrupt(&mut self, apic_id: u32, vector: u8) {
        self.pending_interrupts.push((apic_id, vector));
    }
}

unsafe fn setup_interrupt_notification(
    partition: WHV_PARTITION_HANDLE,
) -> windows::core::Result<HANDLE> {
    // Create event for notification
    let event = CreateEventW(None, false, false, None)?;

    // Register doorbell for IOAPIC redirection table
    let match_data = WHV_DOORBELL_MATCH_DATA {
        GuestAddress: 0xFEC00010,  // IOAPIC IOREGSEL
        Value: 0,
        Length: 4,
        _bitfield: 1,  // Match any value
    };

    WHvRegisterPartitionDoorbellEvent(partition, &match_data, event)?;

    Ok(event)
}
```
