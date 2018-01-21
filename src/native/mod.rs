// Stick
// Copyright (c) 2017-2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/native/mod.rs

#[cfg(target_os = "android")]
mod ffi {
	mod android;

	pub use self::android::Joystick;
}

#[cfg(all(not(target_os = "macos"), unix))]
mod ffi {
	mod linux;

	pub use self::linux::Joystick;
}

#[cfg(target_os = "macos")]
mod ffi {
	mod macos;

	pub use self::macos::Joystick;
}

#[cfg(target_os = "windows")]
mod ffi {
	mod windows;

	pub use self::windows::Joystick;
}

pub use self::ffi::Joystick;
