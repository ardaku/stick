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
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum Event {
    /// Controller unplugged.
    Disconnect,
    /// Exit / Main / Home / Mode
    Exit(bool),
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
    /// Left Menu / Back / Select / Minus / Stop
    MenuL(bool),
    /// Right Menu / Forward / Start / Plus / Play
    MenuR(bool),
    /// Thumb Push Button On Main / Left Joystick
    Joy(bool),
    /// Thumb Push Button On Camera / Right Joystick
    Cam(bool),
    /// Left shoulder button (near button if no trigger)
    BumperL(bool),
    /// Right shoulder button (near button if no trigger)
    BumperR(bool),
    /// Left Bumper Trigger (far button if no trigger) - between 0.0 and 1.0
    TriggerL(f64),
    /// Right Bumper Trigger (far button if no trigger) - between 0.0 and 1.0
    TriggerR(f64),
    /// D-Pad Up
    Up(bool),
    /// D-Pad Down
    Down(bool),
    /// D-Pad Left
    Left(bool),
    /// D-Pad Right
    Right(bool),
    /// POV/Main Hat Left
    PovUp(bool),
    /// POV/Main Hat Down
    PovDown(bool),
    /// POV/Main Hat Left
    PovLeft(bool),
    /// POV/Main Hat Right
    PovRight(bool),
    /// Extra Hat Up
    HatUp(bool),
    /// Extra Hat Down
    HatDown(bool),
    /// Extra Hat Left
    HatLeft(bool),
    /// Extra Hat Right
    HatRight(bool),
    /// Trim Hat Up
    TrimUp(bool),
    /// Trim Hat Down
    TrimDown(bool),
    /// Trim Hat Left
    TrimLeft(bool),
    /// Trim Hat Right
    TrimRight(bool),
    /// Mic Hat Up
    MicUp(bool),
    /// Mic Hat Down
    MicDown(bool),
    /// Mic Hat Left
    MicLeft(bool),
    /// Mic Hat Right
    MicRight(bool),
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
    /// Slew Control - between 0.0 and 1.0
    Slew(f64),
    /// Stationary throttle (0.0 is forward, 1.0 is backward)
    Throttle(f64),
    /// Left stationary throttle (0.0 is forward, 1.0 is backward)
    ThrottleL(f64),
    /// Right stationary throttle (0.0 is forward, 1.0 is backward)
    ThrottleR(f64),
    /// Volume axis (0.0 is off, 1.0 is full volume)
    Volume(f64),
    /// Steering wheel - between 0.0 and 1.0
    Wheel(f64),
    /// Ship rudder - between 0.0 and 1.0
    Rudder(f64),
    /// Gas Pedal - between 0.0 and 1.0
    Gas(f64),
    /// Brake Pedal - between 0.0 and 1.0
    Brake(f64),
    /// Mic Hat Push Button
    MicPush(bool),
    /// Flightstick trigger button on the back.
    Trigger(bool),
    /// Flightstick Side Bumper Button
    Bumper(bool),
    /// Flightstick Top Middle Action Button
    ActionM(bool),
    /// Flightstick Top Left Action Button
    ActionL(bool),
    /// Flightstick Top Right Action Button
    ActionR(bool),
    /// Pinky Button
    Pinky(bool),
    /// Pinky three-way switch Forward.
    PinkyForward(bool),
    /// Pinky three-way switch Backward.
    PinkyBackward(bool),
    /// Flaps three-way switch Forward.
    /// - `true` - Forward (Up)
    /// - `false` - Neutral (Maneuver)
    FlapsUp(bool),
    /// Flaps three-way switch Backward.
    /// - `true` - Backward (Down)
    /// - `false` - Neutral (Maneuver)
    FlapsDown(bool),
    /// Boat three-way switch Forward.
    BoatForward(bool),
    /// Boat three-way switch Backward.
    BoatBackward(bool),
    /// Autopilot three-way switch Forward.
    /// - `true` - Forward (Path)
    /// - `false` - Neutral (Altitude / Heading)
    AutopilotPath(bool),
    /// Autopilot three-way switch Backward.
    /// - `true` - Backward (Alt)
    /// - `false` - Neutral (Altitude / Heading)
    AutopilotAlt(bool),
    /// Left Engine Operate three-way switch Backward.
    /// - `true` - Backward (Motor)
    /// - `false` - Neutral (Normal)
    EngineMotorL(bool),
    /// Right Engine Operate three-way switch Backward.
    /// - `true` - Backward (Motor)
    /// - `false` - Neutral (Normal)
    EngineMotorR(bool),
    /// Engine Fuel Flow Left two-way switch
    /// - `true` - Normal
    /// - `false` - Override
    EngineFuelFlowL(bool),
    /// Engine Fuel Flow Right two-way switch
    /// - `true` - Normal
    /// - `false` - Override
    EngineFuelFlowR(bool),
    /// Left Engine Operate three-way switch Forward.
    /// - `true` - Forward (Ignition)
    /// - `false` - Neutral (Normal)
    EngineIgnitionL(bool),
    /// Right Engine Operate three-way switch Forward.
    /// - `true` - Forward (Ignition)
    /// - `false` - Neutral (Normal)
    EngineIgnitionR(bool),
    /// Speedbrake three-way switch Backward.
    SpeedbrakeBackward(bool),
    /// Speedbrake three-way switch Forward.
    SpeedbrakeForward(bool),
    /// China hat three-way switch Backward.
    ChinaBackward(bool),
    /// China hat three-way switch Forward.
    ChinaForward(bool),
    /// APU (Auxiliary Power Unit) two-way switch
    /// - `true` - Start
    /// - `false` - Off
    Apu(bool),
    /// Radar Altimeter two-way switch (Altitude measurements)
    /// - `true` - Normal
    /// - `false` - Disabled
    RadarAltimeter(bool),
    /// Landing Gear Horn Silence Button
    LandingGearSilence(bool),
    /// EAC (Enhanced Attitude Control - Autopilot) two-way switch
    /// - `true` - Arm
    /// - `false` - Off
    Eac(bool),
    /// Autopilot Toggle Button
    AutopilotToggle(bool),
    /// Throttle button (Left)
    ThrottleButton(bool),
    /// Mouse delta position horizontal - between -1.0 and 1.0
    MouseX(f64),
    /// Mouse delta position vertical - between -1.0 and 1.0
    MouseY(f64),
    /// Mouse primary button
    Mouse(bool),
    /// Numbered or unlabeled programmable action buttons (If unlabelled,
    /// prefer numbering from left to right, upper to lower)
    Number(i8, bool),
    /// Back left grip button (upper if there are two)
    PaddleLeft(bool),
    /// Back right grip button (upper if there are two)
    PaddleRight(bool),
    /// Left Pinky Button / Back lower right grip button
    PinkyLeft(bool),
    /// Right Pinky Button / Back lower left grip button
    PinkyRight(bool),
    /// Context Menu Button on a mouse (Right Click)
    Context(bool),
    /// DPI Button on a mouse
    Dpi(bool),
    /// Scroll Wheel X on a mouse - between -1.0 and 1.0
    ScrollX(f64),
    /// Scroll Wheel Y on a mouse - between -1.0 and 1.0
    ScrollY(f64),
    /// Scroll Button on a mouse
    Scroll(bool),
    /// Horizontal axis under the action buttons - between -1.0 and 1.0
    ActionWheelX(f64),
    /// Vertical axis under the action buttons - between -1.0 and 1.0
    ActionWheelY(f64),
}

impl Event {
    #[inline(always)]
    pub(crate) fn remap(self, new_id: u8) -> Self {
        Self::from_id(new_id, self.to_id().1)
    }

    #[inline(always)]
    fn from_id(id: u8, value: f64) -> Self {
        match id {
            0x00 => Event::Disconnect,
            0x01 => Event::Exit(value != 0.0),
            0x02 => Event::ActionA(value != 0.0),
            0x03 => Event::ActionB(value != 0.0),
            0x04 => Event::ActionC(value != 0.0),
            0x05 => Event::ActionH(value != 0.0),
            0x06 => Event::ActionV(value != 0.0),
            0x07 => Event::ActionD(value != 0.0),
            0x08 => Event::MenuL(value != 0.0),
            0x09 => Event::MenuR(value != 0.0),
            0x0A => Event::Joy(value != 0.0),
            0x0B => Event::Cam(value != 0.0),
            0x0C => Event::BumperL(value != 0.0),
            0x0D => Event::BumperR(value != 0.0),
            0x0E => Event::TriggerL(value),
            0x0F => Event::TriggerR(value),
            0x10 => Event::Up(value != 0.0),
            0x11 => Event::Down(value != 0.0),
            0x12 => Event::Left(value != 0.0),
            0x13 => Event::Right(value != 0.0),
            0x14 => Event::HatUp(value != 0.0),
            0x15 => Event::HatDown(value != 0.0),
            0x16 => Event::HatLeft(value != 0.0),
            0x17 => Event::HatRight(value != 0.0),
            0x18 => Event::MicUp(value != 0.0),
            0x19 => Event::MicDown(value != 0.0),
            0x1A => Event::MicLeft(value != 0.0),
            0x1B => Event::MicRight(value != 0.0),
            0x1C => Event::PovUp(value != 0.0),
            0x1D => Event::PovDown(value != 0.0),
            0x1E => Event::PovLeft(value != 0.0),
            0x1F => Event::PovRight(value != 0.0),
            0x20 => Event::JoyX(value),
            0x21 => Event::JoyY(value),
            0x22 => Event::JoyZ(value),
            0x23 => Event::CamX(value),
            0x24 => Event::CamY(value),
            0x25 => Event::CamZ(value),
            0x26 => Event::Slew(value),
            0x27 => Event::Throttle(value),
            0x28 => Event::ThrottleL(value),
            0x29 => Event::ThrottleR(value),
            0x2A => Event::Volume(value),
            0x2B => Event::Wheel(value),
            0x2C => Event::Rudder(value),
            0x2D => Event::Gas(value),
            0x2E => Event::Brake(value),
            0x2F => Event::MicPush(value != 0.0),
            0x30 => Event::Trigger(value != 0.0),
            0x31 => Event::Bumper(value != 0.0),
            0x32 => Event::ActionL(value != 0.0),
            0x33 => Event::ActionM(value != 0.0),
            0x34 => Event::ActionR(value != 0.0),
            0x35 => Event::Pinky(value != 0.0),
            0x36 => Event::PinkyForward(value != 0.0),
            0x37 => Event::PinkyBackward(value != 0.0),
            0x38 => Event::FlapsUp(value != 0.0),
            0x39 => Event::FlapsDown(value != 0.0),
            0x3A => Event::BoatForward(value != 0.0),
            0x3B => Event::BoatBackward(value != 0.0),
            0x3C => Event::AutopilotPath(value != 0.0),
            0x3D => Event::AutopilotAlt(value != 0.0),
            0x3E => Event::EngineMotorL(value != 0.0),
            0x3F => Event::EngineMotorR(value != 0.0),
            0x40 => Event::EngineFuelFlowL(value != 0.0),
            0x41 => Event::EngineFuelFlowR(value != 0.0),
            0x42 => Event::EngineIgnitionL(value != 0.0),
            0x43 => Event::EngineIgnitionR(value != 0.0),
            0x44 => Event::SpeedbrakeBackward(value != 0.0),
            0x45 => Event::SpeedbrakeForward(value != 0.0),
            0x46 => Event::ChinaBackward(value != 0.0),
            0x47 => Event::ChinaForward(value != 0.0),
            0x48 => Event::Apu(value != 0.0),
            0x49 => Event::RadarAltimeter(value != 0.0),
            0x4A => Event::LandingGearSilence(value != 0.0),
            0x4B => Event::Eac(value != 0.0),
            0x4C => Event::AutopilotToggle(value != 0.0),
            0x4D => Event::ThrottleButton(value != 0.0),
            0x4E => Event::MouseX(value),
            0x4F => Event::MouseY(value),
            0x50 => Event::Mouse(value != 0.0),
            0x51 => Event::PaddleLeft(value != 0.0),
            0x52 => Event::PaddleRight(value != 0.0),
            0x53 => Event::PinkyLeft(value != 0.0),
            0x54 => Event::PinkyRight(value != 0.0),
            0x55 => Event::Context(value != 0.0),
            0x56 => Event::Dpi(value != 0.0),
            0x57 => Event::ScrollX(value),
            0x58 => Event::ScrollY(value),
            0x59 => Event::Scroll(value != 0.0),
            0x5A => Event::TrimUp(value != 0.0),
            0x5B => Event::TrimDown(value != 0.0),
            0x5C => Event::TrimLeft(value != 0.0),
            0x5D => Event::TrimRight(value != 0.0),
            0x5E => Event::ActionWheelX(value),
            0x5F => Event::ActionWheelY(value),
            n => Event::Number((n & !0x80) as i8, value != 0.0),
        }
    }

    #[inline(always)]
    pub(crate) fn to_id(self) -> (u8, f64) {
        use Event::*;
        match self {
            Disconnect => (0x00, f64::NAN),
            Exit(p) => (0x01, f64::from(u8::from(p))),
            ActionA(p) => (0x02, f64::from(u8::from(p))),
            ActionB(p) => (0x03, f64::from(u8::from(p))),
            ActionC(p) => (0x04, f64::from(u8::from(p))),
            ActionH(p) => (0x05, f64::from(u8::from(p))),
            ActionV(p) => (0x06, f64::from(u8::from(p))),
            ActionD(p) => (0x07, f64::from(u8::from(p))),
            MenuL(p) => (0x08, f64::from(u8::from(p))),
            MenuR(p) => (0x09, f64::from(u8::from(p))),
            Joy(p) => (0x0A, f64::from(u8::from(p))),
            Cam(p) => (0x0B, f64::from(u8::from(p))),
            BumperL(p) => (0x0C, f64::from(u8::from(p))),
            BumperR(p) => (0x0D, f64::from(u8::from(p))),
            TriggerL(t) => (0x0E, t),
            TriggerR(t) => (0x0F, t),
            Up(p) => (0x10, f64::from(u8::from(p))),
            Down(p) => (0x11, f64::from(u8::from(p))),
            Left(p) => (0x12, f64::from(u8::from(p))),
            Right(p) => (0x13, f64::from(u8::from(p))),
            HatUp(p) => (0x14, f64::from(u8::from(p))),
            HatDown(p) => (0x15, f64::from(u8::from(p))),
            HatLeft(p) => (0x16, f64::from(u8::from(p))),
            HatRight(p) => (0x17, f64::from(u8::from(p))),
            MicUp(p) => (0x18, f64::from(u8::from(p))),
            MicDown(p) => (0x19, f64::from(u8::from(p))),
            MicLeft(p) => (0x1A, f64::from(u8::from(p))),
            MicRight(p) => (0x1B, f64::from(u8::from(p))),
            PovUp(p) => (0x1C, f64::from(u8::from(p))),
            PovDown(p) => (0x1D, f64::from(u8::from(p))),
            PovLeft(p) => (0x1E, f64::from(u8::from(p))),
            PovRight(p) => (0x1F, f64::from(u8::from(p))),
            JoyX(v) => (0x20, v),
            JoyY(v) => (0x21, v),
            JoyZ(v) => (0x22, v),
            CamX(v) => (0x23, v),
            CamY(v) => (0x24, v),
            CamZ(v) => (0x25, v),
            Slew(t) => (0x26, t),
            Throttle(t) => (0x27, t),
            ThrottleL(t) => (0x28, t),
            ThrottleR(t) => (0x29, t),
            Volume(t) => (0x2A, t),
            Wheel(t) => (0x2B, t),
            Rudder(t) => (0x2C, t),
            Gas(t) => (0x2D, t),
            Brake(t) => (0x2E, t),
            MicPush(p) => (0x2F, f64::from(u8::from(p))),
            Trigger(p) => (0x30, f64::from(u8::from(p))),
            Bumper(p) => (0x31, f64::from(u8::from(p))),
            ActionL(p) => (0x32, f64::from(u8::from(p))),
            ActionM(p) => (0x33, f64::from(u8::from(p))),
            ActionR(p) => (0x34, f64::from(u8::from(p))),
            Pinky(p) => (0x35, f64::from(u8::from(p))),
            PinkyForward(p) => (0x36, f64::from(u8::from(p))),
            PinkyBackward(p) => (0x37, f64::from(u8::from(p))),
            FlapsUp(p) => (0x38, f64::from(u8::from(p))),
            FlapsDown(p) => (0x39, f64::from(u8::from(p))),
            BoatForward(p) => (0x3A, f64::from(u8::from(p))),
            BoatBackward(p) => (0x3B, f64::from(u8::from(p))),
            AutopilotPath(p) => (0x3C, f64::from(u8::from(p))),
            AutopilotAlt(p) => (0x3D, f64::from(u8::from(p))),
            EngineMotorL(p) => (0x3E, f64::from(u8::from(p))),
            EngineMotorR(p) => (0x3F, f64::from(u8::from(p))),
            EngineFuelFlowL(p) => (0x40, f64::from(u8::from(p))),
            EngineFuelFlowR(p) => (0x41, f64::from(u8::from(p))),
            EngineIgnitionL(p) => (0x42, f64::from(u8::from(p))),
            EngineIgnitionR(p) => (0x43, f64::from(u8::from(p))),
            SpeedbrakeBackward(p) => (0x44, f64::from(u8::from(p))),
            SpeedbrakeForward(p) => (0x45, f64::from(u8::from(p))),
            ChinaBackward(p) => (0x46, f64::from(u8::from(p))),
            ChinaForward(p) => (0x47, f64::from(u8::from(p))),
            Apu(p) => (0x48, f64::from(u8::from(p))),
            RadarAltimeter(p) => (0x49, f64::from(u8::from(p))),
            LandingGearSilence(p) => (0x4A, f64::from(u8::from(p))),
            Eac(p) => (0x4B, f64::from(u8::from(p))),
            AutopilotToggle(p) => (0x4C, f64::from(u8::from(p))),
            ThrottleButton(p) => (0x4D, f64::from(u8::from(p))),
            MouseX(v) => (0x4E, v),
            MouseY(v) => (0x4F, v),
            Mouse(p) => (0x50, f64::from(u8::from(p))),
            Number(n, p) => (n as u8 | 0x80, f64::from(u8::from(p))),
            PaddleLeft(p) => (0x51, f64::from(u8::from(p))),
            PaddleRight(p) => (0x52, f64::from(u8::from(p))),
            PinkyLeft(p) => (0x53, f64::from(u8::from(p))),
            PinkyRight(p) => (0x54, f64::from(u8::from(p))),
            Context(p) => (0x55, f64::from(u8::from(p))),
            Dpi(p) => (0x56, f64::from(u8::from(p))),
            ScrollX(v) => (0x57, v),
            ScrollY(v) => (0x58, v),
            Scroll(p) => (0x59, f64::from(u8::from(p))),
            TrimUp(p) => (0x5A, f64::from(u8::from(p))),
            TrimDown(p) => (0x5B, f64::from(u8::from(p))),
            TrimLeft(p) => (0x5C, f64::from(u8::from(p))),
            TrimRight(p) => (0x5D, f64::from(u8::from(p))),
            ActionWheelX(v) => (0x5E, v),
            ActionWheelY(v) => (0x5F, v),
        }
    }
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
            TrimUp(p) => write!(f, "TrimUp {}", pushed(p)),
            TrimDown(p) => write!(f, "TrimDown {}", pushed(p)),
            TrimLeft(p) => write!(f, "TrimLeft {}", pushed(p)),
            TrimRight(p) => write!(f, "TrimRight {}", pushed(p)),
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
            ThrottleButton(p) => write!(f, "ThrottleButton {}", pushed(p)),
            EngineFuelFlowL(t) => write!(f, "EngineFuelFlowL {}", two(t)),
            EngineFuelFlowR(t) => write!(f, "EngineFuelFlowR {}", two(t)),
            Eac(t) => write!(f, "Eac {}", two(t)),
            RadarAltimeter(t) => write!(f, "RadarAltimeter {}", two(t)),
            Apu(t) => write!(f, "Apu {}", two(t)),
            AutopilotPath(p) => write!(f, "AutopilotPath {}", sw(p)),
            AutopilotAlt(p) => write!(f, "AutopilotAlt {}", sw(p)),
            FlapsUp(p) => write!(f, "FlapsUp {}", sw(p)),
            FlapsDown(p) => write!(f, "FlapsDown {}", sw(p)),
            EngineIgnitionL(p) => write!(f, "EngineIgnitionL {}", sw(p)),
            EngineMotorL(p) => write!(f, "EngineMotorL {}", sw(p)),
            EngineIgnitionR(p) => write!(f, "EngineIgnitionR {}", sw(p)),
            EngineMotorR(p) => write!(f, "EngineMotorR {}", sw(p)),
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
            Mouse(p) => write!(f, "Mouse {}", pushed(p)),
            Context(p) => write!(f, "Context {}", pushed(p)),
            ScrollX(v) => write!(f, "ScrollX {}", v),
            ScrollY(v) => write!(f, "ScrollY {}", v),
            Scroll(p) => write!(f, "Scroll {}", pushed(p)),
            Volume(v) => write!(f, "Volume {}", v),
            Bumper(p) => write!(f, "Bumper {}", pushed(p)),
            ActionM(p) => write!(f, "ActionM {}", pushed(p)),
            ActionL(p) => write!(f, "ActionL {}", pushed(p)),
            ActionR(p) => write!(f, "ActionR {}", pushed(p)),
            Pinky(p) => write!(f, "Pinky {}", pushed(p)),
            ActionWheelX(v) => write!(f, "ActionWheelX {}", v),
            ActionWheelY(v) => write!(f, "ActionWheelY {}", v),
        }
    }
}
