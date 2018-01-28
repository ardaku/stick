// Stick
// Copyright (c) 2017-2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/native/linux/mod.rs

use std::mem;
use std::ffi::CString;
use std::fs;

use State;

mod joystick_poll_event;

extern {
	fn open(pathname: *const u8, flags: i32) -> i32;
	fn close(fd: i32) -> i32;
	fn fcntl(fd: i32, cmd: i32, v: i32) -> i32;
}

pub struct Joystick { pub fds: Vec<i32> }
impl Joystick {
	pub fn new() -> Joystick {
		// Find device
		let devices = find_devices();

		// Open device
		let mut fds = Vec::new();

		for i in devices {
			let fd = open_joystick(&i);

			// Setup device for asynchronous reads
			if fd != -1 {
				joystick_async(fd);
				fds.push(fd)
			}
		}

		// Return
		Joystick { fds }
	}

	pub fn get_id(&self, id: usize) -> (i32, bool) {
		joystick_id(self.fds[id])
	}

	pub fn get_abs(&self, id: usize) -> (i32, i32, bool) {
		joystick_abs(self.fds[id])
	}

	pub fn num_plugged_in(&self) -> usize {
		self.fds.len()
	}

	pub fn disconnect(&mut self, i: usize) -> () {
		joystick_drop(self.fds[i]);
		self.fds.remove(i);

/*		for i in self.fds {
			joystick_drop(i);
		}
		self.fds.clear();*/
	}

	pub(crate) fn poll_event(&self, i: usize, state: &mut State) -> bool {
		joystick_poll_event::joystick_poll_event(self.fds[i], state)
	}
}
impl Drop for Joystick {
	fn drop(&mut self) -> () {
		while self.num_plugged_in() != 0 {
			self.disconnect(0);
		}
	}
}

// Find the evdev device.
fn find_devices() -> Vec<String> {
	let mut rtn = Vec::new();
	let paths = fs::read_dir("/dev/input/by-id/").unwrap();

	for path in paths {
		let path_str = path.unwrap().path();
		let path_str = path_str.to_str().unwrap();

		// An evdev device.
		if path_str.ends_with("-event-joystick") {
			rtn.push(path_str.to_string());
		}
	}

	return rtn;
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

	extern "C" {
		fn ioctl(fd: i32, request: usize, v: *mut i16) -> i32;
	}

	if unsafe { ioctl(fd, 0x80084502, &mut a[0]) } == -1 {
		return (0, true)
	}

	(((a[1] as i32) << 16) | (a[2] as i32), false)
}

fn joystick_abs(fd: i32) -> (i32, i32, bool) {
	#[repr(C)]
	struct AbsInfo {
		value: i32,
		minimum: i32,
		maximum: i32,
		fuzz: i32,
		flat: i32,
		resolution: i32,
	}

	let mut a = unsafe { mem::uninitialized() };

	extern "C" {
		fn ioctl(fd: i32, request: usize, v: *mut AbsInfo) -> i32;
	}

	if unsafe { ioctl(fd, 0x80184540, &mut a) } == -1 {
		return (0, 0, true)
	}

	(a.minimum, a.maximum, false)
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
