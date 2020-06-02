/// Describes some gamepad
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};
use toml::value::Table;

#[derive(Deserialize, Debug, Serialize)]
/// Describes some hardware joystick mapping
struct DeviceDescriptor {
    name: String,
    id: String,
    axes: Vec<AxisEvent>,
    buttons: Vec<ButtonEvent>,
    two_way: Vec<TwoWaySwitchEvent>,
    three_way: Vec<ThreeWaySwitchEvent>,
    triggers: Option<Vec<AxisEvent>>,
    hats: Option<Vec<HatEvent>>,
}

impl DeviceDescriptor {
    /// Reads a device description file into a struct.
    fn from_toml(input: PathBuf) -> Self {
        let mut contents = String::new();
        let mut file = OpenOptions::new().read(true).open(input).unwrap();
        file.read_to_string(&mut contents).unwrap();
        let data: DeviceDescriptor = toml::from_str(&contents).unwrap();
        data
    }
}

#[derive(Debug, Deserialize, Serialize)]
/// Represents some named two-state event.
struct Event {
    /// Event ID.
    code: u32,
    /// Emitted event name.
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents an Axis.
struct AxisEvent {
    /// Name of event emitted when this axis changes.
    action: Event,
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents a button.
struct ButtonEvent {
    /// Event ID for this button
    code: u32,
    /// Name of event emitted when button is pressed.
    pressed_name: String,
    /// Name of event emitted when button is released.
    released_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents a two-way switch.
struct TwoWaySwitchEvent {
    // Event ID for this switch
    code: u32,
    // Name of event emitted when the switch is in its high "on" state.
    high: String,
    // Name of event emitted when the switch is in its neutral "off" state
    neutral: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// Three-way switch event
struct ThreeWaySwitchEvent {
    // Name of event emitted when the switch is in its neutral "middle" state
    neutral: String,
    // Name of event emitted when the switch is in its High "up" state
    high: Event,
    // Name of event emitted when the switch is in its Low "down" state
    low: Event,
}

#[derive(Serialize, Deserialize, Debug)]
/// Hat.
struct HatEvent {
    /// Hat's name
    name: String,
    /// ID of north event.
    north: u32,
    /// ID of south event.
    south: u32,
    /// ID of west event.
    west: u32,
    // ID of east event.
    east: u32,
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
#[cfg(all(
    not(target_arch = "wasm32"),
    any(
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "bitrig",
        target_os = "openbsd",
        target_os = "netbsd"
    )
))]
const GAMEPAD_DB: &str = "./gamepad_db/bsd/";
#[cfg(all(not(target_arch = "wasm32"), target_os = "fuchsia"))]
const GAMEPAD_DB: &str = "./gamepad_db/fuchsia/";
#[cfg(all(not(target_arch = "wasm32"), target_os = "redox"))]
const GAMEPAD_DB: &str = "./gamepad_db/redox/";
#[cfg(all(not(target_arch = "wasm32"), target_os = "none"))]
const GAMEPAD_DB: &str = "./gamepad_db/none/";

#[cfg(target_os = "dummy")]
fn generate_from_database() {}

#[cfg(not(target_os = "dummy"))]
fn generate_from_database() {}

fn main() {
    generate_from_database();
}
