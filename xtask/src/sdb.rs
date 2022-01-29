// Stick
// Copyright © 2017-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt::Write;

const LINUX: &str = "./sdb/linux/";
const _MACOS: &str = "./sdb/macos/";
const _WINDOWS: &str = "./sdb/windows/";
const _WEB: &str = "./sdb/web/";
const _USB: &str = "./sdb/usb/";

const SDL: &str = "./gcdb/gamecontrollerdb.txt";

#[derive(Deserialize)]
struct Map {
    name: String,
    r#type: String,
    remap: HashMap<String, toml::value::Value>,
}

fn name_to_hex(name: &str) -> &str {
    match name {
        "None" => "00",
        "Exit" => "01",
        "ActionA" => "02",
        "ActionB" => "03",
        "ActionC" => "04",
        "ActionH" => "05",
        "ActionV" => "06",
        "ActionD" => "07",
        "MenuL" => "08",
        "MenuR" => "09",
        "Joy" => "0A",
        "Cam" => "0B",
        "BumperL" => "0C",
        "BumperR" => "0D",
        "TriggerL" => "0E",
        "TriggerR" => "0F",
        "Up" => "10",
        "Down" => "11",
        "Left" => "12",
        "Right" => "13",
        "HatUp" => "14",
        "HatDown" => "15",
        "HatLeft" => "16",
        "HatRight" => "17",
        "MicUp" => "18",
        "MicDown" => "19",
        "MicLeft" => "1A",
        "MicRight" => "1B",
        "PovUp" => "1C",
        "PovDown" => "1D",
        "PovLeft" => "1E",
        "PovRight" => "1F",
        "JoyX" => "20",
        "JoyY" => "21",
        "JoyZ" => "22",
        "CamX" => "23",
        "CamY" => "24",
        "CamZ" => "25",
        "Slew" => "26",
        "Throttle" => "27",
        "ThrottleL" => "28",
        "ThrottleR" => "29",
        "Volume" => "2A",
        "Wheel" => "2B",
        "Rudder" => "2C",
        "Gas" => "2D",
        "Brake" => "2E",
        "MicPush" => "2F",
        "Trigger" => "30",
        "Bumper" => "31",
        "ActionL" => "32",
        "ActionM" => "33",
        "ActionR" => "34",
        "Pinky" => "35",
        "PinkyForward" => "36",
        "PinkyBackward" => "37",
        "FlapsUp" => "38",
        "FlapsDown" => "39",
        "BoatForward" => "3A",
        "BoatBackward" => "3B",
        "AutopilotPath" => "3C",
        "AutopilotAlt" => "3D",
        "EngineMotorL" => "3E",
        "EngineMotorR" => "3F",
        "EngineFuelFlowL" => "40",
        "EngineFuelFlowR" => "41",
        "EngineIgnitionL" => "42",
        "EngineIgnitionR" => "43",
        "SpeedbrakeBackward" => "44",
        "SpeedbrakeForward" => "45",
        "ChinaBackward" => "46",
        "ChinaForward" => "47",
        "Apu" => "48",
        "RadarAltimeter" => "49",
        "LandingGearSilence" => "4A",
        "Eac" => "4B",
        "AutopilotToggle" => "4C",
        "ThrottleButton" => "4D",
        "MouseX" => "4E",
        "MouseY" => "4F",
        "Mouse" => "50",
        "PaddleLeft" => "51",
        "PaddleRight" => "52",
        "PinkyLeft" => "53",
        "PinkyRight" => "54",
        "Context" => "55",
        "Dpi" => "56",
        "ScrollX" => "57",
        "ScrollY" => "58",
        "Scroll" => "59",
        "TrimUp" => "5A",
        "TrimDown" => "5B",
        "TrimLeft" => "5C",
        "TrimRight" => "5D",
        "ActionWheelX" => "5E",
        "ActionWheelY" => "5F",
        _unknown => panic!("Unknown: {}", _unknown),
    }
}

pub(super) fn main() {
    let mut out = String::new();

    println!("Loading Linux TOML Controller Mappings…");
    for file in std::fs::read_dir(LINUX)
        .expect("Missing database")
        .flatten()
    {
        let path = file.path();
        let file = std::fs::read_to_string(&path).expect("Open file failed");
        let file: Map = toml::from_str(&file).unwrap();

        // ID of Controller
        out.push_str(
            &path.as_path().file_name().unwrap().to_str().unwrap()[..16],
        );

        // Name of Controller.
        out.push_str(&file.name);
        out.push('\t');

        // Type of controller
        let ctlr_type = match file.r#type.as_str() {
            "xbox" => 'x',
            "playstation" => 'p',
            "nintendo" => 'n',
            "gamepad" => 'g',
            "flight" => 'f',
            _type => panic!("Unknown type: {}", _type),
        };
        out.push(ctlr_type);

        // Add remappings
        let mut kv = Vec::new();
        for (key, value) in file.remap {
            kv.push((key, value));
        }
        kv.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
        for (key, value) in kv {
            if let Ok(number) = key.parse::<u8>() {
                write!(&mut out, "{:02X}", number | 0x80).unwrap();
            } else {
                out.push_str(name_to_hex(key.as_str()));
            }
            match value {
                toml::value::Value::String(event) => {
                    out.push_str(name_to_hex(event.as_str()));
                    out.push(';');
                }
                toml::value::Value::Table(table) => {
                    if let Some(event) = table.get("event") {
                        out.push_str(name_to_hex(event.as_str().unwrap()));
                    } else {
                        out.push_str(name_to_hex("None"));
                    }
                    if let Some(max) = table.get("max") {
                        let max = max.as_integer().unwrap();
                        out.push('a');
                        write!(&mut out, "{}", max).unwrap();
                    }
                    if let Some(min) = table.get("min") {
                        let min = min.as_integer().unwrap();
                        out.push('i');
                        write!(&mut out, "{}", min).unwrap();
                    }
                    if let Some(scale) = table.get("scale") {
                        let scale = scale.as_float().unwrap();
                        out.push('s');
                        write!(&mut out, "{}", scale).unwrap();
                    }
                    if let Some(deadzone) = table.get("deadzone") {
                        let deadzone = deadzone.as_float().unwrap();
                        out.push('d');
                        write!(&mut out, "{}", deadzone).unwrap();
                    }
                    out.push(';');
                }
                _map => panic!("invalid mapping: {:?}", _map),
            }
        }
        out.pop();

        // Newline to separate controllers.
        out.push('\n');
    }
    out.pop();

    std::fs::write("./stick/remap_linux.sdb", &out).unwrap();

    println!("Loading Linux (SDL) TOML Controller Mappings…");

    out.clear();

    for line in std::fs::read_to_string(SDL)
        .expect("Missing database")
        .lines()
    {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // ID of Controller
        let guid = line.get(0..32).unwrap();
        // Skip over emulated joysticks.
        if guid.get(2..8) != Some("000000")
            || guid.get(12..16) != Some("0000")
            || guid.get(20..24) != Some("0000")
            || guid.get(28..32) != Some("0000")
            || !line.contains("platform:Linux")
        {
            continue;
        }

        out.push_str(&guid.get(0..4).unwrap().to_uppercase());
        out.push_str(&guid.get(8..12).unwrap().to_uppercase());
        out.push_str(&guid.get(16..24).unwrap().to_uppercase());
        out.push_str(&guid.get(24..28).unwrap().to_uppercase());

        // Name of Controller.
        let mut iter = line[33..].split(',');
        let name = iter.next().expect("No name");
        out.push_str(&name);
        out.push('\t');

        // Type of controller
        out.push('w');

        // Add remappings
        for mapping in iter {
            if mapping.is_empty() {
                continue;
            }

            let mut mapping = mapping.split(':');
            let js_out = mapping.next().unwrap();
            let js_in = mapping.next().unwrap();

            let js_in = match js_in {
                "b0" => name_to_hex("Trigger"),
                "b1" => name_to_hex("ActionM"),
                "b2" => name_to_hex("Bumper"),
                "b3" => name_to_hex("ActionR"),
                "b4" => name_to_hex("ActionL"),
                "b5" => name_to_hex("Pinky"),
                "b6" => "80",
                "b7" => "81",
                "b8" => "82",
                "b9" => "83",
                "b10" => "84",
                "b11" => "85",
                "b12" => "86",
                "b13" => "87",
                "b14" => "88",
                "b15" => "89",
                "b16" => name_to_hex("ActionA"),
                "b17" => name_to_hex("ActionB"),
                "b18" => name_to_hex("ActionC"),
                "b19" => name_to_hex("ActionV"),
                "b20" => name_to_hex("ActionH"),
                "b21" => name_to_hex("ActionD"),
                "b22" => name_to_hex("BumperL"),
                "b32" => continue, // Not a gamepad?
                "h0.1" => name_to_hex("PovUp"),
                "h0.2" => name_to_hex("PovRight"),
                "h0.4" => name_to_hex("PovDown"),
                "h0.8" => name_to_hex("PovLeft"),
                "a0" | "a0~" => name_to_hex("JoyX"),
                "a1" | "a1~" => name_to_hex("JoyY"),
                "a2" | "a2~" => name_to_hex("JoyZ"),
                "a3" | "a3~" => name_to_hex("CamX"),
                "a4" | "a4~" => name_to_hex("CamY"),
                "a5" | "a5~" => name_to_hex("CamZ"),
                "a6" | "a6~" => name_to_hex("Throttle"),
                "a7" | "a7~" => name_to_hex("Rudder"),
                "a8" | "a8~" => name_to_hex("Wheel"),
                "a9" | "a9~" => name_to_hex("Gas"),
                "a10" | "a10~" => name_to_hex("Brake"),
                "a11" | "a11~" => name_to_hex("Slew"),
                "a12" => name_to_hex("ThrottleL"),
                "a13" => name_to_hex("ThrottleR"),
                "a14" => name_to_hex("ScrollX"),
                "+a0" | "+a1" | "+a2" | "+a3" | "+a4" | "+a5" | "-a0"
                | "-a1" | "-a2" | "-a3" | "-a4" | "-a5" => continue,
                "Linux" => continue,
                // ?
                "b122" => name_to_hex("Down"),
                "b119" => name_to_hex("Left"),
                "b120" => name_to_hex("Right"),
                "b117" => name_to_hex("Up"),
                "b161" => "8B",
                "b136" => continue,
                _in => panic!("Unknown input {}", _in),
            };

            let js_out = match js_out {
                "a" => "02",
                "b" => "03",
                "x" => "05",
                "y" => "06",
                "back" => "08",
                "start" => "09",
                "guide" => "01",
                "leftshoulder" => "0C",
                "lefttrigger" => "0E",
                "leftx" => "20",
                "lefty" => "21",
                "rightx" => "23",
                "righty" => "24",
                "rightshoulder" => "0D",
                "righttrigger" => "0F",
                "leftstick" => "0A",
                "rightstick" => "0B",
                "dpleft" => "12",
                "dpright" => "13",
                "dpup" => "10",
                "dpdown" => "11",
                "misc1" => "81",
                "+leftx" | "-leftx" | "+lefty" | "-lefty" => continue,
                "paddle1" => "51",
                "paddle2" => "52",
                _out => panic!("Unknown output {}", _out),
            };

            out.push_str(js_in);
            out.push_str(js_out);
            // FIXME: Tweaks
            out.push(';');
        }
        out.pop();
        out.push('\n');
    }
    out.pop();

    std::fs::write("./stick/sdlgc_linux.sdb", out).unwrap();
}
