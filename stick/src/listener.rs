use std::{
    fmt::Debug,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::Remap;

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
