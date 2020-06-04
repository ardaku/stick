// Stick
//
// Copyright (c) 2017-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// An event from a `Pad`.
///
/// # Gamepad Types
/// ## Standard Gamepad
/// A video game controller similar to w3c's "standard gamepad":
///
/// ## Flightstick
/// A joystick typically used in flight simulations and robotics:
///
#[derive(Debug)]
#[non_exhaustive]
pub enum Event {
    /*
     * Connecting and disconnecting (common to all controllers)
     */

    /*  */
    /// A new pad has just been plugged in.
    Connect(Box<crate::Pad>),
    /// Pad unplugged.
    Disconnect,

    /*
     * Events based on the w3 Standard Gamepad (may appear on other gamepads as
     * well)
     */

    /* Center buttons */
    /// Command button (Exit gameplay, usually into a menu)
    Cmd,
    /// Back / Select Button (Escape)
    Back(bool),
    /// Forward / Start Button (Tab)
    Forward(bool),
    
    /* Main button cluster */
    /// A / 1 / Circle / Return / Left Click.  Main action button to do
    /// something.
    Primary(bool),
    /// B / 2 / Cross / Shift.  Button to exit out of a menu, speed up, or
    /// lower.
    Secondary(bool),
    /// Y / X / Square / Right Click.  Use item.
    Item(bool),
    /// X / Y / Triangle / Space.  Jumping / special move.  Always the topmost
    /// button in the cluster.
    Action(bool),

    /* D-PAD / POV Hat (8-way) */
    /// D-pad / POV Hat Up
    Up(bool),
    /// D-pad / POV Hat Down
    Down(bool),
    /// D-pad / POV Hat Left
    Left(bool),
    /// D-pad / POV Hat Right
    Right(bool),

    /* Shoulder Triggers (LZ, RZ - 2)  */
    /// Left Shoulder Trigger (far button if no trigger) - "Sneak" (Ctrl)
    ShoulderL(f32),
    /// Right Shoulder Trigger (far button if no trigger) - "Precision Action"
    /// (Alt)
    ShoulderR(f32),

    /* Shoulder Buttons (L, R, Z - 1) */
    /// Left shoulder button (near button if no trigger) - "Inventory" (E)
    ShoulderButtonL(bool),
    /// Right shoulder button (near button if no trigger) - "Use" (R)
    ShoulderButtonR(bool),

    /* Joystick */
    /// Main joystick horizontal axis (A / D)
    StickHor(f32),
    /// Main joystick vertical axis (W / S)
    StickVer(f32),
    /// Main joystick yaw axis
    StickYaw(f32),
    /// Secondary Joystick X axis (Mouse X Position)
    CStickHor(f32),
    /// Secondary Joystick Y axis (Mouse Y Position)
    CStickVer(f32),
    /// Secondary Joystick Z axis
    CStickYaw(f32),

    /* Joystick Buttons */
    /// Left Joystick Button (Middle Click)
    StickButton(bool),
    /// Right Joystick Button (F)
    CStickButton(bool),

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

    /* Buttons */
    /// Autopilot Toggle Button
    AutopilotToggle(bool),
    /// Landing Gear Horn Silence Button
    LandingGearSilence(bool),

    /* 4-way Mic Switch */
    /// Mic Hat Up
    MicUp(bool),
    /// Mic Hat Down
    MicDown(bool),
    /// Mic Hat Left
    MicLeft(bool),
    /// Mic Hat Right
    MicRight(bool),

    /// Slew Control
    Slew(f32),
    /// Left stationary throttle
    ThrottleL(f32),
    /// Right stationary throttle
    ThrottleR(f32),
    
    /// Left throttle button
    ThrottleButtonL(bool),

    /// Altitude Heading three-way switch
    /// - `Some(true)` - Forward (Path)
    /// - `None` - Neutral
    /// - `Some(false)` - Backward (Alt)
    AltitudeHeading(Option<bool>),
    /// Maneuver three-way switch
    /// - `Some(true)` - Forward (Up)
    /// - `None` - Neutral
    /// - `Some(false)` - Backward (Down)
    Maneuver(Option<bool>),
    /// Left three-way switch
    /// - `Some(true)` - Forward (Ignition)
    /// - `None` - Neutral
    /// - `Some(false)` - Backward (Motor)
    IgnitionL(Option<bool>),
    /// Right three-way switch pushed forward for ignition, backward for motor.
    /// - `Some(true)` - Forward (Ignition)
    /// - `None` - Neutral
    /// - `Some(false)` - Backward (Motor)
    IgnitionR(Option<bool>),
    /// Pinky Switch
    /// - `Some(true)` - Forward
    /// - `None` - Neutral
    /// - `Some(false)` - Backward
    PinkySwitch(Option<bool>),
    /// Speedbrake
    /// - `Some(true)` - Forward
    /// - `None` - Neutral
    /// - `Some(false)` - Backward
    Speedbrake(Option<bool>),
    /// Boat Switch
    /// - `Some(true)` - Forward
    /// - `None` - Neutral
    /// - `Some(false)` - Backward
    BoatSwitch(Option<bool>),
    /// China Hat
    /// - `Some(true)` - Forward
    /// - `None` - Neutral
    /// - `Some(false)` - Backward
    ChinaHat(Option<bool>),

    /*
     * Mice-like controllers extra buttons, scroll wheel
     */
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Event::*;

        let pushed = |pushed| if pushed { "Pushed" } else { "Released" };
        let three = |three| match three {
            Some(true) => "Forward",
            None => "Neutral",
            Some(false) => "Backward",
        };

        match *self {
            Connect(_) => write!(f, "Controller Connected"),
            Disconnect => write!(f, "Controller Disconnected"),
            Primary(p) => write!(f, "Primary {}", pushed(p)),
            Secondary(p) => write!(f, "Secondary {}", pushed(p)),
            Item(p) => write!(f, "Item {}", pushed(p)),
            Action(p) => write!(f, "Action {}", pushed(p)),
            Up(p) => write!(f, "Up {}", pushed(p)),
            Down(p) => write!(f, "Down {}", pushed(p)),
            Left(p) => write!(f, "Left {}", pushed(p)),
            Right(p) => write!(f, "Right {}", pushed(p)),
            Back(p) => write!(f, "Back {}", pushed(p)),
            Forward(p) => write!(f, "Forward {}", pushed(p)),
            ShoulderButtonL(p) => write!(f, "ShoulderButtonL {}", pushed(p)),
            ShoulderButtonR(p) => write!(f, "ShoulderButtonR {}", pushed(p)),
            ShoulderL(v) => write!(f, "ShoulderL {}", v),
            ShoulderR(v) => write!(f, "ShoulderR {}", v),
            StickHor(v) => write!(f, "StickHor {}", v),
            StickVer(v) => write!(f, "StickVer {}", v),
            StickYaw(v) => write!(f, "StickYaw {}", v),
            CStickHor(v) => write!(f, "CStickHor {}", v),
            CStickVer(v) => write!(f, "CStickVer {}", v),
            CStickYaw(v) => write!(f, "CStickYaw {}", v),
            StickButton(p) => write!(f, "StickButton {}", pushed(p)),
            CStickButton(p) => write!(f, "CStickButton {}", pushed(p)),
            Cmd => write!(f, "Cmd"),
            Generic(l, p) => write!(f, "Generic{} {}", l, pushed(p)),
            AutopilotToggle(p) => write!(f, "AutopilotToggle {}", pushed(p)),
            LandingGearSilence(p) => write!(f, "LandingGearSilence {}", pushed(p)),
            MicUp(p) => write!(f, "MicUp {}", pushed(p)),
            MicDown(p) => write!(f, "MicDown {}", pushed(p)),
            MicLeft(p) => write!(f, "MicLeft {}", pushed(p)),
            MicRight(p) => write!(f, "MicRight {}", pushed(p)),
            Slew(v) => write!(f, "Slew {}", v),
            ThrottleL(v) => write!(f, "ThrottleL {}", v),
            ThrottleR(v) => write!(f, "ThrottleR {}", v),
            ThrottleButtonL(p) => write!(f, "ThrottleButtonL {}", pushed(p)),
            AltitudeHeading(t) => write!(f, "AltitudeHeading {}", three(t)),
            Maneuver(t) => write!(f, "Maneuver {}", three(t)),
            IgnitionL(t) => write!(f, "IgnitionL {}", three(t)),
            IgnitionR(t) => write!(f, "IgnitionR {}", three(t)),
            PinkySwitch(t) => write!(f, "PinkySwitch {}", three(t)),
            Speedbrake(t) => write!(f, "Speedbrake {}", three(t)),
            BoatSwitch(t) => write!(f, "BoatSwitch {}", three(t)),
            ChinaHat(t) => write!(f, "ChinaHat {}", three(t)),
        }
    }
}
