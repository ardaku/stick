// Stick
// Copyright (c) 2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// examples/jstest.rs

extern crate stick;

fn remapper(input: stick::Input) -> stick::Input {
	match input {
		stick::Input::Camera(_, y) => {
			stick::Input::ThrottleL(y)
		}
		a => a
	}
}

fn main() {
	let mut joystick = stick::Joystick::new(vec![
		stick::Remap {
			id: 0x7b50316,
			remapper,
		}
	]);

	loop {
		while let Some(i) = joystick.update() {
			println!("{}", i);
		}
	}
}
