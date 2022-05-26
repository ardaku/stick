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

use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::Event;

#[repr(i8)]
enum Btn {
    Exit = 0,
    MenuL = 1,
    MenuR = 2,
    ActionA = 3,
    ActionB = 4,
    ActionC = 5,
    ActionH = 6,
    ActionV = 7,
    ActionD = 8,
    Up = 9,
    Down = 10,
    Right = 11,
    Left = 12,
    BumperL = 13,
    BumperR = 14,
    Joy = 15,
    Cam = 16,
    PaddleLeft = 17,
    PaddleRight = 18,
    PinkyLeft = 19,
    PinkyRight = 20,
    Trigger = 21,
    HatUp = 22,
    HatDown = 23,
    HatRight = 24,
    HatLeft = 25,
    MicUp = 26,
    MicDown = 27,
    MicRight = 28,
    MicLeft = 29,
    PovUp = 30,
    PovDown = 31,
    PovRight = 32,
    PovLeft = 33,
    MicPush = 34,
    ActionL = 35,
    ActionR = 36,
    Bumper = 37,
    ActionM = 38,
    Pinky = 39,
    PinkyForward = 40,
    PinkyBackward = 41,
    FlapsUp = 42,
    FlapsDown = 43,
    BoatForward = 44,
    BoatBackward = 45,
    AutopilotPath = 46,
    AutopilotAlt = 47,
    EngineMotorL = 48,
    EngineMotorR = 49,
    EngineFuelFlowL = 50,
    EngineFuelFlowR = 51,
    EngineIgnitionL = 52,
    EngineIgnitionR = 53,
    SpeedbrakeBackward = 54,
    SpeedbrakeForward = 55,
    ChinaBackward = 56,
    ChinaForward = 57,
    Apu = 58,
    RadarAltimeter = 59,
    LandingGearSilence = 60,
    Eac = 61,
    AutopilotToggle = 62,
    ThrottleButton = 63,
    Mouse = 64,
    Scroll = 65,
    Context = 66,
    Dpi = 67,
    TrimUp = 68,
    TrimDown = 69,
    TrimRight = 70,
    TrimLeft = 71,
    Enter = 72,
    Sleep = 73,
    Eject = 74,
    Mode = 75,
    Lang1 = 76,
    Lang2 = 77,
    Home = 78,
}

#[repr(i8)]
enum Axs {
    TriggerL = 0,
    TriggerR = 1,
    JoyX = 2,
    JoyY = 3,
    JoyZ = 4,
    CamX = 5,
    CamY = 6,
    CamZ = 7,
    Wheel = 8,
    Brake = 9,
    Gas = 10,
    Rudder = 11,
    Slew = 12,
    Throttle = 13,
    ThrottleL = 14,
    ThrottleR = 15,
    Volume = 16,
    MouseX = 17,
    MouseY = 18,
    ScrollX = 19,
    ScrollY = 20,
    ActionWheelX = 21,
    ActionWheelY = 22,
    Count, // Inferred correctly as long as it's last
}

#[derive(Debug)]
struct Map {
    deadzone: f64,
    scale: f64,
    max: i32,
    min: i32,
    out: u8,
}

#[derive(Debug)]
struct Info {
    name: String,
    maps: HashMap<u8, Map>,
    type_: char,
}

impl Default for Info {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            maps: HashMap::new(),
            type_: 'w',
        }
    }
}

/// Controller remapping information
#[derive(Debug)]
pub struct Remap(HashMap<u64, Rc<Info>>);

impl Default for Remap {
    fn default() -> Self {
        Self::new()
    }
}

impl Remap {
    /// Create new remapper.
    #[allow(unused_mut, clippy::let_and_return)]
    pub fn new() -> Self {
        let mut remapper = Remap(HashMap::new());
        #[cfg(all(feature = "gcdb", target_os = "linux"))]
        {
            let data = include_str!("../sdlgc_linux.sdb");
            remapper = remapper.load(data).unwrap();
        }
        #[cfg(all(feature = "sdb", target_os = "linux"))]
        {
            let data = include_str!("../remap_linux.sdb");
            remapper = remapper.load(data).unwrap();
        }
        remapper
    }

    /// Load a custom re-mapping.
    pub fn load(mut self, data: &str) -> Option<Remap> {
        // Controllers
        for line in data.lines() {
            let id = u64::from_str_radix(&line[..16], 16).ok()?;
            let tab = line.find('\t')?;
            let name = line[16..tab].to_string();
            let type_ = line.get(tab + 1..tab + 2)?.chars().next()?;
            let mut maps = HashMap::new();

            // Events
            for event in line.get(tab + 2..)?.split(';') {
                let in_ = u8::from_str_radix(event.get(0..2)?, 16).ok()?;
                let out = u8::from_str_radix(event.get(2..4)?, 16).ok()?;

                // Tweaks
                let mut cursor = 4;
                let mut deadzone = f64::NAN;
                let mut scale = f64::NAN;
                let mut max: i32 = 0;
                let mut min: i32 = 0;
                while let Some(tweak) = event.get(cursor..)?.chars().next() {
                    match tweak {
                        'd' => {
                            let end = event
                                .get(cursor + 1..)?
                                .find(char::is_lowercase)
                                .unwrap_or(event.get(cursor + 1..)?.len());
                            deadzone = event
                                .get(cursor + 1..cursor + 1 + end)?
                                .parse::<f64>()
                                .ok()?;
                            cursor += end + 1;
                        }
                        's' => {
                            let end = event
                                .get(cursor + 1..)?
                                .find(char::is_lowercase)
                                .unwrap_or(event.get(cursor + 1..)?.len());
                            scale = event
                                .get(cursor + 1..cursor + 1 + end)?
                                .parse::<f64>()
                                .ok()?
                                .recip();
                            cursor += end + 1;
                        }
                        'a' => {
                            let end = event
                                .get(cursor + 1..)?
                                .find(char::is_lowercase)
                                .unwrap_or(event.get(cursor + 1..)?.len());
                            max = event
                                .get(cursor + 1..cursor + 1 + end)?
                                .parse::<i32>()
                                .ok()?;
                            cursor += end + 1;
                        }
                        'i' => {
                            let end = event
                                .get(cursor + 1..)?
                                .find(char::is_lowercase)
                                .unwrap_or(event.get(cursor + 1..)?.len());
                            min = event
                                .get(cursor + 1..cursor + 1 + end)?
                                .parse::<i32>()
                                .ok()?;
                            cursor += end + 1;
                        }
                        _ => return None,
                    }
                }

                maps.insert(
                    in_,
                    Map {
                        deadzone,
                        scale,
                        max,
                        min,
                        out,
                    },
                );
            }

            self.0.insert(id, Rc::new(Info { name, maps, type_ }));
        }

        Some(self)
    }
}

/// A gamepad, flightstick, or other controller.
pub struct Controller {
    // Shared remapping.
    remap: Rc<Info>,
    //
    raw: Box<dyn crate::raw::Controller>,
    // Button states
    btns: u128,
    // Number button states
    nums: u128,
    // Axis states:
    axis: [f64; Axs::Count as usize],
}

impl Debug for Controller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Controller(\"{}\")", self.name())
    }
}

impl Controller {
    #[allow(unused)]
    pub(crate) fn new(
        raw: Box<dyn crate::raw::Controller>,
        remap: &Remap,
    ) -> Self {
        let btns = 0;
        let nums = 0;
        let axis = [0.0; Axs::Count as usize];
        let remap = remap.0.get(&raw.id()).cloned().unwrap_or_default();
        Self {
            remap,
            raw,
            btns,
            nums,
            axis,
        }
    }

    /// Get a unique identifier for the specific model of gamepad.
    pub fn id(&self) -> u64 {
        self.raw.id()
    }

    /// Get the name of this Pad.
    pub fn name(&self) -> &str {
        self.raw.name()
    }

    /// Turn on/off haptic force feedback.
    ///
    /// Takes either an `f32` for mono power or `(f32, f32)` for directional
    /// power.  Power will be clamped between 0.0 (off) and 1.0 (maximum power).
    ///
    /// The first `f32` in directional power is typically low frequency and is
    /// located on the left, and the second is typically high frequency and is
    /// located on the right (controllers may vary).
    pub fn rumble<R: Rumble>(&mut self, power: R) {
        self.raw.rumble(power.left(), power.right());
    }

    fn button(&mut self, b: Btn, f: fn(bool) -> Event, p: bool) -> Poll<Event> {
        let b = 1u128 << b as i8;
        if (self.btns & b != 0) == p {
            Poll::Pending
        } else {
            self.btns ^= b;
            Poll::Ready(f(p))
        }
    }

    fn number(
        &mut self,
        n: i8,
        f: fn(i8, bool) -> Event,
        p: bool,
    ) -> Poll<Event> {
        let b = 1u128 << n;
        if (self.nums & b != 0) == p {
            Poll::Pending
        } else {
            self.nums ^= b;
            Poll::Ready(f(n, p))
        }
    }

    #[allow(clippy::float_cmp)] // imprecision should be consistent
    fn axis(
        &mut self,
        ev: u8,
        a: Axs,
        f: fn(f64) -> Event,
        v: f64,
    ) -> Poll<Event> {
        let map = self.remap.maps.get(&ev);
        let v = if let Some(map) = map {
            let v = if map.min != 0 || map.max != 0 {
                (((v - f64::from(map.min)) / f64::from(map.max - map.min))
                    * 2.0
                    - 1.0)
                    .clamp(-1.0, 1.0)
            } else {
                self.raw.axis(v).clamp(-1.0, 1.0)
            };
            if !map.deadzone.is_nan() && v.abs() <= map.deadzone {
                0.0
            } else {
                v
            }
        } else {
            self.raw.axis(v).clamp(-1.0, 1.0)
        };
        let axis = a as usize;
        if self.axis[axis] == v {
            Poll::Pending
        } else {
            self.axis[axis] = v;
            Poll::Ready(f(v))
        }
    }

    #[allow(clippy::float_cmp)] // imprecision should be consistent
    fn pressure(
        &mut self,
        ev: u8,
        a: Axs,
        f: fn(f64) -> Event,
        v: f64,
    ) -> Poll<Event> {
        let map = self.remap.maps.get(&ev);
        let v = if let Some(map) = map {
            let v = if map.min != 0 || map.max != 0 {
                ((v - f64::from(map.min)) / f64::from(map.max - map.min))
                    .clamp(0.0, 1.0)
            } else {
                self.raw.pressure(v).clamp(0.0, 1.0)
            };
            if !map.deadzone.is_nan() && v <= map.deadzone {
                0.0
            } else {
                v
            }
        } else {
            self.raw.pressure(v).clamp(0.0, 1.0)
        };
        let axis = a as usize;
        if self.axis[axis] == v {
            Poll::Pending
        } else {
            self.axis[axis] = v;
            Poll::Ready(f(v))
        }
    }

    fn process(&mut self, event: Event) -> Poll<Event> {
        // Do remapping step first.
        let ev = event.to_id().0;
        let event = if let Some(new_id) = self.remap.maps.get(&ev) {
            let event = event.remap(new_id.out);
            if matches!(event, Disconnect) {
                return Poll::Pending;
            }
            event
        } else {
            event
        };
        //
        use Event::*;
        match event {
            Disconnect => Poll::Ready(Disconnect),
            Enter(p) => self.button(Btn::Enter, Enter, p),
            Exit(p) => self.button(Btn::Exit, Exit, p),
            Sleep(p) => self.button(Btn::Sleep, Sleep, p),
            Eject(p) => self.button(Btn::Eject, Eject, p),
            MenuL(p) => self.button(Btn::MenuL, MenuL, p),
            MenuR(p) => self.button(Btn::MenuR, MenuR, p),
            ActionA(p) => self.button(Btn::ActionA, ActionA, p),
            ActionB(p) => self.button(Btn::ActionB, ActionB, p),
            ActionC(p) => self.button(Btn::ActionC, ActionC, p),
            ActionH(p) => self.button(Btn::ActionH, ActionH, p),
            ActionV(p) => self.button(Btn::ActionV, ActionV, p),
            ActionD(p) => self.button(Btn::ActionD, ActionD, p),
            Up(p) => self.button(Btn::Up, Up, p),
            Down(p) => self.button(Btn::Down, Down, p),
            Right(p) => self.button(Btn::Right, Right, p),
            Left(p) => self.button(Btn::Left, Left, p),
            BumperL(p) => self.button(Btn::BumperL, BumperL, p),
            BumperR(p) => self.button(Btn::BumperR, BumperR, p),
            TriggerL(v) => self.pressure(ev, Axs::TriggerL, TriggerL, v),
            TriggerR(v) => self.pressure(ev, Axs::TriggerR, TriggerR, v),
            Joy(p) => self.button(Btn::Joy, Joy, p),
            Cam(p) => self.button(Btn::Cam, Cam, p),
            JoyX(v) => self.axis(ev, Axs::JoyX, JoyX, v),
            JoyY(v) => self.axis(ev, Axs::JoyY, JoyY, v),
            JoyZ(v) => self.axis(ev, Axs::JoyZ, JoyZ, v),
            CamX(v) => self.axis(ev, Axs::CamX, CamX, v),
            CamY(v) => self.axis(ev, Axs::CamY, CamY, v),
            CamZ(v) => self.axis(ev, Axs::CamZ, CamZ, v),
            PaddleLeft(p) => self.button(Btn::PaddleLeft, PaddleLeft, p),
            PaddleRight(p) => self.button(Btn::PaddleRight, PaddleRight, p),
            PinkyLeft(p) => self.button(Btn::PinkyLeft, PinkyLeft, p),
            PinkyRight(p) => self.button(Btn::PinkyRight, PinkyRight, p),
            Number(n, p) => self.number(n, Number, p),
            HatUp(p) => self.button(Btn::HatUp, HatUp, p),
            HatDown(p) => self.button(Btn::HatDown, HatDown, p),
            HatRight(p) => self.button(Btn::HatRight, HatRight, p),
            HatLeft(p) => self.button(Btn::HatLeft, HatLeft, p),
            Trigger(p) => self.button(Btn::Trigger, Trigger, p),
            MicUp(p) => self.button(Btn::MicUp, MicUp, p),
            MicDown(p) => self.button(Btn::MicDown, MicDown, p),
            MicRight(p) => self.button(Btn::MicRight, MicRight, p),
            MicLeft(p) => self.button(Btn::MicLeft, MicLeft, p),
            PovUp(p) => self.button(Btn::PovUp, PovUp, p),
            PovDown(p) => self.button(Btn::PovDown, PovDown, p),
            PovRight(p) => self.button(Btn::PovRight, PovRight, p),
            PovLeft(p) => self.button(Btn::PovLeft, PovLeft, p),
            Slew(v) => self.pressure(ev, Axs::Slew, Slew, v),
            Throttle(v) => self.pressure(ev, Axs::Throttle, Throttle, v),
            ThrottleL(v) => self.pressure(ev, Axs::ThrottleL, ThrottleL, v),
            ThrottleR(v) => self.pressure(ev, Axs::ThrottleR, ThrottleR, v),
            Volume(v) => self.pressure(ev, Axs::Volume, Volume, v),
            Wheel(v) => self.pressure(ev, Axs::Wheel, Wheel, v),
            Rudder(v) => self.pressure(ev, Axs::Rudder, Rudder, v),
            Gas(v) => self.pressure(ev, Axs::Gas, Gas, v),
            Brake(v) => self.pressure(ev, Axs::Brake, Brake, v),
            MicPush(p) => self.button(Btn::MicPush, MicPush, p),
            ActionL(p) => self.button(Btn::ActionL, ActionL, p),
            ActionM(p) => self.button(Btn::ActionM, ActionM, p),
            ActionR(p) => self.button(Btn::ActionR, ActionR, p),
            Bumper(p) => self.button(Btn::Bumper, Bumper, p),
            Pinky(p) => self.button(Btn::Pinky, Pinky, p),
            PinkyForward(p) => self.button(Btn::PinkyForward, PinkyForward, p),
            PinkyBackward(p) => {
                self.button(Btn::PinkyBackward, PinkyBackward, p)
            }
            FlapsUp(p) => self.button(Btn::FlapsUp, FlapsUp, p),
            FlapsDown(p) => self.button(Btn::FlapsDown, FlapsDown, p),
            BoatForward(p) => self.button(Btn::BoatForward, BoatForward, p),
            BoatBackward(p) => self.button(Btn::BoatBackward, BoatBackward, p),
            AutopilotPath(p) => {
                self.button(Btn::AutopilotPath, AutopilotPath, p)
            }
            AutopilotAlt(p) => self.button(Btn::AutopilotAlt, AutopilotAlt, p),
            EngineMotorL(p) => self.button(Btn::EngineMotorL, EngineMotorL, p),
            EngineMotorR(p) => self.button(Btn::EngineMotorR, EngineMotorR, p),
            EngineFuelFlowL(p) => {
                self.button(Btn::EngineFuelFlowL, EngineFuelFlowL, p)
            }
            EngineFuelFlowR(p) => {
                self.button(Btn::EngineFuelFlowR, EngineFuelFlowR, p)
            }
            EngineIgnitionL(p) => {
                self.button(Btn::EngineIgnitionL, EngineIgnitionL, p)
            }
            EngineIgnitionR(p) => {
                self.button(Btn::EngineIgnitionR, EngineIgnitionR, p)
            }
            SpeedbrakeBackward(p) => {
                self.button(Btn::SpeedbrakeBackward, SpeedbrakeBackward, p)
            }
            SpeedbrakeForward(p) => {
                self.button(Btn::SpeedbrakeForward, SpeedbrakeForward, p)
            }
            ChinaBackward(p) => {
                self.button(Btn::ChinaBackward, ChinaBackward, p)
            }
            ChinaForward(p) => self.button(Btn::ChinaForward, ChinaForward, p),
            Apu(p) => self.button(Btn::Apu, Apu, p),
            RadarAltimeter(p) => {
                self.button(Btn::RadarAltimeter, RadarAltimeter, p)
            }
            LandingGearSilence(p) => {
                self.button(Btn::LandingGearSilence, LandingGearSilence, p)
            }
            Eac(p) => self.button(Btn::Eac, Eac, p),
            AutopilotToggle(p) => {
                self.button(Btn::AutopilotToggle, AutopilotToggle, p)
            }
            ThrottleButton(p) => {
                self.button(Btn::ThrottleButton, ThrottleButton, p)
            }
            MouseX(v) => self.axis(ev, Axs::MouseX, MouseX, v),
            MouseY(v) => self.axis(ev, Axs::MouseY, MouseY, v),
            ScrollX(v) => self.axis(ev, Axs::ScrollX, ScrollX, v),
            ScrollY(v) => self.axis(ev, Axs::ScrollY, ScrollY, v),
            Mouse(p) => self.button(Btn::Mouse, Mouse, p),
            Scroll(p) => self.button(Btn::Scroll, Scroll, p),
            Context(p) => self.button(Btn::Context, Context, p),
            Dpi(p) => self.button(Btn::Dpi, Dpi, p),
            TrimUp(p) => self.button(Btn::TrimUp, TrimUp, p),
            TrimDown(p) => self.button(Btn::TrimDown, TrimDown, p),
            TrimLeft(p) => self.button(Btn::TrimLeft, TrimLeft, p),
            TrimRight(p) => self.button(Btn::TrimRight, TrimRight, p),
            ActionWheelX(v) => self.axis(ev, Axs::ActionWheelX, ActionWheelX, v),
            ActionWheelY(v) => self.axis(ev, Axs::ActionWheelY, ActionWheelY, v),
            Mode(p) => self.button(Btn::Mode, Mode, p),
            Lang1(p) => self.button(Btn::Lang1, Lang1, p),
            Lang2(p) => self.button(Btn::Lang2, Lang2, p),
            Home(p) => self.button(Btn::Home, Home, p),
        }
    }
}

impl Future for Controller {
    type Output = Event;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Event> {
        let mut this = self.as_mut();

        if let Poll::Ready(event) = this.raw.poll(cx) {
            let out = Self::process(&mut *this, event);
            if out.is_pending() {
                Self::poll(self, cx)
            } else {
                out
            }
        } else {
            Poll::Pending
        }
    }
}

pub trait Rumble {
    fn left(&self) -> f32;
    fn right(&self) -> f32;
}

impl Rumble for f32 {
    #[inline(always)]
    fn left(&self) -> f32 {
        self.clamp(0.0, 1.0)
    }

    #[inline(always)]
    fn right(&self) -> f32 {
        self.clamp(0.0, 1.0)
    }
}

impl Rumble for (f32, f32) {
    #[inline(always)]
    fn left(&self) -> f32 {
        self.0.clamp(0.0, 1.0)
    }

    #[inline(always)]
    fn right(&self) -> f32 {
        self.1.clamp(0.0, 1.0)
    }
}
