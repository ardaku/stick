use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::Event;

/// A future that looks for new gamepad devices.
pub struct Port(crate::ffi::Port);

impl Port {
    /// Create a future to start looking for new connections to gamepads.
    pub fn new() -> Self {
        Port(crate::ffi::Port::new())
    }
}

impl Future for Port {
    type Output = (usize, Event);

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.get_mut().0.poll(cx)
    }
}
