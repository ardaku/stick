use std::fmt;

/// Controller Input
///
/// On buttons, Option<bool> is used to specify button state:
/// * `None // Just Released`
/// * `Some(true) // Just Pressed`
/// * `Some(false) // Held down`
#[derive(PartialEq, Copy, Clone)]
pub enum Input {
    /// Main joystick movement.
    Move(f32, f32),
    /// Camera / C joystick movement.
    Camera(f32, f32),
    /// Left Throttle movement.
    ThrottleL(f32),
    /// Right Throttle movement.
    ThrottleR(f32),
    /// Accept (A Button / Left Top Button - Missle / Circle)
    Accept(Option<bool>),
    /// Cancel (B Button / Side Button / Cross)
    Cancel(Option<bool>),
    /// Execute (X Button / Trigger / Triangle)
    Execute(Option<bool>),
    /// Action (Y Button / Right Top Button / Square)
    Action(Option<bool>),
    /// Left Button (0: L Trigger, 1: LZ / L Bumper).  0 is
    /// farthest away from user, incrementing as buttons get closer.
    L(u8, Option<bool>),
    /// Right Button (0: R Trigger, 1: Z / RZ / R Bumper). 0 is
    /// farthest away from user, incrementing as buttons get closer.
    R(u8, Option<bool>),
    /// Pause Menu (Start Button)
    Menu(Option<bool>),
    /// Show Controls (Guide on XBox, Select on PlayStation).  Use as
    /// alternative for Menu -> "Controls".
    Controls,
    /// Exit This Screen (Back on XBox).  Use as alternative for
    /// Menu -> "Quit" or Cancel, depending on situation.
    Exit,
    /// HAT/DPAD Up Button
    Up(Option<bool>),
    /// HAT/DPAD Down Button
    Down(Option<bool>),
    /// Hat/D-Pad left button
    Left(Option<bool>),
    /// Hat/D-Pad right button.
    Right(Option<bool>),
    /// Movement stick Push
    MoveStick(Option<bool>),
    /// Camera stick Push
    CamStick(Option<bool>),
    /// Device Plugged-In
    PluggedIn(u32),
    /// Device Un-Plugged
    UnPlugged(u32),
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Input::*;

        match *self {
            Move(x, y) => write!(f, "Move ({}, {})", x, y),
            Camera(x, y) => write!(f, "Camera ({}, {})", x, y),
            ThrottleL(x) => write!(f, "ThrottleL ({})", x),
            ThrottleR(x) => write!(f, "ThrottleR ({})", x),
            Accept(s) => write!(f, "Accept {:?}", s),
            Cancel(s) => write!(f, "Cancel {:?}", s),
            Execute(s) => write!(f, "Execute {:?}", s),
            Action(s) => write!(f, "Action {:?}", s),
            L(a, s) => write!(f, "L-{} {:?}", a, s),
            R(a, s) => write!(f, "R-{} {:?}", a, s),
            Menu(s) => write!(f, "Menu {:?}", s),
            Controls => write!(f, "Controls"),
            Exit => write!(f, "Exit"),
            Up(s) => write!(f, "Up {:?}", s),
            Down(s) => write!(f, "Down {:?}", s),
            Left(s) => write!(f, "Left {:?}", s),
            Right(s) => write!(f, "Right {:?}", s),
            MoveStick(s) => write!(f, "Movement Stick Push {:?}", s),
            CamStick(s) => write!(f, "Camera Stick Push {:?}", s),
            PluggedIn(x) => write!(f, "Device Plugged-In {:x}", x),
            UnPlugged(x) => write!(f, "Device Un-Plugged {:x}", x),
        }
    }
}
