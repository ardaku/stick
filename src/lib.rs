// Stick
// Copyright (c) 2017-2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/lib.rs

//! A platform-agnostic Gamepad/Joystick library.

use std::fmt;

mod native;
mod map;

pub use self::map::Map;

/// A Gamepad/Joystick Button
#[derive(PartialEq, Copy, Clone)]
pub enum Button {
	/// A Button / Main Button (Missle)
	A,
	/// B Button / Secondary Button
	B,
	/// C (X) Button / Side Button
	C,
	/// D (Y) Button / Trigger
	D,
	/// L (left) Button
	L,
	/// R (right) Button
	R,
	/// Z Button
	Z,
	/// Start Button
	Start,
	/// DPAD Up Button
	Up,
	/// DPAD Down Button
	Down,
	/// DPAD Left Button
	Left,
	/// DPAD Right Button
	Right,
	/// Unknown Button
	Unknown,
}

/// The Throttle.
#[derive(PartialEq, Copy, Clone)]
pub enum Throttle {
	/// Main X
	MainX,
	/// Main Y
	MainY,
	/// POV X
	PovX,
	/// POV Y
	PovY,
	/// Left Throttle (L)
	L,
	/// Right (Precision) Throttle (R)
	R,
	/// Unknown Throttle
	Unknown,
}

impl fmt::Display for Button {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Button::A => write!(f, "A Button / Main Button (Missle)"),
			Button::B => write!(f, "B Button / Secondary Button"),
			Button::C => write!(f, "C (X) Button / Side Button"),
			Button::D => write!(f, "D (Y) Button / Trigger"),
			Button::L => write!(f, "L (left) Button"),
			Button::R => write!(f, "R (right) Button"),
			Button::Z => write!(f, "Z Button"),
			Button::Start => write!(f, "Start Button"),
			Button::Up => write!(f, "DPAD Up Button"),
			Button::Down => write!(f, "DPAD Down Button"),
			Button::Left => write!(f, "DPAD Left Button"),
			Button::Right => write!(f, "DPAD Right Button"),
			Button::Unknown => write!(f, "Unknown Button"),
		}
	}
}

/// Joystick Input
#[derive(PartialEq, Copy, Clone)]
pub enum Input {
	/// One of the following has happenned,
	///
	/// - The joystick has moved to a different position.
	/// - The C-pad has moved.
	/// - The on-screen joystick 1 has moved.
	Move(f32, f32),
	/// One of the following has happenned,
	///
	/// - The joystick's POV hat has moved.
	/// - The POV-Joystick has moved.
	/// - The on-screen joystick 2 has moved.
	Pov(f32, f32),
	/// One of the following has happenned,
	///
	/// - The joystick's throttle has moved.
	/// - The on-screen throttle has moved.
	ThrottleL(f32),
	/// One of the following has happenned,
	///
	/// - The joystick's throttle has moved.
	/// - The on-screen throttle has moved.
	ThrottleR(f32),
	/// One of the following has happenned,
	///
	/// - One of the joystick's buttons has been pressed.
	/// - An on-screen button has been pressed.
	Press(Button),
	/// One of the following has happenned,
	///
	/// - One of the joystick's buttons has been released.
	/// - An on-screen button has been released.
	Release(Button),
}

impl ::std::fmt::Display for Input {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result<> {
		use Input::*;

		match *self {
			Move(x, y) => write!(f, "Move ({}, {})", x, y),
			Pov(x, y) => write!(f, "Pov ({}, {})", x, y),
			ThrottleL(x) => write!(f, "ThrottleL ({})", x),
			ThrottleR(x) => write!(f, "ThrottleR ({})", x),
			Press(x) => write!(f, "Press ({})", x),
			Release(x) => write!(f, "Release ({})", x),
		}
	}
}

use native::Joystick as NativeJoystick;

/// A USB Joystick Controller.
pub struct Joystick {
	map: Map,
	joystick: NativeJoystick,
	oldstate: (Vec<f32>, Vec<bool>),
	state: (Vec<f32>, Vec<bool>),
	name: String,
}

impl Joystick {
	/// Connect to a Joystick, with optional custom button/axis mapping.
	/// If custom mapping, always map A, B, C, D, MainX and MainY.
	pub fn new(map: Option<Map>) -> Joystick {
		let joystick = NativeJoystick::create();
		let (n_axis, n_buttons, is_out) = joystick.map();

		if is_out {
			return Joystick {
				map: Map {
					buttons: Vec::new(),
					throttles: Vec::new()
				},
				joystick: joystick,
				oldstate: (Vec::new(), Vec::new()),
				state: (Vec::new(), Vec::new()),
				name: "".to_string(),
			};
		}

		let name = joystick.name();

		let mut axis = Vec::new();
		let mut buttons = Vec::new();

		axis.resize(n_axis, 0.0);
		buttons.resize(n_buttons, false);

		let map = if let Some(m) = map {
			m
		} else {
			Map::new(&name)
		};

		println!("New Joystick: {}", name);

		assert_eq!(n_buttons, map.buttons.len());
		assert_eq!(n_axis, map.throttles.len());

		Joystick {
			map,
			joystick,
			oldstate: (axis.clone(), buttons.clone()),
			state: (axis, buttons),
			name,
		}
	}

	/// Poll Joystick Input
	pub fn update(&mut self, input: &mut Vec<Input>) -> () {
		if self.not_plugged_in() {
			return
		}

		while self.joystick.poll_event(&mut self.state) { }

		// TODO: Create GUI widget to configure joystick.
		// Current configuration:
		//	Move - 0 -> 1 (Locked)
		//	Throttle - 2
		//	Pov - 3 -> 4
		//	Trigger(Down,Up) -> 0
		//	Button[0] -> 1
		//	Button[1] -> 2
		//	Button[2] -> 3

//		let js_axis_move = 0;
//		let js_axis_throttle = 2;
//		let js_axis_pov = 3;

//		self.check_axis(input, (js_axis_move, VIRTUAL_AXIS_MOVE));
//		self.check_axis(input, (js_axis_pov, VIRTUAL_AXIS_POV));
//		self.check_axis(input,(js_axis_throttle,VIRTUAL_AXIS_THROTTLE));

		let mut js_main = (false, 0, 0);
		let mut js_pov = (false, 0, 0);

		for i in 0..self.map.throttles.len() {
			let j = self.map.throttle(i);

			match j {
				Throttle::L | Throttle::R =>
					self.check_axis(input, (i, j)),
				Throttle::MainX => {
					js_main.0 = true;
					js_main.1 = i;
				},
				Throttle::MainY => {
					js_main.2 = i;
				},
				Throttle::PovX => {
					js_pov.0 = true;
					js_pov.1 = i;
				},
				Throttle::PovY => {
					js_pov.2 = i;
				}
				_ => {},
			}
		}

		if js_main.0 {
			self.check_coord(input, (js_main.1, Throttle::MainX),
				(js_main.2, Throttle::MainY));
		}

		if js_pov.0 {
			self.check_coord(input, (js_pov.1, Throttle::MainX),
				(js_pov.2, Throttle::MainY));
		}

		for i in 0..self.map.buttons.len() {
			let j = self.map.button(i);

			self.check_button(input, (i, j));
		}
	}

	/// Check to see if gamepad supports a specific input.
	///
	/// A, B, C, and D Buttons are always mapped.
	/// 
	/// 1 Joystick is always mapped.
	pub fn supports(&self, input: Input) -> bool {
		use Input::*;

		match input {
			Move(_, _) => self.map.has_throttle(Throttle::MainX)
				&& self.map.has_throttle(Throttle::MainY),
			Pov(_, _) => self.map.has_throttle(Throttle::PovX)
				&& self.map.has_throttle(Throttle::PovY),
			ThrottleL(_) => self.map.has_throttle(Throttle::L),
			ThrottleR(_) => self.map.has_throttle(Throttle::R),
			Press(x) | Release(x) => self.map.has_button(x),
		}
	}

	/// Get the name of the `Joystick`.
	pub fn name(&self) -> String {
		self.name.to_string()
	}

	fn check_button(&mut self, input: &mut Vec<Input>, i: (usize,Button)) {
		if self.state.1[i.0] != self.oldstate.1[i.0] {
			let value = self.state.1[i.0];

			self.oldstate.1[i.0] = value;

			input.push(match value {
				false => Input::Release(i.1),
				true => Input::Press(i.1),
			});
		}
	}

	fn check_coord(&mut self, input: &mut Vec<Input>, i: (usize,Throttle),
		j: (usize,Throttle))
	{
		if self.state.0[i.0] != self.oldstate.0[i.0] ||
			self.state.0[j.0] != self.oldstate.0[j.0]
		{
			let x = self.state.0[i.0];
			let y = self.state.0[j.0];

			self.oldstate.0[i.0] = x;
			self.oldstate.0[j.0] = y;

			input.push(match i.1 {
				Throttle::MainX => Input::Move(x, y),
				Throttle::PovX => Input::Pov(x, y),
				_ => unreachable!(),
			});
		}
	}

	fn check_axis(&mut self, input: &mut Vec<Input>, i: (usize,Throttle)) {
		if self.state.0[i.0] != self.oldstate.0[i.0] {
			let value = self.state.0[i.0];

			self.oldstate.0[i.0] = value;

			input.push(match i.1 {
				Throttle::L => {
					Input::ThrottleL(value)
				},
				Throttle::R => {
					Input::ThrottleR(value)
				},
				_ => unreachable!(),
			});
		}
	}

	fn not_plugged_in(&mut self) -> bool {
		if self.joystick.is_plugged_in() {
			let (_, _, is_out) = self.joystick.map();

			if is_out {
				println!("Unplugged Joystick: {}", self.name);
				self.joystick.disconnect();
			}

			is_out
		} else {
			self.joystick = NativeJoystick::create();
			self.name = self.joystick.name();
			let (n_axis, n_buttons, is_out) = self.joystick.map();

			if is_out == false {
				self.map = Map::new(&self.name);

				assert_eq!(n_buttons, self.map.buttons.len());
				assert_eq!(n_axis, self.map.throttles.len());

				self.state.0.resize(n_axis, 0.0);
				self.state.1.resize(n_buttons, false);
				self.oldstate.0.resize(n_axis, 0.0);
				self.oldstate.1.resize(n_buttons, false);

				println!("New Joystick: {}", self.name);
			}

			is_out
		}
	}
}
