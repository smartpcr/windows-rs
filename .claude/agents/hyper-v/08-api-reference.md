# Hyper-V API Reference

## Complete Function List

### Partition Management (winhvplatform.dll)

| Function | Description |
|----------|-------------|
| `WHvCreatePartition` | Create a new partition |
| `WHvSetupPartition` | Finalize partition configuration |
| `WHvDeletePartition` | Delete a partition |
| `WHvResetPartition` | Reset partition state |
| `WHvGetPartitionProperty` | Get partition property |
| `WHvSetPartitionProperty` | Set partition property |
| `WHvGetPartitionCounters` | Get performance counters |
| `WHvSuspendPartitionTime` | Suspend partition time |
| `WHvResumePartitionTime` | Resume partition time |
| `WHvStartPartitionMigration` | Begin partition migration |
| `WHvAcceptPartitionMigration` | Accept migration in target process |
| `WHvCompletePartitionMigration` | Complete migration |
| `WHvCancelPartitionMigration` | Cancel migration |

### Virtual Processor (winhvplatform.dll)

| Function | Description |
|----------|-------------|
| `WHvCreateVirtualProcessor` | Create VP with basic flags |
| `WHvCreateVirtualProcessor2` | Create VP with properties |
| `WHvDeleteVirtualProcessor` | Delete VP |
| `WHvRunVirtualProcessor` | Run VP until exit |
| `WHvCancelRunVirtualProcessor` | Cancel VP execution |
| `WHvGetVirtualProcessorRegisters` | Read VP registers |
| `WHvSetVirtualProcessorRegisters` | Write VP registers |
| `WHvGetVirtualProcessorState` | Get VP state |
| `WHvSetVirtualProcessorState` | Set VP state |
| `WHvGetVirtualProcessorInterruptControllerState` | Get APIC state |
| `WHvGetVirtualProcessorInterruptControllerState2` | Get extended APIC state |
| `WHvSetVirtualProcessorInterruptControllerState` | Set APIC state |
| `WHvSetVirtualProcessorInterruptControllerState2` | Set extended APIC state |
| `WHvGetVirtualProcessorXsaveState` | Get XSAVE state |
| `WHvSetVirtualProcessorXsaveState` | Set XSAVE state |
| `WHvGetVirtualProcessorCpuidOutput` | Get CPUID output |
| `WHvGetVirtualProcessorCounters` | Get VP counters |

### Memory Management (winhvplatform.dll)

| Function | Description |
|----------|-------------|
| `WHvMapGpaRange` | Map host memory to GPA |
| `WHvMapGpaRange2` | Map from process handle |
| `WHvUnmapGpaRange` | Unmap GPA range |
| `WHvReadGpaRange` | Read from GPA |
| `WHvWriteGpaRange` | Write to GPA |
| `WHvTranslateGva` | Translate GVA to GPA |
| `WHvQueryGpaRangeDirtyBitmap` | Get dirty pages |
| `WHvAdviseGpaRange` | Advise on GPA usage |

### Interrupts & Events (winhvplatform.dll)

| Function | Description |
|----------|-------------|
| `WHvRequestInterrupt` | Inject interrupt |
| `WHvGetInterruptTargetVpSet` | Get interrupt targets |
| `WHvPostVirtualProcessorSynicMessage` | Post SYNIC message |
| `WHvSignalVirtualProcessorSynicEvent` | Signal SYNIC event |
| `WHvCreateNotificationPort` | Create notification port |
| `WHvDeleteNotificationPort` | Delete notification port |
| `WHvSetNotificationPortProperty` | Configure notification port |
| `WHvRegisterPartitionDoorbellEvent` | Register doorbell |
| `WHvUnregisterPartitionDoorbellEvent` | Unregister doorbell |
| `WHvCreateTrigger` | Create trigger |
| `WHvDeleteTrigger` | Delete trigger |
| `WHvUpdateTriggerParameters` | Update trigger config |

### Capability & Configuration (winhvplatform.dll)

| Function | Description |
|----------|-------------|
| `WHvGetCapability` | Query platform capability |

### VPCI Devices (winhvplatform.dll)

| Function | Description |
|----------|-------------|
| `WHvAllocateVpciResource` | Allocate VPCI resource |
| `WHvCreateVpciDevice` | Create VPCI device |
| `WHvDeleteVpciDevice` | Delete VPCI device |
| `WHvGetVpciDeviceProperty` | Get device property |
| `WHvGetVpciDeviceNotification` | Get device notification |
| `WHvGetVpciDeviceInterruptTarget` | Get interrupt target |
| `WHvMapVpciDeviceInterrupt` | Map device interrupt |
| `WHvUnmapVpciDeviceInterrupt` | Unmap device interrupt |
| `WHvMapVpciDeviceMmioRanges` | Map MMIO ranges |
| `WHvUnmapVpciDeviceMmioRanges` | Unmap MMIO ranges |
| `WHvReadVpciDeviceRegister` | Read device register |
| `WHvWriteVpciDeviceRegister` | Write device register |
| `WHvSetVpciDevicePowerState` | Set device power state |
| `WHvRequestVpciDeviceInterrupt` | Request device interrupt |
| `WHvRetargetVpciDeviceInterrupt` | Retarget interrupt |

### Emulation (winhvemulation.dll)

| Function | Description |
|----------|-------------|
| `WHvEmulatorCreateEmulator` | Create emulator |
| `WHvEmulatorDestroyEmulator` | Destroy emulator |
| `WHvEmulatorTryIoEmulation` | Emulate I/O access |
| `WHvEmulatorTryMmioEmulation` | Emulate MMIO access |

### Host Device Virtualization (vmdevicehost.dll)

| Function | Description |
|----------|-------------|
| `HdvInitializeDeviceHost` | Initialize device host |
| `HdvInitializeDeviceHostEx` | Initialize with flags |
| `HdvTeardownDeviceHost` | Teardown device host |
| `HdvCreateDeviceInstance` | Create device instance |
| `HdvReadGuestMemory` | Read guest memory |
| `HdvWriteGuestMemory` | Write guest memory |
| `HdvCreateGuestMemoryAperture` | Map guest memory |
| `HdvDestroyGuestMemoryAperture` | Unmap aperture |
| `HdvCreateSectionBackedMmioRange` | Create MMIO range |
| `HdvDestroySectionBackedMmioRange` | Destroy MMIO range |
| `HdvRegisterDoorbell` | Register doorbell |
| `HdvUnregisterDoorbell` | Unregister doorbell |
| `HdvDeliverGuestInterrupt` | Deliver interrupt |

### Saved State (vmsavedstatedumpprovider.dll)

| Function | Description |
|----------|-------------|
| `LoadSavedStateFile` | Load .vmrs file |
| `LoadSavedStateFiles` | Load .bin + .vsv files |
| `LocateSavedStateFiles` | Find saved state files |
| `ReleaseSavedStateFiles` | Release handle |
| `GetVpCount` | Get VP count |
| `GetArchitecture` | Get VP architecture |
| `GetRegisterValue` | Read register |
| `GetPagingMode` | Get paging mode |
| `ForcePagingMode` | Force paging mode |
| `ReadGuestPhysicalAddress` | Read physical memory |
| `GuestVirtualAddressToPhysicalAddress` | Translate GVA |
| `GetGuestPhysicalMemoryChunks` | Get memory layout |
| `GetGuestRawSavedMemorySize` | Get raw memory size |
| `ReadGuestRawSavedMemory` | Read raw memory |
| `GetGuestOsInfo` | Get guest OS info |
| `InKernelSpace` | Check kernel mode |
| `IsNestedVirtualizationEnabled` | Check nested virt |
| `ForceNestedHostMode` | Force nested mode |
| `GetActiveVirtualTrustLevel` | Get active VTL |
| `ForceActiveVirtualTrustLevel` | Force VTL |
| `GetEnabledVirtualTrustLevels` | Get enabled VTLs |
| `LoadSavedStateSymbolProvider` | Load symbols |
| `ReleaseSavedStateSymbolProvider` | Release symbols |
| `LoadSavedStateModuleSymbols` | Load module symbols |
| `GetSavedStateSymbolTypeSize` | Get type size |
| `GetSavedStateSymbolFieldInfo` | Get field info |
| `FindSavedStateSymbolFieldInType` | Find field in type |
| `ResolveSavedStateGlobalVariableAddress` | Resolve global |
| `ReadSavedStateGlobalVariable` | Read global |
| `CallStackUnwind` | Unwind call stack |
| `ScanMemoryForDosImages` | Scan for images |
| `ApplyGuestMemoryFix` | Apply memory fix |

---

## Key Enumerations

### WHV_CAPABILITY_CODE

```rust
WHvCapabilityCodeHypervisorPresent       = 0
WHvCapabilityCodeFeatures                = 1
WHvCapabilityCodeExtendedVmExits         = 2
WHvCapabilityCodeExceptionExitBitmap     = 3
WHvCapabilityCodeX64MsrExitBitmap        = 4
WHvCapabilityCodeGpaRangePopulateFlags   = 5
WHvCapabilityCodeSchedulerFeatures       = 6
WHvCapabilityCodeProcessorVendor         = 4096
WHvCapabilityCodeProcessorFeatures       = 4097
WHvCapabilityCodeProcessorClFlushSize    = 4098
WHvCapabilityCodeProcessorXsaveFeatures  = 4099
WHvCapabilityCodeProcessorClockFrequency = 4100
WHvCapabilityCodeInterruptClockFrequency = 4101
WHvCapabilityCodeProcessorFeaturesBanks  = 4102
WHvCapabilityCodeProcessorFrequencyCap   = 4103
WHvCapabilityCodeSyntheticProcessorFeaturesBanks = 4104
WHvCapabilityCodeProcessorPerfmonFeatures = 4105
```

### WHV_PARTITION_PROPERTY_CODE

```rust
WHvPartitionPropertyCodeExtendedVmExits            = 1
WHvPartitionPropertyCodeExceptionExitBitmap        = 2
WHvPartitionPropertyCodeSeparateSecurityDomain     = 3
WHvPartitionPropertyCodeNestedVirtualization       = 4
WHvPartitionPropertyCodeX64MsrExitBitmap           = 5
WHvPartitionPropertyCodePrimaryNumaNode            = 6
WHvPartitionPropertyCodeCpuReserve                 = 7
WHvPartitionPropertyCodeCpuCap                     = 8
WHvPartitionPropertyCodeCpuWeight                  = 9
WHvPartitionPropertyCodeCpuGroupId                 = 10
WHvPartitionPropertyCodeProcessorFrequencyCap     = 11
WHvPartitionPropertyCodeAllowDeviceAssignment     = 12
WHvPartitionPropertyCodeDisableSmt                = 13
WHvPartitionPropertyCodeProcessorFeatures         = 4097
WHvPartitionPropertyCodeProcessorClFlushSize      = 4098
WHvPartitionPropertyCodeCpuidExitList             = 4099
WHvPartitionPropertyCodeCpuidResultList           = 4100
WHvPartitionPropertyCodeLocalApicEmulationMode    = 4101
WHvPartitionPropertyCodeProcessorXsaveFeatures    = 4102
WHvPartitionPropertyCodeProcessorClockFrequency   = 4103
WHvPartitionPropertyCodeInterruptClockFrequency   = 4104
WHvPartitionPropertyCodeApicRemoteReadSupport     = 4105
WHvPartitionPropertyCodeProcessorFeaturesBanks    = 4106
WHvPartitionPropertyCodeReferenceTime             = 4107
WHvPartitionPropertyCodeSyntheticProcessorFeaturesBanks = 4108
WHvPartitionPropertyCodeCpuidResultList2          = 4109
WHvPartitionPropertyCodeProcessorPerfmonFeatures  = 4110
WHvPartitionPropertyCodeMsrActionList             = 4111
WHvPartitionPropertyCodeUnimplementedMsrAction    = 4112
WHvPartitionPropertyCodeProcessorCount            = 8191
```

### WHV_RUN_VP_EXIT_REASON

```rust
WHvRunVpExitReasonNone                        = 0
WHvRunVpExitReasonMemoryAccess                = 1
WHvRunVpExitReasonX64IoPortAccess             = 2
WHvRunVpExitReasonUnrecoverableException      = 4
WHvRunVpExitReasonInvalidVpRegisterValue      = 5
WHvRunVpExitReasonUnsupportedFeature          = 6
WHvRunVpExitReasonX64InterruptWindow          = 7
WHvRunVpExitReasonX64Halt                     = 8
WHvRunVpExitReasonX64ApicEoi                  = 9
WHvRunVpExitReasonSynicSintDeliverable        = 10
WHvRunVpExitReasonX64MsrAccess                = 4096
WHvRunVpExitReasonX64Cpuid                    = 4097
WHvRunVpExitReasonException                   = 4098
WHvRunVpExitReasonX64Rdtsc                    = 4099
WHvRunVpExitReasonX64ApicSmiTrap              = 4100
WHvRunVpExitReasonHypercall                   = 4101
WHvRunVpExitReasonX64ApicInitSipiTrap         = 4102
WHvRunVpExitReasonX64ApicWriteTrap            = 4103
WHvRunVpExitReasonCanceled                    = 8193
```

### WHV_REGISTER_NAME (x64 subset)

```rust
// General Purpose
WHvX64RegisterRax, WHvX64RegisterRcx, WHvX64RegisterRdx, WHvX64RegisterRbx
WHvX64RegisterRsp, WHvX64RegisterRbp, WHvX64RegisterRsi, WHvX64RegisterRdi
WHvX64RegisterR8 - WHvX64RegisterR15
WHvX64RegisterRip, WHvX64RegisterRflags

// Control Registers
WHvX64RegisterCr0, WHvX64RegisterCr2, WHvX64RegisterCr3, WHvX64RegisterCr4, WHvX64RegisterCr8

// Segment Registers
WHvX64RegisterCs, WHvX64RegisterDs, WHvX64RegisterEs
WHvX64RegisterFs, WHvX64RegisterGs, WHvX64RegisterSs

// Descriptor Tables
WHvX64RegisterGdtr, WHvX64RegisterIdtr, WHvX64RegisterLdtr, WHvX64RegisterTr

// System Registers
WHvX64RegisterEfer, WHvX64RegisterTsc, WHvX64RegisterPat

// APIC Registers
WHvX64RegisterApicBase, WHvX64RegisterApicId, etc.

// SYNIC Registers
WHvRegisterScontrol, WHvRegisterSiefp, WHvRegisterSimp
WHvRegisterSint0 - WHvRegisterSint15
```

---

## Key Structures

### WHV_PARTITION_HANDLE

Opaque handle to a partition.

### WHV_RUN_VP_EXIT_CONTEXT

```rust
pub struct WHV_RUN_VP_EXIT_CONTEXT {
    pub ExitReason: WHV_RUN_VP_EXIT_REASON,
    pub Reserved: u32,
    pub VpContext: WHV_VP_EXIT_CONTEXT,
    pub Anonymous: WHV_RUN_VP_EXIT_CONTEXT_0,  // Union
}
```

### WHV_VP_EXIT_CONTEXT

```rust
pub struct WHV_VP_EXIT_CONTEXT {
    pub ExecutionState: WHV_X64_VP_EXECUTION_STATE,
    pub InstructionLength: u8,
    pub Cr8: u8,
    pub Reserved: u8,
    pub Reserved2: u32,
    pub Cs: WHV_X64_SEGMENT_REGISTER,
    pub Rip: u64,
    pub Rflags: u64,
}
```

### WHV_MEMORY_ACCESS_CONTEXT

```rust
pub struct WHV_MEMORY_ACCESS_CONTEXT {
    pub InstructionByteCount: u8,
    pub Reserved: [u8; 3],
    pub InstructionBytes: [u8; 16],
    pub AccessInfo: WHV_MEMORY_ACCESS_INFO,
    pub Gpa: u64,
    pub Gva: u64,
}
```

### WHV_X64_IO_PORT_ACCESS_CONTEXT

```rust
pub struct WHV_X64_IO_PORT_ACCESS_CONTEXT {
    pub InstructionByteCount: u8,
    pub Reserved: [u8; 3],
    pub InstructionBytes: [u8; 16],
    pub AccessInfo: WHV_X64_IO_PORT_ACCESS_INFO,
    pub PortNumber: u16,
    pub Reserved2: [u16; 3],
    pub Rax: u64,
    pub Rcx: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub Ds: WHV_X64_SEGMENT_REGISTER,
    pub Es: WHV_X64_SEGMENT_REGISTER,
}
```

### WHV_REGISTER_VALUE

```rust
pub union WHV_REGISTER_VALUE {
    pub Reg128: WHV_UINT128,
    pub Reg64: u64,
    pub Reg32: u32,
    pub Reg16: u16,
    pub Reg8: u8,
    pub Fp: WHV_X64_FP_REGISTER,
    pub FpControlStatus: WHV_X64_FP_CONTROL_STATUS_REGISTER,
    pub XmmControlStatus: WHV_X64_XMM_CONTROL_STATUS_REGISTER,
    pub Segment: WHV_X64_SEGMENT_REGISTER,
    pub Table: WHV_X64_TABLE_REGISTER,
    pub InterruptState: WHV_X64_INTERRUPT_STATE_REGISTER,
    pub PendingInterruption: WHV_X64_PENDING_INTERRUPTION_REGISTER,
    pub DeliverabilityNotifications: WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER,
    pub ExceptionEvent: WHV_X64_PENDING_EXCEPTION_EVENT,
    pub ExtIntEvent: WHV_X64_PENDING_EXT_INT_EVENT,
}
```

### WHV_INTERRUPT_CONTROL

```rust
pub struct WHV_INTERRUPT_CONTROL {
    pub Type: WHV_INTERRUPT_TYPE,
    pub DestinationMode: WHV_INTERRUPT_DESTINATION_MODE,
    pub TriggerMode: WHV_INTERRUPT_TRIGGER_MODE,
    pub Reserved: u8,
    pub Destination: u32,
    pub Vector: u32,
    pub Reserved2: u32,
}
```

---

## Constants

```rust
pub const WHV_ANY_VP: u32 = 0xFFFFFFFF;
pub const WHV_SYNIC_MESSAGE_SIZE: u32 = 256;
pub const WHV_HYPERCALL_CONTEXT_MAX_XMM_REGISTERS: u32 = 6;
pub const WHV_PROCESSOR_FEATURES_BANKS_COUNT: u32 = 2;
pub const WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_COUNT: u32 = 1;
pub const WHV_READ_WRITE_GPA_RANGE_MAX_SIZE: u32 = 16;
pub const WHV_MAX_DEVICE_ID_SIZE_IN_CHARS: u32 = 200;
pub const HDV_PCI_BAR_COUNT: u32 = 6;
```

---

## Error Codes

Common HRESULT values:

| Code | Name | Description |
|------|------|-------------|
| 0x00000000 | S_OK | Success |
| 0x80070005 | E_ACCESSDENIED | Access denied |
| 0x80070057 | E_INVALIDARG | Invalid argument |
| 0x8007000E | E_OUTOFMEMORY | Out of memory |
| 0x80004005 | E_FAIL | Unspecified failure |
| 0xC0350000 | WHV_E_UNKNOWN_CAPABILITY | Unknown capability |
| 0xC0350001 | WHV_E_INSUFFICIENT_BUFFER | Buffer too small |
| 0xC0350002 | WHV_E_UNKNOWN_PROPERTY | Unknown property |
| 0xC0350003 | WHV_E_INVALID_PARTITION_CONFIG | Invalid config |
| 0xC0350004 | WHV_E_GPA_RANGE_NOT_FOUND | GPA not mapped |
| 0xC0350005 | WHV_E_VP_ALREADY_EXISTS | VP exists |
| 0xC0350006 | WHV_E_VP_DOES_NOT_EXIST | VP not found |
| 0xC0350007 | WHV_E_INVALID_VP_STATE | Invalid VP state |
| 0xC0350008 | WHV_E_INVALID_VP_REGISTER_NAME | Invalid register |
