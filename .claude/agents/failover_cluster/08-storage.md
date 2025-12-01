# Storage and Cluster Shared Volumes

## Overview

Failover clusters require shared storage for clustered workloads. This document covers Cluster Shared Volumes (CSV), physical disk management, and storage-related operations.

## Cluster Shared Volumes (CSV)

CSV provides a shared namespace for clustered file servers and Hyper-V VMs, allowing all nodes to access the same volumes simultaneously.

### Check if Path is on CSV

```rust
use windows::Win32::Networking::Clustering::*;

unsafe fn is_on_csv(path: &str) -> bool {
    let path_wide: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();

    ClusterIsPathOnSharedVolume(
        windows::core::PCWSTR(path_wide.as_ptr()),
    ).as_bool()
}
```

### Check if File is on CSV

```rust
unsafe fn is_file_on_csv(path: &str) -> windows::core::Result<bool> {
    let path_wide: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();
    let mut is_on_csv = windows_core::BOOL::default();

    let result = IsFileOnClusterSharedVolume(
        windows::core::PCWSTR(path_wide.as_ptr()),
        &mut is_on_csv,
    );

    if result == 0 {
        Ok(is_on_csv.as_bool())
    } else {
        Err(windows::core::Error::from_win32())
    }
}
```

### Add Resource to CSV

```rust
unsafe fn add_disk_to_csv(disk_resource: HRESOURCE) -> u32 {
    AddResourceToClusterSharedVolumes(disk_resource)
}
```

### Remove Resource from CSV

```rust
unsafe fn remove_disk_from_csv(disk_resource: HRESOURCE) -> u32 {
    RemoveResourceFromClusterSharedVolumes(disk_resource)
}
```

## Volume Path Operations

### Get Volume Path Name

```rust
unsafe fn get_csv_volume_path(file_path: &str) -> windows::core::Result<String> {
    let file_wide: Vec<u16> = file_path.encode_utf16().chain(Some(0)).collect();
    let mut volume_path = [0u16; 256];

    ClusterGetVolumePathName(
        windows::core::PCWSTR(file_wide.as_ptr()),
        windows::core::PWSTR(volume_path.as_mut_ptr()),
        256,
    )?;

    Ok(String::from_utf16_lossy(&volume_path).trim_matches('\0').to_string())
}
```

### Get Volume Name for Mount Point

```rust
unsafe fn get_volume_name_for_mount_point(
    mount_point: &str,
) -> windows::core::Result<String> {
    let mount_wide: Vec<u16> = mount_point.encode_utf16().chain(Some(0)).collect();
    let mut volume_name = [0u16; 256];

    ClusterGetVolumeNameForVolumeMountPoint(
        windows::core::PCWSTR(mount_wide.as_ptr()),
        windows::core::PWSTR(volume_name.as_mut_ptr()),
        256,
    )?;

    Ok(String::from_utf16_lossy(&volume_name).trim_matches('\0').to_string())
}
```

## CSV Backup Operations

### Prepare CSV for Backup

```rust
unsafe fn prepare_csv_for_backup(
    file_path: &str,
) -> windows::core::Result<(String, String)> {
    let file_wide: Vec<u16> = file_path.encode_utf16().chain(Some(0)).collect();
    let mut volume_path = [0u16; 256];
    let mut volume_path_len = 256u32;
    let mut volume_name = [0u16; 256];
    let mut volume_name_len = 256u32;

    let result = ClusterPrepareSharedVolumeForBackup(
        windows::core::PCWSTR(file_wide.as_ptr()),
        windows::core::PWSTR(volume_path.as_mut_ptr()),
        &mut volume_path_len,
        windows::core::PWSTR(volume_name.as_mut_ptr()),
        &mut volume_name_len,
    );

    if result == 0 {
        Ok((
            String::from_utf16_lossy(&volume_path[..volume_path_len as usize]),
            String::from_utf16_lossy(&volume_name[..volume_name_len as usize]),
        ))
    } else {
        Err(windows::core::Error::from_win32())
    }
}
```

### Clear Backup State

```rust
unsafe fn clear_csv_backup_state(volume_path: &str) -> u32 {
    let path_wide: Vec<u16> = volume_path.encode_utf16().chain(Some(0)).collect();

    ClusterClearBackupStateForSharedVolume(
        windows::core::PCWSTR(path_wide.as_ptr()),
    )
}
```

## CSV Snapshot Operations

### Set CSV Snapshot State

```rust
unsafe fn set_csv_snapshot_state(
    snapshot_set_id: windows::core::GUID,
    volume_name: &str,
    state: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE,
) -> u32 {
    let volume_wide: Vec<u16> = volume_name.encode_utf16().chain(Some(0)).collect();

    ClusterSharedVolumeSetSnapshotState(
        snapshot_set_id,
        windows::core::PCWSTR(volume_wide.as_ptr()),
        state,
    )
}

// Snapshot states:
// ClusterSharedVolumeSnapshotStateUnknown = 0
// ClusterSharedVolumePrepareForHWSnapshot = 1
// ClusterSharedVolumeHWSnapshotCompleted = 2
// ClusterSharedVolumePrepareForFreeze = 3
```

## Physical Disk Resources

### Create Physical Disk Resource

```rust
unsafe fn create_disk_resource(
    group: HGROUP,
    name: &str,
) -> windows::core::Result<HRESOURCE> {
    let name_wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();

    let resource = CreateClusterResource(
        group,
        windows::core::PCWSTR(name_wide.as_ptr()),
        windows::core::w!("Physical Disk"),
        0,
    );

    if resource.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(resource)
    }
}
```

### Get Available Disks

```rust
unsafe fn get_available_disks(cluster: HCLUSTER) {
    let mut buffer = vec![0u8; 65536];
    let mut bytes_returned = 0u32;

    let result = ClusterResourceTypeControl(
        cluster,
        windows::core::w!("Physical Disk"),
        None,
        CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX2_INT.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        buffer.len() as u32,
        Some(&mut bytes_returned),
    );

    if result == 0 {
        // Parse disk information from buffer
        println!("Available disks data: {} bytes", bytes_returned);
    }
}
```

### Configure Disk Resource

```rust
unsafe fn configure_disk_by_signature(
    resource: HRESOURCE,
    disk_signature: u32,
) {
    // Build property list with DiskSignature or DiskIdGuid
    let mut prop_list = Vec::new();

    // Property list format:
    // CLUSPROP_LIST header
    // CLUSPROP_SYNTAX_NAME
    // Property name ("DiskSignature")
    // CLUSPROP_SYNTAX_DWORD
    // Signature value

    ClusterResourceControl(
        resource,
        None,
        CLCTL_SET_PRIVATE_PROPERTIES.0 as u32,
        Some(prop_list.as_ptr() as *const _),
        prop_list.len() as u32,
        None,
        0,
        None,
    );
}
```

## Storage Spaces Direct (S2D)

### Check S2D Support

```rust
unsafe fn is_s2d_supported(cluster: HCLUSTER) -> bool {
    let mut buffer = [0u8; 4];
    let mut bytes_returned = 0u32;

    let result = ClusterControl(
        cluster,
        None,
        CLCTL_IS_S2D_FEATURE_SUPPORTED.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        4,
        Some(&mut bytes_returned),
    );

    if result == 0 && bytes_returned >= 4 {
        u32::from_le_bytes(buffer) != 0
    } else {
        false
    }
}
```

### Get Storage Configuration

```rust
unsafe fn get_storage_configuration(cluster: HCLUSTER) {
    let mut buffer = vec![0u8; 8192];
    let mut bytes_returned = 0u32;

    let result = ClusterControl(
        cluster,
        None,
        CLCTL_GET_STORAGE_CONFIGURATION.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        buffer.len() as u32,
        Some(&mut bytes_returned),
    );

    if result == 0 {
        // Parse storage configuration
    }
}
```

## Quorum Configuration

### Get Quorum Resource

```rust
unsafe fn get_quorum_resource(cluster: HCLUSTER) -> (String, String, u32) {
    let mut resource_name = [0u16; 256];
    let mut resource_len = 256u32;
    let mut device_name = [0u16; 256];
    let mut device_len = 256u32;
    let mut max_log_size = 0u32;

    let result = GetClusterQuorumResource(
        cluster,
        windows::core::PWSTR(resource_name.as_mut_ptr()),
        &mut resource_len,
        windows::core::PWSTR(device_name.as_mut_ptr()),
        &mut device_len,
        &mut max_log_size,
    );

    if result == 0 {
        (
            String::from_utf16_lossy(&resource_name[..resource_len as usize]),
            String::from_utf16_lossy(&device_name[..device_len as usize]),
            max_log_size,
        )
    } else {
        (String::new(), String::new(), 0)
    }
}
```

### Set Quorum Resource

```rust
unsafe fn set_quorum_resource(
    resource: HRESOURCE,
    device_name: Option<&str>,
    max_log_size: u32,
) -> u32 {
    let device_ptr = device_name.map(|d| {
        let wide: Vec<u16> = d.encode_utf16().chain(Some(0)).collect();
        windows::core::PCWSTR(wide.as_ptr())
    });

    SetClusterQuorumResource(
        resource,
        device_ptr,
        max_log_size,
    )
}
```

## Disk Partitioning

### Partition Information Structures

```rust
// CLUSPROP_PARTITION_INFO fields
struct PartitionInfo {
    flags: u32,          // CLUSPROP_PIFLAGS
    device_name: String,
    volume_label: String,
    serial_number: u32,
    maximum_component_length: u32,
    file_system_flags: u32,
    file_system_name: String,
}

// CLUSPROP_PIFLAGS values
const CLUSPROP_PIFLAG_STICKY: u32 = 0x00000001;
const CLUSPROP_PIFLAG_REMOVABLE: u32 = 0x00000002;
const CLUSPROP_PIFLAG_USABLE: u32 = 0x00000004;
const CLUSPROP_PIFLAG_DEFAULT_QUORUM: u32 = 0x00000008;
const CLUSPROP_PIFLAG_USABLE_FOR_CSV: u32 = 0x00000010;
const CLUSPROP_PIFLAG_ENCRYPTION_ENABLED: u32 = 0x00000020;
const CLUSPROP_PIFLAG_RAW: u32 = 0x00000040;
const CLUSPROP_PIFLAG_UNKNOWN: u32 = 0x80000000;
```

## Storage Encryption (BitLocker)

### Check BitLocker Status

```rust
// BitLocker status values
const BitLockerEnabled: i32 = 1;
const BitLockerDecrypted: i32 = 4;
const BitlockerEncrypted: i32 = 8;
const BitLockerDecrypting: i32 = 16;
const BitlockerEncrypting: i32 = 32;
const BitLockerPaused: i32 = 64;
const BitLockerStopped: i32 = 128;
```

## Complete Storage Example

```rust
use windows::Win32::Networking::Clustering::*;

unsafe fn manage_cluster_storage(cluster: HCLUSTER) {
    println!("=== Cluster Storage Information ===\n");

    // Get quorum configuration
    let (quorum_res, quorum_device, log_size) = get_quorum_resource(cluster);
    println!("Quorum Resource: {}", quorum_res);
    println!("Quorum Device: {}", quorum_device);
    println!("Max Log Size: {} bytes\n", log_size);

    // Check S2D support
    if is_s2d_supported(cluster) {
        println!("Storage Spaces Direct: Supported\n");
    } else {
        println!("Storage Spaces Direct: Not supported\n");
    }

    // Enumerate disk resources
    println!("Disk Resources:");
    let henum = ClusterOpenEnum(cluster, CLUSTER_ENUM_RESOURCE.0 as u32);

    if !henum.is_invalid() {
        let mut index = 0u32;
        loop {
            let mut obj_type = 0u32;
            let mut name = [0u16; 256];
            let mut name_len = 256u32;

            if ClusterEnum(
                henum,
                index,
                &mut obj_type,
                windows::core::PWSTR(name.as_mut_ptr()),
                &mut name_len,
            ) != 0 {
                break;
            }

            let resource = OpenClusterResource(
                cluster,
                windows::core::PCWSTR(name.as_ptr()),
            );

            if !resource.is_invalid() {
                // Check if it's a disk resource
                let mut type_buffer = [0u8; 256];
                let mut type_len = 0u32;

                if ClusterResourceControl(
                    resource,
                    None,
                    CLCTL_GET_RESOURCE_TYPE.0 as u32,
                    None,
                    0,
                    Some(type_buffer.as_mut_ptr() as *mut _),
                    256,
                    Some(&mut type_len),
                ) == 0 {
                    let type_str = String::from_utf8_lossy(&type_buffer[..type_len as usize]);
                    if type_str.contains("Physical Disk") || type_str.contains("Disk") {
                        let (state, node, group) = get_resource_state(resource);
                        let name_str = String::from_utf16_lossy(&name[..name_len as usize]);
                        println!("  {} - {:?} (on {}, group: {})",
                            name_str, state, node, group);

                        // Check if CSV
                        // ...
                    }
                }

                let _ = CloseClusterResource(resource);
            }

            index += 1;
        }
        ClusterCloseEnum(henum);
    }
}

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
```

## Storage Replication

### Get Eligible Replication Disks

```rust
unsafe fn get_eligible_source_disks(cluster: HCLUSTER) {
    let mut buffer = vec![0u8; 65536];
    let mut bytes_returned = 0u32;

    let result = ClusterResourceTypeControl(
        cluster,
        windows::core::w!("Physical Disk"),
        None,
        CLCTL_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        buffer.len() as u32,
        Some(&mut bytes_returned),
    );

    if result == 0 {
        // Parse eligible disks
    }
}
```

### Get Replication Log Info

```rust
unsafe fn get_replication_log_info(cluster: HCLUSTER) {
    let mut buffer = vec![0u8; 4096];
    let mut bytes_returned = 0u32;

    let result = ClusterResourceTypeControl(
        cluster,
        windows::core::w!("Physical Disk"),
        None,
        CLCTL_REPLICATION_GET_LOG_INFO.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        buffer.len() as u32,
        Some(&mut bytes_returned),
    );

    if result == 0 {
        // Parse replication log information
    }
}
```
