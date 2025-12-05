//! VHD tests for windows-hyperv crate.
//!
//! Tests VHD creation, info retrieval, and operations.
//!
//! Run with: cargo test -p windows-hyperv --features integration -- --test-threads=1

#![cfg(all(windows, feature = "integration"))]

use windows_hyperv::{HyperV, VhdSettings, VhdType};

#[test]
fn test_vhd_create() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let vhd_path = std::env::temp_dir().join("hyperv_integ_test.vhdx");
    let vhd_path_str = vhd_path.to_string_lossy().to_string();

    // Remove if exists
    let _ = std::fs::remove_file(&vhd_path);

    let settings = VhdSettings::builder()
        .path(&vhd_path_str)
        .size_gb(1)
        .build()
        .expect("Failed to build VHD settings");

    let vhd = hyperv.vhd().create(&settings);
    assert!(vhd.is_ok(), "Failed to create VHD: {:?}", vhd.err());
    assert!(vhd_path.exists(), "VHD file should exist");

    // Cleanup
    let _ = std::fs::remove_file(&vhd_path);
}

#[test]
fn test_vhd_create_fixed() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let vhd_path = std::env::temp_dir().join("hyperv_fixed_test.vhdx");
    let vhd_path_str = vhd_path.to_string_lossy().to_string();

    let _ = std::fs::remove_file(&vhd_path);

    let settings = VhdSettings::builder()
        .path(&vhd_path_str)
        .disk_type(VhdType::Fixed)
        .size_gb(1)
        .build()
        .expect("Failed to build VHD settings");

    let vhd = hyperv.vhd().create(&settings);
    assert!(vhd.is_ok(), "Failed to create fixed VHD: {:?}", vhd.err());
    assert!(vhd_path.exists(), "VHD file should exist");

    // Cleanup
    let _ = std::fs::remove_file(&vhd_path);
}

#[test]
fn test_vhd_get_info() {
    let hyperv = HyperV::connect().expect("Failed to connect");
    let vhd_path = std::env::temp_dir().join("hyperv_info_test.vhdx");
    let vhd_path_str = vhd_path.to_string_lossy().to_string();

    let _ = std::fs::remove_file(&vhd_path);

    let settings = VhdSettings::builder()
        .path(&vhd_path_str)
        .size_gb(1)
        .build()
        .expect("Failed to build VHD settings");

    hyperv
        .vhd()
        .create(&settings)
        .expect("Failed to create VHD");

    let info = hyperv.vhd().get_info(&vhd_path_str);
    assert!(info.is_ok(), "Failed to get VHD info: {:?}", info.err());

    // Cleanup
    let _ = std::fs::remove_file(&vhd_path);
}
