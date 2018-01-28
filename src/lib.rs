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

/// A USB Joystick Controller.
pub struct Joystick {
	map: Map,
	joystick: NativeJoystick,
	oldstate: State,
	state: State,
}

impl Joystick {
	/// Connect to a Joystick, with optional custom button/axis mapping.
	/// If custom mapping, always map A, B, C, D, MainX and MainY.
	pub fn new(map: Option<Map>) -> Joystick {
		// TODO: mut
		let mut joystick = NativeJoystick::new();
		let (id, is_out) = joystick.get_id(0);
		let (min, max, is_out2) = joystick.get_abs(0);

		if is_out || is_out2 {
			return Joystick {
				map: Map {
					buttons: Vec::new(),
					throttles: Vec::new()
				},
				joystick: joystick,
				oldstate: EMPTY_STATE,
				state: EMPTY_STATE,
			};
		}



		let map = if let Some(m) = map {
			m
		} else {
			Map::new(id)
		};

		println!("New Joystick: {:x}", id);

		let mut oldstate = EMPTY_STATE;
		let mut state = EMPTY_STATE;

		oldstate.min = min;
		oldstate.max = max;
		state.min = min;
		state.max = max;

		Joystick {
			map,
			joystick,
			oldstate,
			state,
		}
	}

	/// Poll Joystick Input
	pub fn update(&mut self, input: &mut Vec<Input>) -> () {
		for i in 0..self.not_plugged_in() {
			while self.joystick.poll_event(i, &mut self.state) { }
		}

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

/*		let mut js_main = (false, 0, 0);
		let mut js_pov = (false, 0, 0);

		for i in 0..self.map.throttles.len() {
			let j = self.map.throttle(i);

			match j {
				Throttle::L | Throttle::R => {},
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
		}*/

/*		// 
		if js_main.0 {
			self.check_coord(input, (js_main.1, Throttle::MainX),
				(js_main.2, Throttle::MainY));
		}

		// 
		if js_pov.0 {
			self.check_coord(input, (js_pov.1, Throttle::PovX),
				(js_pov.2, Throttle::PovY));
		}*/

		self.oldstate.left_throttle = check_axis(input,
			(self.state.left_throttle, self.oldstate.left_throttle),
			Throttle::L);
		self.oldstate.right_throttle = check_axis(input,
			(self.state.right_throttle,
			self.oldstate.right_throttle), Throttle::R);

		self.oldstate.move_xy = check_coord(input, (self.state.move_xy.0,
			self.oldstate.move_xy.0), (self.state.move_xy.1,
			self.oldstate.move_xy.1), Throttle::MainX);

		self.oldstate.cam_xy = check_coord(input, (self.state.cam_xy.0,
			self.oldstate.cam_xy.0), (self.state.cam_xy.1,
			self.oldstate.cam_xy.1), Throttle::PovX);

		// Button
		self.oldstate.accept = check_button(input,
			(self.state.accept, self.oldstate.accept),
			Button::Accept);
		self.oldstate.cancel = check_button(input,
			(self.state.cancel, self.oldstate.cancel),
			Button::Cancel);
		self.oldstate.execute = check_button(input,
			(self.state.execute, self.oldstate.execute),
			Button::Execute);
		self.oldstate.trigger = check_button(input,
			(self.state.trigger, self.oldstate.trigger),
			Button::Action);
		self.oldstate.menu = check_button(input,
			(self.state.menu, self.oldstate.menu),
			Button::Menu);
		self.oldstate.left = check_button(input,
			(self.state.left, self.oldstate.left),
			Button::Left);
		self.oldstate.right = check_button(input,
			(self.state.right, self.oldstate.right),
			Button::Right);
		self.oldstate.up = check_button(input,
			(self.state.up, self.oldstate.up),
			Button::Up);
		self.oldstate.down = check_button(input,
			(self.state.down, self.oldstate.down),
			Button::Down);

		for i in 0..32 {
			self.oldstate.l[i] = check_button(input,
				(self.state.l[i], self.oldstate.l[i]),
				Button::L(i as u8));
			self.oldstate.r[i] = check_button(input,
				(self.state.r[i], self.oldstate.r[i]),
				Button::R(i as u8));
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

	fn not_plugged_in(&mut self) -> usize {
		// TODO: plug in when already some plugged in.
		if self.joystick.num_plugged_in() == 0 {
			self.joystick = NativeJoystick::new();
			let (id, is_out) = self.joystick.get_id(0);

			if is_out == false {
				self.map = Map::new(id);

				println!("New Joystick: {:x}", id);
			}
		} else {
			for i in 0..self.joystick.num_plugged_in() {
				let (_, is_out) = self.joystick.get_id(i);

				if is_out {
					// TODO
					println!("Unplugged Joystick: ???");
					self.joystick.disconnect(i);
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
			Throttle::PovX => Input::Pov(i.0, j.0),
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
