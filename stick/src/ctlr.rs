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

use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::Event;

/// A gamepad, flightstick, or other controller.
pub struct Controller(pub(crate) crate::ffi::Ctlr);

impl Debug for Controller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pad(\"{}\")", self.name())
    }
}

impl Controller {
    /// Get a future that returns an event when a new controller is plugged in.
    pub fn listener() -> impl Future<Output = Self> {
        crate::ffi::Hub::new()
    }

    /// enable or disable event generation. Disable events when the application loses focus
    pub fn enable(flag: bool) {
        crate::ffi::Hub::enable(flag);
    }

    /// Get a unique identifier for the specific model of gamepad.
    pub fn id(&self) -> [u16; 4] {
        self.0.id()
    }

    /// Get the name of this Pad.
    pub fn name(&self) -> String {
        self.0.name()
    }

    /// Turn on/off haptic force feedback.
    ///
    /// Takes either an `f32` for mono power or `(f32, f32)` for directional
    /// power.  Power will be clamped between 0.0 (off) and 1.0 (maximum power).
    pub fn rumble<R: Rumble>(&mut self, power: R) {
        self.0.rumble(power.left(), power.right());
    }
}

impl Future for Controller {
    type Output = Event;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().0.poll(cx)
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
