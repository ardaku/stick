// "stick" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use std::mem;
use std::ffi::CString;
use std::fs;

use State;

#[repr(C)]
struct TimeVal {
	tv_sec: isize,
	tv_usec: isize,
}

#[repr(C)]
struct Event {
	ev_time: TimeVal,
	ev_type: i16,
	ev_code: i16,
	ev_value: i32,
}

extern {
	fn open(pathname: *const u8, flags: i32) -> i32;
	fn close(fd: i32) -> i32;
	fn fcntl(fd: i32, cmd: i32, v: i32) -> i32;
	fn read(fd: i32, buf: *mut Event, count: usize) -> isize;
}

struct Device {
	name: Option<String>,
	fd: i32,
}

impl PartialEq for Device {
	fn eq(&self, other: &Device) -> bool {
		if let Some(ref name) = self.name {
			if let Some(ref name2) = other.name {
				name == name2
			} else {
				false
			}
		} else {
			false
		}
	}
}

pub struct NativeManager {
	devices: Vec<Device>,
}

impl NativeManager {
	pub fn new() -> NativeManager {
		NativeManager { devices: Vec::new() }
	}

	/// Do a search for controllers.  Returns number of controllers.
	pub fn search(&mut self) -> (usize, usize) {
		let devices = find_devices();

		// Add devices
		for mut i in devices {
			if self.devices.contains(&i) {
				continue;
			}

			open_joystick(&mut i);

			// Setup device for asynchronous reads
			if i.fd != -1 {
				joystick_async(i.fd);

				let index = self.add(i);
				return (self.devices.len(), index);
			}
		}

		(self.num_plugged_in(), ::std::usize::MAX)
	}

	pub fn get_id(&self, id: usize) -> (i32, bool) {
		if id >= self.devices.len() {
			(0, true)
		} else {
			let (_, a, b) = joystick_id(self.devices[id].fd);

			(a, b)
		}
	}

	pub fn get_abs(&self, id: usize) -> (i32, i32, bool) {
		if id >= self.devices.len() {
			(0, 0, true)
		} else {
			joystick_abs(self.devices[id].fd)
		}
	}

	pub fn get_fd(&self, id: usize) -> (i32, bool, bool) {
		let (_, unplug) = self.get_id(id);

		(self.devices[id].fd, unplug, self.devices[id].name == None)
	}

	pub fn num_plugged_in(&self) -> usize {
		self.devices.len()
	}

	pub fn disconnect(&mut self, fd: i32) -> () {
		for i in 0..self.devices.len() {
			if self.devices[i].fd == fd {
				joystick_drop(fd);
				self.devices[i].name = None;
				return;
			}
		}

		panic!("There was no fd of {}", fd);
	}

	pub(crate) fn poll_event(&self, i: usize, state: &mut State) {
		while joystick_poll_event(self.devices[i].fd, state) {}
	}

	fn add(&mut self, device: Device) -> usize {
		let mut r = 0;

		for i in &mut self.devices {
			if i.name == None {
				*i = device;
				return r;
			}

			r += 1;
		}

		self.devices.push(device);

		r
	}
}
impl Drop for NativeManager {
	fn drop(&mut self) -> () {
		while let Some(device) = self.devices.pop() {
			self.disconnect(device.fd);
		}
	}
}

// Find the evdev device.
fn find_devices() -> Vec<Device> {
	let mut rtn = Vec::new();
	let paths = fs::read_dir("/dev/input/by-id/").unwrap();

	for path in paths {
		let path_str = path.unwrap().path();
		let path_str = path_str.to_str().unwrap();

		// An evdev device.
		if path_str.ends_with("-event-joystick") {
			rtn.push(Device {
				name: Some(path_str.to_string()),
				fd: -1,
			});
		}
	}

	rtn
}

// Open the evdev device.
fn open_joystick(device: &mut Device) -> () {
	let file_name = CString::new(device.name.clone().unwrap()).unwrap();

	device.fd = unsafe {
		open(file_name.as_ptr() as *const _, 0)
	};
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
fn joystick_id(fd: i32) -> (i16, i32, bool) {
	let mut a = [0i16; 4];

	extern "C" {
		fn ioctl(fd: i32, request: usize, v: *mut i16) -> i32;
	}

	if unsafe { ioctl(fd, 0x80084502, &mut a[0]) } == -1 {
		return (0, 0, true)
	}

	(a[0], ((a[1] as i32) << 16) | (a[2] as i32), false)
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
	if unsafe { close(fd) == -1 } {
		panic!("Failed to disconnect joystick.");
	}
}

// Transform joystick coordinates.
fn transform(min: i32, max: i32, val: i32) -> f32 {
	let range = max - min;
	let value = val - min; // 0 - range
	let value = (value as f32) / (range as f32); // 0 - 1
	let value = (value * 2.0) - 1.0; // -1 to 1
	let value = (value * 100.0) as i32;

	// deadzone
	if value < 10 && value > -10 {
		0.0
	} else {
		(value as f32) / 100.0
	}
}

fn joystick_poll_event(fd: i32, state: &mut State) -> bool {
	let mut js = unsafe { mem::uninitialized() };

	let bytes = unsafe {
		read(fd, &mut js, mem::size_of::<Event>())
	};

	if bytes != (mem::size_of::<Event>() as isize) {
		return false;
	}

	match js.ev_type {
		// button press / release (key)
		0x01 => {
			let newstate = js.ev_value == 1;

			match js.ev_code - 0x120 {
				0 => state.execute = newstate,
				1 => state.accept = newstate,
				2 => state.cancel = newstate,
				3 => state.trigger = newstate,
				4 => state.l[0] = newstate,
				5 => state.r[0] = newstate,
				7 => state.r[1] = newstate,
				9 => state.menu = newstate,
				// ignore, duplicate of hat axis
				12 | 13 | 14 | 15 => {},
				a => println!("Unknown Button: {}", a),
			}
		}
		// axis move (abs)
		0x03 => {
			let value = transform(state.min, state.max,
				js.ev_value as i32);

			match js.ev_code {
				0 => state.move_xy.0 = value,
				1 => state.move_xy.1 = value,
				2 => state.cam_xy.1 = value,
				3 => state.left_throttle = value,
				4 => state.right_throttle = value,
				5 => state.cam_xy.0 = value,
				16 => {
					state.right = js.ev_value > 0;
					state.left = js.ev_value < 0;
				},
				17 => {
					state.up = js.ev_value < 0;
					state.down = js.ev_value > 0;
				},
				// precision axis, maybe implement eventually.
				40 => {},
				a => println!("Unknown Axis: {}", a),
			}
		}
		// ignore
		_ => {}
	}

	true
}
