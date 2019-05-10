use super::NativeManager;

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
    Select = 15,
}

/// The state for a joystick, gamepad or controller device.
#[derive(Debug, Copy, Clone)]
pub struct Device {
    // Joystick 1.
    joy: (i16, i16),
    // Panning stick.
    pan: (i16, i16),
    // 64 #'d Buttons (Left=Even,Right=Odd).
    btn: u64,
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let joy: (f32,f32) = ((self.joy.0 as f32) / (std::i16::MAX as f32), (self.joy.1 as f32) / (std::i16::MAX as f32));
        let pan: (f32,f32) = ((self.joy.0 as f32) / (std::i16::MAX as f32), (self.joy.1 as f32) / (std::i16::MAX as f32));
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

        let z: char = if self.btn(Btn::Toggle) { '▣' } else { '□' };
        let c: char = if self.btn(Btn::Camera) { '▣' } else { '□' };
        let esc: char = if self.btn(Btn::Escape) { '▣' } else { '□' };
        let sel: char = if self.btn(Btn::Select) { '▣' } else { '□' };

        write!(f, "joy{:?} pan{:?} 𝑥 {} ✓ {} ⤒ {} ⚔ {} ← {} → {} ↑ {} ↓ {} l {} r {} t {} u {} z {} c {} ␛ {} ␐ {}",
            joy, pan, x, o, u, a, dl, dr, du, dd, lb, rb, lt, rt, z, c, esc, sel)
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

        for controller in self.controllers.iter() {
            
        }

		if added != ::std::usize::MAX {
			self.controllers.push((Controller {
                native_handle: added as u32,
                hardware_id: self.manager.get_id(added).0,
            }, Device {
                joy: (0,0),
                pan: (0,0),
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
    /// ```
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
