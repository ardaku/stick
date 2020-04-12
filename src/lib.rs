#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://libcala.github.io/stick/res/controller.png",
    html_favicon_url = "https://libcala.github.io/stick/res/controller.png"
)]

// New modules
mod event;
pub use event::Event;


mod devices;

pub use devices::{Btn, Device, Port, CONTROLLER_MAX};

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
