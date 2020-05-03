// Stick
//
// Copyright (c) 2017-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::Event;

/// A w3c "Standard Gamepad".
pub struct Gamepad(pub(crate) crate::ffi::Gamepad);

impl std::fmt::Debug for Gamepad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Gamepad")
    }
}

impl Gamepad {
    /// Get a unique identifier for the specific model of gamepad.
    pub fn id(&self) -> u32 {
        self.0.id()
    }

    /// Get the name of this Gamepad.
    pub fn name(&self) -> String {
        self.0.name()
    }

    /// Turn on/off haptic force feedback.  Set `power` between 0.0 (off) and
    /// 1.0 (maximum vibration).  Anything outside that range will be clamped.
    pub fn rumble(&mut self, power: f32) {
        self.0.rumble(power.min(1.0).max(0.0));
    }
}

impl Future for Gamepad {
    type Output = Event;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().0.poll(cx)
    }
}
