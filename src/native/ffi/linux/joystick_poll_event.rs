// Stick
// Copyright (c) 2017-2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/native/linux/joystick_poll_event.rs

use State;
use std::mem;

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
	fn read(fd: i32, buf: *mut Event, count: usize) -> isize;
}

pub(crate) fn joystick_poll_event(fd: i32, state: &mut State) -> bool {
	let mut js = unsafe { mem::uninitialized() };

	let bytes = unsafe {
		read(fd, &mut js, mem::size_of::<Event>())
	};

	if bytes != (mem::size_of::<Event>() as isize) {
		return false;
	}

/*	if js.ev_type != 0 {
		println!("type {}, code {}, value {}", js.ev_type, js.ev_code,
			js.ev_value);
	}*/

	match js.ev_type {
		// button press / release (key)
		0x01 => {
			let newstate = js.ev_value == 1;
//			println!("Button {:x} {}", js.ev_code - 0x120, js.ev_value);

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
			let int_v = js.ev_value as i32;
			let range = state.max - state.min;
			// -12750 to 12750
			let value = (int_v - state.min - 128) * 100 + 50;
			// -1 to 1
			let value = ((value / range) as f32) / 50.0;

			match js.ev_code {
				0 => state.move_xy.0 = value,
				1 => state.move_xy.1 = value,
				2 => state.cam_xy.1 = value,
				3 => state.left_throttle = value,
				4 => state.right_throttle = value,
				5 => state.cam_xy.0 = value,
				16 => {
					state.right = int_v > 0;
					state.left = int_v < 0;
				},
				17 => {
					state.up = int_v < 0;
					state.down = int_v > 0;
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
