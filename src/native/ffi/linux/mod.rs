// Stick
// Copyright (c) 2017-2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/native/linux/mod.rs

use std::ffi::CString;
use std::fs;

mod joystick_name;
mod joystick_poll_event;

extern {
	fn open(pathname: *const u8, flags: i32) -> i32;
	fn close(fd: i32) -> i32;
	fn fcntl(fd: i32, cmd: i32, v: i32) -> i32;
	fn ioctl(fd: i32, request: usize, v: *mut i16) -> i32;
}

pub struct Joystick { pub native: i32 }
impl Joystick {
	pub fn new() -> Joystick {
		// Find device
		let device = find_device();

		// Open device
		let joystick = if device.is_empty() {
			-1
		} else {
			open_joystick(&device)
		};

		// Setup device for asynchronous reads
		if joystick != -1 {
			joystick_async(joystick);
		}

		// Return
		Joystick { native: joystick }
	}

	pub fn get_id(&self) -> (i32, bool) {
		joystick_id(self.native)
	}

	pub fn is_plugged_in(&self) -> bool {
		self.native != -1
	}

	pub fn disconnect(&mut self) -> () {
		joystick_drop(self.native);
		self.native = -1;
	}

	pub fn name(&self) -> String {
		joystick_name::joystick_name(self.native)
	}

	pub fn poll_event(&self, state: &mut (Vec<f32>, Vec<bool>)) -> bool {
		joystick_poll_event::joystick_poll_event(self.native,
			&mut state.0, &mut state.1)
	}
}
impl Drop for Joystick {
	fn drop(&mut self) -> () {
		if self.native != -1 {
			joystick_drop(self.native);
		}
	}
}

// Find the evdev device.
fn find_device() -> String {
	let paths = fs::read_dir("/dev/input/by-id/").unwrap();

	for path in paths {
		let path_str = path.unwrap().path();
		let path_str = path_str.to_str().unwrap();

		// An evdev device.
		if path_str.ends_with("-event-joystick") {
			return path_str.to_string();
		}
	}

	return "".to_string();
}

// Open the evdev device.
fn open_joystick(name: &str) -> i32 {
	let file_name = CString::new(name).unwrap();

	unsafe {
		open(file_name.as_ptr() as *const _, 0)
	}
}

// Set up file descriptor for asynchronous reading.
fn joystick_async(fd: i32) -> () {
	let error = unsafe {
		fcntl(fd, 0x4, 0x800)
	} == -1;

	if error {
		panic!("Joystick unplugged 2!");
	}
}

// Get the joystick id.
fn joystick_id(fd: i32) -> (i32, bool) {
	let mut a = [0i16; 4];

	if unsafe { ioctl(fd, 0x80084502, &mut a[0]) } == -1 {
		return (0, true)
	}

	(((a[1] as i32) << 16) | (a[2] as i32), false)
}

// Disconnect the joystick.
fn joystick_drop(fd: i32) -> () {
	let failure = unsafe {
		close(fd) == -1
	};

	if failure {
		panic!("Failed to disconnect joystick.");
	}
}
