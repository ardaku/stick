use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Button {
    code: u8,
    event: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Axis {
    code: u8,
    event: String,
    max: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Trigger {
    code: u8,
    event: String,
    max: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThreeWay {
    code: u8,
    neg: String,
    pos: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Wheel {
    code: u8,
    event: String,
}

/// A mapping for a specific pad
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PadMapping {
    // Name of the controller.
    name: String,
    // Type of the controller.
    r#type: String,
    // Buttons are simple on or off
    button: Option<Vec<Button>>,
    // Signed axes are "continuous" from min to max value
    axis: Option<Vec<Axis>>,
    // Triggers (Unsigned Axes) are "continuous" from 0 to 255
    trigger: Option<Vec<Trigger>>,
    // Three-Way switches are either on positive, on negative, or off
    three_way: Option<Vec<ThreeWay>>,
    // Signed relative axes are "continuous" from min to max value
    wheel: Option<Vec<Wheel>>,
}

fn relabel() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("a", "ActA");
    map.insert("b", "ActB");
    map.insert("x", "ActH");
    map.insert("y", "ActV");
    map.insert("back", "Prev");
    map.insert("start", "Next");
    map.insert("guide", "Cmd");
    map.insert("leftshoulder", "ShoulderL");
    map.insert("lefttrigger", "TriggerL");
    map.insert("leftx", "StickHor");
    map.insert("lefty", "StickVer");
    map.insert("rightx", "CStickHor");
    map.insert("righty", "CStickVer");
    map.insert("rightshoulder", "ShoulderR");
    map.insert("righttrigger", "TriggerR");
    map.insert("leftstick", "Stick");
    map.insert("rightstick", "CStick");
    map.insert("dpleft", "DirLeft");
    map.insert("dpright", "DirRight");
    map.insert("dpup", "DirUp");
    map.insert("dpdown", "DirDown");
    map
}

fn main() {
    let relabel = relabel();

    let gcdb = fs::read_to_string("SDL_GameControllerDB/gamecontrollerdb.txt")
        .expect("Couldn't find the database");
    let mut pad_map = HashMap::<String, (_, HashMap<_, _>)>::new();

    for line in gcdb.lines() {
        if line.starts_with("#") || line.is_empty() {
            continue;
        }

        let mut line = line.split(',').peekable();

        let guid = line.next().expect("malformed input (GUID)");
        let name = line.next().expect("malformed input (Name)");

        let mut map = HashMap::new();
        while let Some(mapping) = line.next() {
            if line.peek().is_none() {
                break;
            }
            let mut keyvalue = mapping.split(':');
            let key = keyvalue.next().expect("malformed input (Key)");
            let value = keyvalue.next().expect("malformed input (Value)");
            // swap key and value
            map.insert(value, key);
        }

        if map.remove("Linux").is_some() || map.remove("Mac OS X").is_some() {
            if guid == "xinput" {
                continue;
            }
            // Skip over emulated SDL joysticks.
            if guid.get(2..8) != Some("000000")
                || guid.get(12..16) != Some("0000")
                || guid.get(20..24) != Some("0000")
                || guid.get(28..32) != Some("0000")
            {
                continue;
            }
            assert_eq!(guid.len(), 32);
            let mut id = String::with_capacity(32);
            id.push_str(guid.get(0..4).unwrap());
            id.push_str(guid.get(8..8 + 4).unwrap());
            id.push_str(guid.get(16..16 + 4).unwrap());
            id.push_str(guid.get(24..24 + 4).unwrap());

            let id = u64::from_str_radix(&id, 16).unwrap();
            let id: String = format!("{:016X}", id);
            print!("{}", id);
            if let Some((old_name, old_map)) = pad_map.get_mut(&id) {
                print!(" (DUPLICATE!)");
                assert_eq!(old_name, &name);

                for (key, value) in map {
                    if let Some(val) = old_map.get(key) {
                        assert_eq!(val, &value);
                    } else {
                        old_map.insert(key, value);
                    }
                }
            } else {
                pad_map.insert(id, (name, map));
            }
            println!();
        }
    }

    for (id, (name, mut pad)) in pad_map {
        // Initialize pad mapping.
        let mut pad_mapping = PadMapping {
            name: name.to_string(),
            r#type: "Unknown".to_string(),
            button: None,
            axis: None,
            trigger: None,
            three_way: None,
            wheel: None,
        };

        if pad.remove("nes").is_some() {
            pad_mapping.r#type = "NES".to_string();
        }

        for (key, value) in pad {
            if key.starts_with("b") {
                let button = Button {
                    code: key.get(1..).unwrap().parse().unwrap(),
                    event: relabel.get(&value).unwrap().to_string(),
                };
                if let Some(ref mut btns) = pad_mapping.button {
                    btns.push(button);
                } else {
                    pad_mapping.button = Some(vec![button]);
                }
            } else if key.starts_with("a") {
                let key = if let Some(index) = key.find("~") {
                    key.get(..index).unwrap()
                } else {
                    key
                };
                let axis = Axis {
                    code: key.get(1..).unwrap().parse().unwrap(),
                    event: relabel.get(&value).unwrap().to_string(),
                    max: None,
                };
                if axis.event.starts_with("Trigger") {
                    let axis = Trigger {
                        code: axis.code,
                        event: axis.event,
                        max: axis.max,
                    };
                    if let Some(ref mut axes) = pad_mapping.trigger {
                        axes.push(axis);
                    } else {
                        pad_mapping.trigger = Some(vec![axis]);
                    }
                } else {
                    if let Some(ref mut axes) = pad_mapping.axis {
                        axes.push(axis);
                    } else {
                        pad_mapping.axis = Some(vec![axis]);
                    }
                }
            } else if key.starts_with("h0.") {
                let (code, neg) = match key.get(3..).unwrap() {
                    "1" =>
                    /* up */
                    {
                        (0x11, true)
                    }
                    "2" =>
                    /* right */
                    {
                        (0x10, false)
                    }
                    "4" =>
                    /* down */
                    {
                        (0x11, false)
                    }
                    "8" =>
                    /* left */
                    {
                        (0x10, true)
                    }
                    d => panic!("Unknown direction {}!", d),
                };
                if value.starts_with("+") || value.starts_with("-") {
                    let relabel =
                        relabel.get(&value.get(1..).unwrap()).unwrap();
                    let axis = Axis {
                        code,
                        event: relabel.to_string(),
                        max: None,
                    };
                    if let Some(ref mut axes) = pad_mapping.axis {
                        axes.push(axis);
                    } else {
                        pad_mapping.axis = Some(vec![axis]);
                    }
                } else {
                    let relabel = relabel.get(&value).unwrap();
                    let three_way = ThreeWay {
                        code,
                        neg: if neg {
                            relabel.to_string()
                        } else {
                            "".to_string()
                        },
                        pos: if neg {
                            "".to_string()
                        } else {
                            relabel.to_string()
                        },
                    };
                    if let Some(ref mut three_ways) = pad_mapping.three_way {
                        three_ways.push(three_way);
                    } else {
                        pad_mapping.three_way = Some(vec![three_way]);
                    }
                }
            } else if key.starts_with("+a") {
                // Positive axis
                let three_way = ThreeWay {
                    code: key.get(2..).unwrap().parse().unwrap(),
                    neg: "".to_string(),
                    pos: relabel.get(&value).unwrap().to_string(),
                };
                if let Some(ref mut three_ways) = pad_mapping.three_way {
                    three_ways.push(three_way);
                } else {
                    pad_mapping.three_way = Some(vec![three_way]);
                }
            } else if key.starts_with("-a") {
                // Negative axis
                let three_way = ThreeWay {
                    code: key.get(2..).unwrap().parse().unwrap(),
                    neg: relabel.get(&value).unwrap().to_string(),
                    pos: "".to_string(),
                };
                if let Some(ref mut three_ways) = pad_mapping.three_way {
                    three_ways.push(three_way);
                } else {
                    pad_mapping.three_way = Some(vec![three_way]);
                }
            } else {
                panic!("Unknown key: {}, value: {}", key, value);
            }
        }

        // Join Three-Ways
        let mut three_way_map = HashMap::new();
        if let Some(ref mut switches) = pad_mapping.three_way {
            while let Some(three_way) = switches.pop() {
                if let Some(old) = three_way_map
                    .insert(three_way.code, (three_way.neg, three_way.pos))
                {
                    let new = three_way_map.get_mut(&three_way.code).unwrap();
                    if !old.0.is_empty() {
                        assert!(new.0.is_empty());
                        new.0 = old.0;
                    }
                    if !old.1.is_empty() {
                        assert!(new.1.is_empty());
                        new.1 = old.1;
                    }
                }
            }
            for (code, (neg, pos)) in three_way_map {
                switches.push(ThreeWay { code, neg, pos });
            }
        }

        // Join Duplicated axes
        let mut axis_map = HashMap::new();
        if let Some(ref mut axes) = pad_mapping.axis {
            while let Some(axis) = axes.pop() {
                if let Some(old) = axis_map.insert(axis.event, axis.code) {
                    assert_eq!(old, axis.code);
                }
            }
            for (event, code) in axis_map {
                axes.push(Axis {
                    event,
                    code,
                    max: None,
                });
            }
        }

        // Write out to specification file.
        fs::write(
            &format!("Unix/{}.toml", id),
            toml::to_string(&pad_mapping).unwrap(),
        )
        .unwrap();
    }
}
