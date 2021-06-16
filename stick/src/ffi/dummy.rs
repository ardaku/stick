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

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::Event;

pub(crate) struct Hub {}

impl Hub {
    pub(super) fn new() -> Self {
        Hub {}
    }
    pub(super) fn enable(flag: bool) {
        let _ = flag;
    }
}

impl Future for Hub {
    type Output = (usize, Event);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let _ = cx;

        Poll::Pending
    }
}

pub(crate) struct Ctlr {}

impl Ctlr {
    #[allow(unused)]
    fn new(device: ()) -> Self {
        let _ = device;

        Self {}
    }

    pub(super) fn id(&self) -> [u16; 4] {
        [0, 0, 0, 0]
    }

    pub(super) fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Event> {
        let _ = cx;

        Poll::Pending
    }

    pub(super) fn name(&self) -> String {
        "Unknown".to_string()
    }

    pub(super) fn rumble(&mut self, v: f32) {
        let _ = v;
    }

    pub(super) fn rumbles(&mut self, l: f32, r: f32) {
        let _ = (l, r);
    }
}
