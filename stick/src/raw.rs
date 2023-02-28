// Stick
// Copyright © 2017-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

#![allow(unsafe_code)]

use crate::{Event, Remap};
use std::task::{Context, Poll};

#[cfg_attr(
    any(target_arch = "wasm32", target_arch = "asmjs"),
    cfg_attr(target_os = "wasi", path = "raw/wasi.rs"),
    cfg_attr(target_os = "ardaku", path = "raw/ardaku.rs"),
    cfg_attr(
        any(target_os = "unknown", target_os = "emscripten"),
        path = "raw/dom.rs"
    )
)]
#[cfg_attr(
    not(any(target_arch = "wasm32", target_arch = "asmjs")),
    cfg_attr(target_os = "linux", path = "raw/linux.rs"),
    cfg_attr(target_os = "android", path = "raw/android.rs"),
    cfg_attr(target_os = "macos", path = "raw/macos.rs"),
    cfg_attr(target_os = "ios", path = "raw/ios.rs"),
    cfg_attr(target_os = "windows", path = "raw/windows.rs"),
    cfg_attr(target_os = "fuchsia", path = "raw/fuchsia.rs"),
    cfg_attr(target_os = "redox", path = "raw/redox.rs"),
    cfg_attr(
        any(
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "bitrig",
            target_os = "openbsd",
            target_os = "netbsd"
        ),
        path = "raw/bsd.rs",
    )
)]
mod ffi;

/// Global state for when the system implementation can fail.
struct FakeGlobal;

impl Global for FakeGlobal {}

/// A Listener that never returns any controllers for unsupported platforms.
struct FakeListener;

impl Listener for FakeListener {
    fn poll(&mut self, _cx: &mut Context<'_>) -> Poll<crate::Controller> {
        Poll::Pending
    }
}

/// Controller Listener Implementation
pub(crate) trait Listener: Send {
    /// Poll for controllers.
    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<crate::Controller>;
}

/// Controller Implementation
pub(crate) trait Controller: Send {
    /// The hardware identifier for this controller.
    fn id(&self) -> u64 {
        0
    }
    /// Poll for events.
    fn poll(&mut self, _cx: &mut Context<'_>) -> Poll<Event> {
        Poll::Pending
    }
    /// Stereo rumble effect (left is low frequency, right is high frequency).
    fn rumble(&mut self, _left: f32, _right: f32) {}
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
pub(crate) trait Global: std::any::Any {
    /// Enable all events (when window comes in focus).
    fn enable(&self) {}
    /// Disable all events (when window leaves focus).
    fn disable(&self) {}
    /// Create a new listener.
    fn listener(&self, _remap: Remap) -> Box<dyn Listener> {
        Box::new(FakeListener)
    }
}

thread_local! {
    pub(crate) static GLOBAL: Box<dyn Global> = ffi::global();
}
