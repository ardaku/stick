// Stick
// Copyright (c) 2017-2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/native/linux/destroy.rs

extern {
	fn close(fd: i32) -> i32;
}

pub fn joystick(fd: i32) -> () {
	let failure = unsafe {
		close(fd) == -1
	};

	if failure {
		panic!("Failed to disconnect joystick.");
	}
}
