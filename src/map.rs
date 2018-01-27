// Stick
// Copyright (c) 2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/map.rs

use Button;
use Throttle;

// Names of different kinds of joysticks.
const GAMECUBE: &str = "GameCube";

/// A mapping for a joystick.
pub struct Map {
	pub buttons: Vec<Button>,
	pub throttles: Vec<Throttle>,
}

impl Map {
	/// Create a new map for the plugged in joystick.
	pub(crate) fn new(joystick_name: &str) -> Map {
		if joystick_name.contains(GAMECUBE) {
			Map {
				buttons: vec![
					Button::Execute, // 0
					Button::Accept, // 1
					Button::Cancel, // 2
					Button::Trigger, // 3
					Button::L(0), // 4
					Button::R(0), // 5
					Button::Unknown, // 6
					Button::R(1), // 7
					Button::Unknown, // 8
					Button::Menu, // 9
					Button::Unknown, // 10
					Button::Unknown, // 11
					Button::Up, // 12
					Button::Right, // 13
					Button::Down, // 14
					Button::Left, // 15
				],
				throttles: vec![
					Throttle::MainX, // 0
					Throttle::MainY, // 1
					Throttle::PovY, // 2
					Throttle::L, // 3
					Throttle::R, // 4
					Throttle::PovX, // 5
					Throttle::Unknown, // 6
					Throttle::Unknown, // 7
				],
			}
		// Flight Controller
		} else {
			Map {
				buttons: vec![
					Button::Trigger, // 0
					Button::Accept, // 1
					Button::Execute, // 2
					Button::Cancel, // 3
				],
				throttles: vec![
					Throttle::MainX, // 0
					Throttle::MainY, // 1
					Throttle::L, // 2
					Throttle::PovX, // 3
					Throttle::PovY, // 4
					Throttle::R, // 5
				],
			}
		}
	}

	/// Return true, if has the button.
	#[inline(always)]
	pub fn has_button(&self, button: Button) -> bool {
		self.buttons.contains(&button)
	}

	/// Return true, if has the throttle.
	#[inline(always)]
	pub fn has_throttle(&self, throttle: Throttle) -> bool {
		self.throttles.contains(&throttle)
	}

	/// Map button id to Button enum
	#[inline(always)]
	pub(crate) fn button(&self, button: usize) -> Button {
		self.buttons[button]
	}

	/// Map axis id to Throttle enum
	#[inline(always)]
	pub(crate) fn throttle(&self, throttle: usize) -> Throttle {
		self.throttles[throttle]
	}
}
