# Group Management

## Overview

Groups (also called roles) are collections of cluster resources that fail over together. Groups provide logical organization and failover units for clustered applications and services.

## Group States

```rust
// CLUSTER_GROUP_STATE values
const ClusterGroupStateUnknown: i32 = -1;
const ClusterGroupOnline: i32 = 0;
const ClusterGroupOffline: i32 = 1;
const ClusterGroupFailed: i32 = 2;
const ClusterGroupPartialOnline: i32 = 3;
const ClusterGroupPending: i32 = 4;
```

## Opening and Closing Groups

### Open Group by Name

```rust
use windows::Win32::Networking::Clustering::*;

unsafe fn open_group(cluster: HCLUSTER, group_name: &str) -> windows::core::Result<HGROUP> {
    let name_wide: Vec<u16> = group_name.encode_utf16().chain(Some(0)).collect();

    let group = OpenClusterGroup(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
    );

    if group.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(group)
    }
}
```

### Open Group with Extended Options

```rust
unsafe fn open_group_ex(
    cluster: HCLUSTER,
    group_name: &str,
    desired_access: u32,
) -> windows::core::Result<HGROUP> {
    let name_wide: Vec<u16> = group_name.encode_utf16().chain(Some(0)).collect();

    let group = OpenClusterGroupEx(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
        desired_access,
        None,  // Granted access
    );

    if group.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(group)
    }
}
```

### Close Group Handle

```rust
unsafe fn close_group(group: HGROUP) -> windows::core::Result<()> {
    CloseClusterGroup(group)
}
```

## Creating Groups

### Create Simple Group

```rust
unsafe fn create_group(cluster: HCLUSTER, name: &str) -> windows::core::Result<HGROUP> {
    let name_wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();

    let group = CreateClusterGroup(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
    );

    if group.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(group)
    }
}
```

### Create Group with Extended Options

```rust
unsafe fn create_group_ex(
    cluster: HCLUSTER,
    name: &str,
    group_type: CLUSGROUP_TYPE,
) -> windows::core::Result<HGROUP> {
    let name_wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();

    let group_info = CLUSTER_CREATE_GROUP_INFO {
        dwVersion: CLUSTER_CREATE_GROUP_INFO_VERSION as u32,
        groupType: group_type,
    };

    let group = CreateClusterGroupEx(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
        Some(&group_info),
    );

    if group.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(group)
    }
}

// Common group types:
// ClusGroupTypeUnknown = 0
// ClusGroupTypeVirtualMachine = 100
// ClusGroupTypeFileServer = 102
// ClusGroupTypeGenericApplication = 300
// ClusGroupTypeGenericService = 301
// ClusGroupTypeGenericScript = 302
```

## Deleting Groups

### Delete Group

```rust
unsafe fn delete_group(group: HGROUP) -> u32 {
    // Group must be offline and have no resources
    DeleteClusterGroup(group)
}
```

### Destroy Group (Force Delete with Resources)

```rust
unsafe fn destroy_group(group: HGROUP) -> u32 {
    // Takes group offline, deletes all resources, then deletes group
    DestroyClusterGroup(group)
}

unsafe fn destroy_group_with_reason(group: HGROUP, reason: &str) -> u32 {
    let reason_wide: Vec<u16> = reason.encode_utf16().chain(Some(0)).collect();

    DestroyClusterGroupEx(
        group,
        windows::core::PCWSTR(reason_wide.as_ptr()),
    )
}
```

## Getting Group Information

### Get Group State

```rust
unsafe fn get_group_state(group: HGROUP) -> (CLUSTER_GROUP_STATE, String) {
    let mut node_name = [0u16; 256];
    let mut node_name_len = 256u32;

    let state = GetClusterGroupState(
        group,
        Some(windows::core::PWSTR(node_name.as_mut_ptr())),
        Some(&mut node_name_len),
    );

    let owner = String::from_utf16_lossy(&node_name[..node_name_len as usize]);
    (state, owner)
}

unsafe fn print_group_state(group: HGROUP) {
    let (state, owner) = get_group_state(group);

    let state_str = match state.0 {
        -1 => "Unknown",
        0 => "Online",
        1 => "Offline",
        2 => "Failed",
        3 => "Partial Online",
        4 => "Pending",
        _ => "Invalid",
    };

    println!("Group state: {} on node: {}", state_str, owner);
}
```

### Get Group Registry Key

```rust
#[cfg(feature = "Win32_System_Registry")]
unsafe fn get_group_key(group: HGROUP) -> windows::core::Result<super::super::System::Registry::HKEY> {
    use windows::Win32::System::Registry::KEY_READ;

    GetClusterGroupKey(group, KEY_READ.0)
}
```

### Get Cluster from Group

```rust
unsafe fn get_cluster_from_group(group: HGROUP) -> HCLUSTER {
    GetClusterFromGroup(group)
}
```

## Online/Offline Operations

### Bring Group Online

```rust
unsafe fn online_group(group: HGROUP, target_node: Option<HNODE>) -> u32 {
    OnlineClusterGroup(group, target_node)
}
```

### Bring Group Online with Extended Options

```rust
unsafe fn online_group_ex(
    group: HGROUP,
    target_node: Option<HNODE>,
    flags: u32,
) -> u32 {
    OnlineClusterGroupEx(
        group,
        target_node,
        flags,
        None,  // Input buffer
    )
}

// Flags:
// CLUSAPI_GROUP_ONLINE_IGNORE_RESOURCE_STATUS = 1
// CLUSAPI_GROUP_ONLINE_SYNCHRONOUS = 2
// CLUSAPI_GROUP_ONLINE_BEST_POSSIBLE_NODE = 4
// CLUSAPI_GROUP_ONLINE_IGNORE_AFFINITY_RULE = 8
```

### Take Group Offline

```rust
unsafe fn offline_group(group: HGROUP) -> u32 {
    OfflineClusterGroup(group)
}
```

### Take Group Offline with Extended Options

```rust
unsafe fn offline_group_ex(
    group: HGROUP,
    flags: u32,
) -> u32 {
    OfflineClusterGroupEx(
        group,
        flags,
        None,  // Input buffer
    )
}

// Flags:
// CLUSAPI_GROUP_OFFLINE_IGNORE_RESOURCE_STATUS = 1
```

### Offline with Reason

```rust
unsafe fn offline_group_with_reason(
    group: HGROUP,
    flags: u32,
    reason: &str,
) -> u32 {
    let reason_wide: Vec<u16> = reason.encode_utf16().chain(Some(0)).collect();

    OfflineClusterGroupEx2(
        group,
        flags,
        None,
        0,
        windows::core::PCWSTR(reason_wide.as_ptr()),
    )
}
```

## Moving Groups

### Simple Move

```rust
unsafe fn move_group(group: HGROUP, target_node: Option<HNODE>) -> u32 {
    MoveClusterGroup(group, target_node)
}
```

### Move with Extended Options

```rust
unsafe fn move_group_ex(
    group: HGROUP,
    target_node: Option<HNODE>,
    flags: u32,
) -> u32 {
    MoveClusterGroupEx(
        group,
        target_node,
        flags,
        None,  // Input buffer
    )
}

// Move flags:
// CLUSAPI_GROUP_MOVE_IGNORE_RESOURCE_STATUS = 1
// CLUSAPI_GROUP_MOVE_RETURN_TO_SOURCE_NODE_ON_ERROR = 2
// CLUSAPI_GROUP_MOVE_QUEUE_ENABLED = 4
// CLUSAPI_GROUP_MOVE_HIGH_PRIORITY_START = 8
// CLUSAPI_GROUP_MOVE_FAILBACK = 16
// CLUSAPI_GROUP_MOVE_IGNORE_AFFINITY_RULE = 32
```

### Move with Reason

```rust
unsafe fn move_group_with_reason(
    group: HGROUP,
    target_node: Option<HNODE>,
    flags: u32,
    reason: &str,
) -> u32 {
    let reason_wide: Vec<u16> = reason.encode_utf16().chain(Some(0)).collect();

    MoveClusterGroupEx2(
        group,
        target_node,
        flags,
        None,
        windows::core::PCWSTR(reason_wide.as_ptr()),
    )
}
```

### Cancel Group Operation

```rust
unsafe fn cancel_group_operation(group: HGROUP) -> u32 {
    CancelClusterGroupOperation(group, 0)
}
```

## Group Enumeration

### Enumerate Groups

```rust
unsafe fn enumerate_groups(cluster: HCLUSTER) {
    let henum = ClusterOpenEnum(cluster, CLUSTER_ENUM_GROUP.0 as u32);

    if henum.is_invalid() {
        return;
    }

    let mut index = 0u32;
    loop {
        let mut obj_type = 0u32;
        let mut name = [0u16; 256];
        let mut name_len = 256u32;

        let result = ClusterEnum(
            henum,
            index,
            &mut obj_type,
            windows::core::PWSTR(name.as_mut_ptr()),
            &mut name_len,
        );

        if result != 0 {
            break;
        }

        println!("Group: {}", String::from_utf16_lossy(&name[..name_len as usize]));
        index += 1;
    }

    ClusterCloseEnum(henum);
}
```

### Enumerate Resources in Group

```rust
unsafe fn enumerate_group_resources(group: HGROUP) {
    let henum = ClusterGroupOpenEnum(group, CLUSTER_GROUP_ENUM_CONTAINS.0 as u32);

    if henum.is_invalid() {
        return;
    }

    let mut index = 0u32;
    loop {
        let mut obj_type = 0u32;
        let mut name = [0u16; 256];
        let mut name_len = 256u32;

        let result = ClusterGroupEnum(
            henum,
            index,
            &mut obj_type,
            windows::core::PWSTR(name.as_mut_ptr()),
            &mut name_len,
        );

        if result != 0 {
            break;
        }

        println!("  Resource: {}", String::from_utf16_lossy(&name[..name_len as usize]));
        index += 1;
    }

    ClusterGroupCloseEnum(henum);
}
```

### Extended Group Enumeration

```rust
unsafe fn enumerate_groups_with_details(cluster: HCLUSTER) {
    let henum = ClusterGroupOpenEnumEx(
        cluster,
        windows::core::PCWSTR::null(),  // Properties filter
        0,
        windows::core::PCWSTR::null(),  // RO properties filter
        0,
        0,  // Flags
    );

    if henum.is_invalid() {
        return;
    }

    let mut index = 0u32;
    loop {
        let mut item = CLUSTER_GROUP_ENUM_ITEM::default();
        let mut item_size = std::mem::size_of::<CLUSTER_GROUP_ENUM_ITEM>() as u32;

        let result = ClusterGroupEnumEx(henum, index, &mut item, &mut item_size);

        if result != 0 {
            break;
        }

        // item contains:
        // - dwVersion
        // - cbId / lpszId
        // - cbName / lpszName
        // - state
        // - cbOwnerNode / lpszOwnerNode
        // - dwFlags

        index += 1;
    }

    ClusterGroupCloseEnumEx(henum);
}
```

## Group Control Operations

### Get Group Properties

```rust
unsafe fn get_group_properties(group: HGROUP) {
    let mut buffer = vec![0u8; 4096];
    let mut bytes_returned = 0u32;

    let result = ClusterGroupControl(
        group,
        None,
        CLCTL_GET_COMMON_PROPERTIES.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        buffer.len() as u32,
        Some(&mut bytes_returned),
    );

    if result == 0 {
        println!("Group properties: {} bytes", bytes_returned);
    }
}
```

### Set Group Properties

```rust
unsafe fn set_group_property(
    group: HGROUP,
    property_name: &str,
    value: &str,
) -> u32 {
    // Build property list buffer
    // Format: CLUSPROP_LIST header + property entries

    let mut buffer = Vec::new();
    // ... build property list ...

    ClusterGroupControl(
        group,
        None,
        CLCTL_SET_COMMON_PROPERTIES.0 as u32,
        Some(buffer.as_ptr() as *const _),
        buffer.len() as u32,
        None,
        0,
        None,
    )
}
```

### Get Last Move Time

```rust
unsafe fn get_last_move_time(group: HGROUP) -> Option<CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT> {
    let mut output = CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT::default();
    let mut bytes_returned = 0u32;

    let result = ClusterGroupControl(
        group,
        None,
        CLCTL_GROUP_GET_LAST_MOVE_TIME.0 as u32,
        None,
        0,
        Some(&mut output as *mut _ as *mut _),
        std::mem::size_of::<CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT>() as u32,
        Some(&mut bytes_returned),
    );

    if result == 0 {
        Some(output)
    } else {
        None
    }
}
```

## Group Sets

### Create Group Set

```rust
unsafe fn create_group_set(cluster: HCLUSTER, name: &str) -> windows::core::Result<HGROUPSET> {
    let name_wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();

    let groupset = CreateClusterGroupSet(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
    );

    if groupset.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(groupset)
    }
}
```

### Add Group to Group Set

```rust
unsafe fn add_to_group_set(groupset: HGROUPSET, group: HGROUP) -> u32 {
    ClusterAddGroupToGroupSet(groupset, group)
}

unsafe fn add_to_group_set_with_domains(
    groupset: HGROUPSET,
    group: HGROUP,
    fault_domain: u32,
    update_domain: u32,
) -> u32 {
    ClusterAddGroupToGroupSetWithDomains(groupset, group, fault_domain, update_domain)
}
```

### Remove Group from Group Set

```rust
unsafe fn remove_from_group_set(group: HGROUP) -> u32 {
    ClusterRemoveGroupFromGroupSet(group)
}
```

### Enumerate Group Sets

```rust
unsafe fn enumerate_group_sets(cluster: HCLUSTER) {
    let henum = ClusterGroupSetOpenEnum(cluster);

    if henum.is_invalid() {
        return;
    }

    let mut index = 0u32;
    loop {
        let mut name = [0u16; 256];
        let mut name_len = 256u32;

        let result = ClusterGroupSetEnum(
            henum,
            index,
            windows::core::PWSTR(name.as_mut_ptr()),
            &mut name_len,
        );

        if result != 0 {
            break;
        }

        println!("Group Set: {}",
            String::from_utf16_lossy(&name[..name_len as usize]));

        index += 1;
    }

    ClusterGroupSetCloseEnum(henum);
}
```

### Delete Group Set

```rust
unsafe fn delete_group_set(groupset: HGROUPSET) -> u32 {
    DeleteClusterGroupSet(groupset)
}
```

## Group Dependencies

### Add Group Dependency

```rust
unsafe fn add_group_dependency(
    dependent: HGROUP,
    provider: HGROUP,
) -> u32 {
    AddClusterGroupDependency(dependent, provider)
}
```

### Remove Group Dependency

```rust
unsafe fn remove_group_dependency(
    dependent: HGROUP,
    provider: HGROUP,
) -> u32 {
    RemoveClusterGroupDependency(dependent, provider)
}
```

### Add Group Set Dependency

```rust
unsafe fn add_groupset_dependency(
    dependent: HGROUPSET,
    provider: HGROUPSET,
) -> u32 {
    AddClusterGroupSetDependency(dependent, provider)
}
```

### Add Group to Group Set Dependency

```rust
unsafe fn add_group_to_groupset_dependency(
    dependent_group: HGROUP,
    provider_groupset: HGROUPSET,
) -> u32 {
    AddClusterGroupToGroupSetDependency(dependent_group, provider_groupset)
}
```

## Affinity Rules

### Create Affinity Rule

```rust
unsafe fn create_affinity_rule(
    cluster: HCLUSTER,
    rule_name: &str,
    rule_type: CLUS_AFFINITY_RULE_TYPE,
) -> u32 {
    let name_wide: Vec<u16> = rule_name.encode_utf16().chain(Some(0)).collect();

    ClusterCreateAffinityRule(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
        rule_type,
    )
}

// Rule types:
// ClusAffinityRuleNone = 0
// ClusAffinityRuleSameFaultDomain = 1
// ClusAffinityRuleSameNode = 2
// ClusAffinityRuleDifferentFaultDomain = 3
// ClusAffinityRuleDifferentNode = 4
```

### Add Group to Affinity Rule

```rust
unsafe fn add_to_affinity_rule(
    cluster: HCLUSTER,
    rule_name: &str,
    group: HGROUP,
) -> u32 {
    let name_wide: Vec<u16> = rule_name.encode_utf16().chain(Some(0)).collect();

    ClusterAddGroupToAffinityRule(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
        group,
    )
}
```

### Remove Affinity Rule

```rust
unsafe fn remove_affinity_rule(
    cluster: HCLUSTER,
    rule_name: &str,
) -> u32 {
    let name_wide: Vec<u16> = rule_name.encode_utf16().chain(Some(0)).collect();

    ClusterRemoveAffinityRule(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
    )
}
```

## Complete Example

```rust
use windows::Win32::Networking::Clustering::*;

unsafe fn manage_file_server_group(cluster: HCLUSTER) {
    // Create file server group
    let group_info = CLUSTER_CREATE_GROUP_INFO {
        dwVersion: CLUSTER_CREATE_GROUP_INFO_VERSION as u32,
        groupType: ClusGroupTypeFileServer,
    };

    let group = CreateClusterGroupEx(
        cluster,
        windows::core::w!("FileServerRole"),
        Some(&group_info),
    );

    if group.is_invalid() {
        println!("Failed to create group");
        return;
    }

    // Add resources to the group...
    // (See resource management documentation)

    // Bring online on best possible node
    let result = OnlineClusterGroupEx(
        group,
        None,  // Let cluster decide
        4,     // CLUSAPI_GROUP_ONLINE_BEST_POSSIBLE_NODE
        None,
    );

    if result == 0 {
        println!("Group online successfully");
    }

    // Monitor state
    let (state, owner) = get_group_state(group);
    println!("Group is {:?} on {}", state, owner);

    // Move to another node if needed
    // First, find another node
    // let target_node = ...;
    // MoveClusterGroupEx(group, Some(target_node), 0, None);

    // Clean up
    let _ = CloseClusterGroup(group);
}
```
