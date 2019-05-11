use super::NativeManager;

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

/// Newtype for Axis.
#[derive(Copy, Clone)]
pub struct Axis(i16);

impl Axis {
    /// Return true if axis doesn't exist.
    pub fn is_none(self) -> bool {
        self.0 == std::i16::MIN
    }

    /// Return true if axis exists.
    pub fn is_some(self) -> bool {
        !self.is_none()
    }

    /// Convert into an f32.
    pub fn into_f32(self) -> f32 {
        if self.is_none() {
            0.0
        } else {
            (self.0 as f32) / (std::i16::MAX as f32)
        }
    }
}

/// A Joystick.
pub struct Joystick {
    x: i16,
    y: i16,
}

/// 
#[repr(u8)]
pub enum Btn {
    /// B BUTTON / SHIFT KEY "Speed Things Up"
    Cancel = 1,
    /// A BUTTON / ENTER KEY / RIGHT CLICK "Talk/Inspect/Ok"
    Accept = 3,
    /// ONE OF: Y OR X BUTTON / SPACE KEY "Jump"
    Upward = 5,
    /// ONE OF: Y OR X BUTTON / LEFT CLICK "Attack/Execute/Use Item"
    Action = 7,

    /// D-PAD LEFT / LEFT ARROW KEY
    DpadLt = 0,
    /// D-PAD RIGHT / RIGHT ARROW KEY
    DpadRt = 2,
    /// D-PAD UP / UP ARROW KEY
    DpadUp = 4,
    /// D-PAD DOWN / DOWN ARROW KEY
    DpadDn = 6,

    /// L BTN / BACKSPACE KEY "Throw"
    Throws = 8,
    /// R BTN / ALT KEY "Alternative Action/Kick"
    AltAct = 9,
    /// L THROTTLE BTN/ CTRL KEY "Crouch/Sneak"
    Crouch = 10,
    /// R THROTTLE BTN/ Q KEY "Slingshot/Bow & Arrow"
    Aiming = 11,

    /// JOY1 PUSH/Z KEY "Toggle Crouch/Sneak"
    Toggle = 12,
    /// JOY2 PUSH/C KEY "Camera/Binoculars"
    Camera = 13,
    /// BACK/START/ESCAPE KEY "Menu"
    Escape = 14,
    /// SELECT/E KEY "Inventory"
    Pocket = 15,
}

/// The state for a joystick, gamepad or controller device.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Device { // 128 bits.
    // Joystick 1 (XY). 16
    joy: (i8, i8),
    // L & R Throttles. 16
    lrt: (i8, i8),
    // Panning stick (Z-rotation,W-tilt). 32
    pan: (i16, i16),
    // 64 #'d Buttons (Left=Even,Right=Odd). 64
    btn: u64,
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let joy: (f32,f32) = ((self.joy.0 as f32) / (std::i8::MAX as f32), (self.joy.1 as f32) / (std::i8::MAX as f32));
        let pan: (f32,f32) = ((self.pan.0 as f32) / (std::i8::MAX as f32), (self.pan.1 as f32) / (std::i8::MAX as f32));
        let x: char = if self.btn(Btn::Cancel) { '▣' } else { '□' };
        let o: char = if self.btn(Btn::Accept) { '▣' } else { '□' };
        let u: char = if self.btn(Btn::Upward) { '▣' } else { '□' };
        let a: char = if self.btn(Btn::Action) { '▣' } else { '□' };

        let dl: char = if self.btn(Btn::DpadLt) { '▣' } else { '□' };
        let dr: char = if self.btn(Btn::DpadRt) { '▣' } else { '□' };
        let du: char = if self.btn(Btn::DpadUp) { '▣' } else { '□' };
        let dd: char = if self.btn(Btn::DpadDn) { '▣' } else { '□' };

        let lb: char = if self.btn(Btn::Throws) { '▣' } else { '□' };
        let rb: char = if self.btn(Btn::AltAct) { '▣' } else { '□' };
        let lt: char = if self.btn(Btn::Crouch) { '▣' } else { '□' };
        let rt: char = if self.btn(Btn::Aiming) { '▣' } else { '□' };

        let d: char = if self.btn(Btn::Toggle) { '▣' } else { '□' };
        let c: char = if self.btn(Btn::Camera) { '▣' } else { '□' };
        let e: char = if self.btn(Btn::Escape) { '▣' } else { '□' };
        let p: char = if self.btn(Btn::Pocket) { '▣' } else { '□' };

        write!(f, "joy({:.2},{:.2}) pan({:.2},{:.2}) 𝑥 {} ✓ {} ⤒ {} ⚔ {} ← {} → {} ↑ {} ↓ {} l {} r {} t {} u {} d {} c {} e {} p {}",
            joy.0, joy.1, pan.0, pan.1, x, o, u, a, dl, dr, du, dd, lb, rb, lt, rt, d, c, e, p)
    }
}

impl Device {
    /// Return true if a button is pressed.
    pub fn btn(&self, b: Btn) -> bool {
        self.btn & (1 << (b as u8)) != 0
    }
}

/// Controller ID.
#[derive(Copy,Clone)]
pub struct Id(pub u16);

/// A Controller's layout.
pub struct Layout {
    // A joystick.
    joystick: bool,
    // Can joystick be pushed as an extra button?
    joystick_button: bool,
    // An extra joystick.
    alt_joystick: bool,
    // Can extra joystick be pushed as an extra button?
    alt_joystick_button: bool,
    // A direction pad.
    dir_pad: bool,
    // A back button.
    back: bool,
    // A start button.
    start: bool,
    // A select button.
    select: bool,
    // A menu button.
    menu: bool,
    // An accept button (a).
    accept: bool,
    // A cancel button (b).
    cancel: bool,
    // A jump button (x or y).
    jump: bool,
    // An action button (x or y).
    action: bool,
    // A number of numbered buttons.
    numbered: u16,
    // Left throttle (resets position when released).
    l_throttle: bool,
    // Right throttle (resets position when released).
    r_throttle: bool,
    // Left Button
    l: bool,
    // Right Button.
    r: bool,
    // Left Button & Throttle (L on a GameCube controller)
    ll_throttle: bool,
    // Right Button & Throttle (R on a GameCube controller)
    rr_throttle: bool,
    // Trigger button.
    trigger: bool,
    // A throttle that stays stationary while user isn't touching it.
    stationary_throttle: bool,
}

/*impl Layout {
    pub fn new() -> Layout {
        Layout {
        }
    }
}*/

// A joystick, gamepad and controller device.
struct Controller {
    // Native handle to the device (fd or index).
    native_handle: u32,
    // Hardware ID for this device.
    hardware_id: u32,
    abs_min: i32,
    abs_max: i32,
}

/// An interface to all joystick, gamepad and controller devices.
pub struct Devices {
    manager: NativeManager,
    controllers: Vec<(Controller, Device)>,
}

impl Devices {
    /// Create a new interface to all joystick, gamepad and controller devices currently plugged in
    /// to this computer.
    pub fn new() -> Devices {
        let manager = NativeManager::new();
        let mut controllers = vec![];

        Devices {
            manager, controllers
        }
    }

    /// Get the number of devices currently plugged in, and update number if needed.
    pub fn update(&mut self) -> u16 {
		let (device_count, added) = self.manager.search();

        for mut controller in &mut self.controllers {
            while joystick_poll_event(self.manager.get_fd(controller.0.native_handle as usize).0, &mut controller) {
            }
        }

		if added != ::std::usize::MAX {
            println!("s{:08X}", self.manager.get_id(added).0);
            let (min, max, _) = self.manager.get_abs(added);

			self.controllers.push((Controller {
                native_handle: added as u32,
                hardware_id: self.manager.get_id(added).0,
                abs_min: min,
                abs_max: max,
            }, Device {
                joy: (0,0),
                pan: (0,0),
                lrt: (0,0),
                btn: 0,
            }));
		}

        self.controllers.len() as u16
    }

    /// Get the state of a device 
    pub fn state(&self, stick: u16) -> Device {
        self.controllers[stick as usize].1
    }

/*    /// Get a controller device by controller Id.
    pub fn get(&self, id: Id) -> Controller {
        
    }

    /// Get the main (left) joystick input for a specific controller Id.
    pub fn joy(&self, id: Id) -> Joystick {
    }

    /// Get the alternate (right) joystick input for a specific controller Id.
    pub fn joy2(&self, id: Id) -> Joystick {
    }

    /// 
    pub fn joy_btn() -> bool {
        
    }*/

/*    /// 
    pub fn joy2_btn() -> bool {
        
    }*/

    /// Swap two devices in the interface by their indexes.
    /// # Panics
    /// If either `a` or `b` are out of bounds.
    /// # Note
    /// This is useful for if in a game, you want P1 and P2 to swap which controller they are
    /// assigned to.  You can do this with:
    /// ```norun
    /// // Assuming P1 is at index 0, and P2 is at index 1,
    /// devices.swap(Id(0), Id(1));
    /// ```
    pub fn swap(&mut self, a: Id, b: Id) {
        self.controllers.swap(a.0 as usize, b.0 as usize);
    }

    /// Get the name of a device by index.
    #[allow(unused)]
    pub fn name(&self, a: Id) -> String {
        // TODO
        "Unknown".to_string()
    }
}

fn joystick_poll_event(fd: i32, device: &mut (Controller, Device)) -> bool {
    extern {
    	fn read(fd: i32, buf: *mut Event, count: usize) -> isize;
    }

	let mut js = unsafe { std::mem::uninitialized() };

	let bytes = unsafe {
		read(fd, &mut js, std::mem::size_of::<Event>())
	};

	if bytes != (std::mem::size_of::<Event>() as isize) {
		return false;
	}

    fn edit(is: bool, device: &mut (Controller, Device), b: Btn) {
        if is {
            device.1.btn |= 1 << (b as u8)
        } else {
            device.1.btn &= !(1 << (b as u8))
        }
    }

	match js.ev_type {
		// button press / release (key)
		0x01 => {
            println!("EV CODE {}", js.ev_code - 0x120);

			let is = js.ev_value == 1;

		    match js.ev_code - 0x120 {
                // ABXY
			    0|19 => edit(is, device, Btn::Action),
			    1|17 => edit(is, device, Btn::Accept),
			    2|16 => edit(is, device, Btn::Cancel),
			    3|20 => edit(is, device, Btn::Upward),
                // LT/RT
                4|24 => edit(is, device, Btn::Crouch),
                5|25 => edit(is, device, Btn::Aiming),
                // LB/RB
                6|22 => edit(is, device, Btn::Throws), // 6 is a guess.
			    7|23 => edit(is, device, Btn::AltAct),
                // Select/Start
                8|26 => edit(is, device, Btn::Escape), // 8 is a guess.
			    9|27 => edit(is, device, Btn::Pocket),
                // ?
                10 => println!("Button 10 is Unknown"),
                // D-PAD
                12|256 => edit(is, device, Btn::DpadUp),
                13|259 => edit(is, device, Btn::DpadRt),
                14|257 => edit(is, device, Btn::DpadDn),
                15|258 => edit(is, device, Btn::DpadLt),
                // 16-17 already matched
                18 => println!("Button 18 is Unknown"),
                // 19-20 already matched
                21 => println!("Button 21 is Unknown"),
                // 22-27 already matched
                28 => println!("Button 28 is Unknown"),
                29 => edit(is, device, Btn::Toggle),
                30 => edit(is, device, Btn::Camera),
			    a => println!("Button {} is Unknown", a),
            }
		}
		// axis move (abs)
		0x03 => {
			let value = transform(device.0.abs_min, device.0.abs_max,
				js.ev_value);

//           if value != 0 {
//               println!("{} {}", js.ev_code, value);
//            }

			match js.ev_code {
				0 => device.1.joy.0 = value,
				1 => device.1.joy.1 = value,
				2 => {},
				3 => device.1.pan.0 = value.into(), // Pan uses 16 bit
				4 => device.1.pan.1 = value.into(), // Pan uses 16 bit
				5 => {},
				16 => {
				},
				17 => {
				},
				40 => {}, // FIXME: precision axis, maybe implement eventually.
				a => {}, // println!("Unknown Axis: {}", a),
			}
		}
		// ignore
		_ => {}
	}

	true
}

fn deadzone(min: i32, max: i32, val: i32) -> (i32, i32) {
    let range = max - min;
    let halfr = range >> 1;
    let deadz = halfr >> 2; // 1/8th = deadzone.
    let midpt = min + halfr;
    // Center the range.
    let value = val - midpt; // -halfr to halfr
    // Take deadzone into account.
    let value = if value < deadz {
        if value > -deadz {
            0
        } else {
            value + deadz
        }
    } else {
        value - deadz
    };
    (value, (range >> 1) - deadz)
}

fn transform(min: i32, max: i32, val: i32) -> i8 {
    let (value, full) = deadzone(min, max, val);
    // Modify integer range from (-(full) thru (full)) to -127 to 127
    ((value * 127) / full).max(-127).min(127) as i8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_test() {
        let a = deadzone(-100, 100, 100);
        assert_eq!(a.0, a.1);
        assert_eq!(75, a.1);
        let b = deadzone(-100, 100, -100);
        assert_eq!(b.0, -b.1);
        assert_eq!(75, b.1);
        let c = deadzone(-100, 100, 0);
        assert_eq!(c.0, 0);
        assert_eq!(75, b.1);

        assert_eq!(transform(-100, 100, 100), 127);
        assert_eq!(transform(-100, 100, -100), -127);
        assert_eq!(transform(-100, 100, 0), 0);

        assert_eq!(transform(-128, 127, 127), 127);
        assert_eq!(transform(-128, 127, 0), 0);
        assert_eq!(transform(-128, 127, -128), -127);
    }
}
