use crate::error::Result;
use crate::wmi::WbemClassObjectExt;
use windows::Win32::System::Wmi::IWbemClassObject;

/// Represents a physical network adapter that can be used for external virtual switches.
#[derive(Debug, Clone)]
pub struct PhysicalAdapter {
    /// Device ID (unique identifier).
    pub device_id: String,
    /// Adapter name/description.
    pub name: String,
    /// MAC address.
    pub mac_address: Option<String>,
    /// Whether the adapter is enabled.
    pub enabled: bool,
    /// Connection status.
    pub connection_status: ConnectionStatus,
    /// Speed in bits per second (0 if disconnected).
    pub speed_bps: u64,
    /// WMI path.
    path: String,
}

impl PhysicalAdapter {
    /// Create from WMI Msvm_ExternalEthernetPort object.
    pub(crate) fn from_wmi(obj: &IWbemClassObject) -> Result<Self> {
        let device_id = obj.get_string_prop_required("DeviceID")?;
        let name = obj.get_string_prop("ElementName")?.unwrap_or_else(|| device_id.clone());
        let path = obj.get_path()?;
        let mac_address = obj.get_string_prop("PermanentAddress")?;
        let enabled = obj.get_bool("EnabledState")?.map(|v| v).unwrap_or(false);

        // OperationalStatus is an array, we check the first element
        // 2 = OK, 10 = Stopped, 12 = Disabled
        let op_status = obj.get_u16("OperationalStatus")?.unwrap_or(0);
        let connection_status = ConnectionStatus::from_value(op_status);

        let speed_bps = obj.get_u64("Speed")?.unwrap_or(0);

        Ok(Self {
            device_id,
            name,
            mac_address,
            enabled,
            connection_status,
            speed_bps,
            path,
        })
    }

    /// Get the device ID.
    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    /// Get the adapter name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the MAC address if available.
    pub fn mac_address(&self) -> Option<&str> {
        self.mac_address.as_deref()
    }

    /// Check if the adapter is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get the connection status.
    pub fn connection_status(&self) -> ConnectionStatus {
        self.connection_status
    }

    /// Get the speed in Mbps.
    pub fn speed_mbps(&self) -> u64 {
        self.speed_bps / 1_000_000
    }

    /// Get the speed in Gbps.
    pub fn speed_gbps(&self) -> f64 {
        self.speed_bps as f64 / 1_000_000_000.0
    }

    /// Get the WMI path.
    pub(crate) fn path(&self) -> &str {
        &self.path
    }
}

/// Connection status of a physical network adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectionStatus {
    /// Status is unknown.
    #[default]
    Unknown,
    /// Adapter is connected and operational.
    Connected,
    /// Adapter is disconnected (no cable or link).
    Disconnected,
    /// Adapter is disabled.
    Disabled,
    /// Adapter has an error.
    Error,
    /// Other status code.
    Other(u16),
}

impl ConnectionStatus {
    /// Convert from WMI OperationalStatus value.
    pub fn from_value(value: u16) -> Self {
        match value {
            0 => ConnectionStatus::Unknown,
            2 => ConnectionStatus::Connected,  // OK
            10 => ConnectionStatus::Disconnected, // Stopped
            12 => ConnectionStatus::Disabled,
            6 => ConnectionStatus::Error,
            _ => ConnectionStatus::Other(value),
        }
    }

    /// Check if the adapter is connected and usable.
    pub fn is_connected(&self) -> bool {
        matches!(self, ConnectionStatus::Connected)
    }
}

impl std::fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionStatus::Unknown => write!(f, "Unknown"),
            ConnectionStatus::Connected => write!(f, "Connected"),
            ConnectionStatus::Disconnected => write!(f, "Disconnected"),
            ConnectionStatus::Disabled => write!(f, "Disabled"),
            ConnectionStatus::Error => write!(f, "Error"),
            ConnectionStatus::Other(v) => write!(f, "Other({})", v),
        }
    }
}
