# Device Virtualization

## Overview

The Windows Hypervisor Platform provides two main APIs for device virtualization:

1. **VPCI (Virtual PCI)** - Expose PCI devices to VMs
2. **HDV (Host Device Virtualization)** - Implement virtual devices in host process

## Virtual PCI Devices (VPCI)

VPCI allows passing through or emulating PCI devices for guests.

### Allocate VPCI Resource

```rust
use windows::Win32::System::Hypervisor::*;

unsafe fn allocate_vpci_resource(
    flags: WHV_ALLOCATE_VPCI_RESOURCE_FLAGS,
) -> windows::core::Result<HANDLE> {
    WHvAllocateVpciResource(
        None,  // provider_id
        flags,
        None,  // resource_descriptor
    )
}
```

### Create VPCI Device

```rust
unsafe fn create_vpci_device(
    partition: WHV_PARTITION_HANDLE,
    logical_device_id: u64,
    vpci_resource: HANDLE,
) -> windows::core::Result<()> {
    WHvCreateVpciDevice(
        partition,
        logical_device_id,
        vpci_resource,
        WHV_CREATE_VPCI_DEVICE_FLAGS::default(),
        None,  // notification_event_handle
    )
}
```

### Delete VPCI Device

```rust
unsafe fn delete_vpci_device(
    partition: WHV_PARTITION_HANDLE,
    logical_device_id: u64,
) -> windows::core::Result<()> {
    WHvDeleteVpciDevice(partition, logical_device_id)
}
```

### Get VPCI Device Property

```rust
unsafe fn get_vpci_property(
    partition: WHV_PARTITION_HANDLE,
    logical_device_id: u64,
    property_code: WHV_VPCI_DEVICE_PROPERTY_CODE,
) -> windows::core::Result<Vec<u8>> {
    let mut buffer = vec![0u8; 256];
    let mut written_size = 0u32;

    WHvGetVpciDeviceProperty(
        partition,
        logical_device_id,
        property_code,
        buffer.as_mut_ptr() as *mut _,
        buffer.len() as u32,
        Some(&mut written_size),
    )?;

    buffer.truncate(written_size as usize);
    Ok(buffer)
}
```

### Map VPCI MMIO Ranges

```rust
unsafe fn map_vpci_mmio(
    partition: WHV_PARTITION_HANDLE,
    logical_device_id: u64,
) -> windows::core::Result<Vec<WHV_VPCI_MMIO_MAPPING>> {
    let mut mapping_count = 0u32;
    let mut mappings: *mut WHV_VPCI_MMIO_MAPPING = std::ptr::null_mut();

    WHvMapVpciDeviceMmioRanges(
        partition,
        logical_device_id,
        &mut mapping_count,
        &mut mappings,
    )?;

    // Copy mappings to Vec
    let result = std::slice::from_raw_parts(mappings, mapping_count as usize).to_vec();

    // Caller must eventually call WHvUnmapVpciDeviceMmioRanges

    Ok(result)
}
```

### VPCI Device Interrupts

```rust
unsafe fn map_vpci_interrupt(
    partition: WHV_PARTITION_HANDLE,
    logical_device_id: u64,
    index: u32,
    message_count: u32,
    target: &WHV_VPCI_INTERRUPT_TARGET,
) -> windows::core::Result<(u64, u32)> {
    let mut msi_address = 0u64;
    let mut msi_data = 0u32;

    WHvMapVpciDeviceInterrupt(
        partition,
        logical_device_id,
        index,
        message_count,
        target,
        &mut msi_address,
        &mut msi_data,
    )?;

    Ok((msi_address, msi_data))
}

unsafe fn request_vpci_interrupt(
    partition: WHV_PARTITION_HANDLE,
    logical_device_id: u64,
    msi_address: u64,
    msi_data: u32,
) -> windows::core::Result<()> {
    WHvRequestVpciDeviceInterrupt(partition, logical_device_id, msi_address, msi_data)
}
```

### Read/Write VPCI Registers

```rust
unsafe fn read_vpci_register(
    partition: WHV_PARTITION_HANDLE,
    logical_device_id: u64,
    register: &WHV_VPCI_DEVICE_REGISTER,
) -> windows::core::Result<u64> {
    let mut data = 0u64;
    WHvReadVpciDeviceRegister(
        partition,
        logical_device_id,
        register,
        &mut data as *mut _ as *mut _,
    )?;
    Ok(data)
}

unsafe fn write_vpci_register(
    partition: WHV_PARTITION_HANDLE,
    logical_device_id: u64,
    register: &WHV_VPCI_DEVICE_REGISTER,
    data: u64,
) -> windows::core::Result<()> {
    WHvWriteVpciDeviceRegister(
        partition,
        logical_device_id,
        register,
        &data as *const _ as *const _,
    )
}
```

## Host Device Virtualization (HDV)

HDV allows implementing virtual devices in the host process using callbacks.

### Initialize Device Host

```rust
use windows::Win32::System::HostComputeSystem::HCS_SYSTEM;

unsafe fn init_device_host(
    compute_system: HCS_SYSTEM,
) -> windows::core::Result<*mut core::ffi::c_void> {
    let mut device_host_handle = std::ptr::null_mut();

    HdvInitializeDeviceHost(compute_system, &mut device_host_handle)?;

    Ok(device_host_handle)
}

unsafe fn init_device_host_ex(
    compute_system: HCS_SYSTEM,
    flags: HDV_DEVICE_HOST_FLAGS,
) -> windows::core::Result<*mut core::ffi::c_void> {
    let mut device_host_handle = std::ptr::null_mut();

    HdvInitializeDeviceHostEx(compute_system, flags, &mut device_host_handle)?;

    Ok(device_host_handle)
}
```

### Teardown Device Host

```rust
unsafe fn teardown_device_host(
    device_host_handle: *const core::ffi::c_void,
) -> windows::core::Result<()> {
    HdvTeardownDeviceHost(device_host_handle)
}
```

### Create Device Instance

```rust
unsafe fn create_pci_device(
    device_host: *const core::ffi::c_void,
    class_id: &GUID,
    instance_id: &GUID,
    interface: &HDV_PCI_DEVICE_INTERFACE,
    context: *const core::ffi::c_void,
) -> windows::core::Result<*mut core::ffi::c_void> {
    let mut device_handle = std::ptr::null_mut();

    HdvCreateDeviceInstance(
        device_host,
        HdvDeviceTypePCI,
        class_id,
        instance_id,
        interface as *const _ as *const _,
        Some(context),
        &mut device_handle,
    )?;

    Ok(device_handle)
}
```

### PCI Device Interface

Implement callback functions for PCI device emulation:

```rust
// Device interface structure
pub struct HDV_PCI_DEVICE_INTERFACE {
    pub Version: HDV_PCI_INTERFACE_VERSION,
    pub Initialize: HDV_PCI_DEVICE_INITIALIZE,
    pub Teardown: HDV_PCI_DEVICE_TEARDOWN,
    pub SetConfiguration: HDV_PCI_DEVICE_SET_CONFIGURATION,
    pub GetDetails: HDV_PCI_DEVICE_GET_DETAILS,
    pub Start: HDV_PCI_DEVICE_START,
    pub Stop: HDV_PCI_DEVICE_STOP,
    pub ReadConfigSpace: HDV_PCI_READ_CONFIG_SPACE,
    pub WriteConfigSpace: HDV_PCI_WRITE_CONFIG_SPACE,
    pub ReadInterceptedMemory: HDV_PCI_READ_INTERCEPTED_MEMORY,
    pub WriteInterceptedMemory: HDV_PCI_WRITE_INTERCEPTED_MEMORY,
}
```

### Callback Type Signatures

```rust
// Initialize callback
pub type HDV_PCI_DEVICE_INITIALIZE = Option<
    unsafe extern "system" fn(
        devicecontext: *const core::ffi::c_void,
    ) -> HRESULT
>;

// Teardown callback
pub type HDV_PCI_DEVICE_TEARDOWN = Option<
    unsafe extern "system" fn(
        devicecontext: *const core::ffi::c_void,
    )
>;

// Get device details
pub type HDV_PCI_DEVICE_GET_DETAILS = Option<
    unsafe extern "system" fn(
        devicecontext: *const core::ffi::c_void,
        pnpid: *mut HDV_PCI_PNP_ID,
        probedbarscount: u32,
        probedbars: *mut u32,
    ) -> HRESULT
>;

// Read config space
pub type HDV_PCI_READ_CONFIG_SPACE = Option<
    unsafe extern "system" fn(
        devicecontext: *const core::ffi::c_void,
        offset: u32,
        value: *mut u32,
    ) -> HRESULT
>;

// Write config space
pub type HDV_PCI_WRITE_CONFIG_SPACE = Option<
    unsafe extern "system" fn(
        devicecontext: *const core::ffi::c_void,
        offset: u32,
        value: u32,
    ) -> HRESULT
>;

// Read MMIO
pub type HDV_PCI_READ_INTERCEPTED_MEMORY = Option<
    unsafe extern "system" fn(
        devicecontext: *const core::ffi::c_void,
        barindex: HDV_PCI_BAR_SELECTOR,
        offset: u64,
        length: u64,
        value: *mut u8,
    ) -> HRESULT
>;

// Write MMIO
pub type HDV_PCI_WRITE_INTERCEPTED_MEMORY = Option<
    unsafe extern "system" fn(
        devicecontext: *const core::ffi::c_void,
        barindex: HDV_PCI_BAR_SELECTOR,
        offset: u64,
        length: u64,
        value: *const u8,
    ) -> HRESULT
>;
```

### Guest Memory Access

```rust
// Read guest memory
unsafe fn read_guest_memory(
    requestor: *const core::ffi::c_void,
    guest_physical_address: u64,
    buffer: &mut [u8],
) -> windows::core::Result<()> {
    HdvReadGuestMemory(requestor, guest_physical_address, buffer)
}

// Write guest memory
unsafe fn write_guest_memory(
    requestor: *const core::ffi::c_void,
    guest_physical_address: u64,
    buffer: &[u8],
) -> windows::core::Result<()> {
    HdvWriteGuestMemory(requestor, guest_physical_address, buffer)
}
```

### Guest Memory Aperture

Map guest memory directly into host address space:

```rust
unsafe fn create_memory_aperture(
    requestor: *const core::ffi::c_void,
    guest_physical_address: u64,
    byte_count: u32,
    write_protected: bool,
) -> windows::core::Result<*mut core::ffi::c_void> {
    let mut mapped_address = std::ptr::null_mut();

    HdvCreateGuestMemoryAperture(
        requestor,
        guest_physical_address,
        byte_count,
        write_protected,
        &mut mapped_address,
    )?;

    Ok(mapped_address)
}

unsafe fn destroy_memory_aperture(
    requestor: *const core::ffi::c_void,
    mapped_address: *const core::ffi::c_void,
) -> windows::core::Result<()> {
    HdvDestroyGuestMemoryAperture(requestor, mapped_address)
}
```

### Section-Backed MMIO

```rust
unsafe fn create_section_backed_mmio(
    requestor: *const core::ffi::c_void,
    bar_index: HDV_PCI_BAR_SELECTOR,
    offset_in_pages: u64,
    length_in_pages: u64,
    flags: HDV_MMIO_MAPPING_FLAGS,
    section_handle: HANDLE,
    section_offset_in_pages: u64,
) -> windows::core::Result<()> {
    HdvCreateSectionBackedMmioRange(
        requestor,
        bar_index,
        offset_in_pages,
        length_in_pages,
        flags,
        section_handle,
        section_offset_in_pages,
    )
}

unsafe fn destroy_section_backed_mmio(
    requestor: *const core::ffi::c_void,
    bar_index: HDV_PCI_BAR_SELECTOR,
    offset_in_pages: u64,
) -> windows::core::Result<()> {
    HdvDestroySectionBackedMmioRange(requestor, bar_index, offset_in_pages)
}
```

### Doorbell Registration

Register event notification for guest writes to specific BAR offset:

```rust
unsafe fn register_doorbell(
    requestor: *const core::ffi::c_void,
    bar_index: HDV_PCI_BAR_SELECTOR,
    bar_offset: u64,
    trigger_value: u64,
    flags: u64,
    doorbell_event: HANDLE,
) -> windows::core::Result<()> {
    HdvRegisterDoorbell(
        requestor,
        bar_index,
        bar_offset,
        trigger_value,
        flags,
        doorbell_event,
    )
}

unsafe fn unregister_doorbell(
    requestor: *const core::ffi::c_void,
    bar_index: HDV_PCI_BAR_SELECTOR,
    bar_offset: u64,
    trigger_value: u64,
    flags: u64,
) -> windows::core::Result<()> {
    HdvUnregisterDoorbell(requestor, bar_index, bar_offset, trigger_value, flags)
}
```

### Deliver Guest Interrupt

```rust
unsafe fn deliver_guest_interrupt(
    requestor: *const core::ffi::c_void,
    msi_address: u64,
    msi_data: u32,
) -> windows::core::Result<()> {
    HdvDeliverGuestInterrupt(requestor, msi_address, msi_data)
}
```

## PCI BAR Selectors

```rust
pub const HDV_PCI_BAR0: HDV_PCI_BAR_SELECTOR = HDV_PCI_BAR_SELECTOR(0);
pub const HDV_PCI_BAR1: HDV_PCI_BAR_SELECTOR = HDV_PCI_BAR_SELECTOR(1);
pub const HDV_PCI_BAR2: HDV_PCI_BAR_SELECTOR = HDV_PCI_BAR_SELECTOR(2);
pub const HDV_PCI_BAR3: HDV_PCI_BAR_SELECTOR = HDV_PCI_BAR_SELECTOR(3);
pub const HDV_PCI_BAR4: HDV_PCI_BAR_SELECTOR = HDV_PCI_BAR_SELECTOR(4);
pub const HDV_PCI_BAR5: HDV_PCI_BAR_SELECTOR = HDV_PCI_BAR_SELECTOR(5);
pub const HDV_PCI_BAR_COUNT: u32 = 6;
```

## Doorbell Flags

```rust
pub const HDV_DOORBELL_FLAG_TRIGGER_SIZE_ANY: HDV_DOORBELL_FLAGS = HDV_DOORBELL_FLAGS(0);
pub const HDV_DOORBELL_FLAG_TRIGGER_SIZE_BYTE: HDV_DOORBELL_FLAGS = HDV_DOORBELL_FLAGS(1);
pub const HDV_DOORBELL_FLAG_TRIGGER_SIZE_WORD: HDV_DOORBELL_FLAGS = HDV_DOORBELL_FLAGS(2);
pub const HDV_DOORBELL_FLAG_TRIGGER_SIZE_DWORD: HDV_DOORBELL_FLAGS = HDV_DOORBELL_FLAGS(3);
pub const HDV_DOORBELL_FLAG_TRIGGER_SIZE_QWORD: HDV_DOORBELL_FLAGS = HDV_DOORBELL_FLAGS(4);
pub const HDV_DOORBELL_FLAG_TRIGGER_ANY_VALUE: HDV_DOORBELL_FLAGS = HDV_DOORBELL_FLAGS(-2147483648);
```

## Complete Virtual Device Example

```rust
use windows::Win32::System::Hypervisor::*;
use windows::core::HRESULT;
use std::sync::atomic::{AtomicU32, Ordering};

// Device context
struct VirtualSerialPort {
    base_address: u64,
    interrupt_line: u8,
    data_register: AtomicU32,
}

// Callback implementations
unsafe extern "system" fn device_initialize(
    context: *const core::ffi::c_void,
) -> HRESULT {
    // Initialize device state
    HRESULT(0)
}

unsafe extern "system" fn device_teardown(
    context: *const core::ffi::c_void,
) {
    // Cleanup
}

unsafe extern "system" fn device_get_details(
    context: *const core::ffi::c_void,
    pnpid: *mut HDV_PCI_PNP_ID,
    probedbarscount: u32,
    probedbars: *mut u32,
) -> HRESULT {
    // Fill in PNP ID
    (*pnpid).VendorID = 0x1234;
    (*pnpid).DeviceID = 0x5678;
    (*pnpid).RevisionID = 0x01;
    (*pnpid).BaseClass = 0x07;  // Serial controller
    (*pnpid).SubClass = 0x00;
    (*pnpid).ProgIf = 0x00;

    // Configure BAR0 as 16 bytes I/O
    if probedbarscount > 0 {
        *probedbars = 0x11;  // I/O space, 16 bytes
    }

    HRESULT(0)
}

unsafe extern "system" fn device_read_mmio(
    context: *const core::ffi::c_void,
    barindex: HDV_PCI_BAR_SELECTOR,
    offset: u64,
    length: u64,
    value: *mut u8,
) -> HRESULT {
    let device = &*(context as *const VirtualSerialPort);

    if barindex.0 == 0 && offset == 0 && length == 1 {
        // Read data register
        *value = device.data_register.load(Ordering::SeqCst) as u8;
    }

    HRESULT(0)
}

unsafe extern "system" fn device_write_mmio(
    context: *const core::ffi::c_void,
    barindex: HDV_PCI_BAR_SELECTOR,
    offset: u64,
    length: u64,
    value: *const u8,
) -> HRESULT {
    let device = &*(context as *const VirtualSerialPort);

    if barindex.0 == 0 && offset == 0 && length == 1 {
        // Write data register - output character
        print!("{}", *value as char);
    }

    HRESULT(0)
}
```
