# Networking

## Overview

Failover clusters use multiple networks for different purposes: client access, cluster communication (heartbeat), and storage connectivity. This document covers network and network interface management.

## Network States

```rust
// CLUSTER_NETWORK_STATE values
const ClusterNetworkStateUnknown: i32 = -1;
const ClusterNetworkUnavailable: i32 = 0;
const ClusterNetworkDown: i32 = 1;
const ClusterNetworkPartitioned: i32 = 2;
const ClusterNetworkUp: i32 = 3;
```

## Network Interface States

```rust
// CLUSTER_NETINTERFACE_STATE values
const ClusterNetInterfaceStateUnknown: i32 = -1;
const ClusterNetInterfaceUnavailable: i32 = 0;
const ClusterNetInterfaceFailed: i32 = 1;
const ClusterNetInterfaceUnreachable: i32 = 2;
const ClusterNetInterfaceUp: i32 = 3;
```

## Network Roles

```rust
// CLUSTER_NETWORK_ROLE values
const ClusterNetworkRoleNone: i32 = 0;           // Not used by cluster
const ClusterNetworkRoleInternalUse: i32 = 1;    // Cluster communication only
const ClusterNetworkRoleClientAccess: i32 = 2;   // Client access only
const ClusterNetworkRoleInternalAndClient: i32 = 3; // Both
```

## Opening and Closing Networks

### Open Network by Name

```rust
use windows::Win32::Networking::Clustering::*;

unsafe fn open_network(cluster: HCLUSTER, name: &str) -> windows::core::Result<HNETWORK> {
    let name_wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();

    let network = OpenClusterNetwork(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
    );

    if network.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(network)
    }
}
```

### Open Network with Extended Options

```rust
unsafe fn open_network_ex(
    cluster: HCLUSTER,
    name: &str,
    desired_access: u32,
) -> windows::core::Result<HNETWORK> {
    let name_wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();

    let network = OpenClusterNetworkEx(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
        desired_access,
        None,
    );

    if network.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(network)
    }
}
```

### Close Network Handle

```rust
unsafe fn close_network(network: HNETWORK) -> windows::core::Result<()> {
    CloseClusterNetwork(network)
}
```

## Getting Network Information

### Get Network State

```rust
unsafe fn get_network_state(network: HNETWORK) -> CLUSTER_NETWORK_STATE {
    GetClusterNetworkState(network)
}

unsafe fn print_network_state(network: HNETWORK) {
    let state = GetClusterNetworkState(network);

    let state_str = match state.0 {
        -1 => "Unknown",
        0 => "Unavailable",
        1 => "Down",
        2 => "Partitioned",
        3 => "Up",
        _ => "Invalid",
    };

    println!("Network state: {}", state_str);
}
```

### Get Network ID

```rust
unsafe fn get_network_id(network: HNETWORK) -> String {
    let mut id_buffer = [0u16; 64];
    let mut id_len = 64u32;

    let result = GetClusterNetworkId(
        network,
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

### Get Network Registry Key

```rust
#[cfg(feature = "Win32_System_Registry")]
unsafe fn get_network_key(network: HNETWORK) -> windows::core::Result<super::super::System::Registry::HKEY> {
    use windows::Win32::System::Registry::KEY_READ;

    GetClusterNetworkKey(network, KEY_READ.0)
}
```

### Get Cluster from Network

```rust
unsafe fn get_cluster_from_network(network: HNETWORK) -> HCLUSTER {
    GetClusterFromNetwork(network)
}
```

## Network Enumeration

### Enumerate All Networks

```rust
unsafe fn enumerate_networks(cluster: HCLUSTER) {
    let henum = ClusterOpenEnum(cluster, CLUSTER_ENUM_NETWORK.0 as u32);

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

        let network_name = String::from_utf16_lossy(&name[..name_len as usize]);
        println!("Network: {}", network_name);

        // Get more details
        let network = OpenClusterNetwork(
            cluster,
            windows::core::PCWSTR(name.as_ptr()),
        );

        if !network.is_invalid() {
            let state = GetClusterNetworkState(network);
            println!("  State: {:?}", state);
            let _ = CloseClusterNetwork(network);
        }

        index += 1;
    }

    ClusterCloseEnum(henum);
}
```

### Enumerate Network Interfaces in Network

```rust
unsafe fn enumerate_network_interfaces(network: HNETWORK) {
    let henum = ClusterNetworkOpenEnum(
        network,
        CLUSTER_NETWORK_ENUM_NETINTERFACES.0 as u32,
    );

    if henum.is_invalid() {
        return;
    }

    let mut index = 0u32;
    loop {
        let mut obj_type = 0u32;
        let mut name = [0u16; 256];
        let mut name_len = 256u32;

        let result = ClusterNetworkEnum(
            henum,
            index,
            &mut obj_type,
            windows::core::PWSTR(name.as_mut_ptr()),
            &mut name_len,
        );

        if result != 0 {
            break;
        }

        println!("  Interface: {}",
            String::from_utf16_lossy(&name[..name_len as usize]));

        index += 1;
    }

    ClusterNetworkCloseEnum(henum);
}
```

## Network Control Operations

### Get Network Properties

```rust
unsafe fn get_network_properties(network: HNETWORK) {
    let mut buffer = vec![0u8; 4096];
    let mut bytes_returned = 0u32;

    let result = ClusterNetworkControl(
        network,
        None,
        CLCTL_GET_COMMON_PROPERTIES.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        buffer.len() as u32,
        Some(&mut bytes_returned),
    );

    if result == 0 {
        println!("Network properties: {} bytes", bytes_returned);
    }
}
```

### Set Network Name

```rust
unsafe fn set_network_name(network: HNETWORK, new_name: &str) {
    // Build property list with new name
    // Use ClusterNetworkControl with CLCTL_SET_COMMON_PROPERTIES
}
```

### Get Network Role

```rust
unsafe fn get_network_role(network: HNETWORK) -> i32 {
    let mut buffer = [0u8; 4];
    let mut bytes_returned = 0u32;

    let result = ClusterNetworkControl(
        network,
        None,
        CLCTL_GET_COMMON_PROPERTIES.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        4,
        Some(&mut bytes_returned),
    );

    if result == 0 {
        // Parse role from property list
        0  // Placeholder
    } else {
        -1
    }
}
```

## Network Interfaces

### Open Network Interface

```rust
unsafe fn open_net_interface(
    cluster: HCLUSTER,
    name: &str,
) -> windows::core::Result<HNETINTERFACE> {
    let name_wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();

    let netif = OpenClusterNetInterface(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
    );

    if netif.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(netif)
    }
}
```

### Open with Extended Options

```rust
unsafe fn open_net_interface_ex(
    cluster: HCLUSTER,
    name: &str,
    desired_access: u32,
) -> windows::core::Result<HNETINTERFACE> {
    let name_wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();

    let netif = OpenClusterNetInterfaceEx(
        cluster,
        windows::core::PCWSTR(name_wide.as_ptr()),
        desired_access,
        None,
    );

    if netif.is_invalid() {
        Err(windows::core::Error::from_win32())
    } else {
        Ok(netif)
    }
}
```

### Close Network Interface

```rust
unsafe fn close_net_interface(netif: HNETINTERFACE) -> windows::core::Result<()> {
    CloseClusterNetInterface(netif)
}
```

### Get Network Interface State

```rust
unsafe fn get_net_interface_state(netif: HNETINTERFACE) -> CLUSTER_NETINTERFACE_STATE {
    GetClusterNetInterfaceState(netif)
}

unsafe fn print_net_interface_state(netif: HNETINTERFACE) {
    let state = GetClusterNetInterfaceState(netif);

    let state_str = match state.0 {
        -1 => "Unknown",
        0 => "Unavailable",
        1 => "Failed",
        2 => "Unreachable",
        3 => "Up",
        _ => "Invalid",
    };

    println!("Network Interface state: {}", state_str);
}
```

### Get Network Interface by Node and Network

```rust
unsafe fn get_net_interface_name(
    cluster: HCLUSTER,
    node_name: &str,
    network_name: &str,
) -> String {
    let node_wide: Vec<u16> = node_name.encode_utf16().chain(Some(0)).collect();
    let network_wide: Vec<u16> = network_name.encode_utf16().chain(Some(0)).collect();
    let mut interface_name = [0u16; 256];
    let mut name_len = 256u32;

    let result = GetClusterNetInterface(
        cluster,
        windows::core::PCWSTR(node_wide.as_ptr()),
        windows::core::PCWSTR(network_wide.as_ptr()),
        windows::core::PWSTR(interface_name.as_mut_ptr()),
        &mut name_len,
    );

    if result == 0 {
        String::from_utf16_lossy(&interface_name[..name_len as usize])
    } else {
        String::new()
    }
}
```

### Get Network Interface Registry Key

```rust
#[cfg(feature = "Win32_System_Registry")]
unsafe fn get_net_interface_key(
    netif: HNETINTERFACE,
) -> windows::core::Result<super::super::System::Registry::HKEY> {
    use windows::Win32::System::Registry::KEY_READ;

    GetClusterNetInterfaceKey(netif, KEY_READ.0)
}
```

### Get Cluster from Network Interface

```rust
unsafe fn get_cluster_from_net_interface(netif: HNETINTERFACE) -> HCLUSTER {
    GetClusterFromNetInterface(netif)
}
```

## Network Interface Enumeration

### Enumerate All Network Interfaces

```rust
unsafe fn enumerate_all_net_interfaces(cluster: HCLUSTER) {
    let henum = ClusterOpenEnum(cluster, CLUSTER_ENUM_NETINTERFACE.0 as u32);

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

        println!("Network Interface: {}",
            String::from_utf16_lossy(&name[..name_len as usize]));

        index += 1;
    }

    ClusterCloseEnum(henum);
}
```

### Enumerate by Node and Network

```rust
unsafe fn enumerate_net_interfaces_for_node(
    cluster: HCLUSTER,
    node_name: &str,
    network_name: Option<&str>,
) {
    let node_wide: Vec<u16> = node_name.encode_utf16().chain(Some(0)).collect();

    let network_ptr = match network_name {
        Some(n) => {
            let wide: Vec<u16> = n.encode_utf16().chain(Some(0)).collect();
            windows::core::PCWSTR(wide.as_ptr())
        }
        None => windows::core::PCWSTR::null(),
    };

    let henum = ClusterNetInterfaceOpenEnum(
        cluster,
        windows::core::PCWSTR(node_wide.as_ptr()),
        network_ptr,
    );

    if henum.is_invalid() {
        return;
    }

    let mut index = 0u32;
    loop {
        let mut name = [0u16; 256];
        let mut name_len = 256u32;

        let result = ClusterNetInterfaceEnum(
            henum,
            index,
            windows::core::PWSTR(name.as_mut_ptr()),
            &mut name_len,
        );

        if result != 0 {
            break;
        }

        println!("  Interface: {}",
            String::from_utf16_lossy(&name[..name_len as usize]));

        index += 1;
    }

    ClusterNetInterfaceCloseEnum(henum);
}
```

## Network Interface Control Operations

### Get Network Interface Properties

```rust
unsafe fn get_net_interface_properties(netif: HNETINTERFACE) {
    let mut buffer = vec![0u8; 4096];
    let mut bytes_returned = 0u32;

    let result = ClusterNetInterfaceControl(
        netif,
        None,
        CLCTL_GET_COMMON_PROPERTIES.0 as u32,
        None,
        0,
        Some(buffer.as_mut_ptr() as *mut _),
        buffer.len() as u32,
        Some(&mut bytes_returned),
    );

    if result == 0 {
        println!("Network Interface properties: {} bytes", bytes_returned);
    }
}
```

### Get Network Interface Characteristics

```rust
unsafe fn get_net_interface_characteristics(netif: HNETINTERFACE) -> u32 {
    let mut buffer = [0u8; 4];
    let mut bytes_returned = 0u32;

    let result = ClusterNetInterfaceControl(
        netif,
        None,
        CLCTL_GET_CHARACTERISTICS.0 as u32,
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
```

## Complete Network Example

```rust
use windows::Win32::Networking::Clustering::*;

unsafe fn display_cluster_networks(cluster: HCLUSTER) {
    println!("=== Cluster Networks ===\n");

    // Enumerate networks
    let henum = ClusterOpenEnum(cluster, CLUSTER_ENUM_NETWORK.0 as u32);
    if henum.is_invalid() {
        println!("Failed to enumerate networks");
        return;
    }

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

        let network_name = String::from_utf16_lossy(&name[..name_len as usize]);
        println!("Network: {}", network_name);

        // Open network for details
        let network = OpenClusterNetwork(
            cluster,
            windows::core::PCWSTR(name.as_ptr()),
        );

        if !network.is_invalid() {
            // State
            let state = GetClusterNetworkState(network);
            let state_str = match state.0 {
                0 => "Unavailable",
                1 => "Down",
                2 => "Partitioned",
                3 => "Up",
                _ => "Unknown",
            };
            println!("  State: {}", state_str);

            // Network ID
            let mut id = [0u16; 64];
            let mut id_len = 64u32;
            if GetClusterNetworkId(
                network,
                windows::core::PWSTR(id.as_mut_ptr()),
                &mut id_len,
            ) == 0 {
                println!("  ID: {}", String::from_utf16_lossy(&id[..id_len as usize]));
            }

            // Enumerate interfaces on this network
            println!("  Interfaces:");
            let if_enum = ClusterNetworkOpenEnum(
                network,
                CLUSTER_NETWORK_ENUM_NETINTERFACES.0 as u32,
            );

            if !if_enum.is_invalid() {
                let mut if_index = 0u32;
                loop {
                    let mut if_type = 0u32;
                    let mut if_name = [0u16; 256];
                    let mut if_name_len = 256u32;

                    if ClusterNetworkEnum(
                        if_enum,
                        if_index,
                        &mut if_type,
                        windows::core::PWSTR(if_name.as_mut_ptr()),
                        &mut if_name_len,
                    ) != 0 {
                        break;
                    }

                    let interface_name = String::from_utf16_lossy(&if_name[..if_name_len as usize]);

                    // Get interface state
                    let netif = OpenClusterNetInterface(
                        cluster,
                        windows::core::PCWSTR(if_name.as_ptr()),
                    );

                    if !netif.is_invalid() {
                        let if_state = GetClusterNetInterfaceState(netif);
                        let if_state_str = match if_state.0 {
                            0 => "Unavailable",
                            1 => "Failed",
                            2 => "Unreachable",
                            3 => "Up",
                            _ => "Unknown",
                        };
                        println!("    {} ({})", interface_name, if_state_str);
                        let _ = CloseClusterNetInterface(netif);
                    } else {
                        println!("    {}", interface_name);
                    }

                    if_index += 1;
                }
                ClusterNetworkCloseEnum(if_enum);
            }

            let _ = CloseClusterNetwork(network);
        }

        println!();
        index += 1;
    }

    ClusterCloseEnum(henum);
}
```
