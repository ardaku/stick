// Stick
// Copyright (c) 2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/input.rs

use std::fmt;

use super::Button;

/// Controller Input
#[derive(PartialEq, Copy, Clone)]
pub enum Input {
	/// Main joystick movement.
	Move(f32, f32),
	/// Camera / C joystick movement.
	Camera(f32, f32),
	/// Left Throttle movement.
	ThrottleL(f32),
	/// Right Throttle movement.
	ThrottleR(f32),
	/// Button Press
	Press(Button),
	/// Button Release
	Release(Button),
	/// Device Plugged-In
	PluggedIn,
	/// Device Un-Plugged
	UnPlugged,
}

impl fmt::Display for Input {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result<> {
		use Input::*;

		match *self {
			Move(x, y) => write!(f, "Move ({}, {})", x, y),
			Camera(x, y) => write!(f, "Camera ({}, {})", x, y),
			ThrottleL(x) => write!(f, "ThrottleL ({})", x),
			ThrottleR(x) => write!(f, "ThrottleR ({})", x),
			Press(x) => write!(f, "Press ({})", x),
			Release(x) => write!(f, "Release ({})", x),
			PluggedIn => write!(f, "Device Plugged-In"),
			UnPlugged =>  write!(f, "Device Un-Plugged"),
		}
	}
}
