// Stick
//
// Copyright (c) 2017-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// An event from a [`Controller`](crate::Controller).
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
    /// A new controller has just been plugged in.
    Connect(Box<crate::Controller>),
    /// Controller unplugged.
    Disconnect,

    /*
     * Events based on the w3 Standard Gamepad (may appear on other gamepads as
     * well)
     */

    /* Center buttons */
    /// Home button (Exit gameplay, usually into a console menu)
    Home(bool),
    /// Back / Select / Minus / Stop Button (Escape)
    Prev(bool),
    /// Forward / Start / Plus / Play Button (Tab)
    Next(bool),

    /* Action pad - action button cluster */
    /// A / 1 / 4 / Circle / Return / Left Click.  Action A (Main action).
    ActionA(bool),
    /// B / 2 / 3 / Cross / Shift.  Action B (Secondary action).
    ActionB(bool),
    /// C
    ActionC(bool),
    /// Y / X / Square / Right Click / H.  Horizontal action.
    ActionH(bool),
    /// X / Y / Triangle / Space / V.  Vertical action (Topmost action button).
    ActionV(bool),
    /// Numbered or unlabeled programmable action buttons (If unlabelled,
    /// numbered from left to right, upper to lower)
    Action(u16, bool),

    /* D-PAD */
    /// D-pad Up
    DpadUp(bool),
    /// D-pad Down
    DpadDown(bool),
    /// D-pad Left
    DpadLeft(bool),
    /// D-pad Right
    DpadRight(bool),

    /* Bumper Triggers (LZ, RZ - 2)  */
    /// Range(0.0, 1.0) - Left Bumper Trigger (far button if no trigger) -
    /// "Sneak" (Ctrl)
    TriggerL(f64),
    /// Range(0.0, 1.0) - Right Bumper Trigger (far button if no trigger) -
    /// "Precision Action" (Alt)
    TriggerR(f64),

    /* Bumper Buttons (L, R, Z - 1) */
    /// Left shoulder button (near button if no trigger) - "Inventory" (E)
    BumperL(bool),
    /// Right shoulder button (near button if no trigger) - "Use" (R)
    BumperR(bool),

    /* Joystick */
    /// Range(-1.0, 1.0) - Main stick horizontal axis (A / D)
    JoyX(f64),
    /// Range(-1.0, 1.0) - Main stick vertical / depth axis (W / S)
    JoyY(f64),
    /// Range(-1.0, 1.0) - Main stick rotation / yaw axis
    JoyZ(f64),
    /// Range(-1.0, 1.0) - Secondary stick X axis (Mouse X Position)
    CamX(f64),
    /// Range(-1.0, 1.0) - Secondary stick Y axis (Mouse Y Position)
    CamY(f64),
    /// Range(-1.0, 1.0) - Secondary stick Z axis
    CamZ(f64),

    /* Joystick Buttons */
    /// Left Joystick Button (Middle Click)
    JoyPush(bool),
    /// Right Joystick Button (F)
    CamPush(bool),

    /*
     * Special XBox/Steam Controllers Extra Buttons
     */

    /* Paddles */
    /// Back right grip button (upper if there are two)
    PaddleRight(bool),
    /// Back left grip button (upper if there are two)
    PaddleLeft(bool),
    /// Back lower right grip button
    PaddleRightPinky(bool),
    /// Back lower left grip button
    PaddleLeftPinky(bool),

    /*
     * Realistic flight simulation stick extra buttons, switches, etc.
     */

    /* Buttons */
    /// Autopilot Toggle Button
    AutopilotToggle(bool),
    /// Landing Gear Horn Silence Button
    LandingGearSilence(bool),

    /* 8-way POV Hat */
    /// POV Hat Up
    PovUp(bool),
    /// POV Hat Down
    PovDown(bool),
    /// POV Hat Left
    PovLeft(bool),
    /// POV Hat Right
    PovRight(bool),

    /* 4-way Mic Switch */
    /// Mic Hat Up
    MicUp(bool),
    /// Mic Hat Down
    MicDown(bool),
    /// Mic Hat Left
    MicLeft(bool),
    /// Mic Hat Right
    MicRight(bool),
    /// Mic Hat Push Button
    MicPush(bool),

    /// Range(0.0, 1.0) - Slew Control
    Slew(f64),
    /// Range(0.0, 1.0) - Stationary throttle (1.0 is forward, 0.0 is backward)
    Throttle(f64),
    /// Range(0.0, 1.0) - Left stationary throttle (1.0 is forward,
    /// 0.0 is backward)
    ThrottleL(f64),
    /// Range(0.0, 1.0) - Right stationary throttle (1.0 is forward, 0.0 is
    /// backward)
    ThrottleR(f64),

    /// Left throttle button
    ThrottleButtonL(bool),

    /// Engine Fuel Flow Left two-way switch
    /// - `true` - Normal
    /// - `false` - Override
    EngineFuelFlowL(bool),
    /// Engine Fuel Flow Right two-way switch
    /// - `true` - Normal
    /// - `false` - Override
    EngineFuelFlowR(bool),
    /// EAC two-way switch
    /// - `true` - Arm
    /// - `false` - Off
    Eac(bool),
    /// Radar Altimeter two-way switch
    /// - `true` - Normal
    /// - `false` - Disabled
    RadarAltimeter(bool),
    /// APU two-way switch
    /// - `true` - Start
    /// - `false` - Off
    Apu(bool),

    /// Autopilot three-way switch Forward.
    /// - `true` - Forward (Path)
    /// - `false` - Neutral (Altitude / Heading)
    AutopilotPath(bool),
    /// Autopilot three-way switch Backward.
    /// - `true` - Backward (Alt)
    /// - `false` - Neutral (Altitude / Heading)
    AutopilotAlt(bool),
    /// Flaps three-way switch Forward.
    /// - `true` - Forward (Up)
    /// - `false` - Neutral (Maneuver)
    FlapsUp(bool),
    /// Flaps three-way switch Backward.
    /// - `true` - Backward (Down)
    /// - `false` - Neutral (Maneuver)
    FlapsDown(bool),
    /// Left Engine Operate three-way switch Forward.
    /// - `true` - Forward (Ignition)
    /// - `false` - Neutral (Normal)
    EngineLIgnition(bool),
    /// Left Engine Operate three-way switch Backward.
    /// - `true` - Backward (Motor)
    /// - `false` - Neutral (Normal)
    EngineLMotor(bool),
    /// Right Engine Operate three-way switch Forward.
    /// - `true` - Forward (Ignition)
    /// - `false` - Neutral (Normal)
    EngineRIgnition(bool),
    /// Right Engine Operate three-way switch Backward.
    /// - `true` - Backward (Motor)
    /// - `false` - Neutral (Normal)
    EngineRMotor(bool),
    /// Pinky three-way switch Forward.
    PinkyForward(bool),
    /// Pinky three-way switch Backward.
    PinkyBackward(bool),
    /// Speedbrake three-way switch Forward.
    SpeedbrakeForward(bool),
    /// Speedbrake three-way switch Backward.
    SpeedbrakeBackward(bool),
    /// Boat three-way switch Forward.
    BoatForward(bool),
    /// Pinky three-way switch Backward.
    BoatBackward(bool),
    /// China hat three-way switch Forward.
    ChinaForward(bool),
    /// China hat three-way switch Backward.
    ChinaBackward(bool),

    /*
     * Mice-like controllers extra buttons, scroll wheel
     */

    /* Extra Mouse buttons */
    /// DPI Switch
    Dpi(bool),

    /* Mouse Main */
    /// Range(-1.0, 1.0) - Mouse delta position horizontal
    MouseX(f64),
    /// Range(-1.0, 1.0) - Mouse delta position vertical
    MouseY(f64),
    /// Left click (main click, push button)
    MousePush(bool),
    /// Right click (secondary click, push button 2)
    MouseMenu(bool),

    /* Mouse Wheel */
    /// Range(-1.0, 1.0) - Scroll wheel horizontal
    WheelX(f64),
    /// Range(-1.0, 1.0) - Scroll wheel vertical
    WheelY(f64),
    /// Middle click (scroll wheel push button)
    WheelPush(bool),

    /*
     * Ignore Events
     */

    /* */
    #[doc(hidden)]
    Nil(bool),
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Event::*;

        let pushed = |pushd: &bool| if *pushd { "Pushed" } else { "Released" };
        let two = |two: &bool| match two {
            true => "Forward",
            false => "Backward",
        };
        let sw = |three: &bool| match three {
            true => "Enter",
            false => "Leave",
        };

        match self {
            Connect(_) => write!(f, "Controller Connected"),
            Disconnect => write!(f, "Controller Disconnected"),
            ActionA(p) => write!(f, "ActionA {}", pushed(p)),
            ActionB(p) => write!(f, "ActionB {}", pushed(p)),
            ActionC(p) => write!(f, "ActionC {}", pushed(p)),
            ActionH(p) => write!(f, "ActionH {}", pushed(p)),
            ActionV(p) => write!(f, "ActionV {}", pushed(p)),
            DpadUp(p) => write!(f, "DpadUp {}", pushed(p)),
            DpadDown(p) => write!(f, "DpadDown {}", pushed(p)),
            DpadLeft(p) => write!(f, "DpadLeft {}", pushed(p)),
            DpadRight(p) => write!(f, "DpadRight {}", pushed(p)),
            Prev(p) => write!(f, "Prev {}", pushed(p)),
            Next(p) => write!(f, "Next {}", pushed(p)),
            BumperL(p) => write!(f, "BumperL {}", pushed(p)),
            BumperR(p) => write!(f, "BumperR {}", pushed(p)),
            TriggerL(v) => write!(f, "TriggerL {}", v),
            TriggerR(v) => write!(f, "TriggerR {}", v),
            JoyX(v) => write!(f, "JoyX {}", v),
            JoyY(v) => write!(f, "JoyY {}", v),
            JoyZ(v) => write!(f, "JoyZ {}", v),
            CamX(v) => write!(f, "CamX {}", v),
            CamY(v) => write!(f, "CamY {}", v),
            CamZ(v) => write!(f, "CamZ {}", v),
            JoyPush(p) => write!(f, "JoyPush {}", pushed(p)),
            CamPush(p) => write!(f, "CamPush {}", pushed(p)),
            PaddleRight(p) => write!(f, "PaddleRight {}", pushed(p)),
            PaddleLeft(p) => write!(f, "PaddleLeft {}", pushed(p)),
            PaddleRightPinky(p) => write!(f, "PaddleRightPinky {}", pushed(p)),
            PaddleLeftPinky(p) => write!(f, "PaddleLeftPinky {}", pushed(p)),
            Home(p) => write!(f, "Home {}", pushed(p)),
            Action(l, p) => write!(f, "Action{} {}", l, pushed(p)),
            AutopilotToggle(p) => write!(f, "AutopilotToggle {}", pushed(p)),
            LandingGearSilence(p) => {
                write!(f, "LandingGearSilence {}", pushed(p))
            }
            PovUp(p) => write!(f, "PovUp {}", pushed(p)),
            PovDown(p) => write!(f, "PovDown {}", pushed(p)),
            PovLeft(p) => write!(f, "PovLeft {}", pushed(p)),
            PovRight(p) => write!(f, "PovRight {}", pushed(p)),
            MicUp(p) => write!(f, "MicUp {}", pushed(p)),
            MicDown(p) => write!(f, "MicDown {}", pushed(p)),
            MicLeft(p) => write!(f, "MicLeft {}", pushed(p)),
            MicRight(p) => write!(f, "MicRight {}", pushed(p)),
            MicPush(p) => write!(f, "MicPush {}", pushed(p)),
            Slew(v) => write!(f, "Slew {}", v),
            Throttle(v) => write!(f, "Throttle {}", v),
            ThrottleL(v) => write!(f, "ThrottleL {}", v),
            ThrottleR(v) => write!(f, "ThrottleR {}", v),
            ThrottleButtonL(p) => write!(f, "ThrottleButtonL {}", pushed(p)),
            EngineFuelFlowL(t) => write!(f, "EngineFuelFlowL {}", two(t)),
            EngineFuelFlowR(t) => write!(f, "EngineFuelFlowR {}", two(t)),
            Eac(t) => write!(f, "Eac {}", two(t)),
            RadarAltimeter(t) => write!(f, "RadarAltimeter {}", two(t)),
            Apu(t) => write!(f, "Apu {}", two(t)),
            AutopilotPath(p) => write!(f, "AutopilotPath {}", sw(p)),
            AutopilotAlt(p) => write!(f, "AutopilotAlt {}", sw(p)),
            FlapsUp(p) => write!(f, "FlapsUp {}", sw(p)),
            FlapsDown(p) => write!(f, "FlapsDown {}", sw(p)),
            EngineLIgnition(p) => write!(f, "EngineLIgnition {}", sw(p)),
            EngineLMotor(p) => write!(f, "EngineLMotor {}", sw(p)),
            EngineRIgnition(p) => write!(f, "EngineRIgnition {}", sw(p)),
            EngineRMotor(p) => write!(f, "EngineRMotor {}", sw(p)),
            PinkyForward(p) => write!(f, "PinkyForward {}", sw(p)),
            PinkyBackward(p) => write!(f, "PinkyBackward {}", sw(p)),
            SpeedbrakeForward(p) => write!(f, "SpeedbrakeForward {}", sw(p)),
            SpeedbrakeBackward(p) => write!(f, "SpeedbrakeBackward {}", sw(p)),
            BoatForward(p) => write!(f, "BoatForward {}", sw(p)),
            BoatBackward(p) => write!(f, "BoatBackward {}", sw(p)),
            ChinaForward(p) => write!(f, "ChinaForward {}", sw(p)),
            ChinaBackward(p) => write!(f, "ChinaBackward {}", sw(p)),
            Dpi(p) => write!(f, "Dpi {}", pushed(p)),
            MouseX(v) => write!(f, "MouseX {}", v),
            MouseY(v) => write!(f, "MouseY {}", v),
            MousePush(p) => write!(f, "MousePush {}", pushed(p)),
            MouseMenu(p) => write!(f, "MouseMenu {}", pushed(p)),
            WheelX(v) => write!(f, "WheelX {}", v),
            WheelY(v) => write!(f, "WheelY {}", v),
            WheelPush(p) => write!(f, "WheelPush {}", pushed(p)),
            Nil(p) => write!(f, "Nil {}", pushed(p)),
        }
    }
}
