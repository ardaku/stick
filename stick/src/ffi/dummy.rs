// Copyright Jeron Aldaron Lau 2017 - 2020.
// Distributed under either the Apache License, Version 2.0
//    (See accompanying file LICENSE_APACHE_2_0.txt or copy at
//          https://apache.org/licenses/LICENSE-2.0),
// or the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_BOOST_1_0.txt or copy at
//          https://www.boost.org/LICENSE_1_0.txt)
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::{future::Future, task::{Context, Poll}, pin::Pin};

use crate::Event;

pub(crate) struct Hub {}

impl Hub {
    pub(super) fn new() -> Self {
        Hub {}
    }
}

impl Future for Hub {
    type Output = (usize, Event);

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
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
}
