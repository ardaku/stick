// Copyright © 2017-2022 The Stick Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

use std::env;

fn main() {
    let target = &env::var("TARGET").unwrap();
    let target_family = &env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
    let target_os = &env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = &env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_vendor = &env::var("CARGO_CFG_TARGET_VENDOR").unwrap();
    let target_env = &env::var("CARGO_CFG_TARGET_ENV").unwrap();
    let unsupported =
        format!(
        "Target environment {} ({}, {}, {}, {}, {}) not suppported, please \
        consider opening an issue at https://github.com/libcala/stick/issues",
        target, target_family, target_os, target_arch, target_vendor, target_env
    );
    let mut out_file = env::var("OUT_DIR").unwrap();
    out_file.push_str("/unsupported.rs");
    std::fs::write(out_file, unsupported).unwrap();
}
