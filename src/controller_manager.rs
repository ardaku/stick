// controller_manager.rs -- Stick
// Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use super::NativeManager;
use super::Input;
use super::Remapper;

#[derive(Copy, Clone)]
pub(crate) struct State {
	pub min: i32,
	pub max: i32,
	pub accept: bool,
	pub cancel: bool,
	pub execute: bool,
	pub trigger: bool,
	pub l: [bool; 32],
	pub r: [bool; 32],
	pub menu: bool,
	pub controls: bool,
	pub up: bool,
	pub down: bool,
	pub left: bool,
	pub right: bool,
	pub exit: bool,
	pub move_stick: bool,
	pub cam_stick: bool,
	pub move_xy: (f32, f32),
	pub cam_xy: (f32, f32),
	pub left_throttle: f32,
	pub right_throttle: f32,
}

#[derive(Copy, Clone)]
struct Controller {
	oldstate: State,
	state: State,
	id: i32,
	move_xy: (f32, f32),
	cam_xy: (f32, f32),
	l_throttle: f32,
	r_throttle: f32,
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

const NEW_CONTROLLER: Controller = Controller {
	oldstate: EMPTY_STATE,
	state: EMPTY_STATE,
	id: 0,
	move_xy: (0.0, 0.0),
	cam_xy: (0.0, 0.0),
	l_throttle: 0.0,
	r_throttle: 0.0,
};

/// A Manager for Controllers.
pub struct ControllerManager {
	c_manager: NativeManager,
	controllers: Vec<Controller>,
	remap: Vec<Remapper>, // TODO: better, faster remapping.
	input: Vec<(usize, Input)>,
	reset: bool,
}

impl ControllerManager {
	/// Connect to a Joystick, with optional custom button/axis remapping.
	pub fn new(mut remap: Vec<Remapper>) -> ControllerManager {
		let c_manager = NativeManager::new();
		let controllers = Vec::new();
		let input = Vec::new();
		let reset = false;

		// default remappings
		remap.insert(0, include!("remapping/game_cube.rs"));
		remap.push(include!("remapping/default.rs"));

		ControllerManager {
			c_manager, controllers, remap, input, reset
		}
	}

	/// Poll Joystick Input.  Returns an `Option` for use in a `while let`.
	/// The tuple within the `Some` variant is controller id (starting at 0),
	/// followed by the input event for that controller.
	pub fn update(&mut self) -> Option<(usize, Input)> {
		if let Some(input) = self.input.pop() {
			let remapped = self.remap(input);

			if let Some(input) = self.change(remapped) {
				return Some(input);
			} else {
				return self.update();
			}
		} else if self.reset {
			self.reset = false;
			return None;
		}

		self.reset = true;

		let (device_count, added) = self.c_manager.search();

		if added != ::std::usize::MAX {
			self.controllers.resize(device_count, NEW_CONTROLLER);
		}

		for i in 0..device_count {
			let (fd, is_out, ne) = self.c_manager.get_fd(i);

			if ne { continue }
			if is_out {
				self.input.push((i, Input::UnPlugged(
					self.controllers[i].id)));
				self.c_manager.disconnect(fd);
				continue;
			}

			if added == i {
				let (min, max, _) = self.c_manager.get_abs(i);

				self.controllers[i].oldstate.min = min;
				self.controllers[i].oldstate.max = max;
				self.controllers[i].state.min = min;
				self.controllers[i].state.max = max;
				self.controllers[i].id =
					self.c_manager.get_id(i).0;

				self.input.push((i, Input::PluggedIn(
					self.controllers[i].id)))
			}

			self.c_manager.poll_event(i, &mut self.controllers[i].state);

			// TODO: This code is garbage.  Fix it.  Preferably not
			// macros, but maybe is necesity.
			check_axis(&mut self.input, i,
				self.controllers[i].state.left_throttle, false);
			check_axis(&mut self.input, i,
				self.controllers[i].state.right_throttle, true);

			if self.controllers[i].state.move_xy != (0.0, 0.0) {
				self.input.push((i, Input::Move(
					self.controllers[i].state.move_xy.0,
					self.controllers[i].state.move_xy.1))
				);
			}

			if self.controllers[i].state.cam_xy != (0.0, 0.0) {
				self.input.push((i, Input::Camera(
					self.controllers[i].state.cam_xy.0,
					self.controllers[i].state.cam_xy.1))
				);
			}

			// Button ( previous TODO is continued ... )
			if self.controllers[i].state.accept
				!= self.controllers[i].oldstate.accept
			{
				self.input.push((i, match self.controllers[i].state.accept {
					false => Input::Accept(None),
					true => Input::Accept(Some(true)),
				}));
				self.controllers[i].oldstate.accept =
					self.controllers[i].state.accept;
			} else if self.controllers[i].state.accept {
				self.input.push((i, Input::Accept(Some(false))))
			}

			if self.controllers[i].state.cancel
				!= self.controllers[i].oldstate.cancel
			{
				self.input.push((i, match self.controllers[i].state.cancel {
					false => Input::Cancel(None),
					true => Input::Cancel(Some(true)),
				}));
				self.controllers[i].oldstate.cancel =
					self.controllers[i].state.cancel;
			} else if self.controllers[i].state.cancel {
				self.input.push((i, Input::Cancel(Some(false))))
			}

			if self.controllers[i].state.execute
				!= self.controllers[i].oldstate.execute
			{
				self.input.push((i, match self.controllers[i].state.execute {
					false => Input::Execute(None),
					true => Input::Execute(Some(true)),
				}));
				self.controllers[i].oldstate.execute =
					self.controllers[i].state.execute;
			} else if self.controllers[i].state.execute {
				self.input.push((i, Input::Execute(Some(false))))
			}

			if self.controllers[i].state.trigger
				!= self.controllers[i].oldstate.trigger
			{
				self.input.push((i, match self.controllers[i].state.trigger {
					false => Input::Action(None),
					true => Input::Action(Some(true)),
				}));
				self.controllers[i].oldstate.trigger =
					self.controllers[i].state.trigger;
			} else if self.controllers[i].state.trigger {
				self.input.push((i, Input::Action(Some(false))))
			}

			if self.controllers[i].state.menu
				!= self.controllers[i].oldstate.menu
			{
				self.input.push((i, match self.controllers[i].state.menu {
					false => Input::Menu(None),
					true => Input::Menu(Some(true)),
				}));
				self.controllers[i].oldstate.menu =
					self.controllers[i].state.menu;
			} else if self.controllers[i].state.menu {
				self.input.push((i, Input::Menu(Some(false))))
			}

			if self.controllers[i].state.left
				!= self.controllers[i].oldstate.left
			{
				self.input.push((i, match self.controllers[i].state.left {
					false => Input::Left(None),
					true => Input::Left(Some(true)),
				}));
				self.controllers[i].oldstate.left =
					self.controllers[i].state.left;
			} else if self.controllers[i].state.left {
				self.input.push((i, Input::Left(Some(false))))
			}

			if self.controllers[i].state.right
				!= self.controllers[i].oldstate.right
			{
				self.input.push((i, match self.controllers[i].state.right {
					false => Input::Right(None),
					true => Input::Right(Some(true)),
				}));
				self.controllers[i].oldstate.right =
					self.controllers[i].state.right;
			} else if self.controllers[i].state.right {
				self.input.push((i, Input::Right(Some(false))))
			}

			if self.controllers[i].state.up
				!= self.controllers[i].oldstate.up
			{
				self.input.push((i, match self.controllers[i].state.up {
					false => Input::Up(None),
					true => Input::Up(Some(true)),
				}));
				self.controllers[i].oldstate.up =
					self.controllers[i].state.up;
			} else if self.controllers[i].state.up {
				self.input.push((i, Input::Up(Some(false))))
			}

			if self.controllers[i].state.down
				!= self.controllers[i].oldstate.down
			{
				self.input.push((i, match self.controllers[i].state.down {
					false => Input::Down(None),
					true => Input::Down(Some(true)),
				}));
				self.controllers[i].oldstate.down =
					self.controllers[i].state.down;
			} else if self.controllers[i].state.down {
				self.input.push((i, Input::Down(Some(false))))
			}

			if self.controllers[i].state.controls
				!= self.controllers[i].oldstate.controls
			{
				if self.controllers[i].state.controls {
					self.input.push((i, Input::Controls));
				}
				self.controllers[i].oldstate.controls =
					self.controllers[i].state.controls;
			}

			if self.controllers[i].state.move_stick
				!= self.controllers[i].oldstate.move_stick
			{
				self.input.push((i, match self.controllers[i].state.move_stick {
					false => Input::MoveStick(None),
					true => Input::MoveStick(Some(true)),
				}));
				self.controllers[i].oldstate.move_stick =
					self.controllers[i].state.move_stick;
			} else if self.controllers[i].state.move_stick {
				self.input.push((i, Input::MoveStick(Some(false))))
			}

			if self.controllers[i].state.cam_stick
				!= self.controllers[i].oldstate.cam_stick
			{
				self.input.push((i, match self.controllers[i].state.cam_stick {
					false => Input::CamStick(None),
					true => Input::CamStick(Some(true)),
				}));
				self.controllers[i].oldstate.cam_stick =
					self.controllers[i].state.cam_stick;
			} else if self.controllers[i].state.cam_stick {
				self.input.push((i, Input::CamStick(Some(false))))
			}

			if self.controllers[i].state.exit
				!= self.controllers[i].oldstate.exit
			{
				if self.controllers[i].state.exit {
					self.input.push((i, Input::Exit));
				}
				self.controllers[i].oldstate.exit =
					self.controllers[i].state.exit;
			}

			for b in 0..32 {
				if self.controllers[i].state.l[b]
					!= self.controllers[i].oldstate.l[b]
				{
					self.input.push((i, match self.controllers[i].state.l[b] {
						false => Input::L(b as u8, None),
						true => Input::L(b as u8, Some(true)),
					}));
					self.controllers[i].oldstate.l[b] =
						self.controllers[i].state.l[b];
				} else if self.controllers[i].state.l[b] {
					self.input.push((i, Input::L(b as u8, Some(false))))
				}

				if self.controllers[i].state.r[b]
					!= self.controllers[i].oldstate.r[b]
				{
					self.input.push((i, match self.controllers[i].state.r[b] {
						false => Input::R(b as u8, None),
						true => Input::R(b as u8, Some(true)),
					}));
					self.controllers[i].oldstate.r[b] =
						self.controllers[i].state.r[b];
				} else if self.controllers[i].state.r[b] {
					self.input.push((i, Input::R(b as u8, Some(false))))
				}
			}
		}

		self.update()
	}

	// TODO: remove this function, it's not needed anymore
	#[inline(always)]
	fn change(&mut self, input: (usize, Input)) -> Option<(usize, Input)> {
		use Input::*;

		match input.1 {
			Move(x, y) => self.controllers[input.0].move_xy = (x, y),
			Camera(x, y) => self.controllers[input.0].cam_xy = (x, y),

			ThrottleL(x) => if x !=
				self.controllers[input.0].l_throttle
			{
				self.controllers[input.0].l_throttle = x;
			} else { return None },

			ThrottleR(x) => if x !=
				self.controllers[input.0].r_throttle
			{
				self.controllers[input.0].r_throttle = x;
			} else { return None },

			_ => {},
		}

		Some(input)
	}

	#[inline(always)]
	fn remap(&self, mut input: (usize, Input)) -> (usize, Input) {
		for i in &self.remap {
			if i.id == self.controllers[input.0].id || i.id == 0 {
				input = (i.remapper)(input);
			}
		}

		input
	}
}

fn check_axis(input: &mut Vec<(usize, Input)>, id: usize, i: f32,
	rthrottle: bool)
{
	input.push((id, match rthrottle {
		false => Input::ThrottleL(i),
		true => Input::ThrottleR(i),
	}));
}
