//! Checkpoint tests for windows-hyperv crate.
//!
//! Tests checkpoint creation, listing, and deletion.
//!
//! Run with: cargo test -p windows-hyperv --features integration -- --test-threads=1

#![cfg(all(windows, feature = "integration"))]

mod common;

use common::{cleanup_test_vm, test_vm_name};
use windows_hyperv::{CheckpointSettings, Generation, HyperV, VmSettings};

#[test]
fn test_create_checkpoint() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let vm_name = test_vm_name("Checkpoint");
    cleanup_test_vm(&hyperv, &vm_name);

    let settings = VmSettings::builder()
        .name(&vm_name)
        .generation(Generation::Gen2)
        .memory_mb(512)
        .processor_count(1)
        .build()
        .expect("Failed to build VM settings");

    let _vm = hyperv.create_vm(&settings).expect("Failed to create VM");
    let vm = hyperv.get_vm(&vm_name).expect("Failed to get VM");

    let cp_settings = CheckpointSettings::builder()
        .name("TestCheckpoint")
        .notes("Integration test checkpoint")
        .build()
        .expect("Failed to build checkpoint settings");

    let checkpoint = hyperv.create_checkpoint(&vm, &cp_settings);
    assert!(
        checkpoint.is_ok(),
        "Failed to create checkpoint: {:?}",
        checkpoint.err()
    );

    let checkpoints = hyperv.list_checkpoints(&vm);
    assert!(
        checkpoints.is_ok(),
        "Failed to list checkpoints: {:?}",
        checkpoints.err()
    );
    assert!(
        !checkpoints.unwrap().is_empty(),
        "Should have at least one checkpoint"
    );

    if let Ok(cp) = checkpoint {
        let delete_result = hyperv.delete_checkpoint(&cp);
        assert!(
            delete_result.is_ok(),
            "Failed to delete checkpoint: {:?}",
            delete_result.err()
        );
    }

    cleanup_test_vm(&hyperv, &vm_name);
}

#[test]
fn test_list_checkpoints_empty() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let vm_name = test_vm_name("NoCheckpoints");
    cleanup_test_vm(&hyperv, &vm_name);

    let settings = VmSettings::builder()
        .name(&vm_name)
        .generation(Generation::Gen2)
        .memory_mb(512)
        .processor_count(1)
        .build()
        .expect("Failed to build VM settings");

    let vm = hyperv.create_vm(&settings).expect("Failed to create VM");

    let checkpoints = hyperv.list_checkpoints(&vm);
    assert!(checkpoints.is_ok(), "Failed to list checkpoints");
    assert!(
        checkpoints.unwrap().is_empty(),
        "New VM should have no checkpoints"
    );

    cleanup_test_vm(&hyperv, &vm_name);
}
