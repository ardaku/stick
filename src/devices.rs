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

/*/// Newtype for Axis.
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
}*/

/*/// A Joystick.
pub struct Joystick {
    x: i16,
    y: i16,
}*/

/// A button on a controller.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Btn {
    /// B BUTTON / SHIFT KEY "Speed Things Up/Cancel"
    B = 0,
    /// A BUTTON / ENTER KEY / RIGHT CLICK "Talk/Inspect/Ok/Accept"
    A = 1,
    /// ONE OF: Y OR X BUTTON / SPACE KEY "Jump/Upward"
    Y = 2,
    /// ONE OF: Y OR X BUTTON / LEFT CLICK "Action/Attack/Execute/Use Item"
    X = 3,

    /// L THROTTLE BTN / CTRL KEY "Crouch/Sneak"
    Lt = 4,
    /// R THROTTLE BTN / Q KEY "Slingshot/Bow & Arrow"
    Rt = 5,
    /// L BTN / BACKSPACE KEY "Throw"
    Lb = 6,
    /// R BTN / ALT KEY "Alternative Action/Kick"
    Rb = 7,

    /// START / E KEY / MENU "Inventory"
    E = 8,
    /// BACK / SELECT / QUIT / ESCAPE KEY "Menu / Quit"
    Back = 9,
    /// JOY1 PUSH / C KEY "Toggle Crouch/Sneak"
    D = 10,
    /// JOY2 PUSH / F KEY "Camera/Binoculars"
    C = 11,

    /// D-PAD LEFT / LEFT ARROW KEY
    Left = 12,
    /// D-PAD RIGHT / RIGHT ARROW KEY
    Right = 13,
    /// D-PAD UP / UP ARROW KEY
    Up = 14,
    /// D-PAD DOWN / DOWN ARROW KEY
    Down = 15,
}

/// The state for a joystick, gamepad or controller device.
#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct Device {
    // 128 bits.
    // Joystick 1 (XY). 16
    joy: (i8, i8),
    // L & R Throttles. 16
    lrt: (u8, u8),
    // Joystick 2 (Z-rotation,W-tilt) 16
    cam: (i8, i8),
    // Panning stick 16
    pan: i16,
    // 64 #'d Buttons (Left=Even,Right=Odd). 64
    btn: u64,
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let joy: (f32, f32) = (
            (self.joy.0 as f32) / (std::i8::MAX as f32),
            (self.joy.1 as f32) / (std::i8::MAX as f32),
        );
        let cam: (f32, f32) = (
            (self.cam.0 as f32) / (std::i8::MAX as f32),
            (self.cam.1 as f32) / (std::i8::MAX as f32),
        );
        let lrt: (f32, f32) = (
            (self.lrt.0 as f32) / (std::u8::MAX as f32),
            (self.lrt.1 as f32) / (std::u8::MAX as f32),
        );
        let pan: f32 = (self.pan as f32) / (std::i16::MAX as f32);

        let b_btn: char = if self.btn(Btn::B) { '▣' } else { '□' };
        let a_btn: char = if self.btn(Btn::A) { '▣' } else { '□' };
        let y_btn: char = if self.btn(Btn::Y) { '▣' } else { '□' };
        let x_btn: char = if self.btn(Btn::X) { '▣' } else { '□' };

        let dl: char = if self.btn(Btn::Left) { '▣' } else { '□' };
        let dr: char = if self.btn(Btn::Right) { '▣' } else { '□' };
        let du: char = if self.btn(Btn::Up) { '▣' } else { '□' };
        let dd: char = if self.btn(Btn::Down) { '▣' } else { '□' };

        let lb: char = if self.btn(Btn::Lb) { '▣' } else { '□' };
        let rb: char = if self.btn(Btn::Rb) { '▣' } else { '□' };
        let lt: char = if self.btn(Btn::Lt) { '▣' } else { '□' };
        let rt: char = if self.btn(Btn::Rt) { '▣' } else { '□' };

        let d_btn: char = if self.btn(Btn::D) { '▣' } else { '□' };
        let c_btn: char = if self.btn(Btn::C) { '▣' } else { '□' };
        let back: char = if self.btn(Btn::Back) { '▣' } else { '□' };
        let e_btn: char = if self.btn(Btn::E) { '▣' } else { '□' };

        write!(
            f,
            "j({:.2},{:.2}) p({:.2}) c({:.2},{:.2}) T({:.2},{:.2}) b{} a{} y{} x{} ←{} →{} \
             ↑{} ↓{} L{} R{} l{} r{} d{} c{} e{} B{}",
            joy.0,
            joy.1,
            pan,
            cam.0,
            cam.1,
            lrt.0,
            lrt.1,
            b_btn,
            a_btn,
            y_btn,
            x_btn,
            dl,
            dr,
            du,
            dd,
            lt,
            rt,
            lb,
            rb,
            d_btn,
            c_btn,
            e_btn,
            back
        )
    }
}

impl Device {
    /// Return true if a button is pressed.
    pub fn btn(&self, b: Btn) -> bool {
        self.btn & (1 << (b as u8)) != 0
    }

    /// Swap 2 buttons in the mapping.
    pub fn swap_btn(&mut self, a: Btn, b: Btn) {
        let new_b = self.btn(a);
        let new_a = self.btn(b);

        if new_a {
            self.btn |= 1 << a as u8;
        } else {
            self.btn &= !(1 << a as u8);
        }
        if new_b {
            self.btn |= 1 << b as u8;
        } else {
            self.btn &= !(1 << b as u8);
        }
    }

    /// Swap X & Y on joy stick
    pub fn swap_joy(&mut self) {
        std::mem::swap(&mut self.joy.0, &mut self.joy.1)
    }

    /// Copy l value to pan.
    pub fn lt_to_pan(&mut self) {
        let l = self.lrt.0 as i8;
        self.pan = ((l as i32 * std::i16::MAX as i32) / 127) as i16;
    }

    /// Filter out noise for old controllers like GameCube.
    pub fn noise_filter(&mut self) {
        self.joy.0 = self
            .joy
            .0
            .saturating_add((3 * self.joy.0 as i16 / 4) as i8)
            .max(-127);
        self.joy.1 = self
            .joy
            .1
            .saturating_add((3 * self.joy.1 as i16 / 4) as i8)
            .max(-127);

        self.cam.0 = self
            .cam
            .0
            .saturating_add((3 * self.cam.0 as i16 / 4) as i8)
            .max(-127);
        self.cam.1 = self
            .cam
            .1
            .saturating_add((3 * self.cam.1 as i16 / 4) as i8)
            .max(-127);

        let lrt0 = (self.lrt.0 as i8).overflowing_add(-127).0;
        self.lrt.0 = ((lrt0.saturating_add((3 * lrt0 as i16 / 4) as i8).max(-127)) as u8)
            .overflowing_add(127)
            .0;

        let lrt1 = (self.lrt.1 as i8).overflowing_add(-127).0;
        self.lrt.1 = ((lrt1.saturating_add((3 * lrt1 as i16 / 4) as i8).max(-127)) as u8)
            .overflowing_add(127)
            .0;

        //        self.cam.0 = self.cam.0.saturating_add(self.cam.0 / 2);
        //        self.cam.1 = self.cam.1.saturating_add(self.cam.1 / 2);
    }
}

/// Controller ID.
#[derive(Copy, Clone)]
pub struct Id(pub u16);

/*/// A Controller's layout.
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
}*/

/*impl Layout {
    pub fn new() -> Layout {
        Layout {
        }
    }
}*/

// A joystick, gamepad and controller device.
#[derive(Default, Clone)]
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
        let controllers = vec![];

        Devices {
            manager,
            controllers,
        }
    }

    /// Get the number of devices currently plugged in, and update number if needed.
    pub fn update(&mut self) -> u16 {
        for mut controller in &mut self.controllers {
            while joystick_poll_event(
                self.manager.get_fd(controller.0.native_handle as usize).0,
                &mut controller,
            ) {}

            controller.1.pan = controller
                .1
                .pan
                .saturating_add(controller.1.cam.1 as i16 * 8);
        }

        let (device_count, added) = self.manager.search();

        if added != ::std::usize::MAX {
            println!("s{:08X}", self.manager.get_id(added).0);
            let (min, max, _) = self.manager.get_abs(added);

            self.controllers.resize_with(device_count, Default::default);

            self.controllers[added] = (
                Controller {
                    native_handle: added as u32,
                    hardware_id: self.manager.get_id(added).0,
                    abs_min: min,
                    abs_max: max,
                },
                Device {
                    joy: (0, 0),
                    cam: (0, 0),
                    lrt: (0, 0),
                    pan: 0,
                    btn: 0,
                },
            );
        }

        self.controllers.len() as u16
    }

    /// Get the state of a device
    pub fn state(&self, stick: u16, ) -> Device {
        let mut rtn = self.controllers[stick as usize].1;

        // Apply mods
        match self.controllers[stick as usize].0.hardware_id {
            // XBOX MODS
            0x_0E6F_0501 => rtn.swap_btn(Btn::A, Btn::B),
            // PS3 MODS
            0x_054C_0268 => rtn.swap_btn(Btn::Y, Btn::A),
            // THRUSTMASTER MODS
            0x_07B5_0316 => rtn.lt_to_pan(),
            // GAMECUBE MODS
            0x_0079_1844 => rtn.noise_filter(),
            _ => {}
        }

        rtn
    }

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
    extern "C" {
        fn read(fd: i32, buf: *mut Event, count: usize) -> isize;
    }

    let mut js = unsafe { std::mem::uninitialized() };

    let bytes = unsafe { read(fd, &mut js, std::mem::size_of::<Event>()) };

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
                0 | 19 => edit(is, device, Btn::X),
                1 | 17 => edit(is, device, Btn::A),
                2 | 16 => edit(is, device, Btn::B),
                3 | 20 => edit(is, device, Btn::Y),
                // LT/RT
                4 | 24 => edit(is, device, Btn::Lt),
                5 | 25 => edit(is, device, Btn::Rt),
                // LB/RB
                6 | 22 => edit(is, device, Btn::Lb), // 6 is a guess.
                7 | 23 => edit(is, device, Btn::Rb),
                // Select/Start
                8 | 26 => edit(is, device, Btn::Back), // 8 is a guess.
                9 | 27 => edit(is, device, Btn::E),
                // ?
                10 => println!("Button 10 is Unknown"),
                // D-PAD
                12 | 256 => edit(is, device, Btn::Up),
                13 | 259 => edit(is, device, Btn::Right),
                14 | 257 => edit(is, device, Btn::Down),
                15 | 258 => edit(is, device, Btn::Left),
                // 16-17 already matched
                18 => println!("Button 18 is Unknown"),
                // 19-20 already matched
                21 => println!("Button 21 is Unknown"),
                // 22-27 already matched
                28 => println!("Button 28 is Unknown"),
                29 => edit(is, device, Btn::D),
                30 => edit(is, device, Btn::C),
                a => println!("Button {} is Unknown", a),
            }
        }
        // axis move (abs)
        0x03 => {
            let value = transform(device.0.abs_min, device.0.abs_max, js.ev_value);

            //            if value != 0 {
            //                println!("{} {}", js.ev_code, value);
            //            }

            // For some reason this is different on the GameCube controller, so fix it.
            let (cam_x, cam_y, lrt_l, lrt_r) = match device.0.hardware_id {
                0x_0079_1844 => (5, 2, 3, 4),
                _ => (3, 4, 2, 5),
            };

            match js.ev_code {
                0 => device.1.joy.0 = value,
                1 => device.1.joy.1 = value,
                16 => {
                    if js.ev_value < 0 {
                        edit(true, device, Btn::Left);
                        edit(false, device, Btn::Right);
                    } else if js.ev_value > 0 {
                        edit(false, device, Btn::Left);
                        edit(true, device, Btn::Right);
                    } else {
                        edit(false, device, Btn::Left);
                        edit(false, device, Btn::Right);
                    }
                }
                17 => {
                    if js.ev_value < 0 {
                        edit(true, device, Btn::Up);
                        edit(false, device, Btn::Down);
                    } else if js.ev_value > 0 {
                        edit(false, device, Btn::Up);
                        edit(true, device, Btn::Down);
                    } else {
                        edit(false, device, Btn::Up);
                        edit(false, device, Btn::Down);
                    }
                }
                40 => {} // IGNORE: Duplicate axis.
                a => {
                    if a == cam_x {
                        device.1.cam.0 = value;
                    } else if a == cam_y {
                        device.1.cam.1 = value;
                    } else if a == lrt_l {
                        js.ev_value = js.ev_value.max(-127);
                        device.1.lrt.0 = js.ev_value as u8;
                    //                        edit(js.ev_value > 250, device, Btn::Crouch);
                    } else if a == lrt_r {
                        js.ev_value = js.ev_value.max(-127);
                        device.1.lrt.1 = js.ev_value as u8;
                        //                        edit(js.ev_value > 250, device, Btn::Aiming);
                    }
                } // println!("Unknown Axis: {}", a),
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
