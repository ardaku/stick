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

use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

mod format {
    include!("../../stick_db/format.rs");
}

fn main() -> io::Result<()> {
    // Get path to this script's directory from the enviroment
    let mut dir = PathBuf::from(env::var("RUST_SCRIPT_BASE_PATH").unwrap());
    // Add the folder structure to base path
    dir.push("ctlr_db");
    dir.push("ctlr");
    dir.push("list");
    // Printing for clarity
    println!("The directory is: {:?}", dir);
    if dir.is_dir() {
        order_dir(dir.as_path())
    } else {
        Err(io::Error::from(io::ErrorKind::NotFound))
    }
}

/// Loop through each folder in the directory
fn order_dir(dir: &Path) -> io::Result<()> {
    for file in Path::new(dir).read_dir()? {
        order_file(file?.path().as_path())?;
    }
    Ok(())
}

/// This reads a file into a string
/// Tries to parse it from toml into a Controller struct
/// Sorts the inner structs (button, axis, etc.) by code
/// Tries to parse to toml again
/// Write to file
fn order_file(file_path: &Path) -> io::Result<()> {
    println!("{:?}", file_path);
    let content = fs::read_to_string(file_path)?;
    let mut pad: format::CtlrMapping =
        toml::from_str(&content).expect("Error parsing file");
    sort_by_code(&mut pad);
    let toml = toml::to_string(&pad).expect("Error serializing file");
    fs::write(file_path, &toml)
}

fn sort_by_code(pad: &mut format::CtlrMapping) {
    sort_by_code_button(pad);
    sort_by_code_axis(pad);
    sort_by_code_trigger(pad);
    sort_by_code_three_way(pad);
    sort_by_code_wheel(pad);
}

fn sort_by_code_button(pad: &mut format::CtlrMapping) {
    if let Some(ref mut v) = pad.button {
        v.sort_by(|a, b| a.code.cmp(&b.code));
    }
}

fn sort_by_code_axis(pad: &mut format::CtlrMapping) {
    if let Some(ref mut v) = pad.axis {
        v.sort_by(|a, b| a.code.cmp(&b.code));
    }
}

fn sort_by_code_trigger(pad: &mut format::CtlrMapping) {
    if let Some(ref mut v) = pad.trigger {
        v.sort_by(|a, b| a.code.cmp(&b.code));
    }
}

fn sort_by_code_three_way(pad: &mut format::CtlrMapping) {
    if let Some(ref mut v) = pad.three_way {
        v.sort_by(|a, b| a.code.cmp(&b.code));
    }
}

fn sort_by_code_wheel(pad: &mut format::CtlrMapping) {
    if let Some(ref mut v) = pad.wheel {
        v.sort_by(|a, b| a.code.cmp(&b.code));
    }
}
