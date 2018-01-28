// Stick
// Copyright (c) 2017-2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/lib.rs

//! A platform-agnostic Gamepad/Joystick library.

use std::fmt;

mod native;

/// A Gamepad/Joystick Button
#[derive(PartialEq, Copy, Clone)]
pub enum Button {
	/// Accept (A Button / Left Top Button - Missle / Circle)
	Accept,
	/// Cancel (B Button / Side Button / Cross)
	Cancel,
	/// Execute (X Button / Trigger / Triangle)
	Execute,
	/// Action (Y Button / Right Top Button / Square)
	Action,
	/// Left Button (0: L Trigger, 1: LZ / L bumper).  0 is
	/// farthest away from user, incrementing as buttons get closer.
	L(u8),
	/// Right Button (0: R Trigger, 1: Z / RZ / R Button). 0 is
	/// farthest away from user, incrementing as buttons get closer.
	R(u8),
	/// Pause Menu (Start Button)
	Menu,
	/// Show Controls (Guide on XBox, Select on PlayStation).  Use as
	/// alternative for Menu -> "Controls".
	Controls,
	/// Exit This Screen (Back on XBox).  Use as alternative for
	/// Menu -> "Quit" or Cancel, depending on situation.
	Exit,
	/// HAT/DPAD Up Button
	Up,
	/// HAT/DPAD Down Button
	Down,
	/// Hat/D-Pad left button
	Left,
	/// Hat/D-Pad right button.
	Right,
	/// Movement stick Push
	MoveStick,
	/// Camera stick Push
	CamStick,
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
			Button::Accept => write!(f, "Accept"),
			Button::Cancel => write!(f, "Cancel"),
			Button::Execute => write!(f, "Execute"),
			Button::Action => write!(f, "Action"),
			Button::L(a) => write!(f, "L-{}", a),
			Button::R(a) => write!(f, "R-{}", a),
			Button::Menu => write!(f, "Menu"),
			Button::Controls => write!(f, "Controls"),
			Button::Exit => write!(f, "Exit"),
			Button::Up => write!(f, "Up"),
			Button::Down => write!(f, "Down"),
			Button::Left => write!(f, "Left"),
			Button::Right => write!(f, "Right"),
			Button::MoveStick => write!(f, "Movement Stick Push"),
			Button::CamStick => write!(f, "Camera Stick Push"),
			Button::Unknown => write!(f, "Unknown"),
		}
	}
}

/// Joystick Input
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
}

impl ::std::fmt::Display for Input {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result<> {
		use Input::*;

		match *self {
			Move(x, y) => write!(f, "Move ({}, {})", x, y),
			Camera(x, y) => write!(f, "Camera ({}, {})", x, y),
			ThrottleL(x) => write!(f, "ThrottleL ({})", x),
			ThrottleR(x) => write!(f, "ThrottleR ({})", x),
			Press(x) => write!(f, "Press ({})", x),
			Release(x) => write!(f, "Release ({})", x),
		}
	}
}

use native::Joystick as NativeJoystick;

pub(crate) struct State {
	min: i32,
	max: i32,
	// Accept button
	accept: bool,
	// Cancel button
	cancel: bool,
	//
	execute: bool,
	trigger: bool,
	l: [bool; 32],
	r: [bool; 32],
	menu: bool,
	controls: bool,
	up: bool,
	down: bool,
	left: bool,
	right: bool,
	exit: bool,
	move_stick: bool,
	cam_stick: bool,
	move_xy: (f32, f32),
	cam_xy: (f32, f32),
	left_throttle: f32,
	right_throttle: f32,
}

const EMPTY_STATE: State = State {
	min: 0,
	max: 0,
	accept: false,
	cancel: false,
	execute: false,
	trigger: false,
	l: [false; 32],
	r: [false; 32],
	menu: false,
	controls: false,
	up: false,
	down: false,
	left: false,
	right: false,
	exit: false,
	move_stick: false,
	cam_stick: false,
	move_xy: (0.0, 0.0),
	cam_xy: (0.0, 0.0),
	left_throttle: 0.0,
	right_throttle: 0.0,
};

/// A structure to remap an input to a different joystick.
pub struct Remap {
	/// Which joystick should remap an input, 0 for all.
	pub id: i32,
	/// Remapper Function
	pub remapper: fn(Input) -> Input,
}

/// A USB Joystick Controller.
pub struct Joystick {
	joystick: NativeJoystick,
	oldstate: State,
	state: State,
	id: i32,
	remap: Vec<Remap>,
	input: Vec<Input>,
	reset: bool,
}

impl Joystick {
	/// Connect to a Joystick, with optional custom button/axis mapping.
	/// If custom mapping, always map A, B, C, D, MainX and MainY.
	pub fn new(remap: Vec<Remap>) -> Joystick {
		// TODO: mut
		let mut joystick = NativeJoystick::new();
		let (id, is_out) = joystick.get_id(0);
		let (min, max, is_out2) = joystick.get_abs(0);

		if is_out || is_out2 {
			return Joystick {
				joystick: joystick,
				oldstate: EMPTY_STATE,
				state: EMPTY_STATE,
				id: 0,
				remap,
				input: Vec::new(),
				reset: false,
			};
		}

		println!("New Joystick: {:x}", id);

		let mut oldstate = EMPTY_STATE;
		let mut state = EMPTY_STATE;

		oldstate.min = min;
		oldstate.max = max;
		state.min = min;
		state.max = max;

		let input = Vec::new();
		let reset = false;

		Joystick { joystick, oldstate, state, id, remap, input, reset }
	}

	// TODO move
	#[inline(always)]
	fn remap(&self, mut input: Input) -> Input {
		for i in &self.remap {
			if i.id == self.id || i.id == 0 {
				input = (i.remapper)(input);
			}
		}

		input
	}

	/// Poll Joystick Input
	pub fn update(&mut self) -> Option<Input> {
		if self.input.is_empty() == false {
			if let Some(input) = self.input.pop() {
				return Some(self.remap(input));
			} else {
				unreachable!();
			}
		} else if self.reset {
			self.reset = false;
			return None;
		}

		self.reset = true;

		for i in 0..self.not_plugged_in() {
			while self.joystick.poll_event(i, &mut self.state) { }
		}

		self.oldstate.left_throttle = check_axis(&mut self.input,
			(self.state.left_throttle, self.oldstate.left_throttle),
			Throttle::L);
		self.oldstate.right_throttle = check_axis(&mut self.input,
			(self.state.right_throttle,
			self.oldstate.right_throttle), Throttle::R);

		self.oldstate.move_xy = check_coord(&mut self.input,
			(self.state.move_xy.0, self.oldstate.move_xy.0),
			(self.state.move_xy.1, self.oldstate.move_xy.1),
			Throttle::MainX);

		self.oldstate.cam_xy = check_coord(&mut self.input,
			(self.state.cam_xy.0, self.oldstate.cam_xy.0),
			(self.state.cam_xy.1, self.oldstate.cam_xy.1),
			Throttle::PovX);

		// Button
		self.oldstate.accept = check_button(&mut self.input,
			(self.state.accept, self.oldstate.accept),
			Button::Accept);
		self.oldstate.cancel = check_button(&mut self.input,
			(self.state.cancel, self.oldstate.cancel),
			Button::Cancel);
		self.oldstate.execute = check_button(&mut self.input,
			(self.state.execute, self.oldstate.execute),
			Button::Execute);
		self.oldstate.trigger = check_button(&mut self.input,
			(self.state.trigger, self.oldstate.trigger),
			Button::Action);
		self.oldstate.menu = check_button(&mut self.input,
			(self.state.menu, self.oldstate.menu),
			Button::Menu);
		self.oldstate.left = check_button(&mut self.input,
			(self.state.left, self.oldstate.left),
			Button::Left);
		self.oldstate.right = check_button(&mut self.input,
			(self.state.right, self.oldstate.right),
			Button::Right);
		self.oldstate.up = check_button(&mut self.input,
			(self.state.up, self.oldstate.up),
			Button::Up);
		self.oldstate.down = check_button(&mut self.input,
			(self.state.down, self.oldstate.down),
			Button::Down);

		for i in 0..32 {
			self.oldstate.l[i] = check_button(&mut self.input,
				(self.state.l[i], self.oldstate.l[i]),
				Button::L(i as u8));
			self.oldstate.r[i] = check_button(&mut self.input,
				(self.state.r[i], self.oldstate.r[i]),
				Button::R(i as u8));
		}

		self.update()
	}

	fn not_plugged_in(&mut self) -> usize {
		// TODO: plug in when already some plugged in.
		if self.joystick.num_plugged_in() == 0 {
			self.joystick = NativeJoystick::new();
			self.oldstate = EMPTY_STATE;
			self.state = EMPTY_STATE;
			let (id, is_out) = self.joystick.get_id(0);
			let (min, max, is_out2) = self.joystick.get_abs(0);

			if is_out == false && is_out2 == false {
				self.state.min = min;
				self.state.max = max;
				self.id = id;

				println!("New Joystick: {:x}", id);
			}
		} else {
			for i in 0..self.joystick.num_plugged_in() {
				let (_, is_out) = self.joystick.get_id(i);

				if is_out {
					// TODO
					println!("Unplugged Joystick: ???");
					self.joystick.disconnect();
					break;
				}
			}
		}

		self.joystick.num_plugged_in()
	}
}

fn check_coord(input: &mut Vec<Input>, i: (f32, f32), j: (f32, f32),
	throttle: Throttle) -> (f32, f32)
{
	if i.0 != i.1 || j.0 != j.1 {
		input.push(match throttle {
			Throttle::MainX => Input::Move(i.0, j.0),
			Throttle::PovX => Input::Camera(i.0, j.0),
			_ => unreachable!(),
		});
	}

	(i.0, j.0)
}

fn check_axis(input: &mut Vec<Input>, i: (f32, f32), throttle: Throttle) -> f32
{
	if i.0 != i.1 {
		input.push(match throttle {
			Throttle::L => {
				Input::ThrottleL(i.0)
			},
			Throttle::R => {
				Input::ThrottleR(i.0)
			},
			_ => unreachable!(),
		});
	}

	i.0
}

fn check_button(input: &mut Vec<Input>, i: (bool, bool), button: Button) -> bool
{
	if i.0 != i.1 {
		input.push(match i.0 {
			false => Input::Release(button),
			true => Input::Press(button),
		});
	}

	i.0
}
