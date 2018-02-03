// Stick
// Copyright (c) 2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// examples/jstest.rs

extern crate stick;

fn remapper(input: (usize, stick::Input)) -> (usize, stick::Input) {
	(input.0, match input.1 {
		stick::Input::Camera(_, y) => {
			stick::Input::ThrottleL(y)
		}
		a => a
	})
}

fn main() {
	let mut cm = stick::ControllerManager::new(vec![
		stick::Remapper::new(0x7b50316, remapper)
	]);

	loop {
		while let Some((j, i)) = cm.update() {
			println!("{}: {}", j, i);
		}
	}
}
