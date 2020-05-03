use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::Event;

/// A w3c "Standard Gamepad".
pub struct Gamepad(pub(crate) crate::ffi::Gamepad);

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

    /// Set LED light pattern
    pub fn leds(&mut self, pattern: [bool; 4]) {
        self.0.leds(pattern);
    }
}

impl Future for Gamepad {
    type Output = Event;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.get_mut().0.poll(cx)
    }
}
