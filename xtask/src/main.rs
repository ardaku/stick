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

use std::env;

mod sdb;

fn print_help() {
    eprintln!("Tasks:");
    eprintln!();
    eprintln!("--help          Print this help text");
    eprintln!("sdb             Generate stick & gcdb bytecode databases");
}

fn print_unknown(x: &str) {
    eprintln!("cargo xtask {} is an invalid command.", x);
    eprintln!();
    eprintln!("Run `cargo xtask` for help page.");
}

fn sdb() {
    sdb::main()
}

fn main() {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("sdb") => sdb(),
        None | Some("--help") => print_help(),
        Some(x) => print_unknown(x),
    }
}
