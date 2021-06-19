// Stick
// Copyright © 2017-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

/// An event from a [`Controller`](crate::Controller).
///
/// # Gamepad Types
/// ## Standard Gamepad
/// A video game controller similar to w3c's "standard gamepad":
///
/// ## Flightstick
/// A joystick typically used in flight simulations and robotics:
///
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum Event {
    /*
     * All controllers.
     */
    /// Controller unplugged.
    Disconnect,

    /*
     * Gamepad (W3 Standard Gamepad + extras)
     */
    /// Exit / Main / Home / Mode
    Exit(bool),
    /// Left Menu / Back / Select / Minus / Stop
    MenuL(bool),
    /// Righ Menu / Forward / Start / Plus / Play
    MenuR(bool),

    /// A / 1 / 4 / Circle.  Action A (Primary action).
    ActionA(bool),
    /// B / 2 / 3 / Cross.  Action B (Secondary action).
    ActionB(bool),
    /// C.  Action C (Tertiary action).
    ActionC(bool),

    /// Y / X / Square.  Action H (Horizontal action).
    ActionH(bool),
    /// X / Y / Triangle.  Action V (Vertical action).
    ActionV(bool),
    /// Z (in 6-button layout).  Action D.
    ActionD(bool),

    /// D-Pad Up
    Up(bool),
    /// D-Pad Down
    Down(bool),
    /// D-Pad Right
    Right(bool),
    /// D-Pad Left
    Left(bool),

    /// Left shoulder button (near button if no trigger)
    BumperL(bool),
    /// Right shoulder button (near button if no trigger)
    BumperR(bool),

    /// Left Bumper Trigger (far button if no trigger) - between 0.0 and 1.0
    TriggerL(f64),
    /// Right Bumper Trigger (far button if no trigger) - between 0.0 and 1.0
    TriggerR(f64),

    /// Thumb Push Button On Main / Left Joystick
    Joy(bool),
    /// Thumb Push Button On Camera / Right Joystick
    Cam(bool),

    /// Main stick horizontal axis (A / D) - between -1.0 and 1.0
    JoyX(f64),
    /// Main stick vertical / depth axis (W / S) - between -1.0 and 1.0
    JoyY(f64),
    /// Main stick rotation / yaw axis - between -1.0 and 1.0
    JoyZ(f64),

    /// Secondary stick X axis (Mouse X Position) - between -1.0 and 1.0
    CamX(f64),
    /// Secondary stick Y axis (Mouse Y Position) - between -1.0 and 1.0
    CamY(f64),
    /// Secondary stick Z axis - between -1.0 and 1.0
    CamZ(f64),

    /// Back left grip button (upper if there are two)
    PaddleLeft(bool),
    /// Back right grip button (upper if there are two)
    PaddleRight(bool),
    /// Left Pinky Button / Back lower right grip button
    PinkyLeft(bool),
    /// Right Pinky Button / Back lower left grip button
    PinkyRight(bool),

    /*
     * Joystick (For cars and boats)
     */
    /// Numbered or unlabeled programmable action buttons (If unlabelled,
    /// prefer numbering from left to right, upper to lower)
    Number(i8, bool),

    /// Steering wheel
    Wheel(f64),
    /// Brake pedal
    Brake(f64),
    /// Gas pedal
    Gas(f64),
    /// Ship rudder
    Rudder(f64),

    /*
     * Flightstick
     */
    /// Flightstick trigger button on the back.
    Trigger(bool),

    /// Flightstick Hat Up
    HatUp(bool),
    /// Flightstick Hat Down
    HatDown(bool),
    /// Flightstick Hat Left
    HatLeft(bool),
    /// Flightstick Hat Right
    HatRight(bool),

    /* OLD Events: FIXME Move Up */

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
            Disconnect => write!(f, "Controller Disconnected"),
            Exit(p) => write!(f, "Exit {}", pushed(p)),
            MenuL(p) => write!(f, "MenuL {}", pushed(p)),
            MenuR(p) => write!(f, "MenuR {}", pushed(p)),
            ActionA(p) => write!(f, "ActionA {}", pushed(p)),
            ActionB(p) => write!(f, "ActionB {}", pushed(p)),
            ActionC(p) => write!(f, "ActionC {}", pushed(p)),
            ActionH(p) => write!(f, "ActionH {}", pushed(p)),
            ActionV(p) => write!(f, "ActionV {}", pushed(p)),
            ActionD(p) => write!(f, "ActionD {}", pushed(p)),
            Up(p) => write!(f, "Up {}", pushed(p)),
            Down(p) => write!(f, "Down {}", pushed(p)),
            Right(p) => write!(f, "Right {}", pushed(p)),
            Left(p) => write!(f, "Left {}", pushed(p)),
            BumperL(p) => write!(f, "BumperL {}", pushed(p)),
            BumperR(p) => write!(f, "BumperR {}", pushed(p)),
            TriggerL(v) => write!(f, "TriggerL {}", v),
            TriggerR(v) => write!(f, "TriggerR {}", v),
            Joy(p) => write!(f, "Joy {}", pushed(p)),
            Cam(p) => write!(f, "Cam {}", pushed(p)),
            JoyX(v) => write!(f, "JoyX {}", v),
            JoyY(v) => write!(f, "JoyY {}", v),
            JoyZ(v) => write!(f, "JoyZ {}", v),
            CamX(v) => write!(f, "CamX {}", v),
            CamY(v) => write!(f, "CamY {}", v),
            CamZ(v) => write!(f, "CamZ {}", v),
            PaddleLeft(p) => write!(f, "PaddleLeft {}", pushed(p)),
            PaddleRight(p) => write!(f, "PaddleRight {}", pushed(p)),
            PinkyLeft(p) => write!(f, "PinkyLeft {}", pushed(p)),
            PinkyRight(p) => write!(f, "PinkyRight {}", pushed(p)),
            Number(n, p) => write!(f, "Number({}) {}", n, pushed(p)),
            Wheel(v) => write!(f, "Wheel {}", v),
            Brake(v) => write!(f, "Brake {}", v),
            Gas(v) => write!(f, "Gas {}", v),
            Rudder(v) => write!(f, "Rudder {}", v),
            Trigger(p) => write!(f, "Trigger {}", pushed(p)),
            HatUp(p) => write!(f, "HatUp {}", pushed(p)),
            HatDown(p) => write!(f, "HatDown {}", pushed(p)),
            HatLeft(p) => write!(f, "HatLeft {}", pushed(p)),
            HatRight(p) => write!(f, "HatRight {}", pushed(p)),

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
