/// Describes some gamepad
use std::fs::OpenOptions;
use std::io::{Read};
use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};
use toml::value::{ Table};

#[derive(Deserialize, Debug, Serialize)]
/// Describes some hardware joystick mapping
pub(crate) struct DeviceDescriptor {
    /// Name of device.
    pub(crate) name: String,
    /// Hardware ID of device.
    pub(crate) id: String,
    /// Collection of axes belonging to this device.
    pub(crate) axes: Vec<AxisEvent>,
    /// Collection of triggers belonging to this device.
    pub(crate) triggers: Table,
    /// Collection of buttons belonging to this device.
    pub(crate) buttons: Vec<ButtonEvent>,
    /// Collection of two-way switches belonging to this device.
    pub(crate) two_way: Vec<TwoWaySwitchEvent>,
    /// Collection of three-way switches belonging to this device.
    pub(crate) three_way: Vec<ThreeWaySwitchEvent>,
}

impl DeviceDescriptor {
    #[allow(dead_code)]
    /// Reads a device description file into a struct.
    pub(crate) fn from_toml(input: PathBuf) -> Self {
        let mut contents = String::new();
        let mut file = OpenOptions::new().read(true).open(input).unwrap();
        file.read_to_string(&mut contents).unwrap();
        let data: DeviceDescriptor = toml::from_str(&contents).unwrap();
        data
    }
}

#[derive(Debug, Deserialize, Serialize)]
/// Represents some named two-state event.
pub(crate) struct Event {
    /// Event ID.
    pub(crate) code: u32,
    /// Emitted event name.
    pub(crate) name: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents an Axis.
pub(crate) struct AxisEvent {
    /// Name of event emitted when this axis changes.
    pub(crate) action: Event
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents a button.
pub(crate) struct ButtonEvent {
    /// Event ID for this button
    pub(crate) code: u32,
    /// Name of event emitted when button is pressed.
    pub(crate) pressed_name: String,
    /// Name of event emitted when button is released.
    pub(crate) released_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents a two-way switch.
pub(crate) struct TwoWaySwitchEvent {
    // Event ID for this switch
    pub(crate) code: u32,
    // Name of event emitted when the switch is in its high "on" state.
    pub(crate) high: String,
    // Name of event emitted when the switch is in its neutral "off" state
    pub(crate) neutral: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// Three-way switch event
pub(crate) struct ThreeWaySwitchEvent {
    // Name of event emitted when the switch is in its neutral "middle" state
    pub(crate) neutral: String,
    // Name of event emitted when the switch is in its High "up" state
    pub(crate) high: Event,
    // Name of event emitted when the switch is in its Low "down" state
    pub(crate) low: Event,
}
