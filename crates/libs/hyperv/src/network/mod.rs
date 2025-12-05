mod adapter;
mod physical_adapter;
mod switch;

pub use adapter::{
    BandwidthSettings, NetworkAdapter, NetworkAdapterSettings, NetworkAdapterSettingsBuilder,
    PortMirroringMode,
};
pub use physical_adapter::{ConnectionStatus, PhysicalAdapter};
pub use switch::{
    BandwidthReservationMode, IovMode, SwitchType, VirtualSwitch, VirtualSwitchSettings,
    VirtualSwitchSettingsBuilder,
};
