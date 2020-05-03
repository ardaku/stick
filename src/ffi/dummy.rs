// Stick
//
// Copyright (c) 2017-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::task::{Context, Poll};

use crate::Event;

pub(crate) struct Port {
}

impl Port {
    pub(super) fn new() -> Self {
        Port {
        }
    }

    pub(super) fn poll(&mut self, cx: &mut Context<'_>) -> Poll<(usize, Event)> {
        let _ = cx;
    
        Poll::Pending
    }
}

pub(crate) struct Gamepad {
}

impl Gamepad {
    #[allow(unused)]
    fn new(device: ()) -> Self {
        let _ = device;
    
        Gamepad {
        }
    }

    pub(super) fn id(&self) -> u32 {
        0xFFFF_FFFF
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
