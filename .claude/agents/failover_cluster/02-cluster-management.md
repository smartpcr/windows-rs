# Cluster Management

## Overview

This document covers cluster lifecycle operations including creation, connection, configuration, and destruction.

## Connecting to a Cluster

### Open Cluster

```rust
use windows::Win32::Networking::Clustering::*;

unsafe fn connect_to_cluster(name: Option<&str>) -> windows::core::Result<HCLUSTER> {
    let cluster = match name {
        Some(n) => {
            let wide: Vec<u16> = n.encode_utf16().chain(Some(0)).collect();
            OpenCluster(windows::core::PCWSTR(wide.as_ptr()))
        }
        None => {
            // Connect to local cluster
            OpenCluster(windows::core::PCWSTR::null())
        }
    };

    if cluster.is_invalid() {
        return Err(windows::core::Error::from_win32());
    }

    Ok(cluster)
}
```

### Open Cluster with Extended Options

```rust
unsafe fn connect_with_flags(name: &str, flags: u32) -> windows::core::Result<HCLUSTER> {
    let wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();

    let cluster = OpenClusterEx(
        windows::core::PCWSTR(wide.as_ptr()),
        flags,
        None,  // Optional: desired access
    );

    if cluster.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(cluster)
    }
}
```

### Close Cluster

```rust
unsafe fn disconnect_cluster(cluster: HCLUSTER) {
    let result = CloseCluster(cluster);
    // Returns BOOL - true on success
}
```

## Creating a Cluster

### Create New Cluster

```rust
use windows::Win32::Networking::Clustering::*;

unsafe fn create_new_cluster(
    cluster_name: &str,
    first_node: &str,
    ip_address: &str,
    subnet_mask: &str,
) -> windows::core::Result<HCLUSTER> {
    let cluster_name_wide: Vec<u16> = cluster_name.encode_utf16().chain(Some(0)).collect();
    let node_name_wide: Vec<u16> = first_node.encode_utf16().chain(Some(0)).collect();

    // Setup IP entry
    let ip_wide: Vec<u16> = ip_address.encode_utf16().chain(Some(0)).collect();
    let subnet_wide: Vec<u16> = subnet_mask.encode_utf16().chain(Some(0)).collect();

    let ip_entry = CLUSTER_IP_ENTRY {
        lpszIpAddress: windows::core::PCWSTR(ip_wide.as_ptr()),
        dwPrefixLength: 24,  // e.g., /24 for 255.255.255.0
    };

    let config = CREATE_CLUSTER_CONFIG {
        dwVersion: CLUSAPI_VERSION as u32,
        lpszClusterName: windows::core::PCWSTR(cluster_name_wide.as_ptr()),
        cNodes: 1,
        ppszNodeNames: &node_name_wide.as_ptr() as *const _ as *const _,
        cIpEntries: 1,
        pIpEntries: &ip_entry,
        fEmptyCluster: windows::core::BOOL(0),
        managementPointType: CLUSTER_MGMT_POINT_TYPE_CNO,
        managementPointResType: CLUSTER_MGMT_POINT_RESTYPE_AUTO,
    };

    let cluster = CreateCluster(&config, None, None);

    if cluster.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(cluster)
    }
}
```

### Create with Progress Callback

```rust
unsafe extern "system" fn progress_callback(
    pvcallbackarg: *const core::ffi::c_void,
    estatetype: CLUSTER_SETUP_PHASE_TYPE,
    ephasetype: CLUSTER_SETUP_PHASE,
    estatephaseseverity: CLUSTER_SETUP_PHASE_SEVERITY,
    dwpercentcomplete: u32,
    lpszobjectname: windows::core::PCWSTR,
    dwstatus: u32,
) -> windows::core::BOOL {
    println!(
        "Progress: {}% - Phase: {:?}",
        dwpercentcomplete, ephasetype
    );

    // Return TRUE to continue, FALSE to abort
    windows::core::BOOL(1)
}

unsafe fn create_cluster_with_progress(
    config: &CREATE_CLUSTER_CONFIG,
) -> windows::core::Result<HCLUSTER> {
    let cluster = CreateCluster(
        config,
        Some(progress_callback),
        None,  // Optional callback context
    );

    if cluster.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(cluster)
    }
}
```

## Getting Cluster Information

### Basic Cluster Information

```rust
unsafe fn get_cluster_info(cluster: HCLUSTER) {
    let mut name_buffer = [0u16; 256];
    let mut name_len = 256u32;
    let mut version_info = CLUSTERVERSIONINFO {
        dwVersionInfoSize: std::mem::size_of::<CLUSTERVERSIONINFO>() as u32,
        ..Default::default()
    };

    let result = GetClusterInformation(
        cluster,
        windows::core::PWSTR(name_buffer.as_mut_ptr()),
        &mut name_len,
        Some(&mut version_info),
    );

    if result == 0 {
        let name = String::from_utf16_lossy(&name_buffer[..name_len as usize]);
        println!("Cluster Name: {}", name);
        println!("Version: {}.{}", version_info.MajorVersion, version_info.MinorVersion);
        println!("Build: {}", version_info.dwClusterHighestVersion);
    }
}
```

### Get Cluster Quorum Configuration

```rust
unsafe fn get_quorum_info(cluster: HCLUSTER) {
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
        println!("Quorum Resource: {:?}",
            String::from_utf16_lossy(&resource_name[..resource_len as usize]));
        println!("Device: {:?}",
            String::from_utf16_lossy(&device_name[..device_len as usize]));
        println!("Max Log Size: {} bytes", max_log_size);
    }
}
```

### Set Cluster Quorum

```rust
unsafe fn set_quorum_resource(
    cluster: HCLUSTER,
    resource: HRESOURCE,
    max_log_size: u32,
) -> windows::core::Result<()> {
    let result = SetClusterQuorumResource(resource, None, max_log_size);

    if result == 0 {
        Ok(())
    } else {
        Err(windows::core::Error::from_win32())
    }
}
```

## Cluster Registry

### Open Cluster Registry Key

```rust
#[cfg(feature = "Win32_System_Registry")]
unsafe fn open_cluster_key(cluster: HCLUSTER) -> windows::core::Result<super::super::System::Registry::HKEY> {
    use windows::Win32::System::Registry::KEY_READ;

    GetClusterKey(cluster, KEY_READ.0)
}
```

### Create Registry Subkey

```rust
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_Security"))]
unsafe fn create_cluster_subkey(
    cluster: HCLUSTER,
    subkey_name: &str,
) -> windows::core::Result<super::super::System::Registry::HKEY> {
    use windows::Win32::System::Registry::*;

    let root_key = GetClusterKey(cluster, KEY_ALL_ACCESS.0)?;
    let subkey_wide: Vec<u16> = subkey_name.encode_utf16().chain(Some(0)).collect();
    let mut new_key = HKEY::default();
    let mut disposition = 0u32;

    let result = ClusterRegCreateKey(
        root_key,
        windows::core::PCWSTR(subkey_wide.as_ptr()),
        0,  // Options
        KEY_ALL_ACCESS.0,
        None,  // Security attributes
        &mut new_key,
        Some(&mut disposition),
    );

    ClusterRegCloseKey(root_key);

    if result == 0 {
        Ok(new_key)
    } else {
        Err(windows::core::Error::from(windows::core::HRESULT(result)))
    }
}
```

### Set Registry Value

```rust
#[cfg(feature = "Win32_System_Registry")]
unsafe fn set_cluster_value(
    key: super::super::System::Registry::HKEY,
    name: &str,
    value: &str,
) -> u32 {
    use windows::Win32::System::Registry::REG_SZ;

    let name_wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
    let value_wide: Vec<u16> = value.encode_utf16().chain(Some(0)).collect();

    ClusterRegSetValue(
        key,
        windows::core::PCWSTR(name_wide.as_ptr()),
        REG_SZ.0,
        value_wide.as_ptr() as *const u8,
        (value_wide.len() * 2) as u32,
    )
}
```

## Cluster Control Operations

### Get Cluster Properties

```rust
unsafe fn get_cluster_properties(cluster: HCLUSTER) {
    let mut buffer = vec![0u8; 4096];
    let mut bytes_returned = 0u32;

    let result = ClusterControl(
        cluster,
        None,  // Optional host node
        CLCTL_GET_COMMON_PROPERTIES.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        buffer.len() as u32,
        Some(&mut bytes_returned),
    );

    if result == 0 {
        // Parse property list from buffer
        println!("Received {} bytes of properties", bytes_returned);
    }
}
```

### Set Cluster Property

```rust
unsafe fn set_cluster_name(
    cluster: HCLUSTER,
    new_name: &str,
) -> u32 {
    // Build property list buffer
    let mut prop_list = Vec::new();

    // ... construct property list format ...

    ClusterControl(
        cluster,
        None,
        CLCTL_SET_COMMON_PROPERTIES.0 as u32,
        Some(prop_list.as_ptr() as *const _),
        prop_list.len() as u32,
        None,
        0,
        None,
    )
}
```

## Cluster Enumeration

### Enumerate All Cluster Objects

```rust
#[repr(u32)]
enum ClusterEnumType {
    Node = 0x00000001,
    ResType = 0x00000002,
    Resource = 0x00000004,
    Group = 0x00000008,
    Network = 0x00000010,
    NetInterface = 0x00000020,
    GroupSet = 0x00000100,
}

unsafe fn enumerate_cluster(cluster: HCLUSTER, enum_type: u32) {
    let henum = ClusterOpenEnum(cluster, enum_type);

    if henum.is_invalid() {
        println!("Failed to open enumeration");
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
            break;  // No more items or error
        }

        let name_str = String::from_utf16_lossy(&name[..name_len as usize]);
        println!("Type {}: {}", obj_type, name_str);

        index += 1;
    }

    ClusterCloseEnum(henum);
}
```

### Extended Enumeration

```rust
unsafe fn enumerate_with_details(cluster: HCLUSTER) {
    let henum = ClusterOpenEnumEx(
        cluster,
        CLUSTER_ENUM_NODE.0 as u32 | CLUSTER_ENUM_RESOURCE.0 as u32,
        None,  // Options
    );

    let mut index = 0u32;
    loop {
        let mut item = CLUSTER_ENUM_ITEM::default();
        let mut item_size = std::mem::size_of::<CLUSTER_ENUM_ITEM>() as u32;

        let result = ClusterEnumEx(henum, index, &mut item, &mut item_size);

        if result != 0 {
            break;
        }

        // item contains:
        // - dwType: Object type
        // - dwVersion: Structure version
        // - cbId: Size of ID
        // - lpszId: Object ID
        // - cbName: Size of name
        // - lpszName: Object name

        index += 1;
    }

    ClusterCloseEnumEx(henum);
}
```

## Backup and Restore

### Backup Cluster Database

```rust
unsafe fn backup_cluster(cluster: HCLUSTER, path: &str) -> u32 {
    let path_wide: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();

    BackupClusterDatabase(
        cluster,
        windows::core::PCWSTR(path_wide.as_ptr()),
    )
}
```

### Restore Cluster Database

```rust
unsafe fn restore_cluster_database(
    backup_path: &str,
    force: bool,
) -> u32 {
    let path_wide: Vec<u16> = backup_path.encode_utf16().chain(Some(0)).collect();

    RestoreClusterDatabase(
        windows::core::PCWSTR(path_wide.as_ptr()),
        force,
        None,  // Optional: new cluster name
    )
}
```

## Destroying a Cluster

### Destroy Cluster

```rust
unsafe fn destroy_cluster(
    cluster: HCLUSTER,
    delete_vcos: bool,
) -> u32 {
    DestroyCluster(
        cluster,
        None,   // Optional progress callback
        None,   // Optional callback context
        delete_vcos,
    )
}
```

## Upgrading Cluster Functional Level

```rust
unsafe fn upgrade_cluster(cluster: HCLUSTER) -> u32 {
    // First check if upgrade is needed
    let result = ClusterUpgradeFunctionalLevel(
        cluster,
        false,  // perform = false to just check
        None,
        None,
    );

    if result == 0 {
        // Upgrade is possible, now perform it
        ClusterUpgradeFunctionalLevel(
            cluster,
            true,  // Actually perform upgrade
            None,  // Optional progress callback
            None,
        )
    } else {
        result
    }
}
```

## Cloud Type Detection

```rust
unsafe fn detect_cloud_type(cluster: HCLUSTER) -> CLUSTER_CLOUD_TYPE {
    let mut cloud_type = CLUSTER_CLOUD_TYPE::default();

    let result = DetermineClusterCloudTypeFromCluster(cluster, &mut cloud_type);

    if result == 0 {
        cloud_type
    } else {
        CLUSTER_CLOUD_TYPE_UNKNOWN
    }
}
```

## Cluster Account Management

### Set Account Access

```rust
unsafe fn set_cluster_account_access(
    cluster: HCLUSTER,
    account_sid: &str,
    access: u32,
) -> u32 {
    let sid_wide: Vec<u16> = account_sid.encode_utf16().chain(Some(0)).collect();

    ClusterSetAccountAccess(
        cluster,
        windows::core::PCWSTR(sid_wide.as_ptr()),
        access,
        0,  // Control type
    )
}
```
