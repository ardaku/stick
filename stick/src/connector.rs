use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use lookit::Lookit;

use crate::{Controller, Remap};

/// Future that you can `.await` to connect to
/// [`Controller`](crate::Controller)s
#[derive(Debug)]
pub struct Connector(Lookit, Remap);

impl Default for Connector {
    fn default() -> Self {
        Self::new(Remap::default())
    }
}

impl Connector {
    /// Create a new controller connector
    pub fn new(remap: Remap) -> Self {
        Self(Lookit::with_input(), remap)
    }
}

impl Future for Connector {
    type Output = Controller;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        let a = Pin::new(&mut self.as_mut().0)
            .poll(cx)
            .map(|device| Controller::new(device, &self.1));
        match a {
            Poll::Ready(Some(x)) => Poll::Ready(x),
            Poll::Ready(None) => self.poll(cx),
            Poll::Pending => Poll::Pending,
        }
    }
}
