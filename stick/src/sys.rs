// Stick
// Copyright © 2017-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use std::task::{Context, Poll};
use crate::{Remap, Event};

#[cfg_attr(target_arch = "wasm32", path = "sys/web.rs")]
#[cfg_attr(
    not(target_arch = "wasm32"),
    cfg_attr(target_os = "linux", path = "sys/linux.rs"),
    cfg_attr(target_os = "android", path = "sys/android.rs"),
    cfg_attr(target_os = "macos", path = "sys/macos.rs"),
    cfg_attr(target_os = "ios", path = "sys/ios.rs"),
    cfg_attr(target_os = "windows", path = "sys/windows.rs"),
    cfg_attr(
        any(
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "bitrig",
            target_os = "openbsd",
            target_os = "netbsd"
        ),
        path = "sys/bsd.rs",
    ),
    cfg_attr(target_os = "fuchsia", path = "sys/fuchsia.rs"),
    cfg_attr(target_os = "redox", path = "sys/redox.rs"),
    cfg_attr(target_os = "dive", path = "sys/dive.rs"),
)]
#[allow(unsafe_code)]
mod ffi;

/// A Listener that never returns any controllers for unsupported platforms.
struct FakeListener;

impl Listener for FakeListener {
    fn poll(&mut self, _cx: &mut Context<'_>) -> Poll<crate::Controller> {
        Poll::Pending
    }
}

/// Controller Listener Implementation
pub(crate) trait Listener {
    /// Poll for controllers.
    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<crate::Controller>;
}

/// Controller Implementation
pub(crate) trait Controller {
    /// The hardware identifier for this controller.
    fn id(&self) -> u64 { 0 }
    /// Poll for events.
    fn poll(&mut self, _cx: &mut Context<'_>) -> Poll<Event> {
        Poll::Pending
    }
    /// Stereo rumble effect (left is low frequency, right is high frequency).
    fn rumble(&mut self, _left: f32, _right: f32) {
    }
    /// Get the name of this controller.
    fn name(&self) -> &str {
        "Unknown"
    }
    /// Floating Point Translation for pressure axis/buttons.
    fn pressure(&self, input: f64) -> f64 {
        input
    }
    /// Floating Point Translation for full axis values.
    fn axis(&self, input: f64) -> f64 {
        input
    }
}

/// Thread local global state implementation.
pub(crate) trait Global {
    /// Enable all events (when window comes in focus).
    fn enable(&self) { }
    /// Disable all events (when window leaves focus).
    fn disable(&self) { }
    /// Create a new listener.
    fn listener(&self, _remap: Remap) -> Box<dyn Listener> {
        Box::new(FakeListener)
    }
}

thread_local! {
    pub(crate) static GLOBAL: Box<dyn Global> = ffi::global();
}
