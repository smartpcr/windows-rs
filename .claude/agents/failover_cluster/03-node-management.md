# Node Management

## Overview

Nodes are the physical or virtual servers that make up a failover cluster. This document covers node operations including adding, evicting, pausing, and resuming nodes.

## Node States

```rust
// CLUSTER_NODE_STATE values
const ClusterNodeStateUnknown: i32 = -1;
const ClusterNodeUp: i32 = 0;
const ClusterNodeDown: i32 = 1;
const ClusterNodePaused: i32 = 2;
const ClusterNodeJoining: i32 = 3;
```

## Opening and Closing Nodes

### Open Node by Name

```rust
use windows::Win32::Networking::Clustering::*;

unsafe fn open_node(cluster: HCLUSTER, node_name: &str) -> windows::core::Result<HNODE> {
    let name_wide: Vec<u16> = node_name.encode_utf16().chain(Some(0)).collect();

    let node = OpenClusterNode(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
    );

    if node.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(node)
    }
}
```

### Open Node by ID

```rust
unsafe fn open_node_by_id(cluster: HCLUSTER, node_id: u32) -> windows::core::Result<HNODE> {
    let node = OpenClusterNodeById(cluster, node_id);

    if node.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(node)
    }
}
```

### Close Node Handle

```rust
unsafe fn close_node(node: HNODE) -> windows::core::Result<()> {
    CloseClusterNode(node)
}
```

## Getting Node Information

### Get Node State

```rust
unsafe fn get_node_state(node: HNODE) -> CLUSTER_NODE_STATE {
    GetClusterNodeState(node)
}

unsafe fn print_node_state(node: HNODE) {
    let state = GetClusterNodeState(node);

    let state_str = match state.0 {
        -1 => "Unknown",
        0 => "Up",
        1 => "Down",
        2 => "Paused",
        3 => "Joining",
        _ => "Invalid",
    };

    println!("Node state: {}", state_str);
}
```

### Get Node ID

```rust
unsafe fn get_node_id(node: HNODE) -> String {
    let mut id_buffer = [0u16; 64];
    let mut id_len = 64u32;

    let result = GetClusterNodeId(
        Some(node),
        windows::core::PWSTR(id_buffer.as_mut_ptr()),
        &mut id_len,
    );

    if result == 0 {
        String::from_utf16_lossy(&id_buffer[..id_len as usize])
    } else {
        String::new()
    }
}
```

### Get Local Node ID (No Handle Required)

```rust
unsafe fn get_local_node_id() -> String {
    let mut id_buffer = [0u16; 64];
    let mut id_len = 64u32;

    let result = GetClusterNodeId(
        None,  // NULL = local node
        windows::core::PWSTR(id_buffer.as_mut_ptr()),
        &mut id_len,
    );

    if result == 0 {
        String::from_utf16_lossy(&id_buffer[..id_len as usize])
    } else {
        String::new()
    }
}
```

### Get Node Registry Key

```rust
#[cfg(feature = "Win32_System_Registry")]
unsafe fn get_node_key(node: HNODE) -> windows::core::Result<super::super::System::Registry::HKEY> {
    use windows::Win32::System::Registry::KEY_READ;

    GetClusterNodeKey(node, KEY_READ.0)
}
```

### Get Cluster from Node

```rust
unsafe fn get_cluster_from_node(node: HNODE) -> HCLUSTER {
    GetClusterFromNode(node)
}
```

## Adding Nodes

### Add Node to Cluster

```rust
unsafe fn add_node(
    cluster: HCLUSTER,
    node_name: &str,
) -> windows::core::Result<HNODE> {
    let name_wide: Vec<u16> = node_name.encode_utf16().chain(Some(0)).collect();

    let node = AddClusterNode(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
        None,  // Progress callback
        None,  // Callback context
    );

    if node.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(node)
    }
}
```

### Add Node with Progress Callback

```rust
unsafe extern "system" fn add_node_progress(
    pvcallbackarg: *const core::ffi::c_void,
    estatetype: CLUSTER_SETUP_PHASE_TYPE,
    ephasetype: CLUSTER_SETUP_PHASE,
    estatephaseseverity: CLUSTER_SETUP_PHASE_SEVERITY,
    dwpercentcomplete: u32,
    lpszobjectname: windows::core::PCWSTR,
    dwstatus: u32,
) -> windows::core::BOOL {
    println!("Adding node: {}% complete", dwpercentcomplete);
    windows::core::BOOL(1)  // Continue
}

unsafe fn add_node_with_progress(
    cluster: HCLUSTER,
    node_name: &str,
) -> windows::core::Result<HNODE> {
    let name_wide: Vec<u16> = node_name.encode_utf16().chain(Some(0)).collect();

    let node = AddClusterNode(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
        Some(add_node_progress),
        None,
    );

    if node.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(node)
    }
}
```

### Add Node with Flags

```rust
unsafe fn add_node_ex(
    cluster: HCLUSTER,
    node_name: &str,
    flags: u32,
) -> windows::core::Result<HNODE> {
    let name_wide: Vec<u16> = node_name.encode_utf16().chain(Some(0)).collect();

    let node = AddClusterNodeEx(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
        flags,
        None,
        None,
    );

    if node.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(node)
    }
}
```

### Add Storage Node

```rust
unsafe fn add_storage_node(
    cluster: HCLUSTER,
    node_name: &str,
    description: &str,
    location: &str,
) -> u32 {
    let name_wide: Vec<u16> = node_name.encode_utf16().chain(Some(0)).collect();
    let desc_wide: Vec<u16> = description.encode_utf16().chain(Some(0)).collect();
    let loc_wide: Vec<u16> = location.encode_utf16().chain(Some(0)).collect();

    AddClusterStorageNode(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
        None,  // Progress callback
        None,  // Callback context
        windows::core::PCWSTR(desc_wide.as_ptr()),
        windows::core::PCWSTR(loc_wide.as_ptr()),
    )
}
```

## Evicting Nodes

### Simple Evict

```rust
unsafe fn evict_node(node: HNODE) -> u32 {
    EvictClusterNode(node)
}
```

### Evict with Timeout

```rust
unsafe fn evict_node_with_timeout(
    node: HNODE,
    timeout_ms: u32,
) -> (u32, windows::core::HRESULT) {
    let mut cleanup_status = windows::core::HRESULT(0);

    let result = EvictClusterNodeEx(
        node,
        timeout_ms,
        &mut cleanup_status,
    );

    (result, cleanup_status)
}
```

### Evict with Reason

```rust
unsafe fn evict_node_with_reason(
    node: HNODE,
    timeout_ms: u32,
    reason: &str,
) -> (u32, windows::core::HRESULT) {
    let reason_wide: Vec<u16> = reason.encode_utf16().chain(Some(0)).collect();
    let mut cleanup_status = windows::core::HRESULT(0);

    let result = EvictClusterNodeEx2(
        node,
        timeout_ms,
        &mut cleanup_status,
        windows::core::PCWSTR(reason_wide.as_ptr()),
    );

    (result, cleanup_status)
}
```

## Pausing and Resuming Nodes

### Pause Node

```rust
unsafe fn pause_node(node: HNODE) -> u32 {
    PauseClusterNode(node)
}
```

### Pause Node with Drain

```rust
unsafe fn pause_node_with_drain(
    node: HNODE,
    drain_type: CLUSTER_NODE_RESUME_FAILBACK_TYPE,
    target_node: Option<HNODE>,
) -> u32 {
    PauseClusterNodeEx(
        node,
        true,  // Drain roles
        drain_type,
        target_node,
    )
}

// Drain types:
// DoNotFailbackGroups = 0
// FailbackGroupsImmediately = 1
// FailbackGroupsPerPolicy = 2
```

### Pause with Timeout and Reason

```rust
unsafe fn pause_node_ex2(
    node: HNODE,
    drain: bool,
    drain_type: CLUSTER_NODE_RESUME_FAILBACK_TYPE,
    target_node: Option<HNODE>,
    timeout_secs: u32,
    reason: &str,
) -> u32 {
    let reason_wide: Vec<u16> = reason.encode_utf16().chain(Some(0)).collect();

    PauseClusterNodeEx2(
        node,
        drain,
        drain_type,
        target_node,
        timeout_secs,
        windows::core::PCWSTR(reason_wide.as_ptr()),
    )
}
```

### Resume Node

```rust
unsafe fn resume_node(node: HNODE) -> u32 {
    ResumeClusterNode(node)
}
```

### Resume with Failback Policy

```rust
unsafe fn resume_node_with_failback(
    node: HNODE,
    failback_type: CLUSTER_NODE_RESUME_FAILBACK_TYPE,
) -> u32 {
    ResumeClusterNodeEx(node, failback_type, 0)
}

unsafe fn resume_node_ex2(
    node: HNODE,
    failback_type: CLUSTER_NODE_RESUME_FAILBACK_TYPE,
    reason: &str,
) -> u32 {
    let reason_wide: Vec<u16> = reason.encode_utf16().chain(Some(0)).collect();

    ResumeClusterNodeEx2(
        node,
        failback_type,
        0,  // Reserved flags
        windows::core::PCWSTR(reason_wide.as_ptr()),
    )
}
```

## Node Replacement

```rust
unsafe fn replace_node(
    cluster: HCLUSTER,
    current_node_name: &str,
    new_node_name: &str,
) -> u32 {
    let current_wide: Vec<u16> = current_node_name.encode_utf16().chain(Some(0)).collect();
    let new_wide: Vec<u16> = new_node_name.encode_utf16().chain(Some(0)).collect();

    ClusterNodeReplacement(
        cluster,
        windows::core::PCWSTR(current_wide.as_ptr()),
        windows::core::PCWSTR(new_wide.as_ptr()),
    )
}
```

## Node Enumeration

### Enumerate All Nodes

```rust
unsafe fn enumerate_nodes(cluster: HCLUSTER) {
    let henum = ClusterOpenEnum(cluster, CLUSTER_ENUM_NODE.0 as u32);

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

        let node_name = String::from_utf16_lossy(&name[..name_len as usize]);
        println!("Node: {}", node_name);

        // Get node details
        let node = OpenClusterNode(
            cluster,
            windows::core::PCWSTR(name.as_ptr()),
        );

        if !node.is_invalid() {
            let state = GetClusterNodeState(node);
            println!("  State: {:?}", state);
            let _ = CloseClusterNode(node);
        }

        index += 1;
    }

    ClusterCloseEnum(henum);
}
```

### Enumerate Node Resources

```rust
unsafe fn enumerate_node_groups(node: HNODE) {
    let henum = ClusterNodeOpenEnum(node, CLUSTER_NODE_ENUM_GROUPS.0 as u32);

    if henum.is_invalid() {
        return;
    }

    let mut index = 0u32;
    loop {
        let mut obj_type = 0u32;
        let mut name = [0u16; 256];
        let mut name_len = 256u32;

        let result = ClusterNodeEnum(
            henum,
            index,
            &mut obj_type,
            windows::core::PWSTR(name.as_mut_ptr()),
            &mut name_len,
        );

        if result != 0 {
            break;
        }

        println!("Group on node: {}",
            String::from_utf16_lossy(&name[..name_len as usize]));

        index += 1;
    }

    ClusterNodeCloseEnum(henum);
}
```

## Node Control Operations

### Get Node Properties

```rust
unsafe fn get_node_properties(node: HNODE) {
    let mut buffer = vec![0u8; 4096];
    let mut bytes_returned = 0u32;

    let result = ClusterNodeControl(
        node,
        None,  // Host node
        CLCTL_GET_COMMON_PROPERTIES.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        buffer.len() as u32,
        Some(&mut bytes_returned),
    );

    if result == 0 {
        println!("Node properties: {} bytes", bytes_returned);
    }
}
```

### Get Node Status Flags

```rust
unsafe fn get_node_status(node: HNODE) -> u32 {
    let mut buffer = [0u8; 4];
    let mut bytes_returned = 0u32;

    let result = ClusterNodeControl(
        node,
        None,
        CLCTL_GET_FLAGS.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        4,
        Some(&mut bytes_returned),
    );

    if result == 0 && bytes_returned >= 4 {
        u32::from_le_bytes(buffer)
    } else {
        0
    }
}

// Node status flags (CLUSTER_NODE_STATUS)
const CLUSTER_NODE_STATUS_ISOLATED: u32 = 1;
const CLUSTER_NODE_STATUS_QUARANTINED: u32 = 2;
const CLUSTER_NODE_STATUS_DRAIN_IN_PROGRESS: u32 = 4;
```

### Get Node Drain Status

```rust
unsafe fn get_drain_status(node: HNODE) -> CLUSTER_NODE_DRAIN_STATUS {
    let mut buffer = [0u8; 4];
    let mut bytes_returned = 0u32;

    let result = ClusterNodeControl(
        node,
        None,
        CLCTL_GET_NODE_DRAIN_STATUS.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        4,
        Some(&mut bytes_returned),
    );

    if result == 0 && bytes_returned >= 4 {
        CLUSTER_NODE_DRAIN_STATUS(i32::from_le_bytes(buffer))
    } else {
        CLUSTER_NODE_DRAIN_STATUS::default()
    }
}
```

## Checking Node State

### Check Cluster State on Node

```rust
unsafe fn get_node_cluster_state(node_name: &str) -> u32 {
    let name_wide: Vec<u16> = node_name.encode_utf16().chain(Some(0)).collect();
    let mut state = 0u32;

    let result = GetNodeClusterState(
        windows::core::PCWSTR(name_wide.as_ptr()),
        &mut state,
    );

    if result == 0 {
        state
    } else {
        0
    }
}

// Possible states (NODE_CLUSTER_STATE):
const ClusterStateNotInstalled: u32 = 0;
const ClusterStateNotConfigured: u32 = 1;
const ClusterStateNotRunning: u32 = 3;
const ClusterStateRunning: u32 = 19;
```

### Get Node Cloud Type

```rust
unsafe fn get_node_cloud_type(node_name: &str) -> u32 {
    let name_wide: Vec<u16> = node_name.encode_utf16().chain(Some(0)).collect();
    let mut cloud_type = 0u32;

    GetNodeCloudTypeDW(
        windows::core::PCWSTR(name_wide.as_ptr()),
        &mut cloud_type,
    );

    cloud_type
}
```

## Complete Node Example

```rust
use windows::Win32::Networking::Clustering::*;

unsafe fn manage_nodes(cluster: HCLUSTER) {
    // List all nodes
    println!("=== Cluster Nodes ===");
    enumerate_nodes(cluster);

    // Add a new node
    println!("\n=== Adding Node ===");
    match add_node(cluster, "NewServer") {
        Ok(node) => {
            println!("Node added successfully");

            // Wait for node to come up
            loop {
                let state = GetClusterNodeState(node);
                if state == ClusterNodeUp {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_secs(5));
            }

            // Pause for maintenance
            println!("Pausing node for maintenance...");
            PauseClusterNodeEx(
                node,
                true,  // Drain
                CLUSTER_NODE_RESUME_FAILBACK_TYPE(1),  // Immediate failback
                None,  // No target node
            );

            // Do maintenance...

            // Resume
            println!("Resuming node...");
            ResumeClusterNodeEx(
                node,
                CLUSTER_NODE_RESUME_FAILBACK_TYPE(1),
                0,
            );

            let _ = CloseClusterNode(node);
        }
        Err(e) => {
            println!("Failed to add node: {:?}", e);
        }
    }
}
```
