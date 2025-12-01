# Failover Clustering Overview

## Introduction

Windows Failover Clustering provides high availability and disaster recovery for critical workloads. The Clustering APIs in windows-rs enable programmatic management of clusters, nodes, resources, and groups.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Failover Cluster                             │
│                                                                      │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐              │
│  │   Node 1    │    │   Node 2    │    │   Node 3    │              │
│  │  (Active)   │    │  (Standby)  │    │  (Standby)  │              │
│  │             │    │             │    │             │              │
│  │ ┌─────────┐ │    │ ┌─────────┐ │    │ ┌─────────┐ │              │
│  │ │ Group A │ │    │ │ Group B │ │    │ │         │ │              │
│  │ │┌───────┐│ │    │ │┌───────┐│ │    │ │         │ │              │
│  │ ││Res 1  ││ │    │ ││Res 3  ││ │    │ │         │ │              │
│  │ │├───────┤│ │    │ │├───────┤│ │    │ │         │ │              │
│  │ ││Res 2  ││ │    │ ││Res 4  ││ │    │ │         │ │              │
│  │ │└───────┘│ │    │ │└───────┘│ │    │ │         │ │              │
│  │ └─────────┘ │    │ └─────────┘ │    │ └─────────┘ │              │
│  └─────────────┘    └─────────────┘    └─────────────┘              │
│         │                  │                  │                      │
│         └──────────────────┼──────────────────┘                      │
│                            │                                         │
│               ┌────────────┴────────────┐                            │
│               │    Cluster Networks     │                            │
│               │  (Heartbeat + Client)   │                            │
│               └─────────────────────────┘                            │
│                            │                                         │
│               ┌────────────┴────────────┐                            │
│               │   Cluster Shared        │                            │
│               │   Storage (CSV)         │                            │
│               └─────────────────────────┘                            │
└─────────────────────────────────────────────────────────────────────┘
```

## Core Concepts

### Cluster

A cluster is a group of independent servers (nodes) that work together to provide high availability. Key properties:

- **Cluster Name**: DNS name for client access
- **Quorum**: Voting mechanism for cluster decisions
- **Cluster Database**: Distributed configuration store

### Nodes

Physical or virtual servers that are members of a cluster:

| State | Description |
|-------|-------------|
| `ClusterNodeUp` | Node is online and participating |
| `ClusterNodeDown` | Node is offline |
| `ClusterNodePaused` | Node is paused (no new resources) |
| `ClusterNodeJoining` | Node is joining the cluster |

### Groups (Roles)

A group is a collection of resources that fail over together:

- File Server Role
- SQL Server Role
- Virtual Machine Role
- Generic Application/Service Roles

### Resources

Individual cluster-aware components:

- IP Address
- Network Name
- Physical Disk
- File Share
- Generic Application/Script

### Resource Dependencies

Resources can have dependencies on other resources:

```
Network Name
    └── IP Address
          └── (implicitly depends on network)

File Share
    ├── Network Name
    └── Physical Disk
```

## API Organization

### DLLs

| DLL | Purpose |
|-----|---------|
| `clusapi.dll` | Core clustering APIs |
| `resutils.dll` | Resource utility functions |

### Function Categories

```
Cluster Functions (405+)
├── Cluster Lifecycle
│   ├── CreateCluster, OpenCluster, CloseCluster
│   └── DestroyCluster, BackupClusterDatabase
├── Node Management
│   ├── AddClusterNode, EvictClusterNode
│   └── PauseClusterNode, ResumeClusterNode
├── Group Management
│   ├── CreateClusterGroup, DeleteClusterGroup
│   ├── MoveClusterGroup, OnlineClusterGroup
│   └── OfflineClusterGroup
├── Resource Management
│   ├── CreateClusterResource, DeleteClusterResource
│   ├── OnlineClusterResource, OfflineClusterResource
│   └── AddClusterResourceDependency
├── Network Management
│   ├── GetClusterNetwork, ClusterNetworkControl
│   └── GetClusterNetInterfaceState
├── Notification System
│   ├── CreateClusterNotifyPort
│   └── GetClusterNotify, RegisterClusterNotify
├── Registry Operations
│   ├── ClusterRegOpenKey, ClusterRegCreateKey
│   └── ClusterRegSetValue, ClusterRegQueryValue
└── Utility Functions
    ├── ResUtilGetBinaryValue
    └── ClusWorkerCreate, ClusWorkerTerminate
```

## Handle Hierarchy

```
HCLUSTER ─────────────────────────────────────────┐
    │                                              │
    ├── HNODE ─────────────── Node handles         │
    │                                              │
    ├── HGROUP ───────────── Group handles         │
    │       │                                      │
    │       └── HRESOURCE ── Resource handles      │
    │                                              │
    ├── HNETWORK ─────────── Network handles       │
    │       │                                      │
    │       └── HNETINTERFACE Network interface    │
    │                                              │
    ├── HGROUPSET ────────── Group set handles     │
    │                                              │
    └── HCHANGE ──────────── Notification handles  │
                                                   │
Enumeration Handles ───────────────────────────────┤
    ├── HCLUSENUM, HCLUSENUMEX                     │
    ├── HNODEENUM, HNODEENUMEX                     │
    ├── HGROUPENUM, HGROUPENUMEX                   │
    ├── HRESENUM, HRESENUMEX                       │
    ├── HNETWORKENUM                               │
    ├── HNETINTERFACEENUM                          │
    ├── HGROUPSETENUM                              │
    └── HRESTYPEENUM                               │
                                                   │
Registry Handles ──────────────────────────────────┤
    ├── HREGBATCH, HREGBATCHPORT                   │
    ├── HREGREADBATCH, HREGREADBATCHREPLY          │
    └── HREGBATCHNOTIFICATION                      │
                                                   │
Other Handles ─────────────────────────────────────┘
    └── HCLUSCRYPTPROVIDER
```

## Common Operations

### Connecting to a Cluster

```rust
use windows::Win32::Networking::Clustering::*;

unsafe {
    // Connect by name
    let cluster = OpenCluster(windows::core::w!("MyCluster"));

    // Connect to local cluster (NULL name)
    let local = OpenCluster(windows::core::PCWSTR::null());

    // Always close when done
    CloseCluster(cluster);
}
```

### Error Handling Pattern

```rust
unsafe fn open_cluster(name: &str) -> windows::core::Result<HCLUSTER> {
    let cluster = OpenCluster(windows::core::PCWSTR::from_raw(
        name.encode_utf16().chain(Some(0)).collect::<Vec<_>>().as_ptr()
    ));

    if cluster.is_invalid() {
        return Err(windows::core::Error::from_win32());
    }

    Ok(cluster)
}
```

### Version Information

```rust
unsafe fn get_cluster_version(cluster: HCLUSTER) {
    let mut name = [0u16; 256];
    let mut name_len = 256u32;
    let mut info = CLUSTERVERSIONINFO {
        dwVersionInfoSize: std::mem::size_of::<CLUSTERVERSIONINFO>() as u32,
        ..Default::default()
    };

    let result = GetClusterInformation(
        cluster,
        windows::core::PWSTR(name.as_mut_ptr()),
        &mut name_len,
        Some(&mut info),
    );

    if result == 0 {
        println!(
            "Cluster: {}.{}.{} Build {}",
            info.MajorVersion,
            info.MinorVersion,
            info.CSDVersion,
            info.dwClusterHighestVersion
        );
    }
}
```

## Feature Flags

Required Cargo.toml features:

```toml
[dependencies.windows]
version = "0.59"
features = [
    "Win32_Networking_Clustering",
    "Win32_System_Registry",       # For ClusterReg* functions
    "Win32_Security",              # For security descriptors
    "Win32_Foundation",            # Base types
]
```

## Control Codes Overview

Control codes are used with `*Control` functions for advanced operations:

| Category | Examples |
|----------|----------|
| `CLCTL_*` | Low-level control codes |
| `CLUSCTL_CLUSTER_*` | Cluster-level controls |
| `CLUSCTL_NODE_*` | Node-level controls |
| `CLUSCTL_GROUP_*` | Group-level controls |
| `CLUSCTL_RESOURCE_*` | Resource-level controls |
| `CLUSCTL_RESOURCE_TYPE_*` | Resource type controls |
| `CLUSCTL_NETWORK_*` | Network controls |
| `CLUSCTL_NETINTERFACE_*` | Network interface controls |

## Next Steps

- [Cluster Management](02-cluster-management.md) - Cluster lifecycle
- [Node Management](03-node-management.md) - Working with nodes
- [Group Management](04-group-management.md) - Managing groups
- [Resource Management](05-resource-management.md) - Resource operations
