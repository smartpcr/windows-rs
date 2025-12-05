//! Strong types for VM configuration values.
//!
//! These types provide compile-time validation and clear semantics for
//! VM configuration parameters, ensuring values are within Hyper-V limits.

use core::fmt;

/// Memory size in megabytes.
///
/// Validates that memory is within Hyper-V limits:
/// - Minimum: 32 MB
/// - Maximum: 12 TB (12,582,912 MB)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryMB(u64);

impl MemoryMB {
    /// Minimum memory in MB (32 MB).
    pub const MIN: u64 = 32;
    /// Maximum memory in MB (12 TB).
    pub const MAX: u64 = 12 * 1024 * 1024; // 12 TB in MB

    /// Create from megabytes.
    ///
    /// Returns `None` if outside valid range (32 MB - 12 TB).
    pub fn new(mb: u64) -> Option<Self> {
        if mb >= Self::MIN && mb <= Self::MAX {
            Some(Self(mb))
        } else {
            None
        }
    }

    /// Create from gigabytes.
    pub fn from_gb(gb: u64) -> Option<Self> {
        Self::new(gb.saturating_mul(1024))
    }

    /// Get value in megabytes.
    pub fn as_mb(&self) -> u64 {
        self.0
    }

    /// Get value in gigabytes (rounded down).
    pub fn as_gb(&self) -> u64 {
        self.0 / 1024
    }

    /// Get value in bytes.
    pub fn as_bytes(&self) -> u64 {
        self.0.saturating_mul(1024 * 1024)
    }

    /// Common presets
    pub const fn mb_512() -> Self {
        Self(512)
    }
    pub const fn gb_1() -> Self {
        Self(1024)
    }
    pub const fn gb_2() -> Self {
        Self(2048)
    }
    pub const fn gb_4() -> Self {
        Self(4096)
    }
    pub const fn gb_8() -> Self {
        Self(8192)
    }
    pub const fn gb_16() -> Self {
        Self(16384)
    }
    pub const fn gb_32() -> Self {
        Self(32768)
    }
}

impl Default for MemoryMB {
    fn default() -> Self {
        Self::gb_1()
    }
}

impl fmt::Display for MemoryMB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 >= 1024 && self.0 % 1024 == 0 {
            write!(f, "{} GB", self.0 / 1024)
        } else {
            write!(f, "{} MB", self.0)
        }
    }
}

/// Virtual processor count.
///
/// Validates that processor count is within Hyper-V limits:
/// - Minimum: 1
/// - Maximum: 240
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessorCount(u32);

impl ProcessorCount {
    /// Minimum processor count.
    pub const MIN: u32 = 1;
    /// Maximum processor count.
    pub const MAX: u32 = 240;

    /// Create a new processor count.
    ///
    /// Returns `None` if outside valid range (1-240).
    pub fn new(count: u32) -> Option<Self> {
        if count >= Self::MIN && count <= Self::MAX {
            Some(Self(count))
        } else {
            None
        }
    }

    /// Get the processor count.
    pub fn get(&self) -> u32 {
        self.0
    }

    /// Common presets
    pub const fn one() -> Self {
        Self(1)
    }
    pub const fn two() -> Self {
        Self(2)
    }
    pub const fn four() -> Self {
        Self(4)
    }
    pub const fn eight() -> Self {
        Self(8)
    }
}

impl Default for ProcessorCount {
    fn default() -> Self {
        Self::one()
    }
}

impl fmt::Display for ProcessorCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} vCPU{}", self.0, if self.0 == 1 { "" } else { "s" })
    }
}

/// Memory buffer percentage for dynamic memory (0-100).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MemoryBufferPercent(u32);

impl MemoryBufferPercent {
    /// Create a new memory buffer percentage.
    ///
    /// Returns `None` if percentage > 100.
    pub fn new(percent: u32) -> Option<Self> {
        if percent <= 100 {
            Some(Self(percent))
        } else {
            None
        }
    }

    /// Get the percentage value.
    pub fn get(&self) -> u32 {
        self.0
    }

    /// Default 20% buffer.
    pub const fn default_20() -> Self {
        Self(20)
    }
}

impl fmt::Display for MemoryBufferPercent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

/// Memory weight for resource allocation priority (0-10000).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryWeight(u32);

impl MemoryWeight {
    /// Minimum weight.
    pub const MIN: u32 = 0;
    /// Maximum weight.
    pub const MAX: u32 = 10000;
    /// Default weight.
    pub const DEFAULT: u32 = 5000;

    /// Create a new memory weight.
    pub fn new(weight: u32) -> Option<Self> {
        if weight <= Self::MAX {
            Some(Self(weight))
        } else {
            None
        }
    }

    /// Get the weight value.
    pub fn get(&self) -> u32 {
        self.0
    }
}

impl Default for MemoryWeight {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

/// Processor weight for resource allocation priority (0-10000).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessorWeight(u32);

impl ProcessorWeight {
    /// Minimum weight.
    pub const MIN: u32 = 0;
    /// Maximum weight.
    pub const MAX: u32 = 10000;
    /// Default weight.
    pub const DEFAULT: u32 = 100;

    /// Create a new processor weight.
    pub fn new(weight: u32) -> Option<Self> {
        if weight <= Self::MAX {
            Some(Self(weight))
        } else {
            None
        }
    }

    /// Get the weight value.
    pub fn get(&self) -> u32 {
        self.0
    }
}

impl Default for ProcessorWeight {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

/// Processor reservation/limit percentage (0-100000, representing 0-100% with 3 decimal precision).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessorPercent(u64);

impl ProcessorPercent {
    /// Maximum value (100000 = 100%).
    pub const MAX: u64 = 100000;

    /// Create from raw value (0-100000).
    pub fn new(value: u64) -> Option<Self> {
        if value <= Self::MAX {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Create from percentage (0-100).
    pub fn from_percent(percent: u32) -> Option<Self> {
        if percent <= 100 {
            Some(Self(percent as u64 * 1000))
        } else {
            None
        }
    }

    /// Get raw value.
    pub fn get(&self) -> u64 {
        self.0
    }

    /// Get as percentage (0-100).
    pub fn as_percent(&self) -> f64 {
        self.0 as f64 / 1000.0
    }
}

impl Default for ProcessorPercent {
    fn default() -> Self {
        Self(Self::MAX) // 100%
    }
}

/// SCSI controller location (0-63) or IDE location (0-1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiskLocation(u32);

impl DiskLocation {
    /// Maximum SCSI location.
    pub const MAX_SCSI: u32 = 63;
    /// Maximum IDE location.
    pub const MAX_IDE: u32 = 1;

    /// Create a SCSI disk location (0-63).
    pub fn scsi(location: u32) -> Option<Self> {
        if location <= Self::MAX_SCSI {
            Some(Self(location))
        } else {
            None
        }
    }

    /// Create an IDE disk location (0-1).
    pub fn ide(location: u32) -> Option<Self> {
        if location <= Self::MAX_IDE {
            Some(Self(location))
        } else {
            None
        }
    }

    /// Get the location value.
    pub fn get(&self) -> u32 {
        self.0
    }
}

impl Default for DiskLocation {
    fn default() -> Self {
        Self(0)
    }
}

impl fmt::Display for DiskLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Location {}", self.0)
    }
}

/// Sector size for VHD/VHDX (512 or 4096 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectorSize {
    /// 512 bytes (legacy).
    Bytes512,
    /// 4096 bytes (4K native).
    Bytes4K,
}

impl SectorSize {
    /// Get the size in bytes.
    pub fn as_bytes(&self) -> u32 {
        match self {
            SectorSize::Bytes512 => 512,
            SectorSize::Bytes4K => 4096,
        }
    }

    /// Parse from bytes value.
    pub fn from_bytes(bytes: u32) -> Option<Self> {
        match bytes {
            512 => Some(SectorSize::Bytes512),
            4096 => Some(SectorSize::Bytes4K),
            _ => None,
        }
    }
}

impl Default for SectorSize {
    fn default() -> Self {
        SectorSize::Bytes512
    }
}

impl fmt::Display for SectorSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SectorSize::Bytes512 => write!(f, "512 bytes"),
            SectorSize::Bytes4K => write!(f, "4K"),
        }
    }
}

/// VHD block size.
///
/// Valid values: 512 KB, 1 MB, 2 MB, 16 MB, 32 MB, 64 MB, 128 MB, 256 MB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockSize(u32);

impl BlockSize {
    /// 512 KB block size.
    pub const KB_512: Self = Self(512 * 1024);
    /// 1 MB block size.
    pub const MB_1: Self = Self(1024 * 1024);
    /// 2 MB block size (VHDX default for data disks).
    pub const MB_2: Self = Self(2 * 1024 * 1024);
    /// 16 MB block size.
    pub const MB_16: Self = Self(16 * 1024 * 1024);
    /// 32 MB block size (VHDX default).
    pub const MB_32: Self = Self(32 * 1024 * 1024);
    /// 64 MB block size.
    pub const MB_64: Self = Self(64 * 1024 * 1024);
    /// 128 MB block size.
    pub const MB_128: Self = Self(128 * 1024 * 1024);
    /// 256 MB block size.
    pub const MB_256: Self = Self(256 * 1024 * 1024);

    /// Create from bytes.
    pub fn from_bytes(bytes: u32) -> Option<Self> {
        match bytes {
            b if b == Self::KB_512.0 => Some(Self::KB_512),
            b if b == Self::MB_1.0 => Some(Self::MB_1),
            b if b == Self::MB_2.0 => Some(Self::MB_2),
            b if b == Self::MB_16.0 => Some(Self::MB_16),
            b if b == Self::MB_32.0 => Some(Self::MB_32),
            b if b == Self::MB_64.0 => Some(Self::MB_64),
            b if b == Self::MB_128.0 => Some(Self::MB_128),
            b if b == Self::MB_256.0 => Some(Self::MB_256),
            _ => None,
        }
    }

    /// Get the size in bytes.
    pub fn as_bytes(&self) -> u32 {
        self.0
    }
}

impl Default for BlockSize {
    fn default() -> Self {
        Self::MB_32
    }
}

impl fmt::Display for BlockSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 >= 1024 * 1024 {
            write!(f, "{} MB", self.0 / (1024 * 1024))
        } else {
            write!(f, "{} KB", self.0 / 1024)
        }
    }
}

/// VHD/VHDX disk size.
///
/// Validates size limits:
/// - VHD: max 2 TB
/// - VHDX: max 64 TB
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiskSize(u64);

impl DiskSize {
    /// Maximum VHD size (2 TB).
    pub const MAX_VHD: u64 = 2 * 1024 * 1024 * 1024 * 1024;
    /// Maximum VHDX size (64 TB).
    pub const MAX_VHDX: u64 = 64 * 1024 * 1024 * 1024 * 1024;

    /// Create from bytes.
    pub fn from_bytes(bytes: u64) -> Self {
        Self(bytes)
    }

    /// Create from gigabytes.
    pub fn from_gb(gb: u64) -> Self {
        Self(gb.saturating_mul(1024 * 1024 * 1024))
    }

    /// Create from terabytes.
    pub fn from_tb(tb: u64) -> Self {
        Self(tb.saturating_mul(1024 * 1024 * 1024 * 1024))
    }

    /// Get size in bytes.
    pub fn as_bytes(&self) -> u64 {
        self.0
    }

    /// Get size in gigabytes (rounded down).
    pub fn as_gb(&self) -> u64 {
        self.0 / (1024 * 1024 * 1024)
    }

    /// Check if valid for VHD format.
    pub fn is_valid_vhd(&self) -> bool {
        self.0 > 0 && self.0 <= Self::MAX_VHD
    }

    /// Check if valid for VHDX format.
    pub fn is_valid_vhdx(&self) -> bool {
        self.0 > 0 && self.0 <= Self::MAX_VHDX
    }
}

impl fmt::Display for DiskSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tb = 1024 * 1024 * 1024 * 1024u64;
        let gb = 1024 * 1024 * 1024u64;
        if self.0 >= tb && self.0 % tb == 0 {
            write!(f, "{} TB", self.0 / tb)
        } else if self.0 >= gb && self.0 % gb == 0 {
            write!(f, "{} GB", self.0 / gb)
        } else {
            write!(f, "{} bytes", self.0)
        }
    }
}

/// VLAN ID (0-4094).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VlanId(u16);

impl VlanId {
    /// Maximum VLAN ID.
    pub const MAX: u16 = 4094;

    /// Create a new VLAN ID.
    pub fn new(id: u16) -> Option<Self> {
        if id <= Self::MAX {
            Some(Self(id))
        } else {
            None
        }
    }

    /// Get the VLAN ID.
    pub fn get(&self) -> u16 {
        self.0
    }
}

impl fmt::Display for VlanId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VLAN {}", self.0)
    }
}

/// MAC address with validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacAddress([u8; 6]);

impl MacAddress {
    /// Create from 6 bytes.
    pub fn new(bytes: [u8; 6]) -> Self {
        Self(bytes)
    }

    /// Parse from string (XX:XX:XX:XX:XX:XX or XX-XX-XX-XX-XX-XX).
    pub fn parse(s: &str) -> Option<Self> {
        let clean: String = s.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        if clean.len() != 12 {
            return None;
        }

        let mut bytes = [0u8; 6];
        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&clean[i * 2..i * 2 + 2], 16).ok()?;
        }
        Some(Self(bytes))
    }

    /// Get the raw bytes.
    pub fn as_bytes(&self) -> &[u8; 6] {
        &self.0
    }

    /// Format as colon-separated string.
    pub fn to_colon_string(&self) -> String {
        format!(
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }

    /// Format as Hyper-V style (no separators).
    pub fn to_hyperv_string(&self) -> String {
        format!(
            "{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_colon_string())
    }
}

/// Bandwidth weight for network adapters (1-100).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BandwidthWeight(u32);

impl BandwidthWeight {
    /// Minimum weight.
    pub const MIN: u32 = 1;
    /// Maximum weight.
    pub const MAX: u32 = 100;

    /// Create a new bandwidth weight.
    pub fn new(weight: u32) -> Option<Self> {
        if weight >= Self::MIN && weight <= Self::MAX {
            Some(Self(weight))
        } else {
            None
        }
    }

    /// Get the weight value.
    pub fn get(&self) -> u32 {
        self.0
    }
}

impl Default for BandwidthWeight {
    fn default() -> Self {
        Self(50)
    }
}

impl fmt::Display for BandwidthWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== MemoryMB Tests ==========

    #[test]
    fn test_memory_mb_valid_range() {
        assert!(MemoryMB::new(32).is_some());
        assert!(MemoryMB::new(1024).is_some());
        assert!(MemoryMB::new(12 * 1024 * 1024).is_some());
        assert!(MemoryMB::new(31).is_none());
        assert!(MemoryMB::new(12 * 1024 * 1024 + 1).is_none());
    }

    #[test]
    fn test_memory_mb_from_gb() {
        assert_eq!(MemoryMB::from_gb(1).unwrap().as_mb(), 1024);
        assert_eq!(MemoryMB::from_gb(4).unwrap().as_mb(), 4096);
    }

    #[test]
    fn test_memory_mb_as_bytes() {
        let mem = MemoryMB::new(1024).unwrap();
        assert_eq!(mem.as_bytes(), 1024 * 1024 * 1024);
    }

    #[test]
    fn test_memory_mb_as_gb() {
        let mem = MemoryMB::new(2048).unwrap();
        assert_eq!(mem.as_gb(), 2);
    }

    #[test]
    fn test_memory_mb_presets() {
        assert_eq!(MemoryMB::mb_512().as_mb(), 512);
        assert_eq!(MemoryMB::gb_1().as_mb(), 1024);
        assert_eq!(MemoryMB::gb_2().as_mb(), 2048);
        assert_eq!(MemoryMB::gb_4().as_mb(), 4096);
        assert_eq!(MemoryMB::gb_8().as_mb(), 8192);
        assert_eq!(MemoryMB::gb_16().as_mb(), 16384);
        assert_eq!(MemoryMB::gb_32().as_mb(), 32768);
    }

    #[test]
    fn test_memory_mb_default() {
        assert_eq!(MemoryMB::default().as_mb(), 1024);
    }

    #[test]
    fn test_memory_mb_display() {
        assert_eq!(format!("{}", MemoryMB::new(512).unwrap()), "512 MB");
        assert_eq!(format!("{}", MemoryMB::new(1024).unwrap()), "1 GB");
        assert_eq!(format!("{}", MemoryMB::new(2048).unwrap()), "2 GB");
        assert_eq!(format!("{}", MemoryMB::new(1536).unwrap()), "1536 MB"); // Not exact GB
    }

    #[test]
    fn test_memory_mb_constants() {
        assert_eq!(MemoryMB::MIN, 32);
        assert_eq!(MemoryMB::MAX, 12 * 1024 * 1024);
    }

    // ========== ProcessorCount Tests ==========

    #[test]
    fn test_processor_count_valid_range() {
        assert!(ProcessorCount::new(1).is_some());
        assert!(ProcessorCount::new(240).is_some());
        assert!(ProcessorCount::new(0).is_none());
        assert!(ProcessorCount::new(241).is_none());
    }

    #[test]
    fn test_processor_count_get() {
        let pc = ProcessorCount::new(4).unwrap();
        assert_eq!(pc.get(), 4);
    }

    #[test]
    fn test_processor_count_presets() {
        assert_eq!(ProcessorCount::one().get(), 1);
        assert_eq!(ProcessorCount::two().get(), 2);
        assert_eq!(ProcessorCount::four().get(), 4);
        assert_eq!(ProcessorCount::eight().get(), 8);
    }

    #[test]
    fn test_processor_count_default() {
        assert_eq!(ProcessorCount::default().get(), 1);
    }

    #[test]
    fn test_processor_count_display() {
        assert_eq!(format!("{}", ProcessorCount::one()), "1 vCPU");
        assert_eq!(format!("{}", ProcessorCount::two()), "2 vCPUs");
        assert_eq!(format!("{}", ProcessorCount::four()), "4 vCPUs");
    }

    #[test]
    fn test_processor_count_constants() {
        assert_eq!(ProcessorCount::MIN, 1);
        assert_eq!(ProcessorCount::MAX, 240);
    }

    // ========== MemoryBufferPercent Tests ==========

    #[test]
    fn test_memory_buffer_percent_valid() {
        assert!(MemoryBufferPercent::new(0).is_some());
        assert!(MemoryBufferPercent::new(50).is_some());
        assert!(MemoryBufferPercent::new(100).is_some());
        assert!(MemoryBufferPercent::new(101).is_none());
    }

    #[test]
    fn test_memory_buffer_percent_get() {
        let mbp = MemoryBufferPercent::new(25).unwrap();
        assert_eq!(mbp.get(), 25);
    }

    #[test]
    fn test_memory_buffer_percent_default() {
        assert_eq!(MemoryBufferPercent::default().get(), 0);
    }

    #[test]
    fn test_memory_buffer_percent_preset() {
        assert_eq!(MemoryBufferPercent::default_20().get(), 20);
    }

    #[test]
    fn test_memory_buffer_percent_display() {
        assert_eq!(format!("{}", MemoryBufferPercent::new(50).unwrap()), "50%");
    }

    // ========== MemoryWeight Tests ==========

    #[test]
    fn test_memory_weight_valid() {
        assert!(MemoryWeight::new(0).is_some());
        assert!(MemoryWeight::new(5000).is_some());
        assert!(MemoryWeight::new(10000).is_some());
        assert!(MemoryWeight::new(10001).is_none());
    }

    #[test]
    fn test_memory_weight_get() {
        let mw = MemoryWeight::new(5000).unwrap();
        assert_eq!(mw.get(), 5000);
    }

    #[test]
    fn test_memory_weight_default() {
        assert_eq!(MemoryWeight::default().get(), 5000);
    }

    #[test]
    fn test_memory_weight_constants() {
        assert_eq!(MemoryWeight::MIN, 0);
        assert_eq!(MemoryWeight::MAX, 10000);
        assert_eq!(MemoryWeight::DEFAULT, 5000);
    }

    // ========== ProcessorWeight Tests ==========

    #[test]
    fn test_processor_weight_valid() {
        assert!(ProcessorWeight::new(0).is_some());
        assert!(ProcessorWeight::new(100).is_some());
        assert!(ProcessorWeight::new(10000).is_some());
        assert!(ProcessorWeight::new(10001).is_none());
    }

    #[test]
    fn test_processor_weight_get() {
        let pw = ProcessorWeight::new(200).unwrap();
        assert_eq!(pw.get(), 200);
    }

    #[test]
    fn test_processor_weight_default() {
        assert_eq!(ProcessorWeight::default().get(), 100);
    }

    #[test]
    fn test_processor_weight_constants() {
        assert_eq!(ProcessorWeight::MIN, 0);
        assert_eq!(ProcessorWeight::MAX, 10000);
        assert_eq!(ProcessorWeight::DEFAULT, 100);
    }

    // ========== ProcessorPercent Tests ==========

    #[test]
    fn test_processor_percent_valid() {
        assert!(ProcessorPercent::new(0).is_some());
        assert!(ProcessorPercent::new(50000).is_some());
        assert!(ProcessorPercent::new(100000).is_some());
        assert!(ProcessorPercent::new(100001).is_none());
    }

    #[test]
    fn test_processor_percent_from_percent() {
        let pp = ProcessorPercent::from_percent(50).unwrap();
        assert_eq!(pp.get(), 50000);
        assert_eq!(pp.as_percent(), 50.0);

        let pp100 = ProcessorPercent::from_percent(100).unwrap();
        assert_eq!(pp100.get(), 100000);

        assert!(ProcessorPercent::from_percent(101).is_none());
    }

    #[test]
    fn test_processor_percent_default() {
        assert_eq!(ProcessorPercent::default().get(), 100000);
    }

    #[test]
    fn test_processor_percent_constant() {
        assert_eq!(ProcessorPercent::MAX, 100000);
    }

    // ========== DiskLocation Tests ==========

    #[test]
    fn test_disk_location_scsi() {
        assert!(DiskLocation::scsi(0).is_some());
        assert!(DiskLocation::scsi(63).is_some());
        assert!(DiskLocation::scsi(64).is_none());
    }

    #[test]
    fn test_disk_location_ide() {
        assert!(DiskLocation::ide(0).is_some());
        assert!(DiskLocation::ide(1).is_some());
        assert!(DiskLocation::ide(2).is_none());
    }

    #[test]
    fn test_disk_location_get() {
        assert_eq!(DiskLocation::scsi(10).unwrap().get(), 10);
        assert_eq!(DiskLocation::ide(1).unwrap().get(), 1);
    }

    #[test]
    fn test_disk_location_default() {
        assert_eq!(DiskLocation::default().get(), 0);
    }

    #[test]
    fn test_disk_location_display() {
        assert_eq!(format!("{}", DiskLocation::scsi(5).unwrap()), "Location 5");
    }

    #[test]
    fn test_disk_location_constants() {
        assert_eq!(DiskLocation::MAX_SCSI, 63);
        assert_eq!(DiskLocation::MAX_IDE, 1);
    }

    // ========== SectorSize Tests ==========

    #[test]
    fn test_sector_size_as_bytes() {
        assert_eq!(SectorSize::Bytes512.as_bytes(), 512);
        assert_eq!(SectorSize::Bytes4K.as_bytes(), 4096);
    }

    #[test]
    fn test_sector_size_from_bytes() {
        assert_eq!(SectorSize::from_bytes(512), Some(SectorSize::Bytes512));
        assert_eq!(SectorSize::from_bytes(4096), Some(SectorSize::Bytes4K));
        assert_eq!(SectorSize::from_bytes(1024), None);
        assert_eq!(SectorSize::from_bytes(0), None);
    }

    #[test]
    fn test_sector_size_default() {
        assert_eq!(SectorSize::default(), SectorSize::Bytes512);
    }

    #[test]
    fn test_sector_size_display() {
        assert_eq!(format!("{}", SectorSize::Bytes512), "512 bytes");
        assert_eq!(format!("{}", SectorSize::Bytes4K), "4K");
    }

    // ========== BlockSize Tests ==========

    #[test]
    fn test_block_size_from_bytes() {
        assert_eq!(BlockSize::from_bytes(512 * 1024), Some(BlockSize::KB_512));
        assert_eq!(BlockSize::from_bytes(1024 * 1024), Some(BlockSize::MB_1));
        assert_eq!(BlockSize::from_bytes(2 * 1024 * 1024), Some(BlockSize::MB_2));
        assert_eq!(BlockSize::from_bytes(16 * 1024 * 1024), Some(BlockSize::MB_16));
        assert_eq!(BlockSize::from_bytes(32 * 1024 * 1024), Some(BlockSize::MB_32));
        assert_eq!(BlockSize::from_bytes(64 * 1024 * 1024), Some(BlockSize::MB_64));
        assert_eq!(BlockSize::from_bytes(128 * 1024 * 1024), Some(BlockSize::MB_128));
        assert_eq!(BlockSize::from_bytes(256 * 1024 * 1024), Some(BlockSize::MB_256));
        assert_eq!(BlockSize::from_bytes(1234), None);
    }

    #[test]
    fn test_block_size_as_bytes() {
        assert_eq!(BlockSize::KB_512.as_bytes(), 512 * 1024);
        assert_eq!(BlockSize::MB_32.as_bytes(), 32 * 1024 * 1024);
    }

    #[test]
    fn test_block_size_default() {
        assert_eq!(BlockSize::default(), BlockSize::MB_32);
    }

    #[test]
    fn test_block_size_display() {
        assert_eq!(format!("{}", BlockSize::KB_512), "512 KB");
        assert_eq!(format!("{}", BlockSize::MB_1), "1 MB");
        assert_eq!(format!("{}", BlockSize::MB_32), "32 MB");
    }

    // ========== DiskSize Tests ==========

    #[test]
    fn test_disk_size() {
        let size = DiskSize::from_gb(100);
        assert_eq!(size.as_gb(), 100);
        assert!(size.is_valid_vhd());
        assert!(size.is_valid_vhdx());

        let large = DiskSize::from_tb(3);
        assert!(!large.is_valid_vhd());
        assert!(large.is_valid_vhdx());
    }

    #[test]
    fn test_disk_size_from_bytes() {
        let size = DiskSize::from_bytes(1024 * 1024 * 1024);
        assert_eq!(size.as_bytes(), 1024 * 1024 * 1024);
        assert_eq!(size.as_gb(), 1);
    }

    #[test]
    fn test_disk_size_from_tb() {
        let size = DiskSize::from_tb(2);
        assert_eq!(size.as_bytes(), 2 * 1024 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_disk_size_vhd_max() {
        let max_vhd = DiskSize::from_bytes(DiskSize::MAX_VHD);
        assert!(max_vhd.is_valid_vhd());
        let over_vhd = DiskSize::from_bytes(DiskSize::MAX_VHD + 1);
        assert!(!over_vhd.is_valid_vhd());
    }

    #[test]
    fn test_disk_size_vhdx_max() {
        let max_vhdx = DiskSize::from_bytes(DiskSize::MAX_VHDX);
        assert!(max_vhdx.is_valid_vhdx());
        let over_vhdx = DiskSize::from_bytes(DiskSize::MAX_VHDX + 1);
        assert!(!over_vhdx.is_valid_vhdx());
    }

    #[test]
    fn test_disk_size_zero_invalid() {
        let zero = DiskSize::from_bytes(0);
        assert!(!zero.is_valid_vhd());
        assert!(!zero.is_valid_vhdx());
    }

    #[test]
    fn test_disk_size_display() {
        assert_eq!(format!("{}", DiskSize::from_tb(1)), "1 TB");
        assert_eq!(format!("{}", DiskSize::from_gb(100)), "100 GB");
        assert_eq!(format!("{}", DiskSize::from_bytes(1234567)), "1234567 bytes");
    }

    #[test]
    fn test_disk_size_constants() {
        assert_eq!(DiskSize::MAX_VHD, 2 * 1024 * 1024 * 1024 * 1024);
        assert_eq!(DiskSize::MAX_VHDX, 64 * 1024 * 1024 * 1024 * 1024);
    }

    // ========== VlanId Tests ==========

    #[test]
    fn test_vlan_id_valid_range() {
        assert!(VlanId::new(0).is_some());
        assert!(VlanId::new(4094).is_some());
        assert!(VlanId::new(4095).is_none());
    }

    #[test]
    fn test_vlan_id_get() {
        let vlan = VlanId::new(100).unwrap();
        assert_eq!(vlan.get(), 100);
    }

    #[test]
    fn test_vlan_id_display() {
        assert_eq!(format!("{}", VlanId::new(100).unwrap()), "VLAN 100");
    }

    #[test]
    fn test_vlan_id_constant() {
        assert_eq!(VlanId::MAX, 4094);
    }

    // ========== MacAddress Tests ==========

    #[test]
    fn test_mac_address_parse() {
        assert!(MacAddress::parse("00:11:22:33:44:55").is_some());
        assert!(MacAddress::parse("00-11-22-33-44-55").is_some());
        assert!(MacAddress::parse("001122334455").is_some());
        assert!(MacAddress::parse("invalid").is_none());
    }

    #[test]
    fn test_mac_address_parse_invalid_length() {
        assert!(MacAddress::parse("00:11:22:33:44").is_none()); // Too short
        assert!(MacAddress::parse("00:11:22:33:44:55:66").is_none()); // Too long
        assert!(MacAddress::parse("").is_none());
    }

    #[test]
    fn test_mac_address_parse_invalid_chars() {
        assert!(MacAddress::parse("GG:11:22:33:44:55").is_none());
        assert!(MacAddress::parse("00:XX:22:33:44:55").is_none());
    }

    #[test]
    fn test_mac_address_new() {
        let mac = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(mac.as_bytes(), &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    }

    #[test]
    fn test_mac_address_to_colon_string() {
        let mac = MacAddress::parse("001122334455").unwrap();
        assert_eq!(mac.to_colon_string(), "00:11:22:33:44:55");
    }

    #[test]
    fn test_mac_address_to_hyperv_string() {
        let mac = MacAddress::parse("00:11:22:33:44:55").unwrap();
        assert_eq!(mac.to_hyperv_string(), "001122334455");
    }

    #[test]
    fn test_mac_address_display() {
        let mac = MacAddress::parse("00:11:22:33:44:55").unwrap();
        assert_eq!(format!("{}", mac), "00:11:22:33:44:55");
    }

    #[test]
    fn test_mac_address_uppercase() {
        let mac = MacAddress::parse("AA:BB:CC:DD:EE:FF").unwrap();
        assert_eq!(mac.to_colon_string(), "AA:BB:CC:DD:EE:FF");
    }

    #[test]
    fn test_mac_address_mixed_case() {
        let mac = MacAddress::parse("aA:bB:cC:dD:eE:fF").unwrap();
        assert_eq!(mac.as_bytes(), &[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    }

    // ========== BandwidthWeight Tests ==========

    #[test]
    fn test_bandwidth_weight() {
        assert!(BandwidthWeight::new(1).is_some());
        assert!(BandwidthWeight::new(100).is_some());
        assert!(BandwidthWeight::new(0).is_none());
        assert!(BandwidthWeight::new(101).is_none());
    }

    #[test]
    fn test_bandwidth_weight_get() {
        let bw = BandwidthWeight::new(75).unwrap();
        assert_eq!(bw.get(), 75);
    }

    #[test]
    fn test_bandwidth_weight_default() {
        assert_eq!(BandwidthWeight::default().get(), 50);
    }

    #[test]
    fn test_bandwidth_weight_display() {
        assert_eq!(format!("{}", BandwidthWeight::new(50).unwrap()), "50%");
    }

    #[test]
    fn test_bandwidth_weight_constants() {
        assert_eq!(BandwidthWeight::MIN, 1);
        assert_eq!(BandwidthWeight::MAX, 100);
    }
}
