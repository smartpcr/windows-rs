//! Connection tests for windows-hyperv crate.
//!
//! Run with: cargo test -p windows-hyperv --features integration -- --test-threads=1

#![cfg(all(windows, feature = "integration"))]

use windows_hyperv::HyperV;

#[test]
fn test_connect() {
    let result = HyperV::connect();
    assert!(
        result.is_ok(),
        "Failed to connect to Hyper-V: {:?}",
        result.err()
    );
}

#[test]
fn test_connect_multiple() {
    let conn1 = HyperV::connect();
    let conn2 = HyperV::connect();
    assert!(conn1.is_ok());
    assert!(conn2.is_ok());
}
