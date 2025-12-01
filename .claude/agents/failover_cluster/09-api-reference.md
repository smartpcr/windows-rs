# API Reference

## Overview

Complete reference for Windows Failover Clustering APIs in windows-rs. The APIs are organized by category.

## Required Feature

```toml
[dependencies.windows]
version = "0.59"
features = ["Win32_Networking_Clustering"]
```

## Handle Types

| Type | Description |
|------|-------------|
| `HCLUSTER` | Cluster connection handle |
| `HNODE` | Node handle |
| `HGROUP` | Group handle |
| `HGROUPSET` | Group set handle |
| `HRESOURCE` | Resource handle |
| `HNETWORK` | Network handle |
| `HNETINTERFACE` | Network interface handle |
| `HCHANGE` | Notification port handle |
| `HCLUSENUM` | Cluster enumeration handle |
| `HCLUSENUMEX` | Extended cluster enumeration |
| `HNODEENUM` | Node enumeration handle |
| `HNODEENUMEX` | Extended node enumeration |
| `HGROUPENUM` | Group enumeration handle |
| `HGROUPENUMEX` | Extended group enumeration |
| `HGROUPSETENUM` | Group set enumeration |
| `HRESENUM` | Resource enumeration handle |
| `HRESENUMEX` | Extended resource enumeration |
| `HRESTYPEENUM` | Resource type enumeration |
| `HNETWORKENUM` | Network enumeration handle |
| `HNETINTERFACEENUM` | Network interface enumeration |
| `HREGBATCH` | Registry batch handle |
| `HREGBATCHPORT` | Registry batch notification port |
| `HREGREADBATCH` | Registry read batch handle |
| `HREGREADBATCHREPLY` | Registry read batch reply |
| `HREGBATCHNOTIFICATION` | Registry batch notification |
| `HCLUSCRYPTPROVIDER` | Cluster crypto provider |

## Cluster Lifecycle Functions

### Connection Functions

| Function | Description |
|----------|-------------|
| `OpenCluster(name)` | Open cluster by name |
| `OpenClusterEx(name, flags, desired_access)` | Open with extended options |
| `CloseCluster(cluster)` | Close cluster handle |
| `GetClusterInformation(...)` | Get cluster name and version |

### Creation/Destruction Functions

| Function | Description |
|----------|-------------|
| `CreateCluster(config, callback, context)` | Create new cluster |
| `DestroyCluster(cluster, callback, context, delete_vcos)` | Destroy cluster |
| `CreateClusterNameAccount(...)` | Create cluster name account |
| `ClusterUpgradeFunctionalLevel(...)` | Upgrade cluster level |

### Backup/Restore Functions

| Function | Description |
|----------|-------------|
| `BackupClusterDatabase(cluster, path)` | Backup cluster database |
| `RestoreClusterDatabase(path, force, new_name)` | Restore cluster database |
| `ClusterRegSyncDatabase(cluster, flags)` | Sync registry database |

## Node Functions

### Node Handle Functions

| Function | Description |
|----------|-------------|
| `OpenClusterNode(cluster, name)` | Open node by name |
| `OpenClusterNodeEx(cluster, name, access, granted)` | Open with extended options |
| `OpenClusterNodeById(cluster, id)` | Open node by ID |
| `CloseClusterNode(node)` | Close node handle |

### Node State Functions

| Function | Description |
|----------|-------------|
| `GetClusterNodeState(node)` | Get node state |
| `GetClusterNodeId(node, buffer, len)` | Get node ID |
| `GetClusterFromNode(node)` | Get cluster from node |
| `GetNodeClusterState(name, state)` | Get cluster state on node |

### Node Management Functions

| Function | Description |
|----------|-------------|
| `AddClusterNode(cluster, name, callback, context)` | Add node to cluster |
| `AddClusterNodeEx(cluster, name, flags, callback, context)` | Add with flags |
| `AddClusterStorageNode(...)` | Add storage node |
| `EvictClusterNode(node)` | Evict node from cluster |
| `EvictClusterNodeEx(node, timeout, status)` | Evict with timeout |
| `EvictClusterNodeEx2(node, timeout, status, reason)` | Evict with reason |
| `PauseClusterNode(node)` | Pause node |
| `PauseClusterNodeEx(node, drain, type, target)` | Pause with drain |
| `PauseClusterNodeEx2(...)` | Pause with timeout and reason |
| `ResumeClusterNode(node)` | Resume node |
| `ResumeClusterNodeEx(node, failback_type, flags)` | Resume with failback |
| `ResumeClusterNodeEx2(...)` | Resume with reason |
| `ClusterNodeReplacement(cluster, current, new)` | Replace node |

### Node Enumeration Functions

| Function | Description |
|----------|-------------|
| `ClusterNodeOpenEnum(node, type)` | Open node enumeration |
| `ClusterNodeOpenEnumEx(node, type, options)` | Extended enumeration |
| `ClusterNodeEnum(enum, index, type, name, len)` | Enumerate |
| `ClusterNodeEnumEx(enum, index, item, size)` | Extended enumerate |
| `ClusterNodeGetEnumCount(enum)` | Get count |
| `ClusterNodeGetEnumCountEx(enum)` | Extended count |
| `ClusterNodeCloseEnum(enum)` | Close enumeration |
| `ClusterNodeCloseEnumEx(enum)` | Close extended enumeration |

### Node Control Functions

| Function | Description |
|----------|-------------|
| `ClusterNodeControl(...)` | Node control operation |
| `ClusterNodeControlEx(...)` | Control with reason |

## Group Functions

### Group Handle Functions

| Function | Description |
|----------|-------------|
| `OpenClusterGroup(cluster, name)` | Open group by name |
| `OpenClusterGroupEx(cluster, name, access, granted)` | Open with access |
| `CloseClusterGroup(group)` | Close group handle |
| `GetClusterGroupState(group, node, len)` | Get group state |
| `GetClusterFromGroup(group)` | Get cluster from group |

### Group Management Functions

| Function | Description |
|----------|-------------|
| `CreateClusterGroup(cluster, name)` | Create group |
| `CreateClusterGroupEx(cluster, name, info)` | Create with info |
| `DeleteClusterGroup(group)` | Delete empty group |
| `DeleteClusterGroupEx(group, reason)` | Delete with reason |
| `DestroyClusterGroup(group)` | Delete group and resources |
| `DestroyClusterGroupEx(group, reason)` | Destroy with reason |

### Group State Functions

| Function | Description |
|----------|-------------|
| `OnlineClusterGroup(group, node)` | Bring online |
| `OnlineClusterGroupEx(group, node, flags, buffer, size)` | Online with flags |
| `OnlineClusterGroupEx2(...)` | Online with reason |
| `OfflineClusterGroup(group)` | Take offline |
| `OfflineClusterGroupEx(group, flags, buffer, size)` | Offline with flags |
| `OfflineClusterGroupEx2(...)` | Offline with reason |
| `MoveClusterGroup(group, node)` | Move group |
| `MoveClusterGroupEx(group, node, flags, buffer, size)` | Move with flags |
| `MoveClusterGroupEx2(...)` | Move with reason |
| `CancelClusterGroupOperation(group, flags)` | Cancel operation |

### Group Enumeration Functions

| Function | Description |
|----------|-------------|
| `ClusterGroupOpenEnum(group, type)` | Open enumeration |
| `ClusterGroupOpenEnumEx(...)` | Extended enumeration |
| `ClusterGroupEnum(...)` | Enumerate |
| `ClusterGroupEnumEx(...)` | Extended enumerate |
| `ClusterGroupGetEnumCount(enum)` | Get count |
| `ClusterGroupGetEnumCountEx(enum)` | Extended count |
| `ClusterGroupCloseEnum(enum)` | Close enumeration |
| `ClusterGroupCloseEnumEx(enum)` | Close extended |

### Group Control Functions

| Function | Description |
|----------|-------------|
| `ClusterGroupControl(...)` | Group control operation |
| `ClusterGroupControlEx(...)` | Control with reason |

## Group Set Functions

| Function | Description |
|----------|-------------|
| `CreateClusterGroupSet(cluster, name)` | Create group set |
| `OpenClusterGroupSet(cluster, name)` | Open group set |
| `CloseClusterGroupSet(groupset)` | Close handle |
| `DeleteClusterGroupSet(groupset)` | Delete group set |
| `DeleteClusterGroupSetEx(groupset, reason)` | Delete with reason |
| `ClusterAddGroupToGroupSet(groupset, group)` | Add group |
| `ClusterAddGroupToGroupSetWithDomains(...)` | Add with domains |
| `ClusterRemoveGroupFromGroupSet(group)` | Remove group |
| `ClusterRemoveGroupFromGroupSetEx(group, reason)` | Remove with reason |
| `ClusterGroupSetOpenEnum(cluster)` | Open enumeration |
| `ClusterGroupSetEnum(...)` | Enumerate |
| `ClusterGroupSetGetEnumCount(enum)` | Get count |
| `ClusterGroupSetCloseEnum(enum)` | Close enumeration |
| `ClusterGroupSetControl(...)` | Control operation |
| `ClusterGroupSetControlEx(...)` | Control with reason |

## Resource Functions

### Resource Handle Functions

| Function | Description |
|----------|-------------|
| `OpenClusterResource(cluster, name)` | Open resource |
| `OpenClusterResourceEx(cluster, name, access, granted)` | Open with access |
| `CloseClusterResource(resource)` | Close handle |
| `GetClusterResourceState(...)` | Get state |
| `GetClusterFromResource(resource)` | Get cluster |

### Resource Management Functions

| Function | Description |
|----------|-------------|
| `CreateClusterResource(group, name, type, flags)` | Create resource |
| `CreateClusterResourceEx(...)` | Create with reason |
| `DeleteClusterResource(resource)` | Delete resource |
| `DeleteClusterResourceEx(resource, reason)` | Delete with reason |
| `OnlineClusterResource(resource)` | Bring online |
| `OnlineClusterResourceEx(resource, flags, buffer, size)` | Online with flags |
| `OnlineClusterResourceEx2(...)` | Online with reason |
| `OfflineClusterResource(resource)` | Take offline |
| `OfflineClusterResourceEx(resource, flags, buffer, size)` | Offline with flags |
| `OfflineClusterResourceEx2(...)` | Offline with reason |
| `FailClusterResource(resource)` | Simulate failure |
| `FailClusterResourceEx(resource, reason)` | Fail with reason |

### Resource Dependency Functions

| Function | Description |
|----------|-------------|
| `AddClusterResourceDependency(resource, depends_on)` | Add dependency |
| `AddClusterResourceDependencyEx(...)` | Add with reason |
| `RemoveClusterResourceDependency(resource, depends_on)` | Remove dependency |
| `RemoveClusterResourceDependencyEx(...)` | Remove with reason |
| `SetClusterResourceDependencyExpression(resource, expr)` | Set expression |
| `GetClusterResourceDependencyExpression(resource, ...)` | Get expression |
| `CanResourceBeDependent(resource, dependent)` | Check validity |

### Resource Node Functions

| Function | Description |
|----------|-------------|
| `AddClusterResourceNode(resource, node)` | Add possible owner |
| `AddClusterResourceNodeEx(...)` | Add with reason |
| `RemoveClusterResourceNode(resource, node)` | Remove owner |
| `RemoveClusterResourceNodeEx(...)` | Remove with reason |

### Resource Group Functions

| Function | Description |
|----------|-------------|
| `ChangeClusterResourceGroup(resource, group)` | Change group |
| `ChangeClusterResourceGroupEx(resource, group, flags)` | Change with flags |
| `ChangeClusterResourceGroupEx2(...)` | Change with reason |

### Resource Enumeration Functions

| Function | Description |
|----------|-------------|
| `ClusterResourceOpenEnum(resource, type)` | Open enumeration |
| `ClusterResourceOpenEnumEx(...)` | Extended enumeration |
| `ClusterResourceEnum(...)` | Enumerate |
| `ClusterResourceEnumEx(...)` | Extended enumerate |
| `ClusterResourceGetEnumCount(enum)` | Get count |
| `ClusterResourceGetEnumCountEx(enum)` | Extended count |
| `ClusterResourceCloseEnum(enum)` | Close enumeration |
| `ClusterResourceCloseEnumEx(enum)` | Close extended |

### Resource Control Functions

| Function | Description |
|----------|-------------|
| `ClusterResourceControl(...)` | Control operation |
| `ClusterResourceControlAsUser(...)` | Control as user |
| `ClusterResourceControlEx(...)` | Control with reason |
| `ClusterResourceControlAsUserEx(...)` | As user with reason |

## Resource Type Functions

| Function | Description |
|----------|-------------|
| `CreateClusterResourceType(...)` | Create type |
| `CreateClusterResourceTypeEx(...)` | Create with reason |
| `DeleteClusterResourceType(cluster, name)` | Delete type |
| `DeleteClusterResourceTypeEx(...)` | Delete with reason |
| `ClusterResourceTypeOpenEnum(cluster, name, type)` | Open enumeration |
| `ClusterResourceTypeEnum(...)` | Enumerate |
| `ClusterResourceTypeGetEnumCount(enum)` | Get count |
| `ClusterResourceTypeCloseEnum(enum)` | Close enumeration |
| `ClusterResourceTypeControl(...)` | Control operation |
| `ClusterResourceTypeControlAsUser(...)` | Control as user |
| `ClusterResourceTypeControlEx(...)` | Control with reason |
| `ClusterResourceTypeControlAsUserEx(...)` | As user with reason |

## Network Functions

### Network Handle Functions

| Function | Description |
|----------|-------------|
| `OpenClusterNetwork(cluster, name)` | Open network |
| `OpenClusterNetworkEx(...)` | Open with access |
| `CloseClusterNetwork(network)` | Close handle |
| `GetClusterNetworkState(network)` | Get state |
| `GetClusterNetworkId(network, buffer, len)` | Get ID |
| `GetClusterFromNetwork(network)` | Get cluster |

### Network Enumeration Functions

| Function | Description |
|----------|-------------|
| `ClusterNetworkOpenEnum(network, type)` | Open enumeration |
| `ClusterNetworkEnum(...)` | Enumerate |
| `ClusterNetworkGetEnumCount(enum)` | Get count |
| `ClusterNetworkCloseEnum(enum)` | Close enumeration |

### Network Control Functions

| Function | Description |
|----------|-------------|
| `ClusterNetworkControl(...)` | Control operation |
| `ClusterNetworkControlEx(...)` | Control with reason |

## Network Interface Functions

| Function | Description |
|----------|-------------|
| `OpenClusterNetInterface(cluster, name)` | Open interface |
| `OpenClusterNetInterfaceEx(...)` | Open with access |
| `CloseClusterNetInterface(interface)` | Close handle |
| `GetClusterNetInterfaceState(interface)` | Get state |
| `GetClusterNetInterface(cluster, node, network, ...)` | Get by node/network |
| `GetClusterFromNetInterface(interface)` | Get cluster |
| `ClusterNetInterfaceOpenEnum(cluster, node, network)` | Open enumeration |
| `ClusterNetInterfaceEnum(...)` | Enumerate |
| `ClusterNetInterfaceCloseEnum(enum)` | Close enumeration |
| `ClusterNetInterfaceControl(...)` | Control operation |
| `ClusterNetInterfaceControlEx(...)` | Control with reason |

## Notification Functions

| Function | Description |
|----------|-------------|
| `CreateClusterNotifyPort(change, cluster, filter, key)` | Create port |
| `CreateClusterNotifyPortV2(...)` | Create V2 port |
| `CloseClusterNotifyPort(change)` | Close port |
| `RegisterClusterNotify(change, filter, handle, key)` | Register notification |
| `RegisterClusterNotifyV2(change, filter, handle, key)` | Register V2 |
| `GetClusterNotify(...)` | Get notification |
| `GetClusterNotifyV2(...)` | Get V2 notification |
| `GetNotifyEventHandle(change, event)` | Get event handle |

## Cluster Enumeration Functions

| Function | Description |
|----------|-------------|
| `ClusterOpenEnum(cluster, type)` | Open enumeration |
| `ClusterOpenEnumEx(cluster, type, options)` | Extended enumeration |
| `ClusterEnum(enum, index, type, name, len)` | Enumerate |
| `ClusterEnumEx(enum, index, item, size)` | Extended enumerate |
| `ClusterGetEnumCount(enum)` | Get count |
| `ClusterGetEnumCountEx(enum)` | Extended count |
| `ClusterCloseEnum(enum)` | Close enumeration |
| `ClusterCloseEnumEx(enum)` | Close extended |

## Control Functions

### Cluster Control

| Function | Description |
|----------|-------------|
| `ClusterControl(...)` | Cluster control |
| `ClusterControlEx(...)` | Control with reason |

## Registry Functions

| Function | Description |
|----------|-------------|
| `GetClusterKey(cluster, access)` | Get cluster key |
| `GetClusterGroupKey(group, access)` | Get group key |
| `GetClusterNodeKey(node, access)` | Get node key |
| `GetClusterResourceKey(resource, access)` | Get resource key |
| `GetClusterResourceTypeKey(cluster, type, access)` | Get type key |
| `GetClusterNetworkKey(network, access)` | Get network key |
| `GetClusterNetInterfaceKey(interface, access)` | Get interface key |
| `ClusterRegOpenKey(key, subkey, access, result)` | Open subkey |
| `ClusterRegCreateKey(...)` | Create subkey |
| `ClusterRegCreateKeyEx(...)` | Create with reason |
| `ClusterRegDeleteKey(key, subkey)` | Delete subkey |
| `ClusterRegDeleteKeyEx(...)` | Delete with reason |
| `ClusterRegCloseKey(key)` | Close key |
| `ClusterRegQueryValue(...)` | Query value |
| `ClusterRegSetValue(...)` | Set value |
| `ClusterRegSetValueEx(...)` | Set with reason |
| `ClusterRegDeleteValue(key, name)` | Delete value |
| `ClusterRegDeleteValueEx(...)` | Delete with reason |
| `ClusterRegEnumKey(...)` | Enumerate keys |
| `ClusterRegEnumValue(...)` | Enumerate values |
| `ClusterRegQueryInfoKey(...)` | Query key info |
| `ClusterRegGetKeySecurity(...)` | Get security |
| `ClusterRegSetKeySecurity(...)` | Set security |
| `ClusterRegSetKeySecurityEx(...)` | Set with reason |

## CSV Functions

| Function | Description |
|----------|-------------|
| `AddResourceToClusterSharedVolumes(resource)` | Add to CSV |
| `RemoveResourceFromClusterSharedVolumes(resource)` | Remove from CSV |
| `ClusterIsPathOnSharedVolume(path)` | Check if on CSV |
| `IsFileOnClusterSharedVolume(path, result)` | Check file on CSV |
| `ClusterGetVolumePath(file, path, len)` | Get volume path |
| `ClusterGetVolumeNameForVolumeMountPoint(...)` | Get volume name |
| `ClusterPrepareSharedVolumeForBackup(...)` | Prepare for backup |
| `ClusterClearBackupStateForSharedVolume(path)` | Clear backup state |
| `ClusterSharedVolumeSetSnapshotState(...)` | Set snapshot state |
| `GetClusterResourceNetworkName(...)` | Get network name |

## Health Fault Functions

| Function | Description |
|----------|-------------|
| `ClusAddClusterHealthFault(cluster, fault, flags)` | Add fault |
| `ClusRemoveClusterHealthFault(cluster, id, flags)` | Remove fault |
| `ClusGetClusterHealthFaults(cluster, faults, flags)` | Get faults |
| `InitializeClusterHealthFault(fault)` | Initialize fault |
| `InitializeClusterHealthFaultArray(array)` | Initialize array |
| `FreeClusterHealthFault(fault)` | Free fault |
| `FreeClusterHealthFaultArray(array)` | Free array |

## Worker Functions

| Function | Description |
|----------|-------------|
| `ClusWorkerCreate(worker, routine, parameter)` | Create worker |
| `ClusWorkerCheckTerminate(worker)` | Check terminate |
| `ClusWorkerTerminate(worker)` | Terminate worker |
| `ClusWorkerTerminateEx(worker, timeout, wait_only)` | Terminate with timeout |
| `ClusWorkersTerminate(workers, count, timeout, wait_only)` | Terminate multiple |

## Affinity Rule Functions

| Function | Description |
|----------|-------------|
| `ClusterCreateAffinityRule(cluster, name, type)` | Create rule |
| `ClusterRemoveAffinityRule(cluster, name)` | Remove rule |
| `ClusterAddGroupToAffinityRule(cluster, name, group)` | Add group |
| `ClusterRemoveGroupFromAffinityRule(cluster, name, group)` | Remove group |
| `ClusterAffinityRuleControl(...)` | Control operation |

## Dependency Functions

| Function | Description |
|----------|-------------|
| `AddClusterGroupDependency(dependent, provider)` | Group dependency |
| `AddClusterGroupDependencyEx(...)` | With reason |
| `RemoveClusterGroupDependency(dependent, provider)` | Remove group dep |
| `RemoveClusterGroupDependencyEx(...)` | With reason |
| `AddClusterGroupSetDependency(...)` | Group set dependency |
| `AddClusterGroupSetDependencyEx(...)` | With reason |
| `RemoveClusterGroupSetDependency(...)` | Remove groupset dep |
| `RemoveClusterGroupSetDependencyEx(...)` | With reason |
| `AddClusterGroupToGroupSetDependency(...)` | Group to groupset |
| `AddClusterGroupToGroupSetDependencyEx(...)` | With reason |
| `RemoveClusterGroupToGroupSetDependency(...)` | Remove group to groupset |
| `RemoveClusterGroupToGroupSetDependencyEx(...)` | With reason |
| `AddCrossClusterGroupSetDependency(...)` | Cross-cluster |

## Crypto Functions

| Function | Description |
|----------|-------------|
| `OpenClusterCryptProvider(resource, cryptprovider, ...)` | Open provider |
| `OpenClusterCryptProviderEx(...)` | Open extended |
| `CloseClusterCryptProvider(provider)` | Close provider |
| `ClusterEncrypt(provider, data, size, out, out_size)` | Encrypt |
| `ClusterDecrypt(provider, data, size, out, out_size)` | Decrypt |
| `FreeClusterCrypt(cryptinfo)` | Free crypto data |

## Quorum Functions

| Function | Description |
|----------|-------------|
| `GetClusterQuorumResource(...)` | Get quorum resource |
| `SetClusterQuorumResource(resource, device, log_size)` | Set quorum resource |

## Utility Functions

| Function | Description |
|----------|-------------|
| `DetermineClusterCloudTypeFromCluster(cluster, type)` | Detect cloud |
| `DetermineClusterCloudTypeFromNodelist(...)` | Detect from nodes |
| `DetermineCNOResTypeFromCluster(cluster, type)` | Detect CNO type |
| `DetermineCNOResTypeFromNodelist(...)` | Detect from nodes |
| `ClusterSetAccountAccess(cluster, sid, access, control)` | Set account access |
| `ClusapiSetReasonHandler(handler)` | Set reason handler |

## Key Constants

### Cluster Change Filters (V1)

```rust
CLUSTER_CHANGE_NODE_STATE
CLUSTER_CHANGE_NODE_DELETED
CLUSTER_CHANGE_NODE_ADDED
CLUSTER_CHANGE_NODE_PROPERTY
CLUSTER_CHANGE_RESOURCE_STATE
CLUSTER_CHANGE_RESOURCE_DELETED
CLUSTER_CHANGE_RESOURCE_ADDED
CLUSTER_CHANGE_RESOURCE_PROPERTY
CLUSTER_CHANGE_GROUP_STATE
CLUSTER_CHANGE_GROUP_DELETED
CLUSTER_CHANGE_GROUP_ADDED
CLUSTER_CHANGE_GROUP_PROPERTY
CLUSTER_CHANGE_NETWORK_STATE
CLUSTER_CHANGE_CLUSTER_STATE
CLUSTER_CHANGE_CLUSTER_PROPERTY
CLUSTER_CHANGE_ALL
```

### Control Codes

```rust
CLCTL_GET_CHARACTERISTICS
CLCTL_GET_FLAGS
CLCTL_GET_NAME
CLCTL_GET_ID
CLCTL_GET_COMMON_PROPERTIES
CLCTL_GET_RO_COMMON_PROPERTIES
CLCTL_SET_COMMON_PROPERTIES
CLCTL_GET_PRIVATE_PROPERTIES
CLCTL_GET_RO_PRIVATE_PROPERTIES
CLCTL_SET_PRIVATE_PROPERTIES
CLCTL_GET_REQUIRED_DEPENDENCIES
CLCTL_GET_RESOURCE_TYPE
CLCTL_GET_REGISTRY_CHECKPOINTS
CLCTL_GET_CRYPTO_CHECKPOINTS
```

### Object Types

```rust
CLUSTER_OBJECT_TYPE_CLUSTER
CLUSTER_OBJECT_TYPE_GROUP
CLUSTER_OBJECT_TYPE_RESOURCE
CLUSTER_OBJECT_TYPE_RESOURCE_TYPE
CLUSTER_OBJECT_TYPE_NETWORK_INTERFACE
CLUSTER_OBJECT_TYPE_NETWORK
CLUSTER_OBJECT_TYPE_NODE
CLUSTER_OBJECT_TYPE_REGISTRY
CLUSTER_OBJECT_TYPE_QUORUM
CLUSTER_OBJECT_TYPE_SHARED_VOLUME
CLUSTER_OBJECT_TYPE_GROUPSET
CLUSTER_OBJECT_TYPE_AFFINITYRULE
```

### Resource Create Flags

```rust
CLUSTER_RESOURCE_DEFAULT_MONITOR
CLUSTER_RESOURCE_SEPARATE_MONITOR
CLUSTER_RESOURCE_VALID_FLAGS
```

### Group Types

```rust
ClusGroupTypeUnknown
ClusGroupTypeCoreCluster
ClusGroupTypeVirtualMachine
ClusGroupTypeFileServer
ClusGroupTypeDhcpServer
ClusGroupTypeDtcServer
ClusGroupTypeMsmqServer
ClusGroupTypeGenericApplication
ClusGroupTypeGenericService
ClusGroupTypeGenericScript
```

## Error Handling

Most functions return:
- `u32` error code (0 = success, non-zero = Win32 error)
- `windows_core::BOOL` (TRUE = success)
- Handle (invalid = failure, use `is_invalid()`)
- `windows::core::Result<T>` for wrapped functions
