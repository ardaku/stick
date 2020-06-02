/// Describes some gamepad
use std::fs::OpenOptions;
use std::io::{Read};
use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};
use toml::value::{ Table};

#[derive(Deserialize, Debug, Serialize)]
/// Describes some hardware joystick mapping
pub(crate) struct DeviceDescriptor {
    pub(crate) name: String,
    pub(crate) id: String,
    pub(crate) axes: Vec<AxisEvent>,
    pub(crate) buttons: Vec<ButtonEvent>,
    pub(crate) two_way: Vec<TwoWaySwitchEvent>,
    pub(crate) three_way: Vec<ThreeWaySwitchEvent>,
    pub(crate) triggers: Option<Vec<AxisEvent>>,
    pub(crate) hats: Option<Vec<HatEvent>>,
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

#[derive(Serialize, Deserialize, Debug)]
/// Hat.
pub(crate) struct HatEvent {
    /// Hat's name
    pub(crate) name: String,
    /// ID of north event.
    pub(crate) north: u32,
    /// ID of south event.
    pub(crate) south: u32,
    /// ID of west event.
    pub(crate) west: u32,
    // ID of east event.
    pub(crate) east: u32,
}

#[cfg(target_arch = "wasm32")]
const GAMEPAD_DB: &str = "./gamepad_db/wasm32/";
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
const GAMEPAD_DB: &str = "./gamepad_db/linux/";
#[cfg(all(not(target_arch = "wasm32"), target_os = "android"))]
const GAMEPAD_DB: &str = "./gamepad_db/android/";
#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
const GAMEPAD_DB: &str = "./gamepad_db/macos/";
#[cfg(all(not(target_arch = "wasm32"), target_os = "ios"))]
const GAMEPAD_DB: &str = "./gamepad_db/ios/";
#[cfg(all(not(target_arch = "wasm32"), target_os = "windows"))]
const GAMEPAD_DB: &str = "./gamepad_db/windows/";
#[cfg(all(not(target_arch = "wasm32"), 
    any(
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "bitrig",
        target_os = "openbsd",
        target_os = "netbsd"
    ))
)]
const GAMEPAD_DB: &str = "./gamepad_db/bsd/";
#[cfg(all(not(target_arch = "wasm32"), target_os = "fuchsia"))]
const GAMEPAD_DB: &str = "./gamepad_db/fuchsia/";
#[cfg(all(not(target_arch = "wasm32"), target_os = "redox"))]
const GAMEPAD_DB: &str = "./gamepad_db/redox/";
#[cfg(all(not(target_arch = "wasm32"), target_os = "none"))]
const GAMEPAD_DB: &str = "./gamepad_db/none/";

#[cfg(target_os = "dummy")]
fn generate_from_database() {
}

#[cfg(not(target_os = "dummy"))]
fn generate_from_database() {
}

fn main() {
    generate_from_database();
}
