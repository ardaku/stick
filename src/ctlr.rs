// Copyright Jeron Aldaron Lau 2017 - 2020.
// Distributed under either the Apache License, Version 2.0
//    (See accompanying file LICENSE_APACHE_2_0.txt or copy at
//          https://apache.org/licenses/LICENSE-2.0),
// or the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_BOOST_1_0.txt or copy at
//          https://www.boost.org/LICENSE_1_0.txt)
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

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
    pub fn listener() -> impl Future<Output = (usize, Event)> {
        crate::ffi::Hub::new()
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
        self.0.rumbles(left_power.min(1.0).max(0.0), left_power.min(1.0).max(0.0));
    }
}

impl Future for Controller {
    type Output = Event;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().0.poll(cx)
    }
}
