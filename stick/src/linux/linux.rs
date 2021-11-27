// Copyright © 2017-2021 The Stick Crate Developers.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

mod controller;
mod evdev;
mod haptic;

use std::sync::atomic::{Ordering, AtomicBool};
use std::io::{ErrorKind};
use std::os::unix::io::AsRawFd;

use super::Support;
use crate::Event;

pub(crate) use controller::{Controller, connect};

static ENABLED: AtomicBool = AtomicBool::new(true);

pub(super) struct Platform;

impl Support for &Platform {
    fn enable(self) {
        ENABLED.store(true, Ordering::Relaxed);
    }

    fn disable(self) {
        ENABLED.store(false, Ordering::Relaxed);
    }

    fn rumble(self, controller: &mut Controller, left: f32, right: f32) {
        haptic::joystick_ff(controller.stream.get_ref().as_raw_fd(), controller.rumble, left, right);
    }

    fn event(self, controller: &mut Controller) -> Option<Event> {
        if let Some(event) = controller.queued.take() {
            return Some(event);
        }
        match evdev::to_stick_events(controller) {
            Ok(None) => self.event(controller),
            Ok(Some(event)) => Some(event),
            Err(e) => if e.kind() != ErrorKind::WouldBlock {
                Some(Event::Disconnect)
            } else {
                None
            },
        }
    }
}

pub(super) fn platform() -> &'static Platform {
    &Platform
}
