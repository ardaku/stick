// Stick
// Copyright (c) 2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/controller_manager.rs

use super::Button;
use super::NativeManager;
use super::Input;
use super::Remapper;

#[derive(Copy, Clone)]
pub struct State {
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
	id: i32
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

	/// Poll Joystick Input
	pub fn update(&mut self) -> Option<(usize, Input)> {
		if let Some(input) = self.input.pop() {
			return Some(self.remap(input));
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

			while self.c_manager.poll_event(i,
				&mut self.controllers[i].state) { }

			// TODO: This code is garbage.  Fix it.  Preferably not
			// macros, but maybe is necesity.
			self.controllers[i].oldstate.left_throttle=check_axis(
				&mut self.input, i,
				(self.controllers[i].state.left_throttle,
				 self.controllers[i].oldstate.left_throttle),
				false);
			self.controllers[i].oldstate.right_throttle=check_axis(
				&mut self.input, i,
				(self.controllers[i].state.right_throttle,
				 self.controllers[i].oldstate.right_throttle),
				true);

			self.controllers[i].oldstate.move_xy = check_coord(
				&mut self.input, i,
				(self.controllers[i].state.move_xy.0,
				 self.controllers[i].oldstate.move_xy.0),
				(self.controllers[i].state.move_xy.1,
				 self.controllers[i].oldstate.move_xy.1),
				false);

			self.controllers[i].oldstate.cam_xy = check_coord(
				&mut self.input, i,
				(self.controllers[i].state.cam_xy.0,
				 self.controllers[i].oldstate.cam_xy.0),
				(self.controllers[i].state.cam_xy.1,
				 self.controllers[i].oldstate.cam_xy.1),
				true);

			// Button
			self.controllers[i].oldstate.accept = check_button(
				&mut self.input, i,
				(self.controllers[i].state.accept,
				 self.controllers[i].oldstate.accept),
				Button::Accept);
			self.controllers[i].oldstate.cancel = check_button(
				&mut self.input, i,
				(self.controllers[i].state.cancel,
				 self.controllers[i].oldstate.cancel),
				Button::Cancel);
			self.controllers[i].oldstate.execute = check_button(
				&mut self.input, i,
				(self.controllers[i].state.execute,
				 self.controllers[i].oldstate.execute),
				Button::Execute);
			self.controllers[i].oldstate.trigger = check_button(
				&mut self.input, i,
				(self.controllers[i].state.trigger,
				 self.controllers[i].oldstate.trigger),
				Button::Action);
			self.controllers[i].oldstate.menu = check_button(
				&mut self.input, i,
				(self.controllers[i].state.menu,
				 self.controllers[i].oldstate.menu),
				Button::Menu);
			self.controllers[i].oldstate.left = check_button(
				&mut self.input, i,
				(self.controllers[i].state.left,
				 self.controllers[i].oldstate.left),
				Button::Left);
			self.controllers[i].oldstate.right = check_button(
				&mut self.input, i,
				(self.controllers[i].state.right,
				 self.controllers[i].oldstate.right),
				Button::Right);
			self.controllers[i].oldstate.up = check_button(
				&mut self.input, i,
				(self.controllers[i].state.up,
				 self.controllers[i].oldstate.up),
				Button::Up);
			self.controllers[i].oldstate.down = check_button(
				&mut self.input, i,
				(self.controllers[i].state.down,
				 self.controllers[i].oldstate.down),
				Button::Down);
			self.controllers[i].oldstate.controls = check_button(
				&mut self.input, i,
				(self.controllers[i].state.controls,
				 self.controllers[i].oldstate.controls),
				Button::Controls);
			self.controllers[i].oldstate.move_stick = check_button(
				&mut self.input, i,
				(self.controllers[i].state.move_stick,
				 self.controllers[i].oldstate.move_stick),
				Button::MoveStick);
			self.controllers[i].oldstate.cam_stick = check_button(
				&mut self.input, i,
				(self.controllers[i].state.cam_stick,
				 self.controllers[i].oldstate.cam_stick),
				Button::CamStick);
			self.controllers[i].oldstate.exit = check_button(
				&mut self.input, i,
				(self.controllers[i].state.exit,
				 	self.controllers[i].oldstate.exit),
				Button::Exit);

			for b in 0..32 {
				self.controllers[i].oldstate.l[b] = check_button(
					&mut self.input, i,
					(self.controllers[i].state.l[b],
					 self.controllers[i].oldstate.l[b]),
					Button::L(b as u8));
				self.controllers[i].oldstate.r[b] = check_button(
					&mut self.input, i,
					(self.controllers[i].state.r[b],
					 self.controllers[i].oldstate.r[b]),
					Button::R(b as u8));
			}
		}

		self.update()
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

fn check_coord(input: &mut Vec<(usize, Input)>, id: usize, i: (f32, f32),
	j: (f32, f32), cam_stick: bool) -> (f32, f32)
{
	if i.0 != i.1 || j.0 != j.1 {
		input.push((id, match cam_stick {
			false => Input::Move(i.0, j.0),
			true => Input::Camera(i.0, j.0),
		}));
	}

	(i.0, j.0)
}

fn check_axis(input: &mut Vec<(usize, Input)>, id: usize, i: (f32, f32),
	rthrottle: bool) -> f32
{
	if i.0 != i.1 {
		input.push((id, match rthrottle {
			false => Input::ThrottleL(i.0),
			true => Input::ThrottleR(i.0),
		}));
	}

	i.0
}

fn check_button(input: &mut Vec<(usize, Input)>, id: usize, i: (bool, bool),
	button: Button) -> bool
{
	if i.0 != i.1 {
		input.push((id, match i.0 {
			false => Input::Release(button),
			true => Input::Press(button),
		}));
	}

	i.0
}
