// Stick
// Copyright (c) 2017-2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/native/linux/joystick_map.rs

extern {
	fn ioctl(fd: i32, request: usize, v: *mut i32) -> i32;
}

pub fn joystick_map(fd: i32) -> (u32, u32, bool) {
	let mut num_axis = 0;
	let mut num_buttons = 0;

	let a = unsafe { ioctl(fd, 0x80016a11, &mut num_axis) };
	let b = unsafe { ioctl(fd, 0x80016a12, &mut num_buttons) };

	if a == -1 || b == -1 {
		return (0, 0, true)
	}

	(num_axis as u32, num_buttons as u32, false)
}
