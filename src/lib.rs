//! A platform-agnostic joystick / gamepad / controller library for Rust.

#![allow(clippy::cast_lossless)]
#![allow(clippy::new_without_default)]
#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://jeronaldaron.github.io/stick/res/icon.svg",
    html_favicon_url = "https://jeronaldaron.github.io/stick/res/icon.svg"
)]

mod devices;

pub use devices::{Btn, Device, Port};

#[cfg(target_os = "android")]
mod ffi {
    mod android;
    pub use self::android::*;
}
#[cfg(all(not(target_os = "macos"), unix))]
mod ffi {
    mod linux;
    pub use self::linux::*;
}
#[cfg(target_os = "macos")]
mod ffi {
    mod macos;
    pub use self::macos::*;
}
#[cfg(target_os = "windows")]
mod ffi {
    mod windows;
    pub use self::windows::*;
}

pub(crate) use self::ffi::NativeManager;
