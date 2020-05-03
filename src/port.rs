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

/// A future that looks for new gamepad devices.
#[allow(missing_debug_implementations)]
pub struct Port(crate::ffi::Port);

impl Port {
    /// Create a future to start looking for new connections to gamepads.
    pub fn new() -> Self {
        Port(crate::ffi::Port::new())
    }
}

impl Future for Port {
    type Output = (usize, Event);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().0.poll(cx)
    }
}
