// Stick
//
// Copyright (c) 2017-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::pin::Pin;
use std::task::{Context, Poll};

use crate::Event;

/// A future that looks for new `Pad`s on the computer's USB hub or equivalent.
#[allow(missing_debug_implementations)]
pub struct Hub(crate::ffi::Hub);

impl Hub {
    /// Create a future to start looking for new connections to gamepads.
    pub fn new() -> Self {
        Hub()
    }
}

impl Default for Hub {
    fn default() -> Self {
        Self::new()
    }
}

impl Future for Hub {
    type Output = (usize, Event);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().0.poll(cx)
    }
}
