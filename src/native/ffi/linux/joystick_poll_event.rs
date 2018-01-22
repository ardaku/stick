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
		// button press / release
		0x01 => {
			button[js.number as usize] = js.value == 1;
		}
		// axis move
		0x02 => {
			axis[js.number as usize] = (js.value as f32) / 32767.0;
		}
		// ignore
		_ => {}
	}

	true
}
