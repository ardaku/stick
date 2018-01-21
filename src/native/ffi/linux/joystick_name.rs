// Stick
// Copyright (c) 2017-2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/native/linux/joystick_name.rs

use std::str;

extern {
	fn ioctl(fd: i32, request: usize, v: *mut u8) -> i32;
}

pub fn joystick_name(fd: i32) -> String {
	let mut name = [0u8; 80];

	let error = unsafe {
		ioctl(fd, 0x80506a13, &mut name[0])
	} == -1;

	if error {
		return String::from("unknown");
	}

	String::from(str::from_utf8(&name[..]).unwrap_or("unknown"))
}
