//! Virtual switch tests for windows-hyperv crate.
//!
//! Tests switch creation, deletion, and configuration.
//!
//! Run with: cargo test -p windows-hyperv --features integration -- --test-threads=1

#![cfg(all(windows, feature = "integration"))]

mod common;

use common::{cleanup_test_switch, test_switch_name};
use windows_hyperv::{HyperV, SwitchType, VirtualSwitchSettings};

#[test]
fn test_list_switches() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let result = hyperv.list_switches();
    assert!(result.is_ok(), "Failed to list switches: {:?}", result.err());
}

#[test]
fn test_list_physical_adapters() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let result = hyperv.list_physical_adapters();
    assert!(
        result.is_ok(),
        "Failed to list physical adapters: {:?}",
        result.err()
    );
}

#[test]
fn test_create_and_delete_private_switch() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let switch_name = test_switch_name("Private");
    cleanup_test_switch(&hyperv, &switch_name);

    let settings = VirtualSwitchSettings::builder()
        .name(&switch_name)
        .private()
        .notes("Integration test private switch")
        .build()
        .expect("Failed to build switch settings");

    let switch = hyperv.create_switch(&settings);
    assert!(switch.is_ok(), "Failed to create switch: {:?}", switch.err());
    let switch = switch.unwrap();
    assert_eq!(switch.name(), switch_name);
    assert_eq!(switch.switch_type(), SwitchType::Private);

    // Verify switch exists
    let found = hyperv.get_switch(&switch_name);
    assert!(found.is_ok(), "Failed to find created switch");

    // Delete switch
    let delete_result = hyperv.delete_switch(&switch);
    assert!(
        delete_result.is_ok(),
        "Failed to delete switch: {:?}",
        delete_result.err()
    );

    // Verify deleted
    let not_found = hyperv.get_switch(&switch_name);
    assert!(not_found.is_err(), "Switch should not exist after deletion");
}

#[test]
fn test_create_and_delete_internal_switch() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let switch_name = test_switch_name("Internal");
    cleanup_test_switch(&hyperv, &switch_name);

    let settings = VirtualSwitchSettings::builder()
        .name(&switch_name)
        .internal()
        .build()
        .expect("Failed to build switch settings");

    let switch = hyperv.create_switch(&settings);
    assert!(
        switch.is_ok(),
        "Failed to create internal switch: {:?}",
        switch.err()
    );
    let switch = switch.unwrap();
    assert_eq!(switch.switch_type(), SwitchType::Internal);

    cleanup_test_switch(&hyperv, &switch_name);
}

#[test]
fn test_get_switch_by_id() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let switch_name = test_switch_name("GetById");
    cleanup_test_switch(&hyperv, &switch_name);

    let settings = VirtualSwitchSettings::builder()
        .name(&switch_name)
        .private()
        .build()
        .expect("Failed to build switch settings");

    let switch = hyperv
        .create_switch(&settings)
        .expect("Failed to create switch");
    let switch_id = switch.id().to_string();

    let found = hyperv.get_switch_by_id(&switch_id);
    assert!(
        found.is_ok(),
        "Failed to get switch by ID: {:?}",
        found.err()
    );
    assert_eq!(found.unwrap().name(), switch_name);

    cleanup_test_switch(&hyperv, &switch_name);
}
