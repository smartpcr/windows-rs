//! Network adapter tests for windows-hyperv crate.
//!
//! Tests adding and configuring network adapters on VMs.
//!
//! Run with: cargo test -p windows-hyperv --features integration -- --test-threads=1

#![cfg(all(windows, feature = "integration"))]

mod common;

use common::{cleanup_test_switch, cleanup_test_vm, test_switch_name, test_vm_name};
use windows_hyperv::{Generation, HyperV, NetworkAdapterSettings, VirtualSwitchSettings, VmSettings};

#[test]
fn test_add_network_adapter() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let vm_name = test_vm_name("Network");
    cleanup_test_vm(&hyperv, &vm_name);

    let settings = VmSettings::builder()
        .name(&vm_name)
        .generation(Generation::Gen2)
        .memory_mb(512)
        .processor_count(1)
        .build()
        .expect("Failed to build VM settings");

    let vm = hyperv.create_vm(&settings).expect("Failed to create VM");

    let adapter_settings = NetworkAdapterSettings::builder()
        .name("TestAdapter")
        .build()
        .expect("Failed to build adapter settings");

    let adapter = hyperv.add_network_adapter(&vm, &adapter_settings);
    assert!(
        adapter.is_ok(),
        "Failed to add network adapter: {:?}",
        adapter.err()
    );

    let adapters = hyperv.list_network_adapters(&vm);
    assert!(adapters.is_ok(), "Failed to list adapters: {:?}", adapters.err());
    assert!(
        !adapters.unwrap().is_empty(),
        "Should have at least one adapter"
    );

    cleanup_test_vm(&hyperv, &vm_name);
}

#[test]
fn test_add_network_adapter_with_switch() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let vm_name = test_vm_name("NetworkWithSwitch");
    let switch_name = test_switch_name("ForNetwork");
    cleanup_test_vm(&hyperv, &vm_name);
    cleanup_test_switch(&hyperv, &switch_name);

    // Create switch
    let switch_settings = VirtualSwitchSettings::builder()
        .name(&switch_name)
        .private()
        .build()
        .expect("Failed to build switch settings");
    let _switch = hyperv
        .create_switch(&switch_settings)
        .expect("Failed to create switch");

    // Create VM
    let vm_settings = VmSettings::builder()
        .name(&vm_name)
        .generation(Generation::Gen2)
        .memory_mb(512)
        .processor_count(1)
        .build()
        .expect("Failed to build VM settings");
    let vm = hyperv.create_vm(&vm_settings).expect("Failed to create VM");

    // Add adapter connected to switch
    let adapter_settings = NetworkAdapterSettings::builder()
        .name("ConnectedAdapter")
        .switch(&switch_name)
        .build()
        .expect("Failed to build adapter settings");

    let adapter = hyperv.add_network_adapter(&vm, &adapter_settings);
    assert!(
        adapter.is_ok(),
        "Failed to add adapter with switch: {:?}",
        adapter.err()
    );

    // Cleanup
    cleanup_test_vm(&hyperv, &vm_name);
    cleanup_test_switch(&hyperv, &switch_name);
}
