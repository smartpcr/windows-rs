use crate::error::{Error, Result};
use crate::wmi::WbemClassObjectExt;
use windows::Win32::System::Wmi::IWbemClassObject;

/// Virtual switch type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchType {
    /// External - connected to physical network adapter.
    External,
    /// Internal - accessible from host and VMs.
    Internal,
    /// Private - only accessible between VMs.
    Private,
}

impl SwitchType {
    /// Convert from WMI IOVPreferred value.
    /// Note: Switch type is determined by examining the switch's connections,
    /// not a single property. This is a simplified mapping.
    pub fn from_value(value: u32) -> Self {
        match value {
            0 => SwitchType::Private,
            1 => SwitchType::Internal,
            2 => SwitchType::External,
            _ => SwitchType::Private,
        }
    }

    /// Convert to WMI value.
    pub fn to_value(&self) -> u32 {
        match self {
            SwitchType::Private => 0,
            SwitchType::Internal => 1,
            SwitchType::External => 2,
        }
    }

    /// Get the description for this switch type.
    pub fn description(&self) -> &'static str {
        match self {
            SwitchType::External => "Connected to physical network adapter",
            SwitchType::Internal => "Accessible from host and VMs",
            SwitchType::Private => "Only accessible between VMs",
        }
    }
}

impl Default for SwitchType {
    fn default() -> Self {
        SwitchType::Private
    }
}

impl std::fmt::Display for SwitchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SwitchType::External => write!(f, "External"),
            SwitchType::Internal => write!(f, "Internal"),
            SwitchType::Private => write!(f, "Private"),
        }
    }
}

/// IOV (Single Root I/O Virtualization) mode for a virtual switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IovMode {
    /// IOV is disabled.
    #[default]
    Disabled,
    /// IOV is enabled.
    Enabled,
}

impl IovMode {
    pub fn from_bool(enabled: bool) -> Self {
        if enabled {
            IovMode::Enabled
        } else {
            IovMode::Disabled
        }
    }

    pub fn is_enabled(&self) -> bool {
        matches!(self, IovMode::Enabled)
    }
}

/// Represents a Hyper-V virtual switch.
#[derive(Debug)]
pub struct VirtualSwitch {
    /// Switch display name.
    pub name: String,
    /// Switch unique ID (GUID).
    pub id: String,
    /// Switch type.
    pub switch_type: SwitchType,
    /// Description/notes.
    pub notes: Option<String>,
    /// Whether the switch allows management OS access.
    pub allow_management_os: bool,
    /// IOV mode.
    pub iov_mode: IovMode,
    /// Bandwidth reservation mode.
    pub bandwidth_reservation_mode: BandwidthReservationMode,
    /// Default flow minimum bandwidth (absolute, in bits/second).
    pub default_flow_min_bandwidth_absolute: u64,
    /// Default flow minimum bandwidth (weight, 1-100).
    pub default_flow_min_bandwidth_weight: u32,
    /// WMI path.
    path: String,
    /// Settings path (Msvm_VirtualEthernetSwitchSettingData).
    settings_path: Option<String>,
}

impl VirtualSwitch {
    /// Create from WMI object (Msvm_VirtualEthernetSwitch).
    /// This creates a basic switch; use `from_wmi_with_settings` for full details.
    pub(crate) fn from_wmi(obj: &IWbemClassObject) -> Result<Self> {
        let name = obj.get_string_prop_required("ElementName")?;
        let id = obj.get_string_prop_required("Name")?;
        let path = obj.get_path()?;
        let notes = obj.get_string_prop("Notes")?;

        Ok(Self {
            name,
            id,
            switch_type: SwitchType::Private, // Will be updated by from_wmi_with_settings
            notes,
            allow_management_os: false,
            iov_mode: IovMode::Disabled,
            bandwidth_reservation_mode: BandwidthReservationMode::None,
            default_flow_min_bandwidth_absolute: 0,
            default_flow_min_bandwidth_weight: 0,
            path,
            settings_path: None,
        })
    }

    /// Create from WMI objects with full settings.
    pub(crate) fn from_wmi_with_settings(
        switch_obj: &IWbemClassObject,
        settings_obj: Option<&IWbemClassObject>,
        has_external_port: bool,
        has_internal_port: bool,
    ) -> Result<Self> {
        let mut switch = Self::from_wmi(switch_obj)?;

        // Determine switch type based on port connections
        switch.switch_type = if has_external_port {
            SwitchType::External
        } else if has_internal_port {
            SwitchType::Internal
        } else {
            SwitchType::Private
        };

        // If we have settings, extract additional properties
        if let Some(settings) = settings_obj {
            switch.settings_path = Some(settings.get_path()?);
            switch.notes = settings.get_string_prop("Notes")?;

            // IOV support
            if let Some(iov) = settings.get_bool("IOVPreferred")? {
                switch.iov_mode = IovMode::from_bool(iov);
            }

            // Bandwidth settings
            if let Some(mode) = settings.get_u32("BandwidthReservationMode")? {
                switch.bandwidth_reservation_mode = BandwidthReservationMode::from_value(mode);
            }
            if let Some(abs) = settings.get_u64("DefaultFlowMinimumBandwidthAbsolute")? {
                switch.default_flow_min_bandwidth_absolute = abs;
            }
            if let Some(weight) = settings.get_u32("DefaultFlowMinimumBandwidthWeight")? {
                switch.default_flow_min_bandwidth_weight = weight;
            }
        }

        // Management OS access is determined by the presence of an internal ethernet port
        // connected to the management OS
        switch.allow_management_os = has_internal_port;

        Ok(switch)
    }

    /// Get the switch name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the switch ID (GUID).
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the switch type.
    pub fn switch_type(&self) -> SwitchType {
        self.switch_type
    }

    /// Get the notes/description.
    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    /// Check if management OS can access this switch.
    pub fn allows_management_os(&self) -> bool {
        self.allow_management_os
    }

    /// Get the IOV mode.
    pub fn iov_mode(&self) -> IovMode {
        self.iov_mode
    }

    /// Get the bandwidth reservation mode.
    pub fn bandwidth_reservation_mode(&self) -> BandwidthReservationMode {
        self.bandwidth_reservation_mode
    }

    /// Get the WMI path.
    pub(crate) fn path(&self) -> &str {
        &self.path
    }

    /// Get the settings WMI path if available.
    #[allow(dead_code)]
    pub(crate) fn settings_path(&self) -> Option<&str> {
        self.settings_path.as_deref()
    }
}

/// Bandwidth reservation mode for a virtual switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BandwidthReservationMode {
    /// No bandwidth reservation.
    #[default]
    None,
    /// Default mode.
    Default,
    /// Weight-based reservation (1-100).
    Weight,
    /// Absolute reservation (bits/second).
    Absolute,
}

impl BandwidthReservationMode {
    pub fn from_value(value: u32) -> Self {
        match value {
            0 => BandwidthReservationMode::None,
            1 => BandwidthReservationMode::Default,
            2 => BandwidthReservationMode::Weight,
            3 => BandwidthReservationMode::Absolute,
            _ => BandwidthReservationMode::None,
        }
    }

    pub fn to_value(&self) -> u32 {
        match self {
            BandwidthReservationMode::None => 0,
            BandwidthReservationMode::Default => 1,
            BandwidthReservationMode::Weight => 2,
            BandwidthReservationMode::Absolute => 3,
        }
    }
}

/// Settings for creating a virtual switch.
#[derive(Debug, Clone)]
pub struct VirtualSwitchSettings {
    /// Switch name (required).
    pub name: String,
    /// Switch type (required).
    pub switch_type: SwitchType,
    /// Notes/description.
    pub notes: Option<String>,
    /// Allow management OS to use this switch (for External/Internal).
    pub allow_management_os: bool,
    /// Physical adapter device ID (required for External type).
    pub external_adapter_id: Option<String>,
    /// Enable IOV (SR-IOV) - requires compatible hardware.
    pub enable_iov: bool,
    /// Bandwidth reservation mode.
    pub bandwidth_reservation_mode: BandwidthReservationMode,
    /// Default flow minimum bandwidth (absolute, in Mbps).
    /// Only used when bandwidth_reservation_mode is Absolute.
    pub default_flow_min_bandwidth_mbps: Option<u64>,
    /// Default flow minimum bandwidth (weight, 1-100).
    /// Only used when bandwidth_reservation_mode is Weight.
    pub default_flow_min_bandwidth_weight: Option<u32>,
    /// Enable packet direct for improved performance.
    pub enable_packet_direct: bool,
    /// Enable embedded teaming (SET) - Windows Server 2016+.
    pub enable_embedded_teaming: bool,
}

impl VirtualSwitchSettings {
    /// Create a new builder.
    pub fn builder() -> VirtualSwitchSettingsBuilder {
        VirtualSwitchSettingsBuilder::default()
    }

    /// Validate settings.
    pub fn validate(&self) -> Result<()> {
        // Name validation
        if self.name.is_empty() {
            return Err(Error::Validation {
                field: "name",
                message: "Switch name cannot be empty".to_string(),
            });
        }

        if self.name.len() > 100 {
            return Err(Error::Validation {
                field: "name",
                message: "Switch name cannot exceed 100 characters".to_string(),
            });
        }

        // Check for invalid characters in name
        if self.name.contains(['\\', '/', ':', '*', '?', '"', '<', '>', '|']) {
            return Err(Error::Validation {
                field: "name",
                message: "Switch name contains invalid characters".to_string(),
            });
        }

        // External switch requires a physical adapter
        if self.switch_type == SwitchType::External && self.external_adapter_id.is_none() {
            return Err(Error::Validation {
                field: "external_adapter_id",
                message: "External switch requires a physical network adapter".to_string(),
            });
        }

        // Private switch cannot have external adapter
        if self.switch_type == SwitchType::Private && self.external_adapter_id.is_some() {
            return Err(Error::Validation {
                field: "external_adapter_id",
                message: "Private switch cannot be connected to a physical adapter".to_string(),
            });
        }

        // Private switch cannot allow management OS
        if self.switch_type == SwitchType::Private && self.allow_management_os {
            return Err(Error::Validation {
                field: "allow_management_os",
                message: "Private switch cannot allow management OS access".to_string(),
            });
        }

        // Bandwidth weight validation
        if let Some(weight) = self.default_flow_min_bandwidth_weight {
            if weight == 0 || weight > 100 {
                return Err(Error::Validation {
                    field: "default_flow_min_bandwidth_weight",
                    message: "Bandwidth weight must be between 1 and 100".to_string(),
                });
            }
        }

        // Bandwidth mode consistency
        if self.bandwidth_reservation_mode == BandwidthReservationMode::Weight
            && self.default_flow_min_bandwidth_weight.is_none()
        {
            return Err(Error::Validation {
                field: "default_flow_min_bandwidth_weight",
                message: "Weight-based bandwidth mode requires a weight value".to_string(),
            });
        }

        if self.bandwidth_reservation_mode == BandwidthReservationMode::Absolute
            && self.default_flow_min_bandwidth_mbps.is_none()
        {
            return Err(Error::Validation {
                field: "default_flow_min_bandwidth_mbps",
                message: "Absolute bandwidth mode requires a bandwidth value".to_string(),
            });
        }

        // IOV requires external switch
        if self.enable_iov && self.switch_type != SwitchType::External {
            return Err(Error::Validation {
                field: "enable_iov",
                message: "IOV (SR-IOV) can only be enabled on external switches".to_string(),
            });
        }

        // Packet direct requires external switch
        if self.enable_packet_direct && self.switch_type != SwitchType::External {
            return Err(Error::Validation {
                field: "enable_packet_direct",
                message: "Packet Direct can only be enabled on external switches".to_string(),
            });
        }

        Ok(())
    }
}

/// Builder for virtual switch settings.
#[derive(Default)]
pub struct VirtualSwitchSettingsBuilder {
    name: Option<String>,
    switch_type: Option<SwitchType>,
    notes: Option<String>,
    allow_management_os: bool,
    external_adapter_id: Option<String>,
    enable_iov: bool,
    bandwidth_reservation_mode: BandwidthReservationMode,
    default_flow_min_bandwidth_mbps: Option<u64>,
    default_flow_min_bandwidth_weight: Option<u32>,
    enable_packet_direct: bool,
    enable_embedded_teaming: bool,
}

impl VirtualSwitchSettingsBuilder {
    /// Set switch name (required).
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set switch type (required).
    pub fn switch_type(mut self, switch_type: SwitchType) -> Self {
        self.switch_type = Some(switch_type);
        self
    }

    /// Create a private switch (VMs only).
    pub fn private(mut self) -> Self {
        self.switch_type = Some(SwitchType::Private);
        self.allow_management_os = false;
        self.external_adapter_id = None;
        self
    }

    /// Create an internal switch (host + VMs).
    pub fn internal(mut self) -> Self {
        self.switch_type = Some(SwitchType::Internal);
        self.allow_management_os = true;
        self.external_adapter_id = None;
        self
    }

    /// Create an external switch connected to a physical adapter.
    pub fn external(mut self, adapter_device_id: impl Into<String>) -> Self {
        self.switch_type = Some(SwitchType::External);
        self.external_adapter_id = Some(adapter_device_id.into());
        self
    }

    /// Set notes/description.
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Allow management OS to use this switch (External/Internal only).
    pub fn allow_management_os(mut self, allow: bool) -> Self {
        self.allow_management_os = allow;
        self
    }

    /// Enable IOV (SR-IOV) for improved network performance.
    /// Requires compatible hardware and external switch.
    pub fn enable_iov(mut self, enable: bool) -> Self {
        self.enable_iov = enable;
        self
    }

    /// Set bandwidth reservation mode.
    pub fn bandwidth_reservation_mode(mut self, mode: BandwidthReservationMode) -> Self {
        self.bandwidth_reservation_mode = mode;
        self
    }

    /// Set weight-based bandwidth reservation (1-100).
    pub fn bandwidth_weight(mut self, weight: u32) -> Self {
        self.bandwidth_reservation_mode = BandwidthReservationMode::Weight;
        self.default_flow_min_bandwidth_weight = Some(weight);
        self
    }

    /// Set absolute bandwidth reservation in Mbps.
    pub fn bandwidth_absolute_mbps(mut self, mbps: u64) -> Self {
        self.bandwidth_reservation_mode = BandwidthReservationMode::Absolute;
        self.default_flow_min_bandwidth_mbps = Some(mbps);
        self
    }

    /// Enable packet direct for improved performance.
    /// Requires external switch and compatible hardware.
    pub fn enable_packet_direct(mut self, enable: bool) -> Self {
        self.enable_packet_direct = enable;
        self
    }

    /// Enable Switch Embedded Teaming (SET).
    /// Windows Server 2016+ only.
    pub fn enable_embedded_teaming(mut self, enable: bool) -> Self {
        self.enable_embedded_teaming = enable;
        self
    }

    /// Build and validate settings.
    pub fn build(self) -> Result<VirtualSwitchSettings> {
        let name = self.name.ok_or(Error::MissingRequired("name"))?;
        let switch_type = self.switch_type.ok_or(Error::MissingRequired("switch_type"))?;

        let settings = VirtualSwitchSettings {
            name,
            switch_type,
            notes: self.notes,
            allow_management_os: self.allow_management_os,
            external_adapter_id: self.external_adapter_id,
            enable_iov: self.enable_iov,
            bandwidth_reservation_mode: self.bandwidth_reservation_mode,
            default_flow_min_bandwidth_mbps: self.default_flow_min_bandwidth_mbps,
            default_flow_min_bandwidth_weight: self.default_flow_min_bandwidth_weight,
            enable_packet_direct: self.enable_packet_direct,
            enable_embedded_teaming: self.enable_embedded_teaming,
        };

        settings.validate()?;
        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== SwitchType Tests ==========

    #[test]
    fn test_switch_type_from_value() {
        assert_eq!(SwitchType::from_value(0), SwitchType::Private);
        assert_eq!(SwitchType::from_value(1), SwitchType::Internal);
        assert_eq!(SwitchType::from_value(2), SwitchType::External);
        assert_eq!(SwitchType::from_value(99), SwitchType::Private); // Default for unknown
    }

    #[test]
    fn test_switch_type_to_value() {
        assert_eq!(SwitchType::Private.to_value(), 0);
        assert_eq!(SwitchType::Internal.to_value(), 1);
        assert_eq!(SwitchType::External.to_value(), 2);
    }

    #[test]
    fn test_switch_type_roundtrip() {
        for st in [SwitchType::Private, SwitchType::Internal, SwitchType::External] {
            assert_eq!(SwitchType::from_value(st.to_value()), st);
        }
    }

    #[test]
    fn test_switch_type_default() {
        assert_eq!(SwitchType::default(), SwitchType::Private);
    }

    #[test]
    fn test_switch_type_description() {
        assert!(!SwitchType::External.description().is_empty());
        assert!(!SwitchType::Internal.description().is_empty());
        assert!(!SwitchType::Private.description().is_empty());
    }

    #[test]
    fn test_switch_type_display() {
        assert_eq!(format!("{}", SwitchType::External), "External");
        assert_eq!(format!("{}", SwitchType::Internal), "Internal");
        assert_eq!(format!("{}", SwitchType::Private), "Private");
    }

    // ========== IovMode Tests ==========

    #[test]
    fn test_iov_mode_from_bool() {
        assert_eq!(IovMode::from_bool(true), IovMode::Enabled);
        assert_eq!(IovMode::from_bool(false), IovMode::Disabled);
    }

    #[test]
    fn test_iov_mode_is_enabled() {
        assert!(IovMode::Enabled.is_enabled());
        assert!(!IovMode::Disabled.is_enabled());
    }

    #[test]
    fn test_iov_mode_default() {
        assert_eq!(IovMode::default(), IovMode::Disabled);
    }

    // ========== BandwidthReservationMode Tests ==========

    #[test]
    fn test_bandwidth_mode_from_value() {
        assert_eq!(BandwidthReservationMode::from_value(0), BandwidthReservationMode::None);
        assert_eq!(BandwidthReservationMode::from_value(1), BandwidthReservationMode::Default);
        assert_eq!(BandwidthReservationMode::from_value(2), BandwidthReservationMode::Weight);
        assert_eq!(BandwidthReservationMode::from_value(3), BandwidthReservationMode::Absolute);
        assert_eq!(BandwidthReservationMode::from_value(99), BandwidthReservationMode::None);
    }

    #[test]
    fn test_bandwidth_mode_to_value() {
        assert_eq!(BandwidthReservationMode::None.to_value(), 0);
        assert_eq!(BandwidthReservationMode::Default.to_value(), 1);
        assert_eq!(BandwidthReservationMode::Weight.to_value(), 2);
        assert_eq!(BandwidthReservationMode::Absolute.to_value(), 3);
    }

    #[test]
    fn test_bandwidth_mode_roundtrip() {
        for mode in [
            BandwidthReservationMode::None,
            BandwidthReservationMode::Default,
            BandwidthReservationMode::Weight,
            BandwidthReservationMode::Absolute,
        ] {
            assert_eq!(BandwidthReservationMode::from_value(mode.to_value()), mode);
        }
    }

    // ========== VirtualSwitchSettings Builder Tests ==========

    #[test]
    fn test_switch_settings_builder_private() {
        let result = VirtualSwitchSettings::builder()
            .name("TestSwitch")
            .private()
            .build();
        assert!(result.is_ok());
        let settings = result.unwrap();
        assert_eq!(settings.name, "TestSwitch");
        assert_eq!(settings.switch_type, SwitchType::Private);
        assert!(!settings.allow_management_os);
    }

    #[test]
    fn test_switch_settings_builder_internal() {
        let result = VirtualSwitchSettings::builder()
            .name("InternalSwitch")
            .internal()
            .build();
        assert!(result.is_ok());
        let settings = result.unwrap();
        assert_eq!(settings.switch_type, SwitchType::Internal);
        assert!(settings.allow_management_os);
    }

    #[test]
    fn test_switch_settings_builder_external_with_adapter() {
        let result = VirtualSwitchSettings::builder()
            .name("ExternalSwitch")
            .external("Ethernet0")
            .allow_management_os(true)
            .build();
        assert!(result.is_ok());
        let settings = result.unwrap();
        assert_eq!(settings.switch_type, SwitchType::External);
        assert_eq!(settings.external_adapter_id, Some("Ethernet0".to_string()));
    }

    #[test]
    fn test_switch_settings_builder_external_without_adapter() {
        let result = VirtualSwitchSettings::builder()
            .name("ExternalSwitch")
            .switch_type(SwitchType::External)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_switch_settings_builder_missing_name() {
        let result = VirtualSwitchSettings::builder()
            .switch_type(SwitchType::Private)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_switch_settings_builder_missing_type() {
        let result = VirtualSwitchSettings::builder()
            .name("TestSwitch")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_switch_settings_validation_empty_name() {
        let result = VirtualSwitchSettings::builder()
            .name("")
            .private()
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_switch_settings_validation_name_too_long() {
        let long_name = "x".repeat(101);
        let result = VirtualSwitchSettings::builder()
            .name(&long_name)
            .private()
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_switch_settings_validation_invalid_name_chars() {
        for invalid_char in ['\\', '/', ':', '*', '?', '"', '<', '>', '|'] {
            let name = format!("Test{}Switch", invalid_char);
            let result = VirtualSwitchSettings::builder()
                .name(&name)
                .private()
                .build();
            assert!(result.is_err(), "Should reject name with '{}'", invalid_char);
        }
    }

    #[test]
    fn test_switch_settings_private_with_adapter() {
        let result = VirtualSwitchSettings::builder()
            .name("PrivateSwitch")
            .private()
            .build();
        // Private switch doesn't have external_adapter_id, so this should work
        assert!(result.is_ok());

        // But if we manually set external_adapter_id on a private switch, validation should fail
        let settings = VirtualSwitchSettings {
            name: "PrivateSwitch".to_string(),
            switch_type: SwitchType::Private,
            notes: None,
            allow_management_os: false,
            external_adapter_id: Some("Ethernet0".to_string()), // Invalid for private
            enable_iov: false,
            bandwidth_reservation_mode: BandwidthReservationMode::None,
            default_flow_min_bandwidth_mbps: None,
            default_flow_min_bandwidth_weight: None,
            enable_packet_direct: false,
            enable_embedded_teaming: false,
        };
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_switch_settings_private_with_management_os() {
        let settings = VirtualSwitchSettings {
            name: "PrivateSwitch".to_string(),
            switch_type: SwitchType::Private,
            notes: None,
            allow_management_os: true, // Invalid for private
            external_adapter_id: None,
            enable_iov: false,
            bandwidth_reservation_mode: BandwidthReservationMode::None,
            default_flow_min_bandwidth_mbps: None,
            default_flow_min_bandwidth_weight: None,
            enable_packet_direct: false,
            enable_embedded_teaming: false,
        };
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_switch_settings_with_notes() {
        let result = VirtualSwitchSettings::builder()
            .name("TestSwitch")
            .private()
            .notes("This is a test switch")
            .build();
        assert!(result.is_ok());
        let settings = result.unwrap();
        assert_eq!(settings.notes, Some("This is a test switch".to_string()));
    }

    #[test]
    fn test_switch_settings_bandwidth_weight_valid() {
        let result = VirtualSwitchSettings::builder()
            .name("TestSwitch")
            .private()
            .bandwidth_weight(50)
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_switch_settings_bandwidth_weight_invalid_zero() {
        let result = VirtualSwitchSettings::builder()
            .name("TestSwitch")
            .private()
            .bandwidth_weight(0)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_switch_settings_bandwidth_weight_invalid_over_100() {
        let result = VirtualSwitchSettings::builder()
            .name("TestSwitch")
            .private()
            .bandwidth_weight(101)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_switch_settings_bandwidth_weight_mode_without_value() {
        let settings = VirtualSwitchSettings {
            name: "TestSwitch".to_string(),
            switch_type: SwitchType::Private,
            notes: None,
            allow_management_os: false,
            external_adapter_id: None,
            enable_iov: false,
            bandwidth_reservation_mode: BandwidthReservationMode::Weight,
            default_flow_min_bandwidth_mbps: None,
            default_flow_min_bandwidth_weight: None, // Required for Weight mode
            enable_packet_direct: false,
            enable_embedded_teaming: false,
        };
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_switch_settings_bandwidth_absolute_mode_without_value() {
        let settings = VirtualSwitchSettings {
            name: "TestSwitch".to_string(),
            switch_type: SwitchType::Private,
            notes: None,
            allow_management_os: false,
            external_adapter_id: None,
            enable_iov: false,
            bandwidth_reservation_mode: BandwidthReservationMode::Absolute,
            default_flow_min_bandwidth_mbps: None, // Required for Absolute mode
            default_flow_min_bandwidth_weight: None,
            enable_packet_direct: false,
            enable_embedded_teaming: false,
        };
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_switch_settings_bandwidth_absolute_valid() {
        let result = VirtualSwitchSettings::builder()
            .name("TestSwitch")
            .private()
            .bandwidth_absolute_mbps(1000)
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_switch_settings_iov_requires_external() {
        let settings = VirtualSwitchSettings {
            name: "TestSwitch".to_string(),
            switch_type: SwitchType::Internal,
            notes: None,
            allow_management_os: true,
            external_adapter_id: None,
            enable_iov: true, // Invalid for non-external
            bandwidth_reservation_mode: BandwidthReservationMode::None,
            default_flow_min_bandwidth_mbps: None,
            default_flow_min_bandwidth_weight: None,
            enable_packet_direct: false,
            enable_embedded_teaming: false,
        };
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_switch_settings_packet_direct_requires_external() {
        let settings = VirtualSwitchSettings {
            name: "TestSwitch".to_string(),
            switch_type: SwitchType::Private,
            notes: None,
            allow_management_os: false,
            external_adapter_id: None,
            enable_iov: false,
            bandwidth_reservation_mode: BandwidthReservationMode::None,
            default_flow_min_bandwidth_mbps: None,
            default_flow_min_bandwidth_weight: None,
            enable_packet_direct: true, // Invalid for non-external
            enable_embedded_teaming: false,
        };
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_switch_settings_external_with_iov() {
        let result = VirtualSwitchSettings::builder()
            .name("ExternalSwitch")
            .external("Ethernet0")
            .enable_iov(true)
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_switch_settings_external_with_packet_direct() {
        let result = VirtualSwitchSettings::builder()
            .name("ExternalSwitch")
            .external("Ethernet0")
            .enable_packet_direct(true)
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_switch_settings_external_with_embedded_teaming() {
        let result = VirtualSwitchSettings::builder()
            .name("ExternalSwitch")
            .external("Ethernet0")
            .enable_embedded_teaming(true)
            .build();
        assert!(result.is_ok());
    }
}
