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
}

impl Future for Gamepad {
    type Output = Event;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.get_mut().0.poll(cx)
    }
}
