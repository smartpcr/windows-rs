# Hyper-V WMI API Reference

This document provides comprehensive API reference for Hyper-V WMI operations. The APIs are exposed through the `root\virtualization\v2` WMI namespace.

## WMI Namespace

```
root\virtualization\v2
```

## Core Classes

### Msvm_ComputerSystem

Represents a virtual machine or the host computer system.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | VM name or host name |
| `ElementName` | `string` | Display name |
| `EnabledState` | `uint16` | Current state (2=Enabled, 3=Disabled, 32768=Paused, 32769=Suspended, 32770=Starting, 32771=Snapshotting, 32773=Saving, 32774=Stopping, 32776=Pausing, 32777=Resuming) |
| `HealthState` | `uint16` | Health status |
| `ProcessID` | `uint32` | VM worker process ID |
| `OnTimeInMilliseconds` | `uint64` | Total VM uptime |
| `NumberOfNumaNodes` | `uint16` | NUMA node count |
| `EnhancedSessionModeState` | `uint16` | Enhanced session mode status |
| `ReplicationState` | `uint16` | Hyper-V Replica state |
| `ReplicationHealth` | `uint16` | Replica health status |
| `ReplicationMode` | `uint16` | Primary/Replica mode |
| `HwThreadsPerCoreRealized` | `uint32` | Hardware threads per core |
| `TimeOfLastConfigurationChange` | `string` | Last config change timestamp |
| `LastReplicationTime` | `string` | Last replication timestamp |
| `LastSuccessfulBackupTime` | `string` | Last backup timestamp |

**Methods:**

| Method | Parameters | Description |
|--------|------------|-------------|
| `RequestStateChange` | `RequestedState: uint16, Job: CIM_ConcreteJob` | Change VM state (start/stop/pause/resume) |
| `RequestReplicationStateChange` | `RequestedState: uint16, TimeoutPeriod: string, Job: CIM_ConcreteJob` | Change replication state |
| `RequestReplicationStateChangeEx` | `ReplicationRelationship: string, RequestedState: uint16, TimeoutPeriod: string, Job: CIM_ConcreteJob` | Extended replication state change |
| `InjectNonMaskableInterrupt` | `Job: CIM_ConcreteJob` | Inject NMI |
| `InjectNonMaskableInterruptEx` | `Vtl: uint8, Job: CIM_ConcreteJob` | Inject NMI to specific VTL |
| `RequestCustomRestore` | `RestoreSettings: string, Job: CIM_ConcreteJob` | Custom restore operation |

**Related Classes:**
- `Msvm_VirtualSystemSettingData` - VM configuration
- `Msvm_ResourcePool` - Resource pools
- `Msvm_Processor` - vCPU info
- `Msvm_Memory` - Memory info
- `Msvm_DiskDrive` - Virtual disks

---

### Msvm_VirtualSystemManagementService

Primary service for VM lifecycle management.

**Methods:**

#### VM Lifecycle

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `DefineSystem` | `SystemSettings: string, ResourceSettings: string[], ReferenceConfiguration: CIM_VirtualSystemSettingData` | `ResultingSystem: CIM_ComputerSystem, Job: CIM_ConcreteJob` | Create new VM |
| `DestroySystem` | `AffectedSystem: CIM_ComputerSystem` | `Job: CIM_ConcreteJob` | Delete VM |
| `ModifySystemSettings` | `SystemSettings: string` | `Job: CIM_ConcreteJob` | Modify VM settings |
| `DefinePlannedSystem` | `SystemSettings: string, ResourceSettings: string[], ReferenceConfiguration: CIM_VirtualSystemSettingData` | `ResultingSystem: CIM_ComputerSystem, Job: CIM_ConcreteJob` | Create planned VM |
| `ValidatePlannedSystem` | `PlannedSystem: Msvm_PlannedComputerSystem` | `Job: CIM_ConcreteJob` | Validate planned VM |
| `RealizePlannedSystem` | `PlannedSystem: Msvm_PlannedComputerSystem` | `ResultingSystem: CIM_ComputerSystem, Job: CIM_ConcreteJob` | Convert planned to real VM |
| `UpgradeSystemVersion` | `ComputerSystem: CIM_ComputerSystem, UpgradeSettingData: string` | `Job: CIM_ConcreteJob` | Upgrade VM version |

#### Resource Management

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `AddResourceSettings` | `AffectedConfiguration: CIM_VirtualSystemSettingData, ResourceSettings: string[]` | `ResultingResourceSettings: CIM_ResourceAllocationSettingData[], Job: CIM_ConcreteJob` | Add resources |
| `ModifyResourceSettings` | `ResourceSettings: string[]` | `ResultingResourceSettings: CIM_ResourceAllocationSettingData[], Job: CIM_ConcreteJob` | Modify resources |
| `RemoveResourceSettings` | `ResourceSettings: CIM_ResourceAllocationSettingData[]` | `Job: CIM_ConcreteJob` | Remove resources |

#### Boot Configuration

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `AddBootSourceSettings` | `AffectedConfiguration: CIM_VirtualSystemSettingData, BootSourceSettings: string[]` | `ResultingBootSourceSettings: CIM_SettingData[], Job: CIM_ConcreteJob` | Add boot sources |
| `RemoveBootSourceSettings` | `BootSourceSettings: CIM_SettingData[]` | `Job: CIM_ConcreteJob` | Remove boot sources |

#### Guest Services

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `AddGuestServiceSettings` | `AffectedConfiguration: CIM_VirtualSystemSettingData, GuestServiceSettings: string[]` | `ResultingGuestServiceSettings: CIM_SettingData[], Job: CIM_ConcreteJob` | Add guest services |
| `ModifyGuestServiceSettings` | `GuestServiceSettings: string[]` | `ResultingGuestServiceSettings: CIM_SettingData[], Job: CIM_ConcreteJob` | Modify guest services |
| `RemoveGuestServiceSettings` | `GuestServiceSettings: CIM_SettingData[]` | `Job: CIM_ConcreteJob` | Remove guest services |

#### Ethernet Port Features

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `AddFeatureSettings` | `AffectedConfiguration: Msvm_EthernetPortAllocationSettingData, FeatureSettings: string[]` | `ResultingFeatureSettings: Msvm_EthernetSwitchPortFeatureSettingData[], Job: CIM_ConcreteJob` | Add port features |
| `ModifyFeatureSettings` | `FeatureSettings: string[]` | `ResultingFeatureSettings: Msvm_EthernetSwitchPortFeatureSettingData[], Job: CIM_ConcreteJob` | Modify port features |
| `RemoveFeatureSettings` | `FeatureSettings: Msvm_EthernetSwitchPortFeatureSettingData[]` | `Job: CIM_ConcreteJob` | Remove port features |

#### System Components

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `AddSystemComponentSettings` | `AffectedConfiguration: Msvm_VirtualSystemSettingData, ComponentSettings: string[]` | `ResultingComponentSettings: Msvm_SystemComponentSettingData[], Job: CIM_ConcreteJob` | Add system components |
| `ModifySystemComponentSettings` | `ComponentSettings: string[]` | `ResultingComponentSettings: Msvm_SystemComponentSettingData[], Job: CIM_ConcreteJob` | Modify system components |
| `RemoveSystemComponentSettings` | `ComponentSettings: Msvm_SystemComponentSettingData[]` | `Job: CIM_ConcreteJob` | Remove system components |

#### Import/Export

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `ImportSystemDefinition` | `SystemDefinitionFile: string, SnapshotFolder: string, GenerateNewSystemIdentifier: bool` | `ImportedSystem: Msvm_PlannedComputerSystem, Job: CIM_ConcreteJob` | Import VM |
| `ImportSnapshotDefinitions` | `PlannedSystem: Msvm_PlannedComputerSystem, SnapshotFolder: string` | `ImportedSnapshots: Msvm_VirtualSystemSettingData[], Job: CIM_ConcreteJob` | Import snapshots |
| `ExportSystemDefinition` | `ComputerSystem: CIM_ComputerSystem, ExportDirectory: string, ExportSettingData: string` | `Job: CIM_ConcreteJob` | Export VM |

#### KVP (Key-Value Pair) Operations

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `AddKvpItems` | `TargetSystem: CIM_ComputerSystem, DataItems: string[]` | `Job: CIM_ConcreteJob` | Add KVP items |
| `ModifyKvpItems` | `TargetSystem: CIM_ComputerSystem, DataItems: string[]` | `Job: CIM_ConcreteJob` | Modify KVP items |
| `RemoveKvpItems` | `TargetSystem: CIM_ComputerSystem, DataItems: string[]` | `Job: CIM_ConcreteJob` | Remove KVP items |

#### Utility Methods

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `GetSummaryInformation` | `SettingData: CIM_VirtualSystemSettingData[], RequestedInformation: uint32[]` | `SummaryInformation: Msvm_SummaryInformationBase[]` | Get VM summary info |
| `GetDefinitionFileSummaryInformation` | `DefinitionFiles: string[]` | `SummaryInformation: Msvm_SummaryInformationBase[]` | Get definition file info |
| `GetVirtualSystemThumbnailImage` | `TargetSystem: CIM_VirtualSystemSettingData, WidthPixels: uint16, HeightPixels: uint16` | `ImageData: uint8[]` | Get VM thumbnail |
| `GetSizeOfSystemFiles` | `Vssd: CIM_VirtualSystemSettingData` | `Size: uint64` | Get system files size |
| `FormatError` | `Errors: string[]` | `ErrorMessage: string` | Format error messages |
| `ModifyServiceSettings` | `SettingData: string` | `Job: CIM_ConcreteJob` | Modify service settings |
| `ModifyDiskMergeSettings` | `SettingData: string` | `Job: CIM_ConcreteJob` | Modify disk merge settings |
| `GenerateWwpn` | `NumberOfWwpns: uint32` | `GeneratedWwpn: string[]` | Generate WWPNs for FC |
| `GetCurrentWwpnFromGenerator` | - | `CurrentWwpn: string` | Get current WWPN |
| `SetInitialMachineConfigurationData` | `TargetSystem: CIM_ComputerSystem, ImcData: uint8[]` | `Job: CIM_ConcreteJob` | Set initial machine config |
| `SetGuestNetworkAdapterConfiguration` | `ComputerSystem: CIM_ComputerSystem, NetworkConfiguration: string[]` | `Job: CIM_ConcreteJob` | Configure guest network |

#### Fibre Channel

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `AddFibreChannelChap` | `FcPortSettings: string[], SecretEncoding: uint16, SharedSecret: uint8[]` | - | Add FC CHAP auth |
| `RemoveFibreChannelChap` | `FcPortSettings: string[]` | - | Remove FC CHAP auth |

#### Network Diagnostics

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `TestNetworkConnection` | `TargetNetworkAdapter: Msvm_EthernetPortAllocationSettingData, IsSender: bool, SenderIP: string, ReceiverIP: string, ReceiverMac: string, IsolationId: uint32, SequenceNumber: uint32` | `RoundTripTime: uint32, Job: CIM_ConcreteJob` | Test network connectivity |
| `DiagnoseNetworkConnection` | `TargetNetworkAdapter: Msvm_EthernetPortAllocationSettingData, DiagnosticSettings: string` | `DiagnosticInformation: string, Job: CIM_ConcreteJob` | Diagnose network |

---

### Msvm_VirtualSystemSnapshotService

Manages VM checkpoints/snapshots.

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `CreateSnapshot` | `AffectedSystem: CIM_ComputerSystem, SnapshotSettings: string, SnapshotType: uint16` | `ResultingSnapshot: CIM_VirtualSystemSettingData, Job: CIM_ConcreteJob` | Create checkpoint |
| `ApplySnapshot` | `Snapshot: CIM_VirtualSystemSettingData` | `Job: CIM_ConcreteJob` | Apply/restore checkpoint |
| `DestroySnapshot` | `AffectedSnapshot: CIM_VirtualSystemSettingData` | `Job: CIM_ConcreteJob` | Delete checkpoint |
| `DestroySnapshotTree` | `SnapshotSettingData: CIM_VirtualSystemSettingData` | `Job: CIM_ConcreteJob` | Delete checkpoint tree |
| `ConvertToReferencePoint` | `AffectedSnapshot: CIM_VirtualSystemSettingData, ReferencePointSettings: string` | `ResultingReferencePoint: Msvm_VirtualSystemReferencePoint, Job: CIM_ConcreteJob` | Convert to reference point |

---

### Msvm_VirtualSystemMigrationService

Handles live migration operations.

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `MigrateVirtualSystemToHost` | `ComputerSystem: CIM_ComputerSystem, DestinationHost: string, MigrationSettingData: string, NewResourceSettingData: string[], NewSystemSettingData: string` | `Job: CIM_ConcreteJob` | Live migrate VM |
| `MigrateVirtualSystemToSystem` | `ComputerSystem: CIM_ComputerSystem, DestinationSystem: CIM_System, MigrationSettingData: string, NewResourceSettingData: string[], NewSystemSettingData: string` | `Job: CIM_ConcreteJob` | Migrate to system |
| `CheckVirtualSystemIsMigratable` | `ComputerSystem: CIM_ComputerSystem, DestinationHost: string, MigrationSettingData: string, NewResourceSettingData: string[], NewSystemSettingData: string` | `IsMigratable: bool` | Check migration compatibility |

---

### Msvm_VirtualEthernetSwitchManagementService

Manages virtual switches.

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `DefineSystem` | `SystemSettings: string, ResourceSettings: string[], ReferenceConfiguration: CIM_VirtualSystemSettingData` | `ResultingSystem: CIM_ComputerSystem, Job: CIM_ConcreteJob` | Create virtual switch |
| `DestroySystem` | `AffectedSystem: CIM_ComputerSystem` | `Job: CIM_ConcreteJob` | Delete virtual switch |
| `ModifySystemSettings` | `SystemSettings: string` | `Job: CIM_ConcreteJob` | Modify switch settings |
| `AddResourceSettings` | `AffectedConfiguration: CIM_VirtualSystemSettingData, ResourceSettings: string[]` | `ResultingResourceSettings: CIM_ResourceAllocationSettingData[], Job: CIM_ConcreteJob` | Add switch resources |
| `ModifyResourceSettings` | `ResourceSettings: string[]` | `ResultingResourceSettings: CIM_ResourceAllocationSettingData[], Job: CIM_ConcreteJob` | Modify switch resources |
| `RemoveResourceSettings` | `ResourceSettings: CIM_ResourceAllocationSettingData[]` | `Job: CIM_ConcreteJob` | Remove switch resources |
| `AddFeatureSettings` | `AffectedConfiguration: Msvm_EthernetPortAllocationSettingData, FeatureSettings: string[]` | `ResultingFeatureSettings: Msvm_FeatureSettingData[], Job: CIM_ConcreteJob` | Add switch features |
| `ModifyFeatureSettings` | `FeatureSettings: string[]` | `ResultingFeatureSettings: Msvm_FeatureSettingData[], Job: CIM_ConcreteJob` | Modify switch features |
| `RemoveFeatureSettings` | `FeatureSettings: Msvm_FeatureSettingData[]` | `Job: CIM_ConcreteJob` | Remove switch features |

---

### Msvm_ImageManagementService

Manages virtual hard disk images.

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `CreateVirtualHardDisk` | `VirtualDiskSettingData: string` | `Job: CIM_ConcreteJob` | Create VHD/VHDX |
| `CreateVirtualFloppyDisk` | `Path: string` | `Job: CIM_ConcreteJob` | Create virtual floppy |
| `SetVirtualHardDiskSettingData` | `VirtualDiskSettingData: string` | `Job: CIM_ConcreteJob` | Modify VHD settings |
| `GetVirtualHardDiskSettingData` | `Path: string` | `SettingData: string` | Get VHD settings |
| `GetVirtualHardDiskState` | `Path: string` | `State: string` | Get VHD state |
| `MergeVirtualHardDisk` | `SourcePath: string, DestinationPath: string` | `Job: CIM_ConcreteJob` | Merge differencing disk |
| `CompactVirtualHardDisk` | `Path: string, Mode: uint16` | `Job: CIM_ConcreteJob` | Compact VHD |
| `ResizeVirtualHardDisk` | `Path: string, MaxInternalSize: uint64` | `Job: CIM_ConcreteJob` | Resize VHD |
| `ConvertVirtualHardDisk` | `SourcePath: string, VirtualDiskSettingData: string` | `Job: CIM_ConcreteJob` | Convert VHD format |
| `AttachVirtualHardDisk` | `Path: string, AssignDriveLetter: bool, ReadOnly: bool` | - | Attach VHD |
| `ValidateVirtualHardDisk` | `Path: string` | - | Validate VHD |
| `SetParentVirtualHardDisk` | `ChildPath: string, ParentPath: string, LeafPath: string` | `Job: CIM_ConcreteJob` | Set parent disk |
| `GetVHDSetInformation` | `VHDSetPath: string, RequestedInformation: uint32[]` | `Information: string[]` | Get VHD set info |
| `SetVHDSnapshotInformation` | `Information: string` | `Job: CIM_ConcreteJob` | Set VHD snapshot info |
| `GetVHDSnapshotInformation` | `VHDSetPath: string, SnapshotIds: string[], RequestedInformation: uint32[]` | `SnapshotInformation: string[]` | Get snapshot info |
| `DeleteVHDSnapshot` | `VHDSetPath: string, SnapshotId: string, PersistReferenceSnapshot: bool` | `Job: CIM_ConcreteJob` | Delete VHD snapshot |
| `OptimizeVHDSet` | `VHDSetPath: string` | `Job: CIM_ConcreteJob` | Optimize VHD set |

---

### Msvm_ReplicationService

Manages Hyper-V Replica.

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `CreateReplicationRelationship` | `ComputerSystem: CIM_ComputerSystem, ReplicationSettingData: string` | `Job: CIM_ConcreteJob` | Enable replication |
| `ModifyReplicationSettings` | `ComputerSystem: CIM_ComputerSystem, ReplicationSettingData: string` | `Job: CIM_ConcreteJob` | Modify replication |
| `RemoveReplicationRelationship` | `ComputerSystem: CIM_ComputerSystem` | `Job: CIM_ConcreteJob` | Remove replication |
| `StartReplication` | `ComputerSystem: CIM_ComputerSystem, InitialReplicationSettings: string` | `Job: CIM_ConcreteJob` | Start initial replication |
| `ReverseReplicationRelationship` | `ComputerSystem: CIM_ComputerSystem, ReplicationSettingData: string` | `Job: CIM_ConcreteJob` | Reverse replication |
| `InitiateFailover` | `ComputerSystem: CIM_ComputerSystem, SnapshotSettingData: Msvm_VirtualSystemSettingData` | `Job: CIM_ConcreteJob` | Initiate failover |
| `RevertFailover` | `ComputerSystem: CIM_ComputerSystem` | `Job: CIM_ConcreteJob` | Revert failover |
| `CommitFailover` | `ComputerSystem: CIM_ComputerSystem` | `Job: CIM_ConcreteJob` | Commit failover |
| `TestReplicaSystem` | `ComputerSystem: CIM_ComputerSystem, SnapshotSettingData: Msvm_VirtualSystemSettingData` | `ResultingSystem: CIM_ComputerSystem, Job: CIM_ConcreteJob` | Test failover |
| `Resynchronize` | `ComputerSystem: CIM_ComputerSystem, ResynchronizeSettings: string` | `Job: CIM_ConcreteJob` | Resynchronize replica |
| `SetAuthorizationEntry` | `ComputerSystem: CIM_ComputerSystem, ReplicaServerName: string, TrustGroup: string, ReplicaStorageLocation: string` | - | Set auth entry |
| `RemoveAuthorizationEntry` | `ComputerSystem: CIM_ComputerSystem, ReplicaServerName: string` | - | Remove auth entry |
| `ChangeReplicationModeToPrimary` | `ComputerSystem: CIM_ComputerSystem, ReplicationSettingData: string` | `Job: CIM_ConcreteJob` | Change to primary |

---

### Msvm_SecurityService

Manages VM security (shielded VMs, TPM).

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `GetKeyProtector` | `TargetSystem: CIM_ComputerSystem` | `KeyProtector: uint8[]` | Get key protector |
| `SetKeyProtector` | `TargetSystem: CIM_ComputerSystem, KeyProtector: uint8[]` | `Job: CIM_ConcreteJob` | Set key protector |
| `SetSecurityPolicy` | `TargetSystem: CIM_ComputerSystem, SecurityPolicy: uint8[]` | `Job: CIM_ConcreteJob` | Set security policy |
| `GetSecurityPolicy` | `TargetSystem: CIM_ComputerSystem` | `SecurityPolicy: uint8[]` | Get security policy |
| `RestoreLastKnownGoodKeyProtector` | `TargetSystem: CIM_ComputerSystem` | `Job: CIM_ConcreteJob` | Restore key protector |
| `ModifySecuritySettings` | `SettingData: string` | `Job: CIM_ConcreteJob` | Modify security settings |

---

### Msvm_CollectionManagementService

Manages VM collections.

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `DefineCollection` | `Name: string, Id: string, Type: uint16` | `DefinedCollection: CIM_CollectionOfMSEs, Job: CIM_ConcreteJob` | Create collection |
| `DestroyCollection` | `Collection: CIM_CollectionOfMSEs` | `Job: CIM_ConcreteJob` | Delete collection |
| `RenameCollection` | `Collection: CIM_CollectionOfMSEs, NewName: string` | `Job: CIM_ConcreteJob` | Rename collection |
| `AddMember` | `Collection: CIM_CollectionOfMSEs, Member: CIM_ManagedElement` | `Job: CIM_ConcreteJob` | Add member |
| `RemoveMember` | `Collection: CIM_CollectionOfMSEs, Member: CIM_ManagedElement` | `Job: CIM_ConcreteJob` | Remove member |
| `RemoveMemberById` | `Collection: CIM_CollectionOfMSEs, MemberId: string` | `Job: CIM_ConcreteJob` | Remove member by ID |

---

## Setting Data Classes

### Msvm_VirtualSystemSettingData

VM configuration settings.

**Key Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `VirtualSystemIdentifier` | `string` | VM GUID |
| `VirtualSystemType` | `string` | VM type |
| `ElementName` | `string` | VM name |
| `Notes` | `string[]` | VM notes |
| `VirtualNumaEnabled` | `bool` | NUMA enabled |
| `AutomaticStartupAction` | `uint16` | Startup action |
| `AutomaticStartupActionDelay` | `string` | Startup delay |
| `AutomaticShutdownAction` | `uint16` | Shutdown action |
| `AutomaticRecoveryAction` | `uint16` | Recovery action |
| `ConfigurationFile` | `string` | Config file path |
| `SecureBootEnabled` | `bool` | Secure boot |
| `SecureBootTemplateId` | `string` | Secure boot template |
| `GuestControlledCacheTypes` | `bool` | Cache types |
| `LowMmioGapSize` | `uint64` | Low MMIO gap |
| `HighMmioGapSize` | `uint64` | High MMIO gap |
| `EnhancedSessionTransportType` | `uint16` | Enhanced session transport |

### Msvm_ProcessorSettingData

Processor allocation settings.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `VirtualQuantity` | `uint64` | vCPU count |
| `Reservation` | `uint64` | Minimum reservation |
| `Limit` | `uint64` | Maximum limit |
| `Weight` | `uint32` | Relative weight |
| `LimitCPUID` | `bool` | Limit CPUID |
| `HwThreadsPerCore` | `uint64` | HW threads per core |
| `EnableHostResourceProtection` | `bool` | Host resource protection |
| `ExposeVirtualizationExtensions` | `bool` | Nested virtualization |
| `EnablePerfmonLbr` | `bool` | Perfmon LBR |
| `EnablePerfmonPebs` | `bool` | Perfmon PEBS |
| `EnablePerfmonIpt` | `bool` | Perfmon IPT |

### Msvm_MemorySettingData

Memory allocation settings.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `VirtualQuantity` | `uint64` | Startup memory (MB) |
| `Reservation` | `uint64` | Minimum memory (MB) |
| `Limit` | `uint64` | Maximum memory (MB) |
| `DynamicMemoryEnabled` | `bool` | Dynamic memory |
| `TargetMemoryBuffer` | `uint32` | Memory buffer % |
| `MaxMemoryBlocksPerNumaNode` | `uint64` | Max blocks per NUMA |
| `SgxSize` | `uint64` | SGX enclave size |
| `SgxEnabled` | `bool` | SGX enabled |

### Msvm_StorageAllocationSettingData

Storage device settings.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `HostResource` | `string[]` | VHD path |
| `StorageQoSPolicyID` | `string` | QoS policy ID |
| `IOPSLimit` | `uint64` | IOPS limit |
| `IOPSReservation` | `uint64` | IOPS reservation |
| `IOPSMaximum` | `uint64` | Maximum IOPS |
| `PersistentReservationsSupported` | `bool` | Persistent reservations |
| `CachingMode` | `uint16` | Caching mode |
| `SnapshotId` | `string` | Snapshot ID |

### Msvm_EthernetPortAllocationSettingData

Network adapter settings.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `HostResource` | `string[]` | Virtual switch path |
| `Address` | `string` | Static MAC address |
| `EnabledState` | `uint16` | Enabled state |
| `TestReplicaPoolId` | `string` | Test replica pool |
| `TestReplicaSwitchName` | `string` | Test replica switch |
| `RequiredFeatures` | `string[]` | Required features |
| `RequiredFeatureHints` | `string[]` | Feature hints |

---

## Virtual Switch Classes

### Msvm_VirtualEthernetSwitch

Represents a virtual switch.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | Switch GUID |
| `ElementName` | `string` | Switch name |
| `EnabledState` | `uint16` | Enabled state |
| `RequestedState` | `uint16` | Requested state |
| `HealthState` | `uint16` | Health state |
| `MaxVMQOffloads` | `uint32` | Max VMQ offloads |
| `MaxChimneyOffloads` | `uint32` | Max chimney offloads |

### Msvm_VirtualEthernetSwitchSettingData

Virtual switch configuration.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `ElementName` | `string` | Switch name |
| `VirtualSystemIdentifier` | `string` | Switch GUID |
| `Notes` | `string[]` | Switch notes |
| `MaxIOVOffloads` | `uint32` | Max IOV offloads |
| `IOVPreferred` | `bool` | IOV preferred |
| `BandwidthReservationMode` | `uint32` | Bandwidth mode |
| `TeamingEnabled` | `bool` | NIC teaming enabled |
| `PacketDirectEnabled` | `bool` | Packet Direct enabled |

### Msvm_EthernetSwitchPort

Virtual switch port.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | Port name |
| `SystemName` | `string` | System name |
| `EnabledState` | `uint16` | Port state |
| `VMQOffloadWeight` | `uint32` | VMQ weight |

### Msvm_EthernetSwitchPortSecuritySettingData

Port security settings.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `AllowMacSpoofing` | `bool` | Allow MAC spoofing |
| `EnableDhcpGuard` | `bool` | DHCP guard |
| `EnableRouterGuard` | `bool` | Router guard |
| `EnableIeeePriorityTag` | `bool` | 802.1p tagging |
| `MonitorMode` | `uint8` | Monitor mode |
| `MonitorSession` | `uint8` | Monitor session |
| `AllowTeaming` | `bool` | Allow NIC teaming |
| `VirtualSubnetId` | `uint32` | Virtual subnet ID |
| `DynamicIPAddressLimit` | `uint32` | Dynamic IP limit |
| `StormLimit` | `uint32` | Broadcast storm limit |

### Msvm_EthernetSwitchPortVlanSettingData

Port VLAN settings.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `OperationMode` | `uint32` | VLAN operation mode |
| `AccessVlanId` | `uint16` | Access VLAN ID |
| `NativeVlanId` | `uint16` | Native VLAN ID |
| `TrunkVlanIdArray` | `uint16[]` | Trunk VLAN IDs |
| `PruneVlanIdArray` | `uint16[]` | Pruned VLANs |
| `PvlanMode` | `uint32` | Private VLAN mode |
| `PrimaryVlanId` | `uint16` | Primary VLAN ID |
| `SecondaryVlanId` | `uint16` | Secondary VLAN ID |
| `SecondaryVlanIdArray` | `uint16[]` | Secondary VLAN array |

---

## Guest Integration Services

### Msvm_ShutdownComponent

Guest shutdown integration.

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `InitiateShutdown` | `Force: bool, Reason: string` | `ReturnValue: uint32` | Shutdown guest |
| `InitiateReboot` | `Force: bool, Reason: string` | `ReturnValue: uint32` | Reboot guest |

### Msvm_KvpExchangeComponent

Key-Value Pair exchange.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `GuestExchangeItems` | `string[]` | Guest KVP items |
| `GuestIntrinsicExchangeItems` | `string[]` | Guest intrinsic items |

### Msvm_TimeSyncComponent

Time synchronization.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `EnabledState` | `uint16` | Enabled state |

### Msvm_HeartbeatComponent

Heartbeat monitoring.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `EnabledState` | `uint16` | Enabled state |

### Msvm_VssComponent

VSS (Volume Shadow Copy Service) integration.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `EnabledState` | `uint16` | Enabled state |

### Msvm_GuestFileService

Guest file service.

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `CopyFilesToGuest` | `CopyFileToGuestSettings: string[]` | `Job: CIM_ConcreteJob` | Copy files to guest |

---

## Resource Pool Classes

### Msvm_ResourcePool

Resource pool for allocation.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `PoolId` | `string` | Pool ID |
| `ResourceType` | `uint16` | Resource type |
| `ResourceSubType` | `string` | Resource subtype |
| `Primordial` | `bool` | Is primordial pool |
| `Capacity` | `uint64` | Total capacity |
| `Reserved` | `uint64` | Reserved capacity |
| `AllocationUnits` | `string` | Allocation units |

### Msvm_ResourcePoolConfigurationService

Resource pool management.

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `CreatePool` | `PoolSettings: string, ParentPool: CIM_ResourcePool[], AllocationSettings: string[]` | `Pool: CIM_ResourcePool, Job: CIM_ConcreteJob` | Create pool |
| `ChangePoolResources` | `Pool: CIM_ResourcePool, ParentPool: CIM_ResourcePool[], AllocationSettings: string[]` | `Job: CIM_ConcreteJob` | Modify pool |
| `DeletePool` | `Pool: CIM_ResourcePool` | `Job: CIM_ConcreteJob` | Delete pool |

---

## Metrics Classes

### Msvm_MetricService

Performance metrics service.

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `ControlMetrics` | `Subject: CIM_ManagedElement, Definition: CIM_BaseMetricDefinition, MetricCollectionEnabled: uint16` | - | Control metric collection |
| `ControlMetricsByClass` | `Subject: CIM_ManagedElement, Definition: CIM_BaseMetricDefinition, MetricCollectionEnabled: uint16, SubjectClass: string` | - | Control metrics by class |

### Msvm_BaseMetricValue

Metric value.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `MetricDefinitionId` | `string` | Definition ID |
| `MeasuredElementName` | `string` | Element name |
| `MetricValue` | `string` | Value |
| `TimeStamp` | `string` | Timestamp |
| `Duration` | `string` | Duration |

---

## Job Classes

### Msvm_ConcreteJob

Asynchronous operation tracking.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `InstanceID` | `string` | Job ID |
| `Name` | `string` | Job name |
| `JobState` | `uint16` | State (2=New, 3=Starting, 4=Running, 5=Suspended, 6=ShuttingDown, 7=Completed, 8=Terminated, 9=Killed, 10=Exception, 11=Service) |
| `PercentComplete` | `uint16` | Completion percentage |
| `ErrorCode` | `uint16` | Error code |
| `ErrorDescription` | `string` | Error description |
| `ErrorSummaryDescription` | `string` | Error summary |
| `TimeSubmitted` | `string` | Submit time |
| `StartTime` | `string` | Start time |
| `ElapsedTime` | `string` | Elapsed time |

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `GetError` | - | `Error: string` | Get error details |
| `RequestStateChange` | `RequestedState: uint16, TimeoutPeriod: string` | `Job: CIM_ConcreteJob` | Change job state |

### Msvm_StorageJob

Storage-specific job.

**Additional Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Child` | `string` | Child VHD |
| `Parent` | `string` | Parent VHD |
| `JobType` | `uint16` | Storage job type |

---

## Event Classes

### Msvm_ComputerSystemStateChange

VM state change event.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `SourceInstance` | `object` | Source VM |
| `PreviousInstance` | `object` | Previous state |

### Msvm_ComputerSystemCreation

VM creation event.

### Msvm_ComputerSystemDeletion

VM deletion event.

---

## Common Return Values

| Value | Description |
|-------|-------------|
| 0 | Completed with No Error |
| 1 | Not Supported |
| 2 | Failed |
| 3 | Timeout |
| 4 | Invalid Parameter |
| 5 | Invalid State |
| 4096 | Job Started |
| 32768 | Transition Started |
| 32769 | Transition Complete |

---

## Usage Examples

### PowerShell - Create VM

```powershell
$vmms = Get-WmiObject -Namespace root\virtualization\v2 -Class Msvm_VirtualSystemManagementService
$vmSettings = ([WmiClass]"\\.\root\virtualization\v2:Msvm_VirtualSystemSettingData").CreateInstance()
$vmSettings.ElementName = "NewVM"
$vmSettings.ConfigurationDataRoot = "C:\VMs"
$result = $vmms.DefineSystem($vmSettings.GetText(1), $null, $null)
```

### WMI Query - List VMs

```
SELECT * FROM Msvm_ComputerSystem WHERE Caption = 'Virtual Machine'
```

### WMI Query - Get VM State

```
SELECT EnabledState, HealthState FROM Msvm_ComputerSystem WHERE ElementName = 'MyVM'
```
