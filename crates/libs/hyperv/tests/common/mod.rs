//! Common test utilities for integration tests.

use std::thread;
use std::time::Duration;
use windows_hyperv::{HyperV, ShutdownType, VmState};

pub const TEST_VM_PREFIX: &str = "HyperV_IntegTest_";
pub const TEST_SWITCH_PREFIX: &str = "HyperV_TestSwitch_";

pub fn test_vm_name(suffix: &str) -> String {
    format!("{}{}", TEST_VM_PREFIX, suffix)
}

pub fn test_switch_name(suffix: &str) -> String {
    format!("{}{}", TEST_SWITCH_PREFIX, suffix)
}

pub fn cleanup_test_vm(hyperv: &HyperV, name: &str) {
    for _ in 0..3 {
        if let Ok(vm) = hyperv.get_vm(name) {
            if vm.state() != VmState::Off {
                let _ = hyperv
                    .get_vm(name)
                    .and_then(|mut v| v.stop(ShutdownType::Force));
                thread::sleep(Duration::from_secs(3));
            }
            if let Ok(vm) = hyperv.get_vm(name) {
                if vm.state() == VmState::Off {
                    let _ = hyperv.delete_vm(&vm);
                    thread::sleep(Duration::from_secs(1));
                }
            }
        } else {
            break;
        }
    }
    thread::sleep(Duration::from_millis(500));
}

pub fn cleanup_test_switch(hyperv: &HyperV, name: &str) {
    if let Ok(switch) = hyperv.get_switch(name) {
        let _ = hyperv.delete_switch(&switch);
        thread::sleep(Duration::from_millis(500));
    }
}
