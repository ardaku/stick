// Copyright © 2017-2022 The Stick Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

#![allow(unsafe_code)]

use crate::Event;
use std::task::{Poll, Context};

// Choose platform driver implementation.
#[allow(unused_attributes)]
#[cfg_attr(target_os = "linux", path = "../linux/linux.rs")]
#[cfg_attr(target_os = "windows", path = "../windows/windows.rs")]
#[path = "unsupported.rs"]
mod driver;

/// Controller ID.
pub(crate) struct CtlrId(u32);

/// Required platform support trait.
pub(crate) trait Support {
	/// Window gained focus, start receiving events.
    fn enable(self);
	/// Window lost focus, stop receiving events.
    fn disable(self);
	/// Set left and right rumble value for controller.
    fn rumble(self, ctlr: &CtlrId, left: f32, right: f32);
	/// Attempt getting a new event from a connected controller.
    fn event(self, ctlr: &CtlrId, cx: &mut Context<'_>) -> Poll<Event>;
	/// Attempt connecting to a new controller.
	fn connect(self, cx: &mut Context<'_>) -> Poll<(u64, String, CtlrId)>;
}

/// Get platform support implementation.
///
/// Each platform must implement this function to work.
pub(crate) fn platform() -> impl Support {
    driver::platform()
}