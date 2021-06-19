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
use std::convert::TryInto;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
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
    Count = 12,
}

#[derive(Clone, Debug)]
struct Remapping {
    /// Deadzone for axis
    deadzone: f64,
    /// Input to Output ID Mapping
    maps: HashMap<u16, u16>,
}

impl Default for Remapping {
    fn default() -> Self {
        Self {
            deadzone: f64::NAN,
            maps: HashMap::new(),
        }
    }
}

/// Controller remapping information
#[derive(Default, Debug)]
pub struct Remap(HashMap<u64, Remapping>);

impl Remap {
    /// Create new remapper.
    pub fn new() -> Self {
        Self::default()
    }

    /// Load a custom re-mapping.
    pub fn load(mut self, data: &[u8]) -> Option<Remap> {
        let mut cursor = 0;
        while cursor < data.len() {
            // Read 64-Bit Controller ID.
            let mut id: [u8; 8] =
                data.get(cursor..cursor + 8)?.try_into().unwrap();
            cursor += 8;
            // Unset top 4 bits.
            let should_add = match id[0] & 0xF0 {
                0x10 => cfg!(all(not(target_arch = "wasm32"), linux)),
                0x20 => cfg!(all(not(target_arch = "wasm32"), windows)),
                0x30 => cfg!(target_arch = "wasm32"),
                _ => false, // Error
            };
            id[0] &= 0x0F;
            // Read 64-Bit Deadzone
            let deadzone = f64::from_le_bytes(
                data.get(cursor..cursor + 8)?.try_into().unwrap(),
            );
            cursor += 8;
            // Read 32-Bit Length
            let len = u32::from_le_bytes(
                data.get(cursor..cursor + 4)?.try_into().unwrap(),
            );
            cursor += 4;
            // Go through each mapping.
            let mut maps = HashMap::new();
            for _ in 0..len {
                let io: [u8; 4] = data.get(0..4)?.try_into().unwrap();
                let input = u16::from_le_bytes(io[0..2].try_into().unwrap());
                let output = u16::from_le_bytes(io[2..4].try_into().unwrap());
                maps.insert(input, output);
                cursor += 4;
            }
            // Adjust self
            if should_add {
                self.0.insert(
                    u64::from_ne_bytes(id),
                    Remapping { deadzone, maps },
                );
            }
        }
        Some(self)
    }
}

/// A gamepad, flightstick, or other controller.
pub struct Controller {
    // Shared remapping.
    remap: Remapping,
    //
    raw: crate::ffi::Ctlr,
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
    pub(crate) fn new(raw: crate::ffi::Ctlr, remap: &Remap) -> Self {
        let btns = 0;
        let nums = 0;
        let axis = [0.0; Axs::Count as usize];
        let remap = remap
            .0
            .get(&u64::from_ne_bytes(raw.id()))
            .cloned()
            .unwrap_or_default();
        Self {
            remap,
            raw,
            btns,
            nums,
            axis,
        }
    }

    /// enable or disable event generation. Disable events when the application loses focus
    pub fn enable(flag: bool) {
        crate::ffi::Hub::enable(flag);
    }

    /// Get a unique identifier for the specific model of gamepad.
    pub fn id(&self) -> [u8; 8] {
        self.raw.id()
    }

    /// Get the name of this Pad.
    pub fn name(&self) -> String {
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

    fn hat(
        &mut self,
        b: Btn,
        b2: Btn,
        f: fn(bool) -> Event,
        f2: fn(bool) -> Event,
        p: bool,
    ) -> Poll<Event> {
        let b = 1u128 << b as i8;
        if (self.btns & b != 0) == p {
            if !p {
                self.button(b2, f2, p)
            } else {
                Poll::Pending
            }
        } else {
            self.btns ^= b;
            self.remap(f(p))
        }
    }

    fn button(&mut self, b: Btn, f: fn(bool) -> Event, p: bool) -> Poll<Event> {
        let b = 1u128 << b as i8;
        if (self.btns & b != 0) == p {
            Poll::Pending
        } else {
            self.btns ^= b;
            self.remap(f(p))
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
            self.remap(f(n, p))
        }
    }

    #[allow(clippy::float_cmp)] // imprecision should be consistent
    fn axis(&mut self, a: Axs, f: fn(f64) -> Event, v: f64) -> Poll<Event> {
        let mut v = self.raw.axis(v).clamp(-1.0, 1.0);
        if !self.remap.deadzone.is_nan() && v <= self.remap.deadzone {
            v = 0.0;
        }
        let axis = a as usize;
        if self.axis[axis] == v {
            Poll::Pending
        } else {
            self.axis[axis] = v;
            self.remap(f(v))
        }
    }

    #[allow(clippy::float_cmp)] // imprecision should be consistent
    fn pressure(&mut self, a: Axs, f: fn(f64) -> Event, v: f64) -> Poll<Event> {
        let v = self.raw.pressure(v).clamp(0.0, 1.0);
        let axis = a as usize;
        if self.axis[axis] == v {
            Poll::Pending
        } else {
            self.axis[axis] = v;
            self.remap(f(v))
        }
    }

    fn map_to_button(input: u16, p: bool) -> Poll<Event> {
        Poll::Ready(match input - 0x0080 {
            _x if _x == Btn::Exit as u16 => Event::Exit(p),
            _x if _x == Btn::MenuL as u16 => Event::MenuL(p),
            _x if _x == Btn::MenuR as u16 => Event::MenuR(p),
            _x if _x == Btn::ActionA as u16 => Event::ActionA(p),
            _x if _x == Btn::ActionB as u16 => Event::ActionB(p),
            _x if _x == Btn::ActionC as u16 => Event::ActionC(p),
            _x if _x == Btn::ActionH as u16 => Event::ActionH(p),
            _x if _x == Btn::ActionV as u16 => Event::ActionV(p),
            _x if _x == Btn::ActionD as u16 => Event::ActionD(p),
            _x if _x == Btn::Up as u16 => Event::Up(p),
            _x if _x == Btn::Down as u16 => Event::Down(p),
            _x if _x == Btn::Right as u16 => Event::Right(p),
            _x if _x == Btn::Left as u16 => Event::Left(p),
            _x if _x == Btn::BumperL as u16 => Event::BumperL(p),
            _x if _x == Btn::BumperR as u16 => Event::BumperR(p),
            _x if _x == Btn::Joy as u16 => Event::Joy(p),
            _x if _x == Btn::Cam as u16 => Event::Cam(p),
            _x if _x == Btn::PaddleLeft as u16 => Event::PaddleLeft(p),
            _x if _x == Btn::PaddleRight as u16 => Event::PaddleRight(p),
            _x if _x == Btn::PinkyLeft as u16 => Event::PinkyLeft(p),
            _x if _x == Btn::PinkyRight as u16 => Event::PinkyRight(p),
            _x if _x == Btn::Trigger as u16 => Event::Trigger(p),
            _x if _x == Btn::HatUp as u16 => Event::HatUp(p),
            _x if _x == Btn::HatDown as u16 => Event::HatDown(p),
            _x if _x == Btn::HatRight as u16 => Event::HatRight(p),
            _x if _x == Btn::HatLeft as u16 => Event::HatLeft(p),
            _x => return Poll::Pending, // Error
        })
    }

    fn map_to_number(input: u16, p: bool) -> Poll<Event> {
        Poll::Ready(Event::Number((input - 0x0100) as i8, p))
    }

    fn map_to_axis(input: u16, v: f64) -> Poll<Event> {
        Poll::Ready(match input - 0x0180 {
            _x if _x == Axs::JoyX as u16 => Event::JoyX(v),
            _x if _x == Axs::JoyY as u16 => Event::JoyY(v),
            _x if _x == Axs::JoyZ as u16 => Event::JoyZ(v),
            _x if _x == Axs::CamX as u16 => Event::CamX(v),
            _x if _x == Axs::CamY as u16 => Event::CamY(v),
            _x if _x == Axs::CamZ as u16 => Event::CamZ(v),
            _x if _x == Axs::Wheel as u16 => Event::Wheel(v),
            _x if _x == Axs::Brake as u16 => Event::Brake(v),
            _x if _x == Axs::Gas as u16 => Event::Gas(v),
            _x if _x == Axs::Rudder as u16 => Event::Rudder(v),
            _x => return Poll::Pending, // Error
        })
    }

    fn map_to_pressure(input: u16, v: f64) -> Poll<Event> {
        Poll::Ready(match input - 0x0200 {
            _x if _x == Axs::TriggerL as u16 => Event::TriggerL(v),
            _x if _x == Axs::TriggerR as u16 => Event::TriggerR(v),
            _x => return Poll::Pending, // Error
        })
    }

    fn remap_button(&self, b: Btn, p: bool) -> Poll<Event> {
        let b = 0x0080 + b as u16;
        let new_event = self.remap.maps.get(&b).copied().unwrap_or(b);
        match new_event {
            0 => Poll::Pending,
            0x0080..=0x0FF => Self::map_to_button(new_event, p),
            0x0100..=0x17F => Self::map_to_number(new_event, p),
            0x0180..=0x1FF => Poll::Pending, // Axis: Invalid!
            0x0200..=0x27F => {
                Self::map_to_pressure(new_event, f64::from(u8::from(p)))
            }
            _ => Poll::Pending,
        }
    }

    fn remap_number(&self, n: i8, p: bool) -> Poll<Event> {
        let n = 0x0100 + n as u16;
        let new_event = self.remap.maps.get(&n).copied().unwrap_or(n);
        match new_event {
            0 => Poll::Pending,
            0x0080..=0x0FF => Self::map_to_button(new_event, p),
            0x0100..=0x17F => Self::map_to_number(new_event, p),
            0x0180..=0x1FF => Poll::Pending, // Axis: Invalid!
            0x0200..=0x27F => {
                Self::map_to_pressure(new_event, f64::from(u8::from(p)))
            }
            _ => Poll::Pending,
        }
    }

    fn remap_axis(&self, a: Axs, v: f64) -> Poll<Event> {
        let a = 0x0180 + a as u16;
        let new_event = self.remap.maps.get(&a).copied().unwrap_or(a);
        match new_event {
            0 => Poll::Pending,
            0x0080..=0x0FF => Poll::Pending, // Button: Invalid
            0x0100..=0x17F => Poll::Pending, // Number: Invalid
            0x0180..=0x1FF => Self::map_to_axis(new_event, v),
            0x0200..=0x27F => Poll::Pending, // Pressure: Invalid
            _ => Poll::Pending,
        }
    }

    fn remap_pressure(&self, b: Axs, v: f64) -> Poll<Event> {
        let b = 0x0200 + b as u16;
        let new_event = self.remap.maps.get(&b).copied().unwrap_or(b);
        match new_event {
            0 => Poll::Pending,
            0x0080..=0x0FF => Self::map_to_button(new_event, v >= 1.0),
            0x0100..=0x17F => Self::map_to_number(new_event, v >= 1.0),
            0x0180..=0x1FF => Poll::Pending, // Axis: Invalid!
            0x0200..=0x27F => Self::map_to_pressure(new_event, v),
            _ => Poll::Pending,
        }
    }

    fn remap(&self, in_event: Event) -> Poll<Event> {
        use Event::*;

        match in_event {
            Disconnect => Poll::Ready(Disconnect),
            Exit(p) => self.remap_button(Btn::Exit, p),
            MenuL(p) => self.remap_button(Btn::MenuL, p),
            MenuR(p) => self.remap_button(Btn::MenuR, p),
            ActionA(p) => self.remap_button(Btn::ActionA, p),
            ActionB(p) => self.remap_button(Btn::ActionB, p),
            ActionC(p) => self.remap_button(Btn::ActionC, p),
            ActionH(p) => self.remap_button(Btn::ActionH, p),
            ActionV(p) => self.remap_button(Btn::ActionV, p),
            ActionD(p) => self.remap_button(Btn::ActionD, p),
            Up(p) => self.remap_button(Btn::Up, p),
            Down(p) => self.remap_button(Btn::Down, p),
            Right(p) => self.remap_button(Btn::Right, p),
            Left(p) => self.remap_button(Btn::Left, p),
            BumperL(p) => self.remap_button(Btn::BumperL, p),
            BumperR(p) => self.remap_button(Btn::BumperR, p),
            TriggerL(v) => self.remap_pressure(Axs::TriggerL, v),
            TriggerR(v) => self.remap_pressure(Axs::TriggerR, v),
            Joy(p) => self.remap_button(Btn::Joy, p),
            Cam(p) => self.remap_button(Btn::Cam, p),
            JoyX(v) => self.remap_axis(Axs::JoyX, v),
            JoyY(v) => self.remap_axis(Axs::JoyY, v),
            JoyZ(v) => self.remap_axis(Axs::JoyZ, v),
            CamX(v) => self.remap_axis(Axs::CamX, v),
            CamY(v) => self.remap_axis(Axs::CamY, v),
            CamZ(v) => self.remap_axis(Axs::CamZ, v),
            PaddleLeft(p) => self.remap_button(Btn::PaddleLeft, p),
            PaddleRight(p) => self.remap_button(Btn::PaddleRight, p),
            PinkyLeft(p) => self.remap_button(Btn::PinkyLeft, p),
            PinkyRight(p) => self.remap_button(Btn::PinkyRight, p),
            Number(n, p) => self.remap_number(n, p),
            Wheel(v) => self.remap_axis(Axs::Wheel, v),
            Brake(v) => self.remap_axis(Axs::Brake, v),
            Gas(v) => self.remap_axis(Axs::Gas, v),
            Rudder(v) => self.remap_axis(Axs::Rudder, v),
            HatUp(p) => self.remap_button(Btn::HatUp, p),
            HatDown(p) => self.remap_button(Btn::HatDown, p),
            HatRight(p) => self.remap_button(Btn::HatRight, p),
            HatLeft(p) => self.remap_button(Btn::HatLeft, p),
            Trigger(p) => self.remap_button(Btn::Trigger, p),
            _event => todo!(), // FIXME
        }
    }

    fn process(&mut self, in_event: Event) -> Poll<Event> {
        use Event::*;
        match in_event {
            Disconnect => Poll::Ready(Disconnect),
            Exit(p) => self.button(Btn::Exit, Exit, p),
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
            TriggerL(v) => self.pressure(Axs::TriggerL, TriggerL, v),
            TriggerR(v) => self.pressure(Axs::TriggerR, TriggerR, v),
            Joy(p) => self.button(Btn::Joy, Joy, p),
            Cam(p) => self.button(Btn::Cam, Cam, p),
            JoyX(v) => self.axis(Axs::JoyX, JoyX, v),
            JoyY(v) => self.axis(Axs::JoyY, JoyY, v),
            JoyZ(v) => self.axis(Axs::JoyZ, JoyZ, v),
            CamX(v) => self.axis(Axs::CamX, CamX, v),
            CamY(v) => self.axis(Axs::CamY, CamY, v),
            CamZ(v) => self.axis(Axs::CamZ, CamZ, v),
            PaddleLeft(p) => self.button(Btn::PaddleLeft, PaddleLeft, p),
            PaddleRight(p) => self.button(Btn::PaddleRight, PaddleRight, p),
            PinkyLeft(p) => self.button(Btn::PinkyLeft, PinkyLeft, p),
            PinkyRight(p) => self.button(Btn::PinkyRight, PinkyRight, p),
            Number(n, p) => self.number(n, Number, p),
            Wheel(v) => self.axis(Axs::Wheel, Wheel, v),
            Brake(v) => self.axis(Axs::Brake, Brake, v),
            Gas(v) => self.axis(Axs::Gas, Gas, v),
            Rudder(v) => self.axis(Axs::Rudder, Rudder, v),
            HatUp(p) => self.hat(Btn::HatUp, Btn::HatDown, HatUp, HatDown, p),
            HatDown(p) => self.hat(Btn::HatDown, Btn::HatUp, HatDown, HatUp, p),
            HatRight(p) => {
                self.hat(Btn::HatRight, Btn::HatLeft, HatRight, HatLeft, p)
            }
            HatLeft(p) => {
                self.hat(Btn::HatLeft, Btn::HatRight, HatLeft, HatRight, p)
            }
            Trigger(p) => self.button(Btn::Trigger, Trigger, p),

            _event => todo!(), // FIXME
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
