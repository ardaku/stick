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

    /// Turn on/off haptic force feedback.  Set `power` between 0.0 (off) and
    /// 1.0 (maximum vibration).  Anything outside that range will be clamped.
    pub fn rumble(&mut self, power: f32) {
        self.0.rumble(power.min(1.0).max(0.0));
    }

    /// Turn on/off directional haptic force feedback.  Set `left_power` and `right_power` between 0.0 (off) and
    /// 1.0 (maximum vibration).  Anything outside that range will be clamped.
    pub fn rumbles(&mut self, left_power: f32, right_power: f32) {
        self.0.rumbles(
            left_power.min(1.0).max(0.0),
            right_power.min(1.0).max(0.0),
        );
    }
}

impl Future for Controller {
    type Output = Event;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().0.poll(cx)
    }
}
