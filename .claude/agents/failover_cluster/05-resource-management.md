# Resource Management

## Overview

Resources are the individual components managed by a failover cluster. Each resource represents a specific workload element like an IP address, disk, or application. Resources are organized into groups and can have dependencies on other resources.

## Resource States

```rust
// CLUSTER_RESOURCE_STATE values
const ClusterResourceStateUnknown: i32 = -1;
const ClusterResourceInherited: i32 = 0;
const ClusterResourceInitializing: i32 = 1;
const ClusterResourceOnline: i32 = 2;
const ClusterResourceOffline: i32 = 3;
const ClusterResourceFailed: i32 = 4;
const ClusterResourcePending: i32 = 128;
const ClusterResourceOnlinePending: i32 = 129;
const ClusterResourceOfflinePending: i32 = 130;
```

## Opening and Closing Resources

### Open Resource by Name

```rust
use windows::Win32::Networking::Clustering::*;

unsafe fn open_resource(cluster: HCLUSTER, name: &str) -> windows::core::Result<HRESOURCE> {
    let name_wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();

    let resource = OpenClusterResource(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
    );

    if resource.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(resource)
    }
}
```

### Open Resource with Extended Options

```rust
unsafe fn open_resource_ex(
    cluster: HCLUSTER,
    name: &str,
    desired_access: u32,
) -> windows::core::Result<HRESOURCE> {
    let name_wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();

    let resource = OpenClusterResourceEx(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
        desired_access,
        None,  // Granted access
    );

    if resource.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(resource)
    }
}
```

### Close Resource Handle

```rust
unsafe fn close_resource(resource: HRESOURCE) -> windows::core::Result<()> {
    CloseClusterResource(resource)
}
```

## Creating Resources

### Create Resource

```rust
unsafe fn create_resource(
    group: HGROUP,
    name: &str,
    resource_type: &str,
) -> windows::core::Result<HRESOURCE> {
    let name_wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
    let type_wide: Vec<u16> = resource_type.encode_utf16().chain(Some(0)).collect();

    let resource = CreateClusterResource(
        group,
        windows::core::PCWSTR(name_wide.as_ptr()),
        windows::core::PCWSTR(type_wide.as_ptr()),
        0,  // Flags: CLUSTER_RESOURCE_CREATE_FLAGS
    );

    if resource.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(resource)
    }
}

// Common resource types:
// "Physical Disk"
// "IP Address"
// "Network Name"
// "File Share"
// "Generic Application"
// "Generic Service"
// "Generic Script"
// "Virtual Machine"
// "Virtual Machine Configuration"
```

### Create Resource with Flags

```rust
unsafe fn create_resource_separate_monitor(
    group: HGROUP,
    name: &str,
    resource_type: &str,
) -> windows::core::Result<HRESOURCE> {
    let name_wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
    let type_wide: Vec<u16> = resource_type.encode_utf16().chain(Some(0)).collect();

    // CLUSTER_RESOURCE_SEPARATE_MONITOR = 1
    let resource = CreateClusterResource(
        group,
        windows::core::PCWSTR(name_wide.as_ptr()),
        windows::core::PCWSTR(type_wide.as_ptr()),
        1,
    );

    if resource.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(resource)
    }
}
```

## Deleting Resources

### Delete Resource

```rust
unsafe fn delete_resource(resource: HRESOURCE) -> u32 {
    // Resource must be offline first
    DeleteClusterResource(resource)
}
```

### Delete with Reason

```rust
unsafe fn delete_resource_with_reason(resource: HRESOURCE, reason: &str) -> u32 {
    let reason_wide: Vec<u16> = reason.encode_utf16().chain(Some(0)).collect();

    DeleteClusterResourceEx(
        resource,
        windows::core::PCWSTR(reason_wide.as_ptr()),
    )
}
```

## Getting Resource Information

### Get Resource State

```rust
unsafe fn get_resource_state(resource: HRESOURCE) -> (CLUSTER_RESOURCE_STATE, String, String) {
    let mut node_name = [0u16; 256];
    let mut node_len = 256u32;
    let mut group_name = [0u16; 256];
    let mut group_len = 256u32;

    let state = GetClusterResourceState(
        resource,
        Some(windows::core::PWSTR(node_name.as_mut_ptr())),
        Some(&mut node_len),
        Some(windows::core::PWSTR(group_name.as_mut_ptr())),
        Some(&mut group_len),
    );

    (
        state,
        String::from_utf16_lossy(&node_name[..node_len as usize]),
        String::from_utf16_lossy(&group_name[..group_len as usize]),
    )
}

unsafe fn print_resource_state(resource: HRESOURCE) {
    let (state, node, group) = get_resource_state(resource);

    let state_str = match state.0 {
        -1 => "Unknown",
        0 => "Inherited",
        1 => "Initializing",
        2 => "Online",
        3 => "Offline",
        4 => "Failed",
        129 => "Online Pending",
        130 => "Offline Pending",
        _ => "Pending",
    };

    println!("Resource: {} in group {} on node {}", state_str, group, node);
}
```

### Get Resource Registry Key

```rust
#[cfg(feature = "Win32_System_Registry")]
unsafe fn get_resource_key(resource: HRESOURCE) -> windows::core::Result<super::super::System::Registry::HKEY> {
    use windows::Win32::System::Registry::KEY_READ;

    GetClusterResourceKey(resource, KEY_READ.0)
}
```

### Get Resource Network Name

```rust
unsafe fn get_resource_network_name(resource: HRESOURCE) -> windows::core::Result<String> {
    let mut buffer = [0u16; 256];
    let mut size = 256u32;

    GetClusterResourceNetworkName(
        resource,
        windows::core::PWSTR(buffer.as_mut_ptr()),
        &mut size,
    )?;

    Ok(String::from_utf16_lossy(&buffer[..size as usize]))
}
```

### Get Dependency Expression

```rust
unsafe fn get_dependency_expression(resource: HRESOURCE) -> String {
    let mut expr = [0u16; 1024];
    let mut expr_len = 1024u32;

    let result = GetClusterResourceDependencyExpression(
        resource,
        Some(windows::core::PWSTR(expr.as_mut_ptr())),
        &mut expr_len,
    );

    if result == 0 {
        String::from_utf16_lossy(&expr[..expr_len as usize])
    } else {
        String::new()
    }
}
```

### Get Cluster from Resource

```rust
unsafe fn get_cluster_from_resource(resource: HRESOURCE) -> HCLUSTER {
    GetClusterFromResource(resource)
}
```

## Online/Offline Operations

### Bring Resource Online

```rust
unsafe fn online_resource(resource: HRESOURCE) -> u32 {
    OnlineClusterResource(resource)
}
```

### Bring Resource Online with Extended Options

```rust
unsafe fn online_resource_ex(
    resource: HRESOURCE,
    flags: u32,
) -> u32 {
    OnlineClusterResourceEx(
        resource,
        flags,
        None,  // Input buffer
    )
}

// Flags:
// CLUSAPI_RESOURCE_ONLINE_IGNORE_RESOURCE_STATUS = 1
// CLUSAPI_RESOURCE_ONLINE_DO_NOT_UPDATE_PERSISTENT_STATE = 2
// CLUSAPI_RESOURCE_ONLINE_NECESSARY_FOR_QUORUM = 4
// CLUSAPI_RESOURCE_ONLINE_BEST_POSSIBLE_NODE = 8
// CLUSAPI_RESOURCE_ONLINE_IGNORE_AFFINITY_RULE = 32
```

### Take Resource Offline

```rust
unsafe fn offline_resource(resource: HRESOURCE) -> u32 {
    OfflineClusterResource(resource)
}
```

### Offline with Extended Options

```rust
unsafe fn offline_resource_ex(
    resource: HRESOURCE,
    flags: u32,
) -> u32 {
    OfflineClusterResourceEx(
        resource,
        flags,
        None,  // Input buffer
    )
}

// Flags:
// CLUSAPI_RESOURCE_OFFLINE_IGNORE_RESOURCE_STATUS = 1
// CLUSAPI_RESOURCE_OFFLINE_FORCE_WITH_TERMINATION = 2
// CLUSAPI_RESOURCE_OFFLINE_DO_NOT_UPDATE_PERSISTENT_STATE = 4
```

### Fail Resource (Simulate Failure)

```rust
unsafe fn fail_resource(resource: HRESOURCE) -> u32 {
    FailClusterResource(resource)
}
```

## Resource Dependencies

### Add Dependency

```rust
unsafe fn add_dependency(
    resource: HRESOURCE,
    depends_on: HRESOURCE,
) -> u32 {
    AddClusterResourceDependency(resource, depends_on)
}

unsafe fn add_dependency_with_reason(
    resource: HRESOURCE,
    depends_on: HRESOURCE,
    reason: &str,
) -> u32 {
    let reason_wide: Vec<u16> = reason.encode_utf16().chain(Some(0)).collect();

    AddClusterResourceDependencyEx(
        resource,
        depends_on,
        windows::core::PCWSTR(reason_wide.as_ptr()),
    )
}
```

### Remove Dependency

```rust
unsafe fn remove_dependency(
    resource: HRESOURCE,
    depends_on: HRESOURCE,
) -> u32 {
    RemoveClusterResourceDependency(resource, depends_on)
}
```

### Set Dependency Expression

```rust
unsafe fn set_dependency_expression(
    resource: HRESOURCE,
    expression: &str,
) -> u32 {
    let expr_wide: Vec<u16> = expression.encode_utf16().chain(Some(0)).collect();

    SetClusterResourceDependencyExpression(
        resource,
        windows::core::PCWSTR(expr_wide.as_ptr()),
    )
}

// Expression syntax:
// "[Resource1]" - Simple dependency
// "([Resource1] and [Resource2])" - Both required
// "([Resource1] or [Resource2])" - Either sufficient
// "(([Resource1] and [Resource2]) or [Resource3])" - Complex
```

### Check Dependency Validity

```rust
unsafe fn can_be_dependent(
    resource: HRESOURCE,
    dependent: HRESOURCE,
) -> bool {
    CanResourceBeDependent(resource, dependent).as_bool()
}
```

## Possible Owners

### Add Possible Owner

```rust
unsafe fn add_possible_owner(
    resource: HRESOURCE,
    node: HNODE,
) -> u32 {
    AddClusterResourceNode(resource, node)
}
```

### Remove Possible Owner

```rust
unsafe fn remove_possible_owner(
    resource: HRESOURCE,
    node: HNODE,
) -> u32 {
    RemoveClusterResourceNode(resource, node)
}
```

## Change Resource Group

### Move Resource to Another Group

```rust
unsafe fn change_resource_group(
    resource: HRESOURCE,
    new_group: HGROUP,
) -> u32 {
    ChangeClusterResourceGroup(resource, new_group)
}

unsafe fn change_resource_group_ex(
    resource: HRESOURCE,
    new_group: HGROUP,
    flags: u64,
) -> u32 {
    ChangeClusterResourceGroupEx(resource, new_group, flags)
}
```

## Resource Enumeration

### Enumerate Resources

```rust
unsafe fn enumerate_resources(cluster: HCLUSTER) {
    let henum = ClusterOpenEnum(cluster, CLUSTER_ENUM_RESOURCE.0 as u32);

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

        println!("Resource: {}",
            String::from_utf16_lossy(&name[..name_len as usize]));

        index += 1;
    }

    ClusterCloseEnum(henum);
}
```

### Enumerate Resource Dependencies

```rust
unsafe fn enumerate_resource_deps(resource: HRESOURCE) {
    let henum = ClusterResourceOpenEnum(
        resource,
        CLUSTER_RESOURCE_ENUM_DEPENDS.0 as u32 |
        CLUSTER_RESOURCE_ENUM_PROVIDES.0 as u32,
    );

    if henum.is_invalid() {
        return;
    }

    let mut index = 0u32;
    loop {
        let mut obj_type = 0u32;
        let mut name = [0u16; 256];
        let mut name_len = 256u32;

        let result = ClusterResourceEnum(
            henum,
            index,
            &mut obj_type,
            windows::core::PWSTR(name.as_mut_ptr()),
            &mut name_len,
        );

        if result != 0 {
            break;
        }

        let dep_type = if obj_type == CLUSTER_RESOURCE_ENUM_DEPENDS.0 as u32 {
            "depends on"
        } else {
            "provides for"
        };

        println!("  {} {}", dep_type,
            String::from_utf16_lossy(&name[..name_len as usize]));

        index += 1;
    }

    ClusterResourceCloseEnum(henum);
}
```

## Resource Control Operations

### Get Resource Properties

```rust
unsafe fn get_resource_properties(resource: HRESOURCE) {
    let mut buffer = vec![0u8; 8192];
    let mut bytes_returned = 0u32;

    let result = ClusterResourceControl(
        resource,
        None,
        CLCTL_GET_COMMON_PROPERTIES.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        buffer.len() as u32,
        Some(&mut bytes_returned),
    );

    if result == 0 {
        println!("Resource properties: {} bytes", bytes_returned);
        // Parse property list
    }
}
```

### Get Private Properties

```rust
unsafe fn get_private_properties(resource: HRESOURCE) {
    let mut buffer = vec![0u8; 8192];
    let mut bytes_returned = 0u32;

    let result = ClusterResourceControl(
        resource,
        None,
        CLCTL_GET_PRIVATE_PROPERTIES.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        buffer.len() as u32,
        Some(&mut bytes_returned),
    );

    if result == 0 {
        // Parse resource-type-specific properties
    }
}
```

### Get Required Dependencies

```rust
unsafe fn get_required_dependencies(resource: HRESOURCE) {
    let mut buffer = vec![0u8; 4096];
    let mut bytes_returned = 0u32;

    let result = ClusterResourceControl(
        resource,
        None,
        CLCTL_GET_REQUIRED_DEPENDENCIES.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        buffer.len() as u32,
        Some(&mut bytes_returned),
    );

    if result == 0 {
        // Parse dependency list
    }
}
```

## Resource Types

### Create Resource Type

```rust
unsafe fn create_resource_type(
    cluster: HCLUSTER,
    type_name: &str,
    display_name: &str,
    dll_path: &str,
    looks_alive_poll_interval: u32,
    is_alive_poll_interval: u32,
) -> u32 {
    let type_wide: Vec<u16> = type_name.encode_utf16().chain(Some(0)).collect();
    let display_wide: Vec<u16> = display_name.encode_utf16().chain(Some(0)).collect();
    let dll_wide: Vec<u16> = dll_path.encode_utf16().chain(Some(0)).collect();

    CreateClusterResourceType(
        cluster,
        windows::core::PCWSTR(type_wide.as_ptr()),
        windows::core::PCWSTR(display_wide.as_ptr()),
        windows::core::PCWSTR(dll_wide.as_ptr()),
        looks_alive_poll_interval,
        is_alive_poll_interval,
    )
}
```

### Delete Resource Type

```rust
unsafe fn delete_resource_type(
    cluster: HCLUSTER,
    type_name: &str,
) -> u32 {
    let type_wide: Vec<u16> = type_name.encode_utf16().chain(Some(0)).collect();

    DeleteClusterResourceType(
        cluster,
        windows::core::PCWSTR(type_wide.as_ptr()),
    )
}
```

### Enumerate Resource Types

```rust
unsafe fn enumerate_resource_types(cluster: HCLUSTER) {
    let henum = ClusterOpenEnum(cluster, CLUSTER_ENUM_RESTYPE.0 as u32);

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

        println!("Resource Type: {}",
            String::from_utf16_lossy(&name[..name_len as usize]));

        index += 1;
    }

    ClusterCloseEnum(henum);
}
```

### Resource Type Control

```rust
unsafe fn get_resource_type_properties(
    cluster: HCLUSTER,
    type_name: &str,
) {
    let type_wide: Vec<u16> = type_name.encode_utf16().chain(Some(0)).collect();
    let mut buffer = vec![0u8; 4096];
    let mut bytes_returned = 0u32;

    let result = ClusterResourceTypeControl(
        cluster,
        windows::core::PCWSTR(type_wide.as_ptr()),
        None,
        CLCTL_GET_COMMON_PROPERTIES.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        buffer.len() as u32,
        Some(&mut bytes_returned),
    );

    if result == 0 {
        println!("Resource type properties: {} bytes", bytes_returned);
    }
}
```

## Cluster Shared Volumes

### Add Resource to CSV

```rust
unsafe fn add_to_csv(resource: HRESOURCE) -> u32 {
    AddResourceToClusterSharedVolumes(resource)
}
```

### Remove Resource from CSV

```rust
unsafe fn remove_from_csv(resource: HRESOURCE) -> u32 {
    RemoveResourceFromClusterSharedVolumes(resource)
}
```

## Complete Example: File Server Resources

```rust
use windows::Win32::Networking::Clustering::*;

unsafe fn create_file_server_resources(
    cluster: HCLUSTER,
    group: HGROUP,
) -> windows::core::Result<()> {
    // 1. Create IP Address resource
    let ip_resource = CreateClusterResource(
        group,
        windows::core::w!("FS IP Address"),
        windows::core::w!("IP Address"),
        0,
    );

    if ip_resource.is_invalid() {
        return Err(windows::core::Error::from_win32());
    }

    // Configure IP address properties via ClusterResourceControl...

    // 2. Create Network Name resource
    let name_resource = CreateClusterResource(
        group,
        windows::core::w!("FS Network Name"),
        windows::core::w!("Network Name"),
        0,
    );

    if name_resource.is_invalid() {
        let _ = DeleteClusterResource(ip_resource);
        return Err(windows::core::Error::from_win32());
    }

    // 3. Add dependency: Network Name depends on IP Address
    let dep_result = AddClusterResourceDependency(name_resource, ip_resource);
    if dep_result != 0 {
        println!("Warning: Failed to add dependency");
    }

    // 4. Create Physical Disk resource
    let disk_resource = CreateClusterResource(
        group,
        windows::core::w!("FS Data Disk"),
        windows::core::w!("Physical Disk"),
        0,
    );

    // 5. Create File Share resource
    let share_resource = CreateClusterResource(
        group,
        windows::core::w!("FS Share"),
        windows::core::w!("File Share"),
        0,
    );

    if !share_resource.is_invalid() {
        // File Share depends on Network Name and Physical Disk
        SetClusterResourceDependencyExpression(
            share_resource,
            windows::core::w!("([FS Network Name] and [FS Data Disk])"),
        );
    }

    // Bring resources online
    OnlineClusterResource(ip_resource);
    // Wait for online...
    OnlineClusterResource(name_resource);
    // Wait...
    OnlineClusterResource(disk_resource);
    // Wait...
    OnlineClusterResource(share_resource);

    // Clean up handles (resources stay in cluster)
    let _ = CloseClusterResource(ip_resource);
    let _ = CloseClusterResource(name_resource);
    let _ = CloseClusterResource(disk_resource);
    let _ = CloseClusterResource(share_resource);

    Ok(())
}
```
