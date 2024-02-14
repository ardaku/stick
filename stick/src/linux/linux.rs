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
