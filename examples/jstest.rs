// Stick
// Copyright (c) 2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// examples/jstest.rs

extern crate stick;

use stick::{ Joystick };

fn main() {
	let mut joystick = Joystick::new(None);
	let mut input_buffer = Vec::new();

	loop {
		joystick.update(&mut input_buffer);

		for i in input_buffer.iter() {
			println!("{}", i);
		}

		input_buffer.clear();
	}
}
