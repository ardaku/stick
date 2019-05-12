//! A platform-agnostic joystick / gamepad / controller library for Rust.

#![allow(clippy::cast_lossless)]
#![allow(clippy::new_without_default)]

mod devices;

pub use devices::{Btn, Device, Port};

#[cfg(target_os = "android")]
mod ffi {
    mod android;
    pub use self::android::NativeManager;
}
#[cfg(all(not(target_os = "macos"), unix))]
mod ffi {
    mod linux;
    pub use self::linux::NativeManager;
}
#[cfg(target_os = "macos")]
mod ffi {
    mod macos;
    pub use self::macos::NativeManager;
}
#[cfg(target_os = "windows")]
mod ffi {
    mod windows;
    pub use self::windows::NativeManager;
}

pub(crate) use self::ffi::NativeManager;
