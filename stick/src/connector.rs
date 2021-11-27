// Copyright © 2017-2021 The Stick Crate Developers.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

use crate::{Controller, Remap};
use lookit::Lookit;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

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
