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
