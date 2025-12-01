# Windows Failover Clustering API Documentation

Comprehensive documentation for Windows Failover Clustering APIs in windows-rs.

## Quick Start

```rust
use windows::Win32::Networking::Clustering::*;

fn main() -> windows::core::Result<()> {
    unsafe {
        // Open a cluster connection
        let cluster = OpenCluster(windows::core::w!("ClusterName"));
        if cluster.is_invalid() {
            return Err(windows::core::Error::from_win32());
        }

        // Get cluster information
        let mut name = [0u16; 256];
        let mut name_len = 256u32;
        let mut info = CLUSTERVERSIONINFO::default();
        info.dwVersionInfoSize = std::mem::size_of::<CLUSTERVERSIONINFO>() as u32;

        GetClusterInformation(
            cluster,
            windows::core::PWSTR(name.as_mut_ptr()),
            &mut name_len,
            Some(&mut info),
        );

        // Enumerate nodes
        let node_enum = ClusterOpenEnum(cluster, CLUSTER_ENUM_NODE.0 as u32);
        // ... iterate through nodes

        ClusterCloseEnum(node_enum);
        CloseCluster(cluster);
    }
    Ok(())
}
```

## Cargo.toml

```toml
[dependencies]
windows = { version = "0.59", features = [
    "Win32_Networking_Clustering",
    "Win32_System_Registry",      # For registry key access
    "Win32_Security",             # For security operations
]}
```

## Documentation Structure

| Document | Description |
|----------|-------------|
| [01-overview.md](01-overview.md) | Architecture and key concepts |
| [02-cluster-management.md](02-cluster-management.md) | Cluster lifecycle and configuration |
| [03-node-management.md](03-node-management.md) | Node operations and state management |
| [04-group-management.md](04-group-management.md) | Groups, roles, and failover |
| [05-resource-management.md](05-resource-management.md) | Resources, types, and dependencies |
| [06-networking.md](06-networking.md) | Networks and network interfaces |
| [07-notifications.md](07-notifications.md) | Event notifications and monitoring |
| [08-storage.md](08-storage.md) | Cluster Shared Volumes and storage |
| [09-api-reference.md](09-api-reference.md) | Complete API reference |

## Key Concepts

### Handle Types

| Handle | Description |
|--------|-------------|
| `HCLUSTER` | Cluster connection handle |
| `HNODE` | Node handle |
| `HGROUP` | Group (role) handle |
| `HGROUPSET` | Group set handle |
| `HRESOURCE` | Resource handle |
| `HNETWORK` | Network handle |
| `HNETINTERFACE` | Network interface handle |
| `HCHANGE` | Notification port handle |

### Primary DLLs

- **clusapi.dll**: Core cluster management APIs
- **resutils.dll**: Resource utility functions

## Common Patterns

### Error Handling

Most functions return `u32` error codes (Win32 errors):

```rust
let result = AddClusterNode(cluster, node_name, None, None);
if result != 0 {
    // Handle error - use GetLastError() or windows::core::Error
}
```

### Resource Enumeration

```rust
unsafe fn enumerate_resources(cluster: HCLUSTER) {
    let henum = ClusterOpenEnum(cluster, CLUSTER_ENUM_RESOURCE.0 as u32);

    let mut index = 0u32;
    let mut obj_type = 0u32;
    let mut name = [0u16; 256];
    let mut name_len = 256u32;

    while ClusterEnum(
        henum,
        index,
        &mut obj_type,
        windows::core::PWSTR(name.as_mut_ptr()),
        &mut name_len,
    ) == 0 {
        // Process resource
        index += 1;
        name_len = 256;
    }

    ClusterCloseEnum(henum);
}
```

### Control Codes

Use `*Control` functions with control codes for advanced operations:

```rust
// Get resource properties
ClusterResourceControl(
    resource,
    None,
    CLCTL_GET_COMMON_PROPERTIES.0 as u32,
    None,
    0,
    Some(buffer.as_mut_ptr() as *mut _),
    buffer.len() as u32,
    Some(&mut bytes_returned),
);
```
