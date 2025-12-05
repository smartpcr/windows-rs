use core::fmt;

/// VM enabled state (Msvm_ComputerSystem.EnabledState).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum VmState {
    /// Unknown state.
    Unknown = 0,
    /// VM is running.
    Running = 2,
    /// VM is powered off.
    Off = 3,
    /// VM is in the process of shutting down.
    ShuttingDown = 4,
    /// Not applicable.
    NotApplicable = 5,
    /// VM is disabled.
    Disabled = 6,
    /// VM is paused.
    Paused = 32768,
    /// VM is suspended/saved.
    Suspended = 32769,
    /// VM is starting.
    Starting = 32770,
    /// VM is in a saved snapshot state.
    Snapshotting = 32771,
    /// VM is saving state.
    Saving = 32773,
    /// VM is stopping.
    Stopping = 32774,
    /// VM is pausing.
    Pausing = 32776,
    /// VM is resuming.
    Resuming = 32777,
}

impl VmState {
    /// Parse from WMI EnabledState value.
    pub fn from_enabled_state(value: u16) -> Self {
        match value {
            2 => VmState::Running,
            3 => VmState::Off,
            4 => VmState::ShuttingDown,
            5 => VmState::NotApplicable,
            6 => VmState::Disabled,
            32768 => VmState::Paused,
            32769 => VmState::Suspended,
            32770 => VmState::Starting,
            32771 => VmState::Snapshotting,
            32773 => VmState::Saving,
            32774 => VmState::Stopping,
            32776 => VmState::Pausing,
            32777 => VmState::Resuming,
            _ => VmState::Unknown,
        }
    }

    /// Check if VM can be started.
    pub fn can_start(&self) -> bool {
        matches!(self, VmState::Off | VmState::Suspended | VmState::Paused)
    }

    /// Check if VM can be stopped.
    pub fn can_stop(&self) -> bool {
        matches!(self, VmState::Running | VmState::Paused | VmState::Suspended)
    }

    /// Check if VM can be paused.
    pub fn can_pause(&self) -> bool {
        matches!(self, VmState::Running)
    }

    /// Check if VM can be saved.
    pub fn can_save(&self) -> bool {
        matches!(self, VmState::Running | VmState::Paused)
    }

    /// Check if VM is in a transitional state.
    pub fn is_transitional(&self) -> bool {
        matches!(
            self,
            VmState::Starting
                | VmState::Stopping
                | VmState::Saving
                | VmState::Pausing
                | VmState::Resuming
                | VmState::ShuttingDown
                | VmState::Snapshotting
        )
    }
}

impl fmt::Display for VmState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VmState::Unknown => "Unknown",
            VmState::Running => "Running",
            VmState::Off => "Off",
            VmState::ShuttingDown => "Shutting Down",
            VmState::NotApplicable => "Not Applicable",
            VmState::Disabled => "Disabled",
            VmState::Paused => "Paused",
            VmState::Suspended => "Saved",
            VmState::Starting => "Starting",
            VmState::Snapshotting => "Taking Snapshot",
            VmState::Saving => "Saving",
            VmState::Stopping => "Stopping",
            VmState::Pausing => "Pausing",
            VmState::Resuming => "Resuming",
        };
        write!(f, "{}", s)
    }
}

/// VM generation (Gen1 = BIOS, Gen2 = UEFI).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Generation {
    /// Generation 1 VM (BIOS-based, IDE boot).
    #[default]
    Gen1,
    /// Generation 2 VM (UEFI-based, Secure Boot capable).
    Gen2,
}

impl Generation {
    /// Get the WMI VirtualSystemSubType value.
    pub fn to_subtype(&self) -> &'static str {
        match self {
            Generation::Gen1 => "Microsoft:Hyper-V:SubType:1",
            Generation::Gen2 => "Microsoft:Hyper-V:SubType:2",
        }
    }

    /// Parse from WMI VirtualSystemSubType value.
    pub fn from_subtype(subtype: &str) -> Self {
        if subtype.contains(":2") {
            Generation::Gen2
        } else {
            Generation::Gen1
        }
    }
}

impl fmt::Display for Generation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Generation::Gen1 => write!(f, "Generation 1"),
            Generation::Gen2 => write!(f, "Generation 2"),
        }
    }
}

/// VM operational status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum OperationalStatus {
    Unknown = 0,
    Ok = 2,
    Degraded = 3,
    Stressed = 4,
    PredictiveFailure = 5,
    Error = 6,
    NonRecoverableError = 7,
    Starting = 8,
    Stopping = 9,
    Stopped = 10,
    InService = 11,
    NoContact = 12,
    LostCommunication = 13,
    Aborted = 14,
    Dormant = 15,
    SupportingEntity = 16,
    Completed = 17,
    PowerMode = 18,
    ProtocolVersionMismatch = 32775,
    ApplicationCriticalState = 32782,
    CommunicationTimedOut = 32783,
    CommunicationFailed = 32784,
}

impl OperationalStatus {
    pub fn from_value(value: u16) -> Self {
        match value {
            2 => OperationalStatus::Ok,
            3 => OperationalStatus::Degraded,
            4 => OperationalStatus::Stressed,
            5 => OperationalStatus::PredictiveFailure,
            6 => OperationalStatus::Error,
            7 => OperationalStatus::NonRecoverableError,
            8 => OperationalStatus::Starting,
            9 => OperationalStatus::Stopping,
            10 => OperationalStatus::Stopped,
            11 => OperationalStatus::InService,
            12 => OperationalStatus::NoContact,
            13 => OperationalStatus::LostCommunication,
            14 => OperationalStatus::Aborted,
            15 => OperationalStatus::Dormant,
            16 => OperationalStatus::SupportingEntity,
            17 => OperationalStatus::Completed,
            18 => OperationalStatus::PowerMode,
            32775 => OperationalStatus::ProtocolVersionMismatch,
            32782 => OperationalStatus::ApplicationCriticalState,
            32783 => OperationalStatus::CommunicationTimedOut,
            32784 => OperationalStatus::CommunicationFailed,
            _ => OperationalStatus::Unknown,
        }
    }
}

/// Requested state for VM state change operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum RequestedState {
    /// Start the VM.
    Running = 2,
    /// Power off the VM (hard stop).
    Off = 3,
    /// Pause the VM.
    Paused = 32768,
    /// Save (suspend) the VM.
    Saved = 32769,
    /// Reset the VM.
    Reset = 11,
}

/// Shutdown type for graceful shutdown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShutdownType {
    /// Graceful shutdown through guest integration services.
    Graceful,
    /// Force power off.
    Force,
    /// Graceful shutdown, fall back to force if needed.
    GracefulWithForce,
}

/// Checkpoint type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CheckpointType {
    /// Disabled - no checkpoints.
    Disabled,
    /// Production checkpoint (application-consistent, preferred).
    #[default]
    Production,
    /// Production checkpoint only (fail if not possible).
    ProductionOnly,
    /// Standard checkpoint (crash-consistent).
    Standard,
}

impl CheckpointType {
    pub fn to_value(&self) -> u16 {
        match self {
            CheckpointType::Disabled => 0,
            CheckpointType::Production => 1,
            CheckpointType::ProductionOnly => 2,
            CheckpointType::Standard => 3,
        }
    }

    pub fn from_value(value: u16) -> Self {
        match value {
            0 => CheckpointType::Disabled,
            1 => CheckpointType::Production,
            2 => CheckpointType::ProductionOnly,
            3 => CheckpointType::Standard,
            _ => CheckpointType::Production,
        }
    }
}

/// Automatic start action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AutomaticStartAction {
    /// Do nothing on host start.
    #[default]
    Nothing,
    /// Automatically start if VM was running.
    StartIfRunning,
    /// Always start the VM.
    AlwaysStart,
}

impl AutomaticStartAction {
    pub fn to_value(&self) -> u16 {
        match self {
            AutomaticStartAction::Nothing => 0,
            AutomaticStartAction::StartIfRunning => 1,
            AutomaticStartAction::AlwaysStart => 2,
        }
    }

    pub fn from_value(value: u16) -> Self {
        match value {
            0 => AutomaticStartAction::Nothing,
            1 => AutomaticStartAction::StartIfRunning,
            2 => AutomaticStartAction::AlwaysStart,
            _ => AutomaticStartAction::Nothing,
        }
    }
}

use crate::error::VmStateError;

impl VmState {
    /// Convert to VmStateError for error reporting.
    pub fn to_error(&self) -> VmStateError {
        match self {
            VmState::Unknown => VmStateError::Unknown,
            VmState::Running => VmStateError::Running,
            VmState::Off => VmStateError::Off,
            VmState::ShuttingDown => VmStateError::ShuttingDown,
            VmState::Paused => VmStateError::Paused,
            VmState::Suspended => VmStateError::Suspended,
            VmState::Starting => VmStateError::Starting,
            VmState::Stopping => VmStateError::Stopping,
            _ => VmStateError::Other(*self as u16),
        }
    }
}

/// Automatic stop action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AutomaticStopAction {
    /// Turn off the VM.
    TurnOff,
    /// Save the VM state.
    #[default]
    Save,
    /// Graceful shutdown.
    Shutdown,
}

impl AutomaticStopAction {
    pub fn to_value(&self) -> u16 {
        match self {
            AutomaticStopAction::TurnOff => 0,
            AutomaticStopAction::Save => 1,
            AutomaticStopAction::Shutdown => 2,
        }
    }

    pub fn from_value(value: u16) -> Self {
        match value {
            0 => AutomaticStopAction::TurnOff,
            1 => AutomaticStopAction::Save,
            2 => AutomaticStopAction::Shutdown,
            _ => AutomaticStopAction::Save,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== VmState Tests ==========

    #[test]
    fn test_vm_state_from_enabled_state() {
        assert_eq!(VmState::from_enabled_state(2), VmState::Running);
        assert_eq!(VmState::from_enabled_state(3), VmState::Off);
        assert_eq!(VmState::from_enabled_state(4), VmState::ShuttingDown);
        assert_eq!(VmState::from_enabled_state(5), VmState::NotApplicable);
        assert_eq!(VmState::from_enabled_state(6), VmState::Disabled);
        assert_eq!(VmState::from_enabled_state(32768), VmState::Paused);
        assert_eq!(VmState::from_enabled_state(32769), VmState::Suspended);
        assert_eq!(VmState::from_enabled_state(32770), VmState::Starting);
        assert_eq!(VmState::from_enabled_state(32771), VmState::Snapshotting);
        assert_eq!(VmState::from_enabled_state(32773), VmState::Saving);
        assert_eq!(VmState::from_enabled_state(32774), VmState::Stopping);
        assert_eq!(VmState::from_enabled_state(32776), VmState::Pausing);
        assert_eq!(VmState::from_enabled_state(32777), VmState::Resuming);
        assert_eq!(VmState::from_enabled_state(9999), VmState::Unknown);
    }

    #[test]
    fn test_vm_state_can_start() {
        assert!(VmState::Off.can_start());
        assert!(VmState::Suspended.can_start());
        assert!(VmState::Paused.can_start());
        assert!(!VmState::Running.can_start());
        assert!(!VmState::Starting.can_start());
        assert!(!VmState::Unknown.can_start());
    }

    #[test]
    fn test_vm_state_can_stop() {
        assert!(VmState::Running.can_stop());
        assert!(VmState::Paused.can_stop());
        assert!(VmState::Suspended.can_stop());
        assert!(!VmState::Off.can_stop());
        assert!(!VmState::Stopping.can_stop());
        assert!(!VmState::Unknown.can_stop());
    }

    #[test]
    fn test_vm_state_can_pause() {
        assert!(VmState::Running.can_pause());
        assert!(!VmState::Off.can_pause());
        assert!(!VmState::Paused.can_pause());
        assert!(!VmState::Suspended.can_pause());
        assert!(!VmState::Starting.can_pause());
    }

    #[test]
    fn test_vm_state_can_save() {
        assert!(VmState::Running.can_save());
        assert!(VmState::Paused.can_save());
        assert!(!VmState::Off.can_save());
        assert!(!VmState::Suspended.can_save());
        assert!(!VmState::Starting.can_save());
    }

    #[test]
    fn test_vm_state_is_transitional() {
        assert!(VmState::Starting.is_transitional());
        assert!(VmState::Stopping.is_transitional());
        assert!(VmState::Saving.is_transitional());
        assert!(VmState::Pausing.is_transitional());
        assert!(VmState::Resuming.is_transitional());
        assert!(VmState::ShuttingDown.is_transitional());
        assert!(VmState::Snapshotting.is_transitional());

        assert!(!VmState::Running.is_transitional());
        assert!(!VmState::Off.is_transitional());
        assert!(!VmState::Paused.is_transitional());
        assert!(!VmState::Suspended.is_transitional());
        assert!(!VmState::Unknown.is_transitional());
    }

    #[test]
    fn test_vm_state_display() {
        assert_eq!(format!("{}", VmState::Unknown), "Unknown");
        assert_eq!(format!("{}", VmState::Running), "Running");
        assert_eq!(format!("{}", VmState::Off), "Off");
        assert_eq!(format!("{}", VmState::ShuttingDown), "Shutting Down");
        assert_eq!(format!("{}", VmState::NotApplicable), "Not Applicable");
        assert_eq!(format!("{}", VmState::Disabled), "Disabled");
        assert_eq!(format!("{}", VmState::Paused), "Paused");
        assert_eq!(format!("{}", VmState::Suspended), "Saved");
        assert_eq!(format!("{}", VmState::Starting), "Starting");
        assert_eq!(format!("{}", VmState::Snapshotting), "Taking Snapshot");
        assert_eq!(format!("{}", VmState::Saving), "Saving");
        assert_eq!(format!("{}", VmState::Stopping), "Stopping");
        assert_eq!(format!("{}", VmState::Pausing), "Pausing");
        assert_eq!(format!("{}", VmState::Resuming), "Resuming");
    }

    // ========== Generation Tests ==========

    #[test]
    fn test_generation_to_subtype() {
        assert_eq!(Generation::Gen1.to_subtype(), "Microsoft:Hyper-V:SubType:1");
        assert_eq!(Generation::Gen2.to_subtype(), "Microsoft:Hyper-V:SubType:2");
    }

    #[test]
    fn test_generation_from_subtype() {
        assert_eq!(Generation::from_subtype("Microsoft:Hyper-V:SubType:1"), Generation::Gen1);
        assert_eq!(Generation::from_subtype("Microsoft:Hyper-V:SubType:2"), Generation::Gen2);
        assert_eq!(Generation::from_subtype("something:2:else"), Generation::Gen2);
        assert_eq!(Generation::from_subtype("no number"), Generation::Gen1);
    }

    #[test]
    fn test_generation_default() {
        assert_eq!(Generation::default(), Generation::Gen1);
    }

    #[test]
    fn test_generation_display() {
        assert_eq!(format!("{}", Generation::Gen1), "Generation 1");
        assert_eq!(format!("{}", Generation::Gen2), "Generation 2");
    }

    // ========== OperationalStatus Tests ==========

    #[test]
    fn test_operational_status_from_value() {
        assert_eq!(OperationalStatus::from_value(2), OperationalStatus::Ok);
        assert_eq!(OperationalStatus::from_value(3), OperationalStatus::Degraded);
        assert_eq!(OperationalStatus::from_value(4), OperationalStatus::Stressed);
        assert_eq!(OperationalStatus::from_value(5), OperationalStatus::PredictiveFailure);
        assert_eq!(OperationalStatus::from_value(6), OperationalStatus::Error);
        assert_eq!(OperationalStatus::from_value(7), OperationalStatus::NonRecoverableError);
        assert_eq!(OperationalStatus::from_value(8), OperationalStatus::Starting);
        assert_eq!(OperationalStatus::from_value(9), OperationalStatus::Stopping);
        assert_eq!(OperationalStatus::from_value(10), OperationalStatus::Stopped);
        assert_eq!(OperationalStatus::from_value(11), OperationalStatus::InService);
        assert_eq!(OperationalStatus::from_value(12), OperationalStatus::NoContact);
        assert_eq!(OperationalStatus::from_value(13), OperationalStatus::LostCommunication);
        assert_eq!(OperationalStatus::from_value(14), OperationalStatus::Aborted);
        assert_eq!(OperationalStatus::from_value(15), OperationalStatus::Dormant);
        assert_eq!(OperationalStatus::from_value(16), OperationalStatus::SupportingEntity);
        assert_eq!(OperationalStatus::from_value(17), OperationalStatus::Completed);
        assert_eq!(OperationalStatus::from_value(18), OperationalStatus::PowerMode);
        assert_eq!(OperationalStatus::from_value(32775), OperationalStatus::ProtocolVersionMismatch);
        assert_eq!(OperationalStatus::from_value(32782), OperationalStatus::ApplicationCriticalState);
        assert_eq!(OperationalStatus::from_value(32783), OperationalStatus::CommunicationTimedOut);
        assert_eq!(OperationalStatus::from_value(32784), OperationalStatus::CommunicationFailed);
        assert_eq!(OperationalStatus::from_value(9999), OperationalStatus::Unknown);
    }

    // ========== CheckpointType Tests ==========

    #[test]
    fn test_checkpoint_type_to_value() {
        assert_eq!(CheckpointType::Disabled.to_value(), 0);
        assert_eq!(CheckpointType::Production.to_value(), 1);
        assert_eq!(CheckpointType::ProductionOnly.to_value(), 2);
        assert_eq!(CheckpointType::Standard.to_value(), 3);
    }

    #[test]
    fn test_checkpoint_type_from_value() {
        assert_eq!(CheckpointType::from_value(0), CheckpointType::Disabled);
        assert_eq!(CheckpointType::from_value(1), CheckpointType::Production);
        assert_eq!(CheckpointType::from_value(2), CheckpointType::ProductionOnly);
        assert_eq!(CheckpointType::from_value(3), CheckpointType::Standard);
        assert_eq!(CheckpointType::from_value(99), CheckpointType::Production);
    }

    #[test]
    fn test_checkpoint_type_default() {
        assert_eq!(CheckpointType::default(), CheckpointType::Production);
    }

    #[test]
    fn test_checkpoint_type_roundtrip() {
        for ct in [
            CheckpointType::Disabled,
            CheckpointType::Production,
            CheckpointType::ProductionOnly,
            CheckpointType::Standard,
        ] {
            assert_eq!(CheckpointType::from_value(ct.to_value()), ct);
        }
    }

    // ========== AutomaticStartAction Tests ==========

    #[test]
    fn test_automatic_start_action_to_value() {
        assert_eq!(AutomaticStartAction::Nothing.to_value(), 0);
        assert_eq!(AutomaticStartAction::StartIfRunning.to_value(), 1);
        assert_eq!(AutomaticStartAction::AlwaysStart.to_value(), 2);
    }

    #[test]
    fn test_automatic_start_action_from_value() {
        assert_eq!(AutomaticStartAction::from_value(0), AutomaticStartAction::Nothing);
        assert_eq!(AutomaticStartAction::from_value(1), AutomaticStartAction::StartIfRunning);
        assert_eq!(AutomaticStartAction::from_value(2), AutomaticStartAction::AlwaysStart);
        assert_eq!(AutomaticStartAction::from_value(99), AutomaticStartAction::Nothing);
    }

    #[test]
    fn test_automatic_start_action_default() {
        assert_eq!(AutomaticStartAction::default(), AutomaticStartAction::Nothing);
    }

    #[test]
    fn test_automatic_start_action_roundtrip() {
        for action in [
            AutomaticStartAction::Nothing,
            AutomaticStartAction::StartIfRunning,
            AutomaticStartAction::AlwaysStart,
        ] {
            assert_eq!(AutomaticStartAction::from_value(action.to_value()), action);
        }
    }

    // ========== AutomaticStopAction Tests ==========

    #[test]
    fn test_automatic_stop_action_to_value() {
        assert_eq!(AutomaticStopAction::TurnOff.to_value(), 0);
        assert_eq!(AutomaticStopAction::Save.to_value(), 1);
        assert_eq!(AutomaticStopAction::Shutdown.to_value(), 2);
    }

    #[test]
    fn test_automatic_stop_action_from_value() {
        assert_eq!(AutomaticStopAction::from_value(0), AutomaticStopAction::TurnOff);
        assert_eq!(AutomaticStopAction::from_value(1), AutomaticStopAction::Save);
        assert_eq!(AutomaticStopAction::from_value(2), AutomaticStopAction::Shutdown);
        assert_eq!(AutomaticStopAction::from_value(99), AutomaticStopAction::Save);
    }

    #[test]
    fn test_automatic_stop_action_default() {
        assert_eq!(AutomaticStopAction::default(), AutomaticStopAction::Save);
    }

    #[test]
    fn test_automatic_stop_action_roundtrip() {
        for action in [
            AutomaticStopAction::TurnOff,
            AutomaticStopAction::Save,
            AutomaticStopAction::Shutdown,
        ] {
            assert_eq!(AutomaticStopAction::from_value(action.to_value()), action);
        }
    }

    // ========== RequestedState Tests ==========

    #[test]
    fn test_requested_state_values() {
        assert_eq!(RequestedState::Running as u16, 2);
        assert_eq!(RequestedState::Off as u16, 3);
        assert_eq!(RequestedState::Paused as u16, 32768);
        assert_eq!(RequestedState::Saved as u16, 32769);
        assert_eq!(RequestedState::Reset as u16, 11);
    }

    // ========== VmState to_error Tests ==========

    #[test]
    fn test_vm_state_to_error() {
        assert_eq!(VmState::Unknown.to_error(), VmStateError::Unknown);
        assert_eq!(VmState::Running.to_error(), VmStateError::Running);
        assert_eq!(VmState::Off.to_error(), VmStateError::Off);
        assert_eq!(VmState::ShuttingDown.to_error(), VmStateError::ShuttingDown);
        assert_eq!(VmState::Paused.to_error(), VmStateError::Paused);
        assert_eq!(VmState::Suspended.to_error(), VmStateError::Suspended);
        assert_eq!(VmState::Starting.to_error(), VmStateError::Starting);
        assert_eq!(VmState::Stopping.to_error(), VmStateError::Stopping);
        // Other states map to VmStateError::Other
        assert_eq!(VmState::Disabled.to_error(), VmStateError::Other(6));
    }
}
