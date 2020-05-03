// Stick
//
// Copyright (c) 2017-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// An event on the "Standard Gamepad" from w3c shown below.
///
/// ![Standard Gamepad](https://w3c.github.io/gamepad/standard_gamepad.svg)
pub enum Event {
    /// A new controller has just been plugged in.
    Connect(Box<crate::Gamepad>),
    /// Controller unplugged.
    Disconnect,

    /// Bottom right cluster (A / Circle / Return / Right Click).
    Accept(bool),
    /// Bottom right cluster (B / X / Shift).
    Cancel(bool),
    /// Leftmost button in right cluster (Y / X / Square / Left Click).
    Common(bool),
    /// Topmost button in right cluster (X / Y / Triangle / Space).
    Action(bool),

    /// Up arrow / D-pad
    Up(bool),
    /// Down arrow / D-pad
    Down(bool),
    /// Left arrow / D-pad
    Left(bool),
    /// Right arrow / D-pad
    Right(bool),

    /// Back / Select Button (Escape)
    Back(bool),
    /// Forward / Start Button (Tab)
    Forward(bool),

    /// Near L - "Inventory" (E)
    L(bool),
    /// Near R - "Use" (R)
    R(bool),

    /// Far L Throttle - "Sneak" (Ctrl)
    Lz(f32),
    /// Far R Throttle - "Precision Action" (Alt)
    Rz(f32),

    /// Right Joystick (A / D)
    MotionH(f32),
    /// Left Joystick (W / S)
    MotionV(f32),
    /// Left Joystick (Mouse X Position)
    CameraH(f32),
    /// Right Joystick (Mouse Y Position)
    CameraV(f32),

    /// Left Joystick Button (Middle Click)
    MotionButton(bool),
    /// Right Joystick Button (F)
    CameraButton(bool),

    /// Home button (Target platform application close)
    Quit,
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Event::*;

        let pushed = |pushed| if pushed { "Pushed" } else { "Released" };

        match *self {
            Connect(_) => write!(f, "Controller Connected"),
            Disconnect => write!(f, "Controller Disconnected"),
            Accept(p) => write!(f, "Accept {}", pushed(p)),
            Cancel(p) => write!(f, "Cancel {}", pushed(p)),
            Common(p) => write!(f, "Common {}", pushed(p)),
            Action(p) => write!(f, "Action {}", pushed(p)),
            Up(p) => write!(f, "Up {}", pushed(p)),
            Down(p) => write!(f, "Down {}", pushed(p)),
            Left(p) => write!(f, "Left {}", pushed(p)),
            Right(p) => write!(f, "Right {}", pushed(p)),
            Back(p) => write!(f, "Back {}", pushed(p)),
            Forward(p) => write!(f, "Forward {}", pushed(p)),
            L(p) => write!(f, "L {}", pushed(p)),
            R(p) => write!(f, "R {}", pushed(p)),
            Lz(v) => write!(f, "Lz {}", v),
            Rz(v) => write!(f, "Rz {}", v),
            MotionH(v) => write!(f, "MotionH {}", v),
            MotionV(v) => write!(f, "MotionV {}", v),
            CameraH(v) => write!(f, "CameraH {}", v),
            CameraV(v) => write!(f, "CameraV {}", v),
            MotionButton(p) => write!(f, "MotionButton {}", pushed(p)),
            CameraButton(p) => write!(f, "CameraButton {}", pushed(p)),
            Quit => write!(f, "Quit"),
        }
    }
}
