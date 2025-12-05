//! Storage attachment tests for windows-hyperv crate.
//!
//! Tests attaching VHDs and ISOs to VMs.
//!
//! Run with: cargo test -p windows-hyperv --features integration -- --test-threads=1

#![cfg(all(windows, feature = "integration"))]

mod common;

use common::{cleanup_test_vm, test_vm_name};
use windows_hyperv::{DiskAttachment, Generation, HyperV, VhdSettings, VmSettings};

#[test]
fn test_attach_vhd_to_vm() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let vm_name = test_vm_name("AttachVHD");
    let vhd_path = std::env::temp_dir().join("hyperv_attach_test.vhdx");
    let vhd_path_str = vhd_path.to_string_lossy().to_string();

    cleanup_test_vm(&hyperv, &vm_name);
    let _ = std::fs::remove_file(&vhd_path);

    // Create VHD
    let vhd_settings = VhdSettings::builder()
        .path(&vhd_path_str)
        .size_gb(1)
        .build()
        .expect("Failed to build VHD settings");
    hyperv
        .vhd()
        .create(&vhd_settings)
        .expect("Failed to create VHD");

    // Create VM
    let vm_settings = VmSettings::builder()
        .name(&vm_name)
        .generation(Generation::Gen2)
        .memory_mb(512)
        .processor_count(1)
        .build()
        .expect("Failed to build VM settings");
    let vm = hyperv.create_vm(&vm_settings).expect("Failed to create VM");

    // Attach VHD
    let attachment = DiskAttachment::new(&vhd_path_str);
    let attach_result = hyperv.attach_vhd(&vm, &attachment);
    assert!(
        attach_result.is_ok(),
        "Failed to attach VHD: {:?}",
        attach_result.err()
    );

    // Cleanup
    cleanup_test_vm(&hyperv, &vm_name);
    let _ = std::fs::remove_file(&vhd_path);
}
