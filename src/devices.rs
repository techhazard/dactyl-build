use List;
use UnimplementedType;

pub struct Device(UnimplementedType);

pub type Devices = List<Device>;

/// Contains all info about all connected devices
/// e.g USB, BT and maybe other
pub struct DeviceSettings {
    connected: Devices,
    active: Device,
}

pub fn default_settings() -> Option<DeviceSettings> {
    Some(DeviceSettings {
        connected: Devices::new(),
        active: Device(UnimplementedType!()),
    })
}

pub fn detect() -> Option<DeviceSettings> {
    magic!();
}
