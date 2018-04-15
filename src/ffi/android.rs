// ffi/android.rs -- Stick
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

pub struct NativeManager { /*pub native: i32*/ }
impl NativeManager {
	pub fn create() -> NativeManager {
/*		let joystick = joystick_create::joystick_create();

		if joystick != -1 {
			joystick_async::joystick_async(joystick);
		}*/

		NativeManager { /*native: joystick*/ }
	}

	pub fn map(&self) -> (usize, usize, bool) {
/*		joystick_map::joystick_map(self.native)*/
		(0, 0, true) // unplugged
	}

	pub fn is_plugged_in(&self) -> bool {
//		self.native != -1
		false
	}

	pub fn disconnect(&mut self) -> () {
//		destroy::joystick(self.native);
//		self.native = -1;
	}

	pub fn name(&self) -> String {
//		joystick_name::joystick_name(self.native)
		"unknown".to_string()
	}

	pub fn poll_event(&self, _: &mut (Vec<f32>, Vec<bool>)) -> bool {
//		joystick_poll_event::joystick_poll_event(self.native,
//			&mut state.0, &mut state.1)
		false
	}
}
impl Drop for NativeManager {
	fn drop(&mut self) -> () {
//		if self.native != -1 {
//			destroy::joystick(self.native);
//		}
	}
}
