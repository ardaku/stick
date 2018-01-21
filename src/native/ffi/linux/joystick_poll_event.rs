// Stick
// Copyright (c) 2017-2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/native/linux/joystick_poll_event.rs

#[repr(C)]
struct JsEvent {
	time: u32,	// 4
	value: i16,	// 2
	event_type: u8,	// 1
	number: u8,	// 1
}

extern {
	fn read(fd: i32, buf: *mut JsEvent, count: usize) -> isize;
}

/*fn joystick_merge_motion(input: &mut Vec<Input>, is_y: bool, xy: f32) -> () {
	if is_y {
		for i in 0..input.len() {
			match input[i] {
				Input::JoystickMove(x, _) => {
					input[i] = Input::JoystickMove(x, xy);
					return;
				},
				_ => {},
			}
		}
		input.push(Input::JoystickMove(0.0, xy));
	} else {
		for i in 0..input.len() {
			match input[i] {
				Input::JoystickMove(_, y) => {
					input[i] = Input::JoystickMove(xy, y);
					return;
				},
				_ => {},
			}
		}
		input.push(Input::JoystickMove(xy, 0.0));
	}
}*/

pub fn joystick_poll_event(fd: i32, axis: &mut Vec<f32>, button: &mut Vec<bool>)
	-> bool
{
	let mut js = JsEvent { time: 0, value: 0, event_type: 0, number: 0 };

	let bytes = unsafe {
		read(fd, &mut js, 8)
	};

	if bytes != 8 {
		return false;
	}

	match js.event_type & (!0x80) {
		0x01 => {
			button[js.number as usize] = js.value == 1;
//			if js.value == 1 {
//				input.push(Input::JoystickButtonDown(js.number));
//			} else {
//				input.push(Input::JoystickButtonUp(js.number));
//			}
		} // button press / release
		0x02 => {
			axis[js.number as usize] = (js.value as f32) / 32767.0;

/*			match js.number {
				0 => {
					let x = (js.value as f32) / 32767.0;
					joystick_merge_motion(input, false, x);
				},
				1 => {
					let y = (js.value as f32) / 32767.0;
					joystick_merge_motion(input, true, y);
				},
				_ => {
					
				}
			}*/
		} // axis move
		_ => {} // ignore
	}

	true
}
