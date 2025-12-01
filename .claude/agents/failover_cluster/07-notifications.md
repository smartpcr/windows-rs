# Notifications and Events

## Overview

The Failover Clustering API provides a notification system that allows applications to receive events when cluster objects change. This enables reactive programming patterns for cluster management tools.

## Notification Port Concepts

- **Notification Port**: A handle (`HCHANGE`) that receives cluster events
- **Filters**: Specify which events to receive
- **Notify Key**: Application-defined value to identify registrations

## Creating Notification Ports

### Create Basic Notification Port

```rust
use windows::Win32::Networking::Clustering::*;

unsafe fn create_notify_port(
    cluster: HCLUSTER,
    filter: u32,
) -> windows::core::Result<HCHANGE> {
    let hchange = CreateClusterNotifyPort(
        HCHANGE::default(),  // NULL = create new
        cluster,
        filter,
        0,  // Notify key
    );

    if hchange.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(hchange)
    }
}
```

### Common Filter Values (V1)

```rust
// CLUSTER_CHANGE flags
const CLUSTER_CHANGE_NODE_STATE: u32 = 0x00000001;
const CLUSTER_CHANGE_NODE_DELETED: u32 = 0x00000002;
const CLUSTER_CHANGE_NODE_ADDED: u32 = 0x00000004;
const CLUSTER_CHANGE_NODE_PROPERTY: u32 = 0x00000008;
const CLUSTER_CHANGE_REGISTRY_NAME: u32 = 0x00000010;
const CLUSTER_CHANGE_REGISTRY_ATTRIBUTES: u32 = 0x00000020;
const CLUSTER_CHANGE_REGISTRY_VALUE: u32 = 0x00000040;
const CLUSTER_CHANGE_REGISTRY_SUBTREE: u32 = 0x00000080;
const CLUSTER_CHANGE_RESOURCE_STATE: u32 = 0x00000100;
const CLUSTER_CHANGE_RESOURCE_DELETED: u32 = 0x00000200;
const CLUSTER_CHANGE_RESOURCE_ADDED: u32 = 0x00000400;
const CLUSTER_CHANGE_RESOURCE_PROPERTY: u32 = 0x00000800;
const CLUSTER_CHANGE_GROUP_STATE: u32 = 0x00001000;
const CLUSTER_CHANGE_GROUP_DELETED: u32 = 0x00002000;
const CLUSTER_CHANGE_GROUP_ADDED: u32 = 0x00004000;
const CLUSTER_CHANGE_GROUP_PROPERTY: u32 = 0x00008000;
const CLUSTER_CHANGE_RESOURCE_TYPE_DELETED: u32 = 0x00010000;
const CLUSTER_CHANGE_RESOURCE_TYPE_ADDED: u32 = 0x00020000;
const CLUSTER_CHANGE_RESOURCE_TYPE_PROPERTY: u32 = 0x00040000;
const CLUSTER_CHANGE_CLUSTER_RECONNECT: u32 = 0x00080000;
const CLUSTER_CHANGE_NETWORK_STATE: u32 = 0x00100000;
const CLUSTER_CHANGE_NETWORK_DELETED: u32 = 0x00200000;
const CLUSTER_CHANGE_NETWORK_ADDED: u32 = 0x00400000;
const CLUSTER_CHANGE_NETWORK_PROPERTY: u32 = 0x00800000;
const CLUSTER_CHANGE_NETINTERFACE_STATE: u32 = 0x01000000;
const CLUSTER_CHANGE_NETINTERFACE_DELETED: u32 = 0x02000000;
const CLUSTER_CHANGE_NETINTERFACE_ADDED: u32 = 0x04000000;
const CLUSTER_CHANGE_NETINTERFACE_PROPERTY: u32 = 0x08000000;
const CLUSTER_CHANGE_QUORUM_STATE: u32 = 0x10000000;
const CLUSTER_CHANGE_CLUSTER_STATE: u32 = 0x20000000;
const CLUSTER_CHANGE_CLUSTER_PROPERTY: u32 = 0x40000000;
const CLUSTER_CHANGE_HANDLE_CLOSE: u32 = 0x80000000;

const CLUSTER_CHANGE_ALL: u32 = 0xFFFFFFFF;
```

### Create V2 Notification Port

```rust
unsafe fn create_notify_port_v2(
    cluster: HCLUSTER,
) -> windows::core::Result<HCHANGE> {
    // V2 provides more granular filters
    let filters = [
        NOTIFY_FILTER_AND_TYPE {
            dwObjectType: CLUSTER_OBJECT_TYPE_CLUSTER.0 as u32,
            FilterFlags: CLUSTER_CHANGE_CLUSTER_V2_RECONNECT.0 as i64,
        },
        NOTIFY_FILTER_AND_TYPE {
            dwObjectType: CLUSTER_OBJECT_TYPE_NODE.0 as u32,
            FilterFlags: CLUSTER_CHANGE_NODE_V2_STATE.0 as i64,
        },
        NOTIFY_FILTER_AND_TYPE {
            dwObjectType: CLUSTER_OBJECT_TYPE_GROUP.0 as u32,
            FilterFlags: CLUSTER_CHANGE_GROUP_V2_STATE.0 as i64,
        },
        NOTIFY_FILTER_AND_TYPE {
            dwObjectType: CLUSTER_OBJECT_TYPE_RESOURCE.0 as u32,
            FilterFlags: CLUSTER_CHANGE_RESOURCE_V2_STATE.0 as i64,
        },
    ];

    let hchange = CreateClusterNotifyPortV2(
        HCHANGE::default(),
        cluster,
        filters.as_ptr(),
        filters.len() as u32,
        0,
    );

    if hchange.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(hchange)
    }
}
```

## Registering for Notifications

### Register Object for Notification

```rust
unsafe fn register_node_notify(
    hchange: HCHANGE,
    node: HNODE,
    filter: u32,
    notify_key: usize,
) -> u32 {
    RegisterClusterNotify(
        hchange,
        filter,
        node.0 as *mut _,  // HANDLE
        notify_key,
    )
}
```

### Register with V2 API

```rust
unsafe fn register_node_notify_v2(
    hchange: HCHANGE,
    node: HNODE,
    notify_key: usize,
) -> u32 {
    let filter = NOTIFY_FILTER_AND_TYPE {
        dwObjectType: CLUSTER_OBJECT_TYPE_NODE.0 as u32,
        FilterFlags: CLUSTER_CHANGE_NODE_V2_STATE.0 as i64 |
                     CLUSTER_CHANGE_NODE_V2_DELETED.0 as i64,
    };

    RegisterClusterNotifyV2(
        hchange,
        filter,
        node.0 as *mut _,
        notify_key,
    )
}
```

## Getting Notifications

### Wait for and Get Notification (V1)

```rust
unsafe fn get_notification(hchange: HCHANGE, timeout_ms: u32) -> Option<(u32, usize, String)> {
    let mut notify_key = 0usize;
    let mut filter_type = 0u32;
    let mut name = [0u16; 256];
    let mut name_len = 256u32;

    let result = GetClusterNotify(
        hchange,
        &mut notify_key,
        &mut filter_type,
        windows::core::PWSTR(name.as_mut_ptr()),
        &mut name_len,
        timeout_ms,
    );

    if result == 0 {
        Some((
            filter_type,
            notify_key,
            String::from_utf16_lossy(&name[..name_len as usize]),
        ))
    } else {
        None
    }
}
```

### Get Notification (V2)

```rust
unsafe fn get_notification_v2(
    hchange: HCHANGE,
    timeout_ms: u32,
) -> Option<NotificationInfo> {
    let mut notify_key = 0usize;
    let mut filter_and_type = NOTIFY_FILTER_AND_TYPE::default();
    let mut buffer = vec![0u8; 4096];
    let mut buffer_size = 4096u32;
    let mut object_id = [0u16; 256];
    let mut object_id_len = 256u32;
    let mut parent_id = [0u16; 256];
    let mut parent_id_len = 256u32;
    let mut name = [0u16; 256];
    let mut name_len = 256u32;
    let mut type_name = [0u16; 256];
    let mut type_name_len = 256u32;

    let result = GetClusterNotifyV2(
        hchange,
        &mut notify_key,
        Some(&mut filter_and_type),
        Some(buffer.as_mut_ptr()),
        Some(&mut buffer_size),
        Some(windows::core::PWSTR(object_id.as_mut_ptr())),
        Some(&mut object_id_len),
        Some(windows::core::PWSTR(parent_id.as_mut_ptr())),
        Some(&mut parent_id_len),
        Some(windows::core::PWSTR(name.as_mut_ptr())),
        Some(&mut name_len),
        Some(windows::core::PWSTR(type_name.as_mut_ptr())),
        Some(&mut type_name_len),
        Some(timeout_ms),
    );

    if result == 0 {
        Some(NotificationInfo {
            notify_key,
            object_type: filter_and_type.dwObjectType,
            filter_flags: filter_and_type.FilterFlags,
            object_id: String::from_utf16_lossy(&object_id[..object_id_len as usize]),
            parent_id: String::from_utf16_lossy(&parent_id[..parent_id_len as usize]),
            name: String::from_utf16_lossy(&name[..name_len as usize]),
            type_name: String::from_utf16_lossy(&type_name[..type_name_len as usize]),
            buffer: buffer[..buffer_size as usize].to_vec(),
        })
    } else {
        None
    }
}

struct NotificationInfo {
    notify_key: usize,
    object_type: u32,
    filter_flags: i64,
    object_id: String,
    parent_id: String,
    name: String,
    type_name: String,
    buffer: Vec<u8>,
}
```

### Get Event Handle for WaitForSingleObject

```rust
unsafe fn get_notify_event_handle(
    hchange: HCHANGE,
) -> windows::core::Result<super::super::Foundation::HANDLE> {
    let mut event_handle = super::super::Foundation::HANDLE::default();

    let result = GetNotifyEventHandle(hchange, &mut event_handle);

    if result == 0 {
        Ok(event_handle)
    } else {
        Err(windows::core::Error::from_win32())
    }
}
```

## Closing Notification Port

```rust
unsafe fn close_notify_port(hchange: HCHANGE) -> bool {
    CloseClusterNotifyPort(hchange).as_bool()
}
```

## Notification Event Loop Example

### Basic Event Loop

```rust
use windows::Win32::Networking::Clustering::*;

unsafe fn notification_loop(cluster: HCLUSTER) {
    // Create notification port for all changes
    let hchange = CreateClusterNotifyPort(
        HCHANGE::default(),
        cluster,
        CLUSTER_CHANGE_ALL,
        0,
    );

    if hchange.is_invalid() {
        println!("Failed to create notification port");
        return;
    }

    println!("Listening for cluster events (Ctrl+C to stop)...\n");

    loop {
        let mut notify_key = 0usize;
        let mut filter_type = 0u32;
        let mut name = [0u16; 256];
        let mut name_len = 256u32;

        let result = GetClusterNotify(
            hchange,
            &mut notify_key,
            &mut filter_type,
            windows::core::PWSTR(name.as_mut_ptr()),
            &mut name_len,
            5000,  // 5 second timeout
        );

        if result == 0 {
            let event_name = get_event_name(filter_type);
            let object_name = String::from_utf16_lossy(&name[..name_len as usize]);

            println!("[{}] {} - {}",
                chrono::Local::now().format("%H:%M:%S"),
                event_name,
                object_name
            );
        } else if result == 258 {  // WAIT_TIMEOUT
            // No events, continue waiting
            continue;
        } else {
            println!("Error getting notification: {}", result);
            break;
        }
    }

    CloseClusterNotifyPort(hchange);
}

fn get_event_name(filter: u32) -> &'static str {
    match filter {
        0x00000001 => "NODE_STATE",
        0x00000002 => "NODE_DELETED",
        0x00000004 => "NODE_ADDED",
        0x00000008 => "NODE_PROPERTY",
        0x00000100 => "RESOURCE_STATE",
        0x00000200 => "RESOURCE_DELETED",
        0x00000400 => "RESOURCE_ADDED",
        0x00000800 => "RESOURCE_PROPERTY",
        0x00001000 => "GROUP_STATE",
        0x00002000 => "GROUP_DELETED",
        0x00004000 => "GROUP_ADDED",
        0x00008000 => "GROUP_PROPERTY",
        0x00100000 => "NETWORK_STATE",
        0x00200000 => "NETWORK_DELETED",
        0x00400000 => "NETWORK_ADDED",
        0x01000000 => "NETINTERFACE_STATE",
        0x20000000 => "CLUSTER_STATE",
        0x40000000 => "CLUSTER_PROPERTY",
        0x80000000 => "HANDLE_CLOSE",
        _ => "UNKNOWN",
    }
}
```

### V2 Event Loop with Details

```rust
unsafe fn notification_loop_v2(cluster: HCLUSTER) {
    let filters = [
        NOTIFY_FILTER_AND_TYPE {
            dwObjectType: CLUSTER_OBJECT_TYPE_NODE.0 as u32,
            FilterFlags: -1i64,  // All node changes
        },
        NOTIFY_FILTER_AND_TYPE {
            dwObjectType: CLUSTER_OBJECT_TYPE_GROUP.0 as u32,
            FilterFlags: -1i64,  // All group changes
        },
        NOTIFY_FILTER_AND_TYPE {
            dwObjectType: CLUSTER_OBJECT_TYPE_RESOURCE.0 as u32,
            FilterFlags: -1i64,  // All resource changes
        },
    ];

    let hchange = CreateClusterNotifyPortV2(
        HCHANGE::default(),
        cluster,
        filters.as_ptr(),
        filters.len() as u32,
        0,
    );

    if hchange.is_invalid() {
        println!("Failed to create V2 notification port");
        return;
    }

    loop {
        let mut notify_key = 0usize;
        let mut filter = NOTIFY_FILTER_AND_TYPE::default();
        let mut name = [0u16; 256];
        let mut name_len = 256u32;
        let mut object_id = [0u16; 256];
        let mut object_id_len = 256u32;

        let result = GetClusterNotifyV2(
            hchange,
            &mut notify_key,
            Some(&mut filter),
            None,
            None,
            Some(windows::core::PWSTR(object_id.as_mut_ptr())),
            Some(&mut object_id_len),
            None,
            None,
            Some(windows::core::PWSTR(name.as_mut_ptr())),
            Some(&mut name_len),
            None,
            None,
            Some(5000),
        );

        if result == 0 {
            let obj_type = match filter.dwObjectType {
                1 => "Cluster",
                2 => "Group",
                3 => "Resource",
                4 => "ResourceType",
                5 => "NetworkInterface",
                6 => "Network",
                7 => "Node",
                8 => "Registry",
                9 => "Quorum",
                10 => "SharedVolume",
                11 => "GroupSet",
                _ => "Unknown",
            };

            println!(
                "[{}] {}: {} (flags: {:#x})",
                chrono::Local::now().format("%H:%M:%S"),
                obj_type,
                String::from_utf16_lossy(&name[..name_len as usize]),
                filter.FilterFlags
            );
        } else if result != 258 {
            break;
        }
    }

    CloseClusterNotifyPort(hchange);
}
```

### Async Event Processing

```rust
use windows::Win32::System::Threading::*;
use std::thread;

unsafe fn async_notification_handler(cluster: HCLUSTER) {
    let hchange = CreateClusterNotifyPort(
        HCHANGE::default(),
        cluster,
        CLUSTER_CHANGE_GROUP_STATE |
        CLUSTER_CHANGE_RESOURCE_STATE |
        CLUSTER_CHANGE_NODE_STATE,
        0,
    );

    if hchange.is_invalid() {
        return;
    }

    // Get event handle for async waiting
    let mut event_handle = super::super::Foundation::HANDLE::default();
    if GetNotifyEventHandle(hchange, &mut event_handle) != 0 {
        CloseClusterNotifyPort(hchange);
        return;
    }

    // Spawn handler thread
    thread::spawn(move || {
        loop {
            // Wait for event
            let wait_result = WaitForSingleObject(event_handle, 1000);

            if wait_result.0 == 0 {  // WAIT_OBJECT_0
                // Process all pending notifications
                loop {
                    let mut notify_key = 0usize;
                    let mut filter_type = 0u32;
                    let mut name = [0u16; 256];
                    let mut name_len = 256u32;

                    let result = GetClusterNotify(
                        hchange,
                        &mut notify_key,
                        &mut filter_type,
                        windows::core::PWSTR(name.as_mut_ptr()),
                        &mut name_len,
                        0,  // Non-blocking
                    );

                    if result != 0 {
                        break;
                    }

                    // Handle notification
                    process_notification(filter_type, &name[..name_len as usize]);
                }
            } else if wait_result.0 == 258 {  // WAIT_TIMEOUT
                continue;
            } else {
                break;
            }
        }
    });
}

fn process_notification(filter: u32, name: &[u16]) {
    let name_str = String::from_utf16_lossy(name);

    match filter {
        0x00000001 => println!("Node state changed: {}", name_str),
        0x00000100 => println!("Resource state changed: {}", name_str),
        0x00001000 => println!("Group state changed: {}", name_str),
        _ => println!("Event {:#x}: {}", filter, name_str),
    }
}
```

## Health Fault Monitoring

### Get Cluster Health Faults

```rust
unsafe fn get_health_faults(cluster: HCLUSTER) -> Vec<CLUSTER_HEALTH_FAULT> {
    let mut fault_array = CLUSTER_HEALTH_FAULT_ARRAY::default();

    let result = ClusGetClusterHealthFaults(
        cluster,
        &mut fault_array,
        0,  // Flags
    );

    let mut faults = Vec::new();

    if result == 0 {
        for i in 0..fault_array.numFaults {
            // Access faults from fault_array.faults
        }
    }

    faults
}
```

### Add Health Fault

```rust
unsafe fn add_health_fault(
    cluster: HCLUSTER,
    id: &str,
    error_type: u32,
    error_code: u32,
    description: &str,
) -> u32 {
    let id_wide: Vec<u16> = id.encode_utf16().chain(Some(0)).collect();
    let desc_wide: Vec<u16> = description.encode_utf16().chain(Some(0)).collect();

    let mut fault = CLUSTER_HEALTH_FAULT {
        Id: windows::core::PWSTR(id_wide.as_ptr() as *mut _),
        ErrorType: error_type,
        ErrorCode: error_code,
        Description: windows::core::PWSTR(desc_wide.as_ptr() as *mut _),
        Provider: windows::core::PWSTR::null(),
        Flags: 0,
        Reserved: 0,
    };

    ClusAddClusterHealthFault(cluster, &fault, 0)
}
```

### Remove Health Fault

```rust
unsafe fn remove_health_fault(cluster: HCLUSTER, id: &str) -> u32 {
    let id_wide: Vec<u16> = id.encode_utf16().chain(Some(0)).collect();

    ClusRemoveClusterHealthFault(
        cluster,
        windows::core::PCWSTR(id_wide.as_ptr()),
        0,
    )
}
```

## Registry Change Notifications

### Create Registry Batch Notify Port

```rust
#[cfg(feature = "Win32_System_Registry")]
unsafe fn create_registry_notify_port(
    key: super::super::System::Registry::HKEY,
) -> windows::core::Result<HREGBATCHPORT> {
    let mut port = HREGBATCHPORT::default();

    let result = ClusterRegCreateBatchNotifyPort(key, &mut port);

    if result == 0 {
        Ok(port)
    } else {
        Err(windows::core::Error::from(windows::core::HRESULT(result)))
    }
}
```

### Get Registry Batch Notification

```rust
unsafe fn get_registry_notification(
    port: HREGBATCHPORT,
) -> Option<HREGBATCHNOTIFICATION> {
    let mut notification = HREGBATCHNOTIFICATION::default();

    let result = ClusterRegGetBatchNotification(port, &mut notification);

    if result == 0 {
        Some(notification)
    } else {
        None
    }
}
```

### Read Registry Batch Command

```rust
unsafe fn read_registry_batch_command(
    notification: HREGBATCHNOTIFICATION,
) -> Option<CLUSTER_BATCH_COMMAND> {
    let mut command = CLUSTER_BATCH_COMMAND::default();

    let result = ClusterRegBatchReadCommand(notification, &mut command);

    if result == 0 {
        Some(command)
    } else {
        None
    }
}
```
