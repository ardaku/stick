// Stick
//
// Copyright (c) 2017-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

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
