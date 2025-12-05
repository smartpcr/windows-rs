//! Full VM setup test for windows-hyperv crate.
//!
//! Comprehensive test that creates a fully configured VM with all features.
//!
//! Run with: cargo test -p windows-hyperv --features integration -- --test-threads=1

#![cfg(all(windows, feature = "integration"))]

mod common;

use common::{cleanup_test_switch, cleanup_test_vm, test_switch_name, test_vm_name};
use windows_hyperv::{
    CheckpointSettings, DiskAttachment, Generation, HyperV, NetworkAdapterSettings,
    VhdSettings, VirtualSwitchSettings, VmSettings, VmState,
};

#[test]
fn test_full_vm_setup() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let vm_name = test_vm_name("FullSetup");
    let switch_name = test_switch_name("FullSetup");
    let vhd_path = std::env::temp_dir().join("hyperv_fullsetup.vhdx");
    let vhd_path_str = vhd_path.to_string_lossy().to_string();

    // Cleanup
    cleanup_test_vm(&hyperv, &vm_name);
    cleanup_test_switch(&hyperv, &switch_name);
    let _ = std::fs::remove_file(&vhd_path);

    // Create switch
    let switch_settings = VirtualSwitchSettings::builder()
        .name(&switch_name)
        .private()
        .build()
        .expect("Failed to build switch settings");
    hyperv
        .create_switch(&switch_settings)
        .expect("Failed to create switch");

    // Create VHD
    let vhd_settings = VhdSettings::builder()
        .path(&vhd_path_str)
        .size_gb(10)
        .build()
        .expect("Failed to build VHD settings");
    hyperv
        .vhd()
        .create(&vhd_settings)
        .expect("Failed to create VHD");

    // Create VM with all features
    let vm_settings = VmSettings::builder()
        .name(&vm_name)
        .generation(Generation::Gen2)
        .memory_mb(2048)
        .processor_count(2)
        .secure_boot(true)
        .tpm_enabled(true)
        .dynamic_memory(true)
        .dynamic_memory_min_mb(1024)
        .dynamic_memory_max_mb(4096)
        .notes("Full integration test VM")
        .build()
        .expect("Failed to build VM settings");
    let vm = hyperv.create_vm(&vm_settings).expect("Failed to create VM");

    // Attach VHD
    let attachment = DiskAttachment::new(&vhd_path_str);
    hyperv
        .attach_vhd(&vm, &attachment)
        .expect("Failed to attach VHD");

    // Add network adapter
    let adapter_settings = NetworkAdapterSettings::builder()
        .name("MainAdapter")
        .switch(&switch_name)
        .build()
        .expect("Failed to build adapter settings");
    hyperv
        .add_network_adapter(&vm, &adapter_settings)
        .expect("Failed to add adapter");

    // Create checkpoint
    let cp_settings = CheckpointSettings::builder()
        .name("InitialState")
        .build()
        .expect("Failed to build checkpoint settings");
    hyperv
        .create_checkpoint(&vm, &cp_settings)
        .expect("Failed to create checkpoint");

    // Verify everything
    let vm = hyperv.get_vm(&vm_name).expect("Failed to get VM");
    assert_eq!(vm.state(), VmState::Off);

    let adapters = hyperv
        .list_network_adapters(&vm)
        .expect("Failed to list adapters");
    assert!(!adapters.is_empty());

    let checkpoints = hyperv
        .list_checkpoints(&vm)
        .expect("Failed to list checkpoints");
    assert!(!checkpoints.is_empty());

    // Cleanup
    cleanup_test_vm(&hyperv, &vm_name);
    cleanup_test_switch(&hyperv, &switch_name);
    let _ = std::fs::remove_file(&vhd_path);
}
