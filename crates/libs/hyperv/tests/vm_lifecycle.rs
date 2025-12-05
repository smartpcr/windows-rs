//! VM lifecycle tests for windows-hyperv crate.
//!
//! Tests VM creation, deletion, and configuration.
//!
//! Run with: cargo test -p windows-hyperv --features integration -- --test-threads=1

#![cfg(all(windows, feature = "integration"))]

mod common;

use common::{cleanup_test_vm, test_vm_name};
use windows_hyperv::{Generation, HyperV, VmSettings, VmState};

#[test]
fn test_list_vms() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let result = hyperv.list_vms();
    assert!(result.is_ok(), "Failed to list VMs: {:?}", result.err());
}

#[test]
fn test_get_vm_not_found() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let result = hyperv.get_vm("NonExistent_VM_That_Should_Not_Exist_12345");
    assert!(result.is_err(), "Should fail for non-existent VM");
}

#[test]
fn test_create_and_delete_vm_gen1() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let vm_name = test_vm_name("CreateDeleteGen1");
    cleanup_test_vm(&hyperv, &vm_name);

    let settings = VmSettings::builder()
        .name(&vm_name)
        .generation(Generation::Gen1)
        .memory_mb(512)
        .processor_count(1)
        .build()
        .expect("Failed to build VM settings");

    let vm = hyperv.create_vm(&settings);
    assert!(vm.is_ok(), "Failed to create VM: {:?}", vm.err());
    let vm = vm.unwrap();
    assert_eq!(vm.name(), vm_name);
    assert_eq!(vm.state(), VmState::Off);

    let found = hyperv.get_vm(&vm_name);
    assert!(found.is_ok(), "Failed to find created VM");

    let delete_result = hyperv.delete_vm(&vm);
    assert!(
        delete_result.is_ok(),
        "Failed to delete VM: {:?}",
        delete_result.err()
    );

    let not_found = hyperv.get_vm(&vm_name);
    assert!(not_found.is_err(), "VM should not exist after deletion");
}

#[test]
fn test_create_and_delete_vm_gen2() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let vm_name = test_vm_name("CreateDeleteGen2");
    cleanup_test_vm(&hyperv, &vm_name);

    let settings = VmSettings::builder()
        .name(&vm_name)
        .generation(Generation::Gen2)
        .memory_mb(512)
        .processor_count(1)
        .secure_boot(true)
        .build()
        .expect("Failed to build VM settings");

    let vm = hyperv.create_vm(&settings);
    assert!(vm.is_ok(), "Failed to create Gen2 VM: {:?}", vm.err());
    let vm = vm.unwrap();

    let delete_result = hyperv.delete_vm(&vm);
    assert!(delete_result.is_ok(), "Failed to delete VM");
}

#[test]
fn test_create_vm_with_dynamic_memory() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let vm_name = test_vm_name("DynamicMemory");
    cleanup_test_vm(&hyperv, &vm_name);

    let settings = VmSettings::builder()
        .name(&vm_name)
        .generation(Generation::Gen2)
        .memory_mb(1024)
        .processor_count(2)
        .dynamic_memory(true)
        .dynamic_memory_min_mb(512)
        .dynamic_memory_max_mb(4096)
        .memory_buffer_percentage(20)
        .build()
        .expect("Failed to build VM settings");

    let vm = hyperv.create_vm(&settings);
    assert!(
        vm.is_ok(),
        "Failed to create VM with dynamic memory: {:?}",
        vm.err()
    );

    cleanup_test_vm(&hyperv, &vm_name);
}

#[test]
fn test_vm_get_by_id() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let vm_name = test_vm_name("GetById");
    cleanup_test_vm(&hyperv, &vm_name);

    let settings = VmSettings::builder()
        .name(&vm_name)
        .generation(Generation::Gen1)
        .memory_mb(512)
        .processor_count(1)
        .build()
        .expect("Failed to build VM settings");

    let vm = hyperv.create_vm(&settings).expect("Failed to create VM");
    let vm_id = vm.id().to_string();

    let found = hyperv.get_vm_by_id(&vm_id);
    assert!(found.is_ok(), "Failed to get VM by ID: {:?}", found.err());
    assert_eq!(found.unwrap().name(), vm_name);

    cleanup_test_vm(&hyperv, &vm_name);
}
