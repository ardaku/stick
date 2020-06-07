#[macro_use]
extern crate serde_derive;

use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use std::{env, fs, path::Path};

#[path = "./pad_db/format.rs"]
mod format;

fn generate_from_database() -> String {
    let mut ret = String::new();
    ret.push_str("pub(super) fn pad_desc(\n");
    ret.push_str("    bus: u16, vendor: u16, product: u16, ver: u16\n");
    ret.push_str(") -> &'static PadDescriptor\n");
    ret.push_str("{\n");
    ret.push_str("    match (bus, vendor, product, ver) {\n");
    let path = "./pad_db/pad/mapping";
    for dir_entry in fs::read_dir(path).unwrap() {
        let dir_entry = dir_entry.unwrap();
        let entry = dir_entry.path();
        let dir_entry = dir_entry.file_name().to_str().unwrap().to_string();
        assert!(dir_entry.ends_with(".toml") && dir_entry.len() == 21);
        let bus = dir_entry.get(0..4).unwrap();
        let vendor = dir_entry.get(4..8).unwrap();
        let product = dir_entry.get(8..12).unwrap();
        let ver = dir_entry.get(12..16).unwrap();
        ret.push_str("        (");
        if bus == "xxxx" {
            ret.push('_');
        } else {
            ret.push_str("0x");
            ret.push_str(bus);
        }
        ret.push_str(", ");
        if vendor == "xxxx" {
            ret.push('_');
        } else {
            ret.push_str("0x");
            ret.push_str(vendor);
        }
        ret.push_str(", ");
        if product == "xxxx" {
            ret.push('_');
        } else {
            ret.push_str("0x");
            ret.push_str(product);
        }
        ret.push_str(", ");
        if ver == "xxxx" {
            ret.push('_');
        } else {
            ret.push_str("0x");
            ret.push_str(ver);
        }
        ret.push_str(") => PadDescriptor {\n");
        let map: format::PadMapping = toml::from_slice(&fs::read(entry).unwrap()).unwrap();
        ret.push_str("            name: \"");
        ret.push_str(&map.name);
        ret.push_str("\",\n");
        ret.push_str("            buttons: &[\n");
        let mut tb = String::new();
        if let Some(buttons) = map.button {
            for format::Button { code, event } in buttons {
                if event.starts_with("Trigger") {
                    tb.push_str("                (&Event::");
                    tb.push_str(&event);
                    tb.push_str(", ");
                    tb.push_str(&code.to_string());
                    tb.push_str("),\n");
                } else {
                    ret.push_str("                (&Event::");
                    ret.push_str(&event);
                    ret.push_str(", ");
                    ret.push_str(&code.to_string());
                    ret.push_str("),\n");
                }
            }
        }
        ret.push_str("            ],\n");
        ret.push_str("            trigbtns: &[\n");
        ret.push_str(&tb);
        ret.push_str("            ],\n");
        ret.push_str("            axes: &[\n");
        if let Some(axes) = map.axis {
            for format::Axis { code, event, max } in axes {
                ret.push_str("                (&Event::");
                ret.push_str(&event);
                ret.push_str(", ");
                ret.push_str(&code.to_string());
                ret.push_str(", ");
                if let Some(max) = max {
                    ret.push_str("Some(");
                    ret.push_str(&max.to_string());
                    ret.push_str(")");
                } else {
                    ret.push_str("None");
                }
                ret.push_str("),\n");
            }
        }
        ret.push_str("            ],\n");
        ret.push_str("            triggers: &[\n");
        if let Some(triggers) = map.trigger {
            for format::Trigger { code, event, max } in triggers {
                ret.push_str("                (&Event::");
                ret.push_str(&event);
                ret.push_str(", ");
                ret.push_str(&code.to_string());
                ret.push_str(", ");
                if let Some(max) = max {
                    ret.push_str("Some(");
                    ret.push_str(&max.to_string());
                    ret.push_str(")");
                } else {
                    ret.push_str("None");
                }
                ret.push_str("),\n");
            }
        }
        ret.push_str("            ],\n");
        ret.push_str("            three_ways: &[\n");
        if let Some(three_ways) = map.three_way {
            for format::ThreeWay { code, neg, pos } in three_ways {
                ret.push_str("                (&|neg, state| if neg { Event::");
                ret.push_str(&neg);
                ret.push_str("(state) } else { Event::");
                ret.push_str(&pos);
                ret.push_str("(state) }, ");
                ret.push_str(&code.to_string());
                ret.push_str("),\n");
            }
        }
        ret.push_str("            ],\n");
        ret.push_str("            wheels: &[\n");
        if let Some(wheels) = map.wheel {
            for format::Wheel { code, event } in wheels {
                ret.push_str("                (&Event::");
                ret.push_str(&event);
                ret.push_str(", ");
                ret.push_str(&code.to_string());
                ret.push_str("),\n");
            }
        }
        ret.push_str("            ],\n");
        ret.push_str("        },\n");
    }
    ret.push_str("    }\n");
    ret.push_str("}\n");
    ret
}

fn stop_needless_reruns(path: &str) {
    for dir_entry in fs::read_dir(path).unwrap() {
        let dir_entry = dir_entry.unwrap().path();
        let filename = dir_entry.to_str().unwrap();
        println!("cargo:rerun-if-changed={}", filename);
        if dir_entry.is_dir() {
            stop_needless_reruns(filename);
        }
    }
}

fn main() {
    stop_needless_reruns("./pad_db/");
    let output = generate_from_database();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("database.rs");
    fs::write(&dest_path, output).unwrap();
}
