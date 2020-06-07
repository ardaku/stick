// Stick
//
// Copyright (c) 2017-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::Event;

/// A gamepad, flightstick, or other controller.
pub struct Pad(pub(crate) crate::ffi::Pad);

impl Debug for Pad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pad(\"{}\")", self.name())
    }
}

impl Pad {
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
}

impl Future for Pad {
    type Output = Event;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().0.poll(cx)
    }
}
