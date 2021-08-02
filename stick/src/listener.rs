// Stick
// Copyright Jeron Aldaron Lau 2021.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

use crate::Remap;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

/// Listener for when new controllers are plugged in.
pub struct Listener(Box<dyn crate::raw::Listener>);

impl Debug for Listener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Listener")
    }
}

impl Default for Listener {
    fn default() -> Self {
        Self::new(Remap::default())
    }
}

impl Listener {
    /// Create a new listener for when new controllers are plugged in.
    pub fn new(remap: Remap) -> Self {
        Self(crate::raw::GLOBAL.with(|g| g.listener(remap)))
    }
}

impl Future for Listener {
    type Output = crate::Controller;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.get_mut().0).poll(cx)
    }
}
