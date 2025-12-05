//! VM power state tests for windows-hyperv crate.
//!
//! Tests VM start, stop, pause, resume operations.
//!
//! Run with: cargo test -p windows-hyperv --features integration -- --test-threads=1

#![cfg(all(windows, feature = "integration"))]

mod common;

use common::{cleanup_test_vm, test_vm_name};
use std::thread;
use std::time::Duration;
use windows_hyperv::{Generation, HyperV, ShutdownType, VmSettings, VmState};

#[test]
fn test_vm_power_cycle() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let vm_name = test_vm_name("PowerCycle");
    cleanup_test_vm(&hyperv, &vm_name);

    let settings = VmSettings::builder()
        .name(&vm_name)
        .generation(Generation::Gen2)
        .memory_mb(512)
        .processor_count(1)
        .build()
        .expect("Failed to build VM settings");

    let _vm = hyperv.create_vm(&settings).expect("Failed to create VM");

    // Start VM
    let mut vm = hyperv.get_vm(&vm_name).expect("Failed to get VM");
    let start_result = vm.start();
    assert!(
        start_result.is_ok(),
        "Failed to start VM: {:?}",
        start_result.err()
    );

    thread::sleep(Duration::from_secs(3));

    vm.refresh().expect("Failed to refresh VM state");
    assert_eq!(vm.state(), VmState::Running, "VM should be running");

    // Force stop VM
    let stop_result = vm.stop(ShutdownType::Force);
    assert!(
        stop_result.is_ok(),
        "Failed to stop VM: {:?}",
        stop_result.err()
    );

    thread::sleep(Duration::from_secs(2));

    vm.refresh().expect("Failed to refresh VM state");
    assert_eq!(vm.state(), VmState::Off, "VM should be off");

    cleanup_test_vm(&hyperv, &vm_name);
}

#[test]
fn test_vm_pause_resume() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let vm_name = test_vm_name("PauseResume");
    cleanup_test_vm(&hyperv, &vm_name);

    let settings = VmSettings::builder()
        .name(&vm_name)
        .generation(Generation::Gen2)
        .memory_mb(512)
        .processor_count(1)
        .build()
        .expect("Failed to build VM settings");

    let _vm = hyperv.create_vm(&settings).expect("Failed to create VM");
    let mut vm = hyperv.get_vm(&vm_name).expect("Failed to get VM");

    // Start
    vm.start().expect("Failed to start VM");
    thread::sleep(Duration::from_secs(3));
    vm.refresh().expect("Failed to refresh");
    assert_eq!(vm.state(), VmState::Running);

    // Pause
    let pause_result = vm.pause();
    assert!(
        pause_result.is_ok(),
        "Failed to pause VM: {:?}",
        pause_result.err()
    );
    thread::sleep(Duration::from_secs(1));
    vm.refresh().expect("Failed to refresh");
    assert_eq!(vm.state(), VmState::Paused, "VM should be paused");

    // Resume
    let resume_result = vm.resume();
    assert!(
        resume_result.is_ok(),
        "Failed to resume VM: {:?}",
        resume_result.err()
    );
    thread::sleep(Duration::from_secs(1));
    vm.refresh().expect("Failed to refresh");
    assert_eq!(vm.state(), VmState::Running, "VM should be running again");

    // Cleanup
    vm.stop(ShutdownType::Force).ok();
    thread::sleep(Duration::from_secs(2));
    cleanup_test_vm(&hyperv, &vm_name);
}
