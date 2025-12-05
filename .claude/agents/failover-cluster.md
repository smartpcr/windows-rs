# Failover Cluster WMI API Reference

This document provides comprehensive API reference for Windows Failover Clustering WMI operations. The APIs are exposed through the `root\MSCluster` WMI namespace.

## WMI Namespace

```
root\MSCluster
```

## Core Classes

### MSCluster_Cluster

Represents the failover cluster.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | Cluster name |
| `Fqdn` | `string` | Fully qualified domain name |
| `QuorumPath` | `string` | Quorum resource path |
| `QuorumType` | `string` | Quorum type |
| `QuorumTypeValue` | `uint32` | Quorum type value |
| `ClusterFunctionalLevel` | `uint32` | Functional level |
| `ClusterUpgradeVersion` | `uint32` | Upgrade version |
| `DynamicQuorumEnabled` | `uint32` | Dynamic quorum enabled |
| `AdminAccessPoint` | `uint32` | Admin access point type |
| `SharedVolumesRoot` | `string` | CSV root path |
| `EnableSharedVolumes` | `uint32` | Shared volumes enabled |
| `PreferredSite` | `string` | Preferred site |
| `BackupInProgress` | `uint32` | Backup in progress |
| `DrainOnShutdown` | `uint32` | Drain on shutdown |
| `Security` | `uint8[]` | Security descriptor |
| `SecurityLevel` | `uint32` | Security level |
| `SecurityLevelForStorage` | `uint32` | Storage security level |

**Cluster Timing Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `SameSubnetDelay` | `uint32` | Same subnet heartbeat delay (ms) |
| `SameSubnetThreshold` | `uint32` | Same subnet threshold |
| `CrossSubnetDelay` | `uint32` | Cross-subnet heartbeat delay (ms) |
| `CrossSubnetThreshold` | `uint32` | Cross-subnet threshold |
| `CrossSiteDelay` | `uint32` | Cross-site delay (ms) |
| `CrossSiteThreshold` | `uint32` | Cross-site threshold |
| `PlumbAllCrossSubnetRoutes` | `uint32` | Plumb cross-subnet routes |
| `RouteHistoryLength` | `uint32` | Route history length |

**Quorum Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `QuorumArbitrationTimeMax` | `uint32` | Max arbitration time |
| `QuorumArbitrationTimeMin` | `uint32` | Min arbitration time |
| `QuorumLogFileSize` | `uint32` | Quorum log size |
| `WitnessDatabaseWriteTimeout` | `uint32` | Witness write timeout |
| `WitnessDynamicWeight` | `uint32` | Witness dynamic weight |
| `WitnessRestartInterval` | `uint32` | Witness restart interval |
| `FixQuorum` | `uint32` | Fix quorum mode |
| `PreventQuorum` | `uint32` | Prevent quorum |
| `LowerQuorumPriorityNodeId` | `uint32` | Lower priority node ID |

**Migration Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `MaximumParallelMigrations` | `uint32` | Max parallel migrations |
| `UseRdmaForStorage` | `uint32` | Use RDMA for storage |
| `RdmaConnectionsPerInterfaceForStorage` | `uint32` | RDMA connections per interface |

**Storage Spaces Direct (S2D) Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `S2DEnabled` | `uint32` | S2D enabled |
| `S2DBusTypes` | `uint32` | S2D bus types |
| `S2DCacheBehavior` | `uint64` | Cache behavior |
| `S2DCacheDesiredState` | `uint32` | Cache desired state |
| `S2DCacheDeviceModel` | `string[]` | Cache device models |
| `S2DCacheFlashReservePercent` | `uint32` | Flash reserve % |
| `S2DCacheMetadataReserveBytes` | `uint64` | Metadata reserve bytes |
| `S2DCachePageSizeKBytes` | `uint32` | Cache page size |
| `S2DIOLatencyThreshold` | `uint32` | IO latency threshold |
| `S2DOptimizations` | `uint32` | S2D optimizations |

**Auto-Balancer Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `AutoBalancerMode` | `uint32` | Balancer mode (0=Off, 1=Minimal, 2=Aggressive) |
| `AutoBalancerLevel` | `uint32` | Balancer level |

**Resource DLL Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `ResourceDllDeadlockPeriod` | `uint32` | Deadlock period |
| `LogResourceControls` | `uint32` | Log resource controls |

**Cluster Service Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `ClusSvcHangTimeout` | `uint32` | Cluster service hang timeout |
| `ClusSvcRegroupOpeningTimeout` | `uint32` | Regroup opening timeout |
| `ClusSvcRegroupPruningTimeout` | `uint32` | Regroup pruning timeout |
| `ClusSvcRegroupStageTimeout` | `uint32` | Regroup stage timeout |
| `ClusSvcRegroupTickInMilliseconds` | `uint32` | Regroup tick interval |
| `ClusterLogLevel` | `uint32` | Log level |
| `ClusterLogSize` | `uint32` | Log size |
| `HangRecoveryAction` | `uint32` | Hang recovery action |

**Other Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `AddEvictDelay` | `uint32` | Add/evict delay |
| `AutoAssignNodeSite` | `uint32` | Auto-assign node site |
| `BlockCacheSize` | `uint32` | Block cache size |
| `ClusterEnforcedAntiAffinity` | `uint32` | Enforced anti-affinity |
| `ClusterGroupWaitDelay` | `uint32` | Group wait delay |
| `DefaultNetworkRole` | `uint32` | Default network role |
| `DetectedCloudPlatform` | `uint32` | Detected cloud platform |
| `GracePeriodEnabled` | `uint32` | Grace period enabled |
| `GracePeriodTimeout` | `uint32` | Grace period timeout |
| `GroupDependencyTimeout` | `uint32` | Group dependency timeout |
| `MessageBufferLength` | `uint32` | Message buffer length |
| `MinimumNeverPreemptPriority` | `uint32` | Min never preempt priority |
| `MinimumPreemptorPriority` | `uint32` | Min preemptor priority |
| `NetftIPSecEnabled` | `uint32` | Netft IPSec enabled |
| `PlacementOptions` | `uint32` | Placement options |
| `QuarantineDuration` | `uint32` | Quarantine duration |
| `QuarantineThreshold` | `uint32` | Quarantine threshold |
| `RequestReplyTimeout` | `uint32` | Request reply timeout |
| `ResiliencyDefaultPeriod` | `uint32` | Resiliency default period |
| `ResiliencyLevel` | `uint32` | Resiliency level |
| `ShutdownTimeoutInMinutes` | `uint32` | Shutdown timeout |
| `AcceleratedNetworkingEnabled` | `uint32` | Accelerated networking |

**Methods:**

| Method | Parameters | Description |
|--------|------------|-------------|
| `AddNode` | `NodeName: string, SkipValidation: bool` | Add node to cluster |
| `EvictNode` | `NodeName: string, Reason: string` | Evict node from cluster |
| `CreateGroup` | `GroupName: string, GroupType: string` | Create resource group |
| `DestroyGroup` | `GroupName: string` | Delete resource group |
| `SetDynamicQuorum` | `DynamicQuorumEnabled: uint32` | Set dynamic quorum |
| `SetQuorumResource` | `Resource: string, QuorumPath: string` | Set quorum resource |
| `ClearQuorumResource` | - | Clear quorum resource |

---

### MSCluster_Node

Represents a cluster node.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | Node name |
| `Id` | `string` | Node ID |
| `UniqueID` | `string` | Unique ID |
| `NodeInstanceID` | `string` | Instance ID |
| `State` | `uint32` | Node state (0=Unknown, 1=Up, 2=Down, 3=Paused, 4=Joining) |
| `Characteristics` | `uint32` | Node characteristics |
| `Flags` | `uint32` | Node flags |
| `BuildNumber` | `uint32` | Windows build number |
| `MajorVersion` | `uint32` | Major version |
| `MinorVersion` | `uint32` | Minor version |
| `CSDVersion` | `string` | Service pack version |
| `NodeWeight` | `uint32` | Voting weight |
| `DynamicWeight` | `uint32` | Dynamic weight |
| `NodeDrainStatus` | `uint32` | Drain status (0=NotInitiated, 1=InProgress, 2=Completed, 3=Failed) |
| `NodeDrainTarget` | `string` | Drain target node |
| `DrainErrorCode` | `uint32` | Drain error code |
| `NodeFailbackStatus` | `uint32` | Failback status |
| `NeedsPreventQuorum` | `uint32` | Needs prevent quorum |
| `StatusInformation` | `uint32` | Status information |
| `NodeHighestVersion` | `uint32` | Highest version |
| `NodeLowestVersion` | `uint32` | Lowest version |
| `FaultDomain` | `string[]` | Fault domain path |
| `FaultDomainId` | `string` | Fault domain ID |
| `DetectedCloudPlatform` | `uint32` | Cloud platform |

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `Pause` | `DrainType: uint32, TargetNode: string, Reason: string` | - | Pause node (drain workloads) |
| `Resume` | `FailbackType: uint32, Reason: string` | - | Resume node |
| `WillEvictLoseQuorum` | `Reason: string` | `bool` | Check if evict loses quorum |
| `WillOfflineLoseQuorum` | `Reason: string` | `bool` | Check if offline loses quorum |
| `ExecuteNodeControl` | `ControlCode: int32, InputBuffer: uint8[], Reason: string` | `OutputBuffer: uint8[], OutputBufferSize: int32` | Execute node control |

**DrainType Values:**

| Value | Description |
|-------|-------------|
| 0 | Do not drain roles |
| 1 | Drain roles (failover) |
| 2 | Drain roles with no failover |

**FailbackType Values:**

| Value | Description |
|-------|-------------|
| 0 | Do not fail back roles |
| 1 | Fail back roles |

---

### MSCluster_Resource

Represents a cluster resource.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | Resource name |
| `Id` | `string` | Resource ID |
| `Type` | `string` | Resource type |
| `OwnerGroup` | `string` | Owner group name |
| `OwnerNode` | `string` | Owner node name |
| `State` | `uint32` | Resource state (0=Unknown, 1=Inherited, 2=Initializing, 3=Online, 4=Offline, 128=Pending, 129=OnlinePending, 130=OfflinePending) |
| `Subclass` | `uint32` | Resource subclass |
| `ResourceClass` | `uint32` | Resource class |
| `CoreResource` | `bool` | Is core resource |
| `QuorumCapable` | `bool` | Is quorum capable |
| `LocalQuorumCapable` | `bool` | Is local quorum capable |
| `IsClusterSharedVolume` | `bool` | Is CSV |
| `PersistentState` | `bool` | Persistent state |
| `SeparateMonitor` | `bool` | Separate monitor process |
| `MonitorProcessId` | `uint32` | Monitor process ID |
| `RestartAction` | `uint32` | Restart action |
| `RestartDelay` | `uint32` | Restart delay |
| `RestartPeriod` | `uint32` | Restart period |
| `RestartThreshold` | `uint32` | Restart threshold |
| `RetryPeriodOnFailure` | `uint32` | Retry period on failure |
| `PendingTimeout` | `uint32` | Pending timeout |
| `DeadlockTimeout` | `uint32` | Deadlock timeout |
| `IsAlivePollInterval` | `uint32` | IsAlive poll interval |
| `LooksAlivePollInterval` | `uint32` | LooksAlive poll interval |
| `EmbeddedFailureAction` | `uint32` | Embedded failure action |
| `DeleteRequiresAllNodes` | `bool` | Delete requires all nodes |
| `CryptoCheckpoints` | `string[]` | Crypto checkpoints |
| `RegistryCheckpoints` | `string[]` | Registry checkpoints |
| `LastOperationStatusCode` | `uint64` | Last operation status |
| `StatusInformation` | `uint64` | Status information |
| `ResourceSpecificStatus` | `string` | Resource-specific status |
| `ResourceSpecificData1` | `uint64` | Custom data 1 |
| `ResourceSpecificData2` | `uint64` | Custom data 2 |

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `CreateResource` | `Group: string, ResourceName: string, ResourceType: string, SeparateMonitor: bool` | `Id: string` | Create resource |
| `DeleteResource` | `Options: uint32, Reason: string` | - | Delete resource |
| `BringOnline` | `TimeOut: uint32, Reason: string` | - | Bring resource online |
| `TakeOffline` | `TimeOut: uint32, Parameters: MSCluster_Property, Flags: uint32, Reason: string` | - | Take resource offline |
| `TakeOfflineParams` | `TimeOut: uint32, Parameters: uint8[], Flags: uint32, Reason: string` | - | Take offline with params |
| `FailResource` | `Reason: string` | - | Fail resource |
| `MoveToNewGroup` | `Group: string, Reason: string` | - | Move to different group |
| `Rename` | `newName: string, Reason: string` | - | Rename resource |
| `AddDependency` | `Resource: string` | - | Add dependency |
| `RemoveDependency` | `Resource: string` | - | Remove dependency |
| `SetDependencies` | `Expression: string` | - | Set dependency expression |
| `GetDependencies` | `AsResourceIds: bool` | `Expression: string` | Get dependencies |
| `AddPossibleOwner` | `NodeName: string` | - | Add possible owner |
| `RemovePossibleOwner` | `NodeName: string` | - | Remove possible owner |
| `GetPossibleOwners` | - | `NodeNames: string[]` | Get possible owners |
| `AddRegistryCheckpoint` | `CheckpointName: string` | - | Add registry checkpoint |
| `RemoveRegistryCheckpoint` | `CheckpointName: string` | - | Remove registry checkpoint |
| `AddCryptoCheckpoint` | `CheckpointName: string` | - | Add crypto checkpoint |
| `RemoveCryptoCheckpoint` | `CheckpointName: string` | - | Remove crypto checkpoint |
| `RenewAddress` | - | - | Renew DHCP address |
| `ReleaseAddress` | - | - | Release DHCP address |
| `UpdateVirtualMachine` | - | - | Update VM resource |
| `MigrateVirtualMachine` | `SnapshotDestinationPath: string, ConfigurationDestinationPath: string, SwapFileDestinationPath: string, SourcePaths: string[], DestinationPaths: string[], ResourceDestinationPools: string[]` | - | Migrate VM storage |
| `AttachStorageDevice` | `StorageDevice: MSCluster_AvailableDisk` | - | Attach storage device |
| `ExecuteResourceControl` | `ControlCode: int32, InputBuffer: uint8[], Reason: string` | `OutputBuffer: uint8[], OutputBufferSize: int32` | Execute resource control |

---

### MSCluster_ResourceGroup

Represents a resource group.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | Group name |
| `Id` | `string` | Group ID |
| `State` | `uint32` | Group state (0=Unknown, 1=Online, 2=Offline, 3=Failed, 4=PartialOnline, 128=Pending) |
| `OwnerNode` | `string` | Current owner node |
| `Priority` | `uint32` | Group priority |
| `AutoFailbackType` | `uint32` | Auto failback type |
| `FailbackWindowStart` | `uint32` | Failback window start |
| `FailbackWindowEnd` | `uint32` | Failback window end |
| `FailoverPeriod` | `uint32` | Failover period |
| `FailoverThreshold` | `uint32` | Failover threshold |
| `PersistentState` | `bool` | Persistent state |
| `AntiAffinityClassNames` | `string[]` | Anti-affinity class names |
| `CCFOnFailurePolicyAction` | `uint32` | CCF failure policy |
| `Description` | `string` | Group description |
| `DefaultOwner` | `uint32` | Default owner |
| `PlacementOptions` | `uint32` | Placement options |
| `StatusInformation` | `uint32` | Status information |
| `GroupType` | `uint32` | Group type |
| `FaultDomain` | `string` | Fault domain |
| `UpdateDomain` | `uint32` | Update domain |
| `LockedMode` | `uint32` | Locked mode |
| `PreferredSite` | `string` | Preferred site |
| `ColdStartSetting` | `uint32` | Cold start setting |

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `CreateGroup` | `GroupName: string, GroupType: string` | `Id: string` | Create group |
| `DeleteGroup` | `Options: uint32, Reason: string` | - | Delete group |
| `BringOnline` | `TimeOut: uint32, Reason: string` | - | Bring group online |
| `TakeOffline` | `TimeOut: uint32, Reason: string` | - | Take group offline |
| `MoveToNewNode` | `Node: string, MoveType: uint32, Reason: string` | - | Move to different node |
| `Rename` | `newName: string, Reason: string` | - | Rename group |
| `SetNodeList` | `NodeList: string[]` | - | Set preferred owners |
| `GetNodeList` | - | `NodeList: string[]` | Get preferred owners |
| `AddPreferredOwner` | `NodeName: string` | - | Add preferred owner |
| `RemovePreferredOwner` | `NodeName: string` | - | Remove preferred owner |
| `ExecuteGroupControl` | `ControlCode: int32, InputBuffer: uint8[], Reason: string` | `OutputBuffer: uint8[], OutputBufferSize: int32` | Execute group control |

**MoveType Values:**

| Value | Description |
|-------|-------------|
| 0 | Move with default |
| 1 | Move with wait for failback |
| 2 | Move with do not fail back |
| 3 | Move with quick |

---

### MSCluster_ResourceType

Represents a resource type.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | Type name |
| `DisplayName` | `string` | Display name |
| `Description` | `string` | Description |
| `DllName` | `string` | Resource DLL |
| `Characteristics` | `uint32` | Type characteristics |
| `Flags` | `uint32` | Type flags |
| `IsAlivePollInterval` | `uint32` | Default IsAlive interval |
| `LooksAlivePollInterval` | `uint32` | Default LooksAlive interval |
| `PendingTimeout` | `uint32` | Default pending timeout |
| `DeadlockTimeout` | `uint32` | Default deadlock timeout |
| `DumpPolicy` | `uint64` | Dump policy |
| `DumpLogQuery` | `string` | Dump log query |
| `AdminExtensions` | `string[]` | Admin extensions |
| `RequiredDependencyClasses` | `uint32[]` | Required dependency classes |
| `RequiredDependencyTypes` | `string[]` | Required dependency types |
| `QuorumCapable` | `bool` | Is quorum capable |
| `LocalQuorumCapable` | `bool` | Is local quorum capable |
| `DeleteRequiresAllNodes` | `bool` | Delete requires all nodes |

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `CreateResourceType` | `TypeName: string, DisplayName: string, DllName: string, LooksAlivePollInterval: uint32, IsAlivePollInterval: uint32` | - | Create resource type |
| `DeleteResourceType` | `Reason: string` | - | Delete resource type |
| `AddPossibleOwner` | `NodeName: string` | - | Add possible owner |
| `RemovePossibleOwner` | `NodeName: string` | - | Remove possible owner |
| `GetPossibleOwners` | - | `NodeNames: string[]` | Get possible owners |
| `ExecuteResourceTypeControl` | `ControlCode: int32, InputBuffer: uint8[], Reason: string` | `OutputBuffer: uint8[], OutputBufferSize: int32` | Execute type control |

---

### MSCluster_Network

Represents a cluster network.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | Network name |
| `Id` | `string` | Network ID |
| `Description` | `string` | Description |
| `State` | `uint32` | Network state |
| `Role` | `uint32` | Network role (0=None, 1=ClusterOnly, 3=ClusterAndClient) |
| `Characteristics` | `uint32` | Network characteristics |
| `Flags` | `uint32` | Network flags |
| `Address` | `string` | Network address |
| `AddressMask` | `string` | Address mask |
| `Metric` | `uint32` | Network metric |
| `IPv6Addresses` | `string[]` | IPv6 addresses |
| `IPv6PrefixLengths` | `uint32[]` | IPv6 prefix lengths |
| `AutoMetric` | `bool` | Auto metric |

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `Rename` | `newName: string, Reason: string` | - | Rename network |
| `ExecuteNetworkControl` | `ControlCode: int32, InputBuffer: uint8[], Reason: string` | `OutputBuffer: uint8[], OutputBufferSize: int32` | Execute network control |

---

### MSCluster_NetworkInterface

Represents a network interface.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | Interface name |
| `Id` | `string` | Interface ID |
| `Description` | `string` | Description |
| `Node` | `string` | Node name |
| `Network` | `string` | Network name |
| `State` | `uint32` | Interface state |
| `Characteristics` | `uint32` | Interface characteristics |
| `Flags` | `uint32` | Interface flags |
| `Adapter` | `string` | Adapter name |
| `AdapterId` | `string` | Adapter ID |
| `Address` | `string` | IPv4 address |
| `IPv6Addresses` | `string[]` | IPv6 addresses |
| `DhcpEnabled` | `bool` | DHCP enabled |

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `ExecuteNetworkInterfaceControl` | `ControlCode: int32, InputBuffer: uint8[], Reason: string` | `OutputBuffer: uint8[], OutputBufferSize: int32` | Execute interface control |

---

### MSCluster_ClusterSharedVolume

Represents a Cluster Shared Volume (CSV).

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | CSV name |
| `Id` | `string` | CSV ID |
| `State` | `uint32` | CSV state |
| `VolumeName` | `string` | Volume name |
| `VolumeOffset` | `uint64` | Volume offset |
| `SharedVolumeInfo` | `string` | Shared volume info |
| `FaultState` | `uint32` | Fault state |
| `RedirectedAccess` | `bool` | Redirected access mode |
| `BlockRedirectedIOReason` | `uint32` | Block redirected IO reason |
| `MaintenanceMode` | `bool` | Maintenance mode |
| `FileSystemRedirectedIOReason` | `uint64` | FS redirected IO reason |

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `TurnOnRedirectedAccess` | `Reason: string` | - | Enable redirected access |
| `TurnOffRedirectedAccess` | `Reason: string` | - | Disable redirected access |
| `TurnOnMaintenanceMode` | `Reason: string` | - | Enable maintenance mode |
| `TurnOffMaintenanceMode` | `Reason: string` | - | Disable maintenance mode |
| `MoveToNewNode` | `Node: string, Reason: string` | - | Move owner node |

---

### MSCluster_ClusterService

Represents the cluster service.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `SystemName` | `string` | System name |
| `Name` | `string` | Service name |
| `NodeHighestVersion` | `uint32` | Highest node version |
| `NodeLowestVersion` | `uint32` | Lowest node version |
| `ClusterServiceState` | `uint32` | Service state |

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `StartService` | - | - | Start cluster service |
| `StopService` | - | - | Stop cluster service |

---

### MSCluster_ClusterUtilities

Cluster utility operations.

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `CreateCluster` | `ClusterName: string, StaticAddress: string[], IgnoreNetwork: string[], AdministrativeAccessPoint: uint32` | - | Create new cluster |
| `AddNodeToCluster` | `NodeName: string, SkipValidation: bool` | - | Add node to cluster |
| `EvictNode` | `NodeName: string, CleanupAD: uint32, Timeout: uint32` | - | Evict node |
| `SetQuorumResource` | `Resource: string, QuorumPath: string` | - | Set quorum |
| `GetNodes` | - | `Nodes: string[]` | Get all nodes |
| `GetNetworks` | - | `Networks: string[]` | Get all networks |
| `GetResources` | - | `Resources: string[]` | Get all resources |
| `GetResourceGroups` | - | `Groups: string[]` | Get all groups |

---

### MSCluster_GroupSet

Represents a group set (collection of resource groups).

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | Set name |
| `Id` | `string` | Set ID |
| `Description` | `string` | Description |
| `GroupNames` | `string[]` | Member group names |
| `ProviderNames` | `string[]` | Provider names |
| `StartupSetting` | `uint32` | Startup setting |
| `StartupCount` | `uint32` | Startup count |
| `StartupDelay` | `uint32` | Startup delay |
| `IsGlobal` | `uint32` | Is global set |
| `IsAvailabilitySet` | `uint32` | Is availability set |
| `StatusInformation` | `uint32` | Status information |
| `FaultDomains` | `uint32` | Fault domains |
| `UpdateDomains` | `uint32` | Update domains |
| `ReservedSpareNodes` | `uint32` | Reserved spare nodes |
| `ReservedSpareNodesActual` | `uint32` | Actual spare nodes |

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `CreateGroupSet` | `SetName: string` | `Id: string` | Create group set |
| `DeleteGroupSet` | - | - | Delete group set |
| `AddMember` | `GroupName: string` | - | Add member group |
| `RemoveMember` | `GroupName: string` | - | Remove member group |
| `SetDependency` | `ProviderName: string` | - | Set provider dependency |
| `RemoveDependency` | `ProviderName: string` | - | Remove provider dependency |
| `Rename` | `newName: string` | - | Rename set |

---

### MSCluster_AffinityRule

Represents affinity/anti-affinity rules.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | Rule name |
| `Id` | `string` | Rule ID |
| `Description` | `string` | Description |
| `RuleType` | `uint32` | Rule type (1=Affinity, 2=AntiAffinityHard, 3=AntiAffinitySoft) |
| `Groups` | `string[]` | Affected groups |
| `Enabled` | `bool` | Rule enabled |

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `CreateAffinityRule` | `RuleName: string, RuleType: uint32, Groups: string[]` | `Id: string` | Create rule |
| `DeleteAffinityRule` | - | - | Delete rule |
| `AddMember` | `GroupName: string` | - | Add member |
| `RemoveMember` | `GroupName: string` | - | Remove member |
| `Rename` | `newName: string` | - | Rename rule |

---

### MSCluster_FaultDomain

Represents a fault domain.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | Domain name |
| `Id` | `string` | Domain ID |
| `Description` | `string` | Description |
| `Type` | `string` | Domain type (Site, Rack, Chassis, Node) |
| `ParentId` | `string` | Parent domain ID |
| `Location` | `string` | Physical location |

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `CreateFaultDomain` | `DomainName: string, DomainType: string, ParentId: string` | `Id: string` | Create fault domain |
| `DeleteFaultDomain` | - | - | Delete fault domain |
| `SetParent` | `ParentId: string` | - | Set parent domain |
| `Rename` | `newName: string` | - | Rename domain |

---

### MSCluster_StorageSpacesDirect

Storage Spaces Direct management.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Enabled` | `bool` | S2D enabled |
| `State` | `uint32` | S2D state |
| `CacheState` | `uint32` | Cache state |
| `CacheModeSSD` | `uint32` | SSD cache mode |
| `CacheModeHDD` | `uint32` | HDD cache mode |
| `CachePageSizeKBytes` | `uint32` | Cache page size |
| `CacheMetadataReserveBytes` | `uint64` | Metadata reserve |
| `ScmUse` | `uint32` | SCM use mode |
| `UsedStorageForVMs` | `bool` | Used for VMs |

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `Enable` | `CacheState: uint32, CacheDeviceModel: string[]` | - | Enable S2D |
| `Disable` | `CleanupCache: bool` | - | Disable S2D |
| `Repair` | - | - | Repair S2D |
| `SetCacheParameters` | `CachePageSizeKBytes: uint32, CacheMetadataReserveBytes: uint64` | - | Set cache params |

---

### MSCluster_AvailableDisk

Represents available (not clustered) storage.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | Disk name |
| `Id` | `string` | Disk ID |
| `Signature` | `uint32` | Disk signature |
| `GptGuid` | `string` | GPT GUID |
| `Path` | `string` | Device path |
| `TotalSize` | `uint64` | Total size |
| `BusType` | `uint32` | Bus type |
| `Number` | `uint32` | Disk number |
| `FriendlyName` | `string` | Friendly name |
| `Manufacturer` | `string` | Manufacturer |
| `Model` | `string` | Model |
| `SerialNumber` | `string` | Serial number |
| `IsPooled` | `bool` | Is pooled |
| `IsClustered` | `bool` | Is clustered |
| `IsSsd` | `bool` | Is SSD |
| `IsReadOnly` | `bool` | Is read only |

**Methods:**

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `AddToCluster` | `Group: string` | - | Add disk to cluster |
| `AddToClusterSharedVolume` | `Group: string` | - | Add disk as CSV |

---

### MSCluster_ScaleoutVolume

Scale-out File Server volume.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | Volume name |
| `Id` | `string` | Volume ID |
| `Path` | `string` | Volume path |
| `TotalSize` | `uint64` | Total size |
| `FreeSpace` | `uint64` | Free space |
| `HealthStatus` | `uint32` | Health status |
| `OperationalStatus` | `uint32` | Operational status |

---

## Relationship Classes

### MSCluster_ClusterToNode

Maps clusters to nodes.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `GroupComponent` | `MSCluster_Cluster` | Cluster |
| `PartComponent` | `MSCluster_Node` | Node |

### MSCluster_ClusterToResourceGroup

Maps clusters to resource groups.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `GroupComponent` | `MSCluster_Cluster` | Cluster |
| `PartComponent` | `MSCluster_ResourceGroup` | Resource group |

### MSCluster_ResourceGroupToResource

Maps groups to resources.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `GroupComponent` | `MSCluster_ResourceGroup` | Resource group |
| `PartComponent` | `MSCluster_Resource` | Resource |

### MSCluster_ResourceToDependentResource

Maps resource dependencies.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Antecedent` | `MSCluster_Resource` | Dependency (provider) |
| `Dependent` | `MSCluster_Resource` | Dependent resource |

### MSCluster_NodeToActiveGroup

Maps nodes to active groups.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `GroupComponent` | `MSCluster_Node` | Node |
| `PartComponent` | `MSCluster_ResourceGroup` | Active group |

### MSCluster_NodeToActiveResource

Maps nodes to active resources.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `GroupComponent` | `MSCluster_Node` | Node |
| `PartComponent` | `MSCluster_Resource` | Active resource |

### MSCluster_ResourceGroupToPreferredNode

Maps groups to preferred owners.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `GroupComponent` | `MSCluster_ResourceGroup` | Resource group |
| `PartComponent` | `MSCluster_Node` | Preferred node |

### MSCluster_ClusterToNetwork

Maps clusters to networks.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `GroupComponent` | `MSCluster_Cluster` | Cluster |
| `PartComponent` | `MSCluster_Network` | Network |

### MSCluster_ClusterSharedVolumeToResource

Maps CSVs to resources.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `GroupComponent` | `MSCluster_ClusterSharedVolume` | CSV |
| `PartComponent` | `MSCluster_Resource` | Resource |

---

## Event Classes

### MSCluster_Event

Base cluster event class.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `EventTypeMajor` | `uint32` | Major event type |
| `EventTypeMinor` | `uint32` | Minor event type |
| `EventNode` | `string` | Source node |
| `EventObjectName` | `string` | Object name |
| `EventObjectType` | `uint32` | Object type |
| `EventObjectPath` | `string` | Object path |
| `EventFilter` | `string` | Event filter |
| `EventNewState` | `uint32` | New state |

### MSCluster_EventStateChange

State change event.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `EventNewState` | `uint32` | New state |
| `EventOldState` | `uint32` | Previous state |

### MSCluster_EventGroupStateChange

Group state change event.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `EventGroup` | `string` | Group name |
| `EventNode` | `string` | Node name |
| `EventNewState` | `uint32` | New state |

### MSCluster_EventResourceStateChange

Resource state change event.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `EventResource` | `string` | Resource name |
| `EventGroup` | `string` | Group name |
| `EventNode` | `string` | Node name |
| `EventNewState` | `uint32` | New state |

### MSCluster_EventObjectAdd

Object addition event.

### MSCluster_EventObjectRemove

Object removal event.

### MSCluster_EventPropertyChange

Property change event.

### MSCluster_EventRegistryChange

Registry change event.

### MSCluster_ClusterUpgradedEvent

Cluster upgrade event.

---

## Health Classes

### MSCluster_HealthFault

Represents a health fault.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `FaultId` | `string` | Fault ID |
| `FaultType` | `string` | Fault type |
| `Description` | `string` | Description |
| `Reason` | `string` | Reason |
| `RecommendedActions` | `string[]` | Recommended actions |
| `RecoveryActions` | `string[]` | Recovery actions |
| `FaultingObject` | `string` | Faulting object |
| `FaultingObjectType` | `string` | Object type |
| `FaultTime` | `string` | Fault time |
| `PerceivedSeverity` | `uint16` | Severity |

### MSCluster_HealthMetric

Health metric.

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `Name` | `string` | Metric name |
| `Value` | `uint64` | Metric value |
| `Units` | `string` | Units |

---

## Common State Values

### Node State

| Value | Description |
|-------|-------------|
| 0 | Unknown |
| 1 | Up |
| 2 | Down |
| 3 | Paused |
| 4 | Joining |

### Resource State

| Value | Description |
|-------|-------------|
| 0 | Unknown |
| 1 | Inherited |
| 2 | Initializing |
| 3 | Online |
| 4 | Offline |
| 128 | Pending |
| 129 | Online Pending |
| 130 | Offline Pending |

### Group State

| Value | Description |
|-------|-------------|
| 0 | Unknown |
| 1 | Online |
| 2 | Offline |
| 3 | Failed |
| 4 | Partial Online |
| 128 | Pending |

### Network Role

| Value | Description |
|-------|-------------|
| 0 | None (disabled) |
| 1 | Cluster Only (internal) |
| 3 | Cluster and Client |

---

## Common Return Values

| Value | Description |
|-------|-------------|
| 0 | Success |
| 1 | Access denied |
| 2 | Invalid parameter |
| 3 | Node not found |
| 4 | Resource not found |
| 5 | Group not found |
| 6 | Operation failed |
| 7 | Resource busy |
| 8 | Quorum loss |
| 5024 | Cluster service not running |
| 5041 | Node already member |
| 5042 | Node not member |
| 5050 | Resource online |
| 5051 | Resource offline |
| 5062 | Dependency not found |
| 5066 | Resource pending |

---

## Usage Examples

### PowerShell - Get Cluster Nodes

```powershell
Get-WmiObject -Namespace root\MSCluster -Class MSCluster_Node |
    Select-Object Name, State, NodeWeight
```

### PowerShell - Move Resource Group

```powershell
$group = Get-WmiObject -Namespace root\MSCluster -Class MSCluster_ResourceGroup -Filter "Name='Available Storage'"
$group.MoveToNewNode("Node2", 0, "Maintenance")
```

### WMI Query - Get Online Resources

```
SELECT * FROM MSCluster_Resource WHERE State = 3
```

### WMI Query - Get CSV Status

```
SELECT Name, State, RedirectedAccess FROM MSCluster_ClusterSharedVolume
```

### PowerShell - Create Resource

```powershell
$resource = ([WmiClass]"\\.\root\MSCluster:MSCluster_Resource").CreateInstance()
$resource.CreateResource("Available Storage", "New Disk", "Physical Disk", $false)
```

### PowerShell - Pause Node

```powershell
$node = Get-WmiObject -Namespace root\MSCluster -Class MSCluster_Node -Filter "Name='Node1'"
$node.Pause(1, "", "Maintenance")  # DrainType=1 (drain with failover)
```

### PowerShell - Resume Node

```powershell
$node = Get-WmiObject -Namespace root\MSCluster -Class MSCluster_Node -Filter "Name='Node1'"
$node.Resume(1, "Maintenance complete")  # FailbackType=1 (failback)
```
