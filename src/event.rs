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
#[derive(Debug)]
#[non_exhaustive]
pub enum Event {
    /*
     * Connecting and disconnecting (common to all controllers)
     */

    /*  */
    /// A new controller has just been plugged in.
    Connect(Box<crate::Gamepad>),
    /// Controller unplugged.
    Disconnect,

    /*
     * Events based on the w3 Standard Gamepad (may appear on other gamepads as
     * well)
     */

    /* Main button cluster */
    /// A / Circle / Return / Left Click.  Main action button to do something.
    Do(bool),
    /// B / Cross / Shift.  Button to exit out of a menu, speed up, or lower.
    Go(bool),
    /// Y / X / Square / Right Click.  Use item.
    Use(bool),
    /// X / Y / Triangle / Space.  Jumping / special move.
    Top(bool),

    /* D-PAD / Hat */
    /// Up arrow / D-pad
    Up(bool),
    /// Down arrow / D-pad
    Down(bool),
    /// Left arrow / D-pad
    Left(bool),
    /// Right arrow / D-pad
    Right(bool),

    /* Center buttons */
    /// Back / Select Button (Escape)
    Back(bool),
    /// Forward / Start Button (Tab)
    Forward(bool),

    /* Shoulder Buttons (L, R, Z - 1) */
    /// Left shoulder button (near button if no trigger) - "Inventory" (E)
    LShoulder(bool),
    /// Right shoulder button (near button if no trigger) - "Use" (R)
    RShoulder(bool),

    /* Shoulder Triggers (LZ, RZ - 2)  */
    /// Left Shoulder Trigger (far button if no trigger) - "Sneak" (Ctrl)
    LShoulderTrigger(f32),
    /// Right Shoulder Trigger (far button if no trigger) - "Precision Action"
    /// (Alt)
    RShoulderTrigger(f32),

    /* Joystick */
    /// Main joystick X axis (A / D)
    JoystickH(f32),
    /// Main joystick Y axis (W / S)
    JoystickV(f32),
    /// Main joystick Z axis
    JoystickR(f32),
    /// Secondary Joystick X axis (Mouse X Position)
    CStickH(f32),
    /// Secondary Joystick Y axis (Mouse Y Position)
    CStickV(f32),
    /// Secondary Joystick Z axis
    CStickR(f32),

    /// Left Joystick Button (Middle Click)
    JoystickButton(bool),
    /// Right Joystick Button (F)
    CStickButton(bool),

    /// Command button (Exit gameplay, usually into a menu)
    Cmd,

    /*
     * Generic Extra Buttons
     */
    /// Extra unlabeled buttons (Indexed Left to Right, Upper to lower)
    Generic(u16, bool),

    /*
     * Special XBox Controllers Extra Buttons
     */

    /*
     * Realistic flight simulation stick extra buttons, switches, etc.
     */
     
    /*
     * Mice-like controllers extra buttons, scroll wheel
     */
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Event::*;

        let pushed = |pushed| if pushed { "Pushed" } else { "Released" };

        match *self {
            Connect(_) => write!(f, "Controller Connected"),
            Disconnect => write!(f, "Controller Disconnected"),
            Do(p) => write!(f, "Do {}", pushed(p)),
            Go(p) => write!(f, "Go {}", pushed(p)),
            Use(p) => write!(f, "Use {}", pushed(p)),
            Top(p) => write!(f, "Top {}", pushed(p)),
            Up(p) => write!(f, "Up {}", pushed(p)),
            Down(p) => write!(f, "Down {}", pushed(p)),
            Left(p) => write!(f, "Left {}", pushed(p)),
            Right(p) => write!(f, "Right {}", pushed(p)),
            Back(p) => write!(f, "Back {}", pushed(p)),
            Forward(p) => write!(f, "Forward {}", pushed(p)),
            LShoulder(p) => write!(f, "LShoulder {}", pushed(p)),
            RShoulder(p) => write!(f, "RShoulder {}", pushed(p)),
            LShoulderTrigger(v) => write!(f, "LShoulderTrigger {}", v),
            RShoulderTrigger(v) => write!(f, "RShoulderTrigger {}", v),
            JoystickH(v) => write!(f, "JoystickH {}", v),
            JoystickV(v) => write!(f, "JoystickV {}", v),
            JoystickR(v) => write!(f, "JoystickR {}", v),
            CStickH(v) => write!(f, "CStickH {}", v),
            CStickV(v) => write!(f, "CStickV {}", v),
            CStickR(v) => write!(f, "CStickR {}", v),
            JoystickButton(p) => write!(f, "JoystickButton {}", pushed(p)),
            CStickButton(p) => write!(f, "CStickButton {}", pushed(p)),
            Generic(l, p) => write!(f, "Generic{} {}", l, pushed(p)),
            Cmd => write!(f, "Cmd"),
        }
    }
}
