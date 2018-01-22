// Stick
// Copyright (c) 2017-2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/native/windows/mod.rs

use std::mem;

type Tchar = i16;
const MAXPNAMELEN: usize = 32;
const MAX_JOYSTICKOEMVXDNAME: usize = 260;

#[repr(C)]
struct JoyCaps {
	w_mid: u16,
	w_pid: u16,
	szPname: [Tchar; MAXPNAMELEN],
	wXmin: u32,
	wXmax: u32,
	wYmin: u32,
	wYmax: u32,
	wZmin: u32,
	wZmax: u32,
	wNumButtons: u32,
	wPeriodMin: u32,
	wPeriodMax: u32,
	wRmin: u32,
	wRmax: u32,
	wUmin: u32,
	wUmax: u32,
	wVmin: u32,
	wVmax: u32,
	wCaps: u32,
	wMaxAxes: u32,
	wNumAxes: u32,
	wMaxButtons: u32,
	szRegKey: [Tchar; MAXPNAMELEN],
	szOEMVxD: [Tchar; MAX_JOYSTICKOEMVXDNAME],
}

#[repr(C)]
struct JoyInfo {
	dwSize: u32,
	dwFlags: u32,
	dwXpos: u32,
	dwYpos: u32,
	dwZpos: u32,
	dwRpos: u32,
	dwUpos: u32,
	dwVpos: u32,
	dwButtons: u32,
	dwButtonNumber: u32,
	dwPOV: u32,
	dwReserved1: u32,
	dwReserved2: u32,
}

// Link to the windows multimedia library.
#[link(name = "winmm")]
extern "system" {
	// Get number of joysticks that are plugged in.
	// fn joyGetNumDevs() -> u32;
	// 
	fn joyGetDevCapsW(joy_id: usize, caps: *mut JoyCaps, cbjc: u32) -> u32;
	//
	fn joyGetPosEx(joy_id: u32, pji: *mut JoyInfo) -> u32;

}

pub struct Joystick { joy_caps: Option<JoyCaps> }
impl Joystick {
	pub fn new() -> Joystick {
		Joystick { joy_caps: None }
	}

	// TODO: Return Name Here Too, so Joystick is unit struct & no mut ref.
	pub fn map(&mut self) -> (u32, u32, bool) {
		if self.is_plugged_in() {
			let mut joy_caps = unsafe { mem::uninitialized() };

			unsafe {
				joyGetDevCapsW(0, &mut joy_caps,
					mem::size_of::<JoyCaps>() as u32);
			}
			
			let n_axis = joy_caps.wNumAxes;
			let n_buttons = joy_caps.wNumButtons;
			
			self.joy_caps = Some(joy_caps);
			
			(n_axis, n_buttons, false)
		} else {
			(0, 0, true) // unplugged
		}
	}

	pub fn is_plugged_in(&self) -> bool {
		let mut pos = unsafe { mem::uninitialized() };
		
		(unsafe { joyGetPosEx(0, &mut pos) } == 0)
	}

	pub fn disconnect(&mut self) -> () {
		self.joy_caps = None;
	}

	pub fn name(&self) -> String {
//		joystick_name::joystick_name(self.native)
		"unknown".to_string()
	}

	pub fn poll_event(&self, _: &mut (Vec<f32>, Vec<bool>)) -> bool {
//		joystick_poll_event::joystick_poll_event(self.native,
//			&mut state.0, &mut state.1)
		false
	}
}
impl Drop for Joystick {
	fn drop(&mut self) -> () {
//		if self.native != -1 {
//			destroy::joystick(self.native);
//		}
	}
}
