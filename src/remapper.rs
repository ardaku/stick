// Stick
// Copyright (c) 2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/remapper.rs

use super::Input;

/// A structure to remap an input to a different joystick.
pub struct Remapper {
	pub(crate) id: i32,
	pub(crate) remapper: fn((usize, Input)) -> (usize, Input),
}

impl Remapper {
	/// Create a new remapping.  `id` is which joystick should remap an
	/// input, 0 for all.  `remapper` is the function to do the remapping.
	pub fn new(id: i32, remapper: fn((usize, Input)) -> (usize, Input))
		-> Self
	{
		Self { id, remapper }
	}
}
