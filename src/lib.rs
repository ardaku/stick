// "stick" Source Code - Licensed under the MIT LICENSE (see /LICENSE)
//
//! A platform-agnostic joystick / gamepad / controller library for Rust.

mod input;
mod controller_manager;
mod remapper;

pub use input::Input;
pub use controller_manager::ControllerManager;
pub use remapper::Remapper;

#[cfg(target_os = "android")]
mod ffi { mod android; pub use self::android::NativeManager; }
#[cfg(all(not(target_os = "macos"), unix))]
mod ffi { mod linux; pub use self::linux::NativeManager; }
#[cfg(target_os = "macos")]
mod ffi { mod macos; pub use self::macos::NativeManager; }
#[cfg(target_os = "windows")]
mod ffi { mod windows; pub use self::windows::NativeManager; }

pub(crate) use self::ffi::NativeManager;
pub(crate) use controller_manager::State;
